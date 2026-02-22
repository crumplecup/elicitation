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

#![forbid(unsafe_code)]

use prusti_contracts::*;

// Import contract types from elicitation (edition 2024 → edition 2021 is safe)
use elicitation::{
    BoolFalse,
    // Bool types
    BoolTrue,
    // Char types
    CharAlphabetic,
    CharAlphanumeric,
    CharNumeric,
    DurationPositive,
    // Duration types
    DurationNonZero,
    F32Finite,
    F32NonNegative,
    // Float types
    F32Positive,
    F64Finite,
    F64NonNegative,
    F64Positive,
    HashMapNonEmpty,
    HashSetNonEmpty,
    I8NonNegative,
    I8NonZero,
    // Signed integer types
    I8Positive,
    // Range types
    I8Range,
    I16NonNegative,
    I16NonZero,
    I16Positive,
    I16Range,
    I32NonNegative,
    I32NonZero,
    I32Positive,
    I32Range,
    I64NonNegative,
    I64NonZero,
    I64Positive,
    I64Range,
    I128NonNegative,
    I128NonZero,
    I128Positive,
    I128Range,
    IsizeNonNegative,
    IsizeNonZero,
    IsizePositive,
    IsizeRange,
    // Networks
    IpPrivate,
    IpPublic,
    IpV4,
    IpV6,
    Ipv4Loopback,
    Ipv6Loopback,
    LinkedListNonEmpty,
    OptionSome,
    // Paths
    PathBufExists,
    PathBufIsDir,
    PathBufIsFile,
    PathBufReadable,
    ResultOk,
    // Strings
    StringNonEmpty,
    // Tuples
    Tuple2,
    Tuple3,
    Tuple4,
    U8NonZero,
    // Unsigned integer types
    U8Positive,
    U8Range,
    U16NonZero,
    U16Positive,
    U16Range,
    U32NonZero,
    U32Positive,
    U32Range,
    U64NonZero,
    U64Positive,
    U64Range,
    U128NonZero,
    U128Positive,
    U128Range,
    UsizeNonZero,
    UsizePositive,
    UsizeRange,
    // Error type
    ValidationError,
    VecAllSatisfy,
    VecDequeNonEmpty,
    // Collections
    VecNonEmpty,
};

// Feature-gated imports
#[cfg(feature = "uuid")]
use elicitation::{UuidNonNil, UuidV4};

#[cfg(feature = "url")]
use elicitation::{UrlCanBeBase, UrlHttp, UrlHttps, UrlValid, UrlWithHost};

#[cfg(feature = "regex")]
use elicitation::{RegexCaseInsensitive, RegexMultiline, RegexSetNonEmpty, RegexSetValid, RegexValid};

#[cfg(feature = "chrono")]
use elicitation::{DateTimeUtcAfter, DateTimeUtcBefore, NaiveDateTimeAfter};

#[cfg(feature = "time")]
use elicitation::{OffsetDateTimeAfter, OffsetDateTimeBefore};

#[cfg(feature = "jiff")]
use elicitation::{TimestampAfter, TimestampBefore};

// Module declarations
mod bools;
mod chars;
mod collections;
mod durations;
mod floats;
mod integers;
mod mechanisms;
mod networks;
mod strings;

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
