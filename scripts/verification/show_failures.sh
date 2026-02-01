#!/usr/bin/env bash
# Show failed tests from verification CSV

set -euo pipefail

CSV_FILE="${1:-kani_verification_results.csv}"

if [ ! -f "$CSV_FILE" ]; then
    echo "❌ No results found: $CSV_FILE"
    exit 1
fi

echo "❌ Failed Kani Verifications"
echo "============================="
echo ""

FAILED=$(tail -n +2 "$CSV_FILE" | grep -vE ",SUCCESS,")

if [ -z "$FAILED" ]; then
    echo "✅ No failures! All tests passed."
    exit 0
fi

echo "$FAILED" | awk -F, '
{
    printf "Module: %s\n", $2
    printf "Harness: %s\n", $3
    printf "Status: %s\n", $4
    printf "Time: %ds\n", $5
    if ($7 != "") printf "Error: %s\n", $7
    printf "\n"
}'

echo "Total failures: $(echo "$FAILED" | wc -l)"
