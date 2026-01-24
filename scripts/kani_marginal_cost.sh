#!/usr/bin/env bash
# Measure Kani's marginal verification cost per combination
#
# Strategy: Run all micro-benchmarks in a single Kani invocation to share warmup cost,
# then extract marginal time differences between problem space sizes.

set -euo pipefail

echo "🔬 Kani Marginal Cost Analysis"
echo "==============================="
echo ""
echo "Running all benchmarks sequentially (first includes warmup)..."
echo ""

# Run Kani on all benchmark harnesses, capture full output
cargo kani --features verify-kani \
    --harness bench_concrete_1_byte \
    --harness bench_2byte_2x2 \
    --harness bench_2byte_2x3 \
    --harness bench_2byte_3x3 \
    --harness bench_2byte_4x4 \
    --harness bench_4byte_2x2x2x2 \
    2>&1 | tee kani_marginal_benchmark.log

echo ""
echo "==============================="
echo "📊 Extracting verification times..."
echo ""

# Parse verification times
grep -A1 "Verification Time:" kani_marginal_benchmark.log | grep -v "^--$" > times.tmp || true

# Display results
cat times.tmp

echo ""
echo "Saved to kani_marginal_benchmark.log"
echo ""
echo "To analyze marginal costs:"
echo "  - First time includes ~10min warmup (compilation)"
echo "  - Delta between consecutive runs = marginal verification cost"
echo "  - Cost per combination = delta_time / delta_combinations"
