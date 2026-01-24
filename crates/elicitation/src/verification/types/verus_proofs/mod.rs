//! Verus verification proofs for contract types.
//!
//! This module contains formal verification using Verus and Z3 SMT solver.
//! Each proof harness verifies that contract invariants hold.
//!
//! # Running Proofs
//!
//! ```bash
//! # Run all Verus proofs
//! ~/repos/verus/source/target-verus/release/verus \
//!     --crate-type=lib \
//!     crates/elicitation/src/lib.rs
//! ```
//!
//! # Proof Strategy
//!
//! For each contract type T, we prove:
//! 1. **Construction Safety**: `T::new(x)` succeeds ⟹ invariant holds
//! 2. **Invalid Rejection**: `T::new(x)` fails ⟹ invariant violated
//! 3. **Accessor Correctness**: `t.get()` returns validated value
//! 4. **Unwrap Correctness**: `t.into_inner()` returns validated value

#![cfg(all(feature = "verify-verus", not(kani)))]

mod integers;
mod floats;
mod strings;
mod bools;
mod chars;
mod durations;
mod collections;
mod networks;
mod mechanisms;

#[cfg(feature = "url")]
mod urls;

#[cfg(feature = "regex")]
mod regexes;

/// Total number of Verus proofs implemented.
pub const VERUS_PROOF_COUNT: usize = 101;

/// Verification coverage percentage.
pub const VERUS_COVERAGE_PERCENT: usize = 100;
