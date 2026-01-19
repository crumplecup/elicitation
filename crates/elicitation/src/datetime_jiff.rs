//! jiff datetime library elicitation implementations.
//!
//! Available with the `jiff` feature.
//!
//! This module provides `Elicitation` implementations for the newest, most
//! ergonomic Rust datetime library. Features built-in timezone support and
//! excellent DST handling.
//!
//! # Supported Types
//!
//! - [`Timestamp`] - Absolute moment in time (UTC)
//! - [`Zoned`] - Timestamp with timezone (IANA database)
//! - [`civil::DateTime`] - Calendar date + time (no timezone)
//!
//! # Example
//!
//! ```rust,ignore
//! use jiff::Timestamp;
//! use elicitation::Elicitation;
//! use rmcp::service::{Peer, RoleClient};
//!
//! async fn example(client: &Peer<RoleClient>) {
//!     // Elicit an absolute timestamp
//!     let timestamp: Timestamp = Timestamp::elicit(client).await?;
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
//!    - Manual: Six prompts for datetime + timezone (for Zoned)
//! 3. **Validation** - jiff validates datetime construction
//! 4. **Result** - Returns validated datetime or error

use crate::{
    ElicitClient, ElicitError, ElicitErrorKind, ElicitResult, Elicitation, Prompt,
    datetime_common::{DateTimeComponents, DateTimeInputMethod},
    mcp,
};
use jiff::{Timestamp, Zoned, civil::DateTime as CivilDateTime, tz::TimeZone};

// Style enums for jiff types
crate::default_style!(Timestamp => TimestampStyle);
crate::default_style!(Zoned => ZonedStyle);
crate::default_style!(CivilDateTime => CivilDateTimeStyle);

// Timestamp implementation
impl Prompt for Timestamp {
    fn prompt() -> Option<&'static str> {
        Some("Enter UTC timestamp:")
    }
}

impl Elicitation for Timestamp {
    type Style = TimestampStyle;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Timestamp");

        // Step 1: Choose input method
        let method = DateTimeInputMethod::elicit(client).await?;
        tracing::debug!(?method, "Input method selected");

        match method {
            DateTimeInputMethod::Iso8601String => {
                // Elicit ISO 8601 string
                let prompt = "Enter ISO 8601 timestamp (e.g., \"2024-07-11T15:30:00Z\"):";
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
                iso_string.parse::<Timestamp>().map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid ISO 8601 timestamp: {}",
                        e
                    )))
                })
            }
            DateTimeInputMethod::ManualComponents => {
                // Elicit components
                let components = DateTimeComponents::elicit(client).await?;

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

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Zoned");

        // Step 1: Choose input method
        let method = DateTimeInputMethod::elicit(client).await?;
        tracing::debug!(?method, "Input method selected");

        match method {
            DateTimeInputMethod::Iso8601String => {
                // Elicit ISO 8601 string with timezone
                let prompt = "Enter ISO 8601 datetime with timezone (e.g., \"2024-07-11T15:30:00-05[America/New_York]\"):";
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
                iso_string.parse::<Zoned>().map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid ISO 8601 zoned datetime: {}",
                        e
                    )))
                })
            }
            DateTimeInputMethod::ManualComponents => {
                // Elicit components
                let components = DateTimeComponents::elicit(client).await?;

                // Elicit timezone
                let tz_prompt = "Enter IANA timezone (e.g., \"America/New_York\" or \"UTC\"):";
                let tz_params = mcp::text_params(tz_prompt);
                let tz_result = client
                    .peer()
                    .call_tool(rmcp::model::CallToolRequestParam {
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

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting CivilDateTime");

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
                let components = DateTimeComponents::elicit(client).await?;

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
