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

pub use integers::*;
pub use floats::*;
pub use strings::*;
pub use bools::*;
pub use chars::*;

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
}

impl std::error::Error for ValidationError {}
