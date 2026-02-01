//! Prusti verification proofs for contract types.
//!
//! Formal verification using Prusti (Viper/separation logic).

#![cfg(feature = "verify-prusti")]

mod bools;
mod chars;
mod collections;
mod durations;
mod floats;
mod integers;
mod mechanisms;
mod networks;
mod strings;

#[cfg(feature = "url")]
mod urls;

#[cfg(feature = "regex")]
mod regexes;

/// Total number of Prusti proofs implemented.
pub const PRUSTI_PROOF_COUNT: usize = 101;

/// Verification coverage percentage.
pub const PRUSTI_COVERAGE_PERCENT: usize = 100;
