//! `DateTime` — elicitation-enabled wrapper around `chrono::DateTime<chrono::Utc>`.

use std::sync::Arc;

use chrono::{Datelike, Timelike};
use elicitation_derive::reflect_methods;
use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// Elicitation-enabled wrapper around `chrono::DateTime<chrono::Utc>`.
///
/// Serializes to/from RFC 3339 strings (e.g. `"2024-01-15T12:30:00Z"`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DateTime(pub Arc<chrono::DateTime<chrono::Utc>>);

impl JsonSchema for DateTime {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "DateTime".into()
    }

    fn json_schema(schema_gen: &mut SchemaGenerator) -> Schema {
        <chrono::DateTime<chrono::Utc> as JsonSchema>::json_schema(schema_gen)
    }
}

impl std::ops::Deref for DateTime {
    type Target = chrono::DateTime<chrono::Utc>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::convert::AsRef<chrono::DateTime<chrono::Utc>> for DateTime {
    fn as_ref(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.0
    }
}

impl From<chrono::DateTime<chrono::Utc>> for DateTime {
    fn from(inner: chrono::DateTime<chrono::Utc>) -> Self {
        Self(Arc::new(inner))
    }
}

impl From<Arc<chrono::DateTime<chrono::Utc>>> for DateTime {
    fn from(arc: Arc<chrono::DateTime<chrono::Utc>>) -> Self {
        Self(arc)
    }
}

#[reflect_methods]
impl DateTime {
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

    /// Returns the RFC 3339 string representation (e.g. `"2024-01-15T12:30:00Z"`).
    #[instrument(skip(self))]
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339()
    }

    /// Returns the RFC 2822 string representation.
    #[instrument(skip(self))]
    pub fn to_rfc2822(&self) -> String {
        self.0.to_rfc2822()
    }

    /// Converts to the naive UTC datetime (strips timezone info).
    #[instrument(skip(self))]
    pub fn naive_utc(&self) -> crate::NaiveDateTime {
        Arc::new(self.0.naive_utc()).into()
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
    pub fn checked_add_signed(&self, rhs: crate::Duration) -> Option<crate::DateTime> {
        self.0.checked_add_signed(*rhs).map(Into::into)
    }

    /// Returns `self - rhs`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_sub_signed(&self, rhs: crate::Duration) -> Option<crate::DateTime> {
        self.0.checked_sub_signed(*rhs).map(Into::into)
    }

    /// Returns the signed duration between `self` and `rhs` (positive if `self` is later).
    #[instrument(skip(self))]
    pub fn signed_duration_since(&self, rhs: crate::DateTime) -> crate::Duration {
        self.0.signed_duration_since(*rhs).into()
    }

    /// Converts to a `DateTimeFixed` with UTC offset (+00:00).
    #[instrument(skip(self))]
    pub fn fixed_offset(&self) -> crate::DateTimeFixed {
        self.0.fixed_offset().into()
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
    pub fn add(&self, rhs: crate::Duration) -> crate::DateTime {
        (*self.0 + *rhs).into()
    }

    /// Returns `self - rhs` (wraps on overflow).
    #[instrument(skip(self))]
    pub fn sub(&self, rhs: crate::Duration) -> crate::DateTime {
        (*self.0 - *rhs).into()
    }

    /// Returns a copy with the year replaced, or `None` if the resulting date is invalid.
    #[instrument(skip(self))]
    pub fn with_year(&self, year: i32) -> Option<crate::DateTime> {
        self.0.with_year(year).map(Into::into)
    }

    /// Returns a copy with the month replaced (1–12), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_month(&self, month: u32) -> Option<crate::DateTime> {
        self.0.with_month(month).map(Into::into)
    }

    /// Returns a copy with the month replaced (0-indexed; 0–11), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_month0(&self, month0: u32) -> Option<crate::DateTime> {
        self.0.with_month0(month0).map(Into::into)
    }

    /// Returns a copy with the day replaced (1–31), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_day(&self, day: u32) -> Option<crate::DateTime> {
        self.0.with_day(day).map(Into::into)
    }

    /// Returns a copy with the day replaced (0-indexed; 0–30), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_day0(&self, day0: u32) -> Option<crate::DateTime> {
        self.0.with_day0(day0).map(Into::into)
    }

    /// Returns a copy with the day-of-year replaced (1–366), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_ordinal(&self, ordinal: u32) -> Option<crate::DateTime> {
        self.0.with_ordinal(ordinal).map(Into::into)
    }

    /// Returns a copy with the day-of-year replaced (0-indexed; 0–365), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_ordinal0(&self, ordinal0: u32) -> Option<crate::DateTime> {
        self.0.with_ordinal0(ordinal0).map(Into::into)
    }

    /// Returns a copy with the hour replaced (0–23), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_hour(&self, hour: u32) -> Option<crate::DateTime> {
        self.0.with_hour(hour).map(Into::into)
    }

    /// Returns a copy with the minute replaced (0–59), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_minute(&self, minute: u32) -> Option<crate::DateTime> {
        self.0.with_minute(minute).map(Into::into)
    }

    /// Returns a copy with the second replaced (0–59), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_second(&self, second: u32) -> Option<crate::DateTime> {
        self.0.with_second(second).map(Into::into)
    }

    /// Returns a copy with the nanosecond component replaced (0–999_999_999), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_nanosecond(&self, nano: u32) -> Option<crate::DateTime> {
        self.0.with_nanosecond(nano).map(Into::into)
    }

    /// Returns the naive date component (local/UTC — same for UTC datetimes).
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
    pub fn checked_add_days(&self, days: u64) -> Option<crate::DateTime> {
        self.0
            .checked_add_days(chrono::Days::new(days))
            .map(Into::into)
    }

    /// Returns `self - n days`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_sub_days(&self, days: u64) -> Option<crate::DateTime> {
        self.0
            .checked_sub_days(chrono::Days::new(days))
            .map(Into::into)
    }

    /// Returns `self + n months`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_add_months(&self, months: u32) -> Option<crate::DateTime> {
        self.0
            .checked_add_months(chrono::Months::new(months))
            .map(Into::into)
    }

    /// Returns `self - n months`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_sub_months(&self, months: u32) -> Option<crate::DateTime> {
        self.0
            .checked_sub_months(chrono::Months::new(months))
            .map(Into::into)
    }

    /// Rounds `self` to the nearest multiple of `duration`, or `None` if `duration` is zero or negative.
    #[instrument(skip(self))]
    pub fn duration_round(&self, duration: crate::Duration) -> Option<crate::DateTime> {
        use chrono::DurationRound;
        self.0.duration_round(*duration).ok().map(Into::into)
    }

    /// Rounds `self` up to the next multiple of `duration`, or `None` if `duration` is zero or negative.
    #[instrument(skip(self))]
    pub fn duration_round_up(&self, duration: crate::Duration) -> Option<crate::DateTime> {
        use chrono::DurationRound;
        self.0.duration_round_up(*duration).ok().map(Into::into)
    }

    /// Truncates `self` down to the last multiple of `duration`, or `None` if `duration` is zero or negative.
    #[instrument(skip(self))]
    pub fn duration_trunc(&self, duration: crate::Duration) -> Option<crate::DateTime> {
        use chrono::DurationRound;
        self.0.duration_trunc(*duration).ok().map(Into::into)
    }

    /// Returns the naive date portion (for UTC datetimes, identical to the UTC date).
    #[instrument(skip(self))]
    pub fn date_naive(&self) -> crate::NaiveDate {
        self.0.date_naive().into()
    }

    /// Returns the naive local datetime (for UTC datetimes, identical to `naive_utc`).
    #[instrument(skip(self))]
    pub fn naive_local(&self) -> crate::NaiveDateTime {
        self.0.naive_local().into()
    }

    /// Returns the UTC offset as a formatted string (always `"+00:00"` for UTC datetimes).
    #[instrument(skip(self))]
    pub fn offset(&self) -> String {
        self.0.offset().to_string()
    }

    /// Returns the timezone name (always `"UTC"` for this type).
    #[instrument(skip(self))]
    pub fn timezone(&self) -> String {
        "UTC".to_string()
    }

    /// Returns this datetime in UTC (identity for UTC datetimes).
    #[instrument(skip(self))]
    pub fn to_utc(&self) -> crate::DateTime {
        self.0.to_utc().into()
    }

    /// Returns the RFC 3339 string with configurable sub-second precision.
    ///
    /// `secs_format`: `"Secs"`, `"Millis"`, `"Micros"`, `"Nanos"`, or `"AutoSi"`.
    /// `use_z`: render UTC as `"Z"` (true) or `"+00:00"` (false).
    #[instrument(skip(self))]
    pub fn to_rfc3339_opts(&self, secs_format: String, use_z: bool) -> String {
        use chrono::SecondsFormat;
        let sf = match secs_format.as_str() {
            "Millis" => SecondsFormat::Millis,
            "Micros" => SecondsFormat::Micros,
            "Nanos" => SecondsFormat::Nanos,
            "AutoSi" => SecondsFormat::AutoSi,
            _ => SecondsFormat::Secs,
        };
        self.0.to_rfc3339_opts(sf, use_z)
    }

    /// Returns `self` with the time component replaced. Returns `None` on ambiguous local time.
    #[instrument(skip(self))]
    pub fn with_time(&self, time: crate::NaiveTime) -> Option<crate::DateTime> {
        match self.0.with_time(*time) {
            chrono::MappedLocalTime::Single(dt) | chrono::MappedLocalTime::Ambiguous(dt, _) => {
                Some(dt.into())
            }
            chrono::MappedLocalTime::None => None,
        }
    }

    /// Returns the number of whole years elapsed since `rhs`, or `None` if `rhs` is later than `self`.
    #[instrument(skip(self))]
    pub fn years_since(&self, rhs: crate::DateTime) -> Option<u32> {
        self.0.years_since(*rhs)
    }

    /// Formats the datetime using a pre-parsed format spec (delegates to strftime format string).
    #[instrument(skip(self))]
    pub fn format_with_items(&self, fmt: String) -> String {
        self.0.format(&fmt).to_string()
    }

    /// Converts to a `DateTimeFixed` by applying the given UTC offset in seconds.
    ///
    /// Returns `None` if `offset_secs` is out of the valid range (−86399..=86399).
    #[instrument(skip(self))]
    pub fn with_timezone(&self, offset_secs: i32) -> Option<crate::DateTimeFixed> {
        chrono::FixedOffset::east_opt(offset_secs).map(|tz| self.0.with_timezone(&tz).into())
    }
}

impl DateTime {
    /// Returns the current UTC time.
    pub fn now() -> Self {
        chrono::Utc::now().into()
    }

    /// Parse an RFC 3339 string. Returns `None` if the string is invalid.
    pub fn parse(s: &str) -> Option<Self> {
        s.parse::<chrono::DateTime<chrono::Utc>>()
            .ok()
            .map(Into::into)
    }

    /// Parse an RFC 2822 string and convert to UTC. Returns `None` if invalid.
    pub fn parse_from_rfc2822(s: &str) -> Option<Self> {
        chrono::DateTime::parse_from_rfc2822(s)
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc).into())
    }

    /// Parse an RFC 3339 string and convert to UTC. Returns `None` if invalid.
    pub fn parse_from_rfc3339(s: &str) -> Option<Self> {
        chrono::DateTime::parse_from_rfc3339(s)
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc).into())
    }

    /// Parse a datetime string with the given strftime format and convert to UTC. Returns `None` if invalid.
    pub fn parse_from_str(s: &str, fmt: &str) -> Option<Self> {
        chrono::DateTime::parse_from_str(s, fmt)
            .ok()
            .map(|dt| dt.with_timezone(&chrono::Utc).into())
    }

    /// Construct from a Unix timestamp in microseconds (UTC). Returns `None` if out of range.
    pub fn from_timestamp_micros(micros: i64) -> Option<Self> {
        chrono::DateTime::from_timestamp_micros(micros).map(Into::into)
    }

    /// Construct from a Unix timestamp in milliseconds (UTC). Returns `None` if out of range.
    pub fn from_timestamp_millis(millis: i64) -> Option<Self> {
        chrono::DateTime::from_timestamp_millis(millis).map(Into::into)
    }

    /// Construct from a naive UTC datetime and UTC offset.
    pub fn from_naive_utc_and_offset(dt: crate::NaiveDateTime) -> Self {
        chrono::DateTime::from_naive_utc_and_offset(*dt, chrono::Utc).into()
    }

    /// Construct from a naive UTC datetime (deprecated upstream alias for `from_naive_utc_and_offset`).
    pub fn from_utc(dt: crate::NaiveDateTime) -> Self {
        chrono::DateTime::from_naive_utc_and_offset(*dt, chrono::Utc).into()
    }

    /// Construct from a naive local datetime treated as UTC (equivalent to `from_naive_utc_and_offset`).
    pub fn from_local(dt: crate::NaiveDateTime) -> Self {
        chrono::DateTime::from_naive_utc_and_offset(*dt, chrono::Utc).into()
    }

    /// Parse an RFC 3339 string (delegates to `parse`). Returns `None` if invalid.
    pub fn from_str(s: &str) -> Option<Self> {
        Self::parse(s)
    }

    /// Returns the default datetime (Unix epoch, `1970-01-01T00:00:00Z`).
    pub fn default() -> Self {
        chrono::DateTime::<chrono::Utc>::default().into()
    }

    /// Parse a datetime string with format, returning `(datetime, unparsed_remainder)`. Returns `None` if invalid.
    pub fn parse_and_remainder(s: &str, fmt: &str) -> Option<(Self, String)> {
        chrono::DateTime::<chrono::FixedOffset>::parse_and_remainder(s, fmt)
            .ok()
            .map(|(dt, rem)| (dt.with_timezone(&chrono::Utc).into(), rem.to_string()))
    }
}

impl std::ops::AddAssign<crate::Duration> for DateTime {
    fn add_assign(&mut self, rhs: crate::Duration) {
        self.0 = std::sync::Arc::new(*self.0 + *rhs);
    }
}

impl std::ops::SubAssign<crate::Duration> for DateTime {
    fn sub_assign(&mut self, rhs: crate::Duration) {
        self.0 = std::sync::Arc::new(*self.0 - *rhs);
    }
}

// ── Elicitation framework traits — delegate to `chrono::DateTime<Utc>` core impls ──

impl elicitation::Prompt for DateTime {
    fn prompt() -> Option<&'static str> {
        <chrono::DateTime<chrono::Utc> as elicitation::Prompt>::prompt()
    }
}

impl elicitation::Elicitation for DateTime {
    type Style = <chrono::DateTime<chrono::Utc> as elicitation::Elicitation>::Style;

    async fn elicit<C: elicitation::ElicitCommunicator>(
        communicator: &C,
    ) -> elicitation::ElicitResult<Self> {
        chrono::DateTime::<chrono::Utc>::elicit(communicator)
            .await
            .map(Into::into)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <chrono::DateTime<chrono::Utc> as elicitation::Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <chrono::DateTime<chrono::Utc> as elicitation::Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <chrono::DateTime<chrono::Utc> as elicitation::Elicitation>::creusot_proof()
    }
}

impl elicitation::ElicitIntrospect for DateTime {
    fn pattern() -> elicitation::ElicitationPattern {
        <chrono::DateTime<chrono::Utc> as elicitation::ElicitIntrospect>::pattern()
    }
    fn metadata() -> elicitation::TypeMetadata {
        <chrono::DateTime<chrono::Utc> as elicitation::ElicitIntrospect>::metadata()
    }
}

impl elicitation::ElicitPromptTree for DateTime {
    fn prompt_tree() -> elicitation::PromptTree {
        <chrono::DateTime<chrono::Utc> as elicitation::ElicitPromptTree>::prompt_tree()
    }
}

impl elicitation::ElicitSpec for DateTime {
    fn type_spec() -> elicitation::TypeSpec {
        <chrono::DateTime<chrono::Utc> as elicitation::ElicitSpec>::type_spec()
    }
}

mod emit_impls {
    use super::DateTime;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for DateTime {
        fn to_code_literal(&self) -> TokenStream {
            let s = self.0.to_rfc3339();
            quote::quote! {
                ::elicit_chrono::DateTime::from(
                    #s.parse::<::chrono::DateTime<::chrono::Utc>>()
                        .expect("valid UTC datetime")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for DateTime {}
