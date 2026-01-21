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
#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display)]
pub enum ValidationError {
    /// Value is not positive (must be > 0).
    #[display("Value must be positive (> 0), got {}", _0)]
    NotPositive(i128),

    /// Value is negative (must be >= 0).
    #[display("Value must be non-negative (>= 0), got {}", _0)]
    Negative(i128),

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
            Err(ValidationError::Negative(value.into()))
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
                value: value.into(),
                min: MIN.into(),
                max: MAX.into(),
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
