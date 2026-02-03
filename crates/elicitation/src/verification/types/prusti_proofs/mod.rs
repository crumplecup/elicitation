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
mod ipaddr_bytes;
mod mechanisms;
mod networks;
mod strings;
mod utf8;

#[cfg(unix)]
mod pathbytes;

#[cfg(feature = "url")]
mod urls;

#[cfg(feature = "url")]
mod urlbytes;

#[cfg(feature = "regex")]
mod regexes;

#[cfg(feature = "regex")]
mod regexbytes;

/// Total number of Prusti proofs implemented.
pub const PRUSTI_PROOF_COUNT: usize = 285; // 101 + 17 UTF-8 + 33 PathBytes + 45 RegexBytes + 46 UrlBytes + 43 IpAddrBytes

/// Verification coverage percentage.
pub const PRUSTI_COVERAGE_PERCENT: usize = 100;
