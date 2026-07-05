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
    ElicitResult, Elicitation, ElicitationPattern, Generator, PatternDetails, Prompt, PromptTree,
    Select, TypeMetadata, VariantMetadata,
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
        // Use standard Select elicit pattern
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Select an option:"),
            &Self::labels(),
        );

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
        tracing::debug!("Eliciting time::Instant");

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
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Select an option:"),
            &Self::labels(),
        );

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
                // Elicit ISO 8601 string
                let prompt =
                    "Enter ISO 8601 datetime with offset (e.g., \"2024-07-11T15:30:00+05:00\"):";
                let params = mcp::text_params(prompt);
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

                // Elicit offset
                let offset_prompt = "Enter timezone offset in hours (e.g., +5 or -8):";
                let offset_params = mcp::number_params(offset_prompt, -12, 14);
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
                // Elicit ISO 8601 string (no timezone)
                let prompt = "Enter datetime (e.g., \"2024-07-11T15:30:00\"):";
                let params = mcp::text_params(prompt);
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

        let hour_params = mcp::number_params("Enter hour (0-23):", 0, 23);
        let hour_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(hour_params),
            )
            .await?;
        let hour = mcp::parse_integer::<i64>(mcp::extract_value(hour_result)?)? as u8;

        let minute_params = mcp::number_params("Enter minute (0-59):", 0, 59);
        let minute_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(minute_params),
            )
            .await?;
        let minute = mcp::parse_integer::<i64>(mcp::extract_value(minute_result)?)? as u8;

        let second_params = mcp::number_params("Enter second (0-59):", 0, 59);
        let second_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(second_params),
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

        let year_params = mcp::number_params("Enter year:", -9999, 9999);
        let year_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(year_params),
            )
            .await?;
        let year = mcp::parse_integer::<i64>(mcp::extract_value(year_result)?)? as i32;

        let month_params = mcp::number_params("Enter month (1-12):", 1, 12);
        let month_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(month_params),
            )
            .await?;
        let month_num = mcp::parse_integer::<i64>(mcp::extract_value(month_result)?)? as u8;
        let month = time::Month::try_from(month_num).map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid month: {e}")))
        })?;

        let day_params = mcp::number_params("Enter day (1-31):", 1, 31);
        let day_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(day_params),
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

        let seconds_params = mcp::number_params("Enter whole seconds (may be negative):", i64::MIN, i64::MAX);
        let seconds_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(seconds_params),
            )
            .await?;
        let seconds = mcp::parse_integer::<i64>(mcp::extract_value(seconds_result)?)?;

        let nanos_params = mcp::number_params("Enter subsecond nanoseconds (0–999999999):", 0, 999_999_999);
        let nanos_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(nanos_params),
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
        let params =
            mcp::select_params(Self::prompt().unwrap_or("Choose a month:"), &Self::labels());
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
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose a day of the week:"),
            &Self::labels(),
        );
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
        tracing::debug!("Eliciting UtcDateTime");
        let components = DateTimeComponents::elicit(communicator).await?;

        let date = time::Date::from_calendar_date(
            components.year,
            time::Month::try_from(components.month).map_err(|e| {
                ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid month: {e}")))
            })?,
            components.day,
        )
        .map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid date: {e}")))
        })?;

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
        let params = mcp::number_params("Enter UTC offset in whole seconds (-86399 to 86399):", -86_399, 86_399);
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(params),
            )
            .await?;
        let seconds = mcp::parse_integer::<i64>(mcp::extract_value(result)?)? as i32;
        UtcOffset::from_whole_seconds(seconds).map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid UTC offset: {e}")))
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
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose a time component range error:"),
            &Self::labels(),
        );
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

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting time::error::ConversionRange");
        let _ = communicator;
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

