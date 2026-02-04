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

mod benchmark_marginal;
mod bools;
mod chars;
mod collections;
mod durations;
mod floats;
mod integers;
mod ipaddr_bytes;
mod macaddr;
mod mechanisms;
mod networks;
mod regexbytes;
mod socketaddr;
mod strings;
mod unit;
mod urlbytes;
mod utf8;
mod utf8_benchmark;
mod utf8_chunked;

#[cfg(unix)]
mod pathbytes;

#[cfg(feature = "uuid")]
mod uuid_bytes;

#[cfg(feature = "url")]
mod urls;

#[cfg(feature = "regex")]
mod regexes;
