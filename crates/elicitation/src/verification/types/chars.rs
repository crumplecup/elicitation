//! Char contract types.

use crate::{ElicitClient, ElicitResult, Elicitation, Prompt};
use super::ValidationError;
use elicitation_macros::instrumented_impl;

// ============================================================================

/// Contract type for alphabetic char values.
///
/// Validates on construction, then can unwrap to stdlib char via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CharAlphabetic(char);

#[instrumented_impl]
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

#[instrumented_impl]
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
