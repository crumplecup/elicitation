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
mod macaddr;
mod mechanisms;
mod networks;
mod socketaddr;
mod strings;
mod utf8;

#[cfg(kani)]
mod uuid_bytes;

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
#[cfg(prusti)]
pub const PRUSTI_PROOF_COUNT: usize = 375; // 101 + 17 UTF-8 + 33 PathBytes + 45 RegexBytes + 46 UrlBytes + 43 IpAddrBytes + 30 SocketAddr + 27 MacAddr + 33 UuidBytes

/// Verification coverage percentage.
#[cfg(prusti)]
pub const PRUSTI_COVERAGE_PERCENT: usize = 100;
