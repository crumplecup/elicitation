//! Bool contract types.

use crate::{ElicitClient, ElicitResult, Elicitation, Prompt};
use super::ValidationError;
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
