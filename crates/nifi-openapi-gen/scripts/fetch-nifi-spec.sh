#!/usr/bin/env bash
# Fetch the NiFi OpenAPI spec.
#
# Usage:
#   ./crates/nifi-openapi-gen/scripts/fetch-nifi-spec.sh
#
# The spec is saved as:
#   $OUT_DIR/<version>/nifi-api.json
#
# For brand-new versions an empty $OUT_DIR/<version>/fn_names.txt golden is also
# seeded (never clobbering an existing one) so the generator's Overwrite step,
# which requires the file to pre-exist, can populate it on the next run.
#
# Environment variables:
#   NIFI_URL         Base URL of NiFi        (default: https://localhost:8443)
#   NIFI_USERNAME    Login username          (default: admin)
#   NIFI_PASSWORD    Login password          (default: adminpassword123)
#   NIFI_VERSION     Target version, e.g. 2.11.0 — required by the dist method
#   NIFI_SOURCE      auto|docker|http|dist   (default: auto — try each in order)
#   NIFI_DIST_ZIP    Path to an already-downloaded nifi-<version>-bin.zip
#   NIFI_DIST_CACHE  Download cache dir      (default: $TMPDIR/nifi-dist-cache)
#   OUT_DIR          Destination directory   (default: <workspace>/crates/nifi-openapi-gen/specs)
#   COMPOSE_FILE     docker-compose.yml      (default: <workspace>/tests/docker-compose.yml)
#
# The version subdirectory is always determined from the artifact the spec was
# read out of — never from NIFI_VERSION directly.
#
# Examples:
#   # Save to the default location inside the workspace:
#   ./crates/nifi-openapi-gen/scripts/fetch-nifi-spec.sh
#
#   # Save to a custom directory (e.g. target/):
#   OUT_DIR=target/specs ./crates/nifi-openapi-gen/scripts/fetch-nifi-spec.sh
#
#   # Use a non-default docker-compose file:
#   COMPOSE_FILE=/other/docker-compose.yml ./crates/nifi-openapi-gen/scripts/fetch-nifi-spec.sh
#
#   # No running NiFi — pull the release archive from apache.org:
#   NIFI_VERSION=2.11.0 NIFI_SOURCE=dist ./crates/nifi-openapi-gen/scripts/fetch-nifi-spec.sh
#
#   # Reuse an archive you already have on disk:
#   NIFI_SOURCE=dist NIFI_DIST_ZIP=~/Downloads/nifi-2.11.0-bin.zip \
#     ./crates/nifi-openapi-gen/scripts/fetch-nifi-spec.sh
#
# The script tries three methods in order:
#   1. Docker exec — reads the spec directly from the running container
#      (most reliable; requires the NiFi service to be running via docker compose)
#   2. HTTP — logs in, gets version from /nifi-api/flow/about, downloads spec
#      (fallback for non-Docker deployments)
#   3. Apache dist — downloads nifi-<version>-bin.zip from apache.org and digs
#      the spec out of it. Needs no running NiFi at all, which makes it the way
#      in when a release is published on downloads.apache.org before its Docker
#      image lands. Requires NIFI_VERSION (or NIFI_DIST_ZIP).

set -euo pipefail

NIFI_URL="${NIFI_URL:-https://localhost:8443}"
NIFI_USERNAME="${NIFI_USERNAME:-admin}"
NIFI_PASSWORD="${NIFI_PASSWORD:-adminpassword123}"
NIFI_VERSION="${NIFI_VERSION:-}"
NIFI_SOURCE="${NIFI_SOURCE:-auto}"
NIFI_DIST_ZIP="${NIFI_DIST_ZIP:-}"
NIFI_DIST_CACHE="${NIFI_DIST_CACHE:-${TMPDIR:-/tmp}/nifi-dist-cache}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
COMPOSE_FILE="${COMPOSE_FILE:-$WORKSPACE_ROOT/tests/docker-compose.yml}"
SPECS_BASE="${OUT_DIR:-$WORKSPACE_ROOT/crates/nifi-openapi-gen/specs}"

case "$NIFI_SOURCE" in
  auto | docker | http | dist) ;;
  *)
    echo "ERROR: NIFI_SOURCE must be one of auto|docker|http|dist (got '$NIFI_SOURCE')." >&2
    exit 1
    ;;
esac

# Scratch dir for the dist method's nested-archive unpacking; set lazily by
# try_dist. Single-quoted so $WORK_DIR is expanded when the trap fires, and
# guarded so an unset WORK_DIR can't turn into `rm -rf ''`.
WORK_DIR=""
trap 'if [[ -n "$WORK_DIR" ]]; then rm -rf "$WORK_DIR"; fi' EXIT

# Seed an empty fn_names.txt golden for brand-new versions so the generator's
# Overwrite step (which requires the file to pre-exist) can populate it.
# Never clobber an existing golden.
seed_golden() {
  [[ -f "$1/fn_names.txt" ]] || : >"$1/fn_names.txt"
}

# True when $1 is the method the caller asked for (or when nothing was pinned).
want_method() {
  [[ "$NIFI_SOURCE" == "auto" || "$NIFI_SOURCE" == "$1" ]]
}

# ── Method 1: Docker exec ────────────────────────────────────────────────────
try_docker() {
  if ! command -v docker &>/dev/null; then
    return 1
  fi
  if ! docker compose -f "$COMPOSE_FILE" ps --services --filter status=running 2>/dev/null |
    grep -q "^nifi$"; then
    return 1
  fi

  echo "NiFi detected in Docker — extracting spec from container..."

  local war_dir
  war_dir=$(docker compose -f "$COMPOSE_FILE" exec -T nifi \
    sh -c 'ls /opt/nifi/nifi-current/work/jetty/ 2>/dev/null | grep "^nifi-web-api-"' |
    tr -d '\r' | head -1)

  if [[ -z "$war_dir" ]]; then
    echo "ERROR: nifi-web-api-*.war directory not found in container." >&2
    echo "NiFi may still be starting up. Check: docker compose -f $COMPOSE_FILE logs nifi" >&2
    exit 1
  fi

  # nifi-web-api-2.8.0.war → 2.8.0
  local version out_dir
  version=$(echo "$war_dir" | sed 's/nifi-web-api-\(.*\)\.war/\1/')
  echo "NiFi version: $version"

  out_dir="$SPECS_BASE/$version"
  mkdir -p "$out_dir"

  docker compose -f "$COMPOSE_FILE" exec -T nifi \
    cat "/opt/nifi/nifi-current/work/jetty/$war_dir/webapp/docs/rest-api/swagger.json" \
    >"$out_dir/nifi-api.json"

  seed_golden "$out_dir"

  echo "Wrote spec to $out_dir/nifi-api.json"
  return 0
}

# ── Method 2: HTTP ───────────────────────────────────────────────────────────
try_http() {
  echo "Trying HTTP against $NIFI_URL ..."

  if ! command -v curl &>/dev/null; then
    echo "  curl is required for HTTP-based extraction — skipping." >&2
    return 1
  fi
  if ! command -v python3 &>/dev/null; then
    echo "  python3 is required for JSON parsing in HTTP mode — skipping." >&2
    return 1
  fi

  echo "Authenticating..."
  local token
  token=$(curl -sk -X POST "$NIFI_URL/nifi-api/access/token" \
    --data-urlencode "username=$NIFI_USERNAME" \
    --data-urlencode "password=$NIFI_PASSWORD")

  if [[ -z "$token" ]]; then
    echo "  login failed — check NIFI_URL, NIFI_USERNAME, NIFI_PASSWORD." >&2
    return 1
  fi

  # Get version from /nifi-api/flow/about  {"about": {"version": "2.8.0", ...}}
  echo "Getting NiFi version..."
  local version
  version=$(curl -sk -H "Authorization: Bearer $token" \
    "$NIFI_URL/nifi-api/flow/about" |
    python3 -c "import sys,json; print(json.load(sys.stdin)['about']['version'])" 2>/dev/null)

  if [[ -z "$version" ]]; then
    echo "  could not determine NiFi version from $NIFI_URL/nifi-api/flow/about" >&2
    return 1
  fi

  echo "NiFi version: $version"

  local out_dir http_status
  out_dir="$SPECS_BASE/$version"
  mkdir -p "$out_dir"

  # The swagger.json is in the war webapp at /nifi-api/docs/rest-api/swagger.json
  http_status=$(curl -sk \
    -H "Authorization: Bearer $token" \
    -o "$out_dir/nifi-api.json" \
    -w "%{http_code}" \
    "$NIFI_URL/nifi-api/docs/rest-api/swagger.json")

  if [[ "$http_status" != "200" ]]; then
    rm -f "$out_dir/nifi-api.json"
    echo "  GET /nifi-api/docs/rest-api/swagger.json returned HTTP $http_status" >&2
    return 1
  fi

  seed_golden "$out_dir"

  echo "Wrote spec to $out_dir/nifi-api.json"
  return 0
}

# ── Method 3: Apache dist archive ────────────────────────────────────────────

# Verify $1 against the .sha512 published next to $2. A missing sha512sum or an
# unreachable checksum file downgrades to a warning; a genuine mismatch fails.
verify_sha512() {
  local file="$1" url="$2" expected actual

  if ! command -v sha512sum &>/dev/null; then
    echo "  WARNING: sha512sum not found — skipping checksum verification." >&2
    return 0
  fi

  expected=$(curl -fsSL "$url" 2>/dev/null | tr -d '\n' | grep -oE '[0-9a-fA-F]{128}' | head -1)
  if [[ -z "$expected" ]]; then
    echo "  WARNING: could not fetch $url — skipping checksum verification." >&2
    return 0
  fi

  actual=$(sha512sum "$file" | cut -d' ' -f1)
  if [[ "${expected,,}" != "${actual,,}" ]]; then
    echo "ERROR: SHA-512 mismatch for $file" >&2
    echo "  expected: ${expected,,}" >&2
    echo "  actual:   ${actual,,}" >&2
    return 1
  fi

  echo "  SHA-512 verified."
  return 0
}

# Download nifi-<version>-bin.zip to $2. Current releases live on
# downloads.apache.org; older ones are moved to archive.apache.org.
download_dist() {
  local version="$1" dest="$2" base url
  for base in "https://downloads.apache.org/nifi" "https://archive.apache.org/dist/nifi"; do
    url="$base/$version/nifi-$version-bin.zip"
    echo "  downloading $url ..."
    if curl -fsSL --retry 3 -o "$dest.part" "$url"; then
      if ! verify_sha512 "$dest.part" "$url.sha512"; then
        rm -f "$dest.part"
        return 1
      fi
      mv "$dest.part" "$dest"
      return 0
    fi
    rm -f "$dest.part"
    echo "  not available at $base"
  done

  echo "ERROR: could not download nifi-$version-bin.zip." >&2
  echo "       Check that $version is a released NiFi version." >&2
  return 1
}

try_dist() {
  local tool
  for tool in curl unzip; do
    if ! command -v "$tool" &>/dev/null; then
      echo "  $tool is required for dist extraction — skipping." >&2
      return 1
    fi
  done

  local zip="$NIFI_DIST_ZIP"
  if [[ -n "$zip" ]]; then
    if [[ ! -f "$zip" ]]; then
      echo "ERROR: NIFI_DIST_ZIP=$zip does not exist." >&2
      return 1
    fi
    echo "Extracting spec from $zip ..."
  else
    if [[ -z "$NIFI_VERSION" ]]; then
      echo "  dist extraction needs NIFI_VERSION (e.g. NIFI_VERSION=2.11.0) or NIFI_DIST_ZIP — skipping." >&2
      return 1
    fi
    echo "Extracting spec from the Apache release archive for $NIFI_VERSION ..."
    mkdir -p "$NIFI_DIST_CACHE"
    zip="$NIFI_DIST_CACHE/nifi-$NIFI_VERSION-bin.zip"
    if [[ -f "$zip" ]]; then
      echo "  using cached $zip"
    elif ! download_dist "$NIFI_VERSION" "$zip"; then
      return 1
    fi
  fi

  # The spec is nested three archives deep:
  #   nifi-<v>-bin.zip
  #     └ lib/nifi-server-nar-<v>.nar
  #         └ META-INF/bundled-dependencies/nifi-web-api-<v>.war
  #             └ docs/rest-api/swagger.json
  local nar_entry
  nar_entry=$(unzip -Z1 "$zip" '*/lib/nifi-server-nar-*.nar' 2>/dev/null | head -1)
  if [[ -z "$nar_entry" ]]; then
    echo "ERROR: nifi-server-nar-*.nar not found in $zip" >&2
    return 1
  fi

  # nifi-2.11.0/lib/nifi-server-nar-2.11.0.nar → 2.11.0
  local version
  version=$(basename "$nar_entry" | sed 's/nifi-server-nar-\(.*\)\.nar/\1/')
  echo "NiFi version: $version"

  WORK_DIR=$(mktemp -d)
  unzip -p "$zip" "$nar_entry" >"$WORK_DIR/server.nar"

  local war_entry
  war_entry=$(unzip -Z1 "$WORK_DIR/server.nar" '*/nifi-web-api-*.war' 2>/dev/null | head -1)
  if [[ -z "$war_entry" ]]; then
    echo "ERROR: nifi-web-api-*.war not found in $nar_entry" >&2
    return 1
  fi
  unzip -p "$WORK_DIR/server.nar" "$war_entry" >"$WORK_DIR/nifi-web-api.war"

  local out_dir
  out_dir="$SPECS_BASE/$version"
  mkdir -p "$out_dir"

  if ! unzip -p "$WORK_DIR/nifi-web-api.war" 'docs/rest-api/swagger.json' \
    >"$out_dir/nifi-api.json"; then
    rm -f "$out_dir/nifi-api.json"
    echo "ERROR: docs/rest-api/swagger.json not found in $war_entry" >&2
    return 1
  fi

  seed_golden "$out_dir"

  echo "Wrote spec to $out_dir/nifi-api.json"
  return 0
}

# ── Driver ───────────────────────────────────────────────────────────────────
if want_method docker && try_docker; then
  exit 0
fi
if want_method http && try_http; then
  exit 0
fi
if want_method dist && try_dist; then
  exit 0
fi

echo "" >&2
echo "ERROR: could not fetch the NiFi OpenAPI spec (NIFI_SOURCE=$NIFI_SOURCE)." >&2
echo "" >&2
echo "Start NiFi via Docker and retry:" >&2
echo "  docker compose -f $COMPOSE_FILE up -d" >&2
echo "  ./crates/nifi-openapi-gen/scripts/fetch-nifi-spec.sh" >&2
echo "" >&2
echo "Or pull the release archive from apache.org (no running NiFi needed):" >&2
echo "  NIFI_VERSION=x.y.z NIFI_SOURCE=dist ./crates/nifi-openapi-gen/scripts/fetch-nifi-spec.sh" >&2
exit 1
