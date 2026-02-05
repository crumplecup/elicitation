//! Duration contract types.

use super::ValidationError;
use crate::{ElicitClient, ElicitResult, Elicitation, Prompt};
use elicitation_macros::instrumented_impl;
use std::time::Duration;

// DurationPositive - Positive durations (> zero)
/// A Duration that is guaranteed to be positive (not zero).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DurationPositive(Duration);

#[cfg_attr(not(kani), instrumented_impl)]
impl DurationPositive {
    /// Create a new DurationPositive, validating it's not zero.
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

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient) -> ElicitResult<Self> {
        tracing::debug!("Eliciting DurationPositive");
        loop {
            let duration = Duration::elicit(client).await?;
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
