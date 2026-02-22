//! Creusot verification proofs for elicitation contract types.
//!
//! This crate contains pure Rust proofs that can be verified by Creusot.
//! It imports contract types from the main elicitation crate but avoids
//! async code that Creusot cannot handle.

#![forbid(unsafe_code)]

use creusot_std::prelude::*;

// Import contract types from elicitation
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
    // String types
    StringNonEmpty,
    // Float types
    F32Positive,
    F32NonNegative,
    F32Finite,
    F64Positive,
    F64NonNegative,
    F64Finite,
    // Duration types
    DurationPositive,
    // Tuple types
    Tuple2,
    Tuple3,
    Tuple4,
    // Collection types
    VecNonEmpty,
    VecAllSatisfy,
    OptionSome,
    ResultOk,
    BoxSatisfies,
    ArcSatisfies,
    RcSatisfies,
    HashMapNonEmpty,
    BTreeMapNonEmpty,
    HashSetNonEmpty,
    BTreeSetNonEmpty,
    VecDequeNonEmpty,
    LinkedListNonEmpty,
    ArrayAllSatisfy,
    BoxNonNull,
    ArcNonNull,
    RcNonNull,
    // Network types
    IpPrivate,
    IpPublic,
    IpV4,
    IpV6,
    Ipv4Loopback,
    Ipv6Loopback,
    // Path types
    PathBufExists,
    PathBufIsDir,
    PathBufIsFile,
    PathBufReadable,
    // Error type
    ValidationError,
};

// Feature-gated imports
#[cfg(feature = "uuid")]
use elicitation::{UuidNonNil, UuidV4};

#[cfg(feature = "serde_json")]
use elicitation::{ValueArray, ValueNonNull, ValueObject};

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
