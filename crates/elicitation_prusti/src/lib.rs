//! Prusti verification proofs for elicitation contract types.
//!
//! This crate contains Prusti separation logic proofs using `prusti-contracts`.
//! It imports contract types from the main elicitation crate but uses edition 2021
//! for compatibility with Prusti (which requires Rust nightly-2023-09-15).
//!
//! # Edition Boundary
//!
//! This crate uses edition = "2021" while elicitation uses edition = "2024".
//! This is safe because:
//! - Rust editions are per-crate, not transitive
//! - Contract types have edition-agnostic APIs
//! - Tested pattern (Kani tests already do this)
//!
//! When Prusti supports edition 2024, we can upgrade this crate.
//!
//! All proof functions are public and serve as documentation of verification coverage.

#![forbid(unsafe_code)]

// Module declarations
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
mod socketaddr;
mod strings;
mod utf8;

// Platform-specific modules
#[cfg(unix)]
mod pathbytes;

// Feature-gated module declarations
#[cfg(feature = "uuid")]
mod uuid_bytes;

#[cfg(feature = "url")]
mod urls;

#[cfg(feature = "url")]
mod urlbytes;

#[cfg(feature = "regex")]
mod regexes;

#[cfg(feature = "regex")]
mod regexbytes;
