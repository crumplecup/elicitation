//! Kani proof harnesses for contract types.
//!
//! This module contains formal verification proofs using the Kani model checker.
//! Each proof harness verifies that contract invariants hold by construction.
//!
//! # Global Configuration
//!
//! Run with global unwind bound to prevent memchr infinite loops:
//! ```bash
//! cargo kani --package elicitation --features verify-kani -- --default-unwind 20
//! ```
//!
//! Individual harnesses can override with `#[kani::unwind(N)]`.
//!
//! # Running Proofs
//!
//! ```bash
//! # Run all Kani proofs
//! cargo kani --package elicitation --features verify-kani -- --default-unwind 20
//!
//! # Run specific proof
//! cargo kani --package elicitation --features verify-kani --harness verify_i8_positive
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
mod utf8;
mod utf8_benchmark;

#[cfg(feature = "uuid")]
mod uuid_bytes;

#[cfg(feature = "url")]
mod urls;

#[cfg(feature = "regex")]
mod regexes;
