//! String contract types.

use crate::{ElicitClient, ElicitResult, Elicitation, Prompt};
use super::ValidationError;
use elicitation_macros::instrumented_impl;

// ============================================================================

/// Contract type for non-empty String values.
///
/// Validates on construction, then can unwrap to stdlib String via `into_inner()`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringNonEmpty(String);

#[instrumented_impl]
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

#[instrumented_impl]
impl Prompt for StringNonEmpty {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a non-empty string:")
    }
}

#[instrumented_impl]
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
