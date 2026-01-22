#!/usr/bin/env bash
# Benchmark Kani verification times for tiny problem spaces
#
# Runs each micro-benchmark multiple times to get statistical data
# that we can use to fit a curve and extrapolate to larger spaces.

set -euo pipefail

RESULTS_FILE="kani_benchmark_results.csv"
ITERATIONS=3

echo "Problem_Space,Time_Seconds,Iteration,Harness_Name" > "$RESULTS_FILE"

# Micro-benchmarks with TINY problem spaces (< 20 combinations)
# Format: "combinations:harness_name"
BENCHMARKS=(
    "1:bench_concrete_1_byte"
    "4:bench_2byte_2x2"
    "6:bench_2byte_2x3"
    "9:bench_2byte_3x3"
    "16:bench_2byte_4x4"
    "16:bench_4byte_2x2x2x2"
)

echo "🔬 Kani Verification Micro-Benchmark Suite"
echo "==========================================="
echo "Running each benchmark $ITERATIONS times for statistical robustness"
echo ""

for benchmark in "${BENCHMARKS[@]}"; do
    IFS=':' read -r problem_space harness_name <<< "$benchmark"
    
    echo "Benchmarking: $harness_name (problem space: $problem_space combinations)"
    
    for iter in $(seq 1 $ITERATIONS); do
        echo -n "  Run $iter/$ITERATIONS... "
        
        start_time=$(date +%s.%N)
        
        # Run Kani, suppress output
        if cargo kani --features verify-kani --harness "$harness_name" >/dev/null 2>&1; then
            end_time=$(date +%s.%N)
            elapsed=$(echo "$end_time - $start_time" | bc)
            
            echo "✅ ${elapsed}s"
            echo "$problem_space,$elapsed,$iter,$harness_name" >> "$RESULTS_FILE"
        else
            echo "❌ FAILED"
            echo "$problem_space,FAILED,$iter,$harness_name" >> "$RESULTS_FILE"
        fi
    done
    
    echo ""
done

echo "==========================================="
echo "📊 Results saved to $RESULTS_FILE"
echo ""
echo "To analyze results in R or Python:"
echo "  import pandas as pd"
echo "  df = pd.read_csv('$RESULTS_FILE')"
echo "  # Fit power law: time = a * space^b"
echo "  # Extrapolate to 3,968 (2-byte full) and 786,432 (4-byte full)"

