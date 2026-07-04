//! `NaiveDateTime` — elicitation-enabled wrapper around `chrono::NaiveDateTime`.

use chrono::{Datelike, Timelike};
use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(chrono::NaiveDateTime, as NaiveDateTime, serde);
elicitation::elicit_newtype_traits!(NaiveDateTime, chrono::NaiveDateTime, [cmp]);

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
    pub fn format(&self, fmt: String) -> String {
        self.0.format(&fmt).to_string()
    }

    /// Returns the Unix timestamp in microseconds (treating this as UTC).
    #[instrument(skip(self))]
    pub fn timestamp_micros(&self) -> i64 {
        self.0.and_utc().timestamp_micros()
    }

    /// Returns the Unix timestamp in nanoseconds, or `None` if out of range (treating this as UTC).
    #[instrument(skip(self))]
    pub fn timestamp_nanos(&self) -> Option<i64> {
        self.0.and_utc().timestamp_nanos_opt()
    }

    /// Returns the fractional part of the timestamp in microseconds (0–999_999).
    #[instrument(skip(self))]
    pub fn timestamp_subsec_micros(&self) -> u32 {
        self.0.and_utc().timestamp_subsec_micros()
    }

    /// Returns the fractional part of the timestamp in milliseconds (0–999).
    #[instrument(skip(self))]
    pub fn timestamp_subsec_millis(&self) -> u32 {
        self.0.and_utc().timestamp_subsec_millis()
    }

    /// Returns the fractional part of the timestamp in nanoseconds (0–999_999_999).
    #[instrument(skip(self))]
    pub fn timestamp_subsec_nanos(&self) -> u32 {
        self.0.and_utc().timestamp_subsec_nanos()
    }

    /// Returns `self + rhs`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_add_signed(&self, rhs: crate::Duration) -> Option<crate::NaiveDateTime> {
        self.0.checked_add_signed(*rhs).map(Into::into)
    }

    /// Returns `self - rhs`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_sub_signed(&self, rhs: crate::Duration) -> Option<crate::NaiveDateTime> {
        self.0.checked_sub_signed(*rhs).map(Into::into)
    }

    /// Returns the signed duration between `self` and `rhs` (positive if `self` is later).
    #[instrument(skip(self))]
    pub fn signed_duration_since(&self, rhs: crate::NaiveDateTime) -> crate::Duration {
        self.0.signed_duration_since(*rhs).into()
    }

    /// Converts to a UTC datetime by assuming this naive datetime is in UTC.
    #[instrument(skip(self))]
    pub fn and_utc(&self) -> crate::DateTime {
        self.0.and_utc().into()
    }

    /// Returns the Unix timestamp in milliseconds (treating this as UTC).
    #[instrument(skip(self))]
    pub fn timestamp_millis(&self) -> i64 {
        self.0.and_utc().timestamp_millis()
    }

    /// Returns the Unix timestamp in nanoseconds, or `None` if out of range (treating this as UTC).
    #[instrument(skip(self))]
    pub fn timestamp_nanos_opt(&self) -> Option<i64> {
        self.0.and_utc().timestamp_nanos_opt()
    }

    /// Returns the date component.
    #[instrument(skip(self))]
    pub fn date(&self) -> crate::NaiveDate {
        self.0.date().into()
    }

    /// Returns the time component.
    #[instrument(skip(self))]
    pub fn time(&self) -> crate::NaiveTime {
        self.0.time().into()
    }

    /// Returns `self + n days`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_add_days(&self, days: u64) -> Option<NaiveDateTime> {
        self.0
            .checked_add_days(chrono::Days::new(days))
            .map(Into::into)
    }

    /// Returns `self - n days`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_sub_days(&self, days: u64) -> Option<NaiveDateTime> {
        self.0
            .checked_sub_days(chrono::Days::new(days))
            .map(Into::into)
    }

    /// Returns `self + n months`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_add_months(&self, months: u32) -> Option<NaiveDateTime> {
        self.0
            .checked_add_months(chrono::Months::new(months))
            .map(Into::into)
    }

    /// Returns `self - n months`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_sub_months(&self, months: u32) -> Option<NaiveDateTime> {
        self.0
            .checked_sub_months(chrono::Months::new(months))
            .map(Into::into)
    }

    /// Returns `self + offset_secs` as a fixed offset adjustment, or `None` on overflow.
    ///
    /// `offset_secs` is the offset east of UTC in seconds (negative = west).
    #[instrument(skip(self))]
    pub fn checked_add_offset(&self, offset_secs: i32) -> Option<NaiveDateTime> {
        chrono::FixedOffset::east_opt(offset_secs)
            .and_then(|off| self.0.checked_add_offset(off))
            .map(Into::into)
    }

    /// Returns `self - offset_secs` as a fixed offset adjustment, or `None` on overflow.
    ///
    /// `offset_secs` is the offset east of UTC in seconds (negative = west).
    #[instrument(skip(self))]
    pub fn checked_sub_offset(&self, offset_secs: i32) -> Option<NaiveDateTime> {
        chrono::FixedOffset::east_opt(offset_secs)
            .and_then(|off| self.0.checked_sub_offset(off))
            .map(Into::into)
    }

    /// Rounds `self` to the nearest multiple of `duration`, or `None` if `duration` is zero or negative.
    #[instrument(skip(self))]
    pub fn duration_round(&self, duration: crate::Duration) -> Option<NaiveDateTime> {
        use chrono::DurationRound;
        self.0.duration_round(*duration).ok().map(Into::into)
    }

    /// Rounds `self` up to the next multiple of `duration`, or `None` if `duration` is zero or negative.
    #[instrument(skip(self))]
    pub fn duration_round_up(&self, duration: crate::Duration) -> Option<NaiveDateTime> {
        use chrono::DurationRound;
        self.0.duration_round_up(*duration).ok().map(Into::into)
    }

    /// Truncates `self` down to the last multiple of `duration`, or `None` if `duration` is zero or negative.
    #[instrument(skip(self))]
    pub fn duration_trunc(&self, duration: crate::Duration) -> Option<NaiveDateTime> {
        use chrono::DurationRound;
        self.0.duration_trunc(*duration).ok().map(Into::into)
    }

    /// Returns the day of the month (0-indexed; 0 = first day of month).
    #[instrument(skip(self))]
    pub fn day0(&self) -> u32 {
        self.0.day0()
    }

    /// Returns the month (0-indexed; 0 = January).
    #[instrument(skip(self))]
    pub fn month0(&self) -> u32 {
        self.0.month0()
    }

    /// Returns the day of the year (0-indexed; 0 = first day of year).
    #[instrument(skip(self))]
    pub fn ordinal0(&self) -> u32 {
        self.0.ordinal0()
    }

    /// Returns the ISO 8601 week date as `"YYYY-Www-D"` (e.g. `"2024-W03-1"`).
    #[instrument(skip(self))]
    pub fn iso_week(&self) -> String {
        let w = self.0.iso_week();
        format!(
            "{:04}-W{:02}-{}",
            w.year(),
            w.week(),
            self.0.weekday().num_days_from_monday() + 1
        )
    }

    /// Formats the datetime using a pre-parsed format spec (delegates to strftime format string).
    #[instrument(skip(self))]
    pub fn format_with_items(&self, fmt: String) -> String {
        self.0.format(&fmt).to_string()
    }

    /// Converts to a `DateTimeFixed` by attaching the given UTC offset (seconds east).
    ///
    /// Returns `None` if `offset_secs` is out of range or the result is ambiguous.
    #[instrument(skip(self))]
    pub fn and_local_timezone(&self, offset_secs: i32) -> Option<crate::DateTimeFixed> {
        chrono::FixedOffset::east_opt(offset_secs)
            .and_then(|tz| self.0.and_local_timezone(tz).single())
            .map(Into::into)
    }

    /// Returns `self + rhs` (wraps on overflow).
    #[instrument(skip(self))]
    pub fn add(&self, rhs: crate::Duration) -> NaiveDateTime {
        (*self.0 + *rhs).into()
    }

    /// Returns `self - rhs` (wraps on overflow).
    #[instrument(skip(self))]
    pub fn sub(&self, rhs: crate::Duration) -> NaiveDateTime {
        (*self.0 - *rhs).into()
    }

    /// Returns a copy with the year replaced, or `None` if the resulting date is invalid.
    #[instrument(skip(self))]
    pub fn with_year(&self, year: i32) -> Option<NaiveDateTime> {
        self.0.with_year(year).map(Into::into)
    }

    /// Returns a copy with the month replaced (1–12), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_month(&self, month: u32) -> Option<NaiveDateTime> {
        self.0.with_month(month).map(Into::into)
    }

    /// Returns a copy with the month replaced (0-indexed; 0–11), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_month0(&self, month0: u32) -> Option<NaiveDateTime> {
        self.0.with_month0(month0).map(Into::into)
    }

    /// Returns a copy with the day replaced (1–31), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_day(&self, day: u32) -> Option<NaiveDateTime> {
        self.0.with_day(day).map(Into::into)
    }

    /// Returns a copy with the day replaced (0-indexed; 0–30), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_day0(&self, day0: u32) -> Option<NaiveDateTime> {
        self.0.with_day0(day0).map(Into::into)
    }

    /// Returns a copy with the day-of-year replaced (1–366), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_ordinal(&self, ordinal: u32) -> Option<NaiveDateTime> {
        self.0.with_ordinal(ordinal).map(Into::into)
    }

    /// Returns a copy with the day-of-year replaced (0-indexed; 0–365), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_ordinal0(&self, ordinal0: u32) -> Option<NaiveDateTime> {
        self.0.with_ordinal0(ordinal0).map(Into::into)
    }

    /// Returns a copy with the hour replaced (0–23), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_hour(&self, hour: u32) -> Option<NaiveDateTime> {
        self.0.with_hour(hour).map(Into::into)
    }

    /// Returns a copy with the minute replaced (0–59), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_minute(&self, minute: u32) -> Option<NaiveDateTime> {
        self.0.with_minute(minute).map(Into::into)
    }

    /// Returns a copy with the second replaced (0–59), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_second(&self, second: u32) -> Option<NaiveDateTime> {
        self.0.with_second(second).map(Into::into)
    }

    /// Returns a copy with the nanosecond component replaced (0–999_999_999), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_nanosecond(&self, nano: u32) -> Option<NaiveDateTime> {
        self.0.with_nanosecond(nano).map(Into::into)
    }
}

impl NaiveDateTime {
    /// Construct from a `NaiveDate` and `NaiveTime`.
    pub fn new(date: crate::NaiveDate, time: crate::NaiveTime) -> Self {
        chrono::NaiveDateTime::new(*date, *time).into()
    }

    /// Parse an ISO 8601 string (e.g. `"2024-01-15T12:30:00"`). Returns `None` if invalid.
    pub fn parse(s: &str) -> Option<Self> {
        chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
            .or_else(|_| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f"))
            .ok()
            .map(|dt| std::sync::Arc::new(dt).into())
    }

    /// Parse an ISO 8601 datetime string (delegates to `parse`). Returns `None` if invalid.
    pub fn from_str(s: &str) -> Option<Self> {
        Self::parse(s)
    }

    /// Returns the default datetime (January 1, year 1 at midnight).
    pub fn default() -> Self {
        chrono::NaiveDateTime::default().into()
    }

    /// Parse a datetime string with format, returning `(datetime, unparsed_remainder)`. Returns `None` if invalid.
    pub fn parse_and_remainder(s: &str, fmt: &str) -> Option<(Self, String)> {
        chrono::NaiveDateTime::parse_and_remainder(s, fmt)
            .ok()
            .map(|(dt, rem)| (dt.into(), rem.to_string()))
    }

    /// Parse a datetime string using the given strftime format. Returns `None` if invalid.
    pub fn parse_from_str(s: &str, fmt: &str) -> Option<Self> {
        chrono::NaiveDateTime::parse_from_str(s, fmt)
            .ok()
            .map(Into::into)
    }

    /// Construct from a Unix timestamp in microseconds. Returns `None` if out of range.
    pub fn from_timestamp_micros(micros: i64) -> Option<Self> {
        chrono::DateTime::from_timestamp_micros(micros).map(|dt| dt.naive_utc().into())
    }

    /// Construct from a Unix timestamp in milliseconds. Returns `None` if out of range.
    pub fn from_timestamp_millis(millis: i64) -> Option<Self> {
        chrono::DateTime::from_timestamp_millis(millis).map(|dt| dt.naive_utc().into())
    }

    /// Construct from a Unix timestamp in whole seconds plus nanoseconds.
    ///
    /// Returns `None` if the values are out of range.
    pub fn from_timestamp_opt(secs: i64, nsecs: u32) -> Option<Self> {
        chrono::DateTime::from_timestamp(secs, nsecs).map(|dt| dt.naive_utc().into())
    }
}

impl std::ops::AddAssign<crate::Duration> for NaiveDateTime {
    fn add_assign(&mut self, rhs: crate::Duration) {
        self.0 = std::sync::Arc::new(*self.0 + *rhs);
    }
}

impl std::ops::SubAssign<crate::Duration> for NaiveDateTime {
    fn sub_assign(&mut self, rhs: crate::Duration) {
        self.0 = std::sync::Arc::new(*self.0 - *rhs);
    }
}

mod emit_impls {
    use super::NaiveDateTime;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for NaiveDateTime {
        fn to_code_literal(&self) -> TokenStream {
            let s = self.0.to_string();
            quote::quote! {
                ::elicit_chrono::NaiveDateTime::from(
                    ::chrono::NaiveDateTime::parse_from_str(#s, "%Y-%m-%d %H:%M:%S%.f")
                        .expect("valid NaiveDateTime")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for NaiveDateTime {}
