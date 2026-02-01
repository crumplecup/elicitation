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
    ElicitClient, ElicitError, ElicitErrorKind, ElicitResult, Elicitation, Prompt,
    datetime_common::{DateTimeComponents, DateTimeInputMethod},
    mcp,
};
use time::{OffsetDateTime, PrimitiveDateTime, UtcOffset};

// Style enums for time types
crate::default_style!(OffsetDateTime => OffsetDateTimeStyle);
crate::default_style!(PrimitiveDateTime => PrimitiveDateTimeStyle);

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
