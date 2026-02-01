#!/usr/bin/env bash
# Run all Kani proof harnesses individually and track results in CSV
#
# Usage: ./run_tracked_verification.sh [--resume] [--timeout SECONDS]

set -euo pipefail

# Configuration
CSV_FILE="${CSV_FILE:-kani_verification_results.csv}"
TIMEOUT="${TIMEOUT:-300}"  # 5 minutes default
RESUME=false
DEFAULT_UNWIND=20

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --resume)
            RESUME=true
            shift
            ;;
        --timeout)
            TIMEOUT="$2"
            shift 2
            ;;
        --csv)
            CSV_FILE="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: $0 [--resume] [--timeout SECONDS] [--csv FILE]"
            exit 1
            ;;
    esac
done

# Create CSV header if doesn't exist
if [ ! -f "$CSV_FILE" ]; then
    echo "Timestamp,Module,Harness,Status,Time_Seconds,Unwind_Bound,Error_Message" > "$CSV_FILE"
    echo "📝 Created tracking file: $CSV_FILE"
fi

# Discover all harnesses
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
HARNESSES=$("$SCRIPT_DIR/discover_harnesses.sh")
TOTAL=$(echo "$HARNESSES" | wc -l)

echo "🔬 Kani Verification Tracking"
echo "=============================="
echo "Total harnesses: $TOTAL"
echo "CSV output: $CSV_FILE"
echo "Timeout per test: ${TIMEOUT}s"
echo "Resume mode: $RESUME"
echo ""

# Statistics
PASSED=0
FAILED=0
SKIPPED=0

# Run each harness
CURRENT=0
echo "$HARNESSES" | while IFS=, read -r module harness; do
    CURRENT=$((CURRENT + 1))
    
    # Check if already passed (resume mode)
    if [ "$RESUME" = true ] && grep -q "^[^,]*,$module,$harness,SUCCESS," "$CSV_FILE" 2>/dev/null; then
        echo "[$CURRENT/$TOTAL] ⏭️  Skipped (cached): $module::$harness"
        continue
    fi
    
    echo "[$CURRENT/$TOTAL] 🔬 Running: $module::$harness"
    
    # Run the verification
    START=$(date +%s)
    TIMESTAMP=$(date -Iseconds)
    ERROR_MSG=""
    STATUS="SUCCESS"
    
    # Create log file name
    LOG_FILE="kani_verify_${module}_${harness}.log"
    
    # Run with timeout
    if timeout "$TIMEOUT" cargo kani \
        --features verify-kani \
        --harness "$harness" \
        --default-unwind "$DEFAULT_UNWIND" \
        > "$LOG_FILE" 2>&1; then
        STATUS="SUCCESS"
        PASSED=$((PASSED + 1))
        echo "[$CURRENT/$TOTAL] ✅ Passed: $module::$harness"
    else
        EXIT_CODE=$?
        if [ $EXIT_CODE -eq 124 ]; then
            STATUS="TIMEOUT"
            ERROR_MSG="Timeout after ${TIMEOUT}s"
        else
            STATUS="FAILED"
            # Extract first error line from log
            ERROR_MSG=$(grep -E "^(error|FAILED|assertion failed)" "$LOG_FILE" | head -1 | tr ',' ' ' || echo "See log: $LOG_FILE")
        fi
        FAILED=$((FAILED + 1))
        echo "[$CURRENT/$TOTAL] ❌ $STATUS: $module::$harness"
        if [ -n "$ERROR_MSG" ]; then
            echo "    Error: $ERROR_MSG"
        fi
    fi
    
    END=$(date +%s)
    ELAPSED=$((END - START))
    
    # Record result
    echo "$TIMESTAMP,$module,$harness,$STATUS,$ELAPSED,$DEFAULT_UNWIND,\"$ERROR_MSG\"" >> "$CSV_FILE"
    
    # Clean up log file if successful
    if [ "$STATUS" = "SUCCESS" ]; then
        rm -f "$LOG_FILE"
    fi
    
    echo ""
done

# Final summary
echo "=============================="
echo "✅ Passed: $PASSED"
echo "❌ Failed: $FAILED"
echo "⏭️  Skipped: $SKIPPED"
echo ""
echo "📊 Results saved to: $CSV_FILE"

if [ $FAILED -gt 0 ]; then
    echo ""
    echo "Failed tests:"
    tail -n +2 "$CSV_FILE" | grep -E "FAILED|TIMEOUT|ERROR" | \
        awk -F, '{printf "  ❌ %s::%s (%s)\n", $2, $3, $4}'
    exit 1
fi
