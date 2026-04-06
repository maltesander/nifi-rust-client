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
        echo "--- Stopping NiFi..."
        docker compose -f "$COMPOSE_FILE" down
    else
        echo "--- Skipping cleanup (--skip-cleanup). NiFi is still running."
        echo "    Stop manually: docker compose -f $COMPOSE_FILE down"
    fi
}
trap cleanup EXIT

# Generate CA + server cert + PKCS12 keystore/truststore if not already present.
# Certs are reused across runs when --skip-cleanup is active.
if [[ ! -f "$CERTS_DIR/keystore.p12" ]]; then
    "$SCRIPT_DIR/scripts/generate-certs.sh"
else
    echo "--- Reusing existing certificates in tests/scripts/certs/"
fi

echo "--- Starting NiFi..."
NIFI_IMAGE_TAG="$NIFI_IMAGE_TAG" docker compose -f "$COMPOSE_FILE" up -d

echo "--- Waiting for NiFi to be ready (this takes ~90s)..."
MAX_WAIT=180
WAITED=0
until docker compose -f "$COMPOSE_FILE" exec -T nifi \
        grep -q 'Started Server on https://' /opt/nifi/nifi-current/logs/nifi-app.log 2>/dev/null; do
    if [[ $WAITED -ge $MAX_WAIT ]]; then
        echo "ERROR: NiFi did not become ready within ${MAX_WAIT}s"
        exit 1
    fi
    sleep 5
    WAITED=$((WAITED + 5))
    echo "    ... ${WAITED}s elapsed"
done
echo "--- NiFi is ready."

echo "--- Bootstrapping authorization policies..."
NIFI_URL="$NIFI_URL" NIFI_USERNAME="$NIFI_USERNAME" NIFI_PASSWORD="$NIFI_PASSWORD" \
    "$SCRIPT_DIR/scripts/bootstrap-policies.sh"

echo "--- Running integration tests..."
NIFI_URL="$NIFI_URL" \
NIFI_USERNAME="$NIFI_USERNAME" \
NIFI_PASSWORD="$NIFI_PASSWORD" \
NIFI_CA_CERT_PATH="$CERTS_DIR/ca.crt" \
    cargo test -p nifi-integration-tests --no-default-features --features "$NIFI_FEATURE" \
    -- --ignored --nocapture

echo "--- All tests passed."
