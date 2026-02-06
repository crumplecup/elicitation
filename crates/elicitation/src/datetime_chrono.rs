//! chrono datetime library elicitation implementations.
//!
//! Available with the `chrono` feature.
//!
//! Provides both direct elicitation and generator-based creation for chrono types.
//!
//! # Generator Pattern
//!
//! ```rust,no_run
//! use elicitation::{DateTimeUtcGenerationMode, DateTimeUtcGenerator, Generator};
//! use chrono::{DateTime, Utc};
//!
//! // Choose generation mode
//! let mode = DateTimeUtcGenerationMode::Now; // Current UTC time
//!
//! // Create generator
//! let generator = DateTimeUtcGenerator::new(mode);
//!
//! // Generate multiple timestamps
//! let t1 = generator.generate();
//! let t2 = generator.generate();
//! ```

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitResult, Elicitation, Generator, Prompt,
    Select,
    datetime_common::{DateTimeComponents, DateTimeInputMethod},
    mcp,
};
use chrono::{DateTime, Duration, FixedOffset, NaiveDateTime, TimeZone, Utc};

// Style enums for datetime types
crate::default_style!(DateTime<Utc> => DateTimeUtcStyle);
crate::default_style!(DateTime<FixedOffset> => DateTimeFixedOffsetStyle);
crate::default_style!(NaiveDateTime => NaiveDateTimeStyle);
crate::default_style!(DateTimeUtcGenerationMode => DateTimeUtcGenerationModeStyle);
crate::default_style!(NaiveDateTimeGenerationMode => NaiveDateTimeGenerationModeStyle);

// ============================================================================
// DateTime<Utc> Generator
// ============================================================================

/// Generation mode for chrono::DateTime<Utc>.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DateTimeUtcGenerationMode {
    /// Use current UTC time.
    Now,
    /// Use Unix epoch (1970-01-01 00:00:00 UTC).
    UnixEpoch,
    /// Offset from reference time.
    Offset {
        /// Seconds offset (positive = future, negative = past).
        seconds: i64,
    },
}

impl Select for DateTimeUtcGenerationMode {
    fn options() -> &'static [Self] {
        &[
            DateTimeUtcGenerationMode::Now,
            DateTimeUtcGenerationMode::UnixEpoch,
            DateTimeUtcGenerationMode::Offset { seconds: 0 },
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
            "Now (Current UTC)" => Some(DateTimeUtcGenerationMode::Now),
            "Unix Epoch (1970-01-01)" => Some(DateTimeUtcGenerationMode::UnixEpoch),
            "Offset (Custom)" => Some(DateTimeUtcGenerationMode::Offset { seconds: 0 }),
            _ => None,
        }
    }
}

impl Prompt for DateTimeUtcGenerationMode {
    fn prompt() -> Option<&'static str> {
        Some("How should UTC datetimes be generated?")
    }
}

impl Elicitation for DateTimeUtcGenerationMode {
    type Style = DateTimeUtcGenerationModeStyle;

    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Select an option:"),
            Self::labels(),
        );

        let result = communicator
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
                "Invalid DateTime<Utc> generation mode".to_string(),
            ))
        })?;

        match selected {
            DateTimeUtcGenerationMode::Now => Ok(DateTimeUtcGenerationMode::Now),
            DateTimeUtcGenerationMode::UnixEpoch => Ok(DateTimeUtcGenerationMode::UnixEpoch),
            DateTimeUtcGenerationMode::Offset { .. } => {
                let seconds = i64::elicit(communicator).await?;
                Ok(DateTimeUtcGenerationMode::Offset { seconds })
            }
        }
    }
}

/// Generator for creating DateTime<Utc> values.
#[derive(Debug, Clone, Copy)]
pub struct DateTimeUtcGenerator {
    mode: DateTimeUtcGenerationMode,
    reference: DateTime<Utc>,
}

impl DateTimeUtcGenerator {
    /// Create a new DateTime<Utc> generator.
    pub fn new(mode: DateTimeUtcGenerationMode) -> Self {
        Self {
            mode,
            reference: Utc::now(),
        }
    }

    /// Create a generator with a custom reference time.
    pub fn with_reference(mode: DateTimeUtcGenerationMode, reference: DateTime<Utc>) -> Self {
        Self { mode, reference }
    }

    /// Get the generation mode.
    pub fn mode(&self) -> DateTimeUtcGenerationMode {
        self.mode
    }

    /// Get the reference time.
    pub fn reference(&self) -> DateTime<Utc> {
        self.reference
    }
}

impl Generator for DateTimeUtcGenerator {
    type Target = DateTime<Utc>;

    fn generate(&self) -> Self::Target {
        match self.mode {
            DateTimeUtcGenerationMode::Now => Utc::now(),
            DateTimeUtcGenerationMode::UnixEpoch => DateTime::UNIX_EPOCH,
            DateTimeUtcGenerationMode::Offset { seconds } => {
                if seconds >= 0 {
                    self.reference + Duration::try_seconds(seconds).unwrap_or(Duration::zero())
                } else {
                    self.reference - Duration::try_seconds(-seconds).unwrap_or(Duration::zero())
                }
            }
        }
    }
}

// ============================================================================
// NaiveDateTime Generator
// ============================================================================

/// Generation mode for chrono::NaiveDateTime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NaiveDateTimeGenerationMode {
    /// Use current UTC time (without timezone).
    Now,
    /// Use Unix epoch (1970-01-01 00:00:00).
    UnixEpoch,
    /// Offset from reference time.
    Offset {
        /// Seconds offset.
        seconds: i64,
    },
}

impl Select for NaiveDateTimeGenerationMode {
    fn options() -> &'static [Self] {
        &[
            NaiveDateTimeGenerationMode::Now,
            NaiveDateTimeGenerationMode::UnixEpoch,
            NaiveDateTimeGenerationMode::Offset { seconds: 0 },
        ]
    }

    fn labels() -> &'static [&'static str] {
        &[
            "Now (Current time)",
            "Unix Epoch (1970-01-01)",
            "Offset (Custom)",
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Now (Current time)" => Some(NaiveDateTimeGenerationMode::Now),
            "Unix Epoch (1970-01-01)" => Some(NaiveDateTimeGenerationMode::UnixEpoch),
            "Offset (Custom)" => Some(NaiveDateTimeGenerationMode::Offset { seconds: 0 }),
            _ => None,
        }
    }
}

impl Prompt for NaiveDateTimeGenerationMode {
    fn prompt() -> Option<&'static str> {
        Some("How should naive datetimes be generated?")
    }
}

impl Elicitation for NaiveDateTimeGenerationMode {
    type Style = NaiveDateTimeGenerationModeStyle;

    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Select an option:"),
            Self::labels(),
        );

        let result = communicator
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
                "Invalid NaiveDateTime generation mode".to_string(),
            ))
        })?;

        match selected {
            NaiveDateTimeGenerationMode::Now => Ok(NaiveDateTimeGenerationMode::Now),
            NaiveDateTimeGenerationMode::UnixEpoch => Ok(NaiveDateTimeGenerationMode::UnixEpoch),
            NaiveDateTimeGenerationMode::Offset { .. } => {
                let seconds = i64::elicit(communicator).await?;
                Ok(NaiveDateTimeGenerationMode::Offset { seconds })
            }
        }
    }
}

/// Generator for creating NaiveDateTime values.
#[derive(Debug, Clone, Copy)]
pub struct NaiveDateTimeGenerator {
    mode: NaiveDateTimeGenerationMode,
    reference: NaiveDateTime,
}

impl NaiveDateTimeGenerator {
    /// Create a new NaiveDateTime generator.
    pub fn new(mode: NaiveDateTimeGenerationMode) -> Self {
        Self {
            mode,
            reference: Utc::now().naive_utc(),
        }
    }

    /// Create a generator with a custom reference time.
    pub fn with_reference(mode: NaiveDateTimeGenerationMode, reference: NaiveDateTime) -> Self {
        Self { mode, reference }
    }

    /// Get the generation mode.
    pub fn mode(&self) -> NaiveDateTimeGenerationMode {
        self.mode
    }

    /// Get the reference time.
    pub fn reference(&self) -> NaiveDateTime {
        self.reference
    }
}

impl Generator for NaiveDateTimeGenerator {
    type Target = NaiveDateTime;

    fn generate(&self) -> Self::Target {
        match self.mode {
            NaiveDateTimeGenerationMode::Now => Utc::now().naive_utc(),
            NaiveDateTimeGenerationMode::UnixEpoch => DateTime::UNIX_EPOCH.naive_utc(),
            NaiveDateTimeGenerationMode::Offset { seconds } => {
                if seconds >= 0 {
                    self.reference + Duration::try_seconds(seconds).unwrap_or(Duration::zero())
                } else {
                    self.reference - Duration::try_seconds(-seconds).unwrap_or(Duration::zero())
                }
            }
        }
    }
}

// ============================================================================
// DateTime<Utc> Elicitation
// ============================================================================

// DateTime<Utc> implementation
impl Prompt for DateTime<Utc> {
    fn prompt() -> Option<&'static str> {
        Some("Enter UTC datetime:")
    }
}

impl Elicitation for DateTime<Utc> {
    type Style = DateTimeUtcStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting DateTime<Utc>");

        // Step 1: Choose input method
        let method = DateTimeInputMethod::elicit(communicator).await?;
        tracing::debug!(?method, "Input method selected");

        match method {
            DateTimeInputMethod::Iso8601String => {
                // Elicit ISO 8601 string
                let prompt = "Enter ISO 8601 datetime (e.g., \"2024-07-11T15:30:00Z\"):";
                let params = mcp::text_params(prompt);
                let result = communicator
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
                DateTime::parse_from_rfc3339(&iso_string)
                    .map(|dt| dt.with_timezone(&Utc))
                    .map_err(|e| {
                        ElicitError::new(ElicitErrorKind::ParseError(format!(
                            "Invalid ISO 8601 datetime: {}",
                            e
                        )))
                    })
            }
            DateTimeInputMethod::ManualComponents => {
                // Elicit components
                let components = DateTimeComponents::elicit(communicator).await?;

                // Construct DateTime<Utc>
                Utc.with_ymd_and_hms(
                    components.year,
                    components.month as u32,
                    components.day as u32,
                    components.hour as u32,
                    components.minute as u32,
                    components.second as u32,
                )
                .single()
                .ok_or_else(|| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid datetime components: {}-{:02}-{:02} {:02}:{:02}:{:02}",
                        components.year,
                        components.month,
                        components.day,
                        components.hour,
                        components.minute,
                        components.second
                    )))
                })
            }
        }
    }
}

// DateTime<FixedOffset> implementation
impl Prompt for DateTime<FixedOffset> {
    fn prompt() -> Option<&'static str> {
        Some("Enter datetime with timezone offset:")
    }
}

impl Elicitation for DateTime<FixedOffset> {
    type Style = DateTimeFixedOffsetStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting DateTime<FixedOffset>");

        // Step 1: Choose input method
        let method = DateTimeInputMethod::elicit(communicator).await?;
        tracing::debug!(?method, "Input method selected");

        match method {
            DateTimeInputMethod::Iso8601String => {
                // Elicit ISO 8601 string
                let prompt =
                    "Enter ISO 8601 datetime with offset (e.g., \"2024-07-11T15:30:00+05:00\"):";
                let params = mcp::text_params(prompt);
                let result = communicator
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
                DateTime::parse_from_rfc3339(&iso_string).map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid ISO 8601 datetime: {}",
                        e
                    )))
                })
            }
            DateTimeInputMethod::ManualComponents => {
                // Elicit components
                let components = DateTimeComponents::elicit(communicator).await?;

                // Elicit offset
                let offset_prompt = "Enter timezone offset in hours (e.g., +5 or -8):";
                let offset_params = mcp::number_params(offset_prompt, -12, 14);
                let offset_result = communicator
                    .call_tool(rmcp::model::CallToolRequestParams {
                        meta: None,
                        name: mcp::tool_names::elicit_number().into(),
                        arguments: Some(offset_params),
                        task: None,
                    })
                    .await?;

                let offset_value = mcp::extract_value(offset_result)?;
                let offset_hours = mcp::parse_integer::<i64>(offset_value)? as i32;

                let offset = FixedOffset::east_opt(offset_hours * 3600).ok_or_else(|| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid timezone offset: {} hours",
                        offset_hours
                    )))
                })?;

                // Construct DateTime<FixedOffset>
                offset
                    .with_ymd_and_hms(
                        components.year,
                        components.month as u32,
                        components.day as u32,
                        components.hour as u32,
                        components.minute as u32,
                        components.second as u32,
                    )
                    .single()
                    .ok_or_else(|| {
                        ElicitError::new(ElicitErrorKind::ParseError(format!(
                            "Invalid datetime components: {}-{:02}-{:02} {:02}:{:02}:{:02}",
                            components.year,
                            components.month,
                            components.day,
                            components.hour,
                            components.minute,
                            components.second
                        )))
                    })
            }
        }
    }
}

// NaiveDateTime implementation
impl Prompt for NaiveDateTime {
    fn prompt() -> Option<&'static str> {
        Some("Enter datetime (no timezone):")
    }
}

impl Elicitation for NaiveDateTime {
    type Style = NaiveDateTimeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting NaiveDateTime");

        // Step 1: Choose input method
        let method = DateTimeInputMethod::elicit(communicator).await?;
        tracing::debug!(?method, "Input method selected");

        match method {
            DateTimeInputMethod::Iso8601String => {
                // Elicit ISO 8601 string (no timezone)
                let prompt = "Enter datetime (e.g., \"2024-07-11T15:30:00\"):";
                let params = mcp::text_params(prompt);
                let result = communicator
                    .call_tool(rmcp::model::CallToolRequestParams {
                        meta: None,
                        name: mcp::tool_names::elicit_text().into(),
                        arguments: Some(params),
                        task: None,
                    })
                    .await?;

                let value = mcp::extract_value(result)?;
                let iso_string = mcp::parse_string(value)?;

                // Parse ISO 8601 (naive)
                NaiveDateTime::parse_from_str(&iso_string, "%Y-%m-%dT%H:%M:%S").map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid datetime: {}",
                        e
                    )))
                })
            }
            DateTimeInputMethod::ManualComponents => {
                // Elicit components
                let components = DateTimeComponents::elicit(communicator).await?;

                // Construct NaiveDateTime
                chrono::NaiveDate::from_ymd_opt(
                    components.year,
                    components.month as u32,
                    components.day as u32,
                )
                .and_then(|date| {
                    date.and_hms_opt(
                        components.hour as u32,
                        components.minute as u32,
                        components.second as u32,
                    )
                })
                .ok_or_else(|| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid datetime components: {}-{:02}-{:02} {:02}:{:02}:{:02}",
                        components.year,
                        components.month,
                        components.day,
                        components.hour,
                        components.minute,
                        components.second
                    )))
                })
            }
        }
    }
}
