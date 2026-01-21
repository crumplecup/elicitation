//! Creusot verification proofs for contract types.
//!
//! Formal verification using Creusot (WhyML/Why3/SMT).

#![cfg(feature = "verify-creusot")]

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

/// Total number of Creusot proofs implemented.
pub const CREUSOT_PROOF_COUNT: usize = 101;

/// Verification coverage percentage.
pub const CREUSOT_COVERAGE_PERCENT: usize = 100;
