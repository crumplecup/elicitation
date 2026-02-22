//! Creusot verification proofs for elicitation contract types.
//!
//! This crate contains pure Rust proofs that can be verified by Creusot.
//! It imports contract types from the main elicitation crate but avoids
//! async code that Creusot cannot handle.

#![forbid(unsafe_code)]

use creusot_std::prelude::*;

// Import base types needed for proof structure
use elicitation::{
    BoolFalse,
    // Bool types
    BoolTrue,
    // Char types
    CharAlphabetic,
    CharAlphanumeric,
    CharNumeric,
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
    IsizeNonNegative,
    IsizeNonZero,
    IsizePositive,
    IsizeRange,
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
    UsizeNonZero,
    UsizePositive,
    UsizeRange,
    // Error type
    ValidationError,
};

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
