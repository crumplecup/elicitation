//! chrono datetime library elicitation implementations.
//!
//! Available with the `chrono` feature.
//!
//! This module provides `Elicitation` implementations for the most popular
//! Rust datetime library. Supports both ISO 8601 string parsing and manual
//! component entry for maximum flexibility.
//!
//! # Supported Types
//!
//! - [`DateTime<Utc>`] - UTC timestamps
//! - [`DateTime<FixedOffset>`] - Timestamps with fixed timezone offset
//! - [`NaiveDateTime`] - Timezone-agnostic datetime
//!
//! # Example
//!
//! ```rust,ignore
//! use chrono::{DateTime, Utc};
//! use elicitation::Elicitation;
//! use rmcp::service::{Peer, RoleClient};
//!
//! async fn example(client: &Peer<RoleClient>) {
//!     // Elicit a UTC timestamp
//!     let timestamp: DateTime<Utc> = DateTime::elicit(client).await?;
//!     
//!     // User can choose:
//!     // 1. ISO 8601 string: "2024-07-11T15:30:00Z"
//!     // 2. Manual components: year, month, day, hour, minute, second
//! }
//! ```
//!
//! # Elicitation Flow
//!
//! 1. **Input Method Selection** - User chooses ISO 8601 or manual components
//! 2. **Data Entry** - Based on selection:
//!    - ISO: Single string prompt with format validation
//!    - Manual: Six prompts for year, month, day, hour, minute, second
//! 3. **Validation** - chrono validates datetime construction
//! 4. **Result** - Returns validated `DateTime` or error

use crate::{
    ElicitClient, ElicitError, ElicitErrorKind, ElicitResult, Elicitation, Prompt,
    datetime_common::{DateTimeComponents, DateTimeInputMethod},
    mcp,
};
use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};

// Style enums for datetime types
crate::default_style!(DateTime<Utc> => DateTimeUtcStyle);
crate::default_style!(DateTime<FixedOffset> => DateTimeFixedOffsetStyle);
crate::default_style!(NaiveDateTime => NaiveDateTimeStyle);

// DateTime<Utc> implementation
impl Prompt for DateTime<Utc> {
    fn prompt() -> Option<&'static str> {
        Some("Enter UTC datetime:")
    }
}

impl Elicitation for DateTime<Utc> {
    type Style = DateTimeUtcStyle;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting DateTime<Utc>");

        // Step 1: Choose input method
        let method = DateTimeInputMethod::elicit(client).await?;
        tracing::debug!(?method, "Input method selected");

        match method {
            DateTimeInputMethod::Iso8601String => {
                // Elicit ISO 8601 string
                let prompt = "Enter ISO 8601 datetime (e.g., \"2024-07-11T15:30:00Z\"):";
                let params = mcp::text_params(prompt);
                let result = client
                    .peer()
                    .call_tool(rmcp::model::CallToolRequestParam {
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
                let components = DateTimeComponents::elicit(client).await?;

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

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting DateTime<FixedOffset>");

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
                    .call_tool(rmcp::model::CallToolRequestParam {
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
                let components = DateTimeComponents::elicit(client).await?;

                // Elicit offset
                let offset_prompt = "Enter timezone offset in hours (e.g., +5 or -8):";
                let offset_params = mcp::number_params(offset_prompt, -12, 14);
                let offset_result = client
                    .peer()
                    .call_tool(rmcp::model::CallToolRequestParam {
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

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting NaiveDateTime");

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
                    .call_tool(rmcp::model::CallToolRequestParam {
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
                let components = DateTimeComponents::elicit(client).await?;

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
