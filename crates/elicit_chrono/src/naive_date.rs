//! `NaiveDate` — elicitation-enabled wrapper around `chrono::NaiveDate`.

use chrono::Datelike;
use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(chrono::NaiveDate, as NaiveDate, serde);
elicitation::elicit_newtype_traits!(NaiveDate, chrono::NaiveDate, [cmp]);

#[reflect_methods]
impl NaiveDate {
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

    /// Returns the month (0-indexed; 0 = January).
    #[instrument(skip(self))]
    pub fn month0(&self) -> u32 {
        self.0.month0()
    }

    /// Returns the day of the month (1–31).
    #[instrument(skip(self))]
    pub fn day(&self) -> u32 {
        self.0.day()
    }

    /// Returns the day of the month (0-indexed; 0 = first day of month).
    #[instrument(skip(self))]
    pub fn day0(&self) -> u32 {
        self.0.day0()
    }

    /// Returns the day of the year (1–366).
    #[instrument(skip(self))]
    pub fn ordinal(&self) -> u32 {
        self.0.ordinal()
    }

    /// Returns the day of the year (0-indexed; 0 = first day of year).
    #[instrument(skip(self))]
    pub fn ordinal0(&self) -> u32 {
        self.0.ordinal0()
    }

    /// Returns the weekday name (e.g. `"Monday"`).
    #[instrument(skip(self))]
    pub fn weekday(&self) -> String {
        self.0.weekday().to_string()
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

    /// Formats the date using the given strftime format string.
    #[instrument(skip(self))]
    pub fn format(&self, fmt: String) -> String {
        self.0.format(&fmt).to_string()
    }

    /// Returns the previous calendar day, or `None` if this is the minimum date.
    #[instrument(skip(self))]
    pub fn pred_opt(&self) -> Option<NaiveDate> {
        self.0.pred_opt().map(Into::into)
    }

    /// Returns the next calendar day, or `None` if this is the maximum date.
    #[instrument(skip(self))]
    pub fn succ_opt(&self) -> Option<NaiveDate> {
        self.0.succ_opt().map(Into::into)
    }

    /// Returns a copy with the year replaced, or `None` if the resulting date is invalid.
    #[instrument(skip(self))]
    pub fn with_year(&self, year: i32) -> Option<NaiveDate> {
        self.0.with_year(year).map(Into::into)
    }

    /// Returns a copy with the month replaced (1–12), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_month(&self, month: u32) -> Option<NaiveDate> {
        self.0.with_month(month).map(Into::into)
    }

    /// Returns a copy with the month replaced (0-indexed; 0–11), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_month0(&self, month0: u32) -> Option<NaiveDate> {
        self.0.with_month0(month0).map(Into::into)
    }

    /// Returns a copy with the day replaced (1–31), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_day(&self, day: u32) -> Option<NaiveDate> {
        self.0.with_day(day).map(Into::into)
    }

    /// Returns a copy with the day replaced (0-indexed; 0–30), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_day0(&self, day0: u32) -> Option<NaiveDate> {
        self.0.with_day0(day0).map(Into::into)
    }

    /// Returns a copy with the day-of-year replaced (1–366), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_ordinal(&self, ordinal: u32) -> Option<NaiveDate> {
        self.0.with_ordinal(ordinal).map(Into::into)
    }

    /// Returns a copy with the day-of-year replaced (0-indexed; 0–365), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_ordinal0(&self, ordinal0: u32) -> Option<NaiveDate> {
        self.0.with_ordinal0(ordinal0).map(Into::into)
    }

    /// Returns `self + n days`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_add_days(&self, days: u64) -> Option<NaiveDate> {
        self.0
            .checked_add_days(chrono::Days::new(days))
            .map(Into::into)
    }

    /// Returns `self - n days`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_sub_days(&self, days: u64) -> Option<NaiveDate> {
        self.0
            .checked_sub_days(chrono::Days::new(days))
            .map(Into::into)
    }

    /// Returns `self + n months`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_add_months(&self, months: u32) -> Option<NaiveDate> {
        self.0
            .checked_add_months(chrono::Months::new(months))
            .map(Into::into)
    }

    /// Returns `self - n months`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_sub_months(&self, months: u32) -> Option<NaiveDate> {
        self.0
            .checked_sub_months(chrono::Months::new(months))
            .map(Into::into)
    }

    /// Combines this date with a time to produce a `NaiveDateTime`.
    #[instrument(skip(self))]
    pub fn and_time(&self, time: crate::NaiveTime) -> crate::NaiveDateTime {
        self.0.and_time(*time).into()
    }

    /// Formats the date using a pre-parsed format spec (delegates to strftime format string).
    #[instrument(skip(self))]
    pub fn format_with_items(&self, fmt: String) -> String {
        self.0.format(&fmt).to_string()
    }

    /// Returns `self + rhs` (wraps Duration addition onto the date).
    #[instrument(skip(self))]
    pub fn add(&self, rhs: crate::Duration) -> NaiveDate {
        (*self.0 + *rhs).into()
    }

    /// Returns `self - rhs` (subtracts a Duration from the date).
    #[instrument(skip(self))]
    pub fn sub(&self, rhs: crate::Duration) -> NaiveDate {
        (*self.0 - *rhs).into()
    }

    /// Returns `self + rhs`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_add_signed(&self, rhs: crate::Duration) -> Option<NaiveDate> {
        self.0.checked_add_signed(*rhs).map(Into::into)
    }

    /// Returns `self - rhs`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_sub_signed(&self, rhs: crate::Duration) -> Option<NaiveDate> {
        self.0.checked_sub_signed(*rhs).map(Into::into)
    }

    /// Returns the signed duration between `self` and `rhs` (positive if `self` is later).
    #[instrument(skip(self))]
    pub fn signed_duration_since(&self, rhs: crate::NaiveDate) -> crate::Duration {
        self.0.signed_duration_since(*rhs).into()
    }

    /// Returns `true` if the year of this date is a leap year.
    #[instrument(skip(self))]
    pub fn leap_year(&self) -> bool {
        self.0.leap_year()
    }

    /// Returns the number of whole years elapsed since `rhs`, or `None` if `rhs` is later than `self`.
    #[instrument(skip(self))]
    pub fn years_since(&self, rhs: crate::NaiveDate) -> Option<u32> {
        self.0.years_since(*rhs)
    }

    /// Returns the week containing this date as `"YYYY-MM-DD .. YYYY-MM-DD"`.
    ///
    /// `start` is the name of the first day of the week (e.g. `"Mon"`, `"Sun"`).
    /// Returns `None` if `start` is not a valid weekday name.
    #[instrument(skip(self))]
    pub fn week(&self, start: String) -> Option<String> {
        let wd: chrono::Weekday = start.parse().ok()?;
        let w = self.0.week(wd);
        Some(format!("{} .. {}", w.first_day(), w.last_day()))
    }

    /// Combines this date with hour, minute, and second. Returns `None` if any value is out of range.
    #[instrument(skip(self))]
    pub fn and_hms_opt(&self, hour: u32, min: u32, sec: u32) -> Option<crate::NaiveDateTime> {
        self.0.and_hms_opt(hour, min, sec).map(Into::into)
    }

    /// Combines this date with hour, minute, second, and nanosecond. Returns `None` if invalid.
    #[instrument(skip(self))]
    pub fn and_hms_nano_opt(
        &self,
        hour: u32,
        min: u32,
        sec: u32,
        nano: u32,
    ) -> Option<crate::NaiveDateTime> {
        self.0
            .and_hms_nano_opt(hour, min, sec, nano)
            .map(Into::into)
    }

    /// Combines this date with hour, minute, second, and millisecond. Returns `None` if invalid.
    #[instrument(skip(self))]
    pub fn and_hms_milli_opt(
        &self,
        hour: u32,
        min: u32,
        sec: u32,
        milli: u32,
    ) -> Option<crate::NaiveDateTime> {
        self.0
            .and_hms_milli_opt(hour, min, sec, milli)
            .map(Into::into)
    }

    /// Combines this date with hour, minute, second, and microsecond. Returns `None` if invalid.
    #[instrument(skip(self))]
    pub fn and_hms_micro_opt(
        &self,
        hour: u32,
        min: u32,
        sec: u32,
        micro: u32,
    ) -> Option<crate::NaiveDateTime> {
        self.0
            .and_hms_micro_opt(hour, min, sec, micro)
            .map(Into::into)
    }
}

impl NaiveDate {
    /// Construct from year, month (1–12), and day (1–31). Returns `None` if invalid.
    pub fn from_ymd_opt(year: i32, month: u32, day: u32) -> Option<Self> {
        chrono::NaiveDate::from_ymd_opt(year, month, day).map(Into::into)
    }

    /// Construct from year and day-of-year (1–366). Returns `None` if invalid.
    pub fn from_ordinal_opt(year: i32, ordinal: u32) -> Option<Self> {
        chrono::NaiveDate::from_yo_opt(year, ordinal).map(Into::into)
    }

    /// Construct from year and day-of-year ordinal (1–366). Returns `None` if invalid.
    pub fn from_yo_opt(year: i32, ordinal: u32) -> Option<Self> {
        chrono::NaiveDate::from_yo_opt(year, ordinal).map(Into::into)
    }

    /// Parse an ISO 8601 date string (e.g. `"2024-01-15"`). Returns `None` if invalid.
    pub fn from_str(s: &str) -> Option<Self> {
        Self::parse(s)
    }

    /// Returns the default date (January 1, year 1 in the proleptic Gregorian calendar).
    pub fn default() -> Self {
        chrono::NaiveDate::default().into()
    }

    /// Parse a date string with format, returning `(date, unparsed_remainder)`. Returns `None` if invalid.
    pub fn parse_and_remainder(s: &str, fmt: &str) -> Option<(Self, String)> {
        chrono::NaiveDate::parse_and_remainder(s, fmt)
            .ok()
            .map(|(d, rem)| (d.into(), rem.to_string()))
    }

    /// Parse an ISO 8601 date string (e.g. `"2024-01-15"`). Returns `None` if invalid.
    pub fn parse(s: &str) -> Option<Self> {
        chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .ok()
            .map(Into::into)
    }

    /// Parse a date string using the given strftime format. Returns `None` if invalid.
    pub fn parse_from_str(s: &str, fmt: &str) -> Option<Self> {
        chrono::NaiveDate::parse_from_str(s, fmt)
            .ok()
            .map(Into::into)
    }

    /// Construct from number of days since January 1, year 1 (proleptic Gregorian). Returns `None` if out of range.
    pub fn from_num_days_from_ce_opt(days: i32) -> Option<Self> {
        chrono::NaiveDate::from_num_days_from_ce_opt(days).map(Into::into)
    }

    /// Construct from ISO year, week number, and weekday name (e.g. `"Mon"`). Returns `None` if invalid.
    pub fn from_isoywd_opt(year: i32, week: u32, weekday: &str) -> Option<Self> {
        let wd: chrono::Weekday = weekday.parse().ok()?;
        chrono::NaiveDate::from_isoywd_opt(year, week, wd).map(Into::into)
    }

    /// Construct the `n`th occurrence of `weekday` (e.g. `"Mon"`) in the given month/year.
    /// Returns `None` if invalid or no such occurrence exists.
    pub fn from_weekday_of_month_opt(year: i32, month: u32, weekday: &str, n: u8) -> Option<Self> {
        let wd: chrono::Weekday = weekday.parse().ok()?;
        chrono::NaiveDate::from_weekday_of_month_opt(year, month, wd, n).map(Into::into)
    }
}

impl NaiveDate {
    /// Returns `self` as the starting date for a day-by-day iteration sequence.
    ///
    /// To create a stateful iterator that advances across MCP tool calls, use
    /// `NaiveDateIterPlugin` — call `chrono_date_iter__iter_days` with this date's
    /// ISO 8601 string, then advance with `chrono_date_iter__next`.
    pub fn iter_days(&self) -> crate::NaiveDate {
        self.clone()
    }

    /// Returns `self` as the starting date for a week-by-week iteration sequence.
    ///
    /// To create a stateful iterator that advances across MCP tool calls, use
    /// `NaiveDateIterPlugin` — call `chrono_date_iter__iter_weeks` with this date's
    /// ISO 8601 string, then advance with `chrono_date_iter__next`.
    pub fn iter_weeks(&self) -> crate::NaiveDate {
        self.clone()
    }
}

impl std::ops::AddAssign<crate::Duration> for NaiveDate {
    fn add_assign(&mut self, rhs: crate::Duration) {
        self.0 = std::sync::Arc::new(*self.0 + *rhs);
    }
}

impl std::ops::SubAssign<crate::Duration> for NaiveDate {
    fn sub_assign(&mut self, rhs: crate::Duration) {
        self.0 = std::sync::Arc::new(*self.0 - *rhs);
    }
}

mod emit_impls {
    use super::NaiveDate;
    use chrono::Datelike;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for NaiveDate {
        fn to_code_literal(&self) -> TokenStream {
            let y = self.0.year();
            let m = self.0.month();
            let d = self.0.day();
            quote::quote! {
                ::elicit_chrono::NaiveDate::from_ymd_opt(#y, #m, #d)
                    .expect("valid NaiveDate")
            }
        }
    }
}

impl elicitation::ElicitComplete for NaiveDate {}
