//! time datetime library elicitation implementations.
//!
//! Available with the `time` feature.
//!
//! This module provides `Elicitation` implementations for the modern, high
//! performance `time` crate. Supports both ISO 8601 string parsing and manual
//! component entry.
//!
//! # Supported Types
//!
//! - [`OffsetDateTime`] - Datetime with timezone offset
//! - [`PrimitiveDateTime`] - Datetime without timezone
//!
//! # Example
//!
//! ```rust,ignore
//! use time::OffsetDateTime;
//! use elicitation::Elicitation;
//! use rmcp::service::{Peer, RoleClient};
//!
//! async fn example(client: &Peer<RoleClient>) {
//!     // Elicit a datetime with offset
//!     let timestamp: OffsetDateTime = OffsetDateTime::elicit(client).await?;
//!     
//!     // User can choose:
//!     // 1. ISO 8601 string: "2024-07-11T15:30:00+05:00"
//!     // 2. Manual components: year, month, day, hour, minute, second, offset
//! }
//! ```
//!
//! # Elicitation Flow
//!
//! 1. **Input Method Selection** - User chooses ISO 8601 or manual components
//! 2. **Data Entry** - Based on selection:
//!    - ISO: Single string prompt with format validation
//!    - Manual: Six prompts for datetime + offset (for OffsetDateTime)
//! 3. **Validation** - time crate validates datetime construction
//! 4. **Result** - Returns validated datetime or error

use crate::{
    ElicitClient, ElicitError, ElicitErrorKind, ElicitResult, Elicitation, Generator, Prompt,
    Select,
    datetime_common::{DateTimeComponents, DateTimeInputMethod},
    mcp,
};
use std::time::{Duration, Instant};
use time::{OffsetDateTime, PrimitiveDateTime, UtcOffset};

// Style enums for time types
crate::default_style!(OffsetDateTime => OffsetDateTimeStyle);
crate::default_style!(PrimitiveDateTime => PrimitiveDateTimeStyle);
crate::default_style!(Instant => InstantStyle);

// ============================================================================
// Instant Generator
// ============================================================================

/// Generation mode for time::Instant.
///
/// This enum allows an agent (or user) to specify how to create an Instant:
/// - `Now`: Use the actual current instant
/// - `Offset`: Create a mock instant by offsetting from a reference point
///
/// This is particularly useful for test data generation where deterministic
/// or specific timing is needed.
#[derive(Debug, Clone, Copy)]
pub enum InstantGenerationMode {
    /// Use the actual current instant (Instant::now())
    Now,
    
    /// Create an instant offset from a reference point.
    ///
    /// The offset can be positive (future) or negative (past).
    Offset {
        /// Seconds offset from reference (negative = past, positive = future)
        seconds: i64,
        /// Additional nanoseconds (0-999,999,999)
        nanos: u32,
    },
}

// Manual implementation of Select pattern for InstantGenerationMode
crate::default_style!(InstantGenerationMode => InstantGenerationModeStyle);

impl Prompt for InstantGenerationMode {
    fn prompt() -> Option<&'static str> {
        Some("Choose how to generate the instant:")
    }
}

impl crate::Select for InstantGenerationMode {
    fn options() -> &'static [Self] {
        &[
            InstantGenerationMode::Now,
            InstantGenerationMode::Offset { seconds: 0, nanos: 0 },
        ]
    }

    fn labels() -> &'static [&'static str] {
        &["Now (current time)", "Offset (from reference)"]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Now (current time)" => Some(InstantGenerationMode::Now),
            "Offset (from reference)" => Some(InstantGenerationMode::Offset { seconds: 0, nanos: 0 }),
            _ => None,
        }
    }
}

impl Elicitation for InstantGenerationMode {
    type Style = InstantGenerationModeStyle;

    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
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

        let selected = Self::from_label(&label)
            .ok_or_else(|| ElicitError::new(ElicitErrorKind::ParseError(
                "Invalid variant selection".to_string()
            )))?;

        // If Offset was selected, elicit the fields
        match selected {
            InstantGenerationMode::Now => Ok(InstantGenerationMode::Now),
            InstantGenerationMode::Offset { .. } => {
                // Elicit seconds
                let seconds = i64::elicit(client).await?;
                // Elicit nanos
                let nanos = u32::elicit(client).await?;
                Ok(InstantGenerationMode::Offset { seconds, nanos })
            }
        }
    }
}

/// Generator for time::Instant.
///
/// Encapsulates a strategy for creating Instant values. Can be configured
/// once via elicitation and then used to generate multiple instants with
/// the same strategy.
///
/// # Example
///
/// ```rust,ignore
/// // Elicit the strategy
/// let mode = InstantGenerationMode::elicit(client).await?;
/// let generator = InstantGenerator::new(mode);
///
/// // Generate multiple instants with same strategy
/// let event1_time = generator.generate();
/// let event2_time = generator.generate();
/// ```
#[derive(Debug, Clone)]
pub struct InstantGenerator {
    mode: InstantGenerationMode,
    reference: Instant,
}

impl InstantGenerator {
    /// Create a new generator with the given mode.
    ///
    /// The reference instant is captured at creation time.
    pub fn new(mode: InstantGenerationMode) -> Self {
        Self {
            mode,
            reference: Instant::now(),
        }
    }

    /// Create a generator with a specific reference instant.
    ///
    /// Useful for tests where you want deterministic offsets from a known point.
    pub fn with_reference(mode: InstantGenerationMode, reference: Instant) -> Self {
        Self { mode, reference }
    }
}

impl Generator for InstantGenerator {
    type Target = Instant;

    fn generate(&self) -> Instant {
        match &self.mode {
            InstantGenerationMode::Now => Instant::now(),
            InstantGenerationMode::Offset { seconds, nanos } => {
                let duration = Duration::new(*seconds as u64, *nanos);
                
                // For offset mode, we use the reference instant
                if *seconds >= 0 {
                    self.reference + duration
                } else {
                    // Negative offset - subtract duration
                    self.reference - Duration::new((-*seconds) as u64, *nanos)
                }
            }
        }
    }
}

// ============================================================================
// Instant Elicitation
// ============================================================================

#[cfg_attr(not(kani), elicitation_macros::instrumented_impl)]
impl Prompt for Instant {
    fn prompt() -> Option<&'static str> {
        Some("Specify how to create an instant (now vs offset):")
    }
}

#[cfg_attr(not(kani), elicitation_macros::instrumented_impl)]
impl Elicitation for Instant {
    type Style = InstantStyle;

    #[tracing::instrument(skip(client), fields(type_name = "Instant"))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting time::Instant");

        // Elicit the generation mode
        let mode = InstantGenerationMode::elicit(client).await?;

        // Create generator and generate immediately
        let generator = InstantGenerator::new(mode);
        Ok(generator.generate())
    }
}

// ============================================================================
// OffsetDateTime Elicitation  
// ============================================================================

// OffsetDateTime implementation
impl Prompt for OffsetDateTime {
    fn prompt() -> Option<&'static str> {
        Some("Enter datetime with timezone offset:")
    }
}

impl Elicitation for OffsetDateTime {
    type Style = OffsetDateTimeStyle;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting OffsetDateTime");

        // Step 1: Choose input method
        let method = DateTimeInputMethod::elicit(client).await?;
        tracing::debug!(?method, "Input method selected");

        match method {
            DateTimeInputMethod::Iso8601String => {
                // Elicit ISO 8601 string
                let prompt =
                    "Enter ISO 8601 datetime with offset (e.g., \"2024-07-11T15:30:00+05:00\"):";
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
                OffsetDateTime::parse(&iso_string, &time::format_description::well_known::Rfc3339)
                    .map_err(|e| {
                        ElicitError::new(ElicitErrorKind::ParseError(format!(
                            "Invalid ISO 8601 datetime: {}",
                            e
                        )))
                    })
            }
            DateTimeInputMethod::ManualComponents => {
                // Elicit components
                let components = DateTimeComponents::elicit(client).await?;

                // Elicit offset
                let offset_prompt = "Enter timezone offset in hours (e.g., +5 or -8):";
                let offset_params = mcp::number_params(offset_prompt, -12, 14);
                let offset_result = client
                    .peer()
                    .call_tool(rmcp::model::CallToolRequestParams {
                        meta: None,
                        name: mcp::tool_names::elicit_number().into(),
                        arguments: Some(offset_params),
                        task: None,
                    })
                    .await?;

                let offset_value = mcp::extract_value(offset_result)?;
                let offset_hours = mcp::parse_integer::<i64>(offset_value)? as i32;

                let offset = UtcOffset::from_hms(offset_hours as i8, 0, 0).map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid timezone offset: {}",
                        e
                    )))
                })?;

                // Construct PrimitiveDateTime first
                let date = time::Date::from_calendar_date(
                    components.year,
                    time::Month::try_from(components.month).map_err(|e| {
                        ElicitError::new(ElicitErrorKind::ParseError(format!(
                            "Invalid month: {}",
                            e
                        )))
                    })?,
                    components.day,
                )
                .map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid date: {}", e)))
                })?;

                let time =
                    time::Time::from_hms(components.hour, components.minute, components.second)
                        .map_err(|e| {
                            ElicitError::new(ElicitErrorKind::ParseError(format!(
                                "Invalid time: {}",
                                e
                            )))
                        })?;

                Ok(PrimitiveDateTime::new(date, time).assume_offset(offset))
            }
        }
    }
}

// PrimitiveDateTime implementation
impl Prompt for PrimitiveDateTime {
    fn prompt() -> Option<&'static str> {
        Some("Enter datetime (no timezone):")
    }
}

impl Elicitation for PrimitiveDateTime {
    type Style = PrimitiveDateTimeStyle;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting PrimitiveDateTime");

        // Step 1: Choose input method
        let method = DateTimeInputMethod::elicit(client).await?;
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

                // Parse ISO 8601 (primitive)
                PrimitiveDateTime::parse(
                    &iso_string,
                    &time::format_description::well_known::Rfc3339,
                )
                .map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid datetime: {}",
                        e
                    )))
                })
            }
            DateTimeInputMethod::ManualComponents => {
                // Elicit components
                let components = DateTimeComponents::elicit(client).await?;

                // Construct PrimitiveDateTime
                let date = time::Date::from_calendar_date(
                    components.year,
                    time::Month::try_from(components.month).map_err(|e| {
                        ElicitError::new(ElicitErrorKind::ParseError(format!(
                            "Invalid month: {}",
                            e
                        )))
                    })?,
                    components.day,
                )
                .map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid date: {}", e)))
                })?;

                let time =
                    time::Time::from_hms(components.hour, components.minute, components.second)
                        .map_err(|e| {
                            ElicitError::new(ElicitErrorKind::ParseError(format!(
                                "Invalid time: {}",
                                e
                            )))
                        })?;

                Ok(PrimitiveDateTime::new(date, time))
            }
        }
    }
}
