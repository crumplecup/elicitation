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

mod bools;
mod chars;
mod collections;
mod datetimes;
mod durations;
mod floats;
mod integers;
mod ipaddr_bytes;
mod macaddr;
mod networks;
mod pathbufs;
mod pathbytes;
mod regexbytes;
mod socketaddr;
mod strings;
mod tuples;
mod urlbytes;
mod utf8;
mod uuid_bytes;
mod uuids;
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

// UUID Foundation (Kani proofs only)
#[cfg(kani)]
pub use uuid_bytes::{
    UuidBytes, UuidV4Bytes, UuidV7Bytes, has_valid_variant, has_version, is_valid_v4, is_valid_v7,
};

// IP Address Foundation
pub use ipaddr_bytes::{
    Ipv4Bytes, Ipv4Private, Ipv4Public, Ipv6Bytes, Ipv6Private, Ipv6Public, is_ipv4_private,
    is_ipv6_private,
};

// MAC Address Foundation
pub use macaddr::{
    MacAddr, MacLocal, MacMulticast, MacUnicast, MacUniversal, is_local, is_multicast, is_unicast,
    is_universal,
};

// Socket Address Foundation
pub use socketaddr::{
    SocketAddrV4Bytes, SocketAddrV4NonZero, SocketAddrV4Privileged, SocketAddrV4Unprivileged,
    SocketAddrV6Bytes, SocketAddrV6NonZero, SocketAddrV6Privileged, SocketAddrV6Unprivileged,
    is_dynamic_port, is_nonzero_port, is_privileged_port, is_registered_port, is_well_known_port,
};

// Path Foundation (Unix)
#[cfg(unix)]
pub use pathbytes::{
    PathAbsolute, PathBytes, PathNonEmpty, PathRelative, has_null_byte, is_absolute, is_relative,
};

// URL Foundation
pub use urlbytes::{
    AuthorityBytes, SchemeBytes, UrlAbsoluteBytes, UrlBytes, UrlHttpBytes, UrlWithAuthorityBytes,
};

// Regex Foundation
pub use regexbytes::{
    BalancedDelimiters, RegexBytes, ValidCharClass, ValidEscapes, ValidQuantifiers,
};

// Integers
pub use integers::{
    I8Default, // MCP wrapper
    I8NonNegative,
    I8NonZero,
    I8NonZeroStyle,
    // i8 family
    I8Positive,
    I8Range,
    I8RangeStyle,
    I16Default, // MCP wrapper
    I16NonNegative,
    I16NonZero,
    I16NonZeroStyle,
    // i16 family
    I16Positive,
    I16Range,
    I16RangeStyle,
    I32Default, // MCP wrapper
    I32NonNegative,
    I32NonZero,
    // i32 family
    I32Positive,
    I32Range,
    I32RangeStyle,
    I64Default, // MCP wrapper
    I64NonNegative,
    I64NonZero,
    // i64 family
    I64Positive,
    I64Range,
    I64RangeStyle,
    I128Default, // MCP wrapper
    I128NonNegative,
    I128NonZero,
    // i128 family
    I128Positive,
    I128Range,
    I128RangeStyle,
    IsizeDefault, // MCP wrapper
    IsizeNonNegative,
    IsizeNonZero,
    // isize family
    IsizePositive,
    IsizeRange,
    IsizeRangeStyle,
    U8Default, // MCP wrapper
    U8NonZero,
    // u8 family
    U8Positive,
    U8Range,
    U8RangeStyle,
    U16Default, // MCP wrapper
    U16NonZero,
    // u16 family
    U16Positive,
    U16Range,
    U16RangeStyle,
    U32Default, // MCP wrapper
    U32NonZero,
    // u32 family
    U32Positive,
    U32Range,
    U32RangeStyle,
    U64Default, // MCP wrapper
    U64NonZero,
    // u64 family
    U64Positive,
    U64Range,
    U64RangeStyle,
    U128Default, // MCP wrapper
    U128NonZero,
    // u128 family
    U128Positive,
    U128Range,
    U128RangeStyle,
    UsizeDefault, // MCP wrapper
    UsizeNonZero,
    // usize family
    UsizePositive,
    UsizeRange,
    UsizeRangeStyle,
};

// Floats
pub use floats::{
    F32Default, // MCP wrapper
    F32Finite,
    F32NonNegative,
    F32Positive,
    F64Default, // MCP wrapper
    F64Finite,
    F64NonNegative,
    F64Positive,
};

// Bools
pub use bools::{BoolDefault, BoolFalse, BoolTrue};

// Chars
pub use chars::{CharAlphabetic, CharAlphanumeric, CharNumeric};

// Strings
pub use strings::{StringDefault, StringNonEmpty};

// Collections
pub use collections::{
    ArcNonNull, ArcSatisfies, ArrayAllSatisfy, BTreeMapNonEmpty, BTreeSetNonEmpty, BoxNonNull,
    BoxSatisfies, HashMapNonEmpty, HashSetNonEmpty, LinkedListNonEmpty, OptionSome, RcNonNull,
    RcSatisfies, ResultOk, VecAllSatisfy, VecDequeNonEmpty, VecNonEmpty,
};

// Tuples
pub use tuples::{Tuple2, Tuple3, Tuple4};

// Durations
pub use durations::{DurationNonZero, DurationPositive};

// Networks
pub use networks::{IpPrivate, IpPublic, IpV4, IpV6, Ipv4Loopback, Ipv6Loopback};

// Paths
pub use pathbufs::{PathBufExists, PathBufIsDir, PathBufIsFile, PathBufReadable};

// UUIDs (feature-gated)
#[cfg(feature = "uuid")]
pub use uuids::{UuidNonNil, UuidV4};

// DateTimes (feature-gated on chrono/time/jiff)
#[cfg(feature = "chrono")]
pub use datetimes::{DateTimeUtcAfter, DateTimeUtcBefore, NaiveDateTimeAfter};

#[cfg(feature = "time")]
pub use datetimes::{OffsetDateTimeAfter, OffsetDateTimeBefore};

#[cfg(feature = "jiff")]
pub use datetimes::{TimestampAfter, TimestampBefore};

// Values (JSON - feature gated on serde_json but it might be non-optional)
#[cfg(feature = "serde_json")]
pub use values::{ValueArray, ValueNonNull, ValueObject};

// URLs (feature-gated)
#[cfg(feature = "url")]
pub use urls::{UrlCanBeBase, UrlHttp, UrlHttps, UrlValid, UrlWithHost};

// Regexes (feature-gated)
#[cfg(feature = "regex")]
pub use regexes::{
    RegexCaseInsensitive, RegexMultiline, RegexSetNonEmpty, RegexSetValid, RegexValid,
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

    /// IP address is not private (not in RFC 1918 or RFC 4193 ranges).
    #[display("IP address must be private (RFC 1918/4193)")]
    NotPrivateIp,

    /// IP address is not public.
    #[display("IP address must be public (not RFC 1918/4193)")]
    NotPublicIp,

    /// Expected IPv4, got IPv6.
    #[display("Expected IPv4 address, got IPv6")]
    ExpectedIpv4GotIpv6,

    /// Expected IPv6, got IPv4.
    #[display("Expected IPv6 address, got IPv4")]
    ExpectedIpv6GotIpv4,

    /// IP address is not loopback.
    #[display("IP address must be loopback")]
    NotLoopback,

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

    /// URL syntax is invalid (RFC 3986).
    #[display("URL syntax is invalid")]
    InvalidUrlSyntax,

    /// URL is missing authority component.
    #[display("URL must have authority (//host)")]
    UrlMissingAuthority,

    /// URL is not absolute (missing scheme + authority).
    #[display("URL must be absolute (scheme://host)")]
    UrlNotAbsolute,

    /// URL scheme is not HTTPS.
    #[display("URL must use HTTPS scheme")]
    UrlNotHttps,

    /// URL scheme is not HTTP.
    #[display("URL must use HTTP or HTTPS scheme")]
    UrlNotHttp,

    /// Regex syntax is invalid.
    #[display("Regex syntax is invalid")]
    InvalidRegexSyntax,

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

    /// MAC address is not unicast (is multicast).
    #[display("MAC address must be unicast")]
    NotUnicastMac,

    /// MAC address is not multicast (is unicast).
    #[display("MAC address must be multicast")]
    NotMulticastMac,

    /// MAC address is not universal (is locally administered).
    #[display("MAC address must be universal (IEEE assigned)")]
    NotUniversalMac,

    /// MAC address is not locally administered (is universal).
    #[display("MAC address must be locally administered")]
    NotLocalMac,

    /// Port number is zero (invalid for binding).
    #[display("Port must be non-zero")]
    PortIsZero,

    /// Port number is not privileged (>= 1024).
    #[display("Port must be privileged (< 1024), got {}", _0)]
    PortNotPrivileged(u16),

    /// Port number is privileged (< 1024).
    #[display("Port must be unprivileged (>= 1024), got {}", _0)]
    PortIsPrivileged(u16),

    /// Path contains null byte (invalid on Unix).
    #[display("Path contains null byte")]
    PathContainsNull,

    /// Path is not absolute (does not start with /).
    #[display("Path must be absolute (start with /), got: {}", _0)]
    PathNotAbsolute(String),

    /// Path is not relative (starts with /).
    #[display("Path must be relative (not start with /), got: {}", _0)]
    PathNotRelative(String),

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
