//! Common datetime elicitation patterns shared across libraries.
//!
//! Available with datetime feature flags (`chrono`, `time`, `jiff`).
//!
//! This module provides reusable components for datetime elicitation:
//!
//! - [`DateTimeInputMethod`] - Choice between ISO 8601 string or manual components
//! - [`DateTimeComponents`] - Manual entry of year, month, day, hour, minute, second
//!
//! These shared patterns ensure consistent UX across all datetime libraries.

use crate::{
    ElicitClient, ElicitError, ElicitErrorKind, ElicitResult, Elicitation, Prompt, Select, mcp,
};

/// Input method for datetime elicitation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DateTimeInputMethod {
    /// ISO 8601 string (e.g., "2024-07-11T15:30:00Z")
    Iso8601String,
    /// Manual components (year, month, day, hour, minute, second)
    ManualComponents,
}

impl crate::Prompt for DateTimeInputMethod {
    fn prompt() -> Option<&'static str> {
        Some("How would you like to enter the datetime?")
    }
}

impl Select for DateTimeInputMethod {
    fn options() -> &'static [Self] {
        &[
            DateTimeInputMethod::Iso8601String,
            DateTimeInputMethod::ManualComponents,
        ]
    }

    fn labels() -> &'static [&'static str] {
        &[
            "ISO 8601 string (e.g., \"2024-07-11T15:30:00Z\")",
            "Manual components (year, month, day, etc.)",
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        if label.contains("ISO") || label.contains("string") {
            Some(DateTimeInputMethod::Iso8601String)
        } else if label.contains("Manual") || label.contains("components") {
            Some(DateTimeInputMethod::ManualComponents)
        } else {
            None
        }
    }
}

// Default-only style for DateTimeInputMethod
crate::default_style!(DateTimeInputMethod => DateTimeInputMethodStyle);

impl Elicitation for DateTimeInputMethod {
    type Style = DateTimeInputMethodStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let prompt = Self::prompt().unwrap();
        tracing::debug!("Eliciting datetime input method");

        let params = mcp::select_params(prompt, Self::labels());
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

        Self::from_label(&label)
            .ok_or_else(|| ElicitError::new(ElicitErrorKind::InvalidSelection(label)))
    }
}

/// Component values for manual datetime entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DateTimeComponents {
    /// Year (e.g., 2024)
    pub year: i32,
    /// Month (1-12)
    pub month: u8,
    /// Day (1-31)
    pub day: u8,
    /// Hour (0-23)
    pub hour: u8,
    /// Minute (0-59)
    pub minute: u8,
    /// Second (0-59)
    pub second: u8,
}

impl DateTimeComponents {
    /// Elicit datetime components from user.
    #[tracing::instrument(skip(communicator))]
    pub async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting datetime components");

        // Year
        let year_params = mcp::number_params("Enter year:", 1970, 2100);
        let year_result = client
            .peer()
            .call_tool(rmcp::model::CallToolRequestParams {
                meta: None,
                name: mcp::tool_names::elicit_number().into(),
                arguments: Some(year_params),
                task: None,
            })
            .await?;
        let year_value = mcp::extract_value(year_result)?;
        let year = mcp::parse_integer::<i64>(year_value)? as i32;

        // Month
        let month_params = mcp::number_params("Enter month (1-12):", 1, 12);
        let month_result = client
            .peer()
            .call_tool(rmcp::model::CallToolRequestParams {
                meta: None,
                name: mcp::tool_names::elicit_number().into(),
                arguments: Some(month_params),
                task: None,
            })
            .await?;
        let month_value = mcp::extract_value(month_result)?;
        let month = mcp::parse_integer::<i64>(month_value)? as u8;

        // Day
        let day_params = mcp::number_params("Enter day (1-31):", 1, 31);
        let day_result = client
            .peer()
            .call_tool(rmcp::model::CallToolRequestParams {
                meta: None,
                name: mcp::tool_names::elicit_number().into(),
                arguments: Some(day_params),
                task: None,
            })
            .await?;
        let day_value = mcp::extract_value(day_result)?;
        let day = mcp::parse_integer::<i64>(day_value)? as u8;

        // Hour
        let hour_params = mcp::number_params("Enter hour (0-23):", 0, 23);
        let hour_result = client
            .peer()
            .call_tool(rmcp::model::CallToolRequestParams {
                meta: None,
                name: mcp::tool_names::elicit_number().into(),
                arguments: Some(hour_params),
                task: None,
            })
            .await?;
        let hour_value = mcp::extract_value(hour_result)?;
        let hour = mcp::parse_integer::<i64>(hour_value)? as u8;

        // Minute
        let minute_params = mcp::number_params("Enter minute (0-59):", 0, 59);
        let minute_result = client
            .peer()
            .call_tool(rmcp::model::CallToolRequestParams {
                meta: None,
                name: mcp::tool_names::elicit_number().into(),
                arguments: Some(minute_params),
                task: None,
            })
            .await?;
        let minute_value = mcp::extract_value(minute_result)?;
        let minute = mcp::parse_integer::<i64>(minute_value)? as u8;

        // Second
        let second_params = mcp::number_params("Enter second (0-59):", 0, 59);
        let second_result = client
            .peer()
            .call_tool(rmcp::model::CallToolRequestParams {
                meta: None,
                name: mcp::tool_names::elicit_number().into(),
                arguments: Some(second_params),
                task: None,
            })
            .await?;
        let second_value = mcp::extract_value(second_result)?;
        let second = mcp::parse_integer::<i64>(second_value)? as u8;

        tracing::debug!(
            year,
            month,
            day,
            hour,
            minute,
            second,
            "Components elicited"
        );

        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }
}
