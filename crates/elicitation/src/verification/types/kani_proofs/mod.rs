//! Kani proof harnesses for contract types.
//!
//! This module contains formal verification proofs using the Kani model checker.
//! Each proof harness verifies that contract invariants hold by construction.
//!
//! # Running Proofs
//!
//! ```bash
//! # Run all Kani proofs
//! cargo kani --package elicitation
//!
//! # Run specific proof
//! cargo kani --package elicitation --harness verify_i8_positive
//! ```
//!
//! # Proof Strategy
//!
//! For each contract type T, we prove:
//! 1. **Construction Safety**: `T::new(x)` succeeds ⟹ invariant holds
//! 2. **Invalid Rejection**: `T::new(x)` fails ⟹ invariant violated
//! 3. **Accessor Correctness**: `t.get()` returns validated value
//! 4. **Unwrap Correctness**: `t.into_inner()` returns validated value

#![cfg(kani)]

mod integers;
mod floats;
mod strings;
mod bools;
mod chars;
mod durations;
mod networks;
mod collections;
mod mechanisms;

#[cfg(feature = "url")]
mod urls;

#[cfg(feature = "regex")]
mod regexes;
