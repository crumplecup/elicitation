//! Duration type implementation for time duration elicitation.

use crate::{ElicitClient, ElicitError, ElicitErrorKind, ElicitResult, Elicitation, Prompt};
use std::time::Duration;

// Generate default-only style enum
crate::default_style!(Duration => DurationStyle);

impl Prompt for Duration {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a duration in seconds (can be decimal, e.g., 1.5):")
    }
}

impl Elicitation for Duration {
    type Style = DurationStyle;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Duration");

        // Elicit as f64 (supports decimal seconds)
        let seconds = f64::elicit(client).await?;

        // Validate non-negative
        if seconds < 0.0 {
            tracing::warn!(seconds, "Negative duration not allowed");
            return Err(ElicitError::new(ElicitErrorKind::OutOfRange {
                min: "0".to_string(),
                max: "positive number".to_string(),
            }));
        }

        // Convert to Duration
        let duration = Duration::from_secs_f64(seconds);
        tracing::debug!(?duration, seconds, "Duration created");

        Ok(duration)
    }
}
