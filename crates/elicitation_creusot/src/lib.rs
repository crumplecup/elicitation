//! Creusot verification proofs for elicitation contract types.
//!
//! This crate contains pure Rust proofs that can be verified by Creusot.
//! It imports contract types from the main elicitation crate but avoids
//! async code that Creusot cannot handle.
//!
//! All proof functions are public and serve as documentation of verification coverage.

#![forbid(unsafe_code)]

// Creusot attributes
pub use creusot_std::prelude::*;

// Module declarations
mod bools;
mod chars;
mod collections;
mod durations;
mod floats;
mod integers;
mod networks;
mod paths;
mod strings;
mod tuples;

// Trenchcoat verification types (internal wrappers)
mod ipaddr_bytes;
mod macaddr;
mod mechanisms;
mod socketaddr;
mod utf8;

#[cfg(unix)]
mod pathbytes;

// Feature-gated module declarations
#[cfg(feature = "uuid")]
mod uuids;

#[cfg(feature = "uuid")]
mod uuid_bytes;

#[cfg(feature = "serde_json")]
mod values;

#[cfg(feature = "serde_json")]
mod serde_boundary;

#[cfg(feature = "url")]
mod urls;

#[cfg(feature = "url")]
mod urlbytes;

#[cfg(feature = "regex")]
mod regexes;

#[cfg(feature = "regex")]
mod regexbytes;

#[cfg(feature = "chrono")]
mod datetimes_chrono;

#[cfg(feature = "time")]
mod datetimes_time;

#[cfg(feature = "jiff")]
mod datetimes_jiff;

#[cfg(feature = "reqwest")]
mod http;

// Re-export all proof functions for discoverability
pub use bools::*;
pub use chars::*;
pub use collections::*;
pub use durations::*;
pub use floats::*;
pub use integers::*;
pub use networks::*;
pub use paths::*;
pub use strings::*;
pub use tuples::*;

// Trenchcoat proof modules (all #[cfg(creusot)] gated)
#[cfg(creusot)]
pub use ipaddr_bytes::*;
#[cfg(creusot)]
pub use macaddr::*;
#[cfg(creusot)]
pub use mechanisms::*;
#[cfg(creusot)]
pub use socketaddr::*;
#[cfg(creusot)]
pub use utf8::*;

#[cfg(all(unix, creusot))]
pub use pathbytes::*;

#[cfg(feature = "uuid")]
pub use uuids::*;

#[cfg(all(feature = "uuid", creusot))]
pub use uuid_bytes::*;

#[cfg(feature = "serde_json")]
pub use values::*;

#[cfg(feature = "serde_json")]
pub use serde_boundary::*;

#[cfg(feature = "url")]
pub use urls::*;

#[cfg(all(feature = "url", creusot))]
pub use urlbytes::*;

#[cfg(feature = "regex")]
pub use regexes::*;

#[cfg(all(feature = "regex", creusot))]
pub use regexbytes::*;

#[cfg(feature = "chrono")]
pub use datetimes_chrono::*;

#[cfg(feature = "time")]
pub use datetimes_time::*;

#[cfg(feature = "jiff")]
pub use datetimes_jiff::*;

#[cfg(feature = "reqwest")]
pub use http::*;
