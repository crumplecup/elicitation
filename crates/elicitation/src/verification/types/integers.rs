//! Integer contract types.

use super::ValidationError;
use crate::{ElicitCommunicator, ElicitResult, Elicitation, Prompt};
use elicitation_derive::contract_type;
use elicitation_macros::instrumented_impl;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// Macro: Default Wrapper Generation for Integer Types
// ============================================================================

/// Generate a Default wrapper for an integer primitive type.
///
/// This macro creates an unconstrained wrapper type that:
/// - Has Serialize, Deserialize, and JsonSchema derives
/// - Is marked as elicit-safe for rmcp
/// - Uses serde deserialization instead of manual parsing
/// - Provides new/get/into_inner methods
/// - Implements Prompt and Elicitation traits
macro_rules! impl_integer_default_wrapper {
    ($primitive:ty, $wrapper:ident, $min:expr, $max:expr) => {
        #[doc = concat!("Default wrapper for ", stringify!($primitive), " (unconstrained).")]
        ///
        /// Used internally for MCP elicitation of primitive values.
        /// Provides JsonSchema for client-side validation.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
        #[schemars(description = concat!(stringify!($primitive), " value"))]
        pub struct $wrapper(
            #[schemars(range(min = $min, max = $max))]
            $primitive
        );

        rmcp::elicit_safe!($wrapper);

        impl $wrapper {
            /// Creates a new wrapper.
            pub fn new(value: $primitive) -> Self {
                Self(value)
            }

            /// Gets the inner value.
            pub fn get(&self) -> $primitive {
                self.0
            }

            /// Unwraps to the inner value.
            pub fn into_inner(self) -> $primitive {
                self.0
            }
        }

        paste::paste! {
            crate::default_style!($wrapper => [<$wrapper Style>]);

            impl Prompt for $wrapper {
                fn prompt() -> Option<&'static str> {
                    Some(concat!("Please enter a ", stringify!($primitive), ":"))
                }
            }

            impl Elicitation for $wrapper {
                type Style = [<$wrapper Style>];

                #[tracing::instrument(skip(communicator))]
                async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                    let prompt = Self::prompt().unwrap();
                    tracing::debug!(concat!("Eliciting ", stringify!($wrapper), " with prompt-based parsing"));

                    // Use send_prompt instead of call_tool for server compatibility
                    let response = communicator.send_prompt(prompt).await?;
                    
                    tracing::debug!(response = %response, "Received response, parsing as number");

                    // Parse response as integer
                    let trimmed = response.trim();
                    let value: $primitive = trimmed.parse().map_err(|e| {
                        tracing::error!(error = ?e, response = %trimmed, "Failed to parse as integer");
                        crate::ElicitError::new(crate::ElicitErrorKind::ParseError(
                            format!("Invalid {}: '{}' ({})", stringify!($primitive), trimmed, e)
                        ))
                    })?;

                    tracing::debug!(value = value, "Successfully parsed integer");
                    Ok(Self::new(value))
                }
            }
        }
    };
}

// ============================================================================
// Verification Types (Constrained)
// ============================================================================

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
#[contract_type(requires = "value > 0", ensures = "result.get() > 0")]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[schemars(description = "Positive integer value (> 0)")]
pub struct I8Positive(#[schemars(range(min = 1))] i8);

// Mark as safe for MCP elicitation
rmcp::elicit_safe!(I8Positive);

#[cfg_attr(not(kani), instrumented_impl)]
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

#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for I8Positive {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a positive number (> 0):")
    }
}

#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for I8Positive {
    type Style = I8PositiveStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "I8Positive"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting I8Positive (positive i8 value)");

        loop {
            // Elicit base i8 value
            let value = i8::elicit(communicator).await?;

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
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[schemars(description = "Non-negative integer value (>= 0)")]
pub struct I8NonNegative(#[schemars(range(min = 0))] i8);

rmcp::elicit_safe!(I8NonNegative);

#[cfg_attr(not(kani), instrumented_impl)]
#[cfg_attr(not(kani), instrumented_impl)]
#[cfg_attr(not(kani), instrumented_impl)]
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

#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for I8NonNegative {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a non-negative number (>= 0):")
    }
}

#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for I8NonNegative {
    type Style = I8NonNegativeStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "I8NonNegative"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting I8NonNegative (non-negative i8 value)");

        loop {
            let value = i8::elicit(communicator).await?;

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
// I8 NonZero Type
// ============================================================================

/// Contract type for non-zero i8 values (!= 0).
///
/// Validates on construction, unwraps to stdlib i8.
#[contract_type(requires = "value != 0", ensures = "result.get() != 0")]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[schemars(description = "Non-zero integer value (!= 0)")]
pub struct I8NonZero(i8);

rmcp::elicit_safe!(I8NonZero);

#[cfg_attr(not(kani), instrumented_impl)]
impl I8NonZero {
    /// Creates a new `I8NonZero` if value is non-zero.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Zero` if value == 0.
    pub fn new(value: i8) -> Result<Self, ValidationError> {
        if value != 0 {
            Ok(Self(value))
        } else {
            Err(ValidationError::Zero)
        }
    }

    /// Gets the inner value.
    pub fn get(&self) -> i8 {
        self.0
    }

    /// Unwraps to inner i8.
    pub fn into_inner(self) -> i8 {
        self.0
    }
}

crate::default_style!(I8NonZero => I8NonZeroStyle);

#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for I8NonZero {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a non-zero number (!= 0):")
    }
}

#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for I8NonZero {
    type Style = I8NonZeroStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "I8NonZero"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting I8NonZero");

        loop {
            let value = i8::elicit(communicator).await?;

            match Self::new(value) {
                Ok(non_zero) => {
                    tracing::debug!(value, "Valid I8NonZero constructed");
                    return Ok(non_zero);
                }
                Err(e) => {
                    tracing::warn!(value, error = %e, "Invalid I8NonZero, re-prompting");
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

#[cfg_attr(not(kani), instrumented_impl)]
impl crate::Prompt for I8RangeStyle {
    fn prompt() -> Option<&'static str> {
        None // No style selection needed
    }
}

#[cfg_attr(not(kani), instrumented_impl)]
impl crate::Elicitation for I8RangeStyle {
    type Style = I8RangeStyle;

    async fn elicit<C: crate::ElicitCommunicator>(_communicator: &C) -> crate::ElicitResult<Self> {
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

    #[tracing::instrument(skip(communicator), fields(type_name = "I8Range", min = MIN, max = MAX))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting I8Range<{}, {}> (i8 in range)", MIN, MAX);

        loop {
            let value = i8::elicit(communicator).await?;

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

#[cfg_attr(not(kani), instrumented_impl)]
#[cfg_attr(not(kani), instrumented_impl)]
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

#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for I16Positive {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a positive number (> 0):")
    }
}

#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for I16Positive {
    type Style = I16PositiveStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "I16Positive"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting I16Positive (positive i16 value)");

        loop {
            let value = i16::elicit(communicator).await?;

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

#[cfg_attr(not(kani), instrumented_impl)]
#[cfg_attr(not(kani), instrumented_impl)]
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

#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for I16NonNegative {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a non-negative number (>= 0):")
    }
}

#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for I16NonNegative {
    type Style = I16NonNegativeStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "I16NonNegative"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting I16NonNegative (non-negative i16 value)");

        loop {
            let value = i16::elicit(communicator).await?;

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
// I16 NonZero Type
// ============================================================================

/// Contract type for non-zero i16 values (!= 0).
#[contract_type(requires = "value != 0", ensures = "result.get() != 0")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct I16NonZero(i16);

#[cfg_attr(not(kani), instrumented_impl)]
impl I16NonZero {
    /// Creates a new `I16NonZero` if value is non-zero.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::Zero` if value == 0.
    pub fn new(value: i16) -> Result<Self, ValidationError> {
        if value != 0 {
            Ok(Self(value))
        } else {
            Err(ValidationError::Zero)
        }
    }

    /// Gets the inner value.
    pub fn get(&self) -> i16 {
        self.0
    }

    /// Unwraps to inner i16.
    pub fn into_inner(self) -> i16 {
        self.0
    }
}

crate::default_style!(I16NonZero => I16NonZeroStyle);

#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for I16NonZero {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a non-zero number (!= 0):")
    }
}

#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for I16NonZero {
    type Style = I16NonZeroStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "I16NonZero"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting I16NonZero");

        loop {
            let value = i16::elicit(communicator).await?;

            match Self::new(value) {
                Ok(non_zero) => {
                    tracing::debug!(value, "Valid I16NonZero constructed");
                    return Ok(non_zero);
                }
                Err(e) => {
                    tracing::warn!(value, error = %e, "Invalid I16NonZero, re-prompting");
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

#[cfg_attr(not(kani), instrumented_impl)]
impl crate::Prompt for I16RangeStyle {
    fn prompt() -> Option<&'static str> {
        None
    }
}

#[cfg_attr(not(kani), instrumented_impl)]
impl crate::Elicitation for I16RangeStyle {
    type Style = I16RangeStyle;

    async fn elicit<C: crate::ElicitCommunicator>(_communicator: &C) -> crate::ElicitResult<Self> {
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

    #[tracing::instrument(skip(communicator), fields(type_name = "I16Range", min = MIN, max = MAX))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting I16Range<{}, {}> (i16 in range)", MIN, MAX);

        loop {
            let value = i16::elicit(communicator).await?;

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

#[cfg_attr(not(kani), instrumented_impl)]
#[cfg_attr(not(kani), instrumented_impl)]
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

#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for U8NonZero {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a non-zero number (!= 0):")
    }
}

#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for U8NonZero {
    type Style = U8NonZeroStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "U8NonZero"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting U8NonZero (non-zero u8 value)");

        loop {
            let value = u8::elicit(communicator).await?;

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

#[cfg_attr(not(kani), instrumented_impl)]
impl crate::Prompt for U8RangeStyle {
    fn prompt() -> Option<&'static str> {
        None
    }
}

#[cfg_attr(not(kani), instrumented_impl)]
impl crate::Elicitation for U8RangeStyle {
    type Style = U8RangeStyle;

    async fn elicit<C: crate::ElicitCommunicator>(_communicator: &C) -> crate::ElicitResult<Self> {
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

    #[tracing::instrument(skip(communicator), fields(type_name = "U8Range", min = MIN, max = MAX))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting U8Range<{}, {}> (u8 in range)", MIN, MAX);

        loop {
            let value = u8::elicit(communicator).await?;

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

#[cfg_attr(not(kani), instrumented_impl)]
#[cfg_attr(not(kani), instrumented_impl)]
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

#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for U16NonZero {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a non-zero number (!= 0):")
    }
}

#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for U16NonZero {
    type Style = U16NonZeroStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "U16NonZero"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting U16NonZero (non-zero u16 value)");

        loop {
            let value = u16::elicit(communicator).await?;

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

#[cfg_attr(not(kani), instrumented_impl)]
impl crate::Prompt for U16RangeStyle {
    fn prompt() -> Option<&'static str> {
        None
    }
}

#[cfg_attr(not(kani), instrumented_impl)]
impl crate::Elicitation for U16RangeStyle {
    type Style = U16RangeStyle;

    async fn elicit<C: crate::ElicitCommunicator>(_communicator: &C) -> crate::ElicitResult<Self> {
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

    #[tracing::instrument(skip(communicator), fields(type_name = "U16Range", min = MIN, max = MAX))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting U16Range<{}, {}> (u16 in range)", MIN, MAX);

        loop {
            let value = u16::elicit(communicator).await?;

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

// U8Positive - Positive u8 (> 0)
/// Contract type for positive u8 values (> 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct U8Positive(u8);

#[elicitation_macros::instrumented_impl]
impl U8Positive {
    /// Create a new U8Positive, validating value is positive (> 0).
    pub fn new(value: u8) -> Result<Self, ValidationError> {
        if value > 0 {
            Ok(Self(value))
        } else {
            Err(ValidationError::Zero)
        }
    }
    /// Get the inner u8 value.
    pub fn get(&self) -> u8 {
        self.0
    }
    /// Consume self and return the inner u8 value.
    pub fn into_inner(self) -> u8 {
        self.0
    }
}

crate::default_style!(U8Positive => U8PositiveStyle);

#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for U8Positive {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a positive number (> 0):")
    }
}

#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for U8Positive {
    type Style = U8PositiveStyle;
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        loop {
            if let Ok(v) = Self::new(u8::elicit(communicator).await?) {
                return Ok(v);
            }
        }
    }
}

// U16Positive - Positive u16 (> 0)
/// Contract type for positive u16 values (> 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct U16Positive(u16);

#[elicitation_macros::instrumented_impl]
impl U16Positive {
    /// Create a new U16Positive, validating value is positive (> 0).
    pub fn new(value: u16) -> Result<Self, ValidationError> {
        if value > 0 {
            Ok(Self(value))
        } else {
            Err(ValidationError::Zero)
        }
    }
    /// Get the inner u16 value.
    pub fn get(&self) -> u16 {
        self.0
    }
    /// Consume self and return the inner u16 value.
    pub fn into_inner(self) -> u16 {
        self.0
    }
}

crate::default_style!(U16Positive => U16PositiveStyle);

#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for U16Positive {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a positive number (> 0):")
    }
}

#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for U16Positive {
    type Style = U16PositiveStyle;
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        loop {
            if let Ok(v) = Self::new(u16::elicit(communicator).await?) {
                return Ok(v);
            }
        }
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

                #[tracing::instrument(skip(communicator), fields(type_name = stringify!($positive)))]
                async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                    tracing::debug!(concat!("Eliciting ", stringify!($positive), " (positive ", stringify!($base), " value)"));

                    loop {
                        let value = <$base>::elicit(communicator).await?;

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

                #[tracing::instrument(skip(communicator), fields(type_name = stringify!($nonnegative)))]
                async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                    tracing::debug!(concat!("Eliciting ", stringify!($nonnegative), " (non-negative ", stringify!($base), " value)"));

                    loop {
                        let value = <$base>::elicit(communicator).await?;

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

            async fn elicit<C: crate::ElicitCommunicator>(_communicator: &C) -> crate::ElicitResult<Self> {
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

            #[tracing::instrument(skip(communicator), fields(type_name = stringify!($range), min = MIN, max = MAX))]
            async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                tracing::debug!(concat!("Eliciting ", stringify!($range), "<{}, {}> (", stringify!($base), " in range)"), MIN, MAX);

                loop {
                    let value = <$base>::elicit(communicator).await?;

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

                #[tracing::instrument(skip(communicator), fields(type_name = stringify!($nonzero)))]
                async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                    tracing::debug!(concat!("Eliciting ", stringify!($nonzero), " (non-zero ", stringify!($base), " value)"));

                    loop {
                        let value = <$base>::elicit(communicator).await?;

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

            async fn elicit<C: crate::ElicitCommunicator>(_communicator: &C) -> crate::ElicitResult<Self> {
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

            #[tracing::instrument(skip(communicator), fields(type_name = stringify!($range), min = MIN, max = MAX))]
            async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                tracing::debug!(concat!("Eliciting ", stringify!($range), "<{}, {}> (", stringify!($base), " in range)"), MIN, MAX);

                loop {
                    let value = <$base>::elicit(communicator).await?;

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
impl_signed_contracts!(
    i32,
    I32Positive,
    I32NonNegative,
    I32Range,
    I32RangeStyle,
    42,
    100,
    10,
    100,
    50
);

// u32 family
impl_unsigned_contracts!(u32, U32NonZero, U32Range, U32RangeStyle, 42, 10, 100, 50);

// ============================================================================
// Default Wrappers (for MCP elicitation of primitives)
// ============================================================================

// Generate Default wrappers for all signed integer types
impl_integer_default_wrapper!(i8, I8Default, i8::MIN as i64, i8::MAX as i64);
impl_integer_default_wrapper!(i16, I16Default, i16::MIN as i64, i16::MAX as i64);
impl_integer_default_wrapper!(i32, I32Default, i32::MIN as i64, i32::MAX as i64);
impl_integer_default_wrapper!(i64, I64Default, i64::MIN, i64::MAX);
impl_integer_default_wrapper!(i128, I128Default, i64::MIN, i64::MAX); // Clamped to i64 range for MCP
impl_integer_default_wrapper!(isize, IsizeDefault, isize::MIN as i64, isize::MAX as i64);

// Generate Default wrappers for all unsigned integer types
impl_integer_default_wrapper!(u8, U8Default, 0, u8::MAX as i64);
impl_integer_default_wrapper!(u16, U16Default, 0, u16::MAX as i64);
impl_integer_default_wrapper!(u32, U32Default, 0, u32::MAX as i64);
impl_integer_default_wrapper!(u64, U64Default, 0, i64::MAX); // Clamped to i64::MAX for MCP
impl_integer_default_wrapper!(u128, U128Default, 0, i64::MAX); // Clamped to i64::MAX for MCP
impl_integer_default_wrapper!(usize, UsizeDefault, 0, isize::MAX as i64);

// ============================================================================
// Type Families Generated by Macros
// ============================================================================

// i64 family
impl_signed_contracts!(
    i64,
    I64Positive,
    I64NonNegative,
    I64Range,
    I64RangeStyle,
    42,
    100,
    10,
    100,
    50
);

// u64 family
impl_unsigned_contracts!(u64, U64NonZero, U64Range, U64RangeStyle, 42, 10, 100, 50);

// i128 family
impl_signed_contracts!(
    i128,
    I128Positive,
    I128NonNegative,
    I128Range,
    I128RangeStyle,
    42,
    100,
    10,
    100,
    50
);

// u128 family
impl_unsigned_contracts!(
    u128,
    U128NonZero,
    U128Range,
    U128RangeStyle,
    42,
    10,
    100,
    50
);

// isize family
impl_signed_contracts!(
    isize,
    IsizePositive,
    IsizeNonNegative,
    IsizeRange,
    IsizeRangeStyle,
    42,
    100,
    10,
    100,
    50
);

// usize family
impl_unsigned_contracts!(
    usize,
    UsizeNonZero,
    UsizeRange,
    UsizeRangeStyle,
    42,
    10,
    100,
    50
);

// ============================================================================
// Additional Signed NonZero Types (for Prusti proofs)
// ============================================================================

macro_rules! impl_signed_nonzero {
    ($base:ty, $nonzero:ident, $test_value:expr) => {
        #[doc = concat!("Contract type for non-zero ", stringify!($base), " values (!= 0).")]
        #[contract_type(requires = "value != 0", ensures = "result.get() != 0")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $nonzero($base);

        impl $nonzero {
            /// Creates a non-zero value.
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

            /// Unwraps to stdlib type.
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

                #[tracing::instrument(skip(communicator), fields(type_name = stringify!($nonzero)))]
                async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                    loop {
                        let value = <$base>::elicit(communicator).await?;
                        match Self::new(value) {
                            Ok(v) => return Ok(v),
                            Err(e) => tracing::warn!(error = %e, "Invalid, re-prompting"),
                        }
                    }
                }
            }
        }
    };
}

// Generate missing signed NonZero types
impl_signed_nonzero!(i32, I32NonZero, 42);
impl_signed_nonzero!(i64, I64NonZero, 42);
impl_signed_nonzero!(i128, I128NonZero, 42);
impl_signed_nonzero!(isize, IsizeNonZero, 42);

// ============================================================================
// Additional Unsigned Positive Types (for Prusti proofs)
// ============================================================================

macro_rules! impl_unsigned_positive {
    ($base:ty, $positive:ident, $test_value:expr) => {
        #[doc = concat!("Contract type for positive ", stringify!($base), " values (> 0).")]
        #[contract_type(requires = "value > 0", ensures = "result.get() > 0")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $positive($base);

        impl $positive {
            /// Creates a positive value.
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

            /// Unwraps to stdlib type.
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

                #[tracing::instrument(skip(communicator), fields(type_name = stringify!($positive)))]
                async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                    loop {
                        let value = <$base>::elicit(communicator).await?;
                        match Self::new(value) {
                            Ok(v) => return Ok(v),
                            Err(e) => tracing::warn!(error = %e, "Invalid, re-prompting"),
                        }
                    }
                }
            }
        }
    };
}

// Generate missing unsigned Positive types
impl_unsigned_positive!(u32, U32Positive, 42);
impl_unsigned_positive!(u64, U64Positive, 42);
impl_unsigned_positive!(u128, U128Positive, 42);
impl_unsigned_positive!(usize, UsizePositive, 42);

// ============================================================================
// Float Contract Types (f32, f64)
// ============================================================================
