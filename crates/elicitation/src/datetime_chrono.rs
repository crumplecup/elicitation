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
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitPromptTree, ElicitationPattern, Generator, PatternDetails, Prompt, PromptTree, Select,
    TypeMetadata, VariantMetadata,
    datetime_common::{DateTimeComponents, DateTimeInputMethod},
    emit_code::ToCodeLiteral,
    mcp,
};
use chrono::{DateTime, Datelike, Duration, FixedOffset, Month, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike, Utc, Weekday};

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
        proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
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
        proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
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
                // Elicit ISO 8601 string
                let prompt = "Enter ISO 8601 datetime (e.g., \"2024-07-11T15:30:00Z\"):";
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
        proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
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
        proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
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
        proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
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
        let params = mcp::text_params("Enter a date (e.g., \"2024-07-11\"):");
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
        proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
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
            chrono::NaiveDate::from_ymd_opt(#year, #month, #day).expect("valid date")
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
        let params = mcp::text_params("Enter a time (e.g., \"15:30:00\"):");
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
        proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
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
                chrono::NaiveTime::from_hms_opt(#hour, #min, #sec).expect("valid time")
            }
        } else {
            crate::quote::quote! {
                chrono::NaiveTime::from_hms_nano_opt(#hour, #min, #sec, #nano).expect("valid time")
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
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose a month:"),
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
            prompt: Self::prompt()
                .unwrap_or("Choose a month:")
                .to_string(),
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
crate::select_trenchcoat_traits!(MonthSelect, chrono::Month, [copy, eq, hash]);

// ============================================================================
// TimeDelta (chrono::Duration alias)
// ============================================================================

crate::default_style!(Duration => TimeDeltaStyle);

impl Prompt for Duration {
    fn prompt() -> Option<&'static str> {
        Some("Enter duration in whole seconds (can be negative):")
    }
}

impl Elicitation for Duration {
    type Style = TimeDeltaStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TimeDelta");
        let params = mcp::number_params(
            Self::prompt().unwrap_or("Enter duration in seconds:"),
            i64::MIN,
            i64::MAX,
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let secs = mcp::parse_integer::<i64>(value)?;
        Duration::try_seconds(secs).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Duration out of range: {secs} seconds"
            )))
        })
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

impl ElicitIntrospect for Duration {
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


