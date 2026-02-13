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
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitResult, Elicitation, Prompt, Select,
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

        let options_text = Self::labels()
            .iter()
            .enumerate()
            .map(|(i, label)| format!("{}. {}", i + 1, label))
            .collect::<Vec<_>>()
            .join("\n");
        let full_prompt = format!("{}\n\nOptions:\n{}", prompt, options_text);
        
        let response = communicator.send_prompt(&full_prompt).await?;
        let trimmed = response.trim();
        
        // Try to parse as number first
        if let Ok(choice) = trimmed.parse::<usize>()
            && choice >= 1 && choice <= Self::labels().len()
        {
            let label = Self::labels()[choice - 1];
            return Self::from_label(label)
                .ok_or_else(|| ElicitError::new(ElicitErrorKind::InvalidSelection(label.to_string())));
        }
        
        // Otherwise try to match as label
        Self::from_label(trimmed)
            .ok_or_else(|| ElicitError::new(ElicitErrorKind::InvalidSelection(trimmed.to_string())))
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
        let year_prompt = "Enter year (1970-2100):";
        let year_response = communicator.send_prompt(year_prompt).await?;
        let year = year_response.trim().parse::<i32>().map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(
                format!("Invalid year: '{}' ({})", year_response.trim(), e)
            ))
        })?;
        if !(1970..=2100).contains(&year) {
            return Err(ElicitError::new(ElicitErrorKind::ParseError(
                format!("Year must be between 1970 and 2100, got {}", year)
            )));
        }

        // Month
        let month_prompt = "Enter month (1-12):";
        let month_response = communicator.send_prompt(month_prompt).await?;
        let month = month_response.trim().parse::<u8>().map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(
                format!("Invalid month: '{}' ({})", month_response.trim(), e)
            ))
        })?;
        if !(1..=12).contains(&month) {
            return Err(ElicitError::new(ElicitErrorKind::ParseError(
                format!("Month must be between 1 and 12, got {}", month)
            )));
        }

        // Day
        let day_prompt = "Enter day (1-31):";
        let day_response = communicator.send_prompt(day_prompt).await?;
        let day = day_response.trim().parse::<u8>().map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(
                format!("Invalid day: '{}' ({})", day_response.trim(), e)
            ))
        })?;
        if !(1..=31).contains(&day) {
            return Err(ElicitError::new(ElicitErrorKind::ParseError(
                format!("Day must be between 1 and 31, got {}", day)
            )));
        }

        // Hour
        let hour_prompt = "Enter hour (0-23):";
        let hour_response = communicator.send_prompt(hour_prompt).await?;
        let hour = hour_response.trim().parse::<u8>().map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(
                format!("Invalid hour: '{}' ({})", hour_response.trim(), e)
            ))
        })?;
        if hour > 23 {
            return Err(ElicitError::new(ElicitErrorKind::ParseError(
                format!("Hour must be between 0 and 23, got {}", hour)
            )));
        }

        // Minute
        let minute_prompt = "Enter minute (0-59):";
        let minute_response = communicator.send_prompt(minute_prompt).await?;
        let minute = minute_response.trim().parse::<u8>().map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(
                format!("Invalid minute: '{}' ({})", minute_response.trim(), e)
            ))
        })?;
        if minute > 59 {
            return Err(ElicitError::new(ElicitErrorKind::ParseError(
                format!("Minute must be between 0 and 59, got {}", minute)
            )));
        }

        // Second
        let second_prompt = "Enter second (0-59):";
        let second_response = communicator.send_prompt(second_prompt).await?;
        let second = second_response.trim().parse::<u8>().map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(
                format!("Invalid second: '{}' ({})", second_response.trim(), e)
            ))
        })?;
        if second > 59 {
            return Err(ElicitError::new(ElicitErrorKind::ParseError(
                format!("Second must be between 0 and 59, got {}", second)
            )));
        }

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
