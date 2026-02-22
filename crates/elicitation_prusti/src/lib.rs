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
mod paths;
mod tuples;

// Platform-specific modules
#[cfg(unix)]
mod pathbytes;

// Feature-gated module declarations
// Note: Some modules are stubs (uuids, datetimes) - commented out until implemented
// #[cfg(feature = "uuid")]
// mod uuids;

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

// #[cfg(feature = "chrono")]
// mod datetimes_chrono;

// #[cfg(feature = "time")]
// mod datetimes_time;

// #[cfg(feature = "jiff")]
// mod datetimes_jiff;

// Re-export all proof functions for discoverability
pub use bools::*;
pub use chars::*;
pub use collections::*;
pub use durations::*;
pub use floats::*;
pub use integers::*;
pub use ipaddr_bytes::*;
pub use macaddr::*;
pub use mechanisms::*;
pub use networks::*;
pub use socketaddr::*;
pub use strings::*;
pub use utf8::*;
pub use paths::*;
pub use tuples::*;

#[cfg(unix)]
pub use pathbytes::*;

// #[cfg(feature = "uuid")]
// pub use uuids::*;

#[cfg(all(feature = "uuid", prusti))]
pub use uuid_bytes::*;

#[cfg(feature = "url")]
pub use urls::*;

#[cfg(all(feature = "url", prusti))]
pub use urlbytes::*;

#[cfg(feature = "regex")]
pub use regexes::*;

#[cfg(all(feature = "regex", prusti))]
pub use regexbytes::*;

// #[cfg(feature = "chrono")]
// pub use datetimes_chrono::*;

// #[cfg(feature = "time")]
// pub use datetimes_time::*;

// #[cfg(feature = "jiff")]
// pub use datetimes_jiff::*;
