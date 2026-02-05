//! Duration type implementation for time duration elicitation.
//!
//! Provides both direct elicitation and generator-based creation of Duration.
//!
//! # Generator Pattern
//!
//! The Duration generator supports multiple creation strategies useful for testing:
//!
//! ```rust,no_run
//! use elicitation::{DurationGenerationMode, DurationGenerator, Generator};
//! use std::time::Duration;
//!
//! // Choose generation mode
//! let mode = DurationGenerationMode::FromSecs(30); // 30 seconds
//! // or Zero, FromMillis, FromMicros, FromNanos
//!
//! // Create generator
//! let generator = DurationGenerator::new(mode);
//!
//! // Generate multiple durations with same strategy
//! let d1 = generator.generate();
//! let d2 = generator.generate();
//! let d3 = generator.generate();
//! ```

use crate::{
    ElicitClient, ElicitError, ElicitErrorKind, ElicitResult, Elicitation, Generator, Prompt,
    Select, mcp,
};
use std::time::Duration;

// Generate default-only style enums
crate::default_style!(Duration => DurationStyle);
crate::default_style!(DurationGenerationMode => DurationGenerationModeStyle);

// ============================================================================
// Duration Generator
// ============================================================================

/// Generation mode for Duration.
///
/// This enum allows an agent (or user) to specify how to create a Duration:
/// - `Zero`: Zero duration
/// - `FromSecs`: Duration from seconds
/// - `FromMillis`: Duration from milliseconds
/// - `FromMicros`: Duration from microseconds
/// - `FromNanos`: Duration from nanoseconds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DurationGenerationMode {
    /// Zero duration.
    Zero,
    /// Duration from seconds.
    FromSecs(u64),
    /// Duration from milliseconds.
    FromMillis(u64),
    /// Duration from microseconds.
    FromMicros(u64),
    /// Duration from nanoseconds.
    FromNanos(u64),
}

impl Select for DurationGenerationMode {
    fn options() -> &'static [Self] {
        &[
            DurationGenerationMode::Zero,
            DurationGenerationMode::FromSecs(0),
            DurationGenerationMode::FromMillis(0),
            DurationGenerationMode::FromMicros(0),
            DurationGenerationMode::FromNanos(0),
        ]
    }

    fn labels() -> &'static [&'static str] {
        &[
            "Zero",
            "From Seconds",
            "From Milliseconds",
            "From Microseconds",
            "From Nanoseconds",
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Zero" => Some(DurationGenerationMode::Zero),
            "From Seconds" => Some(DurationGenerationMode::FromSecs(0)),
            "From Milliseconds" => Some(DurationGenerationMode::FromMillis(0)),
            "From Microseconds" => Some(DurationGenerationMode::FromMicros(0)),
            "From Nanoseconds" => Some(DurationGenerationMode::FromNanos(0)),
            _ => None,
        }
    }
}

impl Prompt for DurationGenerationMode {
    fn prompt() -> Option<&'static str> {
        Some("How should durations be created?")
    }
}

impl Elicitation for DurationGenerationMode {
    type Style = DurationGenerationModeStyle;

    async fn elicit(client: &ElicitClient) -> ElicitResult<Self> {
        // Use standard Select elicit pattern
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Select an option:"),
            Self::labels(),
        );

        let result = client
            .peer()
            .call_tool(rmcp::model::CallToolRequestParams {
                meta: None,
                name: mcp::tool_names::elicit_select().into(),
                arguments: Some(params),
                task: None,
            })
            .await?;

        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;

        let selected = Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(
                "Invalid Duration generation mode".to_string(),
            ))
        })?;

        // If a time unit was selected, elicit the value
        match selected {
            DurationGenerationMode::Zero => Ok(DurationGenerationMode::Zero),
            DurationGenerationMode::FromSecs(_) => {
                let secs = u64::elicit(client).await?;
                Ok(DurationGenerationMode::FromSecs(secs))
            }
            DurationGenerationMode::FromMillis(_) => {
                let millis = u64::elicit(client).await?;
                Ok(DurationGenerationMode::FromMillis(millis))
            }
            DurationGenerationMode::FromMicros(_) => {
                let micros = u64::elicit(client).await?;
                Ok(DurationGenerationMode::FromMicros(micros))
            }
            DurationGenerationMode::FromNanos(_) => {
                let nanos = u64::elicit(client).await?;
                Ok(DurationGenerationMode::FromNanos(nanos))
            }
        }
    }
}

/// Generator for creating Duration values with a specified strategy.
///
/// Created from a [`DurationGenerationMode`] to enable consistent duration
/// generation across multiple calls.
#[derive(Debug, Clone, Copy)]
pub struct DurationGenerator {
    mode: DurationGenerationMode,
}

impl DurationGenerator {
    /// Create a new Duration generator with the specified mode.
    pub fn new(mode: DurationGenerationMode) -> Self {
        Self { mode }
    }

    /// Get the generation mode.
    pub fn mode(&self) -> DurationGenerationMode {
        self.mode
    }
}

impl Generator for DurationGenerator {
    type Target = Duration;

    fn generate(&self) -> Self::Target {
        match self.mode {
            DurationGenerationMode::Zero => Duration::ZERO,
            DurationGenerationMode::FromSecs(secs) => Duration::from_secs(secs),
            DurationGenerationMode::FromMillis(millis) => Duration::from_millis(millis),
            DurationGenerationMode::FromMicros(micros) => Duration::from_micros(micros),
            DurationGenerationMode::FromNanos(nanos) => Duration::from_nanos(nanos),
        }
    }
}

// ============================================================================
// Duration Elicitation
// ============================================================================

impl Prompt for Duration {
    fn prompt() -> Option<&'static str> {
        Some("Choose how to create the duration:")
    }
}

impl Elicitation for Duration {
    type Style = DurationStyle;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Duration");

        // Elicit generation mode from agent
        let mode = DurationGenerationMode::elicit(client).await?;

        // Create generator and generate duration
        let generator = DurationGenerator::new(mode);
        let duration = generator.generate();

        tracing::debug!(?duration, mode = ?mode, "Generated Duration");
        Ok(duration)
    }
}
