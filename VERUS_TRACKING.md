# Verus Verification Tracking System

This document describes the Verus proof tracking system for the elicitation project, which parallels the Kani verification tracking infrastructure.

## Overview

The Verus tracking system provides:

- **CSV-based tracking** of verification results
- **Resume capability** to skip already-passed proofs
- **Summary statistics** for verification coverage
- **Failed proof analysis** to identify issues
- **CLI commands** and **justfile recipes** for easy execution

## Quick Start

### List All Proofs

```bash
# Via justfile (recommended)
just verify-verus-list

# Via cargo directly
cargo run --features cli --release -- verus list
```

Output shows all 85 Verus proofs organized by module.

### Run Verification with Tracking

```bash
# Run all proofs with CSV tracking
just verify-verus-tracked

# Custom CSV file and timeout
just verify-verus-tracked verus_results.csv 900

# Resume (skip already-passed proofs)
just verify-verus-resume

# Custom CSV for resume
just verify-verus-resume verus_results.csv
```

### View Results

```bash
# Show summary statistics
just verify-verus-summary

# Show only failed proofs
just verify-verus-failed

# Custom CSV file
just verify-verus-summary my_results.csv
just verify-verus-failed my_results.csv
```

## CLI Commands

The tracking system is accessed via the CLI with the `cli` feature:

```bash
cargo run --features cli --release -- verus <subcommand>
```

### Available Subcommands

#### `verus list`

Lists all available Verus proofs grouped by module.

**Example:**

```bash
cargo run --features cli --release -- verus list
```

#### `verus run`

Runs all Verus proofs and tracks results in CSV.

**Options:**

- `-o, --output <FILE>` - CSV output file (default: `verus_verification_results.csv`)
- `-t, --timeout <SECS>` - Timeout per proof in seconds (default: 600)
- `-r, --resume` - Skip already-passed proofs from previous run
- `--verus-path <PATH>` - Path to Verus binary (overrides VERUS_PATH env var)

**Examples:**

```bash
# Basic run with defaults
cargo run --features cli --release -- verus run

# Custom output and timeout
cargo run --features cli --release -- verus run -o my_results.csv -t 900

# Resume previous run
cargo run --features cli --release -- verus run --resume

# Custom Verus binary path
cargo run --features cli --release -- verus run --verus-path ~/custom/verus
```

#### `verus summary`

Shows summary statistics from a CSV results file.

**Options:**

- `-f, --file <FILE>` - CSV file to analyze (default: `verus_verification_results.csv`)

**Example:**

```bash
cargo run --features cli --release -- verus summary --file my_results.csv
```

**Output:**

```
📊 Verus Verification Summary
============================

  Total:   85
  Passed:  82 ✅
  Failed:  2 ❌
  Errors:  1 🔥

  Success Rate: 96.5%
```

#### `verus failed`

Shows detailed information about failed proofs.

**Options:**

- `-f, --file <FILE>` - CSV file to analyze (default: `verus_verification_results.csv`)

**Example:**

```bash
cargo run --features cli --release -- verus failed
```

**Output:**

```
❌ Failed Verus Proofs (2 total):

  integers::verify_i128_range
    Status: Failed
    Time: 45s
    Error: verification results:: 0 verified, 1 errors

  floats::verify_f64_finite
    Status: Timeout
    Time: 600s
```

## Justfile Recipes

The justfile provides convenient recipes that wrap the CLI commands:

### `just verify-verus-list`

List all available Verus proofs.

### `just verify-verus-tracked [csv] [timeout]`

Run all proofs with CSV tracking.

**Parameters:**

- `csv` - CSV output file (default: `verus_verification_results.csv`)
- `timeout` - Timeout per proof in seconds (default: 600)

**Example:**

```bash
just verify-verus-tracked my_results.csv 900
```

### `just verify-verus-resume [csv]`

Resume verification, skipping already-passed proofs.

**Parameters:**

- `csv` - CSV file to resume from (default: `verus_verification_results.csv`)

**Example:**

```bash
just verify-verus-resume my_results.csv
```

### `just verify-verus-summary [csv]`

Show summary statistics.

**Parameters:**

- `csv` - CSV file to analyze (default: `verus_verification_results.csv`)

**Example:**

```bash
just verify-verus-summary my_results.csv
```

### `just verify-verus-failed [csv]`

Show failed proofs.

**Parameters:**

- `csv` - CSV file to analyze (default: `verus_verification_results.csv`)

**Example:**

```bash
just verify-verus-failed my_results.csv
```

## CSV Format

The tracking system uses CSV files with the following schema:

```csv
Module,Proof,Status,Time_Seconds,Timestamp,Error_Message
bools,verify_bool_true,SUCCESS,12,2026-02-21T18:30:45.123Z,
integers,verify_i128_range,FAILED,45,2026-02-21T18:31:30.456Z,"verification results:: 0 verified, 1 errors"
```

### Columns

- **Module** - Proof module name (e.g., `bools`, `integers`, `floats`)
- **Proof** - Proof function name (e.g., `verify_bool_true`)
- **Status** - Verification status:
  - `SUCCESS` - Proof verified successfully
  - `FAILED` - Proof failed verification
  - `TIMEOUT` - Verification timed out
  - `ERROR` - Error running verifier
- **Time_Seconds** - Elapsed time in seconds
- **Timestamp** - ISO 8601 timestamp
- **Error_Message** - Error details (empty for successful proofs)

## Configuration

### Verus Binary Path

The tracking system needs to know where the Verus binary is located. It checks in order:

1. `--verus-path` command-line argument
2. `VERUS_PATH` environment variable
3. Default: `~/repos/verus/source/target-verus/release/verus`

**Setting via environment variable:**

```bash
export VERUS_PATH=~/custom/verus/path/verus
just verify-verus-tracked
```

**Setting via .env file:**

```bash
# .env
VERUS_PATH=~/repos/verus/source/target-verus/release/verus
```

The justfile recipes will automatically load from `.env` if present.

### Timeout Configuration

Default timeout is 600 seconds (10 minutes) per proof. Adjust based on your needs:

```bash
# Short timeout for quick checks
just verify-verus-tracked verus_results.csv 60

# Long timeout for complex proofs
just verify-verus-tracked verus_results.csv 1800
```

## Proof Coverage

Currently tracking **85 Verus proofs** across these modules:

- **bools** (2 proofs) - Boolean contract verification
- **chars** (4 proofs) - Character validation contracts
- **collections** (21 proofs) - Vec, Option, Result, HashMap, etc.
- **durations** (1 proof) - Duration contracts
- **floats** (6 proofs) - f32/f64 finite, positive, non-negative
- **integers** (24 proofs) - Integer range and sign contracts
- **mechanisms** (5 proofs) - Elicitation mechanism contracts
- **networks** (9 proofs) - IP, UUID, PathBuf contracts
- **strings** (11 proofs) - String validation contracts
- **regexes** (1 proof, feature-gated) - Regex pattern contracts
- **urls** (4 proofs, feature-gated) - URL validation contracts

## Workflow Examples

### Initial Verification Run

```bash
# Run all proofs with tracking
just verify-verus-tracked

# View results
just verify-verus-summary

# Investigate failures
just verify-verus-failed
```

### Incremental Development

```bash
# Make code changes...

# Resume verification (skips already-passed)
just verify-verus-resume

# Check if failures are fixed
just verify-verus-failed
```

### CI/CD Integration

```bash
# Run with strict timeout
just verify-verus-tracked ci_results.csv 300

# Check for any failures
if just verify-verus-failed ci_results.csv | grep -q "Failed"; then
    echo "❌ Verification failed"
    exit 1
fi
```

### Performance Analysis

```bash
# Track timing for optimization
just verify-verus-tracked timing.csv

# Analyze slow proofs
cat timing.csv | sort -t, -k4 -rn | head -10
```

## Comparison with Kani Tracking

The Verus tracking system mirrors the Kani tracking infrastructure:

| Feature | Kani | Verus |
|---------|------|-------|
| CSV tracking | ✅ | ✅ |
| Resume capability | ✅ | ✅ |
| Summary statistics | ✅ | ✅ |
| Failed test analysis | ✅ | ✅ |
| CLI interface | ✅ | ✅ |
| Justfile recipes | ✅ | ✅ |
| Timeout configuration | ✅ | ✅ |
| Progress display | ✅ | ✅ |

**Key Differences:**

- **Kani** uses harness-based proofs with `#[kani::proof]` attributes
- **Verus** uses module-level functions with SMT-based verification
- **Kani** timeout per harness, **Verus** timeout per proof module
- **Kani** verification is typically faster (seconds), **Verus** may take minutes

## Troubleshooting

### "Verus not found" Error

**Problem:**

```
❌ Verus not found at: ~/repos/verus/source/target-verus/release/verus
   Set VERUS_PATH environment variable or use --verus-path
```

**Solution:**

1. Verify Verus is installed: `verus --version`
2. Set VERUS_PATH in `.env` file or environment
3. Or use `--verus-path` flag explicitly

### Timeout Issues

**Problem:** Proofs timing out frequently

**Solutions:**

1. Increase timeout: `just verify-verus-tracked results.csv 1800`
2. Run proofs individually to identify slow ones
3. Check system resources (CPU, memory)

### Resume Not Working

**Problem:** Resume mode re-runs passed proofs

**Solution:**

1. Ensure CSV file exists and is readable
2. Check CSV format matches expected schema
3. Verify STATUS column contains "SUCCESS" for passed proofs

### Compilation Errors

**Problem:** CLI commands fail to compile

**Solution:**

1. Ensure `cli` feature is enabled: `cargo build --features cli`
2. Update dependencies: `cargo update`
3. Clean build: `cargo clean && cargo build --features cli`

## Implementation Details

### Source Files

- **`crates/elicitation/src/verification/verus_runner.rs`** - Main tracking implementation
- **`crates/elicitation/src/cli.rs`** - CLI command definitions and handlers
- **`justfile`** - Convenience recipes
- **`crates/elicitation/Cargo.toml`** - Feature and dependency configuration

### Architecture

The tracking system consists of:

1. **`VerusProof`** - Proof identifier (module + name)
2. **`VerusProofResult`** - Result of running a single proof
3. **`VerificationStatus`** - Success/Failed/Timeout/Error enum
4. **`VerusSummary`** - Aggregated statistics
5. **`run_verus_proof()`** - Execute single proof
6. **`run_all_proofs()`** - Execute all proofs with tracking
7. **`summarize_results()`** - Load and summarize CSV
8. **`list_failed_proofs()`** - Extract failed proofs from CSV

### Verification Process

For each proof:

1. Create temporary Rust source file importing the proof function
2. Run Verus binary on the temp file
3. Parse stdout/stderr for verification results
4. Determine status (SUCCESS/FAILED/TIMEOUT/ERROR)
5. Record result to CSV with timing
6. Clean up temp file
7. Display progress to user

## Future Enhancements

Potential improvements:

- [ ] Parallel proof execution (with concurrency limit)
- [ ] Detailed error categorization (assertion, overflow, etc.)
- [ ] HTML report generation
- [ ] Diff comparison between runs
- [ ] Integration with CI/CD platforms
- [ ] Proof dependency analysis
- [ ] Automatic bisection for regressions
- [ ] Performance profiling per proof

## See Also

- [Kani Tracking Documentation](https://docs.rs/elicitation/latest/elicitation/verification/runner/)
- [Verus Guide](https://verus-lang.github.io/verus/guide/)
- [Elicitation Verification Framework](./crates/elicitation/src/verification/mod.rs)
- [Verus Update Summary](./VERUS_UPDATE_SUMMARY.md) - Details on latest Verus version
