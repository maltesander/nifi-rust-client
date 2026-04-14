#!/usr/bin/env python3
"""Release automation for nifi-openapi-gen, nifi-rust-client, and nifictl.

Each crate is released independently with prefixed git tags:
  - gen-vX.Y.Z    for nifi-openapi-gen
  - client-vX.Y.Z for nifi-rust-client
  - ctl-vX.Y.Z    for nifictl

Usage:
    release.py gen <bump>     [--dry-run] [--tag-message MSG]
    release.py client <bump>  [--dry-run] [--skip-integration] [--tag-message MSG]
    release.py ctl <bump>     [--dry-run] [--tag-message MSG]

    bump: major | minor | patch

See AGENTS.md → "Releases" for the full workflow.
"""

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import urllib.error
import urllib.request
from dataclasses import dataclass, field
from datetime import date
from typing import List, Optional

# ---------------------------------------------------------------------------
# Constants
# ---------------------------------------------------------------------------

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
REPO_URL = "https://github.com/maltesander/nifi-rust-client"
CLIENT_BUILD_DEP_CARGO_TOML = os.path.join(ROOT, "crates", "nifi-rust-client", "Cargo.toml")
CTL_BUILD_DEP_CARGO_TOML = os.path.join(ROOT, "crates", "nifictl", "Cargo.toml")

BINARY_EXTENSIONS = {
    ".png", ".jpg", ".jpeg", ".gif", ".bmp", ".ico", ".svg",
    ".pdf", ".zip", ".tar", ".gz", ".bz2", ".xz",
    ".exe", ".dll", ".so", ".dylib", ".a",
    ".woff", ".woff2", ".ttf", ".eot",
}

# Scopes that are purely internal — never shown to users.
_EXCLUDED_SCOPES = {"ci", "release", "rustfmt", "rust-analyzer"}

# Message substrings/patterns that indicate formatting or tooling noise.
_NOISE_PATTERNS = [
    re.compile(r'\bfmt\b', re.IGNORECASE),
    re.compile(r'\bformat(ting)?\b', re.IGNORECASE),
    re.compile(r'\bclippy\b', re.IGNORECASE),
    re.compile(r'\brustfmt\b', re.IGNORECASE),
    re.compile(r'\bregenerate\b', re.IGNORECASE),
]

COMMIT_TYPE_MAP = {
    "feat": "Added",
    "fix": "Fixed",
    "docs": "Documentation",
    "refactor": "Changed",
    "test": "Tests",
    # "chore" intentionally omitted — housekeeping is not user-facing
}

CATEGORY_ORDER = ["Breaking Changes", "Added", "Changed", "Fixed", "Documentation", "Tests"]

# ---------------------------------------------------------------------------
# Crate configuration
# ---------------------------------------------------------------------------


@dataclass
class CrateConfig:
    """Per-crate release configuration."""

    id: str                             # "gen", "client", or "ctl"
    name: str                           # crate name, e.g. "nifi-openapi-gen"
    cargo_toml: str                     # absolute path to Cargo.toml
    changelog: str                      # absolute path to CHANGELOG.md
    tag_prefix: str                     # "gen", "client", or "ctl"
    commit_paths: List[str]             # paths passed to `git log -- ...` for changelog scoping
    stage_files: List[str]              # repo-relative files staged in the release commit
    readme_shorthand_paths: List[str]   # READMEs with version shorthands to bump on minor/major
    validate_build_dep: bool            # client only: ensure nifi-openapi-gen build-dep is published


CRATES = {
    "gen": CrateConfig(
        id="gen",
        name="nifi-openapi-gen",
        cargo_toml=os.path.join(ROOT, "crates", "nifi-openapi-gen", "Cargo.toml"),
        changelog=os.path.join(ROOT, "crates", "nifi-openapi-gen", "CHANGELOG.md"),
        tag_prefix="gen",
        commit_paths=["crates/nifi-openapi-gen/"],
        stage_files=[
            "crates/nifi-openapi-gen/Cargo.toml",
            "crates/nifi-openapi-gen/CHANGELOG.md",
            # Gen release must also bump dependent crates' version constraints
            # so the workspace path deps still resolve. This is a
            # workspace-coherence edit, not a release for those crates.
            "crates/nifi-rust-client/Cargo.toml",
            "crates/nifictl/Cargo.toml",
            "Cargo.lock",
        ],
        readme_shorthand_paths=[],
        validate_build_dep=False,
    ),
    "client": CrateConfig(
        id="client",
        name="nifi-rust-client",
        cargo_toml=os.path.join(ROOT, "crates", "nifi-rust-client", "Cargo.toml"),
        changelog=os.path.join(ROOT, "crates", "nifi-rust-client", "CHANGELOG.md"),
        tag_prefix="client",
        commit_paths=["crates/nifi-rust-client/", "tests/"],
        stage_files=[
            "crates/nifi-rust-client/Cargo.toml",
            "crates/nifi-rust-client/CHANGELOG.md",
            "crates/nifi-rust-client/README.md",
            "crates/nifi-rust-client/src/lib.rs",
            "README.md",
            "Cargo.lock",
        ],
        readme_shorthand_paths=[
            os.path.join(ROOT, "README.md"),
            os.path.join(ROOT, "crates", "nifi-rust-client", "README.md"),
            os.path.join(ROOT, "crates", "nifi-rust-client", "src", "lib.rs"),
        ],
        validate_build_dep=True,
    ),
    "ctl": CrateConfig(
        id="ctl",
        name="nifictl",
        cargo_toml=os.path.join(ROOT, "crates", "nifictl", "Cargo.toml"),
        changelog=os.path.join(ROOT, "crates", "nifictl", "CHANGELOG.md"),
        tag_prefix="ctl",
        commit_paths=["crates/nifictl/"],
        stage_files=[
            "crates/nifictl/Cargo.toml",
            "crates/nifictl/CHANGELOG.md",
            "Cargo.lock",
        ],
        readme_shorthand_paths=[],
        validate_build_dep=False,
    ),
}


# ---------------------------------------------------------------------------
# Shell helpers
# ---------------------------------------------------------------------------


def run(cmd, capture=True, check=True):
    """Run a shell command. Aborts on failure if check=True. Returns stdout.strip()."""
    result = subprocess.run(cmd, shell=True, capture_output=capture, text=True)
    if check and result.returncode != 0:
        print(f"ERROR: Command failed: {cmd}", file=sys.stderr)
        if result.stderr:
            print(result.stderr, file=sys.stderr)
        sys.exit(1)
    return result.stdout.strip() if result.stdout else ""


def _rollback_hint(crate: CrateConfig) -> str:
    staged = " ".join(crate.stage_files)
    return f"To rollback file changes: git checkout -- {staged}"


# ---------------------------------------------------------------------------
# CLI parsing
# ---------------------------------------------------------------------------


def parse_args():
    parser = argparse.ArgumentParser(
        description="Release automation for nifi-openapi-gen, nifi-rust-client, and nifictl.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "crate",
        choices=sorted(CRATES.keys()),
        help="Which crate to release",
    )
    parser.add_argument(
        "bump",
        choices=["major", "minor", "patch"],
        help="Version component to bump",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Simulate the release without making any changes",
    )
    parser.add_argument(
        "--skip-integration",
        action="store_true",
        help="Skip integration tests (client release only)",
    )
    parser.add_argument(
        "--tag-message",
        type=str,
        help="Git tag annotation message (required unless --dry-run)",
    )
    parser.add_argument(
        "--no-push",
        action="store_true",
        help="Stop after creating the local commit and tag; do not push to origin",
    )
    args = parser.parse_args()
    if not args.dry_run and not args.tag_message:
        parser.error("--tag-message is required unless --dry-run is set")
    return args


# ---------------------------------------------------------------------------
# Preflight
# ---------------------------------------------------------------------------


def preflight_checks(new_tag):
    """Clean tree, on main branch, tag absent, tools present."""
    status = run("git status --porcelain")
    if status:
        print("  ABORT: Working tree is not clean. Commit or stash changes first.")
        sys.exit(1)
    print("      Working tree: clean")

    branch = run("git branch --show-current")
    if branch != "main":
        print(f"  ABORT: Must be on 'main' branch, currently on '{branch}'.")
        sys.exit(1)
    print("      Branch: main")

    existing_tags = run("git tag -l").splitlines()
    if new_tag in existing_tags:
        print(f"  ABORT: Tag '{new_tag}' already exists.")
        sys.exit(1)
    print(f"      Tag {new_tag}: does not exist")

    for tool in ("cargo", "git", "pre-commit"):
        if shutil.which(tool) is None:
            print(f"  ABORT: Required tool '{tool}' not found in PATH.")
            sys.exit(1)


# ---------------------------------------------------------------------------
# Version read / bump / Cargo.toml update
# ---------------------------------------------------------------------------


_VERSION_RE = re.compile(r'^version\s*=\s*"(\d+\.\d+\.\d+)"', re.MULTILINE)


def read_current_version(cargo_toml):
    with open(cargo_toml, "r") as f:
        content = f.read()
    match = _VERSION_RE.search(content)
    if not match:
        print(f"ERROR: Could not find version in {cargo_toml}", file=sys.stderr)
        sys.exit(1)
    return match.group(1)


def bump_version(version_str, bump_type):
    major, minor, patch = (int(p) for p in version_str.split("."))
    if bump_type == "major":
        major += 1
        minor = 0
        patch = 0
    elif bump_type == "minor":
        minor += 1
        patch = 0
    elif bump_type == "patch":
        patch += 1
    return f"{major}.{minor}.{patch}"


def update_crate_cargo_toml(cargo_toml, old_version, new_version, dry_run):
    with open(cargo_toml, "r") as f:
        content = f.read()

    old_str = f'version = "{old_version}"'
    new_str = f'version = "{new_version}"'
    if old_str not in content:
        print(f"  WARNING: '{old_str}' not found in {cargo_toml}", file=sys.stderr)
        return

    updated = content.replace(old_str, new_str, 1)
    rel = os.path.relpath(cargo_toml, ROOT)
    print(f"      {rel}: {old_str!r} → {new_str!r}")
    if not dry_run:
        with open(cargo_toml, "w") as f:
            f.write(updated)
    else:
        print("      (skipped write — dry-run)")


def update_client_build_dep(new_gen_version, dry_run):
    """Rewrite nifi-openapi-gen version pins in dependent crates.

    The workspace has path deps with version constraints. When gen's
    Cargo.toml version is bumped, the dependent crates' constraints must
    be bumped in lockstep or `cargo generate-lockfile` fails to resolve.
    This is a workspace-coherence edit — it does NOT bump any dependent
    crate's own version or trigger a release for them.
    """
    pattern = re.compile(
        r'(nifi-openapi-gen\s*=\s*\{[^}]*version\s*=\s*")\d+\.\d+\.\d+(")'
    )
    for toml_path in [CLIENT_BUILD_DEP_CARGO_TOML, CTL_BUILD_DEP_CARGO_TOML]:
        if not os.path.exists(toml_path):
            continue
        with open(toml_path, "r") as f:
            content = f.read()
        new_content, count = pattern.subn(
            rf'\g<1>{new_gen_version}\g<2>',
            content,
        )
        rel = os.path.relpath(toml_path, ROOT)
        if count == 0:
            continue
        print(f"      {rel}: nifi-openapi-gen dep → \"{new_gen_version}\"")
        if not dry_run:
            with open(toml_path, "w") as f:
                f.write(new_content)
        else:
            print("      (skipped write — dry-run)")


# ---------------------------------------------------------------------------
# README version shorthand update (client only)
# ---------------------------------------------------------------------------


FEATURE_EXAMPLE_TAGS = [
    ("<!-- STATIC_FEATURE_EXAMPLE_START -->", "<!-- STATIC_FEATURE_EXAMPLE_END -->"),
    ("<!-- DYNAMIC_FEATURE_EXAMPLE_START -->", "<!-- DYNAMIC_FEATURE_EXAMPLE_END -->"),
    ("<!-- LIB_STATIC_FEATURE_EXAMPLE_START -->", "<!-- LIB_STATIC_FEATURE_EXAMPLE_END -->"),
    ("<!-- LIB_DYNAMIC_FEATURE_EXAMPLE_START -->", "<!-- LIB_DYNAMIC_FEATURE_EXAMPLE_END -->"),
]


def _version_shorthand(version_str):
    parts = version_str.split(".")
    return f"{parts[0]}.{parts[1]}"


def _update_readme_tagged_blocks(path, old_short, new_short):
    with open(path, "r") as f:
        content = f.read()

    changes = 0
    for start_tag, end_tag in FEATURE_EXAMPLE_TAGS:
        pattern = re.compile(
            r'(' + re.escape(start_tag) + r'.*?' + re.escape(end_tag) + r')',
            re.DOTALL,
        )

        def replace_in_block(m):
            nonlocal changes
            block = m.group(1)
            new_block, n = re.subn(
                r'(nifi-rust-client\s*=\s*")' + re.escape(old_short) + r'"',
                r'\g<1>' + new_short + '"',
                block,
            )
            changes += n
            new_block, n = re.subn(
                r'(version\s*=\s*")' + re.escape(old_short) + r'"',
                r'\g<1>' + new_short + '"',
                new_block,
            )
            changes += n
            return new_block

        content = pattern.sub(replace_in_block, content)

    if changes:
        with open(path, "w") as f:
            f.write(content)
    return changes


def update_readmes(crate: CrateConfig, old_version, new_version, bump_type, dry_run):
    if not crate.readme_shorthand_paths:
        return
    if bump_type == "patch":
        print("      README shorthands: skipped (patch bump)")
        return

    old_short = _version_shorthand(old_version)
    new_short = _version_shorthand(new_version)

    for path in crate.readme_shorthand_paths:
        rel = os.path.relpath(path, ROOT)
        if dry_run:
            print(f"      {rel}: would replace '{old_short}' → '{new_short}' (dry-run)")
            continue
        changes = _update_readme_tagged_blocks(path, old_short, new_short)
        if changes:
            print(f"      {rel}: replaced {changes} occurrence(s) of '{old_short}' → '{new_short}'")
        else:
            print(f"      {rel}: no occurrences of '{old_short}' found in tagged blocks")


# ---------------------------------------------------------------------------
# Stale version scan — scoped to the crate's owned files
# ---------------------------------------------------------------------------


def scan_stale_version(crate: CrateConfig, old_version, dry_run):
    """Walk the crate's owned files and warn/abort if old_version still appears.

    Skips the files that were just updated (Cargo.toml, CHANGELOG.md, stage_files).
    Bounded to the crate's commit_paths so a gen release does not complain
    about a stale 0.5.0 left inside nifi-rust-client — its own version is
    supposed to stay put during a gen release. The client's Cargo.toml
    build-dep is already in stage_files (touched by update_client_build_dep)
    so it's automatically excluded from the scan.
    """
    updated = set(crate.stage_files) | {
        os.path.relpath(crate.cargo_toml, ROOT),
        os.path.relpath(crate.changelog, ROOT),
    }
    for p in crate.readme_shorthand_paths:
        updated.add(os.path.relpath(p, ROOT))

    scan_roots = []
    for p in crate.commit_paths:
        abs_p = os.path.join(ROOT, p)
        if os.path.isdir(abs_p):
            scan_roots.append(abs_p)
    # Also scan the READMEs (root README for client, etc.) even if they aren't
    # inside the crate dir.
    extras = [os.path.join(ROOT, s) for s in crate.stage_files]

    found_any = False

    def scan_file(filepath):
        nonlocal found_any
        rel_path = os.path.relpath(filepath, ROOT)
        if rel_path in updated:
            return
        _, ext = os.path.splitext(filepath)
        if ext.lower() in BINARY_EXTENSIONS:
            return
        try:
            with open(filepath, "r", encoding="utf-8", errors="ignore") as f:
                for lineno, line in enumerate(f, start=1):
                    if old_version in line:
                        found_any = True
                        print(f"      {rel_path}:{lineno}: {line.rstrip()}")
        except OSError:
            pass

    for scan_root in scan_roots:
        for dirpath, dirnames, filenames in os.walk(scan_root):
            dirnames[:] = [d for d in dirnames if d not in {".git", "target"}]
            for filename in filenames:
                scan_file(os.path.join(dirpath, filename))

    for extra in extras:
        if os.path.isfile(extra):
            scan_file(extra)

    if found_any:
        if dry_run:
            print("      WARNING: stale version references found (dry-run, continuing)")
        else:
            print(
                f"ERROR: Stale version '{old_version}' found in source files above. "
                "Update them before releasing.",
                file=sys.stderr,
            )
            sys.exit(1)
    else:
        print("      No stale references found.")


# ---------------------------------------------------------------------------
# Cargo.lock update
# ---------------------------------------------------------------------------


def update_lockfile(dry_run):
    if dry_run:
        print("      Skipped (dry-run)")
        return
    run("cargo generate-lockfile", capture=False)


# ---------------------------------------------------------------------------
# Changelog generation (per crate, path-scoped)
# ---------------------------------------------------------------------------


def _resolve_git_range(crate: CrateConfig, old_version) -> str:
    """Pick the best git range for changelog generation.

    Prefer the prefixed tag ({crate}-v{old}). Fall back to the legacy v{old}
    tag for the first release under the new scheme.
    """
    prefixed = f"{crate.tag_prefix}-v{old_version}"
    legacy = f"v{old_version}"
    existing = set(run("git tag -l").splitlines())
    if prefixed in existing:
        return f"{prefixed}..HEAD"
    if legacy in existing:
        return f"{legacy}..HEAD"
    # First release ever: log everything.
    return "HEAD"


def _is_noise(message):
    return any(p.search(message) for p in _NOISE_PATTERNS)


def parse_conventional_commits(crate: CrateConfig, old_version):
    """Parse git log for commits that touched the crate's files, grouped by category.

    Path-scoped via `git log ... -- <commit_paths>`. Commits touching multiple
    crates naturally appear in both crates' changelogs.
    """
    git_range = _resolve_git_range(crate, old_version)
    paths_arg = " ".join(f'"{p}"' for p in crate.commit_paths)
    log = run(f"git log {git_range} --format='%h %s' -- {paths_arg}")

    categories = {}
    for line in log.splitlines():
        line = line.strip()
        if not line:
            continue
        parts = line.split(" ", 1)
        if len(parts) < 2:
            continue
        short_hash = parts[0]
        subject = parts[1]

        match = re.match(r'^(\w+)(?:\(([^)]*)\))?(!)?\s*:\s*(.+)$', subject)
        if match:
            commit_type = match.group(1)
            scope = match.group(2) or ""
            is_breaking = bool(match.group(3))
            message = match.group(4)
        else:
            commit_type = ""
            scope = ""
            is_breaking = False
            message = subject

        if scope in _EXCLUDED_SCOPES:
            continue
        if commit_type == "chore":
            continue
        if _is_noise(message):
            continue

        message = message[0].upper() + message[1:] if message else message
        commit_link = f"[{short_hash}]({REPO_URL}/commit/{short_hash})"
        entry = f"{message} ({commit_link})"

        if is_breaking:
            categories.setdefault("Breaking Changes", []).append(entry)
        else:
            category = COMMIT_TYPE_MAP.get(commit_type, None)
            if category is None:
                continue
            categories.setdefault(category, []).append(entry)

    return categories


def _compare_link_old_ref(crate: CrateConfig, old_version) -> str:
    """Pick the ref name for the previous version's comparison link.

    Fall back to the legacy 'v{old}' tag when the prefixed one doesn't exist yet.
    """
    prefixed = f"{crate.tag_prefix}-v{old_version}"
    legacy = f"v{old_version}"
    existing = set(run("git tag -l").splitlines())
    if prefixed in existing:
        return prefixed
    if legacy in existing:
        return legacy
    return prefixed  # will exist after the very first release


def update_changelog(crate: CrateConfig, old_version, new_version, dry_run):
    categories = parse_conventional_commits(crate, old_version)

    today = date.today().isoformat()
    lines = [f"## [{new_version}] - {today}"]
    for category in CATEGORY_ORDER:
        if category in categories:
            lines.append("")
            lines.append(f"### {category}")
            lines.append("")
            for msg in categories[category]:
                lines.append(f"- {msg}")
    section = "\n".join(lines)

    if dry_run:
        print("      Changelog section (dry-run):")
        for line in section.splitlines():
            print(f"      {line}")
        return

    with open(crate.changelog, "r") as f:
        content = f.read()

    unreleased_marker = "## [Unreleased]\n"
    if unreleased_marker in content:
        insert_pos = content.index(unreleased_marker) + len(unreleased_marker)
        rest = content[insert_pos:].lstrip("\n")
        content = content[:insert_pos] + "\n" + section + "\n\n" + rest
    else:
        print("      WARNING: '## [Unreleased]' marker not found; prepending section")
        content = section + "\n\n" + content

    # Update [Unreleased] comparison link to point at the new prefixed tag.
    # The old link may use either `v{old}...HEAD` (pre-split) or
    # `{prefix}-v{old}...HEAD` (post-split). Match both.
    new_tag = f"{crate.tag_prefix}-v{new_version}"
    content = re.sub(
        r'\[Unreleased\]:\s*' + re.escape(REPO_URL) + r'/compare/[^\s]+\.\.\.HEAD',
        f'[Unreleased]: {REPO_URL}/compare/{new_tag}...HEAD',
        content,
    )

    # Insert new comparison link for this version.
    old_ref = _compare_link_old_ref(crate, old_version)
    old_version_link_marker = f'[{old_version}]: {REPO_URL}/compare/'
    new_version_link = f'[{new_version}]: {REPO_URL}/compare/{old_ref}...{new_tag}\n'
    if old_version_link_marker in content:
        insert_idx = content.index(old_version_link_marker)
        content = content[:insert_idx] + new_version_link + content[insert_idx:]
    else:
        # The section insert above already ends `content` with `\n\n`, so
        # appending the link without a leading newline leaves exactly one
        # blank line between the last bullet and the link. markdownlint
        # MD012 fires on two consecutive blank lines.
        content += new_version_link

    with open(crate.changelog, "w") as f:
        f.write(content)

    editor = os.environ.get("EDITOR") or shutil.which("vi") or shutil.which("nano")
    if editor:
        print(f"      Opening {editor} for review...")
        subprocess.run([editor, crate.changelog])
        with open(crate.changelog, "r") as f:
            updated = f.read()
        if f"## [{new_version}]" not in updated:
            print(f"  ABORT: Version section [{new_version}] was removed from {os.path.relpath(crate.changelog, ROOT)}")
            sys.exit(1)
    else:
        print(f"      WARNING: No editor found ($EDITOR, vi, nano). Review {os.path.relpath(crate.changelog, ROOT)} manually.")
    print("      Changelog updated.")


# ---------------------------------------------------------------------------
# Build-dep validation (client only)
# ---------------------------------------------------------------------------


_BUILD_DEP_RE = re.compile(
    r'nifi-openapi-gen\s*=\s*\{[^}]*version\s*=\s*"(\d+\.\d+\.\d+)"'
)


def read_client_build_dep_version() -> str:
    with open(CLIENT_BUILD_DEP_CARGO_TOML, "r") as f:
        content = f.read()
    match = _BUILD_DEP_RE.search(content)
    if not match:
        print(
            "ERROR: could not find nifi-openapi-gen build-dep version in "
            f"{CLIENT_BUILD_DEP_CARGO_TOML}",
            file=sys.stderr,
        )
        sys.exit(1)
    return match.group(1)


def _is_version_on_crates_io(crate_name: str, version: str) -> bool:
    """Return True if crate@version exists on crates.io."""
    url = f"https://crates.io/api/v1/crates/{crate_name}/{version}"
    req = urllib.request.Request(url, headers={"User-Agent": "nifi-rust-client-release-py"})
    try:
        with urllib.request.urlopen(req, timeout=15) as resp:
            data = json.loads(resp.read().decode("utf-8"))
            return bool(data.get("version"))
    except urllib.error.HTTPError as e:
        if e.code == 404:
            return False
        raise
    except urllib.error.URLError as e:
        print(
            f"      WARNING: could not reach crates.io ({e}). Skipping build-dep check.",
            file=sys.stderr,
        )
        return True  # fail open — don't block a release because crates.io is down


def validate_client_build_dep(dry_run):
    """Ensure the nifi-openapi-gen version pinned in client's build-deps is published."""
    version = read_client_build_dep_version()
    print(f"      nifi-openapi-gen build-dep version: {version}")
    if dry_run:
        print("      Skipped crates.io lookup (dry-run)")
        return
    if _is_version_on_crates_io("nifi-openapi-gen", version):
        print(f"      crates.io: nifi-openapi-gen@{version} is published ✓")
        return
    print(
        f"ERROR: nifi-rust-client declares nifi-openapi-gen = \"{version}\" in its "
        f"build-dependencies, but that version is not published on crates.io.\n"
        f"       Run `release.py gen <bump>` first to release nifi-openapi-gen, or "
        f"edit crates/nifi-rust-client/Cargo.toml to pin a published version.",
        file=sys.stderr,
    )
    sys.exit(1)


# ---------------------------------------------------------------------------
# Pre-update and release checks
# ---------------------------------------------------------------------------


def _run_cmd_list(checks, rollback_hint):
    for cmd, label in checks:
        print(f"      {label}...", end=" ", flush=True)
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
        if result.returncode == 0:
            print("OK")
        else:
            print("FAILED")
            if result.stderr:
                print(result.stderr, file=sys.stderr)
            print(f"ERROR: '{label}' check failed. {rollback_hint}", file=sys.stderr)
            sys.exit(1)


def run_pre_update_checks(crate: CrateConfig, dry_run):
    """Validate packaging BEFORE version bump.

    For the client release, this also requires the declared build-dep version
    to be published (see validate_client_build_dep called from main before
    reaching this point).

    On the very first release, nothing is on crates.io yet: skip via
    SKIP_PACKAGE_CHECK=1.
    """
    if os.environ.get("SKIP_PACKAGE_CHECK"):
        print("      Skipped (SKIP_PACKAGE_CHECK set)")
        return

    checks = [
        (
            f"cargo package -p {crate.name} --allow-dirty",
            f"Package validation ({crate.name})",
        ),
    ]

    if dry_run:
        for cmd, _ in checks:
            print(f"      [dry-run] Would run: {cmd}")
        return

    _run_cmd_list(checks, _rollback_hint(crate))


def run_checks(crate: CrateConfig, dry_run, skip_integration):
    """Run build, test, clippy, and pre-commit checks after version bump."""
    if crate.id == "gen":
        checks = [
            ("cargo build -p nifi-openapi-gen", "Build (nifi-openapi-gen)"),
            ("cargo test -p nifi-openapi-gen", "Tests (nifi-openapi-gen)"),
            (
                "cargo clippy -p nifi-openapi-gen --all-targets --all-features -- -D warnings",
                "Clippy (nifi-openapi-gen)",
            ),
            ("pre-commit run --all-files", "Pre-commit"),
            (
                f"cargo package -p {crate.name} --allow-dirty",
                f"Package validation ({crate.name}, post-lockfile)",
            ),
        ]
    elif crate.id == "ctl":
        checks = [
            ("cargo build -p nifictl", "Build (nifictl)"),
            ("cargo test -p nifictl", "Tests (nifictl)"),
            (
                "cargo clippy -p nifictl --all-targets -- -D warnings",
                "Clippy (nifictl)",
            ),
            ("pre-commit run --all-files", "Pre-commit"),
            (
                f"cargo package -p {crate.name} --allow-dirty",
                f"Package validation ({crate.name}, post-lockfile)",
            ),
        ]
    else:  # client
        checks = [
            ("cargo build --workspace", "Build"),
            (
                "cargo test --workspace --all-features --exclude nifi-integration-tests",
                "Tests (all features)",
            ),
            ("cargo test -p nifi-integration-tests", "Tests (integration compile)"),
            (
                "cargo clippy --workspace --all-targets --all-features --exclude nifi-integration-tests -- -D warnings",
                "Clippy (all features)",
            ),
            ("pre-commit run --all-files", "Pre-commit"),
            (
                f"cargo package -p {crate.name} --allow-dirty",
                f"Package validation ({crate.name}, post-lockfile)",
            ),
        ]
        if not skip_integration:
            checks.insert(-1, ("./tests/run.sh", "Integration tests"))
        else:
            print("      Integration tests: skipped (--skip-integration)")

    if dry_run:
        for cmd, _ in checks:
            print(f"      [dry-run] Would run: {cmd}")
        return

    _run_cmd_list(checks, _rollback_hint(crate))


# ---------------------------------------------------------------------------
# Commit, tag, push
# ---------------------------------------------------------------------------


def commit_release(crate: CrateConfig, new_version, dry_run):
    if dry_run:
        print("      Skipped (dry-run)")
        return
    stage_args = " ".join(crate.stage_files)
    run(f"git add {stage_args}")
    run(f'git commit -m "chore(release): {crate.name} v{new_version}"')
    print(f"      chore(release): {crate.name} v{new_version}")


def tag_and_push(crate: CrateConfig, new_version, tag_message, dry_run, no_push):
    new_tag = f"{crate.tag_prefix}-v{new_version}"
    if dry_run:
        print(f"      Skipped (dry-run); would create tag {new_tag}")
        return

    run(f'git tag -a "{new_tag}" -m "{tag_message}"')
    print(f"      Tagged {new_tag}")

    if no_push:
        print(
            f"\nLocal release {new_tag} staged. --no-push set; nothing pushed to origin.\n"
            f"To publish:\n"
            f"  git push origin main\n"
            f"  git push origin {new_tag}\n"
            f"To abort:\n"
            f"  git tag -d {new_tag}\n"
            f"  git reset --hard HEAD~1"
        )
        return

    commit_hash = run("git rev-parse --short HEAD")

    push_result = subprocess.run(
        "git push origin main", shell=True, capture_output=True, text=True
    )
    if push_result.returncode != 0:
        print(push_result.stderr, file=sys.stderr)
        print(
            f"ERROR: Failed to push to origin/main. "
            f"Commit {commit_hash} was created locally. To undo: git reset HEAD~1",
            file=sys.stderr,
        )
        sys.exit(1)
    print("      Pushed to origin/main")

    tag_result = subprocess.run(
        f"git push origin {new_tag}", shell=True, capture_output=True, text=True
    )
    if tag_result.returncode != 0:
        print(tag_result.stderr, file=sys.stderr)
        print(
            f"ERROR: Commit was pushed but tag was not. "
            f"To push tag manually: git push origin {new_tag}",
            file=sys.stderr,
        )
        sys.exit(1)
    print(f"      Pushed tag {new_tag}")

    print(f"\nRelease {new_tag} complete. CI will publish to crates.io.")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------


def main():
    args = parse_args()
    crate = CRATES[args.crate]

    old_version = read_current_version(crate.cargo_toml)
    new_version = bump_version(old_version, args.bump)
    new_tag = f"{crate.tag_prefix}-v{new_version}"

    dry_label = " (dry-run)" if args.dry_run else ""
    print(f"Release {crate.name}{dry_label}: {old_version} → {new_version}  [{args.bump} bump]")
    print()

    print("[1/10] Pre-flight checks...")
    preflight_checks(new_tag)

    if crate.validate_build_dep:
        print("[2/10] Validating nifi-openapi-gen build-dep is published...")
        validate_client_build_dep(args.dry_run)
    else:
        print("[2/10] Build-dep validation: skipped (not applicable to this crate)")

    print("[3/10] Pre-update packaging validation...")
    run_pre_update_checks(crate, args.dry_run)

    print(f"[4/10] Version: {old_version} → {new_version}")
    update_crate_cargo_toml(crate.cargo_toml, old_version, new_version, args.dry_run)
    if crate.id == "gen":
        update_client_build_dep(new_version, args.dry_run)
    update_readmes(crate, old_version, new_version, args.bump, args.dry_run)

    print(f"[5/10] Scanning for stale version '{old_version}'...")
    scan_stale_version(crate, old_version, args.dry_run)

    print("[6/10] Updating Cargo.lock...")
    update_lockfile(args.dry_run)

    print("[7/10] Generating changelog...")
    update_changelog(crate, old_version, new_version, args.dry_run)

    print("[8/10] Running checks...")
    run_checks(crate, args.dry_run, args.skip_integration)

    print("[9/10] Committing...")
    commit_release(crate, new_version, args.dry_run)

    print("[10/10] Tagging and pushing...")
    tag_and_push(crate, new_version, args.tag_message, args.dry_run, args.no_push)

    if args.dry_run:
        print(f"\nDry-run complete. No changes were made.")


if __name__ == "__main__":
    main()
