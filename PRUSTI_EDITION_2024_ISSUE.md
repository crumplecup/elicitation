# Prusti Edition 2024 Compatibility Issue

## Status: ⚠️ Blocked

Prusti verification is currently non-functional due to Rust Edition 2024 incompatibility.

## Problem

This codebase uses **Rust Edition 2024** (required for let-chains and other modern features).

Prusti uses **nightly-2023-09-15** toolchain, which only supports editions up to **2021**.

## Error

```
error: failed to parse the `edition` key

Caused by:
  this version of Cargo is older than the `2024` edition, 
  and only supports `2015`, `2018`, and `2021` editions.
```

## Attempted Solutions

### 1. ❌ Downgrade to Edition 2021
- **Status**: Rejected by project maintainer
- **Reason**: Code uses Edition 2024 features (let-chains in `elicitation_derive`)

### 2. ❌ Update Prusti Toolchain
- **Attempted**: `nightly-2024-11-15`
- **Result**: Build failures in dependencies
```
error[E0635]: unknown feature `stdsimd`
  --> ahash-0.7.6/src/lib.rs:33:42
```
- **Root cause**: Prusti dependencies (ahash 0.7.6) incompatible with modern nightlies

## Current Workaround

**Use Kani for verification** (fully functional):
```bash
just verify-kani-tracked       # Run all 232 proofs
just verify-kani-summary       # Show results
```

Kani uses current stable Rust and supports Edition 2024.

## Resolution Options

### Option A: Wait for Upstream (Recommended)
Wait for Prusti team to update to newer toolchain that supports Edition 2024.

**Timeline**: Unknown (Prusti last updated March 2024)

**Tracking**: https://github.com/viperproject/prusti-dev/issues

### Option B: Contribute Upstream
Contribute Edition 2024 support and modern toolchain update to Prusti.

**Effort**: High (requires deep Prusti knowledge, dependency updates, testing)

**Skills needed**: Rust internals, Viper backend, formal verification

### Option C: Dual Verification
Keep Kani as primary verifier, add Prusti support when available.

**Status**: Already implemented
- Kani: 232 proofs, all passing ✅
- Prusti: 427 proofs, blocked on Edition 2024 ⚠️

## Infrastructure Ready

All Prusti infrastructure is in place and ready when toolchain updated:

- ✅ `prusti_runner.rs` - Runner with CSV tracking
- ✅ CLI commands - `cargo run -- prusti list/run/summary/failed`
- ✅ Justfile recipes - `verify-prusti-tracked`, etc.
- ✅ Documentation - `PRUSTI_VERIFICATION_TRACKING.md`
- ✅ Feature gates - `#[cfg(feature = "verify-prusti")]`
- ✅ Contracts - All 427 `#[requires]`/`#[ensures]` annotations

**Single command to test** when toolchain updated:
```bash
just verify-prusti-tracked
```

## Technical Details

### Prusti Version
```bash
$ cd ~/repos/prusti-dev && git log -1 --oneline
0d4a8d497ac Fix issue #1505 (#1511)  # March 26, 2024
```

### Toolchain
```toml
[toolchain]
channel = "nightly-2023-09-15"
components = [ "rustc-dev", "llvm-tools-preview", "rust-std" ]
```

### Edition 2024 Features Used
- Let-chains: `if let Some(x) = a && let Some(y) = b`
- Used in `elicitation_derive/src/contract_type.rs:50-54`

## Recommendation

**Continue with Kani** as primary verification framework. Revisit Prusti when:
1. Prusti updates to Edition 2024-compatible toolchain, OR
2. Community demonstrates successful workaround, OR
3. Project requirements demand dual verification

Kani provides equivalent or stronger guarantees for this codebase.

## Related Files

- `crates/elicitation/src/verification/prusti_runner.rs` - Runner implementation
- `PRUSTI_VERIFICATION_TRACKING.md` - Complete Prusti documentation
- `justfile` - Prusti recipes (lines 587-612)
- `KANI_CASTLE_ON_CLOUD_METHODOLOGY.md` - Working Kani verification

## Last Updated

2026-02-03 - Confirmed Edition 2024 incompatibility, documented issue
