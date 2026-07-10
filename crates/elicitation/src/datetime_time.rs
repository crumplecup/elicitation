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
//! - [`time::Time`] - Wall-clock time of day
//!
//! # Example
//!
//! ```rust,ignore
//! use time::OffsetDateTime;
//! use elicitation::Elicitation;
//! use rmcp::service::{Peer, RoleClient};
//!
//! async fn example(communicator: &Peer<RoleClient>) {
//!     // Elicit a datetime with offset
//!     let timestamp: OffsetDateTime = OffsetDateTime::elicit(communicator).await?;
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
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitPromptTree,
    ElicitResult, Elicitation, ElicitationPattern, FieldInfo, Generator, PatternDetails, Prompt,
    PromptTree, Select, TypeMetadata, VariantMetadata,
    datetime_common::{DateTimeComponents, DateTimeInputMethod},
    mcp,
};
use std::time::{Duration, Instant};
use time::{Date, OffsetDateTime, PrimitiveDateTime, Time, UtcDateTime, UtcOffset};

// Style enums for time types
crate::default_style!(OffsetDateTime => OffsetDateTimeStyle);
crate::default_style!(PrimitiveDateTime => PrimitiveDateTimeStyle);
crate::default_style!(Instant => InstantStyle);
crate::default_style!(Time => TimeStyle);
crate::default_style!(Date => DateStyle);
crate::default_style!(time::Duration => TimeDurationStyle);
crate::default_style!(time::Month => TimeMonthStyle);
crate::default_style!(time::Weekday => TimeWeekdayStyle);
crate::default_style!(UtcDateTime => UtcDateTimeStyle);
crate::default_style!(UtcOffset => UtcOffsetStyle);
crate::default_style!(time::error::ComponentRange => ComponentRangeStyle);
crate::default_style!(time::error::ConversionRange => ConversionRangeStyle);
crate::default_style!(time::error::DifferentVariant => DifferentVariantStyle);
crate::default_style!(time::format_description::Component => FmtComponentStyle);
crate::default_style!(time::format_description::modifier::Padding => FmtPaddingStyle);
crate::default_style!(time::format_description::modifier::Day => FmtDayStyle);
crate::default_style!(time::format_description::modifier::End => FmtEndStyle);
crate::default_style!(time::format_description::modifier::Ignore => FmtIgnoreStyle);
crate::default_style!(time::format_description::modifier::Minute => FmtMinuteStyle);
crate::default_style!(time::format_description::modifier::TrailingInput => FmtTrailingInputStyle);
crate::default_style!(OffsetDateTimeGenerationMode => OffsetDateTimeGenerationModeStyle);
crate::default_style!(PrimitiveDateTimeGenerationMode => PrimitiveDateTimeGenerationModeStyle);

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
    fn options() -> Vec<Self> {
        vec![
            InstantGenerationMode::Now,
            InstantGenerationMode::Offset {
                seconds: 0,
                nanos: 0,
            },
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Now (current time)".to_string(),
            "Offset (from reference)".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Now (current time)" => Some(InstantGenerationMode::Now),
            "Offset (from reference)" => Some(InstantGenerationMode::Offset {
                seconds: 0,
                nanos: 0,
            }),
            _ => None,
        }
    }
}

impl Elicitation for InstantGenerationMode {
    type Style = InstantGenerationModeStyle;

    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "value",
                "InstantGenerationMode",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| {
                Self::prompt()
                    .unwrap_or("Choose instant generation mode:")
                    .to_string()
            });
        let params = mcp::select_params(&prompt, &Self::labels());

        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;

        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;

        let selected = Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(
                "Invalid variant selection".to_string(),
            ))
        })?;

        // If Offset was selected, elicit the fields
        match selected {
            InstantGenerationMode::Now => Ok(InstantGenerationMode::Now),
            InstantGenerationMode::Offset { .. } => {
                // Elicit seconds
                let seconds = i64::elicit(communicator).await?;
                // Elicit nanos
                let nanos = u32::elicit(communicator).await?;
                Ok(InstantGenerationMode::Offset { seconds, nanos })
            }
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }
}

impl ElicitIntrospect for InstantGenerationMode {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "InstantGenerationMode",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
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
/// let mode = InstantGenerationMode::elicit(communicator).await?;
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

#[cfg_attr(not(kani), elicitation_derive::instrumented_impl)]
impl Prompt for Instant {
    fn prompt() -> Option<&'static str> {
        Some("Specify how to create an instant (now vs offset):")
    }
}

#[cfg_attr(not(kani), elicitation_derive::instrumented_impl)]
impl Elicitation for Instant {
    type Style = InstantStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "Instant"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        // Elicit the generation mode
        let mode = InstantGenerationMode::elicit(communicator).await?;

        // Create generator and generate immediately
        let generator = InstantGenerator::new(mode);
        Ok(generator.generate())
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }
}

impl ElicitIntrospect for Instant {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::Instant",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

// ============================================================================
// OffsetDateTime Generator
// ============================================================================

/// Generation mode for time::OffsetDateTime.
///
/// This enum allows an agent (or user) to specify how to create an OffsetDateTime:
/// - `Now`: Current UTC time
/// - `UnixEpoch`: Unix epoch (1970-01-01 00:00:00 UTC)
/// - `Offset`: Time offset from a reference point
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OffsetDateTimeGenerationMode {
    /// Use current UTC time.
    Now,
    /// Use Unix epoch (1970-01-01 00:00:00 UTC).
    UnixEpoch,
    /// Offset from reference time.
    Offset {
        /// Seconds offset (positive = future, negative = past).
        seconds: i64,
        /// Nanoseconds component (0-999,999,999).
        nanos: i32,
    },
}

impl Select for OffsetDateTimeGenerationMode {
    fn options() -> Vec<Self> {
        vec![
            OffsetDateTimeGenerationMode::Now,
            OffsetDateTimeGenerationMode::UnixEpoch,
            OffsetDateTimeGenerationMode::Offset {
                seconds: 0,
                nanos: 0,
            },
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Now (Current UTC)".to_string(),
            "Unix Epoch (1970-01-01)".to_string(),
            "Offset (Custom)".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Now (Current UTC)" => Some(OffsetDateTimeGenerationMode::Now),
            "Unix Epoch (1970-01-01)" => Some(OffsetDateTimeGenerationMode::UnixEpoch),
            "Offset (Custom)" => Some(OffsetDateTimeGenerationMode::Offset {
                seconds: 0,
                nanos: 0,
            }),
            _ => None,
        }
    }
}

impl Prompt for OffsetDateTimeGenerationMode {
    fn prompt() -> Option<&'static str> {
        Some("How should OffsetDateTime values be generated?")
    }
}

impl Elicitation for OffsetDateTimeGenerationMode {
    type Style = OffsetDateTimeGenerationModeStyle;

    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "value",
                "OffsetDateTimeGenerationMode",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| {
                Self::prompt()
                    .unwrap_or("Choose OffsetDateTime generation mode:")
                    .to_string()
            });
        let params = mcp::select_params(&prompt, &Self::labels());

        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;

        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;

        let selected = Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(
                "Invalid OffsetDateTime generation mode".to_string(),
            ))
        })?;

        match selected {
            OffsetDateTimeGenerationMode::Now => Ok(OffsetDateTimeGenerationMode::Now),
            OffsetDateTimeGenerationMode::UnixEpoch => Ok(OffsetDateTimeGenerationMode::UnixEpoch),
            OffsetDateTimeGenerationMode::Offset { .. } => {
                let seconds = i64::elicit(communicator).await?;
                let nanos = i32::elicit(communicator).await?;
                Ok(OffsetDateTimeGenerationMode::Offset { seconds, nanos })
            }
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }
}

impl ElicitIntrospect for OffsetDateTimeGenerationMode {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "OffsetDateTimeGenerationMode",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

/// Generator for creating OffsetDateTime values with a specified strategy.
#[derive(Debug, Clone, Copy)]
pub struct OffsetDateTimeGenerator {
    mode: OffsetDateTimeGenerationMode,
    reference: OffsetDateTime,
}

impl OffsetDateTimeGenerator {
    /// Create a new OffsetDateTime generator with the specified mode.
    pub fn new(mode: OffsetDateTimeGenerationMode) -> Self {
        Self {
            mode,
            reference: OffsetDateTime::now_utc(),
        }
    }

    /// Create a generator with a custom reference time.
    pub fn with_reference(mode: OffsetDateTimeGenerationMode, reference: OffsetDateTime) -> Self {
        Self { mode, reference }
    }

    /// Get the generation mode.
    pub fn mode(&self) -> OffsetDateTimeGenerationMode {
        self.mode
    }

    /// Get the reference time.
    pub fn reference(&self) -> OffsetDateTime {
        self.reference
    }
}

impl Generator for OffsetDateTimeGenerator {
    type Target = OffsetDateTime;

    fn generate(&self) -> Self::Target {
        match self.mode {
            OffsetDateTimeGenerationMode::Now => OffsetDateTime::now_utc(),
            OffsetDateTimeGenerationMode::UnixEpoch => OffsetDateTime::UNIX_EPOCH,
            OffsetDateTimeGenerationMode::Offset { seconds, nanos } => {
                if seconds >= 0 {
                    self.reference + Duration::new(seconds as u64, nanos as u32)
                } else {
                    self.reference - Duration::new((-seconds) as u64, nanos.unsigned_abs())
                }
            }
        }
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

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting OffsetDateTime");

        // Step 1: Choose input method
        let method = DateTimeInputMethod::elicit(communicator).await?;
        tracing::debug!(?method, "Input method selected");

        match method {
            DateTimeInputMethod::Iso8601String => {
                let prompt = communicator
                    .style_context()
                    .prompt_for_type::<Self>(
                        "iso8601",
                        "String",
                        &crate::style::PromptContext::new(0, 2),
                    )?
                    .unwrap_or_else(|| {
                        "Enter ISO 8601 datetime with offset (e.g., \"2024-07-11T15:30:00+05:00\"):"
                            .to_string()
                    });
                let params = mcp::text_params(&prompt);
                let result = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                            .with_arguments(params),
                    )
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
                let components = DateTimeComponents::elicit(communicator).await?;

                let offset_prompt = communicator
                    .style_context()
                    .prompt_for_type::<Self>(
                        "offset_hours",
                        "i32",
                        &crate::style::PromptContext::new(1, 2),
                    )?
                    .unwrap_or_else(|| {
                        "Enter timezone offset in hours (e.g., +5 or -8):".to_string()
                    });
                let offset_params = mcp::number_params(&offset_prompt, -12, 14);
                let offset_result = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                            .with_arguments(offset_params),
                    )
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

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("time::OffsetDateTime")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("time::OffsetDateTime")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("time::OffsetDateTime")
    }
}

impl ElicitIntrospect for OffsetDateTime {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::OffsetDateTime",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
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

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting PrimitiveDateTime");

        // Step 1: Choose input method
        let method = DateTimeInputMethod::elicit(communicator).await?;
        tracing::debug!(?method, "Input method selected");

        match method {
            DateTimeInputMethod::Iso8601String => {
                let prompt = communicator
                    .style_context()
                    .prompt_for_type::<Self>(
                        "iso8601",
                        "String",
                        &crate::style::PromptContext::new(0, 1),
                    )?
                    .unwrap_or_else(|| {
                        "Enter datetime (e.g., \"2024-07-11T15:30:00\"):".to_string()
                    });
                let params = mcp::text_params(&prompt);
                let result = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                            .with_arguments(params),
                    )
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
                let components = DateTimeComponents::elicit(communicator).await?;

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

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("time::PrimitiveDateTime")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("time::PrimitiveDateTime")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("time::PrimitiveDateTime")
    }
}

impl ElicitIntrospect for PrimitiveDateTime {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::PrimitiveDateTime",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

// time::Time implementation
impl Prompt for Time {
    fn prompt() -> Option<&'static str> {
        Some("Enter time of day (hour, minute, second):")
    }
}

impl Elicitation for Time {
    type Style = TimeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting time::Time");

        let hour_prompt = communicator
            .style_context()
            .prompt_for_type::<Self>("hour", "u8", &crate::style::PromptContext::new(0, 3))?
            .unwrap_or_else(|| "Enter hour (0-23):".to_string());
        let hour_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(mcp::number_params(&hour_prompt, 0, 23)),
            )
            .await?;
        let hour = mcp::parse_integer::<i64>(mcp::extract_value(hour_result)?)? as u8;

        let minute_prompt = communicator
            .style_context()
            .prompt_for_type::<Self>("minute", "u8", &crate::style::PromptContext::new(1, 3))?
            .unwrap_or_else(|| "Enter minute (0-59):".to_string());
        let minute_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(mcp::number_params(&minute_prompt, 0, 59)),
            )
            .await?;
        let minute = mcp::parse_integer::<i64>(mcp::extract_value(minute_result)?)? as u8;

        let second_prompt = communicator
            .style_context()
            .prompt_for_type::<Self>("second", "u8", &crate::style::PromptContext::new(2, 3))?
            .unwrap_or_else(|| "Enter second (0-59):".to_string());
        let second_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(mcp::number_params(&second_prompt, 0, 59)),
            )
            .await?;
        let second = mcp::parse_integer::<i64>(mcp::extract_value(second_result)?)? as u8;

        Time::from_hms(hour, minute, second).map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid time: {e}")))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("time::Time")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("time::Time")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("time::Time")
    }
}

impl ElicitIntrospect for Time {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::Time",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

// time::Date implementation
impl Prompt for Date {
    fn prompt() -> Option<&'static str> {
        Some("Enter calendar date (year, month, day):")
    }
}

impl Elicitation for Date {
    type Style = DateStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting time::Date");

        let year_prompt = communicator
            .style_context()
            .prompt_for_type::<Self>("year", "i32", &crate::style::PromptContext::new(0, 3))?
            .unwrap_or_else(|| "Enter year:".to_string());
        let year_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(mcp::number_params(&year_prompt, -9999, 9999)),
            )
            .await?;
        let year = mcp::parse_integer::<i64>(mcp::extract_value(year_result)?)? as i32;

        let month_prompt = communicator
            .style_context()
            .prompt_for_type::<Self>("month", "u8", &crate::style::PromptContext::new(1, 3))?
            .unwrap_or_else(|| "Enter month (1-12):".to_string());
        let month_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(mcp::number_params(&month_prompt, 1, 12)),
            )
            .await?;
        let month_num = mcp::parse_integer::<i64>(mcp::extract_value(month_result)?)? as u8;
        let month = time::Month::try_from(month_num).map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid month: {e}")))
        })?;

        let day_prompt = communicator
            .style_context()
            .prompt_for_type::<Self>("day", "u8", &crate::style::PromptContext::new(2, 3))?
            .unwrap_or_else(|| "Enter day (1-31):".to_string());
        let day_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(mcp::number_params(&day_prompt, 1, 31)),
            )
            .await?;
        let day = mcp::parse_integer::<i64>(mcp::extract_value(day_result)?)? as u8;

        Date::from_calendar_date(year, month, day).map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid date: {e}")))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("time::Date")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("time::Date")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("time::Date")
    }
}

impl ElicitIntrospect for Date {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::Date",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

// time::Duration implementation
impl Prompt for time::Duration {
    fn prompt() -> Option<&'static str> {
        Some("Enter duration (whole seconds and subsecond nanoseconds):")
    }
}

impl Elicitation for time::Duration {
    type Style = TimeDurationStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting time::Duration");

        let seconds_prompt = communicator
            .style_context()
            .prompt_for_type::<Self>("seconds", "i64", &crate::style::PromptContext::new(0, 2))?
            .unwrap_or_else(|| "Enter whole seconds (may be negative):".to_string());
        let seconds_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(mcp::number_params(&seconds_prompt, i64::MIN, i64::MAX)),
            )
            .await?;
        let seconds = mcp::parse_integer::<i64>(mcp::extract_value(seconds_result)?)?;

        let nanos_prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "nanoseconds",
                "i32",
                &crate::style::PromptContext::new(1, 2),
            )?
            .unwrap_or_else(|| "Enter subsecond nanoseconds (0–999999999):".to_string());
        let nanos_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(mcp::number_params(&nanos_prompt, 0, 999_999_999)),
            )
            .await?;
        let nanoseconds = mcp::parse_integer::<i64>(mcp::extract_value(nanos_result)?)? as i32;

        Ok(time::Duration::new(seconds, nanoseconds))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("time::Duration")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("time::Duration")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("time::Duration")
    }
}

impl ElicitIntrospect for time::Duration {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::Duration",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

// time::Month implementation
impl Prompt for time::Month {
    fn prompt() -> Option<&'static str> {
        Some("Choose a month:")
    }
}

impl Select for time::Month {
    fn options() -> Vec<Self> {
        vec![
            time::Month::January,
            time::Month::February,
            time::Month::March,
            time::Month::April,
            time::Month::May,
            time::Month::June,
            time::Month::July,
            time::Month::August,
            time::Month::September,
            time::Month::October,
            time::Month::November,
            time::Month::December,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "January".to_string(),
            "February".to_string(),
            "March".to_string(),
            "April".to_string(),
            "May".to_string(),
            "June".to_string(),
            "July".to_string(),
            "August".to_string(),
            "September".to_string(),
            "October".to_string(),
            "November".to_string(),
            "December".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "January" => Some(time::Month::January),
            "February" => Some(time::Month::February),
            "March" => Some(time::Month::March),
            "April" => Some(time::Month::April),
            "May" => Some(time::Month::May),
            "June" => Some(time::Month::June),
            "July" => Some(time::Month::July),
            "August" => Some(time::Month::August),
            "September" => Some(time::Month::September),
            "October" => Some(time::Month::October),
            "November" => Some(time::Month::November),
            "December" => Some(time::Month::December),
            _ => None,
        }
    }
}

impl Elicitation for time::Month {
    type Style = TimeMonthStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting time::Month");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "value",
                "time::Month",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| Self::prompt().unwrap_or("Choose a month:").to_string());
        let params = mcp::select_params(&prompt, &Self::labels());
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid Month: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("time::Month", "January")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("time::Month", "January")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("time::Month", "January")
    }
}

impl ElicitIntrospect for time::Month {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::Month",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

impl ElicitPromptTree for time::Month {
    fn prompt_tree() -> PromptTree {
        let labels = Self::labels();
        let count = labels.len();
        PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Choose a month:").to_string(),
            type_name: "time::Month".to_string(),
            options: labels,
            branches: vec![None; count],
        }
    }
}

// time::Weekday implementation
impl Prompt for time::Weekday {
    fn prompt() -> Option<&'static str> {
        Some("Choose a day of the week:")
    }
}

impl Select for time::Weekday {
    fn options() -> Vec<Self> {
        vec![
            time::Weekday::Monday,
            time::Weekday::Tuesday,
            time::Weekday::Wednesday,
            time::Weekday::Thursday,
            time::Weekday::Friday,
            time::Weekday::Saturday,
            time::Weekday::Sunday,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Monday".to_string(),
            "Tuesday".to_string(),
            "Wednesday".to_string(),
            "Thursday".to_string(),
            "Friday".to_string(),
            "Saturday".to_string(),
            "Sunday".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Monday" => Some(time::Weekday::Monday),
            "Tuesday" => Some(time::Weekday::Tuesday),
            "Wednesday" => Some(time::Weekday::Wednesday),
            "Thursday" => Some(time::Weekday::Thursday),
            "Friday" => Some(time::Weekday::Friday),
            "Saturday" => Some(time::Weekday::Saturday),
            "Sunday" => Some(time::Weekday::Sunday),
            _ => None,
        }
    }
}

impl Elicitation for time::Weekday {
    type Style = TimeWeekdayStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting time::Weekday");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "value",
                "time::Weekday",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| {
                Self::prompt()
                    .unwrap_or("Choose a day of the week:")
                    .to_string()
            });
        let params = mcp::select_params(&prompt, &Self::labels());
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid Weekday: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("time::Weekday", "Monday")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("time::Weekday", "Monday")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("time::Weekday", "Monday")
    }
}

impl ElicitIntrospect for time::Weekday {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::Weekday",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

impl ElicitPromptTree for time::Weekday {
    fn prompt_tree() -> PromptTree {
        let labels = Self::labels();
        let count = labels.len();
        PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose a day of the week:")
                .to_string(),
            type_name: "time::Weekday".to_string(),
            options: labels,
            branches: vec![None; count],
        }
    }
}

// time::UtcDateTime implementation
impl Prompt for UtcDateTime {
    fn prompt() -> Option<&'static str> {
        Some("Enter UTC datetime (year, month, day, hour, minute, second):")
    }
}

impl Elicitation for UtcDateTime {
    type Style = UtcDateTimeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let components = DateTimeComponents::elicit(communicator).await?;

        let date = time::Date::from_calendar_date(
            components.year,
            time::Month::try_from(components.month).map_err(|e| {
                ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid month: {e}")))
            })?,
            components.day,
        )
        .map_err(|e| ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid date: {e}"))))?;

        let time = time::Time::from_hms(components.hour, components.minute, components.second)
            .map_err(|e| {
                ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid time: {e}")))
            })?;

        Ok(UtcDateTime::new(date, time))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("time::UtcDateTime")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("time::UtcDateTime")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("time::UtcDateTime")
    }
}

impl ElicitIntrospect for UtcDateTime {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::UtcDateTime",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

// time::UtcOffset implementation
impl Prompt for UtcOffset {
    fn prompt() -> Option<&'static str> {
        Some("Enter UTC offset in whole seconds (e.g., 3600 for +01:00, -7200 for -02:00):")
    }
}

impl Elicitation for UtcOffset {
    type Style = UtcOffsetStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting UtcOffset");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>("seconds", "i32", &crate::style::PromptContext::new(0, 1))?
            .unwrap_or_else(|| "Enter UTC offset in whole seconds (-86399 to 86399):".to_string());
        let params = mcp::number_params(&prompt, -86_399, 86_399);
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(params),
            )
            .await?;
        let seconds = mcp::parse_integer::<i64>(mcp::extract_value(result)?)? as i32;
        UtcOffset::from_whole_seconds(seconds).map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid UTC offset: {e}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("time::UtcOffset")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("time::UtcOffset")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("time::UtcOffset")
    }
}

impl ElicitIntrospect for UtcOffset {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::UtcOffset",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

// time::error::ComponentRange implementation
impl Prompt for time::error::ComponentRange {
    fn prompt() -> Option<&'static str> {
        Some("Choose a time component range error:")
    }
}

impl Select for time::error::ComponentRange {
    fn options() -> Vec<Self> {
        Self::labels()
            .into_iter()
            .filter_map(|l| Self::from_label(&l))
            .collect()
    }

    fn labels() -> Vec<String> {
        vec![
            "hour".to_string(),
            "minute".to_string(),
            "second".to_string(),
            "nanosecond".to_string(),
            "month".to_string(),
            "offset".to_string(),
            "day".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "hour" => time::Time::from_hms(24, 0, 0).err(),
            "minute" => time::Time::from_hms(0, 60, 0).err(),
            "second" => time::Time::from_hms(0, 0, 60).err(),
            "nanosecond" => time::Time::from_hms_nano(0, 0, 0, 1_000_000_000).err(),
            "month" => time::Month::try_from(13u8).err(),
            "offset" => time::UtcOffset::from_whole_seconds(86400).err(),
            "day" => time::Date::from_calendar_date(2024, time::Month::February, 30).err(),
            _ => None,
        }
    }
}

impl Elicitation for time::error::ComponentRange {
    type Style = ComponentRangeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting time::error::ComponentRange");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "value",
                "time::error::ComponentRange",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| {
                Self::prompt()
                    .unwrap_or("Choose a time component range error:")
                    .to_string()
            });
        let params = mcp::select_params(&prompt, &Self::labels());
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid ComponentRange component: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("time::error::ComponentRange")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("time::error::ComponentRange")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("time::error::ComponentRange")
    }
}

impl ElicitIntrospect for time::error::ComponentRange {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::error::ComponentRange",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

impl ElicitPromptTree for time::error::ComponentRange {
    fn prompt_tree() -> PromptTree {
        let labels = Self::labels();
        let count = labels.len();
        PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose a time component range error:")
                .to_string(),
            type_name: "time::error::ComponentRange".to_string(),
            options: labels,
            branches: vec![None; count],
        }
    }
}

// time::error::ConversionRange implementation
impl Prompt for time::error::ConversionRange {
    fn prompt() -> Option<&'static str> {
        Some("time::error::ConversionRange (conversion out of range — single value)")
    }
}

impl Elicitation for time::error::ConversionRange {
    type Style = ConversionRangeStyle;

    #[tracing::instrument(skip_all)]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        // Single-value type: trigger the one possible error state.
        time::Duration::try_from(std::time::Duration::new(u64::MAX, 0))
            .err()
            .ok_or_else(|| {
                ElicitError::new(ElicitErrorKind::ParseError(
                    "expected ConversionRange error was not produced".to_string(),
                ))
            })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("time::error::ConversionRange")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("time::error::ConversionRange")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("time::error::ConversionRange")
    }
}

impl ElicitIntrospect for time::error::ConversionRange {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::error::ConversionRange",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

// time::error::DifferentVariant implementation
impl Prompt for time::error::DifferentVariant {
    fn prompt() -> Option<&'static str> {
        Some("time::error::DifferentVariant (wrong format-description variant — single value)")
    }
}

impl Elicitation for time::error::DifferentVariant {
    type Style = DifferentVariantStyle;

    #[tracing::instrument(skip_all)]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        // Single-value unit struct: construct directly.
        Ok(time::error::DifferentVariant)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("time::error::DifferentVariant")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("time::error::DifferentVariant")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("time::error::DifferentVariant")
    }
}

impl ElicitIntrospect for time::error::DifferentVariant {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::error::DifferentVariant",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

// time::format_description::Component implementation
impl Prompt for time::format_description::Component {
    fn prompt() -> Option<&'static str> {
        Some("Choose a format description component:")
    }
}

impl Select for time::format_description::Component {
    fn options() -> Vec<Self> {
        use time::format_description::{Component, modifier};
        vec![
            Component::Day(modifier::Day::default()),
            Component::Ordinal(modifier::Ordinal::default()),
            Component::Hour12(modifier::Hour12::default()),
            Component::Hour24(modifier::Hour24::default()),
            Component::Minute(modifier::Minute::default()),
            Component::Period(modifier::Period::default()),
            Component::Second(modifier::Second::default()),
            Component::Subsecond(modifier::Subsecond::default()),
            Component::OffsetHour(modifier::OffsetHour::default()),
            Component::OffsetMinute(modifier::OffsetMinute::default()),
            Component::OffsetSecond(modifier::OffsetSecond::default()),
            Component::Ignore(modifier::Ignore::count(core::num::NonZero::<u16>::MIN)),
            Component::End(modifier::End::default()),
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Day".to_string(),
            "Ordinal".to_string(),
            "Hour12".to_string(),
            "Hour24".to_string(),
            "Minute".to_string(),
            "Period".to_string(),
            "Second".to_string(),
            "Subsecond".to_string(),
            "OffsetHour".to_string(),
            "OffsetMinute".to_string(),
            "OffsetSecond".to_string(),
            "Ignore".to_string(),
            "End".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        use time::format_description::{Component, modifier};
        match label {
            "Day" => Some(Component::Day(modifier::Day::default())),
            "Ordinal" => Some(Component::Ordinal(modifier::Ordinal::default())),
            "Hour12" => Some(Component::Hour12(modifier::Hour12::default())),
            "Hour24" => Some(Component::Hour24(modifier::Hour24::default())),
            "Minute" => Some(Component::Minute(modifier::Minute::default())),
            "Period" => Some(Component::Period(modifier::Period::default())),
            "Second" => Some(Component::Second(modifier::Second::default())),
            "Subsecond" => Some(Component::Subsecond(modifier::Subsecond::default())),
            "OffsetHour" => Some(Component::OffsetHour(modifier::OffsetHour::default())),
            "OffsetMinute" => Some(Component::OffsetMinute(modifier::OffsetMinute::default())),
            "OffsetSecond" => Some(Component::OffsetSecond(modifier::OffsetSecond::default())),
            "Ignore" => Some(Component::Ignore(modifier::Ignore::count(
                core::num::NonZero::<u16>::MIN,
            ))),
            "End" => Some(Component::End(modifier::End::default())),
            _ => None,
        }
    }
}

impl Elicitation for time::format_description::Component {
    type Style = FmtComponentStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting time::format_description::Component");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "value",
                "time::format_description::Component",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| {
                Self::prompt()
                    .unwrap_or("Choose a format description component:")
                    .to_string()
            });
        let params = mcp::select_params(&prompt, &Self::labels());
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid Component variant: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "time::format_description::Component",
            "Day",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "time::format_description::Component",
            "Day",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "time::format_description::Component",
            "Day",
        )
    }
}

impl ElicitIntrospect for time::format_description::Component {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::format_description::Component",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

impl ElicitPromptTree for time::format_description::Component {
    fn prompt_tree() -> PromptTree {
        let labels = Self::labels();
        let count = labels.len();
        PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose a format description component:")
                .to_string(),
            type_name: "time::format_description::Component".to_string(),
            options: labels,
            branches: vec![None; count],
        }
    }
}

// time::format_description::modifier::Padding implementation
impl Prompt for time::format_description::modifier::Padding {
    fn prompt() -> Option<&'static str> {
        Some("Choose a padding type:")
    }
}

impl Select for time::format_description::modifier::Padding {
    fn options() -> Vec<Self> {
        use time::format_description::modifier::Padding;
        vec![Padding::Space, Padding::Zero, Padding::None]
    }

    fn labels() -> Vec<String> {
        vec!["Space".to_string(), "Zero".to_string(), "None".to_string()]
    }

    fn from_label(label: &str) -> Option<Self> {
        use time::format_description::modifier::Padding;
        match label {
            "Space" => Some(Padding::Space),
            "Zero" => Some(Padding::Zero),
            "None" => Some(Padding::None),
            _ => None,
        }
    }
}

impl Elicitation for time::format_description::modifier::Padding {
    type Style = FmtPaddingStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting time::format_description::modifier::Padding");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "value",
                "time::format_description::modifier::Padding",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| {
                Self::prompt()
                    .unwrap_or("Choose a padding type:")
                    .to_string()
            });
        let params = mcp::select_params(&prompt, &Self::labels());
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid Padding: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "time::format_description::modifier::Padding",
            "Space",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "time::format_description::modifier::Padding",
            "Space",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "time::format_description::modifier::Padding",
            "Space",
        )
    }
}

impl ElicitIntrospect for time::format_description::modifier::Padding {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::format_description::modifier::Padding",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

impl ElicitPromptTree for time::format_description::modifier::Padding {
    fn prompt_tree() -> PromptTree {
        let labels = Self::labels();
        let count = labels.len();
        PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose a padding type:")
                .to_string(),
            type_name: "time::format_description::modifier::Padding".to_string(),
            options: labels,
            branches: vec![None; count],
        }
    }
}

// time::format_description::modifier::Day implementation
impl Prompt for time::format_description::modifier::Day {
    fn prompt() -> Option<&'static str> {
        Some("Configure day-of-month formatting:")
    }
}

impl Elicitation for time::format_description::modifier::Day {
    type Style = FmtDayStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting time::format_description::modifier::Day");
        let padding = time::format_description::modifier::Padding::elicit(communicator).await?;
        Ok(time::format_description::modifier::Day::default().with_padding(padding))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trivial_prop(
            "time::format_description::modifier::Day",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trivial_prop(
            "time::format_description::modifier::Day",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trivial_prop(
            "time::format_description::modifier::Day",
        )
    }
}

impl ElicitIntrospect for time::format_description::modifier::Day {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::format_description::modifier::Day",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "padding",
                    type_name: "time::format_description::modifier::Padding",
                    prompt: None,
                }],
            },
        }
    }
}

impl ElicitPromptTree for time::format_description::modifier::Day {
    fn prompt_tree() -> PromptTree {
        PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "time::format_description::modifier::Day".to_string(),
            fields: vec![(
                "padding".to_string(),
                Box::new(time::format_description::modifier::Padding::prompt_tree()),
            )],
        }
    }
}

// time::format_description::modifier::End implementation (single-value — trailing_input is pub(crate))
impl Prompt for time::format_description::modifier::End {
    fn prompt() -> Option<&'static str> {
        Some("End-of-input component (single value)")
    }
}

impl Elicitation for time::format_description::modifier::End {
    type Style = FmtEndStyle;

    #[tracing::instrument(skip_all)]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        Ok(time::format_description::modifier::End::default())
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque(
            "time::format_description::modifier::End",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque(
            "time::format_description::modifier::End",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque(
            "time::format_description::modifier::End",
        )
    }
}

impl ElicitIntrospect for time::format_description::modifier::End {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::format_description::modifier::End",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

// time::format_description::modifier::TrailingInput implementation
impl Prompt for time::format_description::modifier::TrailingInput {
    fn prompt() -> Option<&'static str> {
        Some("Choose how to handle trailing input:")
    }
}

impl Select for time::format_description::modifier::TrailingInput {
    fn options() -> Vec<Self> {
        use time::format_description::modifier::TrailingInput;
        vec![TrailingInput::Prohibit, TrailingInput::Discard]
    }

    fn labels() -> Vec<String> {
        vec!["Prohibit".to_string(), "Discard".to_string()]
    }

    fn from_label(label: &str) -> Option<Self> {
        use time::format_description::modifier::TrailingInput;
        match label {
            "Prohibit" => Some(TrailingInput::Prohibit),
            "Discard" => Some(TrailingInput::Discard),
            _ => None,
        }
    }
}

impl Elicitation for time::format_description::modifier::TrailingInput {
    type Style = FmtTrailingInputStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting time::format_description::modifier::TrailingInput");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "value",
                "time::format_description::modifier::TrailingInput",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| {
                Self::prompt()
                    .unwrap_or("Choose how to handle trailing input:")
                    .to_string()
            });
        let params = mcp::select_params(&prompt, &Self::labels());
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid TrailingInput: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "time::format_description::modifier::TrailingInput",
            "Prohibit",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "time::format_description::modifier::TrailingInput",
            "Prohibit",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "time::format_description::modifier::TrailingInput",
            "Prohibit",
        )
    }
}

impl ElicitIntrospect for time::format_description::modifier::TrailingInput {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::format_description::modifier::TrailingInput",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

impl ElicitPromptTree for time::format_description::modifier::TrailingInput {
    fn prompt_tree() -> PromptTree {
        let labels = Self::labels();
        let count = labels.len();
        PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose how to handle trailing input:")
                .to_string(),
            type_name: "time::format_description::modifier::TrailingInput".to_string(),
            options: labels,
            branches: vec![None; count],
        }
    }
}

// time::format_description::modifier::Ignore (count: NonZero<u16>, no Default)
impl Prompt for time::format_description::modifier::Ignore {
    fn prompt() -> Option<&'static str> {
        Some("Number of bytes to ignore (non-zero):")
    }
}

impl Elicitation for time::format_description::modifier::Ignore {
    type Style = FmtIgnoreStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting time::format_description::modifier::Ignore");
        let raw = u16::elicit(communicator).await?;
        let count = core::num::NonZero::<u16>::new(raw).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(
                "Ignore count must be non-zero".to_string(),
            ))
        })?;
        Ok(time::format_description::modifier::Ignore::count(count))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trivial_prop(
            "time::format_description::modifier::Ignore",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trivial_prop(
            "time::format_description::modifier::Ignore",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trivial_prop(
            "time::format_description::modifier::Ignore",
        )
    }
}

impl ElicitIntrospect for time::format_description::modifier::Ignore {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::format_description::modifier::Ignore",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "count",
                    type_name: "u16",
                    prompt: Some("Number of bytes to ignore (non-zero):"),
                }],
            },
        }
    }
}

impl ElicitPromptTree for time::format_description::modifier::Ignore {
    fn prompt_tree() -> PromptTree {
        PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "time::format_description::modifier::Ignore".to_string(),
            fields: vec![("count".to_string(), Box::new(u16::prompt_tree()))],
        }
    }
}

// time::format_description::modifier::Minute (padding only, like Day)
impl Prompt for time::format_description::modifier::Minute {
    fn prompt() -> Option<&'static str> {
        Some("Configure minute formatting:")
    }
}

impl Elicitation for time::format_description::modifier::Minute {
    type Style = FmtMinuteStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting time::format_description::modifier::Minute");
        let padding = time::format_description::modifier::Padding::elicit(communicator).await?;
        Ok(time::format_description::modifier::Minute::default().with_padding(padding))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trivial_prop(
            "time::format_description::modifier::Minute",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trivial_prop(
            "time::format_description::modifier::Minute",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trivial_prop(
            "time::format_description::modifier::Minute",
        )
    }
}

impl ElicitIntrospect for time::format_description::modifier::Minute {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::format_description::modifier::Minute",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "padding",
                    type_name: "time::format_description::modifier::Padding",
                    prompt: None,
                }],
            },
        }
    }
}

impl ElicitPromptTree for time::format_description::modifier::Minute {
    fn prompt_tree() -> PromptTree {
        PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "time::format_description::modifier::Minute".to_string(),
            fields: vec![(
                "padding".to_string(),
                Box::new(time::format_description::modifier::Padding::prompt_tree()),
            )],
        }
    }
}

// ── padding-only modifiers (pub padding field) ────────────────────────────────
// Ordinal, Second, OffsetMinute, OffsetSecond, WeekNumberIso/Sunday/Monday
// share an identical shape: one `pub padding: Padding` field.

macro_rules! impl_padding_modifier {
    ($ty:ty, $style:ty, $name:literal, $prompt:literal, $kani_fn:ident) => {
        impl Prompt for $ty {
            fn prompt() -> Option<&'static str> {
                Some($prompt)
            }
        }

        impl Elicitation for $ty {
            type Style = $style;

            #[tracing::instrument(skip(communicator))]
            async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                tracing::debug!(concat!("Eliciting ", $name));
                let padding =
                    time::format_description::modifier::Padding::elicit(communicator).await?;
                Ok(Self::default().with_padding(padding))
            }

            fn kani_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::$kani_fn($name)
            }

            fn verus_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::verus_trivial_prop($name)
            }

            fn creusot_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::creusot_trivial_prop($name)
            }
        }

        impl ElicitIntrospect for $ty {
            fn pattern() -> ElicitationPattern {
                ElicitationPattern::Survey
            }

            fn metadata() -> TypeMetadata {
                TypeMetadata {
                    type_name: $name,
                    description: <$ty as Prompt>::prompt(),
                    details: PatternDetails::Survey {
                        fields: vec![FieldInfo {
                            name: "padding",
                            type_name: "time::format_description::modifier::Padding",
                            prompt: None,
                        }],
                    },
                }
            }
        }

        impl ElicitPromptTree for $ty {
            fn prompt_tree() -> PromptTree {
                PromptTree::Survey {
                    prompt: <$ty as Prompt>::prompt().map(str::to_string),
                    type_name: $name.to_string(),
                    fields: vec![(
                        "padding".to_string(),
                        Box::new(time::format_description::modifier::Padding::prompt_tree()),
                    )],
                }
            }
        }
    };
}

crate::default_style!(time::format_description::modifier::Ordinal => FmtOrdinalStyle);
crate::default_style!(time::format_description::modifier::Second => FmtSecondStyle);
crate::default_style!(time::format_description::modifier::OffsetMinute => FmtOffsetMinuteStyle);
crate::default_style!(time::format_description::modifier::OffsetSecond => FmtOffsetSecondStyle);
crate::default_style!(time::format_description::modifier::WeekNumberIso => FmtWeekNumberIsoStyle);
crate::default_style!(time::format_description::modifier::WeekNumberSunday => FmtWeekNumberSundayStyle);
crate::default_style!(time::format_description::modifier::WeekNumberMonday => FmtWeekNumberMondayStyle);

impl_padding_modifier!(
    time::format_description::modifier::Ordinal,
    FmtOrdinalStyle,
    "time::format_description::modifier::Ordinal",
    "Configure ordinal day-of-year formatting:",
    kani_trivial_prop
);
impl_padding_modifier!(
    time::format_description::modifier::Second,
    FmtSecondStyle,
    "time::format_description::modifier::Second",
    "Configure second formatting:",
    kani_trivial_prop
);
impl_padding_modifier!(
    time::format_description::modifier::OffsetMinute,
    FmtOffsetMinuteStyle,
    "time::format_description::modifier::OffsetMinute",
    "Configure UTC offset minute formatting:",
    kani_trivial_prop
);
impl_padding_modifier!(
    time::format_description::modifier::OffsetSecond,
    FmtOffsetSecondStyle,
    "time::format_description::modifier::OffsetSecond",
    "Configure UTC offset second formatting:",
    kani_trivial_prop
);
impl_padding_modifier!(
    time::format_description::modifier::WeekNumberIso,
    FmtWeekNumberIsoStyle,
    "time::format_description::modifier::WeekNumberIso",
    "Configure ISO week number formatting:",
    kani_trivial_prop
);
impl_padding_modifier!(
    time::format_description::modifier::WeekNumberSunday,
    FmtWeekNumberSundayStyle,
    "time::format_description::modifier::WeekNumberSunday",
    "Configure Sunday-based week number formatting:",
    kani_trivial_prop
);
impl_padding_modifier!(
    time::format_description::modifier::WeekNumberMonday,
    FmtWeekNumberMondayStyle,
    "time::format_description::modifier::WeekNumberMonday",
    "Configure Monday-based week number formatting:",
    kani_trivial_prop
);

// ── Opaque-padding modifier types ─────────────────────────────────────────────
// Fields are pub(crate) in time; elicitation works via with_padding(), but
// ToCodeLiteral on the raw type can only emit Type::default() — use the
// FmtXxxWrap trenchcoat for faithful code generation.

crate::default_style!(time::format_description::modifier::Hour12 => FmtHour12Style);
crate::default_style!(time::format_description::modifier::Hour24 => FmtHour24Style);
crate::default_style!(time::format_description::modifier::MonthNumerical => FmtMonthNumericalStyle);
crate::default_style!(time::format_description::modifier::CalendarYearLastTwo => FmtCalendarYearLastTwoStyle);
crate::default_style!(time::format_description::modifier::IsoYearLastTwo => FmtIsoYearLastTwoStyle);

impl_padding_modifier!(
    time::format_description::modifier::Hour12,
    FmtHour12Style,
    "time::format_description::modifier::Hour12",
    "Configure 12-hour clock hour formatting:",
    kani_trivial_prop
);
impl_padding_modifier!(
    time::format_description::modifier::Hour24,
    FmtHour24Style,
    "time::format_description::modifier::Hour24",
    "Configure 24-hour clock hour formatting:",
    kani_trivial_prop
);
impl_padding_modifier!(
    time::format_description::modifier::MonthNumerical,
    FmtMonthNumericalStyle,
    "time::format_description::modifier::MonthNumerical",
    "Configure numerical month formatting:",
    kani_trivial_prop
);
impl_padding_modifier!(
    time::format_description::modifier::CalendarYearLastTwo,
    FmtCalendarYearLastTwoStyle,
    "time::format_description::modifier::CalendarYearLastTwo",
    "Configure calendar year last-two-digits formatting:",
    kani_trivial_prop
);
impl_padding_modifier!(
    time::format_description::modifier::IsoYearLastTwo,
    FmtIsoYearLastTwoStyle,
    "time::format_description::modifier::IsoYearLastTwo",
    "Configure ISO year last-two-digits formatting:",
    kani_trivial_prop
);

// ── Bool-field modifier types ──────────────────────────────────────────────────
// $field_name: the literal field name for metadata/prompt-tree (e.g. "case_sensitive")
// $with_method: the ident of the builder method (e.g. with_case_sensitive)

macro_rules! impl_bool_modifier {
    ($ty:ty, $style:ty, $name:literal, $prompt:literal, $field_name:literal, $with_method:ident, $kani_fn:ident) => {
        impl Prompt for $ty {
            fn prompt() -> Option<&'static str> {
                Some($prompt)
            }
        }

        impl Elicitation for $ty {
            type Style = $style;

            #[tracing::instrument(skip(communicator))]
            async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                tracing::debug!(concat!("Eliciting ", $name));
                let value = bool::elicit(communicator).await?;
                Ok(Self::default().$with_method(value))
            }

            fn kani_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::$kani_fn($name)
            }

            fn verus_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::verus_trivial_prop($name)
            }

            fn creusot_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::creusot_trivial_prop($name)
            }
        }

        impl ElicitIntrospect for $ty {
            fn pattern() -> ElicitationPattern {
                ElicitationPattern::Survey
            }

            fn metadata() -> TypeMetadata {
                TypeMetadata {
                    type_name: $name,
                    description: <$ty as Prompt>::prompt(),
                    details: PatternDetails::Survey {
                        fields: vec![FieldInfo {
                            name: $field_name,
                            type_name: "bool",
                            prompt: None,
                        }],
                    },
                }
            }
        }

        impl ElicitPromptTree for $ty {
            fn prompt_tree() -> PromptTree {
                PromptTree::Survey {
                    prompt: <$ty as Prompt>::prompt().map(str::to_string),
                    type_name: $name.to_string(),
                    fields: vec![($field_name.to_string(), Box::new(bool::prompt_tree()))],
                }
            }
        }
    };
}

crate::default_style!(time::format_description::modifier::MonthShort => FmtMonthShortStyle);
crate::default_style!(time::format_description::modifier::MonthLong => FmtMonthLongStyle);
crate::default_style!(time::format_description::modifier::WeekdayShort => FmtWeekdayShortStyle);
crate::default_style!(time::format_description::modifier::WeekdayLong => FmtWeekdayLongStyle);
crate::default_style!(time::format_description::modifier::WeekdaySunday => FmtWeekdaySundayStyle);
crate::default_style!(time::format_description::modifier::WeekdayMonday => FmtWeekdayMondayStyle);
crate::default_style!(time::format_description::modifier::UnixTimestampSecond => FmtUnixTimestampSecondStyle);
crate::default_style!(time::format_description::modifier::UnixTimestampMillisecond => FmtUnixTimestampMillisecondStyle);
crate::default_style!(time::format_description::modifier::UnixTimestampMicrosecond => FmtUnixTimestampMicrosecondStyle);
crate::default_style!(time::format_description::modifier::UnixTimestampNanosecond => FmtUnixTimestampNanosecondStyle);

impl_bool_modifier!(
    time::format_description::modifier::MonthShort,
    FmtMonthShortStyle,
    "time::format_description::modifier::MonthShort",
    "Configure abbreviated month name formatting:",
    "case_sensitive",
    with_case_sensitive,
    kani_trivial_prop
);
impl_bool_modifier!(
    time::format_description::modifier::MonthLong,
    FmtMonthLongStyle,
    "time::format_description::modifier::MonthLong",
    "Configure full month name formatting:",
    "case_sensitive",
    with_case_sensitive,
    kani_trivial_prop
);
impl_bool_modifier!(
    time::format_description::modifier::WeekdayShort,
    FmtWeekdayShortStyle,
    "time::format_description::modifier::WeekdayShort",
    "Configure abbreviated weekday name formatting:",
    "case_sensitive",
    with_case_sensitive,
    kani_trivial_prop
);
impl_bool_modifier!(
    time::format_description::modifier::WeekdayLong,
    FmtWeekdayLongStyle,
    "time::format_description::modifier::WeekdayLong",
    "Configure full weekday name formatting:",
    "case_sensitive",
    with_case_sensitive,
    kani_trivial_prop
);
impl_bool_modifier!(
    time::format_description::modifier::WeekdaySunday,
    FmtWeekdaySundayStyle,
    "time::format_description::modifier::WeekdaySunday",
    "Configure Sunday-based weekday index formatting:",
    "one_indexed",
    with_one_indexed,
    kani_trivial_prop
);
impl_bool_modifier!(
    time::format_description::modifier::WeekdayMonday,
    FmtWeekdayMondayStyle,
    "time::format_description::modifier::WeekdayMonday",
    "Configure Monday-based weekday index formatting:",
    "one_indexed",
    with_one_indexed,
    kani_trivial_prop
);
impl_bool_modifier!(
    time::format_description::modifier::UnixTimestampSecond,
    FmtUnixTimestampSecondStyle,
    "time::format_description::modifier::UnixTimestampSecond",
    "Configure Unix timestamp (second precision) formatting:",
    "sign_is_mandatory",
    with_sign_is_mandatory,
    kani_trivial_prop
);
impl_bool_modifier!(
    time::format_description::modifier::UnixTimestampMillisecond,
    FmtUnixTimestampMillisecondStyle,
    "time::format_description::modifier::UnixTimestampMillisecond",
    "Configure Unix timestamp (millisecond precision) formatting:",
    "sign_is_mandatory",
    with_sign_is_mandatory,
    kani_trivial_prop
);
impl_bool_modifier!(
    time::format_description::modifier::UnixTimestampMicrosecond,
    FmtUnixTimestampMicrosecondStyle,
    "time::format_description::modifier::UnixTimestampMicrosecond",
    "Configure Unix timestamp (microsecond precision) formatting:",
    "sign_is_mandatory",
    with_sign_is_mandatory,
    kani_trivial_prop
);
impl_bool_modifier!(
    time::format_description::modifier::UnixTimestampNanosecond,
    FmtUnixTimestampNanosecondStyle,
    "time::format_description::modifier::UnixTimestampNanosecond",
    "Configure Unix timestamp (nanosecond precision) formatting:",
    "sign_is_mandatory",
    with_sign_is_mandatory,
    kani_trivial_prop
);

// ── Padding + sign_is_mandatory modifier types ────────────────────────────────

macro_rules! impl_padding_sign_modifier {
    ($ty:ty, $style:ty, $name:literal, $prompt:literal, $kani_fn:ident) => {
        impl Prompt for $ty {
            fn prompt() -> Option<&'static str> {
                Some($prompt)
            }
        }

        impl Elicitation for $ty {
            type Style = $style;

            #[tracing::instrument(skip(communicator))]
            async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                tracing::debug!(concat!("Eliciting ", $name));
                let padding =
                    time::format_description::modifier::Padding::elicit(communicator).await?;
                let sign_is_mandatory = bool::elicit(communicator).await?;
                Ok(Self::default()
                    .with_padding(padding)
                    .with_sign_is_mandatory(sign_is_mandatory))
            }

            fn kani_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::$kani_fn($name)
            }

            fn verus_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::verus_trivial_prop($name)
            }

            fn creusot_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::creusot_trivial_prop($name)
            }
        }

        impl ElicitIntrospect for $ty {
            fn pattern() -> ElicitationPattern {
                ElicitationPattern::Survey
            }

            fn metadata() -> TypeMetadata {
                TypeMetadata {
                    type_name: $name,
                    description: <$ty as Prompt>::prompt(),
                    details: PatternDetails::Survey {
                        fields: vec![
                            FieldInfo {
                                name: "padding",
                                type_name: "time::format_description::modifier::Padding",
                                prompt: None,
                            },
                            FieldInfo {
                                name: "sign_is_mandatory",
                                type_name: "bool",
                                prompt: None,
                            },
                        ],
                    },
                }
            }
        }

        impl ElicitPromptTree for $ty {
            fn prompt_tree() -> PromptTree {
                PromptTree::Survey {
                    prompt: <$ty as Prompt>::prompt().map(str::to_string),
                    type_name: $name.to_string(),
                    fields: vec![
                        (
                            "padding".to_string(),
                            Box::new(time::format_description::modifier::Padding::prompt_tree()),
                        ),
                        (
                            "sign_is_mandatory".to_string(),
                            Box::new(bool::prompt_tree()),
                        ),
                    ],
                }
            }
        }
    };
}

crate::default_style!(time::format_description::modifier::CalendarYearFullExtendedRange => FmtCalendarYearFullExtendedRangeStyle);
crate::default_style!(time::format_description::modifier::CalendarYearFullStandardRange => FmtCalendarYearFullStandardRangeStyle);
crate::default_style!(time::format_description::modifier::CalendarYearCenturyExtendedRange => FmtCalendarYearCenturyExtendedRangeStyle);
crate::default_style!(time::format_description::modifier::CalendarYearCenturyStandardRange => FmtCalendarYearCenturyStandardRangeStyle);
crate::default_style!(time::format_description::modifier::IsoYearFullExtendedRange => FmtIsoYearFullExtendedRangeStyle);
crate::default_style!(time::format_description::modifier::IsoYearFullStandardRange => FmtIsoYearFullStandardRangeStyle);
crate::default_style!(time::format_description::modifier::IsoYearCenturyExtendedRange => FmtIsoYearCenturyExtendedRangeStyle);
crate::default_style!(time::format_description::modifier::IsoYearCenturyStandardRange => FmtIsoYearCenturyStandardRangeStyle);

impl_padding_sign_modifier!(
    time::format_description::modifier::CalendarYearFullExtendedRange,
    FmtCalendarYearFullExtendedRangeStyle,
    "time::format_description::modifier::CalendarYearFullExtendedRange",
    "Configure calendar year (full, extended range) formatting:",
    kani_trivial_prop
);
impl_padding_sign_modifier!(
    time::format_description::modifier::CalendarYearFullStandardRange,
    FmtCalendarYearFullStandardRangeStyle,
    "time::format_description::modifier::CalendarYearFullStandardRange",
    "Configure calendar year (full, standard range) formatting:",
    kani_trivial_prop
);
impl_padding_sign_modifier!(
    time::format_description::modifier::CalendarYearCenturyExtendedRange,
    FmtCalendarYearCenturyExtendedRangeStyle,
    "time::format_description::modifier::CalendarYearCenturyExtendedRange",
    "Configure calendar year century (extended range) formatting:",
    kani_trivial_prop
);
impl_padding_sign_modifier!(
    time::format_description::modifier::CalendarYearCenturyStandardRange,
    FmtCalendarYearCenturyStandardRangeStyle,
    "time::format_description::modifier::CalendarYearCenturyStandardRange",
    "Configure calendar year century (standard range) formatting:",
    kani_trivial_prop
);
impl_padding_sign_modifier!(
    time::format_description::modifier::IsoYearFullExtendedRange,
    FmtIsoYearFullExtendedRangeStyle,
    "time::format_description::modifier::IsoYearFullExtendedRange",
    "Configure ISO year (full, extended range) formatting:",
    kani_trivial_prop
);
impl_padding_sign_modifier!(
    time::format_description::modifier::IsoYearFullStandardRange,
    FmtIsoYearFullStandardRangeStyle,
    "time::format_description::modifier::IsoYearFullStandardRange",
    "Configure ISO year (full, standard range) formatting:",
    kani_trivial_prop
);
impl_padding_sign_modifier!(
    time::format_description::modifier::IsoYearCenturyExtendedRange,
    FmtIsoYearCenturyExtendedRangeStyle,
    "time::format_description::modifier::IsoYearCenturyExtendedRange",
    "Configure ISO year century (extended range) formatting:",
    kani_trivial_prop
);
impl_padding_sign_modifier!(
    time::format_description::modifier::IsoYearCenturyStandardRange,
    FmtIsoYearCenturyStandardRangeStyle,
    "time::format_description::modifier::IsoYearCenturyStandardRange",
    "Configure ISO year century (standard range) formatting:",
    kani_trivial_prop
);

// ── OffsetHour (pub padding + pub sign_is_mandatory) ──────────────────────────

crate::default_style!(time::format_description::modifier::OffsetHour => FmtOffsetHourStyle);

impl_padding_sign_modifier!(
    time::format_description::modifier::OffsetHour,
    FmtOffsetHourStyle,
    "time::format_description::modifier::OffsetHour",
    "Configure UTC offset hour formatting:",
    kani_trivial_prop
);

// ── Period (pub is_uppercase + pub case_sensitive) ────────────────────────────

crate::default_style!(time::format_description::modifier::Period => FmtPeriodStyle);

impl Prompt for time::format_description::modifier::Period {
    fn prompt() -> Option<&'static str> {
        Some("Configure AM/PM period formatting:")
    }
}

impl Elicitation for time::format_description::modifier::Period {
    type Style = FmtPeriodStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting time::format_description::modifier::Period");
        let is_uppercase = bool::elicit(communicator).await?;
        let case_sensitive = bool::elicit(communicator).await?;
        Ok(Self::default()
            .with_is_uppercase(is_uppercase)
            .with_case_sensitive(case_sensitive))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trivial_prop(
            "time::format_description::modifier::Period",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trivial_prop(
            "time::format_description::modifier::Period",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trivial_prop(
            "time::format_description::modifier::Period",
        )
    }
}

impl ElicitIntrospect for time::format_description::modifier::Period {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::format_description::modifier::Period",
            description: <time::format_description::modifier::Period as Prompt>::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "is_uppercase",
                        type_name: "bool",
                        prompt: None,
                    },
                    FieldInfo {
                        name: "case_sensitive",
                        type_name: "bool",
                        prompt: None,
                    },
                ],
            },
        }
    }
}

impl ElicitPromptTree for time::format_description::modifier::Period {
    fn prompt_tree() -> PromptTree {
        PromptTree::Survey {
            prompt: <time::format_description::modifier::Period as Prompt>::prompt()
                .map(str::to_string),
            type_name: "time::format_description::modifier::Period".to_string(),
            fields: vec![
                ("is_uppercase".to_string(), Box::new(bool::prompt_tree())),
                ("case_sensitive".to_string(), Box::new(bool::prompt_tree())),
            ],
        }
    }
}

// ── SubsecondDigits (Select enum) ─────────────────────────────────────────────

crate::default_style!(time::format_description::modifier::SubsecondDigits => FmtSubsecondDigitsStyle);

impl Prompt for time::format_description::modifier::SubsecondDigits {
    fn prompt() -> Option<&'static str> {
        Some("Choose subsecond digit count:")
    }
}

impl Select for time::format_description::modifier::SubsecondDigits {
    fn options() -> Vec<Self> {
        use time::format_description::modifier::SubsecondDigits;
        vec![
            SubsecondDigits::OneOrMore,
            SubsecondDigits::One,
            SubsecondDigits::Two,
            SubsecondDigits::Three,
            SubsecondDigits::Four,
            SubsecondDigits::Five,
            SubsecondDigits::Six,
            SubsecondDigits::Seven,
            SubsecondDigits::Eight,
            SubsecondDigits::Nine,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "OneOrMore".to_string(),
            "One".to_string(),
            "Two".to_string(),
            "Three".to_string(),
            "Four".to_string(),
            "Five".to_string(),
            "Six".to_string(),
            "Seven".to_string(),
            "Eight".to_string(),
            "Nine".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        use time::format_description::modifier::SubsecondDigits;
        match label {
            "OneOrMore" => Some(SubsecondDigits::OneOrMore),
            "One" => Some(SubsecondDigits::One),
            "Two" => Some(SubsecondDigits::Two),
            "Three" => Some(SubsecondDigits::Three),
            "Four" => Some(SubsecondDigits::Four),
            "Five" => Some(SubsecondDigits::Five),
            "Six" => Some(SubsecondDigits::Six),
            "Seven" => Some(SubsecondDigits::Seven),
            "Eight" => Some(SubsecondDigits::Eight),
            "Nine" => Some(SubsecondDigits::Nine),
            _ => None,
        }
    }
}

impl Elicitation for time::format_description::modifier::SubsecondDigits {
    type Style = FmtSubsecondDigitsStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting time::format_description::modifier::SubsecondDigits");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "value",
                "time::format_description::modifier::SubsecondDigits",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| {
                Self::prompt()
                    .unwrap_or("Choose subsecond digit count:")
                    .to_string()
            });
        let params = mcp::select_params(&prompt, &Self::labels());
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid SubsecondDigits: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "time::format_description::modifier::SubsecondDigits",
            "OneOrMore",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "time::format_description::modifier::SubsecondDigits",
            "OneOrMore",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "time::format_description::modifier::SubsecondDigits",
            "OneOrMore",
        )
    }
}

impl ElicitIntrospect for time::format_description::modifier::SubsecondDigits {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::format_description::modifier::SubsecondDigits",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

impl ElicitPromptTree for time::format_description::modifier::SubsecondDigits {
    fn prompt_tree() -> PromptTree {
        let labels = Self::labels();
        let count = labels.len();
        PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose subsecond digit count:")
                .to_string(),
            type_name: "time::format_description::modifier::SubsecondDigits".to_string(),
            options: labels,
            branches: vec![None; count],
        }
    }
}

// ── Subsecond (pub digits: SubsecondDigits) ───────────────────────────────────

crate::default_style!(time::format_description::modifier::Subsecond => FmtSubsecondStyle);

impl Prompt for time::format_description::modifier::Subsecond {
    fn prompt() -> Option<&'static str> {
        Some("Configure subsecond formatting:")
    }
}

impl Elicitation for time::format_description::modifier::Subsecond {
    type Style = FmtSubsecondStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting time::format_description::modifier::Subsecond");
        let digits =
            time::format_description::modifier::SubsecondDigits::elicit(communicator).await?;
        Ok(Self::default().with_digits(digits))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trivial_prop(
            "time::format_description::modifier::Subsecond",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trivial_prop(
            "time::format_description::modifier::Subsecond",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trivial_prop(
            "time::format_description::modifier::Subsecond",
        )
    }
}

impl ElicitIntrospect for time::format_description::modifier::Subsecond {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::format_description::modifier::Subsecond",
            description: <time::format_description::modifier::Subsecond as Prompt>::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "digits",
                    type_name: "time::format_description::modifier::SubsecondDigits",
                    prompt: None,
                }],
            },
        }
    }
}

impl ElicitPromptTree for time::format_description::modifier::Subsecond {
    fn prompt_tree() -> PromptTree {
        PromptTree::Survey {
            prompt: <time::format_description::modifier::Subsecond as Prompt>::prompt()
                .map(str::to_string),
            type_name: "time::format_description::modifier::Subsecond".to_string(),
            fields: vec![(
                "digits".to_string(),
                Box::new(time::format_description::modifier::SubsecondDigits::prompt_tree()),
            )],
        }
    }
}

// ── well_known::Rfc2822 (unit struct) ─────────────────────────────────────────

crate::default_style!(time::format_description::well_known::Rfc2822 => WellKnownRfc2822Style);

impl Prompt for time::format_description::well_known::Rfc2822 {
    fn prompt() -> Option<&'static str> {
        Some("RFC 2822 date-time format (e.g. Fri, 21 Nov 1997 09:55:06 -0600) — no options.")
    }
}

impl Elicitation for time::format_description::well_known::Rfc2822 {
    type Style = WellKnownRfc2822Style;

    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        Ok(Self)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque(
            "time::format_description::well_known::Rfc2822",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque(
            "time::format_description::well_known::Rfc2822",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque(
            "time::format_description::well_known::Rfc2822",
        )
    }
}

impl ElicitIntrospect for time::format_description::well_known::Rfc2822 {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::format_description::well_known::Rfc2822",
            description: <time::format_description::well_known::Rfc2822 as Prompt>::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for time::format_description::well_known::Rfc2822 {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt()
                .unwrap_or("time::format_description::well_known::Rfc2822")
                .to_string(),
            type_name: "time::format_description::well_known::Rfc2822".to_string(),
        }
    }
}

// ── well_known::Iso8601<CONFIG> (unit struct with const generic) ──────────────

/// Style for [`time::format_description::well_known::Iso8601`].
///
/// Const-generic unit struct — no user choices; the CONFIG parameter is fixed
/// at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Iso8601Style {
    #[default]
    /// Default style — no style options available.
    Default,
}

impl Prompt for Iso8601Style {
    fn prompt() -> Option<&'static str> {
        None
    }
}

impl Elicitation for Iso8601Style {
    type Style = Iso8601Style;

    #[tracing::instrument(skip_all)]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        Ok(Self::Default)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_single_variant_enum("Iso8601Style")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_single_variant_enum("Iso8601Style")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_single_variant_enum("Iso8601Style")
    }
}

impl crate::style::ElicitationStyle for Iso8601Style {}

impl<const CONFIG: time::format_description::well_known::iso8601::EncodedConfig> Prompt
    for time::format_description::well_known::Iso8601<CONFIG>
{
    fn prompt() -> Option<&'static str> {
        Some("ISO 8601 date/time format — unit struct, CONFIG parameter is set at compile time.")
    }
}

impl<const CONFIG: time::format_description::well_known::iso8601::EncodedConfig> Elicitation
    for time::format_description::well_known::Iso8601<CONFIG>
{
    type Style = Iso8601Style;

    #[tracing::instrument(skip_all)]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        Ok(time::format_description::well_known::Iso8601)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque(
            "time::format_description::well_known::Iso8601<CONFIG>",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque(
            "time::format_description::well_known::Iso8601<CONFIG>",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque(
            "time::format_description::well_known::Iso8601<CONFIG>",
        )
    }
}

impl<const CONFIG: time::format_description::well_known::iso8601::EncodedConfig> ElicitIntrospect
    for time::format_description::well_known::Iso8601<CONFIG>
{
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::format_description::well_known::Iso8601",
            description: <time::format_description::well_known::Iso8601<CONFIG> as Prompt>::prompt(
            ),
            details: PatternDetails::Primitive,
        }
    }
}

impl<const CONFIG: time::format_description::well_known::iso8601::EncodedConfig> ElicitPromptTree
    for time::format_description::well_known::Iso8601<CONFIG>
{
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: <time::format_description::well_known::Iso8601<CONFIG> as Prompt>::prompt()
                .unwrap_or("time::format_description::well_known::Iso8601")
                .to_string(),
            type_name: "time::format_description::well_known::Iso8601".to_string(),
        }
    }
}

// ── well_known::Rfc3339 (unit struct) ─────────────────────────────────────────

crate::default_style!(time::format_description::well_known::Rfc3339 => WellKnownRfc3339Style);

impl Prompt for time::format_description::well_known::Rfc3339 {
    fn prompt() -> Option<&'static str> {
        Some("RFC 3339 / ISO 8601 date-time format (e.g. 1996-12-19T16:39:57-08:00) — no options.")
    }
}

impl Elicitation for time::format_description::well_known::Rfc3339 {
    type Style = WellKnownRfc3339Style;

    #[tracing::instrument(skip_all)]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        Ok(Self)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque(
            "time::format_description::well_known::Rfc3339",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque(
            "time::format_description::well_known::Rfc3339",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque(
            "time::format_description::well_known::Rfc3339",
        )
    }
}

impl ElicitIntrospect for time::format_description::well_known::Rfc3339 {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::format_description::well_known::Rfc3339",
            description: <time::format_description::well_known::Rfc3339 as Prompt>::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for time::format_description::well_known::Rfc3339 {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt()
                .unwrap_or("time::format_description::well_known::Rfc3339")
                .to_string(),
            type_name: "time::format_description::well_known::Rfc3339".to_string(),
        }
    }
}

// ── iso8601::DateKind ─────────────────────────────────────────────────────────

crate::default_style!(time::format_description::well_known::iso8601::DateKind => Iso8601DateKindStyle);

impl Prompt for time::format_description::well_known::iso8601::DateKind {
    fn prompt() -> Option<&'static str> {
        Some(
            "ISO 8601 date representation: calendar (year-month-day), week (year-week-weekday), or ordinal (year-day)?",
        )
    }
}

impl Select for time::format_description::well_known::iso8601::DateKind {
    fn options() -> Vec<Self> {
        use time::format_description::well_known::iso8601::DateKind;
        vec![DateKind::Calendar, DateKind::Week, DateKind::Ordinal]
    }

    fn labels() -> Vec<String> {
        vec![
            "Calendar".to_string(),
            "Week".to_string(),
            "Ordinal".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        use time::format_description::well_known::iso8601::DateKind;
        match label {
            "Calendar" => Some(DateKind::Calendar),
            "Week" => Some(DateKind::Week),
            "Ordinal" => Some(DateKind::Ordinal),
            _ => None,
        }
    }
}

impl Elicitation for time::format_description::well_known::iso8601::DateKind {
    type Style = Iso8601DateKindStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting iso8601::DateKind");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "value",
                "iso8601::DateKind",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| Self::prompt().unwrap_or("ISO 8601 date kind?").to_string());
        let params = mcp::select_params(&prompt, &Self::labels());
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid iso8601::DateKind: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "time::format_description::well_known::iso8601::DateKind",
            "Calendar",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "time::format_description::well_known::iso8601::DateKind",
            "Calendar",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "time::format_description::well_known::iso8601::DateKind",
            "Calendar",
        )
    }
}

impl ElicitIntrospect for time::format_description::well_known::iso8601::DateKind {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "time::format_description::well_known::iso8601::DateKind",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

impl ElicitPromptTree for time::format_description::well_known::iso8601::DateKind {
    fn prompt_tree() -> PromptTree {
        let labels = Self::labels();
        let count = labels.len();
        PromptTree::Select {
            prompt: Self::prompt().unwrap_or("ISO 8601 date kind?").to_string(),
            type_name: "time::format_description::well_known::iso8601::DateKind".to_string(),
            options: labels,
            branches: vec![None; count],
        }
    }
}
