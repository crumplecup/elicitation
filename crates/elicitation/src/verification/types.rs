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

use crate::{ElicitClient, ElicitResult, Elicitation, Prompt};

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

/// Contract type for positive i8 values (> 0).
///
/// This type validates on construction and can be unwrapped to `i8`.
///
/// # Trenchcoat Pattern
///
/// ```text
/// LLM → I8Positive::new(value) → validate → .into_inner() → i8
///       ^^^^^^^^^^^^^^^^^^^^     ^^^^^^^^   ^^^^^^^^^^^^^   ^^^
///       Put on coat              Check      Take off coat   Familiar
/// ```
///
/// # Example
///
/// ```rust,ignore
/// // Validation at construction
/// let positive = I8Positive::new(42)?;
/// assert_eq!(positive.get(), 42);
///
/// // Unwrap to stdlib type
/// let value: i8 = positive.into_inner();
/// assert_eq!(value, 42);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct I8Positive(i8);

impl I8Positive {
    /// Creates a new `I8Positive` if the value is positive (> 0).
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NotPositive` if value <= 0.
    pub fn new(value: i8) -> Result<Self, ValidationError> {
        if value > 0 {
            Ok(Self(value))
        } else {
            Err(ValidationError::NotPositive(value as i128))
        }
    }

    /// Gets the inner value.
    pub fn get(&self) -> i8 {
        self.0
    }

    /// Unwraps to the inner `i8` (trenchcoat pattern: take off coat).
    ///
    /// This is the "output boundary" of the trenchcoat pattern.
    pub fn into_inner(self) -> i8 {
        self.0
    }
}

// Generate default-only style enum for I8Positive
crate::default_style!(I8Positive => I8PositiveStyle);

impl Prompt for I8Positive {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a positive number (> 0):")
    }
}

impl Elicitation for I8Positive {
    type Style = I8PositiveStyle;

    #[tracing::instrument(skip(client), fields(type_name = "I8Positive"))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting I8Positive (positive i8 value)");

        loop {
            // Elicit base i8 value
            let value = i8::elicit(client).await?;
            
            // Try to construct I8Positive (validates)
            match Self::new(value) {
                Ok(positive) => {
                    tracing::debug!(value, "Valid I8Positive constructed");
                    return Ok(positive);
                }
                Err(e) => {
                    tracing::warn!(value, error = %e, "Invalid I8Positive, re-prompting");
                    // Loop continues, will re-prompt
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i8_positive_new_valid() {
        let result = I8Positive::new(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 1);
    }

    #[test]
    fn i8_positive_new_zero_invalid() {
        let result = I8Positive::new(0);
        assert!(result.is_err());
    }

    #[test]
    fn i8_positive_new_negative_invalid() {
        let result = I8Positive::new(-1);
        assert!(result.is_err());
    }

    #[test]
    fn i8_positive_into_inner() {
        let positive = I8Positive::new(42).unwrap();
        let value: i8 = positive.into_inner();
        assert_eq!(value, 42);
    }
}

// ============================================================================
// I8NonNegative (i8 >= 0)
// ============================================================================

/// Contract type for non-negative i8 values (>= 0).
///
/// Validates on construction, then can unwrap to stdlib i8 via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct I8NonNegative(i8);

impl I8NonNegative {
    /// Constructs a non-negative i8 value.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Negative` if value < 0.
    pub fn new(value: i8) -> Result<Self, ValidationError> {
        if value >= 0 {
            Ok(Self(value))
        } else {
            Err(ValidationError::Negative(value as i128))
        }
    }

    /// Gets the wrapped value.
    pub fn get(&self) -> i8 {
        self.0
    }

    /// Unwraps to stdlib i8 (trenchcoat off).
    pub fn into_inner(self) -> i8 {
        self.0
    }
}

crate::default_style!(I8NonNegative => I8NonNegativeStyle);

impl Prompt for I8NonNegative {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a non-negative number (>= 0):")
    }
}

impl Elicitation for I8NonNegative {
    type Style = I8NonNegativeStyle;

    #[tracing::instrument(skip(client), fields(type_name = "I8NonNegative"))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting I8NonNegative (non-negative i8 value)");

        loop {
            let value = i8::elicit(client).await?;
            
            match Self::new(value) {
                Ok(non_negative) => {
                    tracing::debug!(value, "Valid I8NonNegative constructed");
                    return Ok(non_negative);
                }
                Err(e) => {
                    tracing::warn!(value, error = %e, "Invalid I8NonNegative, re-prompting");
                }
            }
        }
    }
}

// ============================================================================
// I8Range (MIN <= i8 <= MAX)
// ============================================================================

/// Contract type for i8 values within a specified range [MIN, MAX].
///
/// Validates on construction, then can unwrap to stdlib i8 via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct I8Range<const MIN: i8, const MAX: i8>(i8);

impl<const MIN: i8, const MAX: i8> I8Range<MIN, MAX> {
    /// Constructs an i8 value within the specified range.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::OutOfRange` if value not in [MIN, MAX].
    pub fn new(value: i8) -> Result<Self, ValidationError> {
        if value >= MIN && value <= MAX {
            Ok(Self(value))
        } else {
            Err(ValidationError::OutOfRange {
                value: value as i128,
                min: MIN as i128,
                max: MAX as i128,
            })
        }
    }

    /// Gets the wrapped value.
    pub fn get(&self) -> i8 {
        self.0
    }

    /// Unwraps to stdlib i8 (trenchcoat off).
    pub fn into_inner(self) -> i8 {
        self.0
    }
}

// Manual style enum for I8Range (const generics don't work with macros)
/// Default-only style enum for I8Range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum I8RangeStyle {
    /// Default presentation style.
    #[default]
    Default,
}

impl crate::Prompt for I8RangeStyle {
    fn prompt() -> Option<&'static str> {
        None // No style selection needed
    }
}

impl crate::Elicitation for I8RangeStyle {
    type Style = I8RangeStyle;

    async fn elicit(_client: &crate::ElicitClient<'_>) -> crate::ElicitResult<Self> {
        Ok(Self::Default)
    }
}

impl<const MIN: i8, const MAX: i8> Prompt for I8Range<MIN, MAX> {
    fn prompt() -> Option<&'static str> {
        // TODO: Dynamic prompt showing MIN/MAX
        Some("Please enter a number within the specified range:")
    }
}

impl<const MIN: i8, const MAX: i8> Elicitation for I8Range<MIN, MAX> {
    type Style = I8RangeStyle;

    #[tracing::instrument(skip(client), fields(type_name = "I8Range", min = MIN, max = MAX))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting I8Range<{}, {}> (i8 in range)", MIN, MAX);

        loop {
            let value = i8::elicit(client).await?;
            
            match Self::new(value) {
                Ok(ranged) => {
                    tracing::debug!(value, min = MIN, max = MAX, "Valid I8Range constructed");
                    return Ok(ranged);
                }
                Err(e) => {
                    tracing::warn!(value, error = %e, min = MIN, max = MAX, "Invalid I8Range, re-prompting");
                }
            }
        }
    }
}

#[cfg(test)]
mod i8_nonnegative_tests {
    use super::*;

    #[test]
    fn i8_nonnegative_new_valid_positive() {
        let result = I8NonNegative::new(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 42);
    }

    #[test]
    fn i8_nonnegative_new_valid_zero() {
        let result = I8NonNegative::new(0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 0);
    }

    #[test]
    fn i8_nonnegative_new_negative_invalid() {
        let result = I8NonNegative::new(-1);
        assert!(result.is_err());
    }

    #[test]
    fn i8_nonnegative_into_inner() {
        let non_neg = I8NonNegative::new(10).unwrap();
        let value: i8 = non_neg.into_inner();
        assert_eq!(value, 10);
    }
}

#[cfg(test)]
mod i8_range_tests {
    use super::*;

    #[test]
    fn i8_range_new_valid_within_range() {
        let result = I8Range::<10, 20>::new(15);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 15);
    }

    #[test]
    fn i8_range_new_valid_at_min() {
        let result = I8Range::<10, 20>::new(10);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 10);
    }

    #[test]
    fn i8_range_new_valid_at_max() {
        let result = I8Range::<10, 20>::new(20);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 20);
    }

    #[test]
    fn i8_range_new_below_min_invalid() {
        let result = I8Range::<10, 20>::new(9);
        assert!(result.is_err());
    }

    #[test]
    fn i8_range_new_above_max_invalid() {
        let result = I8Range::<10, 20>::new(21);
        assert!(result.is_err());
    }

    #[test]
    fn i8_range_into_inner() {
        let ranged = I8Range::<10, 20>::new(15).unwrap();
        let value: i8 = ranged.into_inner();
        assert_eq!(value, 15);
    }
}

// ============================================================================
// I16Positive (i16 > 0)
// ============================================================================

/// Contract type for positive i16 values (> 0).
///
/// Validates on construction, then can unwrap to stdlib i16 via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct I16Positive(i16);

impl I16Positive {
    /// Constructs a positive i16 value.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NotPositive` if value <= 0.
    pub fn new(value: i16) -> Result<Self, ValidationError> {
        if value > 0 {
            Ok(Self(value))
        } else {
            Err(ValidationError::NotPositive(value as i128))
        }
    }

    /// Gets the wrapped value.
    pub fn get(&self) -> i16 {
        self.0
    }

    /// Unwraps to stdlib i16 (trenchcoat off).
    pub fn into_inner(self) -> i16 {
        self.0
    }
}

crate::default_style!(I16Positive => I16PositiveStyle);

impl Prompt for I16Positive {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a positive number (> 0):")
    }
}

impl Elicitation for I16Positive {
    type Style = I16PositiveStyle;

    #[tracing::instrument(skip(client), fields(type_name = "I16Positive"))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting I16Positive (positive i16 value)");

        loop {
            let value = i16::elicit(client).await?;
            
            match Self::new(value) {
                Ok(positive) => {
                    tracing::debug!(value, "Valid I16Positive constructed");
                    return Ok(positive);
                }
                Err(e) => {
                    tracing::warn!(value, error = %e, "Invalid I16Positive, re-prompting");
                }
            }
        }
    }
}

// ============================================================================
// I16NonNegative (i16 >= 0)
// ============================================================================

/// Contract type for non-negative i16 values (>= 0).
///
/// Validates on construction, then can unwrap to stdlib i16 via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct I16NonNegative(i16);

impl I16NonNegative {
    /// Constructs a non-negative i16 value.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Negative` if value < 0.
    pub fn new(value: i16) -> Result<Self, ValidationError> {
        if value >= 0 {
            Ok(Self(value))
        } else {
            Err(ValidationError::Negative(value as i128))
        }
    }

    /// Gets the wrapped value.
    pub fn get(&self) -> i16 {
        self.0
    }

    /// Unwraps to stdlib i16 (trenchcoat off).
    pub fn into_inner(self) -> i16 {
        self.0
    }
}

crate::default_style!(I16NonNegative => I16NonNegativeStyle);

impl Prompt for I16NonNegative {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a non-negative number (>= 0):")
    }
}

impl Elicitation for I16NonNegative {
    type Style = I16NonNegativeStyle;

    #[tracing::instrument(skip(client), fields(type_name = "I16NonNegative"))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting I16NonNegative (non-negative i16 value)");

        loop {
            let value = i16::elicit(client).await?;
            
            match Self::new(value) {
                Ok(non_negative) => {
                    tracing::debug!(value, "Valid I16NonNegative constructed");
                    return Ok(non_negative);
                }
                Err(e) => {
                    tracing::warn!(value, error = %e, "Invalid I16NonNegative, re-prompting");
                }
            }
        }
    }
}

// ============================================================================
// I16Range (MIN <= i16 <= MAX)
// ============================================================================

/// Contract type for i16 values within a specified range [MIN, MAX].
///
/// Validates on construction, then can unwrap to stdlib i16 via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct I16Range<const MIN: i16, const MAX: i16>(i16);

impl<const MIN: i16, const MAX: i16> I16Range<MIN, MAX> {
    /// Constructs an i16 value within the specified range.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::OutOfRange` if value not in [MIN, MAX].
    pub fn new(value: i16) -> Result<Self, ValidationError> {
        if value >= MIN && value <= MAX {
            Ok(Self(value))
        } else {
            Err(ValidationError::OutOfRange {
                value: value as i128,
                min: MIN as i128,
                max: MAX as i128,
            })
        }
    }

    /// Gets the wrapped value.
    pub fn get(&self) -> i16 {
        self.0
    }

    /// Unwraps to stdlib i16 (trenchcoat off).
    pub fn into_inner(self) -> i16 {
        self.0
    }
}

/// Default-only style enum for I16Range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum I16RangeStyle {
    /// Default presentation style.
    #[default]
    Default,
}

impl crate::Prompt for I16RangeStyle {
    fn prompt() -> Option<&'static str> {
        None
    }
}

impl crate::Elicitation for I16RangeStyle {
    type Style = I16RangeStyle;

    async fn elicit(_client: &crate::ElicitClient<'_>) -> crate::ElicitResult<Self> {
        Ok(Self::Default)
    }
}

impl<const MIN: i16, const MAX: i16> Prompt for I16Range<MIN, MAX> {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a number within the specified range:")
    }
}

impl<const MIN: i16, const MAX: i16> Elicitation for I16Range<MIN, MAX> {
    type Style = I16RangeStyle;

    #[tracing::instrument(skip(client), fields(type_name = "I16Range", min = MIN, max = MAX))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting I16Range<{}, {}> (i16 in range)", MIN, MAX);

        loop {
            let value = i16::elicit(client).await?;
            
            match Self::new(value) {
                Ok(ranged) => {
                    tracing::debug!(value, min = MIN, max = MAX, "Valid I16Range constructed");
                    return Ok(ranged);
                }
                Err(e) => {
                    tracing::warn!(value, error = %e, min = MIN, max = MAX, "Invalid I16Range, re-prompting");
                }
            }
        }
    }
}

#[cfg(test)]
mod i16_positive_tests {
    use super::*;

    #[test]
    fn i16_positive_new_valid() {
        let result = I16Positive::new(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 1);
    }

    #[test]
    fn i16_positive_new_zero_invalid() {
        let result = I16Positive::new(0);
        assert!(result.is_err());
    }

    #[test]
    fn i16_positive_new_negative_invalid() {
        let result = I16Positive::new(-1);
        assert!(result.is_err());
    }

    #[test]
    fn i16_positive_into_inner() {
        let positive = I16Positive::new(1000).unwrap();
        let value: i16 = positive.into_inner();
        assert_eq!(value, 1000);
    }
}

#[cfg(test)]
mod i16_nonnegative_tests {
    use super::*;

    #[test]
    fn i16_nonnegative_new_valid_positive() {
        let result = I16NonNegative::new(1000);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 1000);
    }

    #[test]
    fn i16_nonnegative_new_valid_zero() {
        let result = I16NonNegative::new(0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 0);
    }

    #[test]
    fn i16_nonnegative_new_negative_invalid() {
        let result = I16NonNegative::new(-1);
        assert!(result.is_err());
    }

    #[test]
    fn i16_nonnegative_into_inner() {
        let non_neg = I16NonNegative::new(500).unwrap();
        let value: i16 = non_neg.into_inner();
        assert_eq!(value, 500);
    }
}

#[cfg(test)]
mod i16_range_tests {
    use super::*;

    #[test]
    fn i16_range_new_valid_within_range() {
        let result = I16Range::<100, 200>::new(150);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 150);
    }

    #[test]
    fn i16_range_new_valid_at_min() {
        let result = I16Range::<100, 200>::new(100);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 100);
    }

    #[test]
    fn i16_range_new_valid_at_max() {
        let result = I16Range::<100, 200>::new(200);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 200);
    }

    #[test]
    fn i16_range_new_below_min_invalid() {
        let result = I16Range::<100, 200>::new(99);
        assert!(result.is_err());
    }

    #[test]
    fn i16_range_new_above_max_invalid() {
        let result = I16Range::<100, 200>::new(201);
        assert!(result.is_err());
    }

    #[test]
    fn i16_range_into_inner() {
        let ranged = I16Range::<100, 200>::new(150).unwrap();
        let value: i16 = ranged.into_inner();
        assert_eq!(value, 150);
    }
}

// ============================================================================
// U8NonZero (u8 != 0)
// ============================================================================

/// Contract type for non-zero u8 values (!= 0).
///
/// Validates on construction, then can unwrap to stdlib u8 via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct U8NonZero(u8);

impl U8NonZero {
    /// Constructs a non-zero u8 value.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Zero` if value == 0.
    pub fn new(value: u8) -> Result<Self, ValidationError> {
        if value != 0 {
            Ok(Self(value))
        } else {
            Err(ValidationError::Zero)
        }
    }

    /// Gets the wrapped value.
    pub fn get(&self) -> u8 {
        self.0
    }

    /// Unwraps to stdlib u8 (trenchcoat off).
    pub fn into_inner(self) -> u8 {
        self.0
    }
}

crate::default_style!(U8NonZero => U8NonZeroStyle);

impl Prompt for U8NonZero {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a non-zero number (!= 0):")
    }
}

impl Elicitation for U8NonZero {
    type Style = U8NonZeroStyle;

    #[tracing::instrument(skip(client), fields(type_name = "U8NonZero"))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting U8NonZero (non-zero u8 value)");

        loop {
            let value = u8::elicit(client).await?;
            
            match Self::new(value) {
                Ok(non_zero) => {
                    tracing::debug!(value, "Valid U8NonZero constructed");
                    return Ok(non_zero);
                }
                Err(e) => {
                    tracing::warn!(value, error = %e, "Invalid U8NonZero, re-prompting");
                }
            }
        }
    }
}

// ============================================================================
// U8Range (MIN <= u8 <= MAX)
// ============================================================================

/// Contract type for u8 values within a specified range [MIN, MAX].
///
/// Validates on construction, then can unwrap to stdlib u8 via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct U8Range<const MIN: u8, const MAX: u8>(u8);

impl<const MIN: u8, const MAX: u8> U8Range<MIN, MAX> {
    /// Constructs a u8 value within the specified range.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::OutOfRange` if value not in [MIN, MAX].
    pub fn new(value: u8) -> Result<Self, ValidationError> {
        if value >= MIN && value <= MAX {
            Ok(Self(value))
        } else {
            Err(ValidationError::OutOfRange {
                value: value as i128,
                min: MIN as i128,
                max: MAX as i128,
            })
        }
    }

    /// Gets the wrapped value.
    pub fn get(&self) -> u8 {
        self.0
    }

    /// Unwraps to stdlib u8 (trenchcoat off).
    pub fn into_inner(self) -> u8 {
        self.0
    }
}

/// Default-only style enum for U8Range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum U8RangeStyle {
    /// Default presentation style.
    #[default]
    Default,
}

impl crate::Prompt for U8RangeStyle {
    fn prompt() -> Option<&'static str> {
        None
    }
}

impl crate::Elicitation for U8RangeStyle {
    type Style = U8RangeStyle;

    async fn elicit(_client: &crate::ElicitClient<'_>) -> crate::ElicitResult<Self> {
        Ok(Self::Default)
    }
}

impl<const MIN: u8, const MAX: u8> Prompt for U8Range<MIN, MAX> {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a number within the specified range:")
    }
}

impl<const MIN: u8, const MAX: u8> Elicitation for U8Range<MIN, MAX> {
    type Style = U8RangeStyle;

    #[tracing::instrument(skip(client), fields(type_name = "U8Range", min = MIN, max = MAX))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting U8Range<{}, {}> (u8 in range)", MIN, MAX);

        loop {
            let value = u8::elicit(client).await?;
            
            match Self::new(value) {
                Ok(ranged) => {
                    tracing::debug!(value, min = MIN, max = MAX, "Valid U8Range constructed");
                    return Ok(ranged);
                }
                Err(e) => {
                    tracing::warn!(value, error = %e, min = MIN, max = MAX, "Invalid U8Range, re-prompting");
                }
            }
        }
    }
}

// ============================================================================
// U16NonZero (u16 != 0)
// ============================================================================

/// Contract type for non-zero u16 values (!= 0).
///
/// Validates on construction, then can unwrap to stdlib u16 via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct U16NonZero(u16);

impl U16NonZero {
    /// Constructs a non-zero u16 value.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Zero` if value == 0.
    pub fn new(value: u16) -> Result<Self, ValidationError> {
        if value != 0 {
            Ok(Self(value))
        } else {
            Err(ValidationError::Zero)
        }
    }

    /// Gets the wrapped value.
    pub fn get(&self) -> u16 {
        self.0
    }

    /// Unwraps to stdlib u16 (trenchcoat off).
    pub fn into_inner(self) -> u16 {
        self.0
    }
}

crate::default_style!(U16NonZero => U16NonZeroStyle);

impl Prompt for U16NonZero {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a non-zero number (!= 0):")
    }
}

impl Elicitation for U16NonZero {
    type Style = U16NonZeroStyle;

    #[tracing::instrument(skip(client), fields(type_name = "U16NonZero"))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting U16NonZero (non-zero u16 value)");

        loop {
            let value = u16::elicit(client).await?;
            
            match Self::new(value) {
                Ok(non_zero) => {
                    tracing::debug!(value, "Valid U16NonZero constructed");
                    return Ok(non_zero);
                }
                Err(e) => {
                    tracing::warn!(value, error = %e, "Invalid U16NonZero, re-prompting");
                }
            }
        }
    }
}

// ============================================================================
// U16Range (MIN <= u16 <= MAX)
// ============================================================================

/// Contract type for u16 values within a specified range [MIN, MAX].
///
/// Validates on construction, then can unwrap to stdlib u16 via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct U16Range<const MIN: u16, const MAX: u16>(u16);

impl<const MIN: u16, const MAX: u16> U16Range<MIN, MAX> {
    /// Constructs a u16 value within the specified range.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::OutOfRange` if value not in [MIN, MAX].
    pub fn new(value: u16) -> Result<Self, ValidationError> {
        if value >= MIN && value <= MAX {
            Ok(Self(value))
        } else {
            Err(ValidationError::OutOfRange {
                value: value as i128,
                min: MIN as i128,
                max: MAX as i128,
            })
        }
    }

    /// Gets the wrapped value.
    pub fn get(&self) -> u16 {
        self.0
    }

    /// Unwraps to stdlib u16 (trenchcoat off).
    pub fn into_inner(self) -> u16 {
        self.0
    }
}

/// Default-only style enum for U16Range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum U16RangeStyle {
    /// Default presentation style.
    #[default]
    Default,
}

impl crate::Prompt for U16RangeStyle {
    fn prompt() -> Option<&'static str> {
        None
    }
}

impl crate::Elicitation for U16RangeStyle {
    type Style = U16RangeStyle;

    async fn elicit(_client: &crate::ElicitClient<'_>) -> crate::ElicitResult<Self> {
        Ok(Self::Default)
    }
}

impl<const MIN: u16, const MAX: u16> Prompt for U16Range<MIN, MAX> {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a number within the specified range:")
    }
}

impl<const MIN: u16, const MAX: u16> Elicitation for U16Range<MIN, MAX> {
    type Style = U16RangeStyle;

    #[tracing::instrument(skip(client), fields(type_name = "U16Range", min = MIN, max = MAX))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting U16Range<{}, {}> (u16 in range)", MIN, MAX);

        loop {
            let value = u16::elicit(client).await?;
            
            match Self::new(value) {
                Ok(ranged) => {
                    tracing::debug!(value, min = MIN, max = MAX, "Valid U16Range constructed");
                    return Ok(ranged);
                }
                Err(e) => {
                    tracing::warn!(value, error = %e, min = MIN, max = MAX, "Invalid U16Range, re-prompting");
                }
            }
        }
    }
}

#[cfg(test)]
mod u8_nonzero_tests {
    use super::*;

    #[test]
    fn u8_nonzero_new_valid() {
        let result = U8NonZero::new(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 1);
    }

    #[test]
    fn u8_nonzero_new_zero_invalid() {
        let result = U8NonZero::new(0);
        assert!(result.is_err());
    }

    #[test]
    fn u8_nonzero_into_inner() {
        let non_zero = U8NonZero::new(255).unwrap();
        let value: u8 = non_zero.into_inner();
        assert_eq!(value, 255);
    }
}

#[cfg(test)]
mod u8_range_tests {
    use super::*;

    #[test]
    fn u8_range_new_valid_within_range() {
        let result = U8Range::<10, 20>::new(15);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 15);
    }

    #[test]
    fn u8_range_new_valid_at_min() {
        let result = U8Range::<10, 20>::new(10);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 10);
    }

    #[test]
    fn u8_range_new_valid_at_max() {
        let result = U8Range::<10, 20>::new(20);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 20);
    }

    #[test]
    fn u8_range_new_below_min_invalid() {
        let result = U8Range::<10, 20>::new(9);
        assert!(result.is_err());
    }

    #[test]
    fn u8_range_new_above_max_invalid() {
        let result = U8Range::<10, 20>::new(21);
        assert!(result.is_err());
    }

    #[test]
    fn u8_range_into_inner() {
        let ranged = U8Range::<10, 20>::new(15).unwrap();
        let value: u8 = ranged.into_inner();
        assert_eq!(value, 15);
    }
}

#[cfg(test)]
mod u16_nonzero_tests {
    use super::*;

    #[test]
    fn u16_nonzero_new_valid() {
        let result = U16NonZero::new(1000);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 1000);
    }

    #[test]
    fn u16_nonzero_new_zero_invalid() {
        let result = U16NonZero::new(0);
        assert!(result.is_err());
    }

    #[test]
    fn u16_nonzero_into_inner() {
        let non_zero = U16NonZero::new(65535).unwrap();
        let value: u16 = non_zero.into_inner();
        assert_eq!(value, 65535);
    }
}

#[cfg(test)]
mod u16_range_tests {
    use super::*;

    #[test]
    fn u16_range_new_valid_within_range() {
        let result = U16Range::<1000, 2000>::new(1500);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 1500);
    }

    #[test]
    fn u16_range_new_valid_at_min() {
        let result = U16Range::<1000, 2000>::new(1000);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 1000);
    }

    #[test]
    fn u16_range_new_valid_at_max() {
        let result = U16Range::<1000, 2000>::new(2000);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 2000);
    }

    #[test]
    fn u16_range_new_below_min_invalid() {
        let result = U16Range::<1000, 2000>::new(999);
        assert!(result.is_err());
    }

    #[test]
    fn u16_range_new_above_max_invalid() {
        let result = U16Range::<1000, 2000>::new(2001);
        assert!(result.is_err());
    }

    #[test]
    fn u16_range_into_inner() {
        let ranged = U16Range::<1000, 2000>::new(1500).unwrap();
        let value: u16 = ranged.into_inner();
        assert_eq!(value, 1500);
    }
}

// ============================================================================
// Macro to generate signed integer contract types (Positive, NonNegative, Range)
// ============================================================================

macro_rules! impl_signed_contracts {
    ($base:ty, $positive:ident, $nonnegative:ident, $range:ident, $range_style:ident, $test_pos_value:expr, $test_nonneg_value:expr, $test_range_min:expr, $test_range_max:expr, $test_range_value:expr) => {
        // Positive variant (value > 0)
        #[doc = concat!("Contract type for positive ", stringify!($base), " values (> 0).")]
        ///
        /// Validates on construction, then can unwrap to stdlib type via `into_inner()`.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $positive($base);

        impl $positive {
            /// Constructs a positive value.
            ///
            /// # Errors
            ///
            /// Returns `ValidationError::NotPositive` if value <= 0.
            pub fn new(value: $base) -> Result<Self, ValidationError> {
                if value > 0 {
                    Ok(Self(value))
                } else {
                    Err(ValidationError::NotPositive(value as i128))
                }
            }

            /// Gets the wrapped value.
            pub fn get(&self) -> $base {
                self.0
            }

            /// Unwraps to stdlib type (trenchcoat off).
            pub fn into_inner(self) -> $base {
                self.0
            }
        }

        paste::paste! {
            crate::default_style!($positive => [<$positive Style>]);

            impl Prompt for $positive {
                fn prompt() -> Option<&'static str> {
                    Some("Please enter a positive number (> 0):")
                }
            }

            impl Elicitation for $positive {
                type Style = [<$positive Style>];

                #[tracing::instrument(skip(client), fields(type_name = stringify!($positive)))]
                async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
                    tracing::debug!(concat!("Eliciting ", stringify!($positive), " (positive ", stringify!($base), " value)"));

                    loop {
                        let value = <$base>::elicit(client).await?;
                        
                        match Self::new(value) {
                            Ok(positive) => {
                                tracing::debug!(value, concat!("Valid ", stringify!($positive), " constructed"));
                                return Ok(positive);
                            }
                            Err(e) => {
                                tracing::warn!(value, error = %e, concat!("Invalid ", stringify!($positive), ", re-prompting"));
                            }
                        }
                    }
                }
            }
        }

        // NonNegative variant (value >= 0)
        #[doc = concat!("Contract type for non-negative ", stringify!($base), " values (>= 0).")]
        ///
        /// Validates on construction, then can unwrap to stdlib type via `into_inner()`.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $nonnegative($base);

        impl $nonnegative {
            /// Constructs a non-negative value.
            ///
            /// # Errors
            ///
            /// Returns `ValidationError::Negative` if value < 0.
            pub fn new(value: $base) -> Result<Self, ValidationError> {
                if value >= 0 {
                    Ok(Self(value))
                } else {
                    Err(ValidationError::Negative(value as i128))
                }
            }

            /// Gets the wrapped value.
            pub fn get(&self) -> $base {
                self.0
            }

            /// Unwraps to stdlib type (trenchcoat off).
            pub fn into_inner(self) -> $base {
                self.0
            }
        }

        paste::paste! {
            crate::default_style!($nonnegative => [<$nonnegative Style>]);

            impl Prompt for $nonnegative {
                fn prompt() -> Option<&'static str> {
                    Some("Please enter a non-negative number (>= 0):")
                }
            }

            impl Elicitation for $nonnegative {
                type Style = [<$nonnegative Style>];

                #[tracing::instrument(skip(client), fields(type_name = stringify!($nonnegative)))]
                async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
                    tracing::debug!(concat!("Eliciting ", stringify!($nonnegative), " (non-negative ", stringify!($base), " value)"));

                    loop {
                        let value = <$base>::elicit(client).await?;
                        
                        match Self::new(value) {
                            Ok(non_negative) => {
                                tracing::debug!(value, concat!("Valid ", stringify!($nonnegative), " constructed"));
                                return Ok(non_negative);
                            }
                            Err(e) => {
                                tracing::warn!(value, error = %e, concat!("Invalid ", stringify!($nonnegative), ", re-prompting"));
                            }
                        }
                    }
                }
            }
        }

        // Range variant (MIN <= value <= MAX)
        #[doc = concat!("Contract type for ", stringify!($base), " values within a specified range [MIN, MAX].")]
        ///
        /// Validates on construction, then can unwrap to stdlib type via `into_inner()`.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $range<const MIN: $base, const MAX: $base>($base);

        impl<const MIN: $base, const MAX: $base> $range<MIN, MAX> {
            /// Constructs a value within the specified range.
            ///
            /// # Errors
            ///
            /// Returns `ValidationError::OutOfRange` if value not in [MIN, MAX].
            pub fn new(value: $base) -> Result<Self, ValidationError> {
                if value >= MIN && value <= MAX {
                    Ok(Self(value))
                } else {
                    Err(ValidationError::OutOfRange {
                        value: value as i128,
                        min: MIN as i128,
                        max: MAX as i128,
                    })
                }
            }

            /// Gets the wrapped value.
            pub fn get(&self) -> $base {
                self.0
            }

            /// Unwraps to stdlib type (trenchcoat off).
            pub fn into_inner(self) -> $base {
                self.0
            }
        }

        #[doc = concat!("Default-only style enum for ", stringify!($range), ".")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub enum $range_style {
            /// Default presentation style.
            #[default]
            Default,
        }

        impl crate::Prompt for $range_style {
            fn prompt() -> Option<&'static str> {
                None
            }
        }

        impl crate::Elicitation for $range_style {
            type Style = $range_style;

            async fn elicit(_client: &crate::ElicitClient<'_>) -> crate::ElicitResult<Self> {
                Ok(Self::Default)
            }
        }

        impl<const MIN: $base, const MAX: $base> Prompt for $range<MIN, MAX> {
            fn prompt() -> Option<&'static str> {
                Some("Please enter a number within the specified range:")
            }
        }

        impl<const MIN: $base, const MAX: $base> Elicitation for $range<MIN, MAX> {
            type Style = $range_style;

            #[tracing::instrument(skip(client), fields(type_name = stringify!($range), min = MIN, max = MAX))]
            async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
                tracing::debug!(concat!("Eliciting ", stringify!($range), "<{}, {}> (", stringify!($base), " in range)"), MIN, MAX);

                loop {
                    let value = <$base>::elicit(client).await?;
                    
                    match Self::new(value) {
                        Ok(ranged) => {
                            tracing::debug!(value, min = MIN, max = MAX, concat!("Valid ", stringify!($range), " constructed"));
                            return Ok(ranged);
                        }
                        Err(e) => {
                            tracing::warn!(value, error = %e, min = MIN, max = MAX, concat!("Invalid ", stringify!($range), ", re-prompting"));
                        }
                    }
                }
            }
        }

        // Tests
        paste::paste! {
            #[cfg(test)]
            mod [<$positive:snake _tests>] {
                use super::*;

                #[test]
                fn [<$positive:snake _new_valid>]() {
                    let result = $positive::new($test_pos_value);
                    assert!(result.is_ok());
                    assert_eq!(result.unwrap().get(), $test_pos_value);
                }

                #[test]
                fn [<$positive:snake _new_zero_invalid>]() {
                    let result = $positive::new(0);
                    assert!(result.is_err());
                }

                #[test]
                fn [<$positive:snake _new_negative_invalid>]() {
                    let result = $positive::new(-1);
                    assert!(result.is_err());
                }

                #[test]
                fn [<$positive:snake _into_inner>]() {
                    let positive = $positive::new($test_pos_value).unwrap();
                    let value: $base = positive.into_inner();
                    assert_eq!(value, $test_pos_value);
                }
            }

            #[cfg(test)]
            mod [<$nonnegative:snake _tests>] {
                use super::*;

                #[test]
                fn [<$nonnegative:snake _new_valid_positive>]() {
                    let result = $nonnegative::new($test_nonneg_value);
                    assert!(result.is_ok());
                    assert_eq!(result.unwrap().get(), $test_nonneg_value);
                }

                #[test]
                fn [<$nonnegative:snake _new_valid_zero>]() {
                    let result = $nonnegative::new(0);
                    assert!(result.is_ok());
                    assert_eq!(result.unwrap().get(), 0);
                }

                #[test]
                fn [<$nonnegative:snake _new_negative_invalid>]() {
                    let result = $nonnegative::new(-1);
                    assert!(result.is_err());
                }

                #[test]
                fn [<$nonnegative:snake _into_inner>]() {
                    let non_neg = $nonnegative::new($test_nonneg_value).unwrap();
                    let value: $base = non_neg.into_inner();
                    assert_eq!(value, $test_nonneg_value);
                }
            }

            #[cfg(test)]
            mod [<$range:snake _tests>] {
                use super::*;

                #[test]
                fn [<$range:snake _new_valid_within_range>]() {
                    let result = $range::<$test_range_min, $test_range_max>::new($test_range_value);
                    assert!(result.is_ok());
                    assert_eq!(result.unwrap().get(), $test_range_value);
                }

                #[test]
                fn [<$range:snake _new_valid_at_min>]() {
                    let result = $range::<$test_range_min, $test_range_max>::new($test_range_min);
                    assert!(result.is_ok());
                    assert_eq!(result.unwrap().get(), $test_range_min);
                }

                #[test]
                fn [<$range:snake _new_valid_at_max>]() {
                    let result = $range::<$test_range_min, $test_range_max>::new($test_range_max);
                    assert!(result.is_ok());
                    assert_eq!(result.unwrap().get(), $test_range_max);
                }

                #[test]
                fn [<$range:snake _new_below_min_invalid>]() {
                    let result = $range::<$test_range_min, $test_range_max>::new($test_range_min - 1);
                    assert!(result.is_err());
                }

                #[test]
                fn [<$range:snake _new_above_max_invalid>]() {
                    let result = $range::<$test_range_min, $test_range_max>::new($test_range_max + 1);
                    assert!(result.is_err());
                }

                #[test]
                fn [<$range:snake _into_inner>]() {
                    let ranged = $range::<$test_range_min, $test_range_max>::new($test_range_value).unwrap();
                    let value: $base = ranged.into_inner();
                    assert_eq!(value, $test_range_value);
                }
            }
        }
    };
}

// ============================================================================
// Macro to generate unsigned integer contract types (NonZero, Range)
// ============================================================================

macro_rules! impl_unsigned_contracts {
    ($base:ty, $nonzero:ident, $range:ident, $range_style:ident, $test_nonzero_value:expr, $test_range_min:expr, $test_range_max:expr, $test_range_value:expr) => {
        // NonZero variant (value != 0)
        #[doc = concat!("Contract type for non-zero ", stringify!($base), " values (!= 0).")]
        ///
        /// Validates on construction, then can unwrap to stdlib type via `into_inner()`.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $nonzero($base);

        impl $nonzero {
            /// Constructs a non-zero value.
            ///
            /// # Errors
            ///
            /// Returns `ValidationError::Zero` if value == 0.
            pub fn new(value: $base) -> Result<Self, ValidationError> {
                if value != 0 {
                    Ok(Self(value))
                } else {
                    Err(ValidationError::Zero)
                }
            }

            /// Gets the wrapped value.
            pub fn get(&self) -> $base {
                self.0
            }

            /// Unwraps to stdlib type (trenchcoat off).
            pub fn into_inner(self) -> $base {
                self.0
            }
        }

        paste::paste! {
            crate::default_style!($nonzero => [<$nonzero Style>]);

            impl Prompt for $nonzero {
                fn prompt() -> Option<&'static str> {
                    Some("Please enter a non-zero number (!= 0):")
                }
            }

            impl Elicitation for $nonzero {
                type Style = [<$nonzero Style>];

                #[tracing::instrument(skip(client), fields(type_name = stringify!($nonzero)))]
                async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
                    tracing::debug!(concat!("Eliciting ", stringify!($nonzero), " (non-zero ", stringify!($base), " value)"));

                    loop {
                        let value = <$base>::elicit(client).await?;
                        
                        match Self::new(value) {
                            Ok(non_zero) => {
                                tracing::debug!(value, concat!("Valid ", stringify!($nonzero), " constructed"));
                                return Ok(non_zero);
                            }
                            Err(e) => {
                                tracing::warn!(value, error = %e, concat!("Invalid ", stringify!($nonzero), ", re-prompting"));
                            }
                        }
                    }
                }
            }
        }

        // Range variant (MIN <= value <= MAX)
        #[doc = concat!("Contract type for ", stringify!($base), " values within a specified range [MIN, MAX].")]
        ///
        /// Validates on construction, then can unwrap to stdlib type via `into_inner()`.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $range<const MIN: $base, const MAX: $base>($base);

        impl<const MIN: $base, const MAX: $base> $range<MIN, MAX> {
            /// Constructs a value within the specified range.
            ///
            /// # Errors
            ///
            /// Returns `ValidationError::OutOfRange` if value not in [MIN, MAX].
            pub fn new(value: $base) -> Result<Self, ValidationError> {
                if value >= MIN && value <= MAX {
                    Ok(Self(value))
                } else {
                    Err(ValidationError::OutOfRange {
                        value: value as i128,
                        min: MIN as i128,
                        max: MAX as i128,
                    })
                }
            }

            /// Gets the wrapped value.
            pub fn get(&self) -> $base {
                self.0
            }

            /// Unwraps to stdlib type (trenchcoat off).
            pub fn into_inner(self) -> $base {
                self.0
            }
        }

        #[doc = concat!("Default-only style enum for ", stringify!($range), ".")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub enum $range_style {
            /// Default presentation style.
            #[default]
            Default,
        }

        impl crate::Prompt for $range_style {
            fn prompt() -> Option<&'static str> {
                None
            }
        }

        impl crate::Elicitation for $range_style {
            type Style = $range_style;

            async fn elicit(_client: &crate::ElicitClient<'_>) -> crate::ElicitResult<Self> {
                Ok(Self::Default)
            }
        }

        impl<const MIN: $base, const MAX: $base> Prompt for $range<MIN, MAX> {
            fn prompt() -> Option<&'static str> {
                Some("Please enter a number within the specified range:")
            }
        }

        impl<const MIN: $base, const MAX: $base> Elicitation for $range<MIN, MAX> {
            type Style = $range_style;

            #[tracing::instrument(skip(client), fields(type_name = stringify!($range), min = MIN, max = MAX))]
            async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
                tracing::debug!(concat!("Eliciting ", stringify!($range), "<{}, {}> (", stringify!($base), " in range)"), MIN, MAX);

                loop {
                    let value = <$base>::elicit(client).await?;
                    
                    match Self::new(value) {
                        Ok(ranged) => {
                            tracing::debug!(value, min = MIN, max = MAX, concat!("Valid ", stringify!($range), " constructed"));
                            return Ok(ranged);
                        }
                        Err(e) => {
                            tracing::warn!(value, error = %e, min = MIN, max = MAX, concat!("Invalid ", stringify!($range), ", re-prompting"));
                        }
                    }
                }
            }
        }

        // Tests
        paste::paste! {
            #[cfg(test)]
            mod [<$nonzero:snake _tests>] {
                use super::*;

                #[test]
                fn [<$nonzero:snake _new_valid>]() {
                    let result = $nonzero::new($test_nonzero_value);
                    assert!(result.is_ok());
                    assert_eq!(result.unwrap().get(), $test_nonzero_value);
                }

                #[test]
                fn [<$nonzero:snake _new_zero_invalid>]() {
                    let result = $nonzero::new(0);
                    assert!(result.is_err());
                }

                #[test]
                fn [<$nonzero:snake _into_inner>]() {
                    let non_zero = $nonzero::new($test_nonzero_value).unwrap();
                    let value: $base = non_zero.into_inner();
                    assert_eq!(value, $test_nonzero_value);
                }
            }

            #[cfg(test)]
            mod [<$range:snake _tests>] {
                use super::*;

                #[test]
                fn [<$range:snake _new_valid_within_range>]() {
                    let result = $range::<$test_range_min, $test_range_max>::new($test_range_value);
                    assert!(result.is_ok());
                    assert_eq!(result.unwrap().get(), $test_range_value);
                }

                #[test]
                fn [<$range:snake _new_valid_at_min>]() {
                    let result = $range::<$test_range_min, $test_range_max>::new($test_range_min);
                    assert!(result.is_ok());
                    assert_eq!(result.unwrap().get(), $test_range_min);
                }

                #[test]
                fn [<$range:snake _new_valid_at_max>]() {
                    let result = $range::<$test_range_min, $test_range_max>::new($test_range_max);
                    assert!(result.is_ok());
                    assert_eq!(result.unwrap().get(), $test_range_max);
                }

                #[test]
                fn [<$range:snake _new_below_min_invalid>]() {
                    let result = $range::<$test_range_min, $test_range_max>::new($test_range_min - 1);
                    assert!(result.is_err());
                }

                #[test]
                fn [<$range:snake _new_above_max_invalid>]() {
                    let result = $range::<$test_range_min, $test_range_max>::new($test_range_max + 1);
                    assert!(result.is_err());
                }

                #[test]
                fn [<$range:snake _into_inner>]() {
                    let ranged = $range::<$test_range_min, $test_range_max>::new($test_range_value).unwrap();
                    let value: $base = ranged.into_inner();
                    assert_eq!(value, $test_range_value);
                }
            }
        }
    };
}

// ============================================================================
// Generate remaining integer contract types using macros
// ============================================================================

// i32 family
impl_signed_contracts!(i32, I32Positive, I32NonNegative, I32Range, I32RangeStyle, 42, 100, 10, 100, 50);

// u32 family
impl_unsigned_contracts!(u32, U32NonZero, U32Range, U32RangeStyle, 42, 10, 100, 50);

// i64 family
impl_signed_contracts!(i64, I64Positive, I64NonNegative, I64Range, I64RangeStyle, 42, 100, 10, 100, 50);

// u64 family
impl_unsigned_contracts!(u64, U64NonZero, U64Range, U64RangeStyle, 42, 10, 100, 50);

// i128 family
impl_signed_contracts!(i128, I128Positive, I128NonNegative, I128Range, I128RangeStyle, 42, 100, 10, 100, 50);

// u128 family
impl_unsigned_contracts!(u128, U128NonZero, U128Range, U128RangeStyle, 42, 10, 100, 50);

// isize family
impl_signed_contracts!(isize, IsizePositive, IsizeNonNegative, IsizeRange, IsizeRangeStyle, 42, 100, 10, 100, 50);

// usize family
impl_unsigned_contracts!(usize, UsizeNonZero, UsizeRange, UsizeRangeStyle, 42, 10, 100, 50);

// ============================================================================
// Float Contract Types (f32, f64)
// ============================================================================

// F32Positive (f32 > 0.0 and finite)
/// Contract type for positive f32 values (> 0.0).
///
/// Validates on construction, then can unwrap to stdlib f32 via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct F32Positive(f32);

impl F32Positive {
    /// Constructs a positive f32 value.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NotFinite` if value is NaN or infinite.
    /// Returns `ValidationError::FloatNotPositive` if value <= 0.0.
    pub fn new(value: f32) -> Result<Self, ValidationError> {
        if !value.is_finite() {
            Err(ValidationError::NotFinite(format!("{}", value)))
        } else if value > 0.0 {
            Ok(Self(value))
        } else {
            Err(ValidationError::FloatNotPositive(value as f64))
        }
    }

    /// Gets the wrapped value.
    pub fn get(&self) -> f32 {
        self.0
    }

    /// Unwraps to stdlib f32 (trenchcoat off).
    pub fn into_inner(self) -> f32 {
        self.0
    }
}

crate::default_style!(F32Positive => F32PositiveStyle);

impl Prompt for F32Positive {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a positive number (> 0.0):")
    }
}

impl Elicitation for F32Positive {
    type Style = F32PositiveStyle;

    #[tracing::instrument(skip(client), fields(type_name = "F32Positive"))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting F32Positive (positive f32 value)");

        loop {
            let value = f32::elicit(client).await?;
            
            match Self::new(value) {
                Ok(positive) => {
                    tracing::debug!(value, "Valid F32Positive constructed");
                    return Ok(positive);
                }
                Err(e) => {
                    tracing::warn!(value, error = %e, "Invalid F32Positive, re-prompting");
                }
            }
        }
    }
}

// F32NonNegative (f32 >= 0.0 and finite)
/// Contract type for non-negative f32 values (>= 0.0).
///
/// Validates on construction, then can unwrap to stdlib f32 via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct F32NonNegative(f32);

impl F32NonNegative {
    /// Constructs a non-negative f32 value.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NotFinite` if value is NaN or infinite.
    /// Returns `ValidationError::FloatNegative` if value < 0.0.
    pub fn new(value: f32) -> Result<Self, ValidationError> {
        if !value.is_finite() {
            Err(ValidationError::NotFinite(format!("{}", value)))
        } else if value >= 0.0 {
            Ok(Self(value))
        } else {
            Err(ValidationError::FloatNegative(value as f64))
        }
    }

    /// Gets the wrapped value.
    pub fn get(&self) -> f32 {
        self.0
    }

    /// Unwraps to stdlib f32 (trenchcoat off).
    pub fn into_inner(self) -> f32 {
        self.0
    }
}

crate::default_style!(F32NonNegative => F32NonNegativeStyle);

impl Prompt for F32NonNegative {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a non-negative number (>= 0.0):")
    }
}

impl Elicitation for F32NonNegative {
    type Style = F32NonNegativeStyle;

    #[tracing::instrument(skip(client), fields(type_name = "F32NonNegative"))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting F32NonNegative (non-negative f32 value)");

        loop {
            let value = f32::elicit(client).await?;
            
            match Self::new(value) {
                Ok(non_negative) => {
                    tracing::debug!(value, "Valid F32NonNegative constructed");
                    return Ok(non_negative);
                }
                Err(e) => {
                    tracing::warn!(value, error = %e, "Invalid F32NonNegative, re-prompting");
                }
            }
        }
    }
}

// F32Finite (finite f32, not NaN or infinite)
/// Contract type for finite f32 values (not NaN or infinite).
///
/// Validates on construction, then can unwrap to stdlib f32 via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct F32Finite(f32);

impl F32Finite {
    /// Constructs a finite f32 value.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NotFinite` if value is NaN or infinite.
    pub fn new(value: f32) -> Result<Self, ValidationError> {
        if value.is_finite() {
            Ok(Self(value))
        } else {
            Err(ValidationError::NotFinite(format!("{}", value)))
        }
    }

    /// Gets the wrapped value.
    pub fn get(&self) -> f32 {
        self.0
    }

    /// Unwraps to stdlib f32 (trenchcoat off).
    pub fn into_inner(self) -> f32 {
        self.0
    }
}

crate::default_style!(F32Finite => F32FiniteStyle);

impl Prompt for F32Finite {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a finite number (not NaN or infinite):")
    }
}

impl Elicitation for F32Finite {
    type Style = F32FiniteStyle;

    #[tracing::instrument(skip(client), fields(type_name = "F32Finite"))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting F32Finite (finite f32 value)");

        loop {
            let value = f32::elicit(client).await?;
            
            match Self::new(value) {
                Ok(finite) => {
                    tracing::debug!(value, "Valid F32Finite constructed");
                    return Ok(finite);
                }
                Err(e) => {
                    tracing::warn!(value, error = %e, "Invalid F32Finite, re-prompting");
                }
            }
        }
    }
}

// F64Positive (f64 > 0.0 and finite)
/// Contract type for positive f64 values (> 0.0).
///
/// Validates on construction, then can unwrap to stdlib f64 via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct F64Positive(f64);

impl F64Positive {
    /// Constructs a positive f64 value.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NotFinite` if value is NaN or infinite.
    /// Returns `ValidationError::FloatNotPositive` if value <= 0.0.
    pub fn new(value: f64) -> Result<Self, ValidationError> {
        if !value.is_finite() {
            Err(ValidationError::NotFinite(format!("{}", value)))
        } else if value > 0.0 {
            Ok(Self(value))
        } else {
            Err(ValidationError::FloatNotPositive(value))
        }
    }

    /// Gets the wrapped value.
    pub fn get(&self) -> f64 {
        self.0
    }

    /// Unwraps to stdlib f64 (trenchcoat off).
    pub fn into_inner(self) -> f64 {
        self.0
    }
}

crate::default_style!(F64Positive => F64PositiveStyle);

impl Prompt for F64Positive {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a positive number (> 0.0):")
    }
}

impl Elicitation for F64Positive {
    type Style = F64PositiveStyle;

    #[tracing::instrument(skip(client), fields(type_name = "F64Positive"))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting F64Positive (positive f64 value)");

        loop {
            let value = f64::elicit(client).await?;
            
            match Self::new(value) {
                Ok(positive) => {
                    tracing::debug!(value, "Valid F64Positive constructed");
                    return Ok(positive);
                }
                Err(e) => {
                    tracing::warn!(value, error = %e, "Invalid F64Positive, re-prompting");
                }
            }
        }
    }
}

// F64NonNegative (f64 >= 0.0 and finite)
/// Contract type for non-negative f64 values (>= 0.0).
///
/// Validates on construction, then can unwrap to stdlib f64 via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct F64NonNegative(f64);

impl F64NonNegative {
    /// Constructs a non-negative f64 value.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NotFinite` if value is NaN or infinite.
    /// Returns `ValidationError::FloatNegative` if value < 0.0.
    pub fn new(value: f64) -> Result<Self, ValidationError> {
        if !value.is_finite() {
            Err(ValidationError::NotFinite(format!("{}", value)))
        } else if value >= 0.0 {
            Ok(Self(value))
        } else {
            Err(ValidationError::FloatNegative(value))
        }
    }

    /// Gets the wrapped value.
    pub fn get(&self) -> f64 {
        self.0
    }

    /// Unwraps to stdlib f64 (trenchcoat off).
    pub fn into_inner(self) -> f64 {
        self.0
    }
}

crate::default_style!(F64NonNegative => F64NonNegativeStyle);

impl Prompt for F64NonNegative {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a non-negative number (>= 0.0):")
    }
}

impl Elicitation for F64NonNegative {
    type Style = F64NonNegativeStyle;

    #[tracing::instrument(skip(client), fields(type_name = "F64NonNegative"))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting F64NonNegative (non-negative f64 value)");

        loop {
            let value = f64::elicit(client).await?;
            
            match Self::new(value) {
                Ok(non_negative) => {
                    tracing::debug!(value, "Valid F64NonNegative constructed");
                    return Ok(non_negative);
                }
                Err(e) => {
                    tracing::warn!(value, error = %e, "Invalid F64NonNegative, re-prompting");
                }
            }
        }
    }
}

// F64Finite (finite f64, not NaN or infinite)
/// Contract type for finite f64 values (not NaN or infinite).
///
/// Validates on construction, then can unwrap to stdlib f64 via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct F64Finite(f64);

impl F64Finite {
    /// Constructs a finite f64 value.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NotFinite` if value is NaN or infinite.
    pub fn new(value: f64) -> Result<Self, ValidationError> {
        if value.is_finite() {
            Ok(Self(value))
        } else {
            Err(ValidationError::NotFinite(format!("{}", value)))
        }
    }

    /// Gets the wrapped value.
    pub fn get(&self) -> f64 {
        self.0
    }

    /// Unwraps to stdlib f64 (trenchcoat off).
    pub fn into_inner(self) -> f64 {
        self.0
    }
}

crate::default_style!(F64Finite => F64FiniteStyle);

impl Prompt for F64Finite {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a finite number (not NaN or infinite):")
    }
}

impl Elicitation for F64Finite {
    type Style = F64FiniteStyle;

    #[tracing::instrument(skip(client), fields(type_name = "F64Finite"))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting F64Finite (finite f64 value)");

        loop {
            let value = f64::elicit(client).await?;
            
            match Self::new(value) {
                Ok(finite) => {
                    tracing::debug!(value, "Valid F64Finite constructed");
                    return Ok(finite);
                }
                Err(e) => {
                    tracing::warn!(value, error = %e, "Invalid F64Finite, re-prompting");
                }
            }
        }
    }
}

// Tests
#[cfg(test)]
mod f32_positive_tests {
    use super::*;

    #[test]
    fn f32_positive_new_valid() {
        let result = F32Positive::new(1.5);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 1.5);
    }

    #[test]
    fn f32_positive_new_zero_invalid() {
        let result = F32Positive::new(0.0);
        assert!(result.is_err());
    }

    #[test]
    fn f32_positive_new_negative_invalid() {
        let result = F32Positive::new(-1.5);
        assert!(result.is_err());
    }

    #[test]
    fn f32_positive_new_nan_invalid() {
        let result = F32Positive::new(f32::NAN);
        assert!(result.is_err());
    }

    #[test]
    fn f32_positive_new_infinity_invalid() {
        let result = F32Positive::new(f32::INFINITY);
        assert!(result.is_err());
    }

    #[test]
    fn f32_positive_into_inner() {
        let positive = F32Positive::new(42.5).unwrap();
        let value: f32 = positive.into_inner();
        assert_eq!(value, 42.5);
    }
}

#[cfg(test)]
mod f32_nonnegative_tests {
    use super::*;

    #[test]
    fn f32_nonnegative_new_valid_positive() {
        let result = F32NonNegative::new(1.5);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 1.5);
    }

    #[test]
    fn f32_nonnegative_new_valid_zero() {
        let result = F32NonNegative::new(0.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 0.0);
    }

    #[test]
    fn f32_nonnegative_new_negative_invalid() {
        let result = F32NonNegative::new(-1.5);
        assert!(result.is_err());
    }

    #[test]
    fn f32_nonnegative_new_nan_invalid() {
        let result = F32NonNegative::new(f32::NAN);
        assert!(result.is_err());
    }

    #[test]
    fn f32_nonnegative_into_inner() {
        let non_neg = F32NonNegative::new(10.5).unwrap();
        let value: f32 = non_neg.into_inner();
        assert_eq!(value, 10.5);
    }
}

#[cfg(test)]
mod f32_finite_tests {
    use super::*;

    #[test]
    fn f32_finite_new_valid_positive() {
        let result = F32Finite::new(1.5);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 1.5);
    }

    #[test]
    fn f32_finite_new_valid_negative() {
        let result = F32Finite::new(-1.5);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), -1.5);
    }

    #[test]
    fn f32_finite_new_valid_zero() {
        let result = F32Finite::new(0.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 0.0);
    }

    #[test]
    fn f32_finite_new_nan_invalid() {
        let result = F32Finite::new(f32::NAN);
        assert!(result.is_err());
    }

    #[test]
    fn f32_finite_new_infinity_invalid() {
        let result = F32Finite::new(f32::INFINITY);
        assert!(result.is_err());
    }

    #[test]
    fn f32_finite_new_neg_infinity_invalid() {
        let result = F32Finite::new(f32::NEG_INFINITY);
        assert!(result.is_err());
    }

    #[test]
    fn f32_finite_into_inner() {
        let finite = F32Finite::new(42.5).unwrap();
        let value: f32 = finite.into_inner();
        assert_eq!(value, 42.5);
    }
}

#[cfg(test)]
mod f64_positive_tests {
    use super::*;

    #[test]
    fn f64_positive_new_valid() {
        let result = F64Positive::new(1.5);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 1.5);
    }

    #[test]
    fn f64_positive_new_zero_invalid() {
        let result = F64Positive::new(0.0);
        assert!(result.is_err());
    }

    #[test]
    fn f64_positive_new_negative_invalid() {
        let result = F64Positive::new(-1.5);
        assert!(result.is_err());
    }

    #[test]
    fn f64_positive_new_nan_invalid() {
        let result = F64Positive::new(f64::NAN);
        assert!(result.is_err());
    }

    #[test]
    fn f64_positive_new_infinity_invalid() {
        let result = F64Positive::new(f64::INFINITY);
        assert!(result.is_err());
    }

    #[test]
    fn f64_positive_into_inner() {
        let positive = F64Positive::new(42.5).unwrap();
        let value: f64 = positive.into_inner();
        assert_eq!(value, 42.5);
    }
}

#[cfg(test)]
mod f64_nonnegative_tests {
    use super::*;

    #[test]
    fn f64_nonnegative_new_valid_positive() {
        let result = F64NonNegative::new(1.5);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 1.5);
    }

    #[test]
    fn f64_nonnegative_new_valid_zero() {
        let result = F64NonNegative::new(0.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 0.0);
    }

    #[test]
    fn f64_nonnegative_new_negative_invalid() {
        let result = F64NonNegative::new(-1.5);
        assert!(result.is_err());
    }

    #[test]
    fn f64_nonnegative_new_nan_invalid() {
        let result = F64NonNegative::new(f64::NAN);
        assert!(result.is_err());
    }

    #[test]
    fn f64_nonnegative_into_inner() {
        let non_neg = F64NonNegative::new(10.5).unwrap();
        let value: f64 = non_neg.into_inner();
        assert_eq!(value, 10.5);
    }
}

#[cfg(test)]
mod f64_finite_tests {
    use super::*;

    #[test]
    fn f64_finite_new_valid_positive() {
        let result = F64Finite::new(1.5);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 1.5);
    }

    #[test]
    fn f64_finite_new_valid_negative() {
        let result = F64Finite::new(-1.5);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), -1.5);
    }

    #[test]
    fn f64_finite_new_valid_zero() {
        let result = F64Finite::new(0.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 0.0);
    }

    #[test]
    fn f64_finite_new_nan_invalid() {
        let result = F64Finite::new(f64::NAN);
        assert!(result.is_err());
    }

    #[test]
    fn f64_finite_new_infinity_invalid() {
        let result = F64Finite::new(f64::INFINITY);
        assert!(result.is_err());
    }

    #[test]
    fn f64_finite_new_neg_infinity_invalid() {
        let result = F64Finite::new(f64::NEG_INFINITY);
        assert!(result.is_err());
    }

    #[test]
    fn f64_finite_into_inner() {
        let finite = F64Finite::new(42.5).unwrap();
        let value: f64 = finite.into_inner();
        assert_eq!(value, 42.5);
    }
}

// ============================================================================
// String Contract Types
// ============================================================================

/// Contract type for non-empty String values.
///
/// Validates on construction, then can unwrap to stdlib String via `into_inner()`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringNonEmpty(String);

impl StringNonEmpty {
    /// Constructs a non-empty String.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::EmptyString` if string is empty.
    pub fn new(value: String) -> Result<Self, ValidationError> {
        if !value.is_empty() {
            Ok(Self(value))
        } else {
            Err(ValidationError::EmptyString)
        }
    }

    /// Gets the wrapped value.
    pub fn get(&self) -> &str {
        &self.0
    }

    /// Unwraps to stdlib String (trenchcoat off).
    pub fn into_inner(self) -> String {
        self.0
    }
}

crate::default_style!(StringNonEmpty => StringNonEmptyStyle);

impl Prompt for StringNonEmpty {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a non-empty string:")
    }
}

impl Elicitation for StringNonEmpty {
    type Style = StringNonEmptyStyle;

    #[tracing::instrument(skip(client), fields(type_name = "StringNonEmpty"))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting StringNonEmpty (non-empty string)");

        loop {
            let value = String::elicit(client).await?;
            
            match Self::new(value) {
                Ok(non_empty) => {
                    tracing::debug!(value = %non_empty.get(), "Valid StringNonEmpty constructed");
                    return Ok(non_empty);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Invalid StringNonEmpty, re-prompting");
                }
            }
        }
    }
}

#[cfg(test)]
mod string_nonempty_tests {
    use super::*;

    #[test]
    fn string_nonempty_new_valid() {
        let result = StringNonEmpty::new("hello".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), "hello");
    }

    #[test]
    fn string_nonempty_new_empty_invalid() {
        let result = StringNonEmpty::new(String::new());
        assert!(result.is_err());
    }

    #[test]
    fn string_nonempty_into_inner() {
        let non_empty = StringNonEmpty::new("world".to_string()).unwrap();
        let value: String = non_empty.into_inner();
        assert_eq!(value, "world");
    }
}

// ============================================================================
// Bool Contract Types
// ============================================================================

/// Contract type for true bool values.
///
/// Validates on construction, then can unwrap to stdlib bool via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoolTrue(bool);

impl BoolTrue {
    /// Constructs a true bool value.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NotTrue` if value is false.
    pub fn new(value: bool) -> Result<Self, ValidationError> {
        if value {
            Ok(Self(value))
        } else {
            Err(ValidationError::NotTrue)
        }
    }

    /// Gets the wrapped value (always true).
    pub fn get(&self) -> bool {
        self.0
    }

    /// Unwraps to stdlib bool (trenchcoat off).
    pub fn into_inner(self) -> bool {
        self.0
    }
}

crate::default_style!(BoolTrue => BoolTrueStyle);

impl Prompt for BoolTrue {
    fn prompt() -> Option<&'static str> {
        Some("Please confirm (must be true):")
    }
}

impl Elicitation for BoolTrue {
    type Style = BoolTrueStyle;

    #[tracing::instrument(skip(client), fields(type_name = "BoolTrue"))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting BoolTrue (must be true)");

        loop {
            let value = bool::elicit(client).await?;
            
            match Self::new(value) {
                Ok(bool_true) => {
                    tracing::debug!(value, "Valid BoolTrue constructed");
                    return Ok(bool_true);
                }
                Err(e) => {
                    tracing::warn!(value, error = %e, "Invalid BoolTrue, re-prompting");
                }
            }
        }
    }
}

/// Contract type for false bool values.
///
/// Validates on construction, then can unwrap to stdlib bool via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoolFalse(bool);

impl BoolFalse {
    /// Constructs a false bool value.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NotFalse` if value is true.
    pub fn new(value: bool) -> Result<Self, ValidationError> {
        if !value {
            Ok(Self(value))
        } else {
            Err(ValidationError::NotFalse)
        }
    }

    /// Gets the wrapped value (always false).
    pub fn get(&self) -> bool {
        self.0
    }

    /// Unwraps to stdlib bool (trenchcoat off).
    pub fn into_inner(self) -> bool {
        self.0
    }
}

crate::default_style!(BoolFalse => BoolFalseStyle);

impl Prompt for BoolFalse {
    fn prompt() -> Option<&'static str> {
        Some("Please deny (must be false):")
    }
}

impl Elicitation for BoolFalse {
    type Style = BoolFalseStyle;

    #[tracing::instrument(skip(client), fields(type_name = "BoolFalse"))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting BoolFalse (must be false)");

        loop {
            let value = bool::elicit(client).await?;
            
            match Self::new(value) {
                Ok(bool_false) => {
                    tracing::debug!(value, "Valid BoolFalse constructed");
                    return Ok(bool_false);
                }
                Err(e) => {
                    tracing::warn!(value, error = %e, "Invalid BoolFalse, re-prompting");
                }
            }
        }
    }
}

#[cfg(test)]
mod bool_true_tests {
    use super::*;

    #[test]
    fn bool_true_new_valid() {
        let result = BoolTrue::new(true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), true);
    }

    #[test]
    fn bool_true_new_false_invalid() {
        let result = BoolTrue::new(false);
        assert!(result.is_err());
    }

    #[test]
    fn bool_true_into_inner() {
        let bool_true = BoolTrue::new(true).unwrap();
        let value: bool = bool_true.into_inner();
        assert_eq!(value, true);
    }
}

#[cfg(test)]
mod bool_false_tests {
    use super::*;

    #[test]
    fn bool_false_new_valid() {
        let result = BoolFalse::new(false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), false);
    }

    #[test]
    fn bool_false_new_true_invalid() {
        let result = BoolFalse::new(true);
        assert!(result.is_err());
    }

    #[test]
    fn bool_false_into_inner() {
        let bool_false = BoolFalse::new(false).unwrap();
        let value: bool = bool_false.into_inner();
        assert_eq!(value, false);
    }
}

// ============================================================================
// Char Contract Types
// ============================================================================

/// Contract type for alphabetic char values.
///
/// Validates on construction, then can unwrap to stdlib char via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CharAlphabetic(char);

impl CharAlphabetic {
    /// Constructs an alphabetic char.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NotAlphabetic` if char is not alphabetic.
    pub fn new(value: char) -> Result<Self, ValidationError> {
        if value.is_alphabetic() {
            Ok(Self(value))
        } else {
            Err(ValidationError::NotAlphabetic(value))
        }
    }

    /// Gets the wrapped value.
    pub fn get(&self) -> char {
        self.0
    }

    /// Unwraps to stdlib char (trenchcoat off).
    pub fn into_inner(self) -> char {
        self.0
    }
}

crate::default_style!(CharAlphabetic => CharAlphabeticStyle);

impl Prompt for CharAlphabetic {
    fn prompt() -> Option<&'static str> {
        Some("Please enter an alphabetic character:")
    }
}

impl Elicitation for CharAlphabetic {
    type Style = CharAlphabeticStyle;

    #[tracing::instrument(skip(client), fields(type_name = "CharAlphabetic"))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting CharAlphabetic (alphabetic char)");

        loop {
            let value = char::elicit(client).await?;
            
            match Self::new(value) {
                Ok(alphabetic) => {
                    tracing::debug!(value = %value, "Valid CharAlphabetic constructed");
                    return Ok(alphabetic);
                }
                Err(e) => {
                    tracing::warn!(value = %value, error = %e, "Invalid CharAlphabetic, re-prompting");
                }
            }
        }
    }
}

/// Contract type for numeric char values.
///
/// Validates on construction, then can unwrap to stdlib char via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CharNumeric(char);

impl CharNumeric {
    /// Constructs a numeric char.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NotNumeric` if char is not numeric.
    pub fn new(value: char) -> Result<Self, ValidationError> {
        if value.is_numeric() {
            Ok(Self(value))
        } else {
            Err(ValidationError::NotNumeric(value))
        }
    }

    /// Gets the wrapped value.
    pub fn get(&self) -> char {
        self.0
    }

    /// Unwraps to stdlib char (trenchcoat off).
    pub fn into_inner(self) -> char {
        self.0
    }
}

crate::default_style!(CharNumeric => CharNumericStyle);

impl Prompt for CharNumeric {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a numeric character:")
    }
}

impl Elicitation for CharNumeric {
    type Style = CharNumericStyle;

    #[tracing::instrument(skip(client), fields(type_name = "CharNumeric"))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting CharNumeric (numeric char)");

        loop {
            let value = char::elicit(client).await?;
            
            match Self::new(value) {
                Ok(numeric) => {
                    tracing::debug!(value = %value, "Valid CharNumeric constructed");
                    return Ok(numeric);
                }
                Err(e) => {
                    tracing::warn!(value = %value, error = %e, "Invalid CharNumeric, re-prompting");
                }
            }
        }
    }
}

/// Contract type for alphanumeric char values.
///
/// Validates on construction, then can unwrap to stdlib char via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CharAlphanumeric(char);

impl CharAlphanumeric {
    /// Constructs an alphanumeric char.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::NotAlphanumeric` if char is not alphanumeric.
    pub fn new(value: char) -> Result<Self, ValidationError> {
        if value.is_alphanumeric() {
            Ok(Self(value))
        } else {
            Err(ValidationError::NotAlphanumeric(value))
        }
    }

    /// Gets the wrapped value.
    pub fn get(&self) -> char {
        self.0
    }

    /// Unwraps to stdlib char (trenchcoat off).
    pub fn into_inner(self) -> char {
        self.0
    }
}

crate::default_style!(CharAlphanumeric => CharAlphanumericStyle);

impl Prompt for CharAlphanumeric {
    fn prompt() -> Option<&'static str> {
        Some("Please enter an alphanumeric character:")
    }
}

impl Elicitation for CharAlphanumeric {
    type Style = CharAlphanumericStyle;

    #[tracing::instrument(skip(client), fields(type_name = "CharAlphanumeric"))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting CharAlphanumeric (alphanumeric char)");

        loop {
            let value = char::elicit(client).await?;
            
            match Self::new(value) {
                Ok(alphanumeric) => {
                    tracing::debug!(value = %value, "Valid CharAlphanumeric constructed");
                    return Ok(alphanumeric);
                }
                Err(e) => {
                    tracing::warn!(value = %value, error = %e, "Invalid CharAlphanumeric, re-prompting");
                }
            }
        }
    }
}

#[cfg(test)]
mod char_alphabetic_tests {
    use super::*;

    #[test]
    fn char_alphabetic_new_valid() {
        let result = CharAlphabetic::new('a');
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 'a');
    }

    #[test]
    fn char_alphabetic_new_digit_invalid() {
        let result = CharAlphabetic::new('5');
        assert!(result.is_err());
    }

    #[test]
    fn char_alphabetic_into_inner() {
        let alphabetic = CharAlphabetic::new('z').unwrap();
        let value: char = alphabetic.into_inner();
        assert_eq!(value, 'z');
    }
}

#[cfg(test)]
mod char_numeric_tests {
    use super::*;

    #[test]
    fn char_numeric_new_valid() {
        let result = CharNumeric::new('5');
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), '5');
    }

    #[test]
    fn char_numeric_new_letter_invalid() {
        let result = CharNumeric::new('a');
        assert!(result.is_err());
    }

    #[test]
    fn char_numeric_into_inner() {
        let numeric = CharNumeric::new('9').unwrap();
        let value: char = numeric.into_inner();
        assert_eq!(value, '9');
    }
}

#[cfg(test)]
mod char_alphanumeric_tests {
    use super::*;

    #[test]
    fn char_alphanumeric_new_valid_letter() {
        let result = CharAlphanumeric::new('a');
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 'a');
    }

    #[test]
    fn char_alphanumeric_new_valid_digit() {
        let result = CharAlphanumeric::new('5');
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), '5');
    }

    #[test]
    fn char_alphanumeric_new_symbol_invalid() {
        let result = CharAlphanumeric::new('!');
        assert!(result.is_err());
    }

    #[test]
    fn char_alphanumeric_into_inner() {
        let alphanumeric = CharAlphanumeric::new('x').unwrap();
        let value: char = alphanumeric.into_inner();
        assert_eq!(value, 'x');
    }
}
