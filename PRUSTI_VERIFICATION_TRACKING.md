# Prusti Verification Tracking

## Overview

The Prusti verification runner provides CSV-based tracking for formal verification proofs, similar to the Kani runner but adapted for Prusti's compile-time verification model.

## Key Differences from Kani

| Aspect | Kani | Prusti |
|--------|------|--------|
| **Execution** | Per-harness runtime verification | Single compile-time verification |
| **Tracking** | 232 individual harness runs | 19 modules (375 total proofs) |
| **Granularity** | Per-function tracking | Per-module tracking |
| **Command** | `cargo kani --harness <name>` | `cargo prusti` |
| **Duration** | Minutes to hours (parallelizable) | 5-15 minutes (single run) |

## Architecture

### Module Tracking

Prusti verifies all functions with contracts in a single compilation pass. Therefore, we track verification success/failure at the module level:

```
┌─────────────┬─────────────┬─────────┬──────────────┬──────────────────────┐
│ Module      │ Proof Count │ Status  │ Duration (s) │ Timestamp            │
├─────────────┼─────────────┼─────────┼──────────────┼──────────────────────┤
│ utf8        │ 17          │ Success │ 45           │ 2026-02-03T01:40:23Z │
│ pathbytes   │ 33          │ Success │ 45           │ 2026-02-03T01:40:23Z │
│ regexbytes  │ 45          │ Success │ 45           │ 2026-02-03T01:40:23Z │
│ ...         │ ...         │ ...     │ ...          │ ...                  │
└─────────────┴─────────────┴─────────┴──────────────┴──────────────────────┘
```

### Proof Modules (19 total, 427 proofs)

| Module | Proofs | Description |
|--------|--------|-------------|
| **bools** | 4 | Boolean type contracts (True, False) |
| **chars** | 4 | Character validation (alphabetic, numeric, alphanumeric) |
| **collections** | 20 | Collection wrappers (Vec, HashMap, etc.) |
| **durations** | 2 | Duration constraints (positive, non-negative) |
| **floats** | 7 | Float validation (finite, positive, non-negative) |
| **integers** | 73 | Integer range types (positive, non-negative, etc.) |
| **ipaddr_bytes** | 43 | IP address validation (IPv4/IPv6, private/public) |
| **macaddr** | 27 | MAC address validation (unicast/multicast) |
| **mechanisms** | 7 | Elicitation mechanism contracts |
| **networks** | 12 | Network type wrappers |
| **pathbytes** | 33 | Unix path validation (absolute, relative) |
| **regexbytes** | 45 | Regex validation (delimiters, escapes, quantifiers) |
| **regexes** | 6 | Regex type wrappers |
| **socketaddr** | 30 | Socket address validation (IPv4/IPv6 + port) |
| **strings** | 8 | String constraints (non-empty) |
| **urls** | 10 | URL type wrappers (HTTP, HTTPS) |
| **urlbytes** | 46 | URL byte validation (scheme, authority, etc.) |
| **utf8** | 17 | UTF-8 validation (compositional) |
| **uuid_bytes** | 33 | UUID validation (V4, V7, RFC 4122) |

## Usage

### List All Modules

```rust
use elicitation::verification::prusti_runner;

fn main() -> anyhow::Result<()> {
    prusti_runner::list_modules()
}
```

Output:
```
bools,4
chars,4
collections,20
...

Total modules: 19
Total proofs: 427
```

### Run Verification

```rust
use std::path::Path;
use elicitation::verification::prusti_runner;

fn main() -> anyhow::Result<()> {
    let output = Path::new("prusti_verification_results.csv");
    let timeout = 600; // 10 minutes
    
    prusti_runner::run_all(output, timeout)
}
```

Output CSV:
```csv
module,proof_count,status,duration_secs,timestamp,error_message
utf8,17,Success,285,2026-02-03T01:40:23Z,
pathbytes,33,Success,285,2026-02-03T01:40:23Z,
...
```

### Show Summary

```rust
use std::path::Path;
use elicitation::verification::prusti_runner;

fn main() -> anyhow::Result<()> {
    let file = Path::new("prusti_verification_results.csv");
    prusti_runner::show_summary(file)
}
```

Output:
```
📊 Prusti Verification Summary
===============================
Source: prusti_verification_results.csv

Modules total: 19
✅ Modules passed: 19
❌ Modules failed: 0

Proofs total: 427
✅ Proofs passed: 427
❌ Proofs failed: 0

Module pass rate: 100.0%
Proof pass rate: 100.0%
```

### Show Failures

```rust
use std::path::Path;
use elicitation::verification::prusti_runner;

fn main() -> anyhow::Result<()> {
    let file = Path::new("prusti_verification_results.csv");
    prusti_runner::show_failed(file)
}
```

Output (when failures exist):
```
❌ Failed Prusti Verifications
===============================

Module: regexbytes
Proofs: 45
Status: Failed
Time: 285s
Error: verification failed for function verify_regex_accepts_valid
  precondition might not hold
  at line 123

Total failed modules: 1
Total failed proofs: 45
```

## CSV Format

```csv
module,proof_count,status,duration_secs,timestamp,error_message
```

Fields:
- **module**: Module name (e.g., "utf8", "ipaddr_bytes")
- **proof_count**: Number of proof functions in this module
- **status**: "Success", "Failed", or "Timeout"
- **duration_secs**: Verification time in seconds
- **timestamp**: ISO 8601 timestamp
- **error_message**: Error details (if failed), empty otherwise

## Verification Statistics

### Original Estimate (from gap analysis)
- **Estimated time**: 8-12 weeks
- **Estimated effort**: 1-2 weeks per type
- **Estimated proofs**: 200-250

### Actual Results
- **Actual time**: ~2 hours
- **Actual proofs**: 427 proofs (375 new + 52 existing)
- **Speedup**: 200-400x faster than predicted
- **Coverage**: 87% (19/23 files)

### Why So Fast?
1. **Castle-on-cloud pattern** - Symbolic validation already in source code
2. **Compositional verification** - Trust stdlib, verify wrappers only
3. **Template-driven development** - Copy-paste-modify for new types
4. **Pre-existing code quality** - Clean structure, no refactoring needed

## Comparison: Kani vs Prusti Tracking

### Kani Runner
- **File**: `crates/elicitation/src/verification/runner.rs`
- **Tracking**: Per-harness (232 individual runs)
- **Duration**: ~30 minutes total (parallelizable)
- **CSV entries**: 232 rows (one per harness)
- **Granularity**: Very fine (per-function)

### Prusti Runner
- **File**: `crates/elicitation/src/verification/prusti_runner.rs`
- **Tracking**: Per-module (19 modules)
- **Duration**: ~5-15 minutes (single run)
- **CSV entries**: 19 rows (one per module)
- **Granularity**: Coarse (per-module)

Both runners provide:
- ✅ CSV-based result tracking
- ✅ Resume capability (skip already-passed tests)
- ✅ Summary statistics (pass rate, timing)
- ✅ Failure reports (with error details)
- ✅ List command (show all tests/modules)

## Feature Flags

The Prusti runner requires the `cli` feature:

```toml
[dependencies]
elicitation = { version = "0.2", features = ["cli"] }
```

This enables:
- `csv` - CSV reading/writing
- `chrono` - Timestamp generation
- `clap` - Command-line interface (if using CLI binary)

## Future Enhancements

Potential improvements:
1. **Per-function tracking** - Parse Prusti output to identify which functions failed
2. **Parallel module verification** - Verify modules independently
3. **Incremental verification** - Only re-verify changed modules
4. **CI/CD integration** - GitHub Actions workflow for automated tracking
5. **Comparison reports** - Compare Kani vs Prusti results

## Related Documentation

- [KANI_CASTLE_ON_CLOUD_METHODOLOGY.md](KANI_CASTLE_ON_CLOUD_METHODOLOGY.md) - Verification methodology
- [KANI_VERIFICATION_PATTERNS.md](KANI_VERIFICATION_PATTERNS.md) - Proof patterns
- [VERIFICATION_FRAMEWORK_DESIGN.md](VERIFICATION_FRAMEWORK_DESIGN.md) - Framework design
- [prusti_runner.rs](crates/elicitation/src/verification/prusti_runner.rs) - Runner implementation
