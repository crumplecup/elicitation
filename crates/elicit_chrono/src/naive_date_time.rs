//! `NaiveDateTime` — elicitation-enabled wrapper around `chrono::NaiveDateTime`.

use chrono::{Datelike, Timelike};
use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(chrono::NaiveDateTime, as NaiveDateTime, serde);

#[reflect_methods]
impl NaiveDateTime {
    /// Returns the year.
    #[instrument(skip(self))]
    pub fn year(&self) -> i32 {
        self.0.year()
    }

    /// Returns the month (1 = January … 12 = December).
    #[instrument(skip(self))]
    pub fn month(&self) -> u32 {
        self.0.month()
    }

    /// Returns the day of the month (1–31).
    #[instrument(skip(self))]
    pub fn day(&self) -> u32 {
        self.0.day()
    }

    /// Returns the hour (0–23).
    #[instrument(skip(self))]
    pub fn hour(&self) -> u32 {
        self.0.hour()
    }

    /// Returns the minute (0–59).
    #[instrument(skip(self))]
    pub fn minute(&self) -> u32 {
        self.0.minute()
    }

    /// Returns the second (0–59).
    #[instrument(skip(self))]
    pub fn second(&self) -> u32 {
        self.0.second()
    }

    /// Returns the nanosecond component (0–999_999_999).
    #[instrument(skip(self))]
    pub fn nanosecond(&self) -> u32 {
        self.0.nanosecond()
    }

    /// Returns the Unix timestamp in whole seconds (treating this as UTC).
    #[instrument(skip(self))]
    pub fn timestamp(&self) -> i64 {
        self.0.and_utc().timestamp()
    }

    /// Returns the day of the year (1–366).
    #[instrument(skip(self))]
    pub fn ordinal(&self) -> u32 {
        self.0.ordinal()
    }

    /// Returns the weekday name (e.g. `"Monday"`).
    #[instrument(skip(self))]
    pub fn weekday(&self) -> String {
        self.0.weekday().to_string()
    }

    /// Formats the datetime using the given strftime format string.
    ///
    /// See <https://docs.rs/chrono/latest/chrono/format/strftime/index.html> for format tokens.
    #[instrument(skip(self))]
    pub fn format_str(&self, fmt: String) -> String {
        self.0.format(&fmt).to_string()
    }
}

impl NaiveDateTime {
    /// Parse an ISO 8601 string (e.g. `"2024-01-15T12:30:00"`). Returns `None` if invalid.
    pub fn parse(s: &str) -> Option<Self> {
        chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
            .or_else(|_| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f"))
            .ok()
            .map(|dt| std::sync::Arc::new(dt).into())
    }
}
