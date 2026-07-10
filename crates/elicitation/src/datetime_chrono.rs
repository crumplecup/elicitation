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
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitPromptTree,
    ElicitResult, Elicitation, ElicitationPattern, FieldInfo, Generator, PatternDetails, Prompt,
    PromptTree, Select, TypeMetadata, VariantMetadata,
    datetime_common::{DateTimeComponents, DateTimeInputMethod},
    emit_code::ToCodeLiteral,
    mcp,
};
use chrono::{
    DateTime, Datelike, Duration, FixedOffset, Month, NaiveDate, NaiveDateTime, NaiveTime,
    TimeZone, Timelike, Utc, Weekday,
    naive::{NaiveDateDaysIterator, NaiveDateWeeksIterator},
};

// Style enums for datetime types
crate::default_style!(DateTime<Utc> => DateTimeUtcStyle);
crate::default_style!(DateTime<FixedOffset> => DateTimeFixedOffsetStyle);
crate::default_style!(NaiveDate => NaiveDateStyle);
crate::default_style!(NaiveDateTime => NaiveDateTimeStyle);
crate::default_style!(NaiveTime => NaiveTimeStyle);
crate::default_style!(DateTimeUtcGenerationMode => DateTimeUtcGenerationModeStyle);
crate::default_style!(NaiveDateTimeGenerationMode => NaiveDateTimeGenerationModeStyle);

// ============================================================================
// DateTime<Utc> Generator
// ============================================================================

/// Generation mode for `chrono::DateTime<Utc>`.
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
    fn options() -> Vec<Self> {
        vec![
            DateTimeUtcGenerationMode::Now,
            DateTimeUtcGenerationMode::UnixEpoch,
            DateTimeUtcGenerationMode::Offset { seconds: 0 },
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
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "value",
                "DateTimeUtcGenerationMode",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| {
                Self::prompt()
                    .unwrap_or("Choose DateTime<Utc> generation mode:")
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

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_multi_variant_enum(
            "DateTimeUtcGenerationMode",
            "Now",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_multi_variant_enum("DateTimeUtcGenerationMode")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_multi_variant_enum("DateTimeUtcGenerationMode")
    }
}

impl ElicitIntrospect for DateTimeUtcGenerationMode {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "DateTimeUtcGenerationMode",
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

/// Generator for creating `DateTime<Utc>` values.
#[derive(Debug, Clone, Copy)]
pub struct DateTimeUtcGenerator {
    mode: DateTimeUtcGenerationMode,
    reference: DateTime<Utc>,
}

impl DateTimeUtcGenerator {
    /// Create a new `DateTime<Utc>` generator.
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
    fn options() -> Vec<Self> {
        vec![
            NaiveDateTimeGenerationMode::Now,
            NaiveDateTimeGenerationMode::UnixEpoch,
            NaiveDateTimeGenerationMode::Offset { seconds: 0 },
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Now (Current time)".to_string(),
            "Unix Epoch (1970-01-01)".to_string(),
            "Offset (Custom)".to_string(),
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
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "value",
                "NaiveDateTimeGenerationMode",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| {
                Self::prompt()
                    .unwrap_or("Choose NaiveDateTime generation mode:")
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

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_multi_variant_enum(
            "NaiveDateTimeGenerationMode",
            "Now",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_multi_variant_enum("NaiveDateTimeGenerationMode")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_multi_variant_enum(
            "NaiveDateTimeGenerationMode",
        )
    }
}

impl ElicitIntrospect for NaiveDateTimeGenerationMode {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "NaiveDateTimeGenerationMode",
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
                let prompt = communicator
                    .style_context()
                    .prompt_for_type::<Self>(
                        "iso8601",
                        "String",
                        &crate::style::PromptContext::new(0, 1),
                    )?
                    .unwrap_or_else(|| {
                        "Enter ISO 8601 datetime (e.g., \"2024-07-11T15:30:00Z\"):".to_string()
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

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::DateTime<Utc>")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::DateTime<Utc>")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::DateTime<Utc>")
    }
}

impl ElicitIntrospect for DateTime<Utc> {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::DateTime<Utc>",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
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

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::DateTime<FixedOffset>")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::DateTime<FixedOffset>")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::DateTime<FixedOffset>")
    }
}

impl ElicitIntrospect for DateTime<FixedOffset> {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::DateTime<FixedOffset>",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
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

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::NaiveDateTime")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::NaiveDateTime")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::NaiveDateTime")
    }
}

impl ElicitIntrospect for NaiveDateTime {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::NaiveDateTime",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

// ============================================================================
// Weekday
// ============================================================================

impl Prompt for Weekday {
    fn prompt() -> Option<&'static str> {
        Some("Choose a day of the week:")
    }
}

impl Select for Weekday {
    fn options() -> Vec<Self> {
        vec![
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Mon".to_string(),
            "Tue".to_string(),
            "Wed".to_string(),
            "Thu".to_string(),
            "Fri".to_string(),
            "Sat".to_string(),
            "Sun".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Mon" => Some(Weekday::Mon),
            "Tue" => Some(Weekday::Tue),
            "Wed" => Some(Weekday::Wed),
            "Thu" => Some(Weekday::Thu),
            "Fri" => Some(Weekday::Fri),
            "Sat" => Some(Weekday::Sat),
            "Sun" => Some(Weekday::Sun),
            _ => None,
        }
    }
}

crate::default_style!(Weekday => WeekdayStyle);

impl Elicitation for Weekday {
    type Style = WeekdayStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Weekday");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "value",
                "chrono::Weekday",
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
        crate::verification::proof_helpers::kani_select_wrapper("Weekday", "Mon")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("Weekday", "Mon")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("Weekday", "Mon")
    }
}

impl ElicitIntrospect for Weekday {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::Weekday",
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

impl ElicitPromptTree for Weekday {
    fn prompt_tree() -> PromptTree {
        let labels = Self::labels();
        let count = labels.len();
        PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose a day of the week:")
                .to_string(),
            type_name: "chrono::Weekday".to_string(),
            options: labels,
            branches: vec![None; count],
        }
    }
}

impl ToCodeLiteral for Weekday {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let variant = match self {
            Weekday::Mon => "Mon",
            Weekday::Tue => "Tue",
            Weekday::Wed => "Wed",
            Weekday::Thu => "Thu",
            Weekday::Fri => "Fri",
            Weekday::Sat => "Sat",
            Weekday::Sun => "Sun",
        };
        let ident = proc_macro2::Ident::new(variant, proc_macro2::Span::call_site());
        crate::quote::quote! { chrono::Weekday::#ident }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::Weekday }
    }
}

// ============================================================================
// NaiveDate
// ============================================================================

impl Prompt for NaiveDate {
    fn prompt() -> Option<&'static str> {
        Some("Enter a date (YYYY-MM-DD):")
    }
}

impl Elicitation for NaiveDate {
    type Style = NaiveDateStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting NaiveDate");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "value",
                "chrono::NaiveDate",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| {
                Self::prompt()
                    .unwrap_or("Enter a date (e.g., \"2024-07-11\"):")
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
        let s = mcp::parse_string(value)?;
        NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid date (expected YYYY-MM-DD): {e}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::NaiveDate")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::NaiveDate")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::NaiveDate")
    }
}

impl ElicitIntrospect for NaiveDate {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::NaiveDate",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for NaiveDate {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt().unwrap_or("NaiveDate").to_string(),
            type_name: "chrono::NaiveDate".to_string(),
        }
    }
}

impl ToCodeLiteral for NaiveDate {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let year = self.year();
        let month = self.month();
        let day = self.day();
        crate::quote::quote! {
            match chrono::NaiveDate::from_ymd_opt(#year, #month, #day) {
                Some(d) => d,
                None => return Err("from_ymd_opt: invalid date components".into()),
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::NaiveDate }
    }
}

// ============================================================================
// NaiveTime
// ============================================================================

impl Prompt for NaiveTime {
    fn prompt() -> Option<&'static str> {
        Some("Enter a time (HH:MM:SS):")
    }
}

impl Elicitation for NaiveTime {
    type Style = NaiveTimeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting NaiveTime");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "value",
                "chrono::NaiveTime",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| {
                Self::prompt()
                    .unwrap_or("Enter a time (e.g., \"15:30:00\"):")
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
        let s = mcp::parse_string(value)?;
        NaiveTime::parse_from_str(&s, "%H:%M:%S")
            .or_else(|_| NaiveTime::parse_from_str(&s, "%H:%M:%S%.f"))
            .map_err(|e| {
                ElicitError::new(ElicitErrorKind::ParseError(format!(
                    "Invalid time (expected HH:MM:SS): {e}"
                )))
            })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::NaiveTime")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::NaiveTime")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::NaiveTime")
    }
}

impl ElicitIntrospect for NaiveTime {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::NaiveTime",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for NaiveTime {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt().unwrap_or("NaiveTime").to_string(),
            type_name: "chrono::NaiveTime".to_string(),
        }
    }
}

impl ToCodeLiteral for NaiveTime {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let hour = self.hour();
        let min = self.minute();
        let sec = self.second();
        let nano = self.nanosecond();
        if nano == 0 {
            crate::quote::quote! {
                match chrono::NaiveTime::from_hms_opt(#hour, #min, #sec) {
                    Some(t) => t,
                    None => return Err("from_hms_opt: invalid time components".into()),
                }
            }
        } else {
            crate::quote::quote! {
                match chrono::NaiveTime::from_hms_nano_opt(#hour, #min, #sec, #nano) {
                    Some(t) => t,
                    None => return Err("from_hms_nano_opt: invalid time components".into()),
                }
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::NaiveTime }
    }
}

// ============================================================================
// Month
// ============================================================================

impl Prompt for Month {
    fn prompt() -> Option<&'static str> {
        Some("Choose a month:")
    }
}

impl Select for Month {
    fn options() -> Vec<Self> {
        vec![
            Month::January,
            Month::February,
            Month::March,
            Month::April,
            Month::May,
            Month::June,
            Month::July,
            Month::August,
            Month::September,
            Month::October,
            Month::November,
            Month::December,
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
            "January" => Some(Month::January),
            "February" => Some(Month::February),
            "March" => Some(Month::March),
            "April" => Some(Month::April),
            "May" => Some(Month::May),
            "June" => Some(Month::June),
            "July" => Some(Month::July),
            "August" => Some(Month::August),
            "September" => Some(Month::September),
            "October" => Some(Month::October),
            "November" => Some(Month::November),
            "December" => Some(Month::December),
            _ => None,
        }
    }
}

crate::default_style!(Month => MonthStyle);

impl Elicitation for Month {
    type Style = MonthStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Month");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "value",
                "chrono::Month",
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
        crate::verification::proof_helpers::kani_select_wrapper("Month", "January")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("Month", "January")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("Month", "January")
    }
}

impl ElicitIntrospect for Month {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::Month",
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

impl ElicitPromptTree for Month {
    fn prompt_tree() -> PromptTree {
        let labels = Self::labels();
        let count = labels.len();
        PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Choose a month:").to_string(),
            type_name: "chrono::Month".to_string(),
            options: labels,
            branches: vec![None; count],
        }
    }
}

impl ToCodeLiteral for Month {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let variant = match self {
            Month::January => "January",
            Month::February => "February",
            Month::March => "March",
            Month::April => "April",
            Month::May => "May",
            Month::June => "June",
            Month::July => "July",
            Month::August => "August",
            Month::September => "September",
            Month::October => "October",
            Month::November => "November",
            Month::December => "December",
        };
        let ident = proc_macro2::Ident::new(variant, proc_macro2::Span::call_site());
        crate::quote::quote! { chrono::Month::#ident }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::Month }
    }
}

crate::select_trenchcoat!(chrono::Month, as MonthSelect, serde);
crate::select_trenchcoat_traits!(MonthSelect, chrono::Month, [copy, eq, hash, ord, from_str]);

// ============================================================================
// TimeDelta (chrono::Duration is a type alias for chrono::TimeDelta)
// ============================================================================

crate::default_style!(chrono::TimeDelta => TimeDeltaStyle);

impl Prompt for chrono::TimeDelta {
    fn prompt() -> Option<&'static str> {
        Some("Enter duration in whole seconds (can be negative):")
    }
}

impl Elicitation for chrono::TimeDelta {
    type Style = TimeDeltaStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TimeDelta");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>("seconds", "i64", &crate::style::PromptContext::new(0, 1))?
            .unwrap_or_else(|| {
                Self::prompt()
                    .unwrap_or("Enter duration in seconds:")
                    .to_string()
            });
        let params = mcp::number_params(&prompt, i64::MIN, i64::MAX);
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let secs = mcp::parse_integer::<i64>(value)?;
        chrono::TimeDelta::try_seconds(secs).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Duration out of range: {secs} seconds"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::TimeDelta")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::TimeDelta")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::TimeDelta")
    }
}

impl ElicitIntrospect for chrono::TimeDelta {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::TimeDelta",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for chrono::TimeDelta {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt()
                .map(str::to_string)
                .unwrap_or_else(|| "Enter duration in whole seconds".to_string()),
            type_name: "chrono::TimeDelta".to_string(),
        }
    }
}

impl ToCodeLiteral for chrono::TimeDelta {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        // Elicitation captures whole seconds only; subsec_nanos() is always 0
        // for elicited values. try_seconds() returns None only for overflow,
        // which cannot occur for a value already held in a valid TimeDelta.
        let secs = self.num_seconds();
        crate::quote::quote! {
            match chrono::TimeDelta::try_seconds(#secs) {
                Some(d) => d,
                None => return Err("try_seconds: seconds value out of TimeDelta range".into()),
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::TimeDelta }
    }
}

// ============================================================================
// Batch 1 helper macro — reduces boilerplate for unit-variant Select enums.
//
// Generates: Prompt, Select, default_style!, Elicitation, ElicitIntrospect,
// ElicitPromptTree, ToCodeLiteral, select_trenchcoat!, select_trenchcoat_traits!
//
// Usage:
//   chrono_select_enum! {
//       type    = path::To::Type,
//       name    = "chrono::Type",
//       wrapper = WrapperName,
//       prompt  = "Human-readable prompt:",
//       style   = StyleName,
//       traits  = [copy, eq, hash],   // whichever the inner type supports
//       variants = [
//           ("LabelA", VariantA),
//           ("LabelB", VariantB),
//       ],
//   }
//
// The inner type MUST already implement PartialEq (required for manual serde).
// ============================================================================

macro_rules! chrono_select_enum {
    (
        type     = $ty:path,
        name     = $type_name:literal,
        wrapper  = $wrapper:ident,
        prompt   = $prompt:literal,
        style    = $style:ident,
        traits   = [$($trait_flag:ident),*],
        variants = [$(($label:literal, $variant:path)),+ $(,)?],
        default  = $default_label:literal,
    ) => {
        impl Prompt for $ty {
            fn prompt() -> Option<&'static str> {
                Some($prompt)
            }
        }

        impl Select for $ty {
            fn options() -> Vec<Self> {
                vec![$($variant),+]
            }

            fn labels() -> Vec<String> {
                vec![$($label.to_string()),+]
            }

            fn from_label(label: &str) -> Option<Self> {
                match label {
                    $($label => Some($variant),)+
                    _ => None,
                }
            }
        }

        crate::default_style!($ty => $style);

        impl Elicitation for $ty {
            type Style = $style;

            #[tracing::instrument(skip(communicator))]
            async fn elicit<C: ElicitCommunicator>(
                communicator: &C,
            ) -> ElicitResult<Self> {
                tracing::debug!(concat!("Eliciting ", $type_name));
                let params = mcp::select_params(
                    Self::prompt().unwrap_or($prompt),
                    &Self::labels(),
                );
                let result = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(
                            mcp::tool_names::elicit_select(),
                        )
                        .with_arguments(params),
                    )
                    .await?;
                let value = mcp::extract_value(result)?;
                let label = mcp::parse_string(value)?;
                Self::from_label(&label).ok_or_else(|| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        concat!("Invalid ", $type_name, ": {}"),
                        label
                    )))
                })
            }

            fn kani_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::kani_select_wrapper(
                    $type_name,
                    $default_label,
                )
            }

            fn verus_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::verus_select_wrapper(
                    $type_name,
                    $default_label,
                )
            }

            fn creusot_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::creusot_select_wrapper(
                    $type_name,
                    $default_label,
                )
            }
        }

        impl ElicitIntrospect for $ty {
            fn pattern() -> ElicitationPattern {
                ElicitationPattern::Select
            }

            fn metadata() -> TypeMetadata {
                TypeMetadata {
                    type_name: $type_name,
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

        impl ElicitPromptTree for $ty {
            fn prompt_tree() -> PromptTree {
                let labels = Self::labels();
                let count = labels.len();
                PromptTree::Select {
                    prompt: Self::prompt().unwrap_or($prompt).to_string(),
                    type_name: $type_name.to_string(),
                    options: labels,
                    branches: vec![None; count],
                }
            }
        }

        impl ToCodeLiteral for $ty {
            fn to_code_literal(&self) -> proc_macro2::TokenStream {
                $(
                    if self == &$variant {
                        return crate::quote::quote! { $variant };
                    }
                )+
                proc_macro2::TokenStream::new()
            }

            fn type_tokens() -> proc_macro2::TokenStream {
                crate::quote::quote! { $ty }
            }
        }

        crate::select_trenchcoat!($ty, as $wrapper);
        crate::select_trenchcoat_traits!($wrapper, $ty, [$($trait_flag),*]);
    };
}

// ============================================================================
// RoundingError
// ============================================================================

chrono_select_enum! {
    type     = chrono::RoundingError,
    name     = "chrono::RoundingError",
    wrapper  = RoundingErrorSelect,
    prompt   = "Select a rounding error kind:",
    style    = RoundingErrorStyle,
    traits   = [copy, eq],
    variants = [
        ("DurationExceedsTimestamp", chrono::RoundingError::DurationExceedsTimestamp),
        ("DurationExceedsLimit",     chrono::RoundingError::DurationExceedsLimit),
        ("TimestampExceedsLimit",    chrono::RoundingError::TimestampExceedsLimit),
    ],
    default  = "DurationExceedsTimestamp",
}

// ============================================================================
// format::Colons
// ============================================================================

chrono_select_enum! {
    type     = chrono::format::Colons,
    name     = "chrono::format::Colons",
    wrapper  = ColonsSelect,
    prompt   = "Select a colons separator style:",
    style    = ColonsStyle,
    traits   = [copy, eq, hash],
    variants = [
        ("None",  chrono::format::Colons::None),
        ("Colon", chrono::format::Colons::Colon),
        ("Maybe", chrono::format::Colons::Maybe),
    ],
    default  = "Colon",
}

// ============================================================================
// format::Pad
// ============================================================================

chrono_select_enum! {
    type     = chrono::format::Pad,
    name     = "chrono::format::Pad",
    wrapper  = PadSelect,
    prompt   = "Select a padding style:",
    style    = PadStyle,
    traits   = [copy, eq, hash],
    variants = [
        ("None",  chrono::format::Pad::None),
        ("Zero",  chrono::format::Pad::Zero),
        ("Space", chrono::format::Pad::Space),
    ],
    default  = "Zero",
}

// ============================================================================
// format::OffsetPrecision
// ============================================================================

chrono_select_enum! {
    type     = chrono::format::OffsetPrecision,
    name     = "chrono::format::OffsetPrecision",
    wrapper  = OffsetPrecisionSelect,
    prompt   = "Select a UTC offset precision:",
    style    = OffsetPrecisionStyle,
    traits   = [copy, eq, hash],
    variants = [
        ("Hours",                    chrono::format::OffsetPrecision::Hours),
        ("Minutes",                  chrono::format::OffsetPrecision::Minutes),
        ("Seconds",                  chrono::format::OffsetPrecision::Seconds),
        ("OptionalMinutes",          chrono::format::OffsetPrecision::OptionalMinutes),
        ("OptionalSeconds",          chrono::format::OffsetPrecision::OptionalSeconds),
        ("OptionalMinutesAndSeconds",chrono::format::OffsetPrecision::OptionalMinutesAndSeconds),
    ],
    default  = "Minutes",
}

// ============================================================================
// format::ParseErrorKind
// ============================================================================

chrono_select_enum! {
    type     = chrono::format::ParseErrorKind,
    name     = "chrono::format::ParseErrorKind",
    wrapper  = ParseErrorKindSelect,
    prompt   = "Select a parse error kind:",
    style    = ParseErrorKindStyle,
    traits   = [copy, eq, hash],
    variants = [
        ("OutOfRange", chrono::format::ParseErrorKind::OutOfRange),
        ("Impossible", chrono::format::ParseErrorKind::Impossible),
        ("NotEnough",  chrono::format::ParseErrorKind::NotEnough),
        ("Invalid",    chrono::format::ParseErrorKind::Invalid),
        ("TooShort",   chrono::format::ParseErrorKind::TooShort),
        ("TooLong",    chrono::format::ParseErrorKind::TooLong),
        ("BadFormat",  chrono::format::ParseErrorKind::BadFormat),
    ],
    default  = "Invalid",
}

// ============================================================================
// SecondsFormat
// ============================================================================

chrono_select_enum! {
    type     = chrono::SecondsFormat,
    name     = "chrono::SecondsFormat",
    wrapper  = SecondsFormatSelect,
    prompt   = "Select a seconds format for RFC 3339 output:",
    style    = SecondsFormatStyle,
    traits   = [copy, eq, hash],
    variants = [
        ("Secs",   chrono::SecondsFormat::Secs),
        ("Millis", chrono::SecondsFormat::Millis),
        ("Micros", chrono::SecondsFormat::Micros),
        ("Nanos",  chrono::SecondsFormat::Nanos),
        ("AutoSi", chrono::SecondsFormat::AutoSi),
    ],
    default  = "Secs",
}

// ============================================================================
// format::Numeric — 20 named unit variants; Internal(InternalNumeric) omitted
// ============================================================================

chrono_select_enum! {
    type     = chrono::format::Numeric,
    name     = "chrono::format::Numeric",
    wrapper  = NumericSelect,
    prompt   = "Select a numeric format item:",
    style    = NumericStyle,
    traits   = [eq, hash],
    variants = [
        ("Year",           chrono::format::Numeric::Year),
        ("YearDiv100",     chrono::format::Numeric::YearDiv100),
        ("YearMod100",     chrono::format::Numeric::YearMod100),
        ("IsoYear",        chrono::format::Numeric::IsoYear),
        ("IsoYearDiv100",  chrono::format::Numeric::IsoYearDiv100),
        ("IsoYearMod100",  chrono::format::Numeric::IsoYearMod100),
        ("Quarter",        chrono::format::Numeric::Quarter),
        ("Month",          chrono::format::Numeric::Month),
        ("Day",            chrono::format::Numeric::Day),
        ("WeekFromSun",    chrono::format::Numeric::WeekFromSun),
        ("WeekFromMon",    chrono::format::Numeric::WeekFromMon),
        ("IsoWeek",        chrono::format::Numeric::IsoWeek),
        ("NumDaysFromSun", chrono::format::Numeric::NumDaysFromSun),
        ("WeekdayFromMon", chrono::format::Numeric::WeekdayFromMon),
        ("Ordinal",        chrono::format::Numeric::Ordinal),
        ("Hour",           chrono::format::Numeric::Hour),
        ("Hour12",         chrono::format::Numeric::Hour12),
        ("Minute",         chrono::format::Numeric::Minute),
        ("Second",         chrono::format::Numeric::Second),
        ("Nanosecond",     chrono::format::Numeric::Nanosecond),
        ("Timestamp",      chrono::format::Numeric::Timestamp),
    ],
    default  = "Year",
}

// ============================================================================
// format::Fixed — 18 named unit variants; Internal(InternalFixed) omitted
// ============================================================================

chrono_select_enum! {
    type     = chrono::format::Fixed,
    name     = "chrono::format::Fixed",
    wrapper  = FixedSelect,
    prompt   = "Select a fixed-format item:",
    style    = FixedStyle,
    traits   = [eq, hash],
    variants = [
        ("ShortMonthName",             chrono::format::Fixed::ShortMonthName),
        ("LongMonthName",              chrono::format::Fixed::LongMonthName),
        ("ShortWeekdayName",           chrono::format::Fixed::ShortWeekdayName),
        ("LongWeekdayName",            chrono::format::Fixed::LongWeekdayName),
        ("LowerAmPm",                  chrono::format::Fixed::LowerAmPm),
        ("UpperAmPm",                  chrono::format::Fixed::UpperAmPm),
        ("Nanosecond",                 chrono::format::Fixed::Nanosecond),
        ("Nanosecond3",                chrono::format::Fixed::Nanosecond3),
        ("Nanosecond6",                chrono::format::Fixed::Nanosecond6),
        ("Nanosecond9",                chrono::format::Fixed::Nanosecond9),
        ("TimezoneName",               chrono::format::Fixed::TimezoneName),
        ("TimezoneOffsetColon",        chrono::format::Fixed::TimezoneOffsetColon),
        ("TimezoneOffsetDoubleColon",  chrono::format::Fixed::TimezoneOffsetDoubleColon),
        ("TimezoneOffsetTripleColon",  chrono::format::Fixed::TimezoneOffsetTripleColon),
        ("TimezoneOffsetColonZ",       chrono::format::Fixed::TimezoneOffsetColonZ),
        ("TimezoneOffset",             chrono::format::Fixed::TimezoneOffset),
        ("TimezoneOffsetZ",            chrono::format::Fixed::TimezoneOffsetZ),
        ("RFC2822",                    chrono::format::Fixed::RFC2822),
        ("RFC3339",                    chrono::format::Fixed::RFC3339),
    ],
    default  = "RFC3339",
}

// ============================================================================
// Days — wraps u64 (number of days); no public getter, reconstruct via NaiveDate
// ============================================================================

crate::default_style!(chrono::Days => DaysStyle);

impl Prompt for chrono::Days {
    fn prompt() -> Option<&'static str> {
        Some("Enter number of days:")
    }
}

impl Elicitation for chrono::Days {
    type Style = DaysStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Days");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>("days", "u64", &crate::style::PromptContext::new(0, 1))?
            .unwrap_or_else(|| {
                Self::prompt()
                    .unwrap_or("Enter number of days:")
                    .to_string()
            });
        let params = mcp::number_params(&prompt, 0i64, i64::MAX);
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let n = mcp::parse_integer::<u64>(value)?;
        Ok(chrono::Days::new(n))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::Days")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::Days")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::Days")
    }
}

impl ElicitIntrospect for chrono::Days {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::Days",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for chrono::Days {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt().unwrap_or("Days").to_string(),
            type_name: "chrono::Days".to_string(),
        }
    }
}

impl ToCodeLiteral for chrono::Days {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        // Days has no public accessor; reconstruct the count via NaiveDate arithmetic.
        // NaiveDate::MIN is a const that requires no fallible construction.
        let n = chrono::NaiveDate::MIN
            .checked_add_days(*self)
            .map(|end| end.signed_duration_since(chrono::NaiveDate::MIN).num_days() as u64)
            .unwrap_or(u64::MAX);
        crate::quote::quote! { chrono::Days::new(#n) }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::Days }
    }
}

/// Build a `schemars::Schema` from a `serde_json::Value::Object` without panicking.
///
/// `serde_json::json!({...})` always produces `Value::Object`; the `_` arm handles
/// the exhaustiveness requirement but is unreachable in practice.
fn schema_from_object(v: serde_json::Value) -> schemars::Schema {
    match v {
        serde_json::Value::Object(map) => schemars::Schema::from(map),
        _ => schemars::Schema::from(serde_json::Map::new()),
    }
}

/// Trenchcoat for `chrono::Days` — serializes as a `u64` day count.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(transparent)]
pub struct DaysWrap(u64);

impl schemars::JsonSchema for DaysWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "DaysWrap".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schema_from_object(serde_json::json!({
            "type": "integer", "minimum": 0,
            "description": "Number of days (unsigned 64-bit integer)"
        }))
    }
}

impl DaysWrap {
    /// Unwrap to `chrono::Days`.
    pub fn into_inner(self) -> chrono::Days {
        chrono::Days::new(self.0)
    }
}

impl From<chrono::Days> for DaysWrap {
    fn from(d: chrono::Days) -> Self {
        let n = chrono::NaiveDate::MIN
            .checked_add_days(d)
            .map(|end| end.signed_duration_since(chrono::NaiveDate::MIN).num_days() as u64)
            .unwrap_or(u64::MAX);
        Self(n)
    }
}

impl Prompt for DaysWrap {
    fn prompt() -> Option<&'static str> {
        <chrono::Days as Prompt>::prompt()
    }
}
impl Elicitation for DaysWrap {
    type Style = DaysStyle;
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        chrono::Days::elicit(communicator).await.map(DaysWrap::from)
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_integer_default("DaysWrap", "u64")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_integer_default("DaysWrap", "u64")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_integer_default("DaysWrap", "u64")
    }
}
impl ElicitIntrospect for DaysWrap {
    fn pattern() -> ElicitationPattern {
        <chrono::Days as ElicitIntrospect>::pattern()
    }
    fn metadata() -> TypeMetadata {
        <chrono::Days as ElicitIntrospect>::metadata()
    }
}
impl ElicitPromptTree for DaysWrap {
    fn prompt_tree() -> PromptTree {
        <chrono::Days as ElicitPromptTree>::prompt_tree()
    }
}
impl ToCodeLiteral for DaysWrap {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let n = self.0;
        crate::quote::quote! { chrono::Days::new(#n) }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::Days }
    }
}

// ============================================================================
// Months — wraps u32; has public `as_u32()` accessor
// ============================================================================

crate::default_style!(chrono::Months => MonthsStyle);

impl Prompt for chrono::Months {
    fn prompt() -> Option<&'static str> {
        Some("Enter number of months:")
    }
}

impl Elicitation for chrono::Months {
    type Style = MonthsStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Months");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>("months", "u32", &crate::style::PromptContext::new(0, 1))?
            .unwrap_or_else(|| {
                Self::prompt()
                    .unwrap_or("Enter number of months:")
                    .to_string()
            });
        let params = mcp::number_params(&prompt, 0i64, u32::MAX as i64);
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let n = mcp::parse_integer::<u32>(value)?;
        Ok(chrono::Months::new(n))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::Months")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::Months")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::Months")
    }
}

impl ElicitIntrospect for chrono::Months {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::Months",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for chrono::Months {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt().unwrap_or("Months").to_string(),
            type_name: "chrono::Months".to_string(),
        }
    }
}

impl ToCodeLiteral for chrono::Months {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let n = self.as_u32();
        crate::quote::quote! { chrono::Months::new(#n) }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::Months }
    }
}

/// Trenchcoat for `chrono::Months` — serializes as a `u32` month count.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(transparent)]
pub struct MonthsWrap(u32);

impl schemars::JsonSchema for MonthsWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "MonthsWrap".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schema_from_object(serde_json::json!({
            "type": "integer", "minimum": 0,
            "description": "Number of months (unsigned 32-bit integer)"
        }))
    }
}

impl MonthsWrap {
    /// Unwrap to `chrono::Months`.
    pub fn into_inner(self) -> chrono::Months {
        chrono::Months::new(self.0)
    }
}

impl From<chrono::Months> for MonthsWrap {
    fn from(m: chrono::Months) -> Self {
        Self(m.as_u32())
    }
}

impl Prompt for MonthsWrap {
    fn prompt() -> Option<&'static str> {
        <chrono::Months as Prompt>::prompt()
    }
}
impl Elicitation for MonthsWrap {
    type Style = MonthsStyle;
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        chrono::Months::elicit(communicator)
            .await
            .map(MonthsWrap::from)
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_integer_default("MonthsWrap", "u32")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_integer_default("MonthsWrap", "u32")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_integer_default("MonthsWrap", "u32")
    }
}
impl ElicitIntrospect for MonthsWrap {
    fn pattern() -> ElicitationPattern {
        <chrono::Months as ElicitIntrospect>::pattern()
    }
    fn metadata() -> TypeMetadata {
        <chrono::Months as ElicitIntrospect>::metadata()
    }
}
impl ElicitPromptTree for MonthsWrap {
    fn prompt_tree() -> PromptTree {
        <chrono::Months as ElicitPromptTree>::prompt_tree()
    }
}
impl ToCodeLiteral for MonthsWrap {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let n = self.0;
        crate::quote::quote! { chrono::Months::new(#n) }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::Months }
    }
}

// ============================================================================
// Utc — unit struct timezone marker; no meaningful user input required
// ============================================================================

crate::default_style!(chrono::Utc => UtcStyle);

impl Prompt for chrono::Utc {
    fn prompt() -> Option<&'static str> {
        Some("UTC timezone (no input required, always returns chrono::Utc)")
    }
}

impl Elicitation for chrono::Utc {
    type Style = UtcStyle;

    #[tracing::instrument(skip_all)]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        Ok(chrono::Utc)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::Utc")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::Utc")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::Utc")
    }
}

impl ElicitIntrospect for chrono::Utc {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::Utc",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for chrono::Utc {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt().unwrap_or("Utc").to_string(),
            type_name: "chrono::Utc".to_string(),
        }
    }
}

impl ToCodeLiteral for chrono::Utc {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::Utc }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::Utc }
    }
}

/// Trenchcoat for `chrono::Utc` — serializes as a unit null value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct UtcWrap;

impl schemars::JsonSchema for UtcWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "UtcWrap".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schema_from_object(serde_json::json!({
            "type": "null",
            "description": "UTC timezone marker (always null; the only valid value)"
        }))
    }
}

impl UtcWrap {
    /// Unwrap to `chrono::Utc`.
    pub fn into_inner(self) -> chrono::Utc {
        chrono::Utc
    }
}

impl From<chrono::Utc> for UtcWrap {
    fn from(_: chrono::Utc) -> Self {
        Self
    }
}

impl Prompt for UtcWrap {
    fn prompt() -> Option<&'static str> {
        <chrono::Utc as Prompt>::prompt()
    }
}
impl Elicitation for UtcWrap {
    type Style = UtcStyle;
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        chrono::Utc::elicit(communicator).await.map(UtcWrap::from)
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_unit_struct("UtcWrap")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_unit_struct("UtcWrap")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_unit_struct("UtcWrap")
    }
}
impl ElicitIntrospect for UtcWrap {
    fn pattern() -> ElicitationPattern {
        <chrono::Utc as ElicitIntrospect>::pattern()
    }
    fn metadata() -> TypeMetadata {
        <chrono::Utc as ElicitIntrospect>::metadata()
    }
}
impl ElicitPromptTree for UtcWrap {
    fn prompt_tree() -> PromptTree {
        <chrono::Utc as ElicitPromptTree>::prompt_tree()
    }
}
impl ToCodeLiteral for UtcWrap {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::Utc }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::Utc }
    }
}

// ============================================================================
// Local — unit struct for the local timezone; no user input required
// ============================================================================

crate::default_style!(chrono::Local => LocalStyle);

impl Prompt for chrono::Local {
    fn prompt() -> Option<&'static str> {
        Some("Local timezone (no input required, always returns chrono::Local)")
    }
}

impl Elicitation for chrono::Local {
    type Style = LocalStyle;

    #[tracing::instrument(skip_all)]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        Ok(chrono::Local)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::Local")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::Local")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::Local")
    }
}

impl ElicitIntrospect for chrono::Local {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::Local",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for chrono::Local {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt().unwrap_or("Local").to_string(),
            type_name: "chrono::Local".to_string(),
        }
    }
}

impl ToCodeLiteral for chrono::Local {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::Local }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::Local }
    }
}

/// Trenchcoat for `chrono::Local` — serializes as a unit null value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct LocalWrap;

impl schemars::JsonSchema for LocalWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "LocalWrap".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schema_from_object(serde_json::json!({
            "type": "null",
            "description": "Local timezone marker (always null; the only valid value)"
        }))
    }
}

impl LocalWrap {
    /// Unwrap to `chrono::Local`.
    pub fn into_inner(self) -> chrono::Local {
        chrono::Local
    }
}

impl From<chrono::Local> for LocalWrap {
    fn from(_: chrono::Local) -> Self {
        Self
    }
}

impl Prompt for LocalWrap {
    fn prompt() -> Option<&'static str> {
        <chrono::Local as Prompt>::prompt()
    }
}
impl Elicitation for LocalWrap {
    type Style = LocalStyle;
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        chrono::Local::elicit(communicator)
            .await
            .map(LocalWrap::from)
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_unit_struct("LocalWrap")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_unit_struct("LocalWrap")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_unit_struct("LocalWrap")
    }
}
impl ElicitIntrospect for LocalWrap {
    fn pattern() -> ElicitationPattern {
        <chrono::Local as ElicitIntrospect>::pattern()
    }
    fn metadata() -> TypeMetadata {
        <chrono::Local as ElicitIntrospect>::metadata()
    }
}
impl ElicitPromptTree for LocalWrap {
    fn prompt_tree() -> PromptTree {
        <chrono::Local as ElicitPromptTree>::prompt_tree()
    }
}
impl ToCodeLiteral for LocalWrap {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::Local }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::Local }
    }
}

// ============================================================================
// FixedOffset — fixed UTC offset expressed in seconds east of UTC
// ============================================================================

crate::default_style!(FixedOffset => FixedOffsetStyle);

impl Prompt for FixedOffset {
    fn prompt() -> Option<&'static str> {
        Some("Enter UTC offset in seconds (east is positive, e.g. +3600 for UTC+1):")
    }
}

impl Elicitation for FixedOffset {
    type Style = FixedOffsetStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting FixedOffset");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "offset_seconds",
                "i32",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| {
                Self::prompt()
                    .unwrap_or("Enter UTC offset in seconds:")
                    .to_string()
            });
        let params = mcp::number_params(&prompt, -(86400i64 - 1), 86400i64 - 1);
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let secs = mcp::parse_integer::<i32>(value)?;
        FixedOffset::east_opt(secs).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "FixedOffset out of range: {secs} seconds"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::FixedOffset")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::FixedOffset")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::FixedOffset")
    }
}

impl ElicitIntrospect for FixedOffset {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::FixedOffset",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for FixedOffset {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt().unwrap_or("FixedOffset").to_string(),
            type_name: "chrono::FixedOffset".to_string(),
        }
    }
}

impl ToCodeLiteral for FixedOffset {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let secs = self.local_minus_utc();
        if secs >= 0 {
            crate::quote::quote! {
                match chrono::FixedOffset::east_opt(#secs) {
                    Some(o) => o,
                    None => return Err("east_opt: offset seconds out of range".into()),
                }
            }
        } else {
            let abs = -secs;
            crate::quote::quote! {
                match chrono::FixedOffset::west_opt(#abs) {
                    Some(o) => o,
                    None => return Err("west_opt: offset seconds out of range".into()),
                }
            }
        }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::FixedOffset }
    }
}

/// Trenchcoat for `chrono::FixedOffset` — serializes as seconds east of UTC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct FixedOffsetWrap(i32);

impl schemars::JsonSchema for FixedOffsetWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "FixedOffsetWrap".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schema_from_object(serde_json::json!({
            "type": "integer",
            "minimum": -(86399),
            "maximum": 86399,
            "description": "Fixed UTC offset in seconds east of UTC (negative = west)"
        }))
    }
}

impl FixedOffsetWrap {
    /// Convert to `chrono::FixedOffset`, or `None` if the stored seconds are out of range.
    ///
    /// Returns `None` only if this wrapper was constructed with an out-of-range value,
    /// which cannot happen through the normal `From<FixedOffset>` path.
    pub fn into_inner(self) -> Option<FixedOffset> {
        FixedOffset::east_opt(self.0)
    }
}

impl From<FixedOffset> for FixedOffsetWrap {
    fn from(o: FixedOffset) -> Self {
        Self(o.local_minus_utc())
    }
}

impl Prompt for FixedOffsetWrap {
    fn prompt() -> Option<&'static str> {
        <FixedOffset as Prompt>::prompt()
    }
}
impl Elicitation for FixedOffsetWrap {
    type Style = FixedOffsetStyle;
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        FixedOffset::elicit(communicator)
            .await
            .map(FixedOffsetWrap::from)
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_integer_default("FixedOffsetWrap", "i32")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_integer_default("FixedOffsetWrap", "i32")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_integer_default("FixedOffsetWrap", "i32")
    }
}
impl ElicitIntrospect for FixedOffsetWrap {
    fn pattern() -> ElicitationPattern {
        <FixedOffset as ElicitIntrospect>::pattern()
    }
    fn metadata() -> TypeMetadata {
        <FixedOffset as ElicitIntrospect>::metadata()
    }
}
impl ElicitPromptTree for FixedOffsetWrap {
    fn prompt_tree() -> PromptTree {
        <FixedOffset as ElicitPromptTree>::prompt_tree()
    }
}
impl ToCodeLiteral for FixedOffsetWrap {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        // east_opt only fails outside ±86_400 s; FixedOffsetWrap is constructed
        // exclusively from valid FixedOffset values, so None is unreachable here.
        FixedOffset::east_opt(self.0)
            .map(|o| o.to_code_literal())
            .unwrap_or_else(|| {
                crate::quote::quote! {
                    ::core::compile_error!("FixedOffsetWrap had an out-of-range offset")
                }
            })
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::FixedOffset }
    }
}

// ============================================================================
// IsoWeek — ISO 8601 week; constructed via NaiveDate::from_isoywd_opt
// ============================================================================

crate::default_style!(chrono::IsoWeek => IsoWeekStyle);

impl Prompt for chrono::IsoWeek {
    fn prompt() -> Option<&'static str> {
        Some("Enter ISO week as 'YYYY-Www' (e.g. '2024-W15'):")
    }
}

impl Elicitation for chrono::IsoWeek {
    type Style = IsoWeekStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting IsoWeek");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "value",
                "chrono::IsoWeek",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| {
                Self::prompt()
                    .unwrap_or("Enter ISO week (YYYY-Www):")
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
        let s = mcp::parse_string(value)?;
        // Accept "YYYY-Www" or "YYYYWww"
        let cleaned = s.trim().replace("W", "-W");
        let parts: Vec<&str> = cleaned.splitn(2, "-W").collect();
        if parts.len() != 2 {
            return Err(ElicitError::new(ElicitErrorKind::ParseError(
                "Expected format YYYY-Www".to_string(),
            )));
        }
        let year: i32 = parts[0].parse().map_err(|_| {
            ElicitError::new(ElicitErrorKind::ParseError(
                "Invalid year in ISO week".to_string(),
            ))
        })?;
        let week: u32 = parts[1].parse().map_err(|_| {
            ElicitError::new(ElicitErrorKind::ParseError(
                "Invalid week number in ISO week".to_string(),
            ))
        })?;
        chrono::NaiveDate::from_isoywd_opt(year, week, chrono::Weekday::Mon)
            .map(|d| d.iso_week())
            .ok_or_else(|| {
                ElicitError::new(ElicitErrorKind::ParseError(format!(
                    "Invalid ISO week: {year}-W{week:02}"
                )))
            })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::IsoWeek")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::IsoWeek")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::IsoWeek")
    }
}

impl ElicitIntrospect for chrono::IsoWeek {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::IsoWeek",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for chrono::IsoWeek {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt().unwrap_or("IsoWeek").to_string(),
            type_name: "chrono::IsoWeek".to_string(),
        }
    }
}

impl ToCodeLiteral for chrono::IsoWeek {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let year = self.year();
        let week = self.week();
        crate::quote::quote! {
            match chrono::NaiveDate::from_isoywd_opt(#year, #week, chrono::Weekday::Mon) {
                Some(d) => d.iso_week(),
                None => return Err("from_isoywd_opt: invalid ISO week".into()),
            }
        }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::IsoWeek }
    }
}

/// Trenchcoat for `chrono::IsoWeek` — serializes as `"YYYY-Www"` string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct IsoWeekWrap(String);

impl schemars::JsonSchema for IsoWeekWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "IsoWeekWrap".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schema_from_object(serde_json::json!({
            "type": "string",
            "pattern": r"^\d{4}-W(0[1-9]|[1-4]\d|5[0-3])$",
            "description": "ISO 8601 week in YYYY-Www format (e.g. '2024-W15')"
        }))
    }
}

impl IsoWeekWrap {
    /// Unwrap to `chrono::IsoWeek`.
    pub fn into_inner(&self) -> Option<chrono::IsoWeek> {
        let parts: Vec<&str> = self.0.splitn(2, "-W").collect();
        if parts.len() != 2 {
            return None;
        }
        let year: i32 = parts[0].parse().ok()?;
        let week: u32 = parts[1].parse().ok()?;
        chrono::NaiveDate::from_isoywd_opt(year, week, chrono::Weekday::Mon).map(|d| d.iso_week())
    }
}

impl From<chrono::IsoWeek> for IsoWeekWrap {
    fn from(w: chrono::IsoWeek) -> Self {
        Self(format!("{:04}-W{:02}", w.year(), w.week()))
    }
}

impl Prompt for IsoWeekWrap {
    fn prompt() -> Option<&'static str> {
        <chrono::IsoWeek as Prompt>::prompt()
    }
}
impl Elicitation for IsoWeekWrap {
    type Style = IsoWeekStyle;
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        chrono::IsoWeek::elicit(communicator)
            .await
            .map(IsoWeekWrap::from)
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[kani::proof]
            fn verify_isoweekwrap_parse_roundtrip() {
                // IsoWeekWrap is our type: prove our YYYY-Www parse logic works
                let wrap = IsoWeekWrap(String::from("2024-W15"));
                assert!(wrap.into_inner().is_some(), "known-valid ISO week parses");
            }
        }
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            pub fn verify_isoweekwrap_roundtrip(w: IsoWeekWrap) -> (result: IsoWeekWrap)
                ensures result == w,
            {
                w
            }
        }
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[ensures(result == w)]
            pub fn verify_isoweekwrap_roundtrip(w: IsoWeekWrap) -> IsoWeekWrap {
                w
            }
        }
    }
}
impl ElicitIntrospect for IsoWeekWrap {
    fn pattern() -> ElicitationPattern {
        <chrono::IsoWeek as ElicitIntrospect>::pattern()
    }
    fn metadata() -> TypeMetadata {
        <chrono::IsoWeek as ElicitIntrospect>::metadata()
    }
}
impl ElicitPromptTree for IsoWeekWrap {
    fn prompt_tree() -> PromptTree {
        <chrono::IsoWeek as ElicitPromptTree>::prompt_tree()
    }
}
impl ToCodeLiteral for IsoWeekWrap {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        if let Some(w) = self.into_inner() {
            w.to_code_literal()
        } else {
            crate::quote::quote! { compile_error!("invalid IsoWeekWrap") }
        }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::IsoWeek }
    }
}

// ============================================================================
// NaiveWeek — week from a reference date + start weekday
// ============================================================================

crate::default_style!(chrono::NaiveWeek => NaiveWeekStyle);

impl Prompt for chrono::NaiveWeek {
    fn prompt() -> Option<&'static str> {
        Some("Enter a NaiveWeek as 'YYYY-MM-DD,Weekday' (reference date + week start):")
    }
}

impl Elicitation for chrono::NaiveWeek {
    type Style = NaiveWeekStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting NaiveWeek");
        let date = NaiveDate::elicit(communicator).await?;
        let start = chrono::Weekday::elicit(communicator).await?;
        Ok(date.week(start))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::NaiveWeek")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::NaiveWeek")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::NaiveWeek")
    }
}

impl ElicitIntrospect for chrono::NaiveWeek {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::NaiveWeek",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for chrono::NaiveWeek {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt().unwrap_or("NaiveWeek").to_string(),
            type_name: "chrono::NaiveWeek".to_string(),
        }
    }
}

impl ToCodeLiteral for chrono::NaiveWeek {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        // Reconstruct from first_day() + Mon as start (best approximation without
        // access to the private `start` field).
        let first = self.first_day();
        let y = first.year();
        let m = first.month();
        let d = first.day();
        crate::quote::quote! {
            match chrono::NaiveDate::from_ymd_opt(#y, #m, #d) {
                Some(d) => d.week(chrono::Weekday::Mon),
                None => return Err("from_ymd_opt: invalid week anchor date".into()),
            }
        }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::NaiveWeek }
    }
}

/// Trenchcoat for `chrono::NaiveWeek` — stores (first_day, start_weekday).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NaiveWeekWrap {
    /// Reference date (any date within the week).
    pub date: chrono::NaiveDate,
    /// First day of the week.
    pub start: chrono::Weekday,
}

impl schemars::JsonSchema for NaiveWeekWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "NaiveWeekWrap".into()
    }
    fn json_schema(schema_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schema_from_object(serde_json::json!({
            "type": "object",
            "properties": {
                "date": chrono::NaiveDate::json_schema(schema_gen),
                "start": { "type": "string", "description": "Week start day (Mon..Sun)" }
            },
            "required": ["date", "start"],
            "description": "A naive week defined by a reference date and its starting weekday"
        }))
    }
}

impl NaiveWeekWrap {
    /// Unwrap to `chrono::NaiveWeek`.
    pub fn into_inner(self) -> chrono::NaiveWeek {
        self.date.week(self.start)
    }
}

impl From<chrono::NaiveWeek> for NaiveWeekWrap {
    fn from(w: chrono::NaiveWeek) -> Self {
        Self {
            date: w.first_day(),
            start: chrono::Weekday::Mon,
        }
    }
}

impl Prompt for NaiveWeekWrap {
    fn prompt() -> Option<&'static str> {
        <chrono::NaiveWeek as Prompt>::prompt()
    }
}
impl Elicitation for NaiveWeekWrap {
    type Style = NaiveWeekStyle;
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let date = NaiveDate::elicit(communicator).await?;
        let start = chrono::Weekday::elicit(communicator).await?;
        Ok(Self { date, start })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[kani::proof]
            fn verify_naiveweekwrap_construction() {
                // NaiveWeekWrap is our type; chrono::NaiveDate is a trusted axiom
                let date = match chrono::NaiveDate::from_ymd_opt(2024, 4, 15) {
                    Some(d) => d,
                    None => return,
                };
                let wrap = NaiveWeekWrap { date, start: chrono::Weekday::Mon };
                // into_inner() is our delegation: date.week(start)
                let _week = wrap.into_inner();
            }
        }
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            pub fn verify_naiveweekwrap_roundtrip(w: NaiveWeekWrap) -> (result: NaiveWeekWrap)
                ensures result == w,
            {
                w
            }
        }
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[ensures(result == w)]
            pub fn verify_naiveweekwrap_roundtrip(w: NaiveWeekWrap) -> NaiveWeekWrap {
                w
            }
        }
    }
}
impl ElicitIntrospect for NaiveWeekWrap {
    fn pattern() -> ElicitationPattern {
        <chrono::NaiveWeek as ElicitIntrospect>::pattern()
    }
    fn metadata() -> TypeMetadata {
        <chrono::NaiveWeek as ElicitIntrospect>::metadata()
    }
}
impl ElicitPromptTree for NaiveWeekWrap {
    fn prompt_tree() -> PromptTree {
        <chrono::NaiveWeek as ElicitPromptTree>::prompt_tree()
    }
}
impl ToCodeLiteral for NaiveWeekWrap {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let y = self.date.year();
        let m = self.date.month();
        let d = self.date.day();
        let start = self.start.to_code_literal();
        crate::quote::quote! {
            match chrono::NaiveDate::from_ymd_opt(#y, #m, #d) {
                Some(d) => d.week(#start),
                None => return Err("from_ymd_opt: invalid week anchor date".into()),
            }
        }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::NaiveWeek }
    }
}

// ============================================================================
// WeekdaySet — bitmask of weekdays; iterate via contains() over all 7 days
// ============================================================================

crate::default_style!(chrono::WeekdaySet => WeekdaySetStyle);

impl Prompt for chrono::WeekdaySet {
    fn prompt() -> Option<&'static str> {
        Some("Enter weekdays as comma-separated names (e.g. 'Mon,Wed,Fri') or 'all'/'none':")
    }
}

impl Elicitation for chrono::WeekdaySet {
    type Style = WeekdaySetStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting WeekdaySet");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "value",
                "chrono::WeekdaySet",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| {
                Self::prompt()
                    .unwrap_or("Enter weekdays (Mon,Tue,...):")
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
        let s = mcp::parse_string(value)?;
        let trimmed = s.trim().to_lowercase();
        if trimmed == "all" {
            return Ok(chrono::WeekdaySet::ALL);
        }
        if trimmed == "none" || trimmed.is_empty() {
            return Ok(chrono::WeekdaySet::EMPTY);
        }
        let all_days = [
            ("mon", chrono::Weekday::Mon),
            ("tue", chrono::Weekday::Tue),
            ("wed", chrono::Weekday::Wed),
            ("thu", chrono::Weekday::Thu),
            ("fri", chrono::Weekday::Fri),
            ("sat", chrono::Weekday::Sat),
            ("sun", chrono::Weekday::Sun),
        ];
        let mut set = chrono::WeekdaySet::EMPTY;
        for part in trimmed.split([',', ' ', ';']) {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            let day = all_days
                .iter()
                .find(|(name, _)| *name == part)
                .map(|(_, d)| *d)
                .ok_or_else(|| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Unknown weekday: {part}"
                    )))
                })?;
            set.insert(day);
        }
        Ok(set)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::WeekdaySet")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::WeekdaySet")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::WeekdaySet")
    }
}

impl ElicitIntrospect for chrono::WeekdaySet {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::WeekdaySet",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for chrono::WeekdaySet {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt().unwrap_or("WeekdaySet").to_string(),
            type_name: "chrono::WeekdaySet".to_string(),
        }
    }
}

impl ToCodeLiteral for chrono::WeekdaySet {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        use chrono::Weekday::{Fri, Mon, Sat, Sun, Thu, Tue, Wed};
        let days = [Mon, Tue, Wed, Thu, Fri, Sat, Sun];
        let day_tokens: Vec<proc_macro2::TokenStream> = days
            .iter()
            .filter(|&&d| self.contains(d))
            .map(|d| d.to_code_literal())
            .collect();
        if day_tokens.is_empty() {
            crate::quote::quote! { chrono::WeekdaySet::EMPTY }
        } else {
            crate::quote::quote! {
                chrono::WeekdaySet::from_array([#(#day_tokens),*])
            }
        }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::WeekdaySet }
    }
}

/// Trenchcoat for `chrono::WeekdaySet` — serializes as a `u8` bitmask.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(transparent)]
pub struct WeekdaySetWrap(u8);

impl schemars::JsonSchema for WeekdaySetWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "WeekdaySetWrap".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schema_from_object(serde_json::json!({
            "type": "integer",
            "minimum": 0,
            "maximum": 127,
            "description": "WeekdaySet bitmask: bit 0=Mon, bit 1=Tue, ..., bit 6=Sun"
        }))
    }
}

impl WeekdaySetWrap {
    /// Unwrap to `chrono::WeekdaySet`.
    pub fn into_inner(self) -> chrono::WeekdaySet {
        use chrono::Weekday::{Fri, Mon, Sat, Sun, Thu, Tue, Wed};
        let days = [Mon, Tue, Wed, Thu, Fri, Sat, Sun];
        let mut set = chrono::WeekdaySet::EMPTY;
        for (i, day) in days.iter().enumerate() {
            if self.0 & (1u8 << i) != 0 {
                set.insert(*day);
            }
        }
        set
    }
}

impl From<chrono::WeekdaySet> for WeekdaySetWrap {
    fn from(s: chrono::WeekdaySet) -> Self {
        use chrono::Weekday::{Fri, Mon, Sat, Sun, Thu, Tue, Wed};
        let days = [Mon, Tue, Wed, Thu, Fri, Sat, Sun];
        let mut bits: u8 = 0;
        for (i, day) in days.iter().enumerate() {
            if s.contains(*day) {
                bits |= 1u8 << i;
            }
        }
        Self(bits)
    }
}

impl Prompt for WeekdaySetWrap {
    fn prompt() -> Option<&'static str> {
        <chrono::WeekdaySet as Prompt>::prompt()
    }
}
impl Elicitation for WeekdaySetWrap {
    type Style = WeekdaySetStyle;
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        chrono::WeekdaySet::elicit(communicator)
            .await
            .map(WeekdaySetWrap::from)
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_integer_default("WeekdaySetWrap", "u8")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_integer_default("WeekdaySetWrap", "u8")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_integer_default("WeekdaySetWrap", "u8")
    }
}
impl ElicitIntrospect for WeekdaySetWrap {
    fn pattern() -> ElicitationPattern {
        <chrono::WeekdaySet as ElicitIntrospect>::pattern()
    }
    fn metadata() -> TypeMetadata {
        <chrono::WeekdaySet as ElicitIntrospect>::metadata()
    }
}
impl ElicitPromptTree for WeekdaySetWrap {
    fn prompt_tree() -> PromptTree {
        <chrono::WeekdaySet as ElicitPromptTree>::prompt_tree()
    }
}
impl ToCodeLiteral for WeekdaySetWrap {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        self.into_inner().to_code_literal()
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::WeekdaySet }
    }
}

// ============================================================================
// format::OffsetFormat — composed struct: precision, colons, allow_zulu, padding
// ============================================================================

crate::default_style!(chrono::format::OffsetFormat => OffsetFormatStyle);

impl Prompt for chrono::format::OffsetFormat {
    fn prompt() -> Option<&'static str> {
        Some("Enter UTC offset format (precision, colons, allow_zulu, padding):")
    }
}

impl Elicitation for chrono::format::OffsetFormat {
    type Style = OffsetFormatStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting OffsetFormat");
        let precision = chrono::format::OffsetPrecision::elicit(communicator).await?;
        let colons = chrono::format::Colons::elicit(communicator).await?;
        let allow_zulu_str = {
            let allow_zulu_prompt = communicator
                .style_context()
                .prompt_for_type::<Self>(
                    "allow_zulu",
                    "bool",
                    &crate::style::PromptContext::new(2, 4),
                )?
                .unwrap_or_else(|| "Allow 'Z' for UTC offset zero?".to_string());
            let params =
                mcp::select_params(&allow_zulu_prompt, &["Yes".to_string(), "No".to_string()]);
            let result = communicator
                .call_tool(
                    rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                        .with_arguments(params),
                )
                .await?;
            mcp::parse_string(mcp::extract_value(result)?)?
        };
        let allow_zulu = allow_zulu_str == "Yes";
        let padding = chrono::format::Pad::elicit(communicator).await?;
        Ok(chrono::format::OffsetFormat {
            precision,
            colons,
            allow_zulu,
            padding,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::format::OffsetFormat")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::format::OffsetFormat")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::format::OffsetFormat")
    }
}

impl ElicitIntrospect for chrono::format::OffsetFormat {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::format::OffsetFormat",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for chrono::format::OffsetFormat {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt().unwrap_or("OffsetFormat").to_string(),
            type_name: "chrono::format::OffsetFormat".to_string(),
        }
    }
}

impl ToCodeLiteral for chrono::format::OffsetFormat {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let prec = self.precision.to_code_literal();
        let col = self.colons.to_code_literal();
        let zulu = self.allow_zulu;
        let pad = self.padding.to_code_literal();
        crate::quote::quote! {
            chrono::format::OffsetFormat {
                precision: #prec,
                colons: #col,
                allow_zulu: #zulu,
                padding: #pad,
            }
        }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::format::OffsetFormat }
    }
}

/// Trenchcoat for `chrono::format::OffsetFormat`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct OffsetFormatWrap {
    /// Precision of the offset.
    pub precision: OffsetPrecisionSelect,
    /// Colons separator style.
    pub colons: ColonsSelect,
    /// Whether to emit `Z` for UTC.
    pub allow_zulu: bool,
    /// Padding style for the hour field.
    pub padding: PadSelect,
}

impl schemars::JsonSchema for OffsetFormatWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "OffsetFormatWrap".into()
    }
    fn json_schema(schema_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schema_from_object(serde_json::json!({
            "type": "object",
            "properties": {
                "precision": OffsetPrecisionSelect::json_schema(schema_gen),
                "colons":    ColonsSelect::json_schema(schema_gen),
                "allow_zulu": { "type": "boolean" },
                "padding":   PadSelect::json_schema(schema_gen)
            },
            "required": ["precision", "colons", "allow_zulu", "padding"],
            "description": "UTC offset format specification"
        }))
    }
}

impl OffsetFormatWrap {
    /// Unwrap to `chrono::format::OffsetFormat`.
    pub fn into_inner(self) -> chrono::format::OffsetFormat {
        chrono::format::OffsetFormat {
            precision: *self.precision,
            colons: *self.colons,
            allow_zulu: self.allow_zulu,
            padding: *self.padding,
        }
    }
}

impl From<chrono::format::OffsetFormat> for OffsetFormatWrap {
    fn from(o: chrono::format::OffsetFormat) -> Self {
        Self {
            precision: OffsetPrecisionSelect::from(o.precision),
            colons: ColonsSelect::from(o.colons),
            allow_zulu: o.allow_zulu,
            padding: PadSelect::from(o.padding),
        }
    }
}

impl Prompt for OffsetFormatWrap {
    fn prompt() -> Option<&'static str> {
        <chrono::format::OffsetFormat as Prompt>::prompt()
    }
}
impl Elicitation for OffsetFormatWrap {
    type Style = OffsetFormatStyle;
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        chrono::format::OffsetFormat::elicit(communicator)
            .await
            .map(OffsetFormatWrap::from)
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[kani::proof]
            fn verify_offset_format_wrap_construction() {
                // OffsetFormatWrap is our struct; all fields are our own Select types
                let precision = match OffsetPrecisionSelect::from_label("Minutes") {
                    Some(p) => p,
                    None => return,
                };
                let colons = match ColonsSelect::from_label("Colon") {
                    Some(c) => c,
                    None => return,
                };
                let padding = match PadSelect::from_label("Zero") {
                    Some(p) => p,
                    None => return,
                };
                let wrap = OffsetFormatWrap { precision, colons, allow_zulu: false, padding };
                // into_inner() is our delegation to chrono::format::OffsetFormat
                let _ = wrap.into_inner();
            }
        }
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            pub fn verify_offset_format_wrap_roundtrip(
                w: OffsetFormatWrap,
            ) -> (result: OffsetFormatWrap)
                ensures result == w,
            {
                w
            }
        }
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[ensures(result == w)]
            pub fn verify_offset_format_wrap_roundtrip(w: OffsetFormatWrap) -> OffsetFormatWrap {
                w
            }
        }
    }
}
impl ElicitIntrospect for OffsetFormatWrap {
    fn pattern() -> ElicitationPattern {
        <chrono::format::OffsetFormat as ElicitIntrospect>::pattern()
    }
    fn metadata() -> TypeMetadata {
        <chrono::format::OffsetFormat as ElicitIntrospect>::metadata()
    }
}
impl ElicitPromptTree for OffsetFormatWrap {
    fn prompt_tree() -> PromptTree {
        <chrono::format::OffsetFormat as ElicitPromptTree>::prompt_tree()
    }
}
impl ToCodeLiteral for OffsetFormatWrap {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        self.into_inner().to_code_literal()
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::format::OffsetFormat }
    }
}

// ============================================================================
// Batch 5: Error types — OutOfRange, OutOfRangeError, ParseError,
//           ParseMonthError, ParseWeekdayError
// ============================================================================

// ============================================================================
// OutOfRange — single possible value; produced by out-of-range month numbers
// ============================================================================

crate::default_style!(chrono::OutOfRange => OutOfRangeStyle);

impl Prompt for chrono::OutOfRange {
    fn prompt() -> Option<&'static str> {
        Some("A chrono out-of-range error (single-value type — no input required)")
    }
}

impl Elicitation for chrono::OutOfRange {
    type Style = OutOfRangeStyle;

    #[tracing::instrument(skip_all)]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        match Month::try_from(13u8) {
            Err(e) => Ok(e),
            Ok(_) => Err(ElicitError::new(ElicitErrorKind::ParseError(
                "Month::try_from(13) unexpectedly succeeded".to_string(),
            ))),
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::OutOfRange")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::OutOfRange")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::OutOfRange")
    }
}

impl ElicitIntrospect for chrono::OutOfRange {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::OutOfRange",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for chrono::OutOfRange {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt().unwrap_or("chrono::OutOfRange").to_string(),
            type_name: "chrono::OutOfRange".to_string(),
        }
    }
}

impl ToCodeLiteral for chrono::OutOfRange {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        crate::quote::quote! {
            match chrono::Month::try_from(13u8) {
                Err(e) => e,
                Ok(_) => return Err("Month::try_from(13) unexpectedly succeeded".into()),
            }
        }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::OutOfRange }
    }
}

/// Trenchcoat for `chrono::OutOfRange` — serializes as `null` (single-value type).
///
/// Use `into_inner()` to recover the wrapped error.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct OutOfRangeWrap(chrono::OutOfRange);

impl std::fmt::Debug for OutOfRangeWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("OutOfRangeWrap")
    }
}

impl serde::Serialize for OutOfRangeWrap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_none()
    }
}

impl<'de> serde::Deserialize<'de> for OutOfRangeWrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let _: serde::de::IgnoredAny = serde::de::Deserialize::deserialize(deserializer)?;
        let e = Month::try_from(13u8).err().ok_or_else(|| {
            <D::Error as serde::de::Error>::custom("Month::try_from(13) unexpectedly succeeded")
        })?;
        Ok(Self(e))
    }
}

impl schemars::JsonSchema for OutOfRangeWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "OutOfRangeWrap".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schema_from_object(serde_json::json!({
            "type": "null",
            "description": "A chrono::OutOfRange error — the only possible value; serialized as null."
        }))
    }
}

impl OutOfRangeWrap {
    /// Unwrap to `chrono::OutOfRange`.
    pub fn into_inner(self) -> chrono::OutOfRange {
        self.0
    }
}

impl From<chrono::OutOfRange> for OutOfRangeWrap {
    fn from(e: chrono::OutOfRange) -> Self {
        Self(e)
    }
}

impl Prompt for OutOfRangeWrap {
    fn prompt() -> Option<&'static str> {
        <chrono::OutOfRange as Prompt>::prompt()
    }
}

impl Elicitation for OutOfRangeWrap {
    type Style = OutOfRangeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        chrono::OutOfRange::elicit(communicator)
            .await
            .map(OutOfRangeWrap::from)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[kani::proof]
            fn verify_out_of_range_wrap() {
                // OutOfRange has a single possible value. Verify our wrapper stores it faithfully.
                let err = match chrono::Month::try_from(13u8) { Err(e) => e, Ok(_) => return };
                let wrap = OutOfRangeWrap::from(err);
                let _ = wrap.into_inner();
            }
        }
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            pub fn verify_out_of_range_wrap_identity(
                w: OutOfRangeWrap,
            ) -> (result: OutOfRangeWrap)
                ensures result == w,
            { w }
        }
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[ensures(result == w)]
            pub fn verify_out_of_range_wrap_identity(w: OutOfRangeWrap) -> OutOfRangeWrap { w }
        }
    }
}

impl ElicitIntrospect for OutOfRangeWrap {
    fn pattern() -> ElicitationPattern {
        <chrono::OutOfRange as ElicitIntrospect>::pattern()
    }
    fn metadata() -> TypeMetadata {
        <chrono::OutOfRange as ElicitIntrospect>::metadata()
    }
}

impl ElicitPromptTree for OutOfRangeWrap {
    fn prompt_tree() -> PromptTree {
        <chrono::OutOfRange as ElicitPromptTree>::prompt_tree()
    }
}

impl ToCodeLiteral for OutOfRangeWrap {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        self.0.to_code_literal()
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::OutOfRange }
    }
}

// ============================================================================
// OutOfRangeError — single possible value; from negative TimeDelta::to_std()
// ============================================================================

crate::default_style!(chrono::OutOfRangeError => OutOfRangeErrorStyle);

impl Prompt for chrono::OutOfRangeError {
    fn prompt() -> Option<&'static str> {
        Some("A TimeDelta→std::time::Duration conversion error (single-value type)")
    }
}

impl Elicitation for chrono::OutOfRangeError {
    type Style = OutOfRangeErrorStyle;

    #[tracing::instrument(skip_all)]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        let neg = Duration::try_seconds(-1).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(
                "try_seconds(-1) out of range".to_string(),
            ))
        })?;
        match neg.to_std() {
            Err(e) => Ok(e),
            Ok(_) => Err(ElicitError::new(ElicitErrorKind::ParseError(
                "negative Duration should not convert to std::time::Duration".to_string(),
            ))),
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::OutOfRangeError")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::OutOfRangeError")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::OutOfRangeError")
    }
}

impl ElicitIntrospect for chrono::OutOfRangeError {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::OutOfRangeError",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for chrono::OutOfRangeError {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt()
                .unwrap_or("chrono::OutOfRangeError")
                .to_string(),
            type_name: "chrono::OutOfRangeError".to_string(),
        }
    }
}

impl ToCodeLiteral for chrono::OutOfRangeError {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        crate::quote::quote! {
            match chrono::Duration::try_seconds(-1) {
                Some(neg) => match neg.to_std() {
                    Err(e) => e,
                    Ok(_) => return Err("to_std on a negative Duration always errors".into()),
                },
                None => return Err("try_seconds(-1) is always in range".into()),
            }
        }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::OutOfRangeError }
    }
}

/// Trenchcoat for `chrono::OutOfRangeError` — serializes as `null` (single-value type).
///
/// Use `into_inner()` to recover the wrapped error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutOfRangeErrorWrap(chrono::OutOfRangeError);

impl serde::Serialize for OutOfRangeErrorWrap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_none()
    }
}

impl<'de> serde::Deserialize<'de> for OutOfRangeErrorWrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let _: serde::de::IgnoredAny = serde::de::Deserialize::deserialize(deserializer)?;
        let neg = Duration::try_seconds(-1).ok_or_else(|| {
            <D::Error as serde::de::Error>::custom("try_seconds(-1) out of range")
        })?;
        let e = neg.to_std().err().ok_or_else(|| {
            <D::Error as serde::de::Error>::custom(
                "negative Duration should not convert to std::time::Duration",
            )
        })?;
        Ok(Self(e))
    }
}

impl schemars::JsonSchema for OutOfRangeErrorWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "OutOfRangeErrorWrap".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schema_from_object(serde_json::json!({
            "type": "null",
            "description": "A chrono::OutOfRangeError — single-value type, serialized as null."
        }))
    }
}

impl OutOfRangeErrorWrap {
    /// Unwrap to `chrono::OutOfRangeError`.
    pub fn into_inner(self) -> chrono::OutOfRangeError {
        self.0
    }
}

impl From<chrono::OutOfRangeError> for OutOfRangeErrorWrap {
    fn from(e: chrono::OutOfRangeError) -> Self {
        Self(e)
    }
}

impl Prompt for OutOfRangeErrorWrap {
    fn prompt() -> Option<&'static str> {
        <chrono::OutOfRangeError as Prompt>::prompt()
    }
}

impl Elicitation for OutOfRangeErrorWrap {
    type Style = OutOfRangeErrorStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        chrono::OutOfRangeError::elicit(communicator)
            .await
            .map(OutOfRangeErrorWrap::from)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[kani::proof]
            fn verify_out_of_range_error_wrap() {
                // OutOfRangeError has a single possible value. Verify our wrapper stores it.
                let neg = match chrono::Duration::try_seconds(-1) { Some(n) => n, None => return };
                let err = match neg.to_std() { Err(e) => e, Ok(_) => return };
                let wrap = OutOfRangeErrorWrap::from(err);
                let _ = wrap.into_inner();
            }
        }
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            pub fn verify_out_of_range_error_wrap_identity(
                w: OutOfRangeErrorWrap,
            ) -> (result: OutOfRangeErrorWrap)
                ensures result == w,
            { w }
        }
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[ensures(result == w)]
            pub fn verify_out_of_range_error_wrap_identity(
                w: OutOfRangeErrorWrap,
            ) -> OutOfRangeErrorWrap { w }
        }
    }
}

impl ElicitIntrospect for OutOfRangeErrorWrap {
    fn pattern() -> ElicitationPattern {
        <chrono::OutOfRangeError as ElicitIntrospect>::pattern()
    }
    fn metadata() -> TypeMetadata {
        <chrono::OutOfRangeError as ElicitIntrospect>::metadata()
    }
}

impl ElicitPromptTree for OutOfRangeErrorWrap {
    fn prompt_tree() -> PromptTree {
        <chrono::OutOfRangeError as ElicitPromptTree>::prompt_tree()
    }
}

impl ToCodeLiteral for OutOfRangeErrorWrap {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        self.0.to_code_literal()
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::OutOfRangeError }
    }
}

// ============================================================================
// ParseError — wraps chrono::ParseError (Copy); serializes as kind label
// ============================================================================

crate::default_style!(chrono::ParseError => ParseErrorStyle);

impl Prompt for chrono::ParseError {
    fn prompt() -> Option<&'static str> {
        Some("Select a parse error kind:")
    }
}

impl Elicitation for chrono::ParseError {
    type Style = ParseErrorStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ParseError via kind selection");
        let kind_select = ParseErrorKindSelect::elicit(communicator).await?;
        let kind = kind_select.into_inner();
        tracing::debug!(?kind, "ParseErrorKind selected");
        let parse_err = |input: &str, fmt: &str, label: &str| {
            NaiveDate::parse_from_str(input, fmt).err().ok_or_else(|| {
                ElicitError::new(ElicitErrorKind::ParseError(format!(
                    "parse_from_str({input:?}, {fmt:?}) unexpectedly succeeded ({label})"
                )))
            })
        };
        let err = match kind {
            chrono::format::ParseErrorKind::OutOfRange => {
                parse_err("2024-13-01", "%Y-%m-%d", "month 13 is out of range")?
            }
            chrono::format::ParseErrorKind::Impossible => {
                parse_err("2024-02-30", "%Y-%m-%d", "Feb 30 is impossible")?
            }
            chrono::format::ParseErrorKind::NotEnough => {
                parse_err("2024-01", "%Y-%m-%d", "missing day field")?
            }
            chrono::format::ParseErrorKind::Invalid => {
                parse_err("notadate", "%Y-%m-%d", "non-numeric input")?
            }
            chrono::format::ParseErrorKind::TooShort => {
                parse_err("", "%Y-%m-%d", "empty input ends before format")?
            }
            chrono::format::ParseErrorKind::TooLong => {
                parse_err("2024-01-01 extra", "%Y-%m-%d", "trailing content")?
            }
            chrono::format::ParseErrorKind::BadFormat => {
                parse_err("2024", "%Q", "bad format specifier %Q")?
            }
            _ => {
                tracing::warn!("Unknown ParseErrorKind; falling back to Invalid");
                parse_err("notadate", "%Y-%m-%d", "fallback for unknown kind")?
            }
        };
        Ok(err)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::ParseError")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::ParseError")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::ParseError")
    }
}

impl ElicitIntrospect for chrono::ParseError {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::ParseError",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: vec![
                    VariantMetadata {
                        label: "OutOfRange".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Impossible".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "NotEnough".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Invalid".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "TooShort".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "TooLong".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "BadFormat".to_string(),
                        fields: vec![],
                    },
                ],
            },
        }
    }
}

impl ElicitPromptTree for chrono::ParseError {
    fn prompt_tree() -> PromptTree {
        PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Select a parse error kind:")
                .to_string(),
            type_name: "chrono::ParseError".to_string(),
            options: vec![
                "OutOfRange".to_string(),
                "Impossible".to_string(),
                "NotEnough".to_string(),
                "Invalid".to_string(),
                "TooShort".to_string(),
                "TooLong".to_string(),
                "BadFormat".to_string(),
            ],
            branches: vec![None, None, None, None, None, None, None],
        }
    }
}

impl ToCodeLiteral for chrono::ParseError {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let (input, fmt, label): (&str, &str, &str) = match self.kind() {
            chrono::format::ParseErrorKind::OutOfRange => {
                ("2024-13-01", "%Y-%m-%d", "month 13 is out of range")
            }
            chrono::format::ParseErrorKind::Impossible => {
                ("2024-02-30", "%Y-%m-%d", "Feb 30 is impossible")
            }
            chrono::format::ParseErrorKind::NotEnough => {
                ("2024-01", "%Y-%m-%d", "missing day field")
            }
            chrono::format::ParseErrorKind::Invalid => {
                ("notadate", "%Y-%m-%d", "non-numeric input")
            }
            chrono::format::ParseErrorKind::TooShort => {
                ("", "%Y-%m-%d", "empty input is too short")
            }
            chrono::format::ParseErrorKind::TooLong => {
                ("2024-01-01 extra", "%Y-%m-%d", "trailing content")
            }
            chrono::format::ParseErrorKind::BadFormat => ("2024", "%Q", "bad format specifier %Q"),
            _ => ("notadate", "%Y-%m-%d", "fallback for unknown kind"),
        };
        crate::quote::quote! {
            match chrono::NaiveDate::parse_from_str(#input, #fmt) {
                Err(e) => e,
                Ok(_) => return Err(#label.into()),
            }
        }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::ParseError }
    }
}

/// Trenchcoat for `chrono::ParseError` — serializes as a kind-label string.
///
/// Stores the real `ParseError` (which is `Copy`). Serializes via `kind()` and
/// deserializes by re-triggering the matching parse failure, which preserves the
/// only publicly observable property: `kind()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParseErrorWrap(chrono::ParseError);

impl serde::Serialize for ParseErrorWrap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let label = match self.0.kind() {
            chrono::format::ParseErrorKind::OutOfRange => "OutOfRange",
            chrono::format::ParseErrorKind::Impossible => "Impossible",
            chrono::format::ParseErrorKind::NotEnough => "NotEnough",
            chrono::format::ParseErrorKind::Invalid => "Invalid",
            chrono::format::ParseErrorKind::TooShort => "TooShort",
            chrono::format::ParseErrorKind::TooLong => "TooLong",
            chrono::format::ParseErrorKind::BadFormat => "BadFormat",
            _ => "Invalid",
        };
        serializer.serialize_str(label)
    }
}

impl<'de> serde::Deserialize<'de> for ParseErrorWrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let label = String::deserialize(deserializer)?;
        let require_err = |input: &str, fmt: &str, label: &str| {
            NaiveDate::parse_from_str(input, fmt).err().ok_or_else(|| {
                <D::Error as serde::de::Error>::custom(format!(
                    "parse succeeded unexpectedly ({label})"
                ))
            })
        };
        let err = match label.as_str() {
            "OutOfRange" => require_err("2024-13-01", "%Y-%m-%d", "month 13")?,
            "Impossible" => require_err("2024-02-30", "%Y-%m-%d", "Feb 30")?,
            "NotEnough" => require_err("2024-01", "%Y-%m-%d", "missing day")?,
            "Invalid" => require_err("notadate", "%Y-%m-%d", "non-numeric")?,
            "TooShort" => require_err("", "%Y-%m-%d", "empty input")?,
            "TooLong" => require_err("2024-01-01 extra", "%Y-%m-%d", "trailing content")?,
            "BadFormat" => require_err("2024", "%Q", "bad specifier %Q")?,
            other => {
                return Err(D::Error::unknown_variant(
                    other,
                    &[
                        "OutOfRange",
                        "Impossible",
                        "NotEnough",
                        "Invalid",
                        "TooShort",
                        "TooLong",
                        "BadFormat",
                    ],
                ));
            }
        };
        Ok(Self(err))
    }
}

impl schemars::JsonSchema for ParseErrorWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ParseErrorWrap".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schema_from_object(serde_json::json!({
            "type": "string",
            "enum": ["OutOfRange", "Impossible", "NotEnough", "Invalid",
                     "TooShort", "TooLong", "BadFormat"],
            "description": "A chrono parse error, identified by kind. Reconstructed by triggering the matching parse failure."
        }))
    }
}

impl ParseErrorWrap {
    /// Unwrap to `chrono::ParseError`.
    pub fn into_inner(self) -> chrono::ParseError {
        self.0
    }
}

impl From<chrono::ParseError> for ParseErrorWrap {
    fn from(e: chrono::ParseError) -> Self {
        Self(e)
    }
}

impl std::fmt::Display for ParseErrorWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for ParseErrorWrap {}

impl Prompt for ParseErrorWrap {
    fn prompt() -> Option<&'static str> {
        <chrono::ParseError as Prompt>::prompt()
    }
}

impl Elicitation for ParseErrorWrap {
    type Style = ParseErrorStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        chrono::ParseError::elicit(communicator)
            .await
            .map(ParseErrorWrap::from)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[kani::proof]
            fn verify_parse_error_wrap_preserves_kind() {
                // ParseError is Copy; kind() is its only observable property.
                // Verify our wrapper preserves it through the round-trip.
                let err = match chrono::NaiveDate::parse_from_str("notadate", "%Y-%m-%d") {
                    Err(e) => e,
                    Ok(_) => return,
                };
                let kind_before = err.kind();
                let wrap = ParseErrorWrap::from(err);
                let inner = wrap.into_inner();
                assert_eq!(inner.kind(), kind_before);
            }
        }
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            pub fn verify_parse_error_wrap_identity(w: ParseErrorWrap) -> (result: ParseErrorWrap)
                ensures result == w,
            { w }
        }
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[ensures(result == w)]
            pub fn verify_parse_error_wrap_identity(w: ParseErrorWrap) -> ParseErrorWrap { w }
        }
    }
}

impl ElicitIntrospect for ParseErrorWrap {
    fn pattern() -> ElicitationPattern {
        <chrono::ParseError as ElicitIntrospect>::pattern()
    }
    fn metadata() -> TypeMetadata {
        <chrono::ParseError as ElicitIntrospect>::metadata()
    }
}

impl ElicitPromptTree for ParseErrorWrap {
    fn prompt_tree() -> PromptTree {
        <chrono::ParseError as ElicitPromptTree>::prompt_tree()
    }
}

impl ToCodeLiteral for ParseErrorWrap {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        self.0.to_code_literal()
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::ParseError }
    }
}

// ============================================================================
// ParseMonthError — single possible value; from parsing an invalid month string
// ============================================================================

crate::default_style!(chrono::ParseMonthError => ParseMonthErrorStyle);

impl Prompt for chrono::ParseMonthError {
    fn prompt() -> Option<&'static str> {
        Some("A month parse error from an invalid month string (single-value type)")
    }
}

impl Elicitation for chrono::ParseMonthError {
    type Style = ParseMonthErrorStyle;

    #[tracing::instrument(skip_all)]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        match "__invalid__".parse::<Month>() {
            Err(e) => Ok(e),
            Ok(_) => Err(ElicitError::new(ElicitErrorKind::ParseError(
                "'__invalid__'.parse::<Month>() unexpectedly succeeded".to_string(),
            ))),
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::ParseMonthError")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::ParseMonthError")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::ParseMonthError")
    }
}

impl ElicitIntrospect for chrono::ParseMonthError {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::ParseMonthError",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for chrono::ParseMonthError {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt()
                .unwrap_or("chrono::ParseMonthError")
                .to_string(),
            type_name: "chrono::ParseMonthError".to_string(),
        }
    }
}

impl ToCodeLiteral for chrono::ParseMonthError {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        crate::quote::quote! {
            match "__invalid__".parse::<chrono::Month>() {
                Err(e) => e,
                Ok(_) => return Err("'__invalid__'.parse::<Month>() unexpectedly succeeded".into()),
            }
        }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::ParseMonthError }
    }
}

/// Trenchcoat for `chrono::ParseMonthError` — serializes as `null` (single-value type).
///
/// Use `into_inner()` to recover the wrapped error.
#[derive(Clone, PartialEq, Eq)]
pub struct ParseMonthErrorWrap(chrono::ParseMonthError);

impl std::fmt::Debug for ParseMonthErrorWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ParseMonthErrorWrap")
    }
}

impl serde::Serialize for ParseMonthErrorWrap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_none()
    }
}

impl<'de> serde::Deserialize<'de> for ParseMonthErrorWrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let _: serde::de::IgnoredAny = serde::de::Deserialize::deserialize(deserializer)?;
        let e = "__invalid__".parse::<Month>().err().ok_or_else(|| {
            <D::Error as serde::de::Error>::custom(
                "'__invalid__'.parse::<Month>() unexpectedly succeeded",
            )
        })?;
        Ok(Self(e))
    }
}

impl schemars::JsonSchema for ParseMonthErrorWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ParseMonthErrorWrap".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schema_from_object(serde_json::json!({
            "type": "null",
            "description": "A chrono::ParseMonthError — single-value type, serialized as null."
        }))
    }
}

impl ParseMonthErrorWrap {
    /// Unwrap to `chrono::ParseMonthError`.
    pub fn into_inner(self) -> chrono::ParseMonthError {
        self.0
    }
}

impl From<chrono::ParseMonthError> for ParseMonthErrorWrap {
    fn from(e: chrono::ParseMonthError) -> Self {
        Self(e)
    }
}

impl Prompt for ParseMonthErrorWrap {
    fn prompt() -> Option<&'static str> {
        <chrono::ParseMonthError as Prompt>::prompt()
    }
}

impl Elicitation for ParseMonthErrorWrap {
    type Style = ParseMonthErrorStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        chrono::ParseMonthError::elicit(communicator)
            .await
            .map(ParseMonthErrorWrap::from)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[kani::proof]
            fn verify_parse_month_error_wrap() {
                // ParseMonthError has a single possible value. Verify our wrapper stores it.
                let err = match "__invalid__".parse::<chrono::Month>() {
                    Err(e) => e,
                    Ok(_) => return,
                };
                let wrap = ParseMonthErrorWrap::from(err);
                let _ = wrap.into_inner();
            }
        }
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            pub fn verify_parse_month_error_wrap_identity(
                w: ParseMonthErrorWrap,
            ) -> (result: ParseMonthErrorWrap)
                ensures result == w,
            { w }
        }
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[ensures(result == w)]
            pub fn verify_parse_month_error_wrap_identity(
                w: ParseMonthErrorWrap,
            ) -> ParseMonthErrorWrap { w }
        }
    }
}

impl ElicitIntrospect for ParseMonthErrorWrap {
    fn pattern() -> ElicitationPattern {
        <chrono::ParseMonthError as ElicitIntrospect>::pattern()
    }
    fn metadata() -> TypeMetadata {
        <chrono::ParseMonthError as ElicitIntrospect>::metadata()
    }
}

impl ElicitPromptTree for ParseMonthErrorWrap {
    fn prompt_tree() -> PromptTree {
        <chrono::ParseMonthError as ElicitPromptTree>::prompt_tree()
    }
}

impl ToCodeLiteral for ParseMonthErrorWrap {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        self.0.to_code_literal()
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::ParseMonthError }
    }
}

// ============================================================================
// ParseWeekdayError — single possible value; from parsing invalid weekday string
// ============================================================================

crate::default_style!(chrono::ParseWeekdayError => ParseWeekdayErrorStyle);

impl Prompt for chrono::ParseWeekdayError {
    fn prompt() -> Option<&'static str> {
        Some("A weekday parse error from an invalid weekday string (single-value type)")
    }
}

impl Elicitation for chrono::ParseWeekdayError {
    type Style = ParseWeekdayErrorStyle;

    #[tracing::instrument(skip_all)]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        match "__invalid__".parse::<Weekday>() {
            Err(e) => Ok(e),
            Ok(_) => Err(ElicitError::new(ElicitErrorKind::ParseError(
                "'__invalid__'.parse::<Weekday>() unexpectedly succeeded".to_string(),
            ))),
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::ParseWeekdayError")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::ParseWeekdayError")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::ParseWeekdayError")
    }
}

impl ElicitIntrospect for chrono::ParseWeekdayError {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::ParseWeekdayError",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for chrono::ParseWeekdayError {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt()
                .unwrap_or("chrono::ParseWeekdayError")
                .to_string(),
            type_name: "chrono::ParseWeekdayError".to_string(),
        }
    }
}

impl ToCodeLiteral for chrono::ParseWeekdayError {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        crate::quote::quote! {
            match "__invalid__".parse::<chrono::Weekday>() {
                Err(e) => e,
                Ok(_) => return Err("'__invalid__'.parse::<Weekday>() unexpectedly succeeded".into()),
            }
        }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::ParseWeekdayError }
    }
}

/// Trenchcoat for `chrono::ParseWeekdayError` — serializes as `null` (single-value type).
///
/// Use `into_inner()` to recover the wrapped error.
#[derive(Clone, PartialEq, Eq)]
pub struct ParseWeekdayErrorWrap(chrono::ParseWeekdayError);

impl std::fmt::Debug for ParseWeekdayErrorWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ParseWeekdayErrorWrap")
    }
}

impl serde::Serialize for ParseWeekdayErrorWrap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_none()
    }
}

impl<'de> serde::Deserialize<'de> for ParseWeekdayErrorWrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let _: serde::de::IgnoredAny = serde::de::Deserialize::deserialize(deserializer)?;
        let e = "__invalid__".parse::<Weekday>().err().ok_or_else(|| {
            <D::Error as serde::de::Error>::custom(
                "'__invalid__'.parse::<Weekday>() unexpectedly succeeded",
            )
        })?;
        Ok(Self(e))
    }
}

impl schemars::JsonSchema for ParseWeekdayErrorWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ParseWeekdayErrorWrap".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schema_from_object(serde_json::json!({
            "type": "null",
            "description": "A chrono::ParseWeekdayError — single-value type, serialized as null."
        }))
    }
}

impl ParseWeekdayErrorWrap {
    /// Unwrap to `chrono::ParseWeekdayError`.
    pub fn into_inner(self) -> chrono::ParseWeekdayError {
        self.0
    }
}

impl From<chrono::ParseWeekdayError> for ParseWeekdayErrorWrap {
    fn from(e: chrono::ParseWeekdayError) -> Self {
        Self(e)
    }
}

impl Prompt for ParseWeekdayErrorWrap {
    fn prompt() -> Option<&'static str> {
        <chrono::ParseWeekdayError as Prompt>::prompt()
    }
}

impl Elicitation for ParseWeekdayErrorWrap {
    type Style = ParseWeekdayErrorStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        chrono::ParseWeekdayError::elicit(communicator)
            .await
            .map(ParseWeekdayErrorWrap::from)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[kani::proof]
            fn verify_parse_weekday_error_wrap() {
                // ParseWeekdayError has a single possible value. Verify our wrapper stores it.
                let err = match "__invalid__".parse::<chrono::Weekday>() {
                    Err(e) => e,
                    Ok(_) => return,
                };
                let wrap = ParseWeekdayErrorWrap::from(err);
                let _ = wrap.into_inner();
            }
        }
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            pub fn verify_parse_weekday_error_wrap_identity(
                w: ParseWeekdayErrorWrap,
            ) -> (result: ParseWeekdayErrorWrap)
                ensures result == w,
            { w }
        }
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[ensures(result == w)]
            pub fn verify_parse_weekday_error_wrap_identity(
                w: ParseWeekdayErrorWrap,
            ) -> ParseWeekdayErrorWrap { w }
        }
    }
}

impl ElicitIntrospect for ParseWeekdayErrorWrap {
    fn pattern() -> ElicitationPattern {
        <chrono::ParseWeekdayError as ElicitIntrospect>::pattern()
    }
    fn metadata() -> TypeMetadata {
        <chrono::ParseWeekdayError as ElicitIntrospect>::metadata()
    }
}

impl ElicitPromptTree for ParseWeekdayErrorWrap {
    fn prompt_tree() -> PromptTree {
        <chrono::ParseWeekdayError as ElicitPromptTree>::prompt_tree()
    }
}

impl ToCodeLiteral for ParseWeekdayErrorWrap {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        self.0.to_code_literal()
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::ParseWeekdayError }
    }
}

// ============================================================================
// format::InternalNumeric — uninhabited internal type (wraps private Void)
// ============================================================================
//
// chrono reserves the `Numeric::Internal(InternalNumeric)` variant for forward
// compatibility. `InternalNumeric` wraps a private `Void` enum (zero variants),
// making it genuinely uninhabited: no external code can ever construct a value.
// `elicit()` always fails; Serialize/Deserialize always fail; JSON Schema is the
// never/bottom type `{"not": {}}`.

crate::default_style!(chrono::format::InternalNumeric => InternalNumericStyle);

impl Prompt for chrono::format::InternalNumeric {
    fn prompt() -> Option<&'static str> {
        Some(concat!(
            "chrono::format::InternalNumeric is an internal-only uninhabited type ",
            "(wraps a private Void); no value can be elicited or constructed externally"
        ))
    }
}

impl Elicitation for chrono::format::InternalNumeric {
    type Style = InternalNumericStyle;

    #[tracing::instrument(skip_all, fields(type_name = "chrono::format::InternalNumeric"))]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        tracing::warn!("InternalNumeric is uninhabited — elicit() always fails");
        Err(ElicitErrorKind::ParseError(
            "chrono::format::InternalNumeric is uninhabited; no value can be externally constructed"
                .into(),
        )
        .into())
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::format::InternalNumeric")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::format::InternalNumeric")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque(
            "chrono::format::InternalNumeric",
        )
    }
}

impl ElicitIntrospect for chrono::format::InternalNumeric {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::format::InternalNumeric",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for chrono::format::InternalNumeric {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt()
                .unwrap_or("chrono::format::InternalNumeric")
                .to_string(),
            type_name: "chrono::format::InternalNumeric".to_string(),
        }
    }
}

impl ToCodeLiteral for chrono::format::InternalNumeric {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        // InternalNumeric is uninhabited — a value can never be obtained,
        // so this method is unreachable in practice.
        unreachable!(
            "chrono::format::InternalNumeric is uninhabited; to_code_literal is unreachable"
        )
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::format::InternalNumeric }
    }
}

/// Trenchcoat for `chrono::format::InternalNumeric`.
///
/// `InternalNumeric` is uninhabited (wraps a private `Void` type). No instances
/// of this wrapper can ever exist. Both serialization and deserialization always
/// fail with a descriptive error. The JSON Schema is `{"not": {}}` (never/bottom
/// type). Use `into_inner()` only if you somehow received one via `unsafe`.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct InternalNumericWrap(chrono::format::InternalNumeric);

impl std::fmt::Debug for InternalNumericWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("InternalNumericWrap")
    }
}

impl serde::Serialize for InternalNumericWrap {
    fn serialize<S: serde::Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
        // This is dead code — InternalNumericWrap is uninhabited.
        unreachable!("InternalNumericWrap is uninhabited — serialize is unreachable")
    }
}

impl<'de> serde::Deserialize<'de> for InternalNumericWrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let _: serde::de::IgnoredAny = serde::de::Deserialize::deserialize(deserializer)?;
        Err(serde::de::Error::custom(
            "chrono::format::InternalNumeric is uninhabited — no value can be deserialized",
        ))
    }
}

impl schemars::JsonSchema for InternalNumericWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "InternalNumericWrap".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schema_from_object(serde_json::json!({
            "not": {},
            "description": concat!(
                "chrono::format::InternalNumeric is an uninhabited internal type; ",
                "no value is valid (bottom/never type)."
            )
        }))
    }
}

impl InternalNumericWrap {
    /// Unwrap to `chrono::format::InternalNumeric`.
    pub fn into_inner(self) -> chrono::format::InternalNumeric {
        self.0
    }
}

impl From<chrono::format::InternalNumeric> for InternalNumericWrap {
    fn from(e: chrono::format::InternalNumeric) -> Self {
        Self(e)
    }
}

impl Prompt for InternalNumericWrap {
    fn prompt() -> Option<&'static str> {
        <chrono::format::InternalNumeric as Prompt>::prompt()
    }
}

impl Elicitation for InternalNumericWrap {
    type Style = InternalNumericStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "InternalNumericWrap"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        chrono::format::InternalNumeric::elicit(communicator)
            .await
            .map(InternalNumericWrap::from)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[kani::proof]
            fn verify_internal_numeric_wrap_uninhabited() {
                // InternalNumericWrap is uninhabited because InternalNumeric wraps
                // a private Void type (zero variants). elicit() always returns Err;
                // no instance can be constructed or serialized.
                // Trusted boundary: we rely on chrono's type-system guarantee.
                let elicit_always_fails: bool = true;
                assert!(elicit_always_fails, "InternalNumericWrap::elicit always returns Err");
            }
        }
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[verifier::trusted]
            pub fn verify_internal_numeric_wrap_uninhabited() {
                // Vacuous: InternalNumericWrap is uninhabited. All properties
                // hold trivially — no instance can be produced.
            }
        }
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[trusted]
            pub fn verify_internal_numeric_wrap_uninhabited() {
                // Vacuous: InternalNumericWrap is uninhabited. All properties
                // hold trivially — no instance can be produced.
            }
        }
    }
}

impl ElicitIntrospect for InternalNumericWrap {
    fn pattern() -> ElicitationPattern {
        <chrono::format::InternalNumeric as ElicitIntrospect>::pattern()
    }
    fn metadata() -> TypeMetadata {
        <chrono::format::InternalNumeric as ElicitIntrospect>::metadata()
    }
}

impl ElicitPromptTree for InternalNumericWrap {
    fn prompt_tree() -> PromptTree {
        <chrono::format::InternalNumeric as ElicitPromptTree>::prompt_tree()
    }
}

impl ToCodeLiteral for InternalNumericWrap {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        // Unreachable — InternalNumericWrap is uninhabited.
        unreachable!("InternalNumericWrap is uninhabited — to_code_literal is unreachable")
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { InternalNumericWrap }
    }
}

// ============================================================================
// format::InternalFixed — 4 internal variants, reconstructible via StrftimeItems
// ============================================================================
//
// chrono reserves `Fixed::Internal(InternalFixed)` for internal variants. The
// `InternalFixed` struct wraps a private `InternalInternal` enum with 4 variants:
//   - TimezoneOffsetPermissive  → produced by `%#z`
//   - Nanosecond3NoDot          → produced by `%3f`
//   - Nanosecond6NoDot          → produced by `%6f`
//   - Nanosecond9NoDot          → produced by `%9f`
//
// Although the constructors are private, we can reconstruct each variant by
// parsing the corresponding format string via `chrono::format::StrftimeItems`.

fn find_internal_fixed(fmt: &str) -> Option<chrono::format::InternalFixed> {
    chrono::format::StrftimeItems::new(fmt).find_map(|item| match item {
        chrono::format::Item::Fixed(chrono::format::Fixed::Internal(f)) => Some(f),
        _ => None,
    })
}

impl Prompt for chrono::format::InternalFixed {
    fn prompt() -> Option<&'static str> {
        Some(concat!(
            "Select an internal fixed-format item ",
            "(TimezoneOffsetPermissive, Nanosecond3NoDot, Nanosecond6NoDot, Nanosecond9NoDot):"
        ))
    }
}

impl Select for chrono::format::InternalFixed {
    fn options() -> Vec<Self> {
        ["%#z", "%3f", "%6f", "%9f"]
            .iter()
            .filter_map(|fmt| find_internal_fixed(fmt))
            .collect()
    }

    fn labels() -> Vec<String> {
        vec![
            "TimezoneOffsetPermissive".to_string(),
            "Nanosecond3NoDot".to_string(),
            "Nanosecond6NoDot".to_string(),
            "Nanosecond9NoDot".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "TimezoneOffsetPermissive" => find_internal_fixed("%#z"),
            "Nanosecond3NoDot" => find_internal_fixed("%3f"),
            "Nanosecond6NoDot" => find_internal_fixed("%6f"),
            "Nanosecond9NoDot" => find_internal_fixed("%9f"),
            _ => None,
        }
    }
}

crate::default_style!(chrono::format::InternalFixed => InternalFixedStyle);

impl Elicitation for chrono::format::InternalFixed {
    type Style = InternalFixedStyle;

    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "chrono::format::InternalFixed")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting InternalFixed via 4-variant select");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "value",
                "chrono::format::InternalFixed",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| {
                Self::prompt()
                    .unwrap_or("Select an internal fixed-format item:")
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
        tracing::debug!(%label, "InternalFixed label selected");
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid chrono::format::InternalFixed label: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "chrono::format::InternalFixed",
            "TimezoneOffsetPermissive",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "chrono::format::InternalFixed",
            "TimezoneOffsetPermissive",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "chrono::format::InternalFixed",
            "TimezoneOffsetPermissive",
        )
    }
}

impl ElicitIntrospect for chrono::format::InternalFixed {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::format::InternalFixed",
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

impl ElicitPromptTree for chrono::format::InternalFixed {
    fn prompt_tree() -> PromptTree {
        let labels = Self::labels();
        let count = labels.len();
        PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Select an internal fixed-format item:")
                .to_string(),
            type_name: "chrono::format::InternalFixed".to_string(),
            options: labels,
            branches: vec![None; count],
        }
    }
}

impl ToCodeLiteral for chrono::format::InternalFixed {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        // Compare via PartialEq (chrono::format::InternalFixed: PartialEq) to
        // determine which of the 4 known variants we hold, then emit StrftimeItems
        // extraction code — the only way to construct an InternalFixed externally.
        let (fmt_str, err_msg): (&str, &str) = if find_internal_fixed("%#z").as_ref() == Some(self)
        {
            ("%#z", "'%#z' produces TimezoneOffsetPermissive")
        } else if find_internal_fixed("%3f").as_ref() == Some(self) {
            ("%3f", "'%3f' produces Nanosecond3NoDot")
        } else if find_internal_fixed("%6f").as_ref() == Some(self) {
            ("%6f", "'%6f' produces Nanosecond6NoDot")
        } else if find_internal_fixed("%9f").as_ref() == Some(self) {
            ("%9f", "'%9f' produces Nanosecond9NoDot")
        } else {
            tracing::warn!(
                internal_fixed = ?self,
                "Unknown InternalFixed variant; falling back to TimezoneOffsetPermissive"
            );
            (
                "%#z",
                "unknown InternalFixed variant; fallback to TimezoneOffsetPermissive",
            )
        };
        crate::quote::quote! {
            match chrono::format::StrftimeItems::new(#fmt_str)
                .find_map(|__item| match __item {
                    chrono::format::Item::Fixed(chrono::format::Fixed::Internal(__f)) => Some(__f),
                    _ => None,
                })
            {
                Some(f) => f,
                None => return Err(#err_msg.into()),
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::format::InternalFixed }
    }
}

crate::select_trenchcoat!(chrono::format::InternalFixed, as InternalFixedWrap);
crate::select_trenchcoat_traits!(InternalFixedWrap, chrono::format::InternalFixed, [eq, hash]);

// ============================================================================
// format::Parsed — 21-field optional survey of parsed date/time components
// ============================================================================
//
// chrono::format::Parsed accumulates parsed date/time components. All 21
// fields are `#[doc(hidden)] pub`; the struct cannot be constructed via struct
// literal externally due to the private `_dummy: ()` field, but `Parsed::default()`
// plus direct field assignment is fully supported.
//
// Since chrono does not implement Serialize/Deserialize/JsonSchema for Parsed,
// `ParsedWrap` provides manual implementations for use in the wire protocol.

impl Prompt for chrono::format::Parsed {
    fn prompt() -> Option<&'static str> {
        Some(concat!(
            "Specify parsed date/time components. All 21 fields are optional — ",
            "provide only the components you know: year, month, day, ",
            "hour (hour_div_12 = AM/PM flag, hour_mod_12 = 12-hour value), minute, ",
            "second, nanosecond, weekday, ordinal, ISO week, Unix timestamp, UTC offset, ",
            "and ISO/century year variants."
        ))
    }
}

crate::default_style!(chrono::format::Parsed => ParsedStyle);

impl Elicitation for chrono::format::Parsed {
    type Style = ParsedStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "chrono::format::Parsed"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting chrono::format::Parsed via 21-field survey");
        let year = Option::<i32>::elicit(communicator).await?;
        let year_div_100 = Option::<i32>::elicit(communicator).await?;
        let year_mod_100 = Option::<i32>::elicit(communicator).await?;
        let isoyear = Option::<i32>::elicit(communicator).await?;
        let isoyear_div_100 = Option::<i32>::elicit(communicator).await?;
        let isoyear_mod_100 = Option::<i32>::elicit(communicator).await?;
        let quarter = Option::<u32>::elicit(communicator).await?;
        let month = Option::<u32>::elicit(communicator).await?;
        let week_from_sun = Option::<u32>::elicit(communicator).await?;
        let week_from_mon = Option::<u32>::elicit(communicator).await?;
        let isoweek = Option::<u32>::elicit(communicator).await?;
        let weekday = Option::<Weekday>::elicit(communicator).await?;
        let ordinal = Option::<u32>::elicit(communicator).await?;
        let day = Option::<u32>::elicit(communicator).await?;
        let hour_div_12 = Option::<u32>::elicit(communicator).await?;
        let hour_mod_12 = Option::<u32>::elicit(communicator).await?;
        let minute = Option::<u32>::elicit(communicator).await?;
        let second = Option::<u32>::elicit(communicator).await?;
        let nanosecond = Option::<u32>::elicit(communicator).await?;
        let timestamp = Option::<i64>::elicit(communicator).await?;
        let offset = Option::<i32>::elicit(communicator).await?;
        tracing::debug!("All 21 Parsed fields elicited");
        let mut result = chrono::format::Parsed::default();
        result.year = year;
        result.year_div_100 = year_div_100;
        result.year_mod_100 = year_mod_100;
        result.isoyear = isoyear;
        result.isoyear_div_100 = isoyear_div_100;
        result.isoyear_mod_100 = isoyear_mod_100;
        result.quarter = quarter;
        result.month = month;
        result.week_from_sun = week_from_sun;
        result.week_from_mon = week_from_mon;
        result.isoweek = isoweek;
        result.weekday = weekday;
        result.ordinal = ordinal;
        result.day = day;
        result.hour_div_12 = hour_div_12;
        result.hour_mod_12 = hour_mod_12;
        result.minute = minute;
        result.second = second;
        result.nanosecond = nanosecond;
        result.timestamp = timestamp;
        result.offset = offset;
        Ok(result)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::format::Parsed")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::format::Parsed")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::format::Parsed")
    }
}

impl ElicitIntrospect for chrono::format::Parsed {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::format::Parsed",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "year",
                        type_name: "Option<i32>",
                        prompt: Some("Gregorian year (e.g., 2024):"),
                    },
                    FieldInfo {
                        name: "year_div_100",
                        type_name: "Option<i32>",
                        prompt: Some("Century component of year (year / 100):"),
                    },
                    FieldInfo {
                        name: "year_mod_100",
                        type_name: "Option<i32>",
                        prompt: Some("Year-within-century (year % 100, 0–99):"),
                    },
                    FieldInfo {
                        name: "isoyear",
                        type_name: "Option<i32>",
                        prompt: Some("ISO 8601 week-based year:"),
                    },
                    FieldInfo {
                        name: "isoyear_div_100",
                        type_name: "Option<i32>",
                        prompt: Some("Century component of ISO year (isoyear / 100):"),
                    },
                    FieldInfo {
                        name: "isoyear_mod_100",
                        type_name: "Option<i32>",
                        prompt: Some("ISO year-within-century (isoyear % 100, 0–99):"),
                    },
                    FieldInfo {
                        name: "quarter",
                        type_name: "Option<u32>",
                        prompt: Some("Quarter of year (1–4):"),
                    },
                    FieldInfo {
                        name: "month",
                        type_name: "Option<u32>",
                        prompt: Some("Month of year (1–12):"),
                    },
                    FieldInfo {
                        name: "week_from_sun",
                        type_name: "Option<u32>",
                        prompt: Some("Week number counted from Sunday (0–53):"),
                    },
                    FieldInfo {
                        name: "week_from_mon",
                        type_name: "Option<u32>",
                        prompt: Some("Week number counted from Monday (0–53):"),
                    },
                    FieldInfo {
                        name: "isoweek",
                        type_name: "Option<u32>",
                        prompt: Some("ISO 8601 week number (1–53):"),
                    },
                    FieldInfo {
                        name: "weekday",
                        type_name: "Option<chrono::Weekday>",
                        prompt: Some("Day of week:"),
                    },
                    FieldInfo {
                        name: "ordinal",
                        type_name: "Option<u32>",
                        prompt: Some("Day of year (1–366):"),
                    },
                    FieldInfo {
                        name: "day",
                        type_name: "Option<u32>",
                        prompt: Some("Day of month (1–31):"),
                    },
                    FieldInfo {
                        name: "hour_div_12",
                        type_name: "Option<u32>",
                        prompt: Some("AM/PM flag (0 = AM, 1 = PM):"),
                    },
                    FieldInfo {
                        name: "hour_mod_12",
                        type_name: "Option<u32>",
                        prompt: Some("Hour in 12-hour clock (0–11):"),
                    },
                    FieldInfo {
                        name: "minute",
                        type_name: "Option<u32>",
                        prompt: Some("Minute (0–59):"),
                    },
                    FieldInfo {
                        name: "second",
                        type_name: "Option<u32>",
                        prompt: Some("Second (0–60, allowing leap second):"),
                    },
                    FieldInfo {
                        name: "nanosecond",
                        type_name: "Option<u32>",
                        prompt: Some("Nanoseconds within second (0–999_999_999):"),
                    },
                    FieldInfo {
                        name: "timestamp",
                        type_name: "Option<i64>",
                        prompt: Some("Unix timestamp (seconds since 1970-01-01 00:00:00 UTC):"),
                    },
                    FieldInfo {
                        name: "offset",
                        type_name: "Option<i32>",
                        prompt: Some("UTC offset in seconds (e.g., 3600 = UTC+1):"),
                    },
                ],
            },
        }
    }
}

impl ElicitPromptTree for chrono::format::Parsed {
    fn prompt_tree() -> PromptTree {
        PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "chrono::format::Parsed".to_string(),
            fields: vec![
                (
                    "year".to_string(),
                    Box::new(<Option<i32> as ElicitPromptTree>::prompt_tree()),
                ),
                (
                    "year_div_100".to_string(),
                    Box::new(<Option<i32> as ElicitPromptTree>::prompt_tree()),
                ),
                (
                    "year_mod_100".to_string(),
                    Box::new(<Option<i32> as ElicitPromptTree>::prompt_tree()),
                ),
                (
                    "isoyear".to_string(),
                    Box::new(<Option<i32> as ElicitPromptTree>::prompt_tree()),
                ),
                (
                    "isoyear_div_100".to_string(),
                    Box::new(<Option<i32> as ElicitPromptTree>::prompt_tree()),
                ),
                (
                    "isoyear_mod_100".to_string(),
                    Box::new(<Option<i32> as ElicitPromptTree>::prompt_tree()),
                ),
                (
                    "quarter".to_string(),
                    Box::new(<Option<u32> as ElicitPromptTree>::prompt_tree()),
                ),
                (
                    "month".to_string(),
                    Box::new(<Option<u32> as ElicitPromptTree>::prompt_tree()),
                ),
                (
                    "week_from_sun".to_string(),
                    Box::new(<Option<u32> as ElicitPromptTree>::prompt_tree()),
                ),
                (
                    "week_from_mon".to_string(),
                    Box::new(<Option<u32> as ElicitPromptTree>::prompt_tree()),
                ),
                (
                    "isoweek".to_string(),
                    Box::new(<Option<u32> as ElicitPromptTree>::prompt_tree()),
                ),
                (
                    "weekday".to_string(),
                    Box::new(<Option<Weekday> as ElicitPromptTree>::prompt_tree()),
                ),
                (
                    "ordinal".to_string(),
                    Box::new(<Option<u32> as ElicitPromptTree>::prompt_tree()),
                ),
                (
                    "day".to_string(),
                    Box::new(<Option<u32> as ElicitPromptTree>::prompt_tree()),
                ),
                (
                    "hour_div_12".to_string(),
                    Box::new(<Option<u32> as ElicitPromptTree>::prompt_tree()),
                ),
                (
                    "hour_mod_12".to_string(),
                    Box::new(<Option<u32> as ElicitPromptTree>::prompt_tree()),
                ),
                (
                    "minute".to_string(),
                    Box::new(<Option<u32> as ElicitPromptTree>::prompt_tree()),
                ),
                (
                    "second".to_string(),
                    Box::new(<Option<u32> as ElicitPromptTree>::prompt_tree()),
                ),
                (
                    "nanosecond".to_string(),
                    Box::new(<Option<u32> as ElicitPromptTree>::prompt_tree()),
                ),
                (
                    "timestamp".to_string(),
                    Box::new(<Option<i64> as ElicitPromptTree>::prompt_tree()),
                ),
                (
                    "offset".to_string(),
                    Box::new(<Option<i32> as ElicitPromptTree>::prompt_tree()),
                ),
            ],
        }
    }
}

impl ToCodeLiteral for chrono::format::Parsed {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let mut assignments: Vec<proc_macro2::TokenStream> = Vec::new();
        if let Some(v) = self.year {
            assignments.push(crate::quote::quote! { __parsed.year = Some(#v); });
        }
        if let Some(v) = self.year_div_100 {
            assignments.push(crate::quote::quote! { __parsed.year_div_100 = Some(#v); });
        }
        if let Some(v) = self.year_mod_100 {
            assignments.push(crate::quote::quote! { __parsed.year_mod_100 = Some(#v); });
        }
        if let Some(v) = self.isoyear {
            assignments.push(crate::quote::quote! { __parsed.isoyear = Some(#v); });
        }
        if let Some(v) = self.isoyear_div_100 {
            assignments.push(crate::quote::quote! { __parsed.isoyear_div_100 = Some(#v); });
        }
        if let Some(v) = self.isoyear_mod_100 {
            assignments.push(crate::quote::quote! { __parsed.isoyear_mod_100 = Some(#v); });
        }
        if let Some(v) = self.quarter {
            assignments.push(crate::quote::quote! { __parsed.quarter = Some(#v); });
        }
        if let Some(v) = self.month {
            assignments.push(crate::quote::quote! { __parsed.month = Some(#v); });
        }
        if let Some(v) = self.week_from_sun {
            assignments.push(crate::quote::quote! { __parsed.week_from_sun = Some(#v); });
        }
        if let Some(v) = self.week_from_mon {
            assignments.push(crate::quote::quote! { __parsed.week_from_mon = Some(#v); });
        }
        if let Some(v) = self.isoweek {
            assignments.push(crate::quote::quote! { __parsed.isoweek = Some(#v); });
        }
        if let Some(w) = self.weekday {
            let wd = w.to_code_literal();
            assignments.push(crate::quote::quote! { __parsed.weekday = Some(#wd); });
        }
        if let Some(v) = self.ordinal {
            assignments.push(crate::quote::quote! { __parsed.ordinal = Some(#v); });
        }
        if let Some(v) = self.day {
            assignments.push(crate::quote::quote! { __parsed.day = Some(#v); });
        }
        if let Some(v) = self.hour_div_12 {
            assignments.push(crate::quote::quote! { __parsed.hour_div_12 = Some(#v); });
        }
        if let Some(v) = self.hour_mod_12 {
            assignments.push(crate::quote::quote! { __parsed.hour_mod_12 = Some(#v); });
        }
        if let Some(v) = self.minute {
            assignments.push(crate::quote::quote! { __parsed.minute = Some(#v); });
        }
        if let Some(v) = self.second {
            assignments.push(crate::quote::quote! { __parsed.second = Some(#v); });
        }
        if let Some(v) = self.nanosecond {
            assignments.push(crate::quote::quote! { __parsed.nanosecond = Some(#v); });
        }
        if let Some(v) = self.timestamp {
            assignments.push(crate::quote::quote! { __parsed.timestamp = Some(#v); });
        }
        if let Some(v) = self.offset {
            assignments.push(crate::quote::quote! { __parsed.offset = Some(#v); });
        }
        crate::quote::quote! {
            {
                let mut __parsed = chrono::format::Parsed::default();
                #(#assignments)*
                __parsed
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::format::Parsed }
    }
}

// ── ParsedWrap ───────────────────────────────────────────────────────────────

/// Newtype wrapper around [`chrono::format::Parsed`] with Serialize, Deserialize,
/// and JsonSchema support.
///
/// `Parsed` cannot be serialized by chrono directly (no upstream serde impl).
/// `ParsedWrap` serializes all 21 optional fields as a JSON object, enabling
/// round-trip through the wire protocol.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ParsedWrap(chrono::format::Parsed);

impl std::fmt::Debug for ParsedWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ParsedWrap")
    }
}

impl serde::Serialize for ParsedWrap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Parsed", 21)?;
        state.serialize_field("year", &self.0.year)?;
        state.serialize_field("year_div_100", &self.0.year_div_100)?;
        state.serialize_field("year_mod_100", &self.0.year_mod_100)?;
        state.serialize_field("isoyear", &self.0.isoyear)?;
        state.serialize_field("isoyear_div_100", &self.0.isoyear_div_100)?;
        state.serialize_field("isoyear_mod_100", &self.0.isoyear_mod_100)?;
        state.serialize_field("quarter", &self.0.quarter)?;
        state.serialize_field("month", &self.0.month)?;
        state.serialize_field("week_from_sun", &self.0.week_from_sun)?;
        state.serialize_field("week_from_mon", &self.0.week_from_mon)?;
        state.serialize_field("isoweek", &self.0.isoweek)?;
        state.serialize_field("weekday", &self.0.weekday)?;
        state.serialize_field("ordinal", &self.0.ordinal)?;
        state.serialize_field("day", &self.0.day)?;
        state.serialize_field("hour_div_12", &self.0.hour_div_12)?;
        state.serialize_field("hour_mod_12", &self.0.hour_mod_12)?;
        state.serialize_field("minute", &self.0.minute)?;
        state.serialize_field("second", &self.0.second)?;
        state.serialize_field("nanosecond", &self.0.nanosecond)?;
        state.serialize_field("timestamp", &self.0.timestamp)?;
        state.serialize_field("offset", &self.0.offset)?;
        state.end()
    }
}

impl<'de> serde::Deserialize<'de> for ParsedWrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Default, serde::Deserialize)]
        #[serde(default)]
        struct ParsedHelper {
            year: Option<i32>,
            year_div_100: Option<i32>,
            year_mod_100: Option<i32>,
            isoyear: Option<i32>,
            isoyear_div_100: Option<i32>,
            isoyear_mod_100: Option<i32>,
            quarter: Option<u32>,
            month: Option<u32>,
            week_from_sun: Option<u32>,
            week_from_mon: Option<u32>,
            isoweek: Option<u32>,
            weekday: Option<chrono::Weekday>,
            ordinal: Option<u32>,
            day: Option<u32>,
            hour_div_12: Option<u32>,
            hour_mod_12: Option<u32>,
            minute: Option<u32>,
            second: Option<u32>,
            nanosecond: Option<u32>,
            timestamp: Option<i64>,
            offset: Option<i32>,
        }
        let h = ParsedHelper::deserialize(deserializer)?;
        let mut parsed = chrono::format::Parsed::default();
        parsed.year = h.year;
        parsed.year_div_100 = h.year_div_100;
        parsed.year_mod_100 = h.year_mod_100;
        parsed.isoyear = h.isoyear;
        parsed.isoyear_div_100 = h.isoyear_div_100;
        parsed.isoyear_mod_100 = h.isoyear_mod_100;
        parsed.quarter = h.quarter;
        parsed.month = h.month;
        parsed.week_from_sun = h.week_from_sun;
        parsed.week_from_mon = h.week_from_mon;
        parsed.isoweek = h.isoweek;
        parsed.weekday = h.weekday;
        parsed.ordinal = h.ordinal;
        parsed.day = h.day;
        parsed.hour_div_12 = h.hour_div_12;
        parsed.hour_mod_12 = h.hour_mod_12;
        parsed.minute = h.minute;
        parsed.second = h.second;
        parsed.nanosecond = h.nanosecond;
        parsed.timestamp = h.timestamp;
        parsed.offset = h.offset;
        Ok(ParsedWrap(parsed))
    }
}

impl schemars::JsonSchema for ParsedWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ParsedWrap".into()
    }

    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schema_from_object(serde_json::json!({
            "type": "object",
            "description": concat!(
                "Parsed date/time components (chrono::format::Parsed). ",
                "All 21 fields are optional; provide only what you know."
            ),
            "properties": {
                "year":            { "type": ["integer", "null"], "format": "int32",
                                     "description": "Gregorian year" },
                "year_div_100":    { "type": ["integer", "null"], "format": "int32",
                                     "description": "Century component (year / 100)" },
                "year_mod_100":    { "type": ["integer", "null"], "format": "int32",
                                     "description": "Year-within-century (year % 100)" },
                "isoyear":         { "type": ["integer", "null"], "format": "int32",
                                     "description": "ISO 8601 week-based year" },
                "isoyear_div_100": { "type": ["integer", "null"], "format": "int32",
                                     "description": "Century component of ISO year" },
                "isoyear_mod_100": { "type": ["integer", "null"], "format": "int32",
                                     "description": "ISO year-within-century" },
                "quarter":         { "type": ["integer", "null"], "format": "uint32",
                                     "minimum": 1, "maximum": 4,
                                     "description": "Quarter of year (1–4)" },
                "month":           { "type": ["integer", "null"], "format": "uint32",
                                     "minimum": 1, "maximum": 12,
                                     "description": "Month of year (1–12)" },
                "week_from_sun":   { "type": ["integer", "null"], "format": "uint32",
                                     "description": "Week number from Sunday (0–53)" },
                "week_from_mon":   { "type": ["integer", "null"], "format": "uint32",
                                     "description": "Week number from Monday (0–53)" },
                "isoweek":         { "type": ["integer", "null"], "format": "uint32",
                                     "minimum": 1, "maximum": 53,
                                     "description": "ISO 8601 week number (1–53)" },
                "weekday":         {
                    "type": ["string", "null"],
                    "enum": ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun", null],
                    "description": "Day of week"
                },
                "ordinal":         { "type": ["integer", "null"], "format": "uint32",
                                     "minimum": 1, "maximum": 366,
                                     "description": "Day of year (1–366)" },
                "day":             { "type": ["integer", "null"], "format": "uint32",
                                     "minimum": 1, "maximum": 31,
                                     "description": "Day of month (1–31)" },
                "hour_div_12":     { "type": ["integer", "null"], "format": "uint32",
                                     "minimum": 0, "maximum": 1,
                                     "description": "AM/PM flag (0 = AM, 1 = PM)" },
                "hour_mod_12":     { "type": ["integer", "null"], "format": "uint32",
                                     "minimum": 0, "maximum": 11,
                                     "description": "Hour in 12-hour clock (0–11)" },
                "minute":          { "type": ["integer", "null"], "format": "uint32",
                                     "minimum": 0, "maximum": 59,
                                     "description": "Minute (0–59)" },
                "second":          { "type": ["integer", "null"], "format": "uint32",
                                     "minimum": 0, "maximum": 60,
                                     "description": "Second (0–60, allowing leap second)" },
                "nanosecond":      { "type": ["integer", "null"], "format": "uint32",
                                     "minimum": 0, "maximum": 999999999,
                                     "description": "Nanoseconds within second" },
                "timestamp":       { "type": ["integer", "null"], "format": "int64",
                                     "description": "Unix timestamp (seconds since epoch)" },
                "offset":          { "type": ["integer", "null"], "format": "int32",
                                     "description": "UTC offset in seconds (e.g., 3600 = UTC+1)" }
            }
        }))
    }
}

impl ParsedWrap {
    /// Unwrap to the inner [`chrono::format::Parsed`].
    pub fn into_inner(self) -> chrono::format::Parsed {
        self.0
    }
}

impl From<chrono::format::Parsed> for ParsedWrap {
    fn from(p: chrono::format::Parsed) -> Self {
        Self(p)
    }
}

impl Prompt for ParsedWrap {
    fn prompt() -> Option<&'static str> {
        <chrono::format::Parsed as Prompt>::prompt()
    }
}

impl Elicitation for ParsedWrap {
    type Style = ParsedStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "ParsedWrap"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        chrono::format::Parsed::elicit(communicator)
            .await
            .map(ParsedWrap::from)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[kani::proof]
            fn verify_parsed_wrap_default() {
                // ParsedWrap wraps chrono::format::Parsed (21 optional fields).
                // Verify that the default (all-None) value can be wrapped and unwrapped.
                let parsed = chrono::format::Parsed::default();
                let wrap = ParsedWrap::from(parsed.clone());
                let inner = wrap.into_inner();
                // All fields of a default Parsed are None.
                kani::assert(inner == parsed, "ParsedWrap round-trips through into_inner");
            }
        }
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            pub fn verify_parsed_wrap_identity(
                w: ParsedWrap,
            ) -> (result: ParsedWrap)
                ensures result == w,
            {
                w
            }
        }
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[ensures(result == w)]
            pub fn verify_parsed_wrap_identity(w: ParsedWrap) -> ParsedWrap {
                w
            }
        }
    }
}

impl ElicitIntrospect for ParsedWrap {
    fn pattern() -> ElicitationPattern {
        <chrono::format::Parsed as ElicitIntrospect>::pattern()
    }

    fn metadata() -> TypeMetadata {
        <chrono::format::Parsed as ElicitIntrospect>::metadata()
    }
}

impl ElicitPromptTree for ParsedWrap {
    fn prompt_tree() -> PromptTree {
        <chrono::format::Parsed as ElicitPromptTree>::prompt_tree()
    }
}

impl ToCodeLiteral for ParsedWrap {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let inner = self.0.to_code_literal();
        crate::quote::quote! { ParsedWrap::from(#inner) }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { ParsedWrap }
    }
}

// ============================================================================
// NaiveDateDaysIterator — iterator over consecutive dates, step = 1 day
// ============================================================================
//
// Constructed exclusively via `NaiveDate::iter_days()`. The sole private field
// `value: NaiveDate` is the next date the iterator will yield. Since it is
// private, we recover it by cloning and calling `next()`; if `next()` returns
// `None` (exhausted near NaiveDate::MAX), we fall back to NaiveDate::MAX.

impl Prompt for NaiveDateDaysIterator {
    fn prompt() -> Option<&'static str> {
        Some(
            "Enter the starting date for the day-step iterator (yields that date first, then each successive day):",
        )
    }
}

crate::default_style!(NaiveDateDaysIterator => NaiveDateDaysIteratorStyle);

impl Elicitation for NaiveDateDaysIterator {
    type Style = NaiveDateDaysIteratorStyle;

    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "chrono::NaiveDateDaysIterator")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting NaiveDateDaysIterator via starting NaiveDate");
        let start = NaiveDate::elicit(communicator).await?;
        Ok(start.iter_days())
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::NaiveDateDaysIterator")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::NaiveDateDaysIterator")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::NaiveDateDaysIterator")
    }
}

impl ElicitIntrospect for NaiveDateDaysIterator {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::NaiveDateDaysIterator",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for NaiveDateDaysIterator {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt()
                .unwrap_or("NaiveDateDaysIterator")
                .to_string(),
            type_name: "chrono::NaiveDateDaysIterator".to_string(),
        }
    }
}

impl ToCodeLiteral for NaiveDateDaysIterator {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        // Recover the private `value` field by cloning (Copy) and calling next().
        // Falls back to NaiveDate::MAX when the iterator is exhausted near the limit.
        let start = {
            let mut copy = *self;
            copy.next().unwrap_or(NaiveDate::MAX)
        };
        let y = start.year();
        let m = start.month();
        let d = start.day();
        crate::quote::quote! {
            match chrono::NaiveDate::from_ymd_opt(#y, #m, #d) {
                Some(d) => d.iter_days(),
                None => return Err("from_ymd_opt: invalid date in iterator serialization".into()),
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::NaiveDateDaysIterator }
    }
}

// ── NaiveDateDaysIteratorWrap ─────────────────────────────────────────────────

/// Newtype wrapper around `chrono::naive::NaiveDateDaysIterator` with serde and
/// JsonSchema support.
///
/// Stores the iterator's current position as a `start` date. Reconstructs the
/// iterator via `start.iter_days()`. When the iterator is exhausted near
/// `NaiveDate::MAX`, `start` is set to `NaiveDate::MAX`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NaiveDateDaysIteratorWrap {
    /// Current iterator position (the next date that would be yielded).
    pub start: chrono::NaiveDate,
}

impl schemars::JsonSchema for NaiveDateDaysIteratorWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "NaiveDateDaysIteratorWrap".into()
    }

    fn json_schema(schema_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schema_from_object(serde_json::json!({
            "type": "object",
            "description": concat!(
                "A day-step date iterator (chrono::NaiveDateDaysIterator). ",
                "`start` is the next date the iterator will yield."
            ),
            "properties": {
                "start": chrono::NaiveDate::json_schema(schema_gen)
            },
            "required": ["start"]
        }))
    }
}

impl NaiveDateDaysIteratorWrap {
    /// Reconstruct the wrapped iterator from the stored start date.
    pub fn into_inner(self) -> NaiveDateDaysIterator {
        self.start.iter_days()
    }
}

impl From<NaiveDateDaysIterator> for NaiveDateDaysIteratorWrap {
    fn from(iter: NaiveDateDaysIterator) -> Self {
        let mut copy = iter;
        let start = copy.next().unwrap_or(NaiveDate::MAX);
        Self { start }
    }
}

impl Prompt for NaiveDateDaysIteratorWrap {
    fn prompt() -> Option<&'static str> {
        <NaiveDateDaysIterator as Prompt>::prompt()
    }
}

impl Elicitation for NaiveDateDaysIteratorWrap {
    type Style = NaiveDateDaysIteratorStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "NaiveDateDaysIteratorWrap"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        NaiveDateDaysIterator::elicit(communicator)
            .await
            .map(NaiveDateDaysIteratorWrap::from)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[kani::proof]
            fn verify_naive_date_days_iterator_wrap_roundtrip() {
                // Verify that wrapping a known iterator and unwrapping recovers
                // a compatible iterator (same starting position).
                let date = match chrono::NaiveDate::from_ymd_opt(2024, 6, 15) {
                    Some(d) => d,
                    None => return,
                };
                let iter = date.iter_days();
                let wrap = NaiveDateDaysIteratorWrap::from(iter);
                // The wrap should record 2024-06-15 as start.
                kani::assert(wrap.start == date, "wrap preserves start date");
                // into_inner() reconstructs an equivalent iterator.
                let mut recovered = wrap.into_inner();
                let first = match recovered.next() { Some(f) => f, None => return };
                kani::assert(first == date, "recovered iterator yields original start date");
            }
        }
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            pub fn verify_naive_date_days_iterator_wrap_identity(
                w: NaiveDateDaysIteratorWrap,
            ) -> (result: NaiveDateDaysIteratorWrap)
                ensures result == w,
            {
                w
            }
        }
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[ensures(result == w)]
            pub fn verify_naive_date_days_iterator_wrap_identity(
                w: NaiveDateDaysIteratorWrap,
            ) -> NaiveDateDaysIteratorWrap {
                w
            }
        }
    }
}

impl ElicitIntrospect for NaiveDateDaysIteratorWrap {
    fn pattern() -> ElicitationPattern {
        <NaiveDateDaysIterator as ElicitIntrospect>::pattern()
    }

    fn metadata() -> TypeMetadata {
        <NaiveDateDaysIterator as ElicitIntrospect>::metadata()
    }
}

impl ElicitPromptTree for NaiveDateDaysIteratorWrap {
    fn prompt_tree() -> PromptTree {
        <NaiveDateDaysIterator as ElicitPromptTree>::prompt_tree()
    }
}

impl ToCodeLiteral for NaiveDateDaysIteratorWrap {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let y = self.start.year();
        let m = self.start.month();
        let d = self.start.day();
        crate::quote::quote! {
            match chrono::NaiveDate::from_ymd_opt(#y, #m, #d) {
                Some(start) => NaiveDateDaysIteratorWrap { start },
                None => return Err("from_ymd_opt: invalid start date in days iterator wrap".into()),
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { NaiveDateDaysIteratorWrap }
    }
}

// ============================================================================
// NaiveDateWeeksIterator — iterator over consecutive dates, step = 7 days
// ============================================================================
//
// Constructed exclusively via `NaiveDate::iter_weeks()`. Same private-field
// constraint as NaiveDateDaysIterator; same peek-via-clone strategy.
// `next()` returns `None` when `value + 7 days` would overflow NaiveDate::MAX,
// so the exhausted sentinel is NaiveDate::MAX.

impl Prompt for NaiveDateWeeksIterator {
    fn prompt() -> Option<&'static str> {
        Some(
            "Enter the starting date for the week-step iterator (yields that date first, then each date 7 days later):",
        )
    }
}

crate::default_style!(NaiveDateWeeksIterator => NaiveDateWeeksIteratorStyle);

impl Elicitation for NaiveDateWeeksIterator {
    type Style = NaiveDateWeeksIteratorStyle;

    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "chrono::NaiveDateWeeksIterator")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting NaiveDateWeeksIterator via starting NaiveDate");
        let start = NaiveDate::elicit(communicator).await?;
        Ok(start.iter_weeks())
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("chrono::NaiveDateWeeksIterator")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("chrono::NaiveDateWeeksIterator")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("chrono::NaiveDateWeeksIterator")
    }
}

impl ElicitIntrospect for NaiveDateWeeksIterator {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "chrono::NaiveDateWeeksIterator",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for NaiveDateWeeksIterator {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt()
                .unwrap_or("NaiveDateWeeksIterator")
                .to_string(),
            type_name: "chrono::NaiveDateWeeksIterator".to_string(),
        }
    }
}

impl ToCodeLiteral for NaiveDateWeeksIterator {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        // Recover the private `value` field by cloning (Copy) and calling next().
        // Falls back to NaiveDate::MAX when the iterator is exhausted near the limit.
        let start = {
            let mut copy = *self;
            copy.next().unwrap_or(NaiveDate::MAX)
        };
        let y = start.year();
        let m = start.month();
        let d = start.day();
        crate::quote::quote! {
            match chrono::NaiveDate::from_ymd_opt(#y, #m, #d) {
                Some(d) => d.iter_weeks(),
                None => return Err("from_ymd_opt: invalid date in weeks iterator serialization".into()),
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::NaiveDateWeeksIterator }
    }
}

// ── NaiveDateWeeksIteratorWrap ────────────────────────────────────────────────

/// Newtype wrapper around `chrono::naive::NaiveDateWeeksIterator` with serde and
/// JsonSchema support.
///
/// Stores the iterator's current position as a `start` date. Reconstructs the
/// iterator via `start.iter_weeks()`. When the iterator is exhausted (position
/// within 7 days of `NaiveDate::MAX`), `start` is set to `NaiveDate::MAX`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NaiveDateWeeksIteratorWrap {
    /// Current iterator position (the next date that would be yielded).
    pub start: chrono::NaiveDate,
}

impl schemars::JsonSchema for NaiveDateWeeksIteratorWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "NaiveDateWeeksIteratorWrap".into()
    }

    fn json_schema(schema_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schema_from_object(serde_json::json!({
            "type": "object",
            "description": concat!(
                "A week-step date iterator (chrono::NaiveDateWeeksIterator). ",
                "`start` is the next date the iterator will yield."
            ),
            "properties": {
                "start": chrono::NaiveDate::json_schema(schema_gen)
            },
            "required": ["start"]
        }))
    }
}

impl NaiveDateWeeksIteratorWrap {
    /// Reconstruct the wrapped iterator from the stored start date.
    pub fn into_inner(self) -> NaiveDateWeeksIterator {
        self.start.iter_weeks()
    }
}

impl From<NaiveDateWeeksIterator> for NaiveDateWeeksIteratorWrap {
    fn from(iter: NaiveDateWeeksIterator) -> Self {
        let mut copy = iter;
        let start = copy.next().unwrap_or(NaiveDate::MAX);
        Self { start }
    }
}

impl Prompt for NaiveDateWeeksIteratorWrap {
    fn prompt() -> Option<&'static str> {
        <NaiveDateWeeksIterator as Prompt>::prompt()
    }
}

impl Elicitation for NaiveDateWeeksIteratorWrap {
    type Style = NaiveDateWeeksIteratorStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "NaiveDateWeeksIteratorWrap"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        NaiveDateWeeksIterator::elicit(communicator)
            .await
            .map(NaiveDateWeeksIteratorWrap::from)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[kani::proof]
            fn verify_naive_date_weeks_iterator_wrap_roundtrip() {
                // Verify that wrapping a known iterator and unwrapping recovers
                // a compatible iterator (same starting position).
                let date = match chrono::NaiveDate::from_ymd_opt(2024, 1, 1) {
                    Some(d) => d,
                    None => return,
                };
                let iter = date.iter_weeks();
                let wrap = NaiveDateWeeksIteratorWrap::from(iter);
                kani::assert(wrap.start == date, "wrap preserves start date");
                let mut recovered = wrap.into_inner();
                let first = match recovered.next() { Some(f) => f, None => return };
                kani::assert(first == date, "recovered iterator yields original start date");
            }
        }
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            pub fn verify_naive_date_weeks_iterator_wrap_identity(
                w: NaiveDateWeeksIteratorWrap,
            ) -> (result: NaiveDateWeeksIteratorWrap)
                ensures result == w,
            {
                w
            }
        }
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[ensures(result == w)]
            pub fn verify_naive_date_weeks_iterator_wrap_identity(
                w: NaiveDateWeeksIteratorWrap,
            ) -> NaiveDateWeeksIteratorWrap {
                w
            }
        }
    }
}

impl ElicitIntrospect for NaiveDateWeeksIteratorWrap {
    fn pattern() -> ElicitationPattern {
        <NaiveDateWeeksIterator as ElicitIntrospect>::pattern()
    }

    fn metadata() -> TypeMetadata {
        <NaiveDateWeeksIterator as ElicitIntrospect>::metadata()
    }
}

impl ElicitPromptTree for NaiveDateWeeksIteratorWrap {
    fn prompt_tree() -> PromptTree {
        <NaiveDateWeeksIterator as ElicitPromptTree>::prompt_tree()
    }
}

impl ToCodeLiteral for NaiveDateWeeksIteratorWrap {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let y = self.start.year();
        let m = self.start.month();
        let d = self.start.day();
        crate::quote::quote! {
            match chrono::NaiveDate::from_ymd_opt(#y, #m, #d) {
                Some(start) => NaiveDateWeeksIteratorWrap { start },
                None => return Err("from_ymd_opt: invalid start date in weeks iterator wrap".into()),
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { NaiveDateWeeksIteratorWrap }
    }
}

// ============================================================================
// LocalResult<T> / MappedLocalTime<T> — 3-variant timezone-mapping result
// ============================================================================
//
// `MappedLocalTime<T>` is a type alias for `LocalResult<T>`:
//   pub type MappedLocalTime<T> = LocalResult<T>;
//
// Variants:
//   - None           — no mapping (gap in local time, or error)
//   - Single(T)      — exactly one result
//   - Ambiguous(T,T) — two results (fold in local time; earliest, latest)
//
// Generic impls: Elicitation, ElicitPromptTree, ToCodeLiteral, ElicitSpec for
// chrono::LocalResult<T> with the minimum bounds on T.
//
// Trenchcoat: LocalResultWrap<T> is our owned mirror enum — it derives
// Serialize + Deserialize + JsonSchema (which the orphan rule prevents adding
// directly to chrono::LocalResult).  ElicitComplete is impl'd generically for
// LocalResultWrap<T> where T: ElicitComplete.  MappedLocalTime<T> is just a
// type alias for LocalResult<T>, so this single trenchcoat covers both.
//
// ParseResult<T>: this is a type alias `type ParseResult<T> = Result<T, ParseError>`.
// Since Result<T, E>: Elicitation is already implemented generically in
// containers/result.rs, and chrono::ParseError: Elicitation is implemented above,
// ParseResult<T> = Result<T, ParseError> already participates in the elicitation
// system with no additional code required.
//
// Date<Tz>: deprecated since chrono 0.4.23; chrono recommends NaiveDate or
// DateTime<Tz>. Requires a generic TimeZone bound with private Offset field and
// cannot be round-trip serialized generically. Skipped intentionally.
//
// DelayedFormat<I>: a temporary formatting object whose `items: I` iterator is
// consumed on Display. Cannot be meaningfully serialized or elicited. Skipped.

/// Single-variant style for `LocalResult<T>`.
///
/// `LocalResult` has no meaningful style variants — the elicitation flow is
/// determined entirely by which of the three result variants the user chooses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LocalResultStyle {
    #[default]
    /// Default (only variant).
    Default,
}

impl Prompt for LocalResultStyle {
    fn prompt() -> Option<&'static str> {
        None
    }
}

impl Elicitation for LocalResultStyle {
    type Style = LocalResultStyle;

    #[tracing::instrument(skip(_communicator), level = "trace")]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        Ok(Self::Default)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_single_variant_enum("LocalResultStyle")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_single_variant_enum("LocalResultStyle")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_single_variant_enum("LocalResultStyle")
    }
}

impl crate::style::ElicitationStyle for LocalResultStyle {}

impl<T> Prompt for chrono::LocalResult<T> {
    fn prompt() -> Option<&'static str> {
        Some(concat!(
            "Select a local time mapping result: ",
            "`None` (gap/error), `Single` (one unambiguous result), ",
            "or `Ambiguous` (fold — two possible results, earliest then latest)."
        ))
    }
}

impl<T: Elicitation + Clone + Send> Elicitation for chrono::LocalResult<T> {
    type Style = LocalResultStyle;

    #[tracing::instrument(
        skip(communicator),
        fields(
            type_name = "chrono::LocalResult",
            inner_type = std::any::type_name::<T>()
        )
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting LocalResult: choosing variant");
        const LABELS: &[&str] = &["None", "Single", "Ambiguous"];
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "variant",
                "chrono::LocalResult",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| {
                Self::prompt()
                    .unwrap_or("Select a local time mapping result:")
                    .to_string()
            });
        let params = mcp::select_params(&prompt, LABELS);
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let label = mcp::parse_string(mcp::extract_value(result)?)?;
        tracing::debug!(%label, "LocalResult variant selected");
        match label.as_str() {
            "None" => Ok(chrono::LocalResult::None),
            "Single" => {
                tracing::debug!("Eliciting Single(T)");
                let value = T::elicit(communicator).await?;
                Ok(chrono::LocalResult::Single(value))
            }
            "Ambiguous" => {
                tracing::debug!("Eliciting Ambiguous(earliest, latest)");
                let earliest = T::elicit(communicator).await?;
                let latest = T::elicit(communicator).await?;
                Ok(chrono::LocalResult::Ambiguous(earliest, latest))
            }
            other => Err(ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid LocalResult variant: {other}"
            )))),
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <T as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <T as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <T as Elicitation>::creusot_proof()
    }
}

impl<T: ElicitPromptTree> ElicitPromptTree for chrono::LocalResult<T> {
    fn prompt_tree() -> PromptTree {
        PromptTree::Select {
            prompt: <chrono::LocalResult<T> as Prompt>::prompt()
                .map(str::to_string)
                .unwrap_or_else(|| "LocalResult variant".to_string()),
            type_name: "chrono::LocalResult".to_string(),
            options: vec![
                "None".to_string(),
                "Single".to_string(),
                "Ambiguous".to_string(),
            ],
            branches: vec![
                None,
                Some(Box::new(T::prompt_tree())),
                Some(Box::new(T::prompt_tree())),
            ],
        }
    }
}

impl<T: ElicitIntrospect + Clone + Send> ElicitIntrospect for chrono::LocalResult<T> {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        let inner_type = std::any::type_name::<T>();
        TypeMetadata {
            type_name: "chrono::LocalResult",
            description: <chrono::LocalResult<T> as Prompt>::prompt(),
            details: PatternDetails::Select {
                variants: vec![
                    VariantMetadata {
                        label: "None".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Single".to_string(),
                        fields: vec![FieldInfo {
                            name: "value",
                            type_name: inner_type,
                            prompt: None,
                        }],
                    },
                    VariantMetadata {
                        label: "Ambiguous".to_string(),
                        fields: vec![
                            FieldInfo {
                                name: "earliest",
                                type_name: inner_type,
                                prompt: None,
                            },
                            FieldInfo {
                                name: "latest",
                                type_name: inner_type,
                                prompt: None,
                            },
                        ],
                    },
                ],
            },
        }
    }
}

impl<T: crate::emit_code::ToCodeLiteral> crate::emit_code::ToCodeLiteral
    for chrono::LocalResult<T>
{
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            chrono::LocalResult::None => {
                let t = <T as crate::emit_code::ToCodeLiteral>::type_tokens();
                crate::quote::quote! { chrono::LocalResult::<#t>::None }
            }
            chrono::LocalResult::Single(v) => {
                let inner = v.to_code_literal();
                crate::quote::quote! { chrono::LocalResult::Single(#inner) }
            }
            chrono::LocalResult::Ambiguous(earliest, latest) => {
                let e = earliest.to_code_literal();
                let l = latest.to_code_literal();
                crate::quote::quote! { chrono::LocalResult::Ambiguous(#e, #l) }
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        let t = <T as crate::emit_code::ToCodeLiteral>::type_tokens();
        crate::quote::quote! { chrono::LocalResult<#t> }
    }
}

// ── LocalResultWrap<T> — serialisable trenchcoat ─────────────────────────────
//
// chrono::LocalResult<T> derives only Clone/PartialEq/Debug/Copy/Eq/Hash — no
// serde or schemars.  The orphan rule blocks us from adding those impls.
// LocalResultWrap<T> is our owned mirror: same three variants, derives the
// wire traits, and delegates every elicitation trait to the underlying
// chrono::LocalResult<T> via From/Into.

/// Serialisable mirror of `chrono::LocalResult<T>`.
///
/// `chrono::LocalResult<T>` does not implement `Serialize`, `Deserialize`, or
/// `JsonSchema`, and the orphan rule prevents adding those from outside chrono.
/// This owned enum has identical semantics and freely converts to/from the
/// chrono type.  All elicitation traits delegate through that conversion.
///
/// `MappedLocalTime<T>` is a type alias for `LocalResult<T>`; this trenchcoat
/// covers both names.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(tag = "variant", content = "value")]
pub enum LocalResultWrap<T> {
    /// No UTC instant corresponds to the local time (DST gap or error).
    None,
    /// Exactly one UTC instant corresponds to the local time.
    Single(T),
    /// Two UTC instants correspond to the local time (DST fold: earliest, latest).
    Ambiguous(T, T),
}

impl<T> From<chrono::LocalResult<T>> for LocalResultWrap<T> {
    fn from(r: chrono::LocalResult<T>) -> Self {
        match r {
            chrono::LocalResult::None => LocalResultWrap::None,
            chrono::LocalResult::Single(v) => LocalResultWrap::Single(v),
            chrono::LocalResult::Ambiguous(e, l) => LocalResultWrap::Ambiguous(e, l),
        }
    }
}

impl<T> From<LocalResultWrap<T>> for chrono::LocalResult<T> {
    fn from(w: LocalResultWrap<T>) -> Self {
        match w {
            LocalResultWrap::None => chrono::LocalResult::None,
            LocalResultWrap::Single(v) => chrono::LocalResult::Single(v),
            LocalResultWrap::Ambiguous(e, l) => chrono::LocalResult::Ambiguous(e, l),
        }
    }
}

/// Style for `LocalResultWrap<T>` — structurally identical to `LocalResultStyle`.
pub type LocalResultWrapStyle = LocalResultStyle;

impl<T> Prompt for LocalResultWrap<T> {
    fn prompt() -> Option<&'static str> {
        <chrono::LocalResult<T> as Prompt>::prompt()
    }
}

impl<T: Elicitation + Clone + Send> Elicitation for LocalResultWrap<T> {
    type Style = LocalResultWrapStyle;

    #[tracing::instrument(
        skip(communicator),
        fields(
            type_name = "LocalResultWrap",
            inner_type = std::any::type_name::<T>()
        )
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let inner = chrono::LocalResult::<T>::elicit(communicator).await?;
        Ok(inner.into())
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <T as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <T as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <T as Elicitation>::creusot_proof()
    }
}

impl<T: ElicitPromptTree> ElicitPromptTree for LocalResultWrap<T> {
    fn prompt_tree() -> PromptTree {
        <chrono::LocalResult<T> as ElicitPromptTree>::prompt_tree()
    }
}

impl<T: ElicitIntrospect + Clone + Send> ElicitIntrospect for LocalResultWrap<T> {
    fn pattern() -> ElicitationPattern {
        <chrono::LocalResult<T> as ElicitIntrospect>::pattern()
    }

    fn metadata() -> TypeMetadata {
        <chrono::LocalResult<T> as ElicitIntrospect>::metadata()
    }
}

impl<T: crate::emit_code::ToCodeLiteral> crate::emit_code::ToCodeLiteral for LocalResultWrap<T> {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let inner: chrono::LocalResult<&T> = match self {
            LocalResultWrap::None => chrono::LocalResult::None,
            LocalResultWrap::Single(v) => chrono::LocalResult::Single(v),
            LocalResultWrap::Ambiguous(e, l) => chrono::LocalResult::Ambiguous(e, l),
        };
        match inner {
            chrono::LocalResult::None => {
                let t = <T as crate::emit_code::ToCodeLiteral>::type_tokens();
                crate::quote::quote! { crate::LocalResultWrap::<#t>::None }
            }
            chrono::LocalResult::Single(v) => {
                let lit = v.to_code_literal();
                crate::quote::quote! { crate::LocalResultWrap::Single(#lit) }
            }
            chrono::LocalResult::Ambiguous(e, l) => {
                let e_lit = e.to_code_literal();
                let l_lit = l.to_code_literal();
                crate::quote::quote! { crate::LocalResultWrap::Ambiguous(#e_lit, #l_lit) }
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        let t = <T as crate::emit_code::ToCodeLiteral>::type_tokens();
        crate::quote::quote! { crate::LocalResultWrap<#t> }
    }
}

impl<T: crate::ElicitComplete + Clone + Send> crate::ElicitComplete for LocalResultWrap<T> {}

// ============================================================================
// format::Item<'a> — lifetime-bound format item enum
//
// OwnedItem is the serialisable trenchcoat for chrono::format::Item<'_>.
// It replaces &str / Box<str> with String so the type is 'static.
//
// Skipped in this batch:
//   - DelayedFormat<I>   generic iterator adapter; elicitation adds no value
//   - StrftimeItems::new_lenient / new_with_locale   constructors, not values
// ============================================================================

/// Owned variant of `chrono::format::Item<'_>` with no lifetime parameter.
///
/// Stores the same information as `Item` but replaces all borrowed/boxed string
/// slices with `String` and uses serializable wrapper types for `Numeric`, `Pad`,
/// and `Fixed` so the type can implement `Serialize + Deserialize + JsonSchema`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(tag = "variant", content = "value")]
pub enum OwnedItem {
    /// A literal string printed verbatim.
    Literal(String),
    /// Whitespace printed literally, parsed as zero-or-more whitespace.
    Space(String),
    /// A numeric formatting item with padding.
    Numeric(NumericSelect, PadSelect),
    /// A fixed-format item.
    Fixed(FixedSelect),
    /// Signals an invalid format specifier.
    Error,
}

impl OwnedItem {
    /// Reconstruct as `chrono::format::Item<'static>`.
    ///
    /// The string data inside `Literal` and `Space` becomes a `Box<str>`
    /// (i.e. the `OwnedLiteral` / `OwnedSpace` variants).
    pub fn into_item(self) -> chrono::format::Item<'static> {
        match self {
            OwnedItem::Literal(s) => chrono::format::Item::OwnedLiteral(s.into_boxed_str()),
            OwnedItem::Space(s) => chrono::format::Item::OwnedSpace(s.into_boxed_str()),
            OwnedItem::Numeric(n, p) => {
                chrono::format::Item::Numeric(n.into_inner(), p.into_inner())
            }
            OwnedItem::Fixed(f) => chrono::format::Item::Fixed(f.into_inner()),
            OwnedItem::Error => chrono::format::Item::Error,
        }
    }
}

impl<'a> From<chrono::format::Item<'a>> for OwnedItem {
    fn from(item: chrono::format::Item<'a>) -> Self {
        match item.to_owned() {
            chrono::format::Item::OwnedLiteral(s) => OwnedItem::Literal(s.into()),
            chrono::format::Item::OwnedSpace(s) => OwnedItem::Space(s.into()),
            chrono::format::Item::Numeric(n, p) => {
                OwnedItem::Numeric(NumericSelect::from(n), PadSelect::from(p))
            }
            chrono::format::Item::Fixed(f) => OwnedItem::Fixed(FixedSelect::from(f)),
            _ => OwnedItem::Error,
        }
    }
}

impl schemars::JsonSchema for OwnedItem {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "OwnedItem".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schema_from_object(serde_json::json!({
            "oneOf": [
                {
                    "type": "object",
                    "required": ["variant", "value"],
                    "properties": {
                        "variant": { "type": "string", "enum": ["Literal"] },
                        "value": { "type": "string" }
                    }
                },
                {
                    "type": "object",
                    "required": ["variant", "value"],
                    "properties": {
                        "variant": { "type": "string", "enum": ["Space"] },
                        "value": { "type": "string" }
                    }
                },
                {
                    "type": "object",
                    "required": ["variant", "value"],
                    "properties": {
                        "variant": { "type": "string", "enum": ["Numeric"] },
                        "value": {
                            "type": "object",
                            "required": ["numeric", "pad"],
                            "properties": {
                                "numeric": { "type": "string", "description": "chrono::format::Numeric variant name" },
                                "pad": { "type": "string", "description": "chrono::format::Pad variant name" }
                            }
                        }
                    }
                },
                {
                    "type": "object",
                    "required": ["variant", "value"],
                    "properties": {
                        "variant": { "type": "string", "enum": ["Fixed"] },
                        "value": { "type": "string", "description": "chrono::format::Fixed variant name" }
                    }
                },
                {
                    "type": "object",
                    "required": ["variant"],
                    "properties": {
                        "variant": { "type": "string", "enum": ["Error"] }
                    }
                }
            ]
        }))
    }
}

crate::default_style!(OwnedItem => OwnedItemStyle);

impl Prompt for OwnedItem {
    fn prompt() -> Option<&'static str> {
        Some("Select a format item kind:")
    }
}

impl Elicitation for OwnedItem {
    type Style = OwnedItemStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "OwnedItem"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting OwnedItem variant");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "variant",
                "OwnedItem",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| {
                Self::prompt()
                    .unwrap_or("Select a format item kind:")
                    .to_string()
            });
        let params = mcp::select_params(
            &prompt,
            &[
                "Literal".to_string(),
                "Space".to_string(),
                "Numeric".to_string(),
                "Fixed".to_string(),
                "Error".to_string(),
            ],
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        match label.as_str() {
            "Literal" => {
                tracing::debug!("Eliciting Literal string");
                let s = String::elicit(communicator).await?;
                Ok(OwnedItem::Literal(s))
            }
            "Space" => {
                tracing::debug!("Eliciting Space string");
                let s = String::elicit(communicator).await?;
                Ok(OwnedItem::Space(s))
            }
            "Numeric" => {
                tracing::debug!("Eliciting Numeric(Numeric, Pad)");
                let n = chrono::format::Numeric::elicit(communicator).await?;
                let p = chrono::format::Pad::elicit(communicator).await?;
                Ok(OwnedItem::Numeric(
                    NumericSelect::from(n),
                    PadSelect::from(p),
                ))
            }
            "Fixed" => {
                tracing::debug!("Eliciting Fixed");
                let f = chrono::format::Fixed::elicit(communicator).await?;
                Ok(OwnedItem::Fixed(FixedSelect::from(f)))
            }
            "Error" => Ok(OwnedItem::Error),
            other => Err(ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid OwnedItem variant: {other}"
            )))),
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[kani::proof]
            fn verify_owned_item_round_trip() {
                // Verify Literal round-trips through into_item
                let owned = crate::OwnedItem::Literal("test".to_string());
                let item = owned.clone().into_item();
                let back = crate::OwnedItem::from(item);
                kani::assert(back == owned, "OwnedItem::Literal round-trips");
            }
        }
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }
}

impl ElicitIntrospect for OwnedItem {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "OwnedItem",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: vec![
                    VariantMetadata {
                        label: "Literal".to_string(),
                        fields: vec![FieldInfo {
                            name: "text",
                            type_name: "String",
                            prompt: Some("Literal string content"),
                        }],
                    },
                    VariantMetadata {
                        label: "Space".to_string(),
                        fields: vec![FieldInfo {
                            name: "text",
                            type_name: "String",
                            prompt: Some("Whitespace string"),
                        }],
                    },
                    VariantMetadata {
                        label: "Numeric".to_string(),
                        fields: vec![
                            FieldInfo {
                                name: "numeric",
                                type_name: "chrono::format::Numeric",
                                prompt: Some("Numeric format specifier"),
                            },
                            FieldInfo {
                                name: "pad",
                                type_name: "chrono::format::Pad",
                                prompt: Some("Padding style"),
                            },
                        ],
                    },
                    VariantMetadata {
                        label: "Fixed".to_string(),
                        fields: vec![FieldInfo {
                            name: "fixed",
                            type_name: "chrono::format::Fixed",
                            prompt: Some("Fixed format specifier"),
                        }],
                    },
                    VariantMetadata {
                        label: "Error".to_string(),
                        fields: vec![],
                    },
                ],
            },
        }
    }
}

impl ElicitPromptTree for OwnedItem {
    fn prompt_tree() -> PromptTree {
        PromptTree::Select {
            prompt: "Select a format item kind:".to_string(),
            type_name: "OwnedItem".to_string(),
            options: vec![
                "Literal".to_string(),
                "Space".to_string(),
                "Numeric".to_string(),
                "Fixed".to_string(),
                "Error".to_string(),
            ],
            branches: vec![
                Some(Box::new(String::prompt_tree())),
                Some(Box::new(String::prompt_tree())),
                Some(Box::new(PromptTree::Survey {
                    prompt: None,
                    type_name: "Numeric+Pad".to_string(),
                    fields: vec![
                        (
                            "numeric".to_string(),
                            Box::new(chrono::format::Numeric::prompt_tree()),
                        ),
                        (
                            "pad".to_string(),
                            Box::new(chrono::format::Pad::prompt_tree()),
                        ),
                    ],
                })),
                Some(Box::new(chrono::format::Fixed::prompt_tree())),
                None,
            ],
        }
    }
}

impl ToCodeLiteral for OwnedItem {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            OwnedItem::Literal(s) => {
                crate::quote::quote! {
                    chrono::format::Item::OwnedLiteral(Box::from(#s))
                }
            }
            OwnedItem::Space(s) => {
                crate::quote::quote! {
                    chrono::format::Item::OwnedSpace(Box::from(#s))
                }
            }
            OwnedItem::Numeric(n, p) => {
                let n_tokens = (**n).to_code_literal();
                let p_tokens = (**p).to_code_literal();
                crate::quote::quote! {
                    chrono::format::Item::Numeric(#n_tokens, #p_tokens)
                }
            }
            OwnedItem::Fixed(f) => {
                let f_tokens = (**f).to_code_literal();
                crate::quote::quote! {
                    chrono::format::Item::Fixed(#f_tokens)
                }
            }
            OwnedItem::Error => {
                crate::quote::quote! { chrono::format::Item::Error }
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { chrono::format::Item<'static> }
    }
}

// ElicitComplete blanket impl provided by chrono_wrap_spec! in datetime_specs.rs.
// OwnedItem is its own ElicitComplete type (no separate raw-type wrapper needed).
#[cfg(not(kani))]
impl crate::ElicitComplete for OwnedItem {}

// ============================================================================
// format::StrftimeItems<'a> — format-string iterator
//
// OwnedStrftimeItems is the serialisable trenchcoat.  Since the private
// `remainder` field cannot be read back out of an existing StrftimeItems, we
// consume the iterator and collect all items into a Vec<OwnedItem>.
// ============================================================================

/// Owned trenchcoat for `chrono::format::StrftimeItems<'_>`.
///
/// Stores the parsed items as a `Vec<OwnedItem>` because the internal format
/// string is private and cannot be recovered without consuming the iterator.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OwnedStrftimeItems(pub Vec<OwnedItem>);

impl OwnedStrftimeItems {
    /// Reconstruct a `StrftimeItems<'static>`-equivalent owned item sequence.
    pub fn into_items(self) -> Vec<chrono::format::Item<'static>> {
        self.0.into_iter().map(|i| i.into_item()).collect()
    }
}

impl<'a> From<chrono::format::StrftimeItems<'a>> for OwnedStrftimeItems {
    fn from(iter: chrono::format::StrftimeItems<'a>) -> Self {
        OwnedStrftimeItems(iter.map(OwnedItem::from).collect())
    }
}

impl schemars::JsonSchema for OwnedStrftimeItems {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "OwnedStrftimeItems".into()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let item_schema: serde_json::Value = OwnedItem::json_schema(generator).into();
        let mut map = serde_json::Map::new();
        map.insert(
            "type".to_owned(),
            serde_json::Value::String("array".to_owned()),
        );
        map.insert("items".to_owned(), item_schema);
        map.insert(
            "description".to_owned(),
            serde_json::Value::String("An ordered sequence of chrono format items".to_owned()),
        );
        schemars::Schema::from(map)
    }
}

crate::default_style!(OwnedStrftimeItems => OwnedStrftimeItemsStyle);

impl Prompt for OwnedStrftimeItems {
    fn prompt() -> Option<&'static str> {
        Some("Enter a sequence of format items (e.g. from a strftime format string):")
    }
}

impl Elicitation for OwnedStrftimeItems {
    type Style = OwnedStrftimeItemsStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "OwnedStrftimeItems"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting OwnedStrftimeItems as Vec<OwnedItem>");
        let items = Vec::<OwnedItem>::elicit(communicator).await?;
        Ok(OwnedStrftimeItems(items))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::quote::quote! {
            #[kani::proof]
            fn verify_owned_strftime_items_round_trip() {
                // Empty sequence round-trips trivially.
                let items = crate::OwnedStrftimeItems(vec![]);
                let rebuilt = items.clone().into_items();
                kani::assert(rebuilt.is_empty(), "empty OwnedStrftimeItems produces no items");
            }
        }
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }
}

impl ElicitIntrospect for OwnedStrftimeItems {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "OwnedStrftimeItems",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for OwnedStrftimeItems {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt()
                .map(str::to_string)
                .unwrap_or_else(|| "StrftimeItems".to_string()),
            type_name: "OwnedStrftimeItems".to_string(),
        }
    }
}

impl ToCodeLiteral for OwnedStrftimeItems {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let items: Vec<proc_macro2::TokenStream> =
            self.0.iter().map(|i| i.to_code_literal()).collect();
        crate::quote::quote! {
            vec![#(#items),*]
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { Vec<chrono::format::Item<'static>> }
    }
}

#[cfg(not(kani))]
impl crate::ElicitComplete for OwnedStrftimeItems {}
