//! Float contract types.

use crate::{ElicitClient, ElicitResult, Elicitation, Prompt};
use super::ValidationError;
use elicitation_macros::instrumented_impl;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

// ============================================================================
// Macro: Default Wrapper Generation for Float Types
// ============================================================================

/// Generate a Default wrapper for a float primitive type.
///
/// This macro creates an unconstrained wrapper type that:
/// - Has Serialize, Deserialize, and JsonSchema derives
/// - Is marked as elicit-safe for rmcp
/// - Uses serde deserialization instead of manual parsing
/// - Provides new/get/into_inner methods
/// - Implements Prompt and Elicitation traits
macro_rules! impl_float_default_wrapper {
    ($primitive:ty, $wrapper:ident) => {
        #[doc = concat!("Default wrapper for ", stringify!($primitive), " (unconstrained).")]
        ///
        /// Used internally for MCP elicitation of primitive float values.
        /// Provides JsonSchema for client-side validation.
        #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
        #[schemars(description = concat!(stringify!($primitive), " value"))]
        pub struct $wrapper(
            #[schemars(description = "Float value")]
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
                    Some("Please enter a number:")
                }
            }

            impl Elicitation for $wrapper {
                type Style = [<$wrapper Style>];

                #[tracing::instrument(skip(client))]
                async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
                    let prompt = Self::prompt().unwrap();
                    tracing::debug!(concat!("Eliciting ", stringify!($wrapper), " with serde deserialization"));

                    let params = crate::mcp::text_params(prompt);

                    let result = client
                        .peer()
                        .call_tool(rmcp::model::CallToolRequestParams {
                            meta: None,
                            name: crate::mcp::tool_names::elicit_text().into(),
                            arguments: Some(params),
                            task: None,
                        })
                        .await?;

                    let value = crate::mcp::extract_value(result)?;

                    // Use serde to deserialize directly into wrapper type
                    // Preserves error source via From<serde_json::Error> chain
                    Ok(serde_json::from_value(value)?)
                }
            }
        }
    };
}

// ============================================================================
// Verification Types (Constrained)
// ============================================================================

// F32Positive (f32 > 0.0 and finite)
/// Contract type for positive f32 values (> 0.0).
///
/// Validates on construction, then can unwrap to stdlib f32 via `into_inner()`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct F32Positive(f32);

#[instrumented_impl]
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

#[instrumented_impl]
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

#[instrumented_impl]
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

#[instrumented_impl]
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

#[instrumented_impl]
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

#[instrumented_impl]
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
// Default Wrappers (for MCP elicitation of primitives)
// ============================================================================

// Generate Default wrappers for all float types
impl_float_default_wrapper!(f32, F32Default);
impl_float_default_wrapper!(f64, F64Default);
