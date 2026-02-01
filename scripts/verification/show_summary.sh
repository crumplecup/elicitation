#!/usr/bin/env bash
# Show summary statistics from verification CSV

set -euo pipefail

CSV_FILE="${1:-kani_verification_results.csv}"

if [ ! -f "$CSV_FILE" ]; then
    echo "❌ No results found: $CSV_FILE"
    echo ""
    echo "Run verification first with: just verify-kani-tracked"
    exit 1
fi

echo "📊 Kani Verification Summary"
echo "============================="
echo "Source: $CSV_FILE"
echo ""

# Count results by status
TOTAL=$(tail -n +2 "$CSV_FILE" | wc -l)
SUCCESS=$(tail -n +2 "$CSV_FILE" | grep -c ",SUCCESS," || echo 0)
FAILED=$(tail -n +2 "$CSV_FILE" | grep -c ",FAILED," || echo 0)
TIMEOUT=$(tail -n +2 "$CSV_FILE" | grep -c ",TIMEOUT," || echo 0)
ERROR=$(tail -n +2 "$CSV_FILE" | grep -c ",ERROR," || echo 0)

echo "Total runs: $TOTAL"
echo "✅ Passed: $SUCCESS"
echo "❌ Failed: $FAILED"
echo "⏱️  Timeout: $TIMEOUT"
echo "💥 Error: $ERROR"

if [ $TOTAL -gt 0 ]; then
    PASS_RATE=$(awk "BEGIN {printf \"%.1f\", ($SUCCESS / $TOTAL) * 100}")
    echo ""
    echo "Pass rate: ${PASS_RATE}%"
fi

# Time statistics
if [ $SUCCESS -gt 0 ]; then
    echo ""
    echo "Timing (successful tests):"
    tail -n +2 "$CSV_FILE" | grep ",SUCCESS," | \
        awk -F, '{sum+=$5; if(NR==1){min=max=$5} if($5<min){min=$5} if($5>max){max=$5}} 
                 END {printf "  Average: %.1fs\n  Min: %ds\n  Max: %ds\n  Total: %ds\n", 
                      sum/NR, min, max, sum}'
fi

# Module breakdown
echo ""
echo "Results by module:"
tail -n +2 "$CSV_FILE" | awk -F, '
{
    module=$2
    status=$4
    counts[module]++
    if (status == "SUCCESS") success[module]++
}
END {
    for (m in counts) {
        pass = (m in success) ? success[m] : 0
        printf "  %s: %d/%d passed\n", m, pass, counts[m]
    }
}' | sort

# Recent failures
RECENT_FAIL=$(tail -n +2 "$CSV_FILE" | grep -E ",FAILED,|,TIMEOUT,|,ERROR," | tail -5)
if [ -n "$RECENT_FAIL" ]; then
    echo ""
    echo "Recent failures:"
    echo "$RECENT_FAIL" | awk -F, '{printf "  ❌ %s::%s (%s) - %s\n", $2, $3, $4, $7}'
fi
