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

#[cfg(feature = "verification")]
use super::Contract;

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

#[cfg(feature = "verification")]
impl Contract for I8Positive {
    type Input = I8Positive;
    type Output = I8Positive;

    fn requires(input: &Self::Input) -> bool {
        input.0 > 0
    }

    fn ensures(output: &Self::Output) -> bool {
        output.0 > 0
    }

    fn invariant(value: &Self) -> bool {
        value.0 > 0 // Always true by construction!
    }
}

// TODO: Implement Elicitation for I8Positive
// This will:
// 1. Elicit i8 from client
// 2. Loop until I8Positive::new() succeeds
// 3. Return I8Positive (ready to be unwrapped via .into_inner())

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

    #[cfg(feature = "verification")]
    #[test]
    fn i8_positive_invariant_holds() {
        let positive = I8Positive::new(42).unwrap();
        assert!(I8Positive::invariant(&positive));
    }

    #[cfg(feature = "verification")]
    #[test]
    fn i8_positive_contract_requires() {
        let positive = I8Positive::new(42).unwrap();
        assert!(I8Positive::requires(&positive));
    }

    #[cfg(feature = "verification")]
    #[test]
    fn i8_positive_contract_ensures() {
        let positive = I8Positive::new(42).unwrap();
        assert!(I8Positive::ensures(&positive));
    }
}
