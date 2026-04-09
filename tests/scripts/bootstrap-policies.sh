#!/usr/bin/env bash
# Bootstrap additional authorization policies for the admin user via the NiFi REST API.
# Called by run.sh after NiFi is ready.
#
# The initial managed-authorizer setup only grants the admin access to:
#   /flow(R), /tenants(R,W), /policies(R,W), /controller(R,W), /restricted-components(W)
#
# This script adds everything else needed by the integration tests.
#
# The admin user UUID (21232f29-7a57-35a7-8389-4a0e4a801fc3) is deterministic:
# it is UUID v3 of "admin" (the value of SINGLE_USER_CREDENTIALS_USERNAME).

set -euo pipefail

NIFI_URL="${NIFI_URL:-https://localhost:8443}"
NIFI_USERNAME="${NIFI_USERNAME:-admin}"
NIFI_PASSWORD="${NIFI_PASSWORD:-adminpassword123}"
# UUID v3 of "admin" — must match SINGLE_USER_CREDENTIALS_USERNAME in docker-compose.yml
ADMIN_USER_ID="21232f29-7a57-35a7-8389-4a0e4a801fc3"

# Login and get JWT token.
TOKEN=$(curl -sk -X POST "${NIFI_URL}/nifi-api/access/token" \
    -H "Content-Type: application/x-www-form-urlencoded" \
    -d "username=${NIFI_USERNAME}&password=${NIFI_PASSWORD}")

if [ -z "${TOKEN}" ]; then
    echo "ERROR: Failed to obtain NiFi token during policy bootstrap" >&2
    exit 1
fi

# Create a policy; silently ignore 409 Conflict (policy already exists).
# HTTP 400 means the resource path is not valid in this NiFi version — skip it silently.
# This handles version differences (e.g. /controller-services only exists in NiFi 2.8+).
create_policy() {
    local resource="$1"
    local action="$2"
    local http_status
    http_status=$(curl -sk -o /dev/null -w "%{http_code}" \
        -X POST "${NIFI_URL}/nifi-api/policies" \
        -H "Content-Type: application/json" \
        -H "Authorization: Bearer ${TOKEN}" \
        -d "{
              \"revision\": {\"version\": 0},
              \"component\": {
                \"resource\": \"${resource}\",
                \"action\": \"${action}\",
                \"users\": [{\"id\": \"${ADMIN_USER_ID}\", \"revision\": {\"version\": 0}}],
                \"userGroups\": []
              }
            }")
    if [ "${http_status}" = "201" ] || [ "${http_status}" = "409" ]; then
        echo "    ${resource} (${action}): ok (HTTP ${http_status})"
    elif [ "${http_status}" = "400" ]; then
        echo "    ${resource} (${action}): skipped (resource not valid in this NiFi version)"
    else
        echo "    WARNING: ${resource} (${action}): unexpected HTTP ${http_status}" >&2
    fi
}

echo "--- Bootstrapping NiFi authorization policies..."

# System diagnostics — required by connect() and readonly tests.
create_policy "/system" "read"

# Resources listing — required by readonly tests.
create_policy "/resources" "read"

# Site-to-site — required by readonly tests.
create_policy "/site-to-site" "read"

# Counters — required by readonly tests.
create_policy "/counters" "read"
create_policy "/counters" "write"

# Provenance — required by provenance tests.
create_policy "/provenance" "read"
create_policy "/provenance" "write"

# Root process group — required by all component CRUD tests (funnels, processors, etc.).
# The root PG UUID is random, so we resolve it here at runtime.
ROOT_PG_ID=$(curl -sk \
    -H "Authorization: Bearer ${TOKEN}" \
    "${NIFI_URL}/nifi-api/flow/process-groups/root" \
    | python3 -c "import sys, json; print(json.load(sys.stdin)['processGroupFlow']['id'])")
echo "    Root process group: ${ROOT_PG_ID}"
create_policy "/process-groups/${ROOT_PG_ID}" "read"
create_policy "/process-groups/${ROOT_PG_ID}" "write"

# Data access (queue listing/drop, provenance event content) — cascades to all components
# inside the root process group, including any nested PGs created during tests.
create_policy "/data/process-groups/${ROOT_PG_ID}" "read"
create_policy "/data/process-groups/${ROOT_PG_ID}" "write"

# Provenance data access (view provenance events per component) — cascades to all components.
# This is separate from /data/ which covers flowfile content; /provenance-data/ controls
# visibility of provenance events in query results.
create_policy "/provenance-data/process-groups/${ROOT_PG_ID}" "read"
create_policy "/provenance-data/process-groups/${ROOT_PG_ID}" "write"

# Parameter contexts — required by parameter context tests.
create_policy "/parameter-contexts" "read"
create_policy "/parameter-contexts" "write"

# Controller services — required by controller service CRUD tests.
create_policy "/controller-services" "read"
create_policy "/controller-services" "write"

echo "--- Policy bootstrap complete."
