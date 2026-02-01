#!/usr/bin/env bash
# Test the tracking system with a small subset

set -euo pipefail

CSV_FILE="kani_test_small.csv"

# Clean up any existing test file
rm -f "$CSV_FILE"

# Create temporary discovery script output with just 3 harnesses
cat > /tmp/test_harnesses.txt <<'EOF'
ipaddr_bytes,verify_ipv4_10_network_is_private
ipaddr_bytes,verify_ipv4_192_168_is_private
macaddr,verify_macaddr_roundtrip
EOF

echo "🧪 Testing Kani Verification Tracking System"
echo "============================================="
echo ""

# Create CSV header
echo "Timestamp,Module,Harness,Status,Time_Seconds,Unwind_Bound,Error_Message" > "$CSV_FILE"
echo "📝 Created: $CSV_FILE"
echo ""

DEFAULT_UNWIND=20
PASSED=0
FAILED=0

# Run each harness
while IFS=, read -r module harness; do
    echo "🔬 Running: $module::$harness"
    
    START=$(date +%s)
    TIMESTAMP=$(date -Iseconds)
    ERROR_MSG=""
    STATUS="SUCCESS"
    LOG_FILE="kani_test_${module}_${harness}.log"
    
    if timeout 60 cargo kani \
        --features verify-kani \
        --harness "$harness" \
        --default-unwind "$DEFAULT_UNWIND" \
        > "$LOG_FILE" 2>&1; then
        STATUS="SUCCESS"
        PASSED=$((PASSED + 1))
        echo "  ✅ Passed"
        rm -f "$LOG_FILE"
    else
        EXIT_CODE=$?
        if [ $EXIT_CODE -eq 124 ]; then
            STATUS="TIMEOUT"
            ERROR_MSG="Timeout after 60s"
        else
            STATUS="FAILED"
            ERROR_MSG=$(grep -E "^(error|FAILED)" "$LOG_FILE" | head -1 | tr ',' ' ' || echo "See $LOG_FILE")
        fi
        FAILED=$((FAILED + 1))
        echo "  ❌ $STATUS: $ERROR_MSG"
    fi
    
    END=$(date +%s)
    ELAPSED=$((END - START))
    
    echo "$TIMESTAMP,$module,$harness,$STATUS,$ELAPSED,$DEFAULT_UNWIND,\"$ERROR_MSG\"" >> "$CSV_FILE"
    echo ""
done < /tmp/test_harnesses.txt

echo "============================================="
echo "📊 Test Results"
echo "============================================="
echo "✅ Passed: $PASSED"
echo "❌ Failed: $FAILED"
echo ""
echo "CSV output:"
cat "$CSV_FILE"

rm -f /tmp/test_harnesses.txt
