//! Bool contract types.

use super::ValidationError;
use crate::{ElicitCommunicator, ElicitResult, Elicitation, Prompt};
use elicitation_macros::instrumented_impl;

// ============================================================================

/// Contract type for true bool values.
///
/// Validates on construction, then can unwrap to stdlib bool via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoolTrue(bool);

#[cfg_attr(not(kani), instrumented_impl)]
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

    #[tracing::instrument(skip(communicator), fields(type_name = "BoolTrue"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting BoolTrue (must be true)");

        loop {
            let value = bool::elicit(communicator).await?;

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

    #[tracing::instrument(skip(communicator), fields(type_name = "BoolFalse"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting BoolFalse (must be false)");

        loop {
            let value = bool::elicit(communicator).await?;

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
        assert!(result.unwrap().get());
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
        assert!(value);
    }
}

#[cfg(test)]
mod bool_false_tests {
    use super::*;

    #[test]
    fn bool_false_new_valid() {
        let result = BoolFalse::new(false);
        assert!(result.is_ok());
        assert!(!result.unwrap().get());
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
        assert!(!value);
    }
}

// ============================================================================

/// Default bool wrapper for MCP elicitation.
///
/// Provides JSON Schema validation and serialization for booleans.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[schemars(description = "A boolean value")]
pub struct BoolDefault(#[schemars(description = "True or false")] bool);

impl BoolDefault {
    /// Creates a new bool wrapper.
    pub fn new(b: bool) -> Self {
        Self(b)
    }

    /// Returns the inner bool.
    pub fn get(&self) -> bool {
        self.0
    }

    /// Converts to inner bool.
    pub fn into_inner(self) -> bool {
        self.0
    }
}

// Mark as elicit-safe for rmcp
rmcp::elicit_safe!(BoolDefault);

crate::default_style!(BoolDefault => BoolDefaultStyle);

impl Prompt for BoolDefault {
    fn prompt() -> Option<&'static str> {
        Some("Please enter true or false:")
    }
}

impl Elicitation for BoolDefault {
    type Style = BoolDefaultStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let prompt = Self::prompt().ok_or_else(|| {
            crate::ElicitError::new(crate::ElicitErrorKind::InvalidFormat {
                expected: "valid prompt".to_string(),
                received: "None".to_string(),
            })
        })?;
        tracing::debug!("Eliciting BoolDefault with prompt-based parsing");

        // Use send_prompt for server compatibility
        let response = communicator.send_prompt(prompt).await?;
        
        tracing::debug!(response = %response, "Received response, parsing as bool");

        // Parse response as boolean (case-insensitive)
        let trimmed = response.trim().to_lowercase();
        let value = match trimmed.as_str() {
            "true" | "yes" | "y" | "1" => true,
            "false" | "no" | "n" | "0" => false,
            _ => {
                tracing::error!(response = %trimmed, "Invalid boolean response");
                return Err(crate::ElicitError::new(crate::ElicitErrorKind::ParseError(
                    format!("Invalid boolean: '{}' (expected true/false, yes/no, y/n, 1/0)", trimmed)
                )));
            }
        };

        tracing::debug!(value = value, "Successfully parsed boolean");
        Ok(Self::new(value))
    }
}
