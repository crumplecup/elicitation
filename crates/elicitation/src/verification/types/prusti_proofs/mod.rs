//! Prusti verification proofs for contract types.
//!
//! Formal verification using Prusti (Viper/separation logic).

#![cfg(feature = "verify-prusti")]

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

/// Total number of Prusti proofs implemented.
pub const PRUSTI_PROOF_COUNT: usize = 101;

/// Verification coverage percentage.
pub const PRUSTI_COVERAGE_PERCENT: usize = 100;
