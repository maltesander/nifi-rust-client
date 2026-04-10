#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SPECS_DIR="$SCRIPT_DIR/../crates/nifi-openapi-gen/specs"
FILTER_VERSIONS=""

for arg in "$@"; do
    case "$arg" in
        --versions=*) FILTER_VERSIONS="${arg#--versions=}" ;;
        --versions) echo "Error: --versions requires a value (e.g. --versions=2.8.0,2.9.0)"; exit 1 ;;
        --help|-h)
            AVAILABLE="$(ls "$SPECS_DIR" | sort -t. -k1,1n -k2,2n -k3,3n | tr '\n' ',' | sed 's/,$//')"
            echo "Usage: $0 [--versions=${AVAILABLE}]"
            echo ""
            echo "Runs integration tests against each supported NiFi version."
            echo "Discovers versions from specs/ directory by default."
            echo ""
            echo "Options:"
            echo "  --versions=V1,V2,...  Only test these versions (comma-separated)"
            exit 0
            ;;
    esac
done

# Discover versions from specs directory, sorted by semver
if [[ -n "$FILTER_VERSIONS" ]]; then
    IFS=',' read -ra VERSIONS <<< "$FILTER_VERSIONS"
else
    mapfile -t VERSIONS < <(
        for dir in "$SPECS_DIR"/*/; do
            basename "$dir"
        done | sort -t. -k1,1n -k2,2n -k3,3n
    )
fi

if [[ ${#VERSIONS[@]} -eq 0 ]]; then
    echo "ERROR: No spec versions found in $SPECS_DIR"
    exit 1
fi

echo "=== NiFi Integration Test Matrix ==="
echo "Versions: ${VERSIONS[*]}"
echo ""

declare -A RESULTS

FAILED=0
for version in "${VERSIONS[@]}"; do
    echo "=== Testing NiFi $version ==="
    if "$SCRIPT_DIR/run.sh" --nifi-version="$version"; then
        RESULTS[$version]="PASS"
        echo "=== NiFi $version: PASS ==="
    else
        RESULTS[$version]="FAIL"
        FAILED=$((FAILED + 1))
        echo "=== NiFi $version: FAIL ==="
    fi
    echo ""
done

echo "=== Matrix Summary ==="
printf "%-12s %s\n" "Version" "Result"
printf "%-12s %s\n" "-------" "------"
for version in "${VERSIONS[@]}"; do
    printf "%-12s %s\n" "$version" "${RESULTS[$version]}"
done
echo ""

if [[ $FAILED -gt 0 ]]; then
    echo "FAILED: $FAILED version(s) failed."
    exit 1
else
    echo "ALL PASSED: ${#VERSIONS[@]} version(s) tested."
fi
