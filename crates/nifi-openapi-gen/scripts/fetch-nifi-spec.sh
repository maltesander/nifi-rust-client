#!/usr/bin/env bash
# Fetch the NiFi OpenAPI spec from a running NiFi instance.
#
# Usage:
#   ./crates/nifi-openapi-gen/scripts/fetch-nifi-spec.sh
#
# The spec is saved as:
#   $OUT_DIR/<version>/nifi-api.json
#
# Environment variables:
#   NIFI_URL       Base URL of NiFi       (default: https://localhost:8443)
#   NIFI_USERNAME  Login username         (default: admin)
#   NIFI_PASSWORD  Login password         (default: adminpassword123)
#   OUT_DIR        Destination directory  (default: <workspace>/crates/nifi-openapi-gen/specs)
#   COMPOSE_FILE   docker-compose.yml     (default: <workspace>/tests/docker-compose.yml)
#
# The version subdirectory is always determined from the running NiFi instance.
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
# The script tries two methods in order:
#   1. Docker exec — reads the spec directly from the running container
#      (most reliable; requires the NiFi service to be running via docker compose)
#   2. HTTP — logs in, gets version from /nifi-api/flow/about, downloads spec
#      (fallback for non-Docker deployments)

set -euo pipefail

NIFI_URL="${NIFI_URL:-https://localhost:8443}"
NIFI_USERNAME="${NIFI_USERNAME:-admin}"
NIFI_PASSWORD="${NIFI_PASSWORD:-adminpassword123}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
COMPOSE_FILE="${COMPOSE_FILE:-$WORKSPACE_ROOT/tests/docker-compose.yml}"
SPECS_BASE="${OUT_DIR:-$WORKSPACE_ROOT/crates/nifi-openapi-gen/specs}"

# ── Method 1: Docker exec ────────────────────────────────────────────────────
if command -v docker &>/dev/null && \
   docker compose -f "$COMPOSE_FILE" ps --services --filter status=running 2>/dev/null \
     | grep -q "^nifi$"; then

  echo "NiFi detected in Docker — extracting spec from container..."

  WAR_DIR=$(docker compose -f "$COMPOSE_FILE" exec -T nifi \
    sh -c 'ls /opt/nifi/nifi-current/work/jetty/ 2>/dev/null | grep "^nifi-web-api-"' \
    | tr -d '\r' | head -1)

  if [[ -z "$WAR_DIR" ]]; then
    echo "ERROR: nifi-web-api-*.war directory not found in container." >&2
    echo "NiFi may still be starting up. Check: docker compose -f $COMPOSE_FILE logs nifi" >&2
    exit 1
  fi

  # nifi-web-api-2.8.0.war → 2.8.0
  VERSION=$(echo "$WAR_DIR" | sed 's/nifi-web-api-\(.*\)\.war/\1/')
  echo "NiFi version: $VERSION"

  OUT_DIR="$SPECS_BASE/$VERSION"
  mkdir -p "$OUT_DIR"

  docker compose -f "$COMPOSE_FILE" exec -T nifi \
    cat "/opt/nifi/nifi-current/work/jetty/$WAR_DIR/webapp/docs/rest-api/swagger.json" \
    > "$OUT_DIR/nifi-api.json"

  echo "Wrote spec to $OUT_DIR/nifi-api.json"
  exit 0
fi

# ── Method 2: HTTP ───────────────────────────────────────────────────────────
echo "Docker method unavailable — trying HTTP against $NIFI_URL ..."

if ! command -v curl &>/dev/null; then
  echo "ERROR: curl is required for HTTP-based extraction." >&2
  exit 1
fi
if ! command -v python3 &>/dev/null; then
  echo "ERROR: python3 is required for JSON parsing in HTTP mode." >&2
  exit 1
fi

echo "Authenticating..."
TOKEN=$(curl -sk -X POST "$NIFI_URL/nifi-api/access/token" \
  --data-urlencode "username=$NIFI_USERNAME" \
  --data-urlencode "password=$NIFI_PASSWORD")

if [[ -z "$TOKEN" ]]; then
  echo "ERROR: login failed — check NIFI_URL, NIFI_USERNAME, NIFI_PASSWORD." >&2
  echo "       Make sure NiFi is running and reachable at $NIFI_URL" >&2
  exit 1
fi

# Get version from /nifi-api/flow/about  {"about": {"version": "2.8.0", ...}}
echo "Getting NiFi version..."
VERSION=$(curl -sk -H "Authorization: Bearer $TOKEN" \
  "$NIFI_URL/nifi-api/flow/about" \
  | python3 -c "import sys,json; print(json.load(sys.stdin)['about']['version'])" 2>/dev/null)

if [[ -z "$VERSION" ]]; then
  echo "ERROR: could not determine NiFi version from $NIFI_URL/nifi-api/flow/about" >&2
  exit 1
fi

echo "NiFi version: $VERSION"

OUT_DIR="$SPECS_BASE/$VERSION"
mkdir -p "$OUT_DIR"

# The swagger.json is in the war webapp at /nifi-api/docs/rest-api/swagger.json
HTTP_STATUS=$(curl -sk \
  -H "Authorization: Bearer $TOKEN" \
  -o "$OUT_DIR/nifi-api.json" \
  -w "%{http_code}" \
  "$NIFI_URL/nifi-api/docs/rest-api/swagger.json")

if [[ "$HTTP_STATUS" == "200" ]]; then
  echo "Wrote spec to $OUT_DIR/nifi-api.json"
  exit 0
fi

rm -f "$OUT_DIR/nifi-api.json"
echo "ERROR: GET /nifi-api/docs/rest-api/swagger.json returned HTTP $HTTP_STATUS" >&2
echo "" >&2
echo "HTTP extraction failed. Start NiFi via Docker and retry:" >&2
echo "  docker compose -f $COMPOSE_FILE up -d" >&2
echo "  ./crates/nifi-openapi-gen/scripts/fetch-nifi-spec.sh" >&2
exit 1
