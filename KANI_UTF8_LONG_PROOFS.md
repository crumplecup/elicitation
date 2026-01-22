# Kani UTF-8 Verification: Long-Running Proofs

## Philosophy

This crate takes a **purist approach** to formal verification: we prove correctness symbolically over the entire valid input space, even if proofs take days to complete. Symbolic verification is not CI/CD - it's mathematical proof.

## Resource Requirements

### Fast Proofs (< 1 hour)
- `verify_ascii_always_valid` - 7 seconds
- `verify_invalid_continuation_rejected` - Seconds
- `verify_overlong_two_byte_rejected` - Seconds
- `verify_surrogate_rejected` - Seconds
- `verify_incomplete_sequence_rejected` - Seconds

### Expensive Proofs (hours to days)
- `verify_valid_two_byte_accepted` - **3,968 combinations (62 × 64)**
  - Estimated: 1-48 hours depending on hardware
  - Proves ALL valid 2-byte UTF-8 sequences accepted

- `verify_valid_three_byte_accepted` - **49,152 combinations (12 × 64²)**
  - Estimated: 6-72 hours depending on hardware
  - Proves valid 3-byte UTF-8 (excluding surrogates/overlongs)

### Very Expensive Proofs (days to weeks)
- `verify_valid_four_byte_accepted` - **786,432 combinations (3 × 64³)**
  - Estimated: 2-14 days depending on hardware
  - Proves valid 4-byte UTF-8 (code points ≤ U+10FFFF)

## Hardware Scaling

Verification time scales with:
- **CPU cores:** CBMC SAT solver is single-threaded (no benefit)
- **CPU speed:** Linear speedup with clock speed
- **RAM:** Minimum 4GB, 8-16GB recommended for 4-byte proofs
- **Disk:** Minimal impact

**Example scaling:**
- Laptop (i7-1165G7 @ 2.8GHz): 48 hours for 2-byte
- Workstation (Ryzen 9 5950X @ 3.4GHz): ~36 hours for 2-byte
- Server (EPYC 7763 @ 2.45GHz but 64 cores won't help): ~50 hours for 2-byte

## Running Long Proofs

### Option 1: Single proof with observability
```bash
cargo kani --features verify-kani --harness verify_valid_two_byte_accepted 2>&1 | tee utf8_2byte_proof.log
```

### Option 2: Background with nohup
```bash
nohup cargo kani --features verify-kani --harness verify_valid_four_byte_accepted > utf8_4byte_proof.log 2>&1 &
```

### Option 3: Screen/tmux session
```bash
screen -S kani_utf8
cargo kani --features verify-kani --harness verify_valid_four_byte_accepted
# Detach with Ctrl+A, D
# Reattach with: screen -r kani_utf8
```

## Monitoring Progress

Kani outputs loop unwinding iterations in real-time:
```
Unwinding loop iteration 1
Unwinding loop iteration 2
...
```

When this stops and memory usage stabilizes, CBMC is working on SAT solving (the expensive part).

## Expected Output

Successful proof will show:
```
SUMMARY:
 ** 0 of N failed

VERIFICATION:- SUCCESSFUL
Verification Time: XXXXs
```

## Feature Gating

These expensive proofs are behind the `verify-kani` feature. They are **not** run in:
- `cargo test`
- `cargo check`
- `just check-all`

They are **only** run with explicit:
```bash
just verify-kani
# OR
cargo kani --features verify-kani
```

## Why This Matters

Once these proofs complete, they prove that `is_valid_utf8()` correctly validates:
- **All 3,968** valid 2-byte UTF-8 sequences
- **All 49,152** valid 3-byte UTF-8 sequences (non-surrogate)
- **All 786,432** valid 4-byte UTF-8 sequences

This is **mathematical certainty**, not statistical confidence from fuzzing.

## Results Repository

Completed proof logs are tracked in:
```
proofs/
├── utf8_2byte_proof_<date>_<machine>.log
├── utf8_3byte_proof_<date>_<machine>.log
└── utf8_4byte_proof_<date>_<machine>.log
```

Commit logs with verification times to establish baseline performance across hardware.

## Marginal Cost Analysis

See `kani_marginal_benchmark.log` for empirical measurement of verification cost scaling.

The script `scripts/kani_marginal_cost.sh` runs micro-benchmarks to calculate:
```
cost_per_combination = Δtime / Δcombinations
```

This lets us predict completion times on different hardware.

## Just Recipes

The justfile provides convenient commands for running benchmarks and long proofs:

### Marginal Cost Benchmark

Measure Kani's verification cost scaling:

```bash
just kani-benchmark
```

This runs micro-benchmarks (4-16 combinations) multiple times to calculate the marginal cost per symbolic combination. Results saved to `kani_marginal_benchmark.log`.

**Expected time:** 1-2 hours

### Long-Running Proofs

Run expensive symbolic UTF-8 proofs:

```bash
# 2-byte proof (3,968 combinations, hours-days)
just kani-long-proofs 2byte

# 3-byte proof (49,152 combinations, days)
just kani-long-proofs 3byte

# 4-byte proof (786,432 combinations, days-weeks)
just kani-long-proofs 4byte

# All proofs sequentially (weeks-months)
just kani-long-proofs all
```

Each proof:
- Prompts for confirmation (interactive)
- Logs output to `utf8_Nbyte_proof.log`
- Shows unwinding iterations and verification progress
- Reports final VERIFICATION:- SUCCESSFUL or FAILED

**Tip:** Run in background with screen/tmux:
```bash
screen -S kani_utf8
just kani-long-proofs 2byte
# Detach: Ctrl+A, D
# Reattach: screen -r kani_utf8
```

Or with nohup:
```bash
nohup just kani-long-proofs 2byte > 2byte.out 2>&1 &
tail -f 2byte.out
```
