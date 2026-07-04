//! `DateTimeFixed` — elicitation-enabled wrapper around `chrono::DateTime<chrono::FixedOffset>`.

use std::sync::Arc;

use chrono::{Datelike, Timelike};
use elicitation_derive::reflect_methods;
use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// Elicitation-enabled wrapper around `chrono::DateTime<chrono::FixedOffset>`.
///
/// Serializes to/from RFC 3339 strings (e.g. `"2024-01-15T12:30:00+05:30"`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DateTimeFixed(pub Arc<chrono::DateTime<chrono::FixedOffset>>);

impl JsonSchema for DateTimeFixed {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "DateTimeFixed".into()
    }

    fn json_schema(schema_gen: &mut SchemaGenerator) -> Schema {
        <chrono::DateTime<chrono::FixedOffset> as JsonSchema>::json_schema(schema_gen)
    }
}

impl std::ops::Deref for DateTimeFixed {
    type Target = chrono::DateTime<chrono::FixedOffset>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::convert::AsRef<chrono::DateTime<chrono::FixedOffset>> for DateTimeFixed {
    fn as_ref(&self) -> &chrono::DateTime<chrono::FixedOffset> {
        &self.0
    }
}

impl From<chrono::DateTime<chrono::FixedOffset>> for DateTimeFixed {
    fn from(inner: chrono::DateTime<chrono::FixedOffset>) -> Self {
        Self(Arc::new(inner))
    }
}

impl From<Arc<chrono::DateTime<chrono::FixedOffset>>> for DateTimeFixed {
    fn from(arc: Arc<chrono::DateTime<chrono::FixedOffset>>) -> Self {
        Self(arc)
    }
}

#[reflect_methods]
impl DateTimeFixed {
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

    /// Returns the Unix timestamp in whole seconds.
    #[instrument(skip(self))]
    pub fn timestamp(&self) -> i64 {
        self.0.timestamp()
    }

    /// Returns the Unix timestamp in milliseconds.
    #[instrument(skip(self))]
    pub fn timestamp_millis(&self) -> i64 {
        self.0.timestamp_millis()
    }

    /// Returns the Unix timestamp in nanoseconds, or `None` if out of range.
    #[instrument(skip(self))]
    pub fn timestamp_nanos_opt(&self) -> Option<i64> {
        self.0.timestamp_nanos_opt()
    }

    /// Returns the UTC offset in seconds east of UTC (negative = west).
    #[instrument(skip(self))]
    pub fn offset_seconds(&self) -> i32 {
        use chrono::Offset;
        self.0.offset().fix().local_minus_utc()
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

    /// Returns the RFC 3339 string representation.
    #[instrument(skip(self))]
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }

    /// Converts to the naive local datetime (stripping timezone info).
    #[instrument(skip(self))]
    pub fn naive_local(&self) -> crate::NaiveDateTime {
        Arc::new(self.0.naive_local()).into()
    }

    /// Converts to the naive UTC datetime (shifts to UTC, then strips timezone info).
    #[instrument(skip(self))]
    pub fn naive_utc(&self) -> crate::NaiveDateTime {
        Arc::new(self.0.naive_utc()).into()
    }

    /// Returns the RFC 2822 string representation.
    #[instrument(skip(self))]
    pub fn to_rfc2822(&self) -> String {
        self.0.to_rfc2822()
    }

    /// Returns the Unix timestamp in microseconds.
    #[instrument(skip(self))]
    pub fn timestamp_micros(&self) -> i64 {
        self.0.timestamp_micros()
    }

    /// Returns the fractional part of the timestamp in microseconds (0–999_999).
    #[instrument(skip(self))]
    pub fn timestamp_subsec_micros(&self) -> u32 {
        self.0.timestamp_subsec_micros()
    }

    /// Returns the fractional part of the timestamp in milliseconds (0–999).
    #[instrument(skip(self))]
    pub fn timestamp_subsec_millis(&self) -> u32 {
        self.0.timestamp_subsec_millis()
    }

    /// Returns the fractional part of the timestamp in nanoseconds (0–999_999_999).
    #[instrument(skip(self))]
    pub fn timestamp_subsec_nanos(&self) -> u32 {
        self.0.timestamp_subsec_nanos()
    }

    /// Returns `self + rhs`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_add_signed(&self, rhs: crate::Duration) -> Option<crate::DateTimeFixed> {
        self.0.checked_add_signed(*rhs).map(Into::into)
    }

    /// Returns `self - rhs`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_sub_signed(&self, rhs: crate::Duration) -> Option<crate::DateTimeFixed> {
        self.0.checked_sub_signed(*rhs).map(Into::into)
    }

    /// Returns the signed duration between `self` and `rhs` (positive if `self` is later).
    #[instrument(skip(self))]
    pub fn signed_duration_since(&self, rhs: crate::DateTimeFixed) -> crate::Duration {
        self.0.signed_duration_since(*rhs).into()
    }

    /// Converts to UTC, shifting the instant to the UTC timezone.
    #[instrument(skip(self))]
    pub fn to_utc(&self) -> crate::DateTime {
        self.0.to_utc().into()
    }

    /// Formats the datetime using the given strftime format string.
    ///
    /// See <https://docs.rs/chrono/latest/chrono/format/strftime/index.html> for format tokens.
    #[instrument(skip(self))]
    pub fn format(&self, fmt: String) -> String {
        self.0.format(&fmt).to_string()
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

    /// Returns `self + rhs` (wraps on overflow).
    #[instrument(skip(self))]
    pub fn add(&self, rhs: crate::Duration) -> crate::DateTimeFixed {
        (*self.0 + *rhs).into()
    }

    /// Returns `self - rhs` (wraps on overflow).
    #[instrument(skip(self))]
    pub fn sub(&self, rhs: crate::Duration) -> crate::DateTimeFixed {
        (*self.0 - *rhs).into()
    }

    /// Returns a copy with the year replaced, or `None` if the resulting date is invalid.
    #[instrument(skip(self))]
    pub fn with_year(&self, year: i32) -> Option<crate::DateTimeFixed> {
        self.0.with_year(year).map(Into::into)
    }

    /// Returns a copy with the month replaced (1–12), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_month(&self, month: u32) -> Option<crate::DateTimeFixed> {
        self.0.with_month(month).map(Into::into)
    }

    /// Returns a copy with the month replaced (0-indexed; 0–11), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_month0(&self, month0: u32) -> Option<crate::DateTimeFixed> {
        self.0.with_month0(month0).map(Into::into)
    }

    /// Returns a copy with the day replaced (1–31), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_day(&self, day: u32) -> Option<crate::DateTimeFixed> {
        self.0.with_day(day).map(Into::into)
    }

    /// Returns a copy with the day replaced (0-indexed; 0–30), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_day0(&self, day0: u32) -> Option<crate::DateTimeFixed> {
        self.0.with_day0(day0).map(Into::into)
    }

    /// Returns a copy with the day-of-year replaced (1–366), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_ordinal(&self, ordinal: u32) -> Option<crate::DateTimeFixed> {
        self.0.with_ordinal(ordinal).map(Into::into)
    }

    /// Returns a copy with the day-of-year replaced (0-indexed; 0–365), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_ordinal0(&self, ordinal0: u32) -> Option<crate::DateTimeFixed> {
        self.0.with_ordinal0(ordinal0).map(Into::into)
    }

    /// Returns a copy with the hour replaced (0–23), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_hour(&self, hour: u32) -> Option<crate::DateTimeFixed> {
        self.0.with_hour(hour).map(Into::into)
    }

    /// Returns a copy with the minute replaced (0–59), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_minute(&self, minute: u32) -> Option<crate::DateTimeFixed> {
        self.0.with_minute(minute).map(Into::into)
    }

    /// Returns a copy with the second replaced (0–59), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_second(&self, second: u32) -> Option<crate::DateTimeFixed> {
        self.0.with_second(second).map(Into::into)
    }

    /// Returns a copy with the nanosecond component replaced (0–999_999_999), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_nanosecond(&self, nano: u32) -> Option<crate::DateTimeFixed> {
        self.0.with_nanosecond(nano).map(Into::into)
    }

    /// Returns the naive date component.
    #[instrument(skip(self))]
    pub fn date(&self) -> crate::NaiveDate {
        self.0.date_naive().into()
    }

    /// Returns the naive time component.
    #[instrument(skip(self))]
    pub fn time(&self) -> crate::NaiveTime {
        self.0.time().into()
    }

    /// Returns `self + n days`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_add_days(&self, days: u64) -> Option<crate::DateTimeFixed> {
        self.0
            .checked_add_days(chrono::Days::new(days))
            .map(Into::into)
    }

    /// Returns `self - n days`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_sub_days(&self, days: u64) -> Option<crate::DateTimeFixed> {
        self.0
            .checked_sub_days(chrono::Days::new(days))
            .map(Into::into)
    }

    /// Returns `self + n months`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_add_months(&self, months: u32) -> Option<crate::DateTimeFixed> {
        self.0
            .checked_add_months(chrono::Months::new(months))
            .map(Into::into)
    }

    /// Returns `self - n months`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_sub_months(&self, months: u32) -> Option<crate::DateTimeFixed> {
        self.0
            .checked_sub_months(chrono::Months::new(months))
            .map(Into::into)
    }

    /// Rounds `self` to the nearest multiple of `duration`, or `None` if `duration` is zero or negative.
    #[instrument(skip(self))]
    pub fn duration_round(&self, duration: crate::Duration) -> Option<crate::DateTimeFixed> {
        use chrono::DurationRound;
        self.0.duration_round(*duration).ok().map(Into::into)
    }

    /// Rounds `self` up to the next multiple of `duration`, or `None` if `duration` is zero or negative.
    #[instrument(skip(self))]
    pub fn duration_round_up(&self, duration: crate::Duration) -> Option<crate::DateTimeFixed> {
        use chrono::DurationRound;
        self.0.duration_round_up(*duration).ok().map(Into::into)
    }

    /// Truncates `self` down to the last multiple of `duration`, or `None` if `duration` is zero or negative.
    #[instrument(skip(self))]
    pub fn duration_trunc(&self, duration: crate::Duration) -> Option<crate::DateTimeFixed> {
        use chrono::DurationRound;
        self.0.duration_trunc(*duration).ok().map(Into::into)
    }
}

impl DateTimeFixed {
    /// Parse an RFC 3339 string with a fixed offset. Returns `None` if invalid.
    pub fn parse(s: &str) -> Option<Self> {
        chrono::DateTime::parse_from_rfc3339(s).ok().map(Into::into)
    }
}

// ── Elicitation framework traits — delegate to `chrono::DateTime<FixedOffset>` core impls ──

impl elicitation::Prompt for DateTimeFixed {
    fn prompt() -> Option<&'static str> {
        <chrono::DateTime<chrono::FixedOffset> as elicitation::Prompt>::prompt()
    }
}

impl elicitation::Elicitation for DateTimeFixed {
    type Style = <chrono::DateTime<chrono::FixedOffset> as elicitation::Elicitation>::Style;

    async fn elicit<C: elicitation::ElicitCommunicator>(
        communicator: &C,
    ) -> elicitation::ElicitResult<Self> {
        chrono::DateTime::<chrono::FixedOffset>::elicit(communicator)
            .await
            .map(Into::into)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <chrono::DateTime<chrono::FixedOffset> as elicitation::Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <chrono::DateTime<chrono::FixedOffset> as elicitation::Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <chrono::DateTime<chrono::FixedOffset> as elicitation::Elicitation>::creusot_proof()
    }
}

impl elicitation::ElicitIntrospect for DateTimeFixed {
    fn pattern() -> elicitation::ElicitationPattern {
        <chrono::DateTime<chrono::FixedOffset> as elicitation::ElicitIntrospect>::pattern()
    }
    fn metadata() -> elicitation::TypeMetadata {
        <chrono::DateTime<chrono::FixedOffset> as elicitation::ElicitIntrospect>::metadata()
    }
}

impl elicitation::ElicitPromptTree for DateTimeFixed {
    fn prompt_tree() -> elicitation::PromptTree {
        <chrono::DateTime<chrono::FixedOffset> as elicitation::ElicitPromptTree>::prompt_tree()
    }
}

impl elicitation::ElicitSpec for DateTimeFixed {
    fn type_spec() -> elicitation::TypeSpec {
        <chrono::DateTime<chrono::FixedOffset> as elicitation::ElicitSpec>::type_spec()
    }
}

mod emit_impls {
    use super::DateTimeFixed;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for DateTimeFixed {
        fn to_code_literal(&self) -> TokenStream {
            let s = self.0.to_rfc3339();
            quote::quote! {
                ::elicit_chrono::DateTimeFixed::from(
                    #s.parse::<::chrono::DateTime<::chrono::FixedOffset>>()
                        .expect("valid fixed datetime")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for DateTimeFixed {}
