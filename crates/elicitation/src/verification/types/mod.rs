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

pub use integers::*;
pub use floats::*;
pub use strings::*;
pub use bools::*;
pub use chars::*;
pub use uuids::*;
pub use durations::*;
pub use networks::*;
pub use pathbufs::*;
pub use datetimes::*;
pub use tuples::*;
pub use collections::*;
pub use values::*;

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
}

impl std::error::Error for ValidationError {}
