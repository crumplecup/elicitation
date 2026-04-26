//! Duration contract types.

use super::ValidationError;
use crate::{ElicitCommunicator, ElicitResult, Elicitation, Prompt};
use anodized::spec;
#[cfg(not(kani))]
use elicitation_derive::instrumented_impl;
use std::time::Duration;

// DurationPositive - Positive durations (> zero)
/// A Duration that is guaranteed to be positive (not zero).
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
#[schemars(description = "A positive duration (greater than zero)")]
pub struct DurationPositive(Duration);

#[cfg_attr(not(kani), instrumented_impl)]
impl DurationPositive {
    /// Create a new DurationPositive, validating it's not zero.
    #[spec(requires: [duration.as_nanos() > 0])]
    pub fn new(duration: Duration) -> Result<Self, ValidationError> {
        if duration.as_nanos() > 0 {
            Ok(Self(duration))
        } else {
            Err(ValidationError::DurationNotPositive)
        }
    }

    /// Get the inner Duration.
    pub fn get(&self) -> Duration {
        self.0
    }

    /// Unwrap into the inner Duration.
    pub fn into_inner(self) -> Duration {
        self.0
    }
}

#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for DurationPositive {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a positive duration (greater than zero seconds):")
    }
}

#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for DurationPositive {
    type Style = <Duration as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting DurationPositive");
        loop {
            let duration = Duration::elicit(communicator).await?;
            match Self::new(duration) {
                Ok(valid) => {
                    tracing::debug!(duration = ?valid.0, "Valid positive duration");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Duration not positive, re-prompting");
                    continue;
                }
            }
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_duration_positive()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_duration()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_duration()
    }
}

// DurationNonZero - Non-zero durations (same as positive for Duration)
/// A Duration that is guaranteed to be non-zero.
///
/// Note: Duration can't be negative, so NonZero is equivalent to Positive.
pub type DurationNonZero = DurationPositive;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_positive_new_valid() {
        let duration = Duration::from_secs(1);
        let result = DurationPositive::new(duration);
        assert!(result.is_ok());
    }

    #[test]
    fn test_duration_positive_new_zero() {
        let duration = Duration::from_secs(0);
        let result = DurationPositive::new(duration);
        assert!(result.is_err());
    }

    #[test]
    fn test_duration_positive_get() {
        let duration = Duration::from_millis(500);
        let positive = DurationPositive::new(duration).unwrap();
        assert_eq!(positive.get(), duration);
    }

    #[test]
    fn test_duration_positive_into_inner() {
        let duration = Duration::from_nanos(123456789);
        let positive = DurationPositive::new(duration).unwrap();
        assert_eq!(positive.into_inner(), duration);
    }

    #[test]
    fn test_duration_non_zero_alias() {
        let duration = Duration::from_secs(5);
        let non_zero: DurationNonZero = DurationPositive::new(duration).unwrap();
        assert_eq!(non_zero.into_inner(), duration);
    }
}

// ── ToCodeLiteral impls ───────────────────────────────────────────────────────

mod emit_impls {
    use super::*;
    use crate::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for DurationPositive {
        fn to_code_literal(&self) -> TokenStream {
            let secs = self.get().as_secs();
            let nanos = self.get().subsec_nanos();
            quote::quote! {
                elicitation::DurationPositive::new(
                    ::std::time::Duration::new(#secs, #nanos)
                ).expect("valid DurationPositive")
            }
        }
    }
}

// ── ElicitIntrospect impls ────────────────────────────────────────────────────

macro_rules! impl_primitive_introspect {
    ($($ty:ty => $name:literal),+ $(,)?) => {
        $(
            impl crate::ElicitIntrospect for $ty {
                fn pattern() -> crate::ElicitationPattern {
                    crate::ElicitationPattern::Primitive
                }
                fn metadata() -> crate::TypeMetadata {
                    crate::TypeMetadata {
                        type_name: $name,
                        description: <$ty as crate::Prompt>::prompt(),
                        details: crate::PatternDetails::Primitive,
                    }
                }
            }
        )+
    };
}

impl_primitive_introspect!(
    DurationPositive => "DurationPositive",
);
