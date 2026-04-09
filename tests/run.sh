#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPOSE_FILE="$SCRIPT_DIR/docker-compose.yml"
CERTS_DIR="$SCRIPT_DIR/scripts/certs"
NIFI_URL="${NIFI_URL:-https://localhost:8443}"
NIFI_USERNAME="${NIFI_USERNAME:-admin}"
NIFI_PASSWORD="${NIFI_PASSWORD:-adminpassword123}"
NIFI_VERSION="2.8.0"
SKIP_CLEANUP=false

for arg in "$@"; do
    case "$arg" in
        --skip-cleanup) SKIP_CLEANUP=true ;;
        --nifi-version=*) NIFI_VERSION="${arg#--nifi-version=}" ;;
        --nifi-version) echo "Error: --nifi-version requires a value (e.g. --nifi-version=2.7.2)"; exit 1 ;;
    esac
done

NIFI_IMAGE_TAG="$NIFI_VERSION"
NIFI_FEATURE="nifi-$(echo "$NIFI_VERSION" | tr '.' '-')"

cleanup() {
    if [[ "$SKIP_CLEANUP" == "false" ]]; then
        echo ""
        echo "--- Stopping NiFi..."
        docker compose -f "$COMPOSE_FILE" down
    else
        echo ""
        echo "--- Skipping cleanup (--skip-cleanup). NiFi is still running."
        echo "    Stop manually: docker compose -f $COMPOSE_FILE down"
    fi
}
trap cleanup EXIT

# ── Helpers ──────────────────────────────────────────────────────────────────

start_nifi() {
    local label="${1:-}"
    echo ""
    echo "=== Starting NiFi $NIFI_VERSION${label:+ ($label)} ==="
    NIFI_IMAGE_TAG="$NIFI_IMAGE_TAG" docker compose -f "$COMPOSE_FILE" up -d

    echo "    Waiting for NiFi to be ready..."
    local max_wait=180 waited=0
    until docker compose -f "$COMPOSE_FILE" exec -T nifi \
            grep -q 'Started Server on https://' /opt/nifi/nifi-current/logs/nifi-app.log 2>/dev/null; do
        if [[ $waited -ge $max_wait ]]; then
            echo "    ERROR: NiFi did not become ready within ${max_wait}s"
            exit 1
        fi
        sleep 5
        waited=$((waited + 5))
        printf "    ... %ds\r" "$waited"
    done
    echo "    NiFi ready after ${waited}s.                "

    echo "    Bootstrapping authorization policies..."
    NIFI_URL="$NIFI_URL" NIFI_USERNAME="$NIFI_USERNAME" NIFI_PASSWORD="$NIFI_PASSWORD" \
        "$SCRIPT_DIR/scripts/bootstrap-policies.sh"

    # Clear cached tokens from any previous NiFi instance.
    rm -f /tmp/nifi_test_token_*
}

stop_nifi() {
    echo ""
    echo "--- Tearing down NiFi..."
    docker compose -f "$COMPOSE_FILE" down
}

run_tests() {
    local mode="$1" features="$2"
    echo ""
    echo "--- Running integration tests ($mode, features=$features)..."
    NIFI_URL="$NIFI_URL" \
    NIFI_USERNAME="$NIFI_USERNAME" \
    NIFI_PASSWORD="$NIFI_PASSWORD" \
    NIFI_CA_CERT_PATH="$CERTS_DIR/ca.crt" \
        cargo test -p nifi-integration-tests --no-default-features --features "$features" \
        -- --ignored --nocapture
}

# ── Certificates ─────────────────────────────────────────────────────────────

if [[ ! -f "$CERTS_DIR/keystore.p12" ]]; then
    "$SCRIPT_DIR/scripts/generate-certs.sh"
else
    echo "--- Reusing existing certificates in tests/scripts/certs/"
fi

# ── Phase 1: Static tests ───────────────────────────────────────────────────

start_nifi "static tests"
run_tests "static" "$NIFI_FEATURE"

# ── Phase 2: Dynamic tests on a fresh NiFi instance ─────────────────────────
# A separate NiFi instance avoids provenance index interference between the
# static provenance test and the dynamic field-presence test.

stop_nifi
start_nifi "dynamic tests"
run_tests "dynamic" "$NIFI_FEATURE,dynamic"

echo ""
echo "=== All NiFi $NIFI_VERSION tests passed. ==="
