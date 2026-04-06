#!/usr/bin/env python3
"""Release automation script for nifi-rust-client.

Automates version bumping, changelog generation, preflight checks,
tagging, and pushing for the nifi-rust-client crate.

Usage:
    release.py <bump> [--dry-run] [--skip-integration] [--tag-message MSG]

    bump: major | minor | patch
"""

import argparse
import os
import re
import subprocess
import sys
import shutil
from datetime import date

# ---------------------------------------------------------------------------
# Constants
# ---------------------------------------------------------------------------

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CARGO_TOML = os.path.join(ROOT, "Cargo.toml")
README = os.path.join(ROOT, "README.md")
CHANGELOG = os.path.join(ROOT, "CHANGELOG.md")

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

BINARY_EXTENSIONS = {
    ".png", ".jpg", ".jpeg", ".gif", ".bmp", ".ico", ".svg",
    ".pdf", ".zip", ".tar", ".gz", ".bz2", ".xz",
    ".exe", ".dll", ".so", ".dylib", ".a",
    ".woff", ".woff2", ".ttf", ".eot",
}

EXCLUDE_DIRS = {
    "target", ".git",
    os.path.join("docs", "superpowers", "specs"),
    os.path.join("docs", "superpowers", "plans"),
}

EXCLUDE_FILES = {
    "Cargo.lock",
    "CHANGELOG.md",
    os.path.join("release", "release.py"),
    "Cargo.toml",
    "README.md",
}


def run(cmd, capture=True, check=True):
    """Run a shell command. Aborts on failure if check=True. Returns stdout.strip()."""
    result = subprocess.run(cmd, shell=True, capture_output=capture, text=True)
    if check and result.returncode != 0:
        print(f"ERROR: Command failed: {cmd}", file=sys.stderr)
        if result.stderr:
            print(result.stderr, file=sys.stderr)
        sys.exit(1)
    return result.stdout.strip() if result.stdout else ""


# ---------------------------------------------------------------------------
# Task 1: CLI parsing and preflight checks
# ---------------------------------------------------------------------------

def parse_args():
    """Parse command-line arguments."""
    parser = argparse.ArgumentParser(
        description="Release automation for nifi-rust-client.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
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
        help="Skip integration tests",
    )
    parser.add_argument(
        "--tag-message",
        type=str,
        help="Git tag annotation message (required unless --dry-run)",
    )
    args = parser.parse_args()
    if not args.dry_run and not args.tag_message:
        parser.error("--tag-message is required unless --dry-run is set")
    return args


def preflight_checks(new_tag):
    """Run preflight checks: clean tree, main branch, tag absent, tools present."""
    # Clean working tree
    status = run("git status --porcelain")
    if status:
        print("  ABORT: Working tree is not clean. Commit or stash changes first.")
        sys.exit(1)
    print("      Working tree: clean")

    # On main branch
    branch = run("git branch --show-current")
    if branch != "main":
        print(f"  ABORT: Must be on 'main' branch, currently on '{branch}'.")
        sys.exit(1)
    print("      Branch: main")

    # Tag does not already exist
    existing_tags = run("git tag -l").splitlines()
    if new_tag in existing_tags:
        print(f"  ABORT: Tag '{new_tag}' already exists.")
        sys.exit(1)
    print(f"      Tag {new_tag}: does not exist")

    # Required tools
    for tool in ("cargo", "git", "pre-commit"):
        if shutil.which(tool) is None:
            print(f"  ABORT: Required tool '{tool}' not found in PATH.")
            sys.exit(1)


# ---------------------------------------------------------------------------
# Task 2: Version reading, bumping, and file updates
# ---------------------------------------------------------------------------

def read_current_version():
    """Read the current workspace version from Cargo.toml."""
    with open(CARGO_TOML, "r") as f:
        content = f.read()
    match = re.search(r'^version\s*=\s*"(\d+\.\d+\.\d+)"', content, re.MULTILINE)
    if not match:
        print("ERROR: Could not find version in Cargo.toml", file=sys.stderr)
        sys.exit(1)
    return match.group(1)


def bump_version(version_str, bump_type):
    """Bump a version string according to bump_type (major/minor/patch)."""
    parts = version_str.split(".")
    major, minor, patch = int(parts[0]), int(parts[1]), int(parts[2])
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


def update_cargo_toml(old_version, new_version, dry_run):
    """Replace first occurrence of version = "old" with version = "new" in Cargo.toml."""
    with open(CARGO_TOML, "r") as f:
        content = f.read()

    old_str = f'version = "{old_version}"'
    new_str = f'version = "{new_version}"'
    if old_str not in content:
        print(f"  WARNING: '{old_str}' not found in Cargo.toml", file=sys.stderr)
        return

    updated = content.replace(old_str, new_str, 1)
    print(f"      Cargo.toml: {old_str!r} → {new_str!r}")
    if not dry_run:
        with open(CARGO_TOML, "w") as f:
            f.write(updated)
    else:
        print("      (skipped write — dry-run)")


def _version_shorthand(version_str):
    """Return the major.minor shorthand for a version string."""
    parts = version_str.split(".")
    return f"{parts[0]}.{parts[1]}"


def update_readme(old_version, new_version, bump_type, dry_run):
    """Update README.md version shorthands on major/minor bumps."""
    if bump_type == "patch":
        print("      README.md: skipped (patch bump)")
        return

    old_short = _version_shorthand(old_version)
    new_short = _version_shorthand(new_version)

    with open(README, "r") as f:
        lines = f.readlines()

    updated_lines = []
    changes = 0
    for line in lines:
        new_line = line
        # Pattern 1: nifi-rust-client = "old_short"
        if re.search(r'nifi-rust-client\s*=\s*"' + re.escape(old_short) + r'"', line):
            new_line = re.sub(
                r'(nifi-rust-client\s*=\s*")' + re.escape(old_short) + r'"',
                r'\g<1>' + new_short + '"',
                new_line,
            )
            changes += 1
        # Pattern 2: lines containing nifi-rust-client AND version = "old_short"
        elif "nifi-rust-client" in line and f'version = "{old_short}"' in line:
            new_line = new_line.replace(f'version = "{old_short}"', f'version = "{new_short}"')
            changes += 1
        updated_lines.append(new_line)

    if changes == 0:
        print(f"      README.md: no occurrences of shorthand '{old_short}' found")
        return

    print(f"      README.md: replaced {changes} occurrence(s) of '{old_short}' → '{new_short}'")
    if not dry_run:
        with open(README, "w") as f:
            f.writelines(updated_lines)
    else:
        print("      (skipped write — dry-run)")


# ---------------------------------------------------------------------------
# Task 3: Post-update stale version scan
# ---------------------------------------------------------------------------

def scan_stale_version(old_version, dry_run):
    """Walk the repo and warn/abort if old_version still appears in source files."""
    print(f"[4/9] Scanning for stale version '{old_version}'...")

    found_any = False
    for dirpath, dirnames, filenames in os.walk(ROOT):
        # Prune excluded directories (modify in-place for os.walk efficiency)
        rel_dir = os.path.relpath(dirpath, ROOT)
        dirnames[:] = [
            d for d in dirnames
            if os.path.join(rel_dir, d) not in EXCLUDE_DIRS
            and d not in {".git", "target"}
        ]

        for filename in filenames:
            filepath = os.path.join(dirpath, filename)
            rel_path = os.path.relpath(filepath, ROOT)

            # Skip excluded files
            if rel_path in EXCLUDE_FILES or filename in EXCLUDE_FILES:
                continue

            # Skip binary extensions
            _, ext = os.path.splitext(filename)
            if ext.lower() in BINARY_EXTENSIONS:
                continue

            # Scan line by line
            try:
                with open(filepath, "r", encoding="utf-8", errors="ignore") as f:
                    for lineno, line in enumerate(f, start=1):
                        if old_version in line:
                            if not found_any:
                                found_any = True
                            print(f"      {rel_path}:{lineno}: {line.rstrip()}")
            except OSError:
                pass

    if found_any:
        if dry_run:
            print(f"      WARNING: stale version references found (dry-run, continuing)")
        else:
            print(
                f"ERROR: Stale version '{old_version}' found in source files above. "
                "Update them before releasing.",
                file=sys.stderr,
            )
            sys.exit(1)
    else:
        print(f"      No stale references found.")


# ---------------------------------------------------------------------------
# Task 4: Cargo.lock update
# ---------------------------------------------------------------------------

def update_lockfile(dry_run):
    """Regenerate Cargo.lock after version bump."""
    print("[5/9] Updating Cargo.lock...")
    if dry_run:
        print("      Skipped (dry-run)")
        return
    run("cargo generate-lockfile", capture=False)


# ---------------------------------------------------------------------------
# Task 5: Changelog generation with $EDITOR
# ---------------------------------------------------------------------------

COMMIT_TYPE_MAP = {
    "feat": "Added",
    "fix": "Fixed",
    "docs": "Documentation",
    "refactor": "Changed",
    "chore": "Changed",
    "test": "Tests",
}

CATEGORY_ORDER = ["Added", "Changed", "Fixed", "Documentation", "Tests", "Other"]

REPO_URL = "https://github.com/maltesander/nifi-rust-client"


def parse_conventional_commits(old_version):
    """Parse git log since old_version tag and group commits by category."""
    log = run(f"git log v{old_version}..HEAD --format='%h %s'")
    categories = {}
    for line in log.splitlines():
        line = line.strip()
        if not line:
            continue
        # Split short hash from subject
        parts = line.split(" ", 1)
        if len(parts) < 2:
            continue
        short_hash = parts[0]
        subject = parts[1]
        match = re.match(r'^(\w+)(?:\([^)]*\))?:\s*(.+)$', subject)
        if match:
            commit_type = match.group(1)
            message = match.group(2)
        else:
            commit_type = ""
            message = subject
        category = COMMIT_TYPE_MAP.get(commit_type, "Other")
        message = message[0].upper() + message[1:] if message else message
        commit_link = f"[{short_hash}]({REPO_URL}/commit/{short_hash})"
        categories.setdefault(category, []).append(f"{message} ({commit_link})")
    return categories


def update_changelog(old_version, new_version, dry_run):
    """Generate and insert a changelog section for new_version."""
    print("[6/9] Generating changelog...")
    categories = parse_conventional_commits(old_version)

    today = date.today().isoformat()
    lines = [f"## [{new_version}] - {today}"]
    for category in CATEGORY_ORDER:
        if category in categories:
            lines.append(f"\n### {category}")
            for msg in categories[category]:
                lines.append(f"- {msg}")
    section = "\n".join(lines)

    if dry_run:
        print("      Changelog section (dry-run):")
        for line in section.splitlines():
            print(f"      {line}")
        return

    with open(CHANGELOG, "r") as f:
        content = f.read()

    # Insert new section after ## [Unreleased] marker
    unreleased_marker = "## [Unreleased]\n"
    if unreleased_marker in content:
        insert_pos = content.index(unreleased_marker) + len(unreleased_marker)
        content = content[:insert_pos] + "\n" + section + "\n\n" + content[insert_pos:]
    else:
        print("      WARNING: '## [Unreleased]' marker not found in CHANGELOG.md; prepending section")
        content = section + "\n\n" + content

    # Update [Unreleased] comparison link
    content = re.sub(
        r'\[Unreleased\]:\s*' + re.escape(REPO_URL) + r'/compare/v[^.]+\.[^.]+\.[^.]+\.\.\.HEAD',
        f'[Unreleased]: {REPO_URL}/compare/v{new_version}...HEAD',
        content,
    )

    # Add new version comparison link before the old version link
    old_version_link = f'[{old_version}]: {REPO_URL}/compare/'
    new_version_link = f'[{new_version}]: {REPO_URL}/compare/v{old_version}...v{new_version}\n'
    if old_version_link in content:
        insert_idx = content.index(old_version_link)
        content = content[:insert_idx] + new_version_link + content[insert_idx:]
    else:
        content += f"\n{new_version_link}"

    with open(CHANGELOG, "w") as f:
        f.write(content)

    # Open editor for review
    editor = os.environ.get("EDITOR") or shutil.which("vi") or shutil.which("nano")
    if editor:
        print(f"      Opening {editor} for review...")
        subprocess.run([editor, CHANGELOG])
        # Verify the section is still present after editing
        with open(CHANGELOG, "r") as f:
            updated = f.read()
        if f"## [{new_version}]" not in updated:
            print(f"  ABORT: Version section [{new_version}] was removed from CHANGELOG.md")
            sys.exit(1)
    else:
        print("      WARNING: No editor found ($EDITOR, vi, nano). Review CHANGELOG.md manually.")
    print("      Changelog updated.")


# ---------------------------------------------------------------------------
# Task 6: Run checks
# ---------------------------------------------------------------------------

def run_checks(dry_run, skip_integration):
    """Run build, test, clippy, pre-commit, and package validation checks."""
    print("[7/9] Running checks...")

    checks = [
        ("cargo build --workspace", "Build"),
        ("cargo test --workspace", "Tests"),
        ("cargo clippy -p nifi-rust-client --features dynamic -- -D warnings", "Clippy"),
        ("pre-commit run --all-files", "Pre-commit"),
        ("cargo publish -p nifi-rust-client --dry-run", "Package validation"),
    ]

    if skip_integration:
        print("      Integration tests: skipped (--skip-integration)")
    else:
        checks.append(("./tests/run.sh", "Integration tests"))

    for cmd, label in checks:
        if dry_run:
            print(f"      [dry-run] Would run: {cmd}")
        else:
            print(f"      {label}...", end=" ", flush=True)
            result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
            if result.returncode == 0:
                print("OK")
            else:
                print("FAILED")
                if result.stderr:
                    print(result.stderr, file=sys.stderr)
                print(
                    f"ERROR: '{label}' check failed. "
                    "To rollback file changes: git checkout -- Cargo.toml README.md CHANGELOG.md",
                    file=sys.stderr,
                )
                sys.exit(1)


# ---------------------------------------------------------------------------
# Task 7: Commit, tag, and push
# ---------------------------------------------------------------------------

def commit_release(new_version, dry_run):
    """Stage release files and create a release commit."""
    print("[8/9] Committing...")
    if dry_run:
        print("      Skipped (dry-run)")
        return
    run("git add Cargo.toml Cargo.lock README.md CHANGELOG.md")
    run(f'git commit -m "chore: release v{new_version}"')
    print(f"      chore: release v{new_version}")


def tag_and_push(new_version, tag_message, dry_run):
    """Create an annotated tag and push commit + tag to origin."""
    print("[9/9] Tagging and pushing...")
    if dry_run:
        print("      Skipped (dry-run)")
        return

    run(f'git tag -a "v{new_version}" -m "{tag_message}"')
    print(f"      Tagged v{new_version}")

    commit_hash = run("git rev-parse --short HEAD")

    push_result = subprocess.run("git push origin main", shell=True, capture_output=True, text=True)
    if push_result.returncode != 0:
        print(push_result.stderr, file=sys.stderr)
        print(
            f"ERROR: Failed to push to origin/main. "
            f"Commit {commit_hash} was created locally. To undo: git reset HEAD~1",
            file=sys.stderr,
        )
        sys.exit(1)
    print("      Pushed to origin/main")

    tag_result = subprocess.run(f"git push origin v{new_version}", shell=True, capture_output=True, text=True)
    if tag_result.returncode != 0:
        print(tag_result.stderr, file=sys.stderr)
        print(
            f"ERROR: Commit was pushed but tag was not. "
            f"To push tag manually: git push origin v{new_version}",
            file=sys.stderr,
        )
        sys.exit(1)
    print(f"      Pushed tag v{new_version}")

    print(f"\nRelease v{new_version} complete. CI will publish to crates.io.")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    args = parse_args()

    # Read current version and compute new version before preflight
    old_version = read_current_version()
    new_version = bump_version(old_version, args.bump)
    new_tag = f"v{new_version}"

    dry_label = " (dry-run)" if args.dry_run else ""
    print(f"Release{dry_label}: {old_version} → {new_version}  [{args.bump} bump]")
    print()

    # [1/9] Preflight checks
    print("[1/9] Pre-flight checks...")
    preflight_checks(new_tag)

    # [2/9] Version bump summary
    print(f"[2/9] Version: {old_version} → {new_version}")

    # [3/9] Update files
    print("[3/9] Updating version in files...")
    update_cargo_toml(old_version, new_version, args.dry_run)
    update_readme(old_version, new_version, args.bump, args.dry_run)

    # [4/9] Stale version scan
    scan_stale_version(old_version, args.dry_run)

    # [5/9] Update lockfile
    update_lockfile(args.dry_run)

    # [6/9] Changelog
    update_changelog(old_version, new_version, args.dry_run)

    # [7/9] Run checks
    run_checks(args.dry_run, args.skip_integration)

    # [8/9] Commit
    commit_release(new_version, args.dry_run)

    # [9/9] Tag and push
    tag_and_push(new_version, args.tag_message, args.dry_run)

    if args.dry_run:
        print(f"\nDry run complete. To release for real:")
        print(f"  ./release/release.py {args.bump} --tag-message \"<your message>\"")


if __name__ == "__main__":
    main()
