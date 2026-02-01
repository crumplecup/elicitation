# Kani Verification Tracking System

This directory contains scripts for running and tracking Kani formal verification proofs with CSV-based checkpointing and progress monitoring.

## Overview

The tracking system runs each of the 104+ Kani proof harnesses individually and records results in a CSV file, providing:
- Individual test visibility (know exactly which harness passed/failed)
- Historical tracking of verification runs
- Resume capability (skip already-passed tests)
- Summary statistics and failure reports
- Audit trail with timestamps and durations

## Quick Start

```bash
# List all available proof harnesses
just verify-kani-list

# Run all proofs with tracking (recommended)
just verify-kani-tracked

# Show summary statistics
just verify-kani-summary

# Show only failed tests
just verify-kani-failed

# Resume after interruption (skips passed tests)
just verify-kani-resume
```

## Scripts

### discover_harnesses.sh
Scans the codebase and outputs all Kani proof harnesses in CSV format:
```
MODULE,HARNESS_NAME
ipaddr_bytes,verify_ipv4_10_network_is_private
macaddr,verify_macaddr_roundtrip
...
```

### run_tracked_verification.sh
Main verification runner that:
- Discovers all harnesses
- Runs each individually with `cargo kani`
- Records results in CSV format
- Handles timeouts and failures gracefully
- Supports resume mode to skip already-passed tests

Usage:
```bash
# Default (300s timeout per test)
./run_tracked_verification.sh

# Custom timeout
./run_tracked_verification.sh --timeout 600

# Resume mode (skip passed tests)
./run_tracked_verification.sh --resume

# Custom CSV file
./run_tracked_verification.sh --csv my_results.csv
```

### show_summary.sh
Displays summary statistics from a verification run:
- Total tests, pass/fail counts, pass rate
- Timing statistics (min/max/avg)
- Results by module
- Recent failures

### show_failures.sh
Lists only failed tests with details (harness, status, error message).

## CSV Schema

```csv
Timestamp,Module,Harness,Status,Time_Seconds,Unwind_Bound,Error_Message
2026-02-01T10:02:01-08:00,ipaddr_bytes,verify_ipv4_10_network_is_private,SUCCESS,9,20,""
2026-02-01T10:02:28-08:00,macaddr,verify_macaddr_roundtrip,FAILED,13,20,"unwinding assertion loop 0"
```

Fields:
- **Timestamp**: ISO 8601 format with timezone
- **Module**: Proof file name (without .rs extension)
- **Harness**: Function name of the proof
- **Status**: `SUCCESS`, `FAILED`, `TIMEOUT`, or `ERROR`
- **Time_Seconds**: Duration of verification in seconds
- **Unwind_Bound**: Kani unwind value used (default 20)
- **Error_Message**: First error line (if failed) or empty

## Justfile Recipes

### verify-kani
Original batch mode - runs all proofs in one `cargo kani` invocation. Fast but hides individual failures.

### verify-kani-tracked
**Recommended** - Runs all proofs individually with CSV tracking. Accepts optional parameters:
```bash
just verify-kani-tracked                    # Default settings
just verify-kani-tracked custom_output.csv  # Custom CSV file
just verify-kani-tracked results.csv 600    # Custom CSV and timeout
```

### verify-kani-resume
Resumes verification, skipping tests that already passed in the CSV file:
```bash
just verify-kani-resume                     # Default CSV
just verify-kani-resume custom_output.csv   # Custom CSV
```

### verify-kani-summary
Show summary statistics:
```bash
just verify-kani-summary                    # Default CSV
just verify-kani-summary custom_output.csv  # Custom CSV
```

### verify-kani-failed
List only failed tests:
```bash
just verify-kani-failed                     # Default CSV
just verify-kani-failed custom_output.csv   # Custom CSV
```

### verify-kani-list
List all discovered proof harnesses (no verification, just discovery).

## Workflow Examples

### Initial full verification run
```bash
# Run all tests with tracking
just verify-kani-tracked

# Check summary
just verify-kani-summary

# If there are failures
just verify-kani-failed
```

### After fixing failures
```bash
# Resume (skips already-passed tests)
just verify-kani-resume

# Or re-run specific harness
cargo kani --features verify-kani --harness verify_specific_test --default-unwind 20
```

### Continuous integration
```bash
# Run with shorter timeout for CI
CSV_FILE=ci_results.csv TIMEOUT=180 ./scripts/verification/run_tracked_verification.sh

# Generate report
./scripts/verification/show_summary.sh ci_results.csv
```

## Log Files

For each failed test, a log file is created: `kani_verify_{module}_{harness}.log`

Successful tests have their logs automatically cleaned up to reduce clutter.

## Configuration

Environment variables:
- `CSV_FILE`: Output CSV filename (default: `kani_verification_results.csv`)
- `TIMEOUT`: Timeout per test in seconds (default: `300`)

Kani settings:
- Default unwind bound: `20`
- Feature flag: `verify-kani`

## Troubleshooting

**"No harnesses found"**
- Ensure you're in the repository root
- Check that `crates/elicitation/src/verification/types/kani_proofs/` exists

**"Compilation failed"**
- Run `cargo check --features verify-kani` to identify issues
- Fix compilation errors before running verifications

**"Timeout on many tests"**
- Increase timeout: `just verify-kani-tracked results.csv 600`
- Some proofs may require higher unwind bounds (edit harness directly)

**"Resume not skipping tests"**
- Ensure CSV file exists and has correct format
- Check that harness names match exactly

## Comparison: Batch vs Tracked

| Feature | Batch Mode (`verify-kani`) | Tracked Mode (`verify-kani-tracked`) |
|---------|----------------------------|--------------------------------------|
| Startup overhead | Low (one invocation) | Higher (104+ invocations) |
| Failure visibility | Hidden in logs | Immediate, per-test |
| Progress tracking | None | CSV audit trail |
| Resume capability | No | Yes |
| Historical data | No | Yes (timestamped CSV) |
| Best for | Quick spot-checks | CI/CD, tracking, debugging |

## Future Enhancements

Potential improvements:
- Parallel execution (GNU parallel or xargs -P)
- HTML report generation
- Integration with GitHub Actions artifacts
- Automatic retry logic for flaky tests
- Time-series trending (track pass rate over commits)
