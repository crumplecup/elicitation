//! Kani formal verification proofs for elicitation contracts.
//!
//! This crate contains 291 proof harnesses verifying contract invariants
//! using the Kani model checker's symbolic execution engine.
//!
//! # Architecture
//!
//! Kani proofs are organized by type category:
//! - **Primitives**: strings, integers, floats, chars, bools
//! - **Collections**: Vec, HashMap, HashSet, arrays, tuples
//! - **DateTime**: chrono, time, jiff DateTime types
//! - **Network**: IP addresses, MAC addresses, socket addresses
//! - **External**: URLs, UUIDs, regex, paths
//! - **Mechanisms**: Contract composition and verification
//!
//! # Running Proofs
//!
//! ```bash
//! # All proofs (291 harnesses)
//! cargo kani -p elicitation_kani --all-features
//!
//! # Specific module
//! cargo kani -p elicitation_kani --harness verify_string_non_empty
//!
//! # With specific features
//! cargo kani -p elicitation_kani --features uuid,url
//! ```
//!
//! # Proof Strategy
//!
//! For each contract type T, we prove:
//! 1. **Construction Safety**: `T::new(x)` succeeds ⟹ invariant holds
//! 2. **Invalid Rejection**: `T::new(x)` fails ⟹ invariant violated
//! 3. **Accessor Correctness**: `t.get()` returns validated value
//! 4. **Unwrap Correctness**: `t.into_inner()` returns validated value
//!
//! # Global Configuration
//!
//! Run with global unwind bound to prevent memchr infinite loops:
//! ```bash
//! cargo kani -p elicitation_kani --all-features -- --default-unwind 20
//! ```

// Re-export verification framework from main crate
pub use elicitation::verification::contracts;
pub use elicitation::verification::{Contract, WithContract};

// Proof modules (organized by type category)
#[cfg(kani)]
mod bools;
#[cfg(kani)]
mod chars;
#[cfg(kani)]
mod collections;
#[cfg(kani)]
mod durations;
#[cfg(kani)]
mod errors;
#[cfg(kani)]
mod floats;
#[cfg(kani)]
mod integers;
#[cfg(kani)]
mod ipaddr_bytes;
#[cfg(kani)]
mod macaddr;
#[cfg(kani)]
mod mechanisms;
#[cfg(kani)]
mod networks;
#[cfg(kani)]
mod socketaddr;
#[cfg(kani)]
mod strings;
#[cfg(kani)]
mod systemtime;
#[cfg(kani)]
mod unit;
#[cfg(kani)]
mod utf8;

// Feature-gated proof modules
#[cfg(all(kani, unix))]
mod pathbytes;

#[cfg(all(kani, feature = "uuid"))]
mod uuid_bytes;

#[cfg(all(kani, feature = "url"))]
mod urlbytes;
#[cfg(all(kani, feature = "url"))]
mod urls;

// Serde boundary consistency proofs (require verification + serde_json)
#[cfg(kani)]
mod serde_boundary;

#[cfg(all(kani, feature = "regex"))]
mod regexbytes;
#[cfg(all(kani, feature = "regex"))]
mod regexes;

#[cfg(all(kani, feature = "chrono"))]
mod datetimes_chrono;

#[cfg(all(kani, feature = "time"))]
mod datetimes_time;

#[cfg(all(kani, feature = "jiff"))]
mod datetimes_jiff;

#[cfg(all(kani, feature = "clap-types"))]
mod clap_types;
