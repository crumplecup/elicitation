//! Contract types for the trenchcoat pattern.
//!
//! Contract types validate data at construction boundaries, then unwrap to stdlib types.
//!
//! # The Trenchcoat Pattern
//!
//! ```text
//! INPUT → Put on coat → VALIDATE → Take off coat → OUTPUT
//! LLM     (wrap)        (contract)  (unwrap)       stdlib
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! // Contract type validates on construction
//! let validated = I8Positive::elicit(client).await?;
//!
//! // Unwrap to familiar stdlib type
//! let value: i8 = validated.into_inner();
//! ```

/// Trait for types that validate other types.
///
/// Used in Prusti proofs to express that a contract validates a base type.
pub trait ValidatesType<T> {
    /// Validates that a value conforms to this contract.
    fn validates(value: &T) -> bool;
}

mod utf8;
mod uuid_bytes;
mod ipaddr_bytes;
mod integers;
mod floats;
mod strings;
mod bools;
mod chars;
mod uuids;
mod durations;
mod networks;
mod pathbufs;
mod datetimes;
mod tuples;
mod collections;
mod values;

#[cfg(feature = "url")]
mod urls;

#[cfg(feature = "regex")]
mod regexes;

#[cfg(kani)]
mod kani_proofs;

#[cfg(feature = "verify-verus")]
mod verus_proofs;

#[cfg(feature = "verify-creusot")]
mod creusot_proofs;

#[cfg(feature = "verify-prusti")]
mod prusti_proofs;

// Explicit exports (no globs - helps compiler show what's missing)

// UTF-8 Foundation
pub use utf8::{Utf8Bytes, is_valid_utf8};

// UUID Foundation (behind feature)
#[cfg(feature = "uuid")]
pub use uuid_bytes::{
    UuidBytes, UuidV4Bytes, UuidV7Bytes,
    has_valid_variant, has_version, is_valid_v4, is_valid_v7,
};

// IP Address Foundation
pub use ipaddr_bytes::{
    Ipv4Bytes, Ipv4Private, Ipv4Public,
    Ipv6Bytes, Ipv6Private, Ipv6Public,
    is_ipv4_private, is_ipv6_private,
};

// Integers
pub use integers::{
    // i8 family
    I8Positive, I8NonNegative, I8NonZero, I8Range, I8RangeStyle, I8NonZeroStyle,
    // i16 family
    I16Positive, I16NonNegative, I16NonZero, I16Range, I16RangeStyle, I16NonZeroStyle,
    // i32 family
    I32Positive, I32NonNegative, I32NonZero, I32Range, I32RangeStyle,
    // i64 family
    I64Positive, I64NonNegative, I64NonZero, I64Range, I64RangeStyle,
    // i128 family
    I128Positive, I128NonNegative, I128NonZero, I128Range, I128RangeStyle,
    // isize family
    IsizePositive, IsizeNonNegative, IsizeNonZero, IsizeRange, IsizeRangeStyle,
    // u8 family
    U8Positive, U8NonZero, U8Range, U8RangeStyle,
    // u16 family
    U16Positive, U16NonZero, U16Range, U16RangeStyle,
    // u32 family
    U32Positive, U32NonZero, U32Range, U32RangeStyle,
    // u64 family
    U64Positive, U64NonZero, U64Range, U64RangeStyle,
    // u128 family
    U128Positive, U128NonZero, U128Range, U128RangeStyle,
    // usize family
    UsizePositive, UsizeNonZero, UsizeRange, UsizeRangeStyle,
};

// Floats
pub use floats::{
    F32Positive, F32NonNegative, F32Finite,
    F64Positive, F64NonNegative, F64Finite,
};

// Bools
pub use bools::{BoolTrue, BoolFalse};

// Chars
pub use chars::{CharAlphabetic, CharNumeric, CharAlphanumeric};

// Strings
pub use strings::StringNonEmpty;

// Collections
pub use collections::{
    VecNonEmpty, VecAllSatisfy,
    OptionSome, ResultOk,
    BoxSatisfies, ArcSatisfies, RcSatisfies,
    BoxNonNull, ArcNonNull, RcNonNull,
    HashMapNonEmpty, BTreeMapNonEmpty,
    HashSetNonEmpty, BTreeSetNonEmpty,
    VecDequeNonEmpty, LinkedListNonEmpty,
    ArrayAllSatisfy,
};

// Tuples
pub use tuples::{Tuple2, Tuple3, Tuple4};

// Durations
pub use durations::{DurationPositive, DurationNonZero};

// Networks
pub use networks::{
    IpPrivate, IpPublic, IpV4, IpV6,
    Ipv4Loopback, Ipv6Loopback,
};

// Paths
pub use pathbufs::{
    PathBufExists, PathBufReadable,
    PathBufIsDir, PathBufIsFile,
};

// UUIDs (feature-gated)
#[cfg(feature = "uuid")]
pub use uuids::{UuidV4, UuidNonNil};

// DateTimes (feature-gated on chrono/time/jiff)
#[cfg(feature = "chrono")]
pub use datetimes::{
    DateTimeUtcAfter, DateTimeUtcBefore,
    NaiveDateTimeAfter,
};

#[cfg(feature = "time")]
pub use datetimes::{
    OffsetDateTimeAfter, OffsetDateTimeBefore,
};

#[cfg(feature = "jiff")]
pub use datetimes::{
    TimestampAfter, TimestampBefore,
};

// Values (JSON - feature gated on serde_json but it might be non-optional)
#[cfg(feature = "serde_json")]
pub use values::{ValueObject, ValueArray, ValueNonNull};

// URLs (feature-gated)
#[cfg(feature = "url")]
pub use urls::{
    UrlValid, UrlHttps, UrlHttp,
    UrlWithHost, UrlCanBeBase,
};

// Regexes (feature-gated)
#[cfg(feature = "regex")]
pub use regexes::{
    RegexValid, RegexSetValid,
    RegexCaseInsensitive, RegexMultiline,
    RegexSetNonEmpty,
};

/// Error type for contract validation failures.
#[derive(Debug, Clone, PartialEq, derive_more::Display)]
pub enum ValidationError {
    /// Value is not positive (must be > 0).
    #[display("Value must be positive (> 0), got {}", _0)]
    NotPositive(i128),

    /// Value is negative (must be >= 0).
    #[display("Value must be non-negative (>= 0), got {}", _0)]
    Negative(i128),

    /// Value is zero (must be non-zero).
    #[display("Value must be non-zero")]
    Zero,

    /// Value is not finite (NaN or infinite).
    #[display("Value must be finite (not NaN or infinite), got {}", _0)]
    NotFinite(String),

    /// Float value is not positive (must be > 0.0).
    #[display("Value must be positive (> 0.0), got {}", _0)]
    FloatNotPositive(f64),

    /// Float value is negative (must be >= 0.0).
    #[display("Value must be non-negative (>= 0.0), got {}", _0)]
    FloatNegative(f64),

    /// Float value is out of range.
    #[display("Value {} is outside range [{}, {}]", value, min, max)]
    FloatOutOfRange {
        /// The value that was out of range.
        value: f64,
        /// Minimum allowed value.
        min: f64,
        /// Maximum allowed value.
        max: f64,
    },

    /// Value is out of range.
    #[display("Value {} is outside range [{}, {}]", value, min, max)]
    OutOfRange {
        /// The value that was out of range.
        value: i128,
        /// Minimum allowed value.
        min: i128,
        /// Maximum allowed value.
        max: i128,
    },

    /// String is empty (must be non-empty).
    #[display("String must be non-empty")]
    EmptyString,

    /// String exceeds maximum length.
    #[display("String length {} exceeds maximum {}", actual, max)]
    StringTooLong {
        /// Actual length.
        actual: usize,
        /// Maximum allowed length.
        max: usize,
    },

    /// String is below minimum length.
    #[display("String length {} is below minimum {}", actual, min)]
    StringTooShort {
        /// Actual length.
        actual: usize,
        /// Minimum allowed length.
        min: usize,
    },

    /// Bool value is not true (must be true).
    #[display("Value must be true, got false")]
    NotTrue,

    /// Bool value is not false (must be false).
    #[display("Value must be false, got true")]
    NotFalse,

    /// Char is not alphabetic.
    #[display("Character '{}' is not alphabetic", _0)]
    NotAlphabetic(char),

    /// Char is not numeric.
    #[display("Character '{}' is not numeric", _0)]
    NotNumeric(char),

    /// Char is not alphanumeric.
    #[display("Character '{}' is not alphanumeric", _0)]
    NotAlphanumeric(char),

    /// UUID is wrong version.
    #[display("UUID must be version {}, got version {}", expected, got)]
    WrongUuidVersion {
        /// Expected version number.
        expected: u8,
        /// Actual version number.
        got: u8,
    },

    /// UUID is nil.
    #[display("UUID must be non-nil (not all zeros)")]
    NilUuid,

    /// Duration is not positive (must be > zero).
    #[display("Duration must be positive (not zero)")]
    DurationNotPositive,

    /// IP address is not private.
    #[display("IP address must be private (RFC 1918/4193), got {}", _0)]
    NotPrivateIp(String),

    /// IP address is not public.
    #[display("IP address must be public (not RFC 1918/4193), got {}", _0)]
    NotPublicIp(String),

    /// IP address is wrong version.
    #[display("Expected {} address, got {}", expected, got)]
    WrongIpVersion {
        /// Expected IP version.
        expected: String,
        /// Actual IP version.
        got: String,
    },

    /// IP address is not loopback.
    #[display("IP address must be loopback, got {}", _0)]
    NotLoopback(String),

    /// Path does not exist on filesystem.
    #[display("Path does not exist: {}", _0)]
    PathDoesNotExist(String),

    /// Path is not readable.
    #[display("Path is not readable: {}", _0)]
    PathNotReadable(String),

    /// Path is not a directory.
    #[display("Path is not a directory: {}", _0)]
    PathNotDirectory(String),

    /// Path is not a file.
    #[display("Path is not a file: {}", _0)]
    PathNotFile(String),

    /// DateTime is too early (before threshold).
    #[display("DateTime must be after {}, got {}", threshold, value)]
    DateTimeTooEarly {
        /// The value that was too early.
        value: String,
        /// The threshold it must be after.
        threshold: String,
    },

    /// DateTime is too late (after threshold).
    #[display("DateTime must be before {}, got {}", threshold, value)]
    DateTimeTooLate {
        /// The value that was too late.
        value: String,
        /// The threshold it must be before.
        threshold: String,
    },

    /// Collection is empty (must be non-empty).
    #[display("Collection must be non-empty")]
    EmptyCollection,

    /// Option is None (must be Some).
    #[display("Option must be Some, not None")]
    OptionIsNone,

    /// Result is Err (must be Ok).
    #[display("Result must be Ok, not Err")]
    ResultIsErr,

    /// JSON Value is wrong type.
    #[display("JSON must be {}, got {}", expected, got)]
    WrongJsonType {
        /// Expected JSON type.
        expected: String,
        /// Actual JSON type.
        got: String,
    },

    /// JSON Value is null.
    #[display("JSON must be non-null")]
    JsonIsNull,

    /// URL is invalid or malformed.
    #[display("URL is invalid or cannot be parsed")]
    UrlInvalid,

    /// URL scheme is not HTTPS.
    #[display("URL must use HTTPS scheme")]
    UrlNotHttps,

    /// URL scheme is not HTTP.
    #[display("URL must use HTTP scheme")]
    UrlNotHttp,

    /// URL has no host component.
    #[display("URL must have a host")]
    UrlNoHost,

    /// URL cannot be a base for relative URLs.
    #[display("URL cannot be used as a base")]
    UrlCannotBeBase,

    /// Regex pattern is invalid or cannot be compiled.
    #[display("Regex pattern is invalid or cannot be compiled")]
    RegexInvalid,

    /// UUID variant bits are invalid (not RFC 4122 10xx pattern).
    #[display("Invalid UUID variant bits")]
    InvalidUuidVariant,

    /// UTF-8 validation failed.
    #[display("Invalid UTF-8 byte sequence")]
    InvalidUtf8,

    /// Value exceeds maximum length (generic).
    #[display("Value too long: max {max}, got {actual}")]
    TooLong {
        /// Maximum allowed length.
        max: usize,
        /// Actual length.
        actual: usize,
    },
}

impl std::error::Error for ValidationError {}
