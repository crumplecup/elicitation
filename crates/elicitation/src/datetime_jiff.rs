//! jiff datetime library elicitation implementations.
//!
//! Available with the `jiff` feature.
//!
//! Provides both direct elicitation and generator-based creation for jiff types.
//!
//! # Generator Pattern
//!
//! ```rust,no_run
//! use elicitation::{TimestampGenerationMode, TimestampGenerator, Generator};
//! use jiff::Timestamp;
//!
//! // Choose generation mode
//! let mode = TimestampGenerationMode::Now; // Current UTC time
//!
//! // Create generator
//! let generator = TimestampGenerator::new(mode);
//!
//! // Generate multiple timestamps
//! let t1 = generator.generate();
//! let t2 = generator.generate();
//! ```

use crate::{
    ElicitClient, ElicitError, ElicitErrorKind, ElicitResult, Elicitation, Generator, Prompt,
    Select,
    datetime_common::{DateTimeComponents, DateTimeInputMethod},
    mcp,
};
use jiff::{Span, Timestamp, Zoned, civil::DateTime as CivilDateTime, tz::TimeZone};

// Style enums for jiff types
crate::default_style!(Timestamp => TimestampStyle);
crate::default_style!(Zoned => ZonedStyle);
crate::default_style!(CivilDateTime => CivilDateTimeStyle);
crate::default_style!(TimestampGenerationMode => TimestampGenerationModeStyle);

// ============================================================================
// Timestamp Generator
// ============================================================================

/// Generation mode for jiff::Timestamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimestampGenerationMode {
    /// Use current UTC time.
    Now,
    /// Use Unix epoch (1970-01-01 00:00:00 UTC).
    UnixEpoch,
    /// Offset from reference time.
    Offset {
        /// Seconds offset.
        seconds: i64,
    },
}

impl Select for TimestampGenerationMode {
    fn options() -> &'static [Self] {
        &[
            TimestampGenerationMode::Now,
            TimestampGenerationMode::UnixEpoch,
            TimestampGenerationMode::Offset { seconds: 0 },
        ]
    }

    fn labels() -> &'static [&'static str] {
        &[
            "Now (Current UTC)",
            "Unix Epoch (1970-01-01)",
            "Offset (Custom)",
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Now (Current UTC)" => Some(TimestampGenerationMode::Now),
            "Unix Epoch (1970-01-01)" => Some(TimestampGenerationMode::UnixEpoch),
            "Offset (Custom)" => Some(TimestampGenerationMode::Offset { seconds: 0 }),
            _ => None,
        }
    }
}

impl Prompt for TimestampGenerationMode {
    fn prompt() -> Option<&'static str> {
        Some("How should timestamps be generated?")
    }
}

impl Elicitation for TimestampGenerationMode {
    type Style = TimestampGenerationModeStyle;

    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
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
                "Invalid Timestamp generation mode".to_string(),
            ))
        })?;

        match selected {
            TimestampGenerationMode::Now => Ok(TimestampGenerationMode::Now),
            TimestampGenerationMode::UnixEpoch => Ok(TimestampGenerationMode::UnixEpoch),
            TimestampGenerationMode::Offset { .. } => {
                let seconds = i64::elicit(communicator).await?;
                Ok(TimestampGenerationMode::Offset { seconds })
            }
        }
    }
}

/// Generator for creating Timestamp values.
#[derive(Debug, Clone, Copy)]
pub struct TimestampGenerator {
    mode: TimestampGenerationMode,
    reference: Timestamp,
}

impl TimestampGenerator {
    /// Create a new Timestamp generator.
    pub fn new(mode: TimestampGenerationMode) -> Self {
        Self {
            mode,
            reference: Timestamp::now(),
        }
    }

    /// Create a generator with a custom reference time.
    pub fn with_reference(mode: TimestampGenerationMode, reference: Timestamp) -> Self {
        Self { mode, reference }
    }

    /// Get the generation mode.
    pub fn mode(&self) -> TimestampGenerationMode {
        self.mode
    }

    /// Get the reference time.
    pub fn reference(&self) -> Timestamp {
        self.reference
    }
}

impl Generator for TimestampGenerator {
    type Target = Timestamp;

    fn generate(&self) -> Self::Target {
        match self.mode {
            TimestampGenerationMode::Now => Timestamp::now(),
            TimestampGenerationMode::UnixEpoch => Timestamp::UNIX_EPOCH,
            TimestampGenerationMode::Offset { seconds } => {
                let span = Span::new().seconds(seconds);
                self.reference.checked_add(span).unwrap_or(self.reference)
            }
        }
    }
}

// ============================================================================
// Timestamp Elicitation
// ============================================================================

// Timestamp implementation
impl Prompt for Timestamp {
    fn prompt() -> Option<&'static str> {
        Some("Enter UTC timestamp:")
    }
}

impl Elicitation for Timestamp {
    type Style = TimestampStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Timestamp");

        // Step 1: Choose input method
        let method = DateTimeInputMethod::elicit(communicator).await?;
        tracing::debug!(?method, "Input method selected");

        match method {
            DateTimeInputMethod::Iso8601String => {
                // Elicit ISO 8601 string
                let prompt = "Enter ISO 8601 timestamp (e.g., \"2024-07-11T15:30:00Z\"):";
                let params = mcp::text_params(prompt);
                let result = client
                    .peer()
                    .call_tool(rmcp::model::CallToolRequestParams {
                        meta: None,
                        name: mcp::tool_names::elicit_text().into(),
                        arguments: Some(params),
                        task: None,
                    })
                    .await?;

                let value = mcp::extract_value(result)?;
                let iso_string = mcp::parse_string(value)?;

                // Parse ISO 8601
                iso_string.parse::<Timestamp>().map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid ISO 8601 timestamp: {}",
                        e
                    )))
                })
            }
            DateTimeInputMethod::ManualComponents => {
                // Elicit components
                let components = DateTimeComponents::elicit(communicator).await?;

                // Construct CivilDateTime then convert to Timestamp (assumes UTC)
                let dt = CivilDateTime::new(
                    components.year as i16,
                    components.month as i8,
                    components.day as i8,
                    components.hour as i8,
                    components.minute as i8,
                    components.second as i8,
                    0,
                )
                .map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid datetime components: {}",
                        e
                    )))
                })?;

                // Convert to timestamp (assumes UTC)
                dt.to_zoned(TimeZone::UTC)
                    .map(|z| z.timestamp())
                    .map_err(|e| {
                        ElicitError::new(ElicitErrorKind::ParseError(format!(
                            "Failed to create timestamp: {}",
                            e
                        )))
                    })
            }
        }
    }
}

// Zoned implementation
impl Prompt for Zoned {
    fn prompt() -> Option<&'static str> {
        Some("Enter datetime with timezone:")
    }
}

impl Elicitation for Zoned {
    type Style = ZonedStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Zoned");

        // Step 1: Choose input method
        let method = DateTimeInputMethod::elicit(communicator).await?;
        tracing::debug!(?method, "Input method selected");

        match method {
            DateTimeInputMethod::Iso8601String => {
                // Elicit ISO 8601 string with timezone
                let prompt = "Enter ISO 8601 datetime with timezone (e.g., \"2024-07-11T15:30:00-05[America/New_York]\"):";
                let params = mcp::text_params(prompt);
                let result = client
                    .peer()
                    .call_tool(rmcp::model::CallToolRequestParams {
                        meta: None,
                        name: mcp::tool_names::elicit_text().into(),
                        arguments: Some(params),
                        task: None,
                    })
                    .await?;

                let value = mcp::extract_value(result)?;
                let iso_string = mcp::parse_string(value)?;

                // Parse ISO 8601
                iso_string.parse::<Zoned>().map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid ISO 8601 zoned datetime: {}",
                        e
                    )))
                })
            }
            DateTimeInputMethod::ManualComponents => {
                // Elicit components
                let components = DateTimeComponents::elicit(communicator).await?;

                // Elicit timezone
                let tz_prompt = "Enter IANA timezone (e.g., \"America/New_York\" or \"UTC\"):";
                let tz_params = mcp::text_params(tz_prompt);
                let tz_result = client
                    .peer()
                    .call_tool(rmcp::model::CallToolRequestParams {
                        meta: None,
                        name: mcp::tool_names::elicit_text().into(),
                        arguments: Some(tz_params),
                        task: None,
                    })
                    .await?;

                let tz_value = mcp::extract_value(tz_result)?;
                let tz_string = mcp::parse_string(tz_value)?;

                let tz = TimeZone::get(&tz_string).map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid timezone: {}",
                        e
                    )))
                })?;

                // Construct CivilDateTime
                let dt = CivilDateTime::new(
                    components.year as i16,
                    components.month as i8,
                    components.day as i8,
                    components.hour as i8,
                    components.minute as i8,
                    components.second as i8,
                    0,
                )
                .map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid datetime components: {}",
                        e
                    )))
                })?;

                // Convert to zoned
                dt.to_zoned(tz).map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Failed to create zoned datetime: {}",
                        e
                    )))
                })
            }
        }
    }
}

// civil::DateTime implementation
impl Prompt for CivilDateTime {
    fn prompt() -> Option<&'static str> {
        Some("Enter civil datetime (no timezone):")
    }
}

impl Elicitation for CivilDateTime {
    type Style = CivilDateTimeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting CivilDateTime");

        // Step 1: Choose input method
        let method = DateTimeInputMethod::elicit(communicator).await?;
        tracing::debug!(?method, "Input method selected");

        match method {
            DateTimeInputMethod::Iso8601String => {
                // Elicit ISO 8601 string (no timezone)
                let prompt = "Enter datetime (e.g., \"2024-07-11T15:30:00\"):";
                let params = mcp::text_params(prompt);
                let result = client
                    .peer()
                    .call_tool(rmcp::model::CallToolRequestParams {
                        meta: None,
                        name: mcp::tool_names::elicit_text().into(),
                        arguments: Some(params),
                        task: None,
                    })
                    .await?;

                let value = mcp::extract_value(result)?;
                let iso_string = mcp::parse_string(value)?;

                // Parse ISO 8601
                iso_string.parse::<CivilDateTime>().map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid civil datetime: {}",
                        e
                    )))
                })
            }
            DateTimeInputMethod::ManualComponents => {
                // Elicit components
                let components = DateTimeComponents::elicit(communicator).await?;

                // Construct CivilDateTime
                CivilDateTime::new(
                    components.year as i16,
                    components.month as i8,
                    components.day as i8,
                    components.hour as i8,
                    components.minute as i8,
                    components.second as i8,
                    0,
                )
                .map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid datetime components: {}",
                        e
                    )))
                })
            }
        }
    }
}
