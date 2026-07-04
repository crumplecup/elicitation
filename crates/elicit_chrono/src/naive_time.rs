//! `NaiveTime` — elicitation-enabled wrapper around `chrono::NaiveTime`.

use chrono::Timelike;
use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(chrono::NaiveTime, as NaiveTime, serde);
elicitation::elicit_newtype_traits!(NaiveTime, chrono::NaiveTime, [cmp]);

#[reflect_methods]
impl NaiveTime {
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

    /// Returns the number of non-leap seconds since midnight (0–86399).
    #[instrument(skip(self))]
    pub fn num_seconds_from_midnight(&self) -> u32 {
        self.0.num_seconds_from_midnight()
    }

    /// Formats the time using the given strftime format string.
    #[instrument(skip(self))]
    pub fn format(&self, fmt: String) -> String {
        self.0.format(&fmt).to_string()
    }

    /// Returns a copy with the hour replaced (0–23), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_hour(&self, hour: u32) -> Option<NaiveTime> {
        self.0.with_hour(hour).map(Into::into)
    }

    /// Returns a copy with the minute replaced (0–59), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_minute(&self, minute: u32) -> Option<NaiveTime> {
        self.0.with_minute(minute).map(Into::into)
    }

    /// Returns a copy with the second replaced (0–59), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_second(&self, second: u32) -> Option<NaiveTime> {
        self.0.with_second(second).map(Into::into)
    }

    /// Returns a copy with the nanosecond component replaced (0–999_999_999), or `None` if invalid.
    #[instrument(skip(self))]
    pub fn with_nanosecond(&self, nano: u32) -> Option<NaiveTime> {
        self.0.with_nanosecond(nano).map(Into::into)
    }

    /// Returns `self + rhs` (wraps on overflow), with the number of whole days the result overflowed.
    ///
    /// Returns `(new_time, days_overflowed)` where `days_overflowed` is positive if the addition
    /// crossed midnight forward, negative if backward.
    #[instrument(skip(self))]
    pub fn overflowing_add_signed(&self, rhs: crate::Duration) -> (NaiveTime, i64) {
        let (t, days) = self.0.overflowing_add_signed(*rhs);
        (t.into(), days)
    }

    /// Returns `self - rhs` (wraps on overflow), with the number of whole days the result underflowed.
    ///
    /// Returns `(new_time, days_underflowed)` where `days_underflowed` is negative if subtraction
    /// crossed midnight backward.
    #[instrument(skip(self))]
    pub fn overflowing_sub_signed(&self, rhs: crate::Duration) -> (NaiveTime, i64) {
        let (t, days) = self.0.overflowing_sub_signed(*rhs);
        (t.into(), days)
    }

    /// Returns the signed duration between `self` and `rhs` (positive if `self` is later).
    #[instrument(skip(self))]
    pub fn signed_duration_since(&self, rhs: NaiveTime) -> crate::Duration {
        self.0.signed_duration_since(*rhs).into()
    }

    /// Returns `self + rhs` (wraps on overflow).
    #[instrument(skip(self))]
    pub fn add(&self, rhs: crate::Duration) -> NaiveTime {
        (*self.0 + *rhs).into()
    }

    /// Returns `self - rhs` (wraps on overflow).
    #[instrument(skip(self))]
    pub fn sub(&self, rhs: crate::Duration) -> NaiveTime {
        (*self.0 - *rhs).into()
    }

    /// Formats the time using a pre-parsed format spec (delegates to strftime format string).
    #[instrument(skip(self))]
    pub fn format_with_items(&self, fmt: String) -> String {
        self.0.format(&fmt).to_string()
    }
}

impl NaiveTime {
    /// Construct from hour, minute, and second. Returns `None` if any value is out of range.
    pub fn from_hms_opt(hour: u32, minute: u32, second: u32) -> Option<Self> {
        chrono::NaiveTime::from_hms_opt(hour, minute, second).map(Into::into)
    }

    /// Construct from hour, minute, second, and nanosecond. Returns `None` if invalid.
    pub fn from_hms_nano_opt(hour: u32, minute: u32, second: u32, nano: u32) -> Option<Self> {
        chrono::NaiveTime::from_hms_nano_opt(hour, minute, second, nano).map(Into::into)
    }

    /// Parse an ISO 8601 time string (e.g. `"12:30:45"`). Returns `None` if invalid.
    pub fn parse(s: &str) -> Option<Self> {
        chrono::NaiveTime::parse_from_str(s, "%H:%M:%S")
            .or_else(|_| chrono::NaiveTime::parse_from_str(s, "%H:%M:%S%.f"))
            .ok()
            .map(Into::into)
    }

    /// Parse a time string using the given strftime format. Returns `None` if invalid.
    pub fn parse_from_str(s: &str, fmt: &str) -> Option<Self> {
        chrono::NaiveTime::parse_from_str(s, fmt)
            .ok()
            .map(Into::into)
    }

    /// Parse an ISO 8601 time string (delegates to `parse`). Returns `None` if invalid.
    pub fn from_str(s: &str) -> Option<Self> {
        Self::parse(s)
    }

    /// Returns the default time (midnight, `00:00:00`).
    pub fn default() -> Self {
        chrono::NaiveTime::default().into()
    }

    /// Parse a time string with format, returning `(time, unparsed_remainder)`. Returns `None` if invalid.
    pub fn parse_and_remainder(s: &str, fmt: &str) -> Option<(Self, String)> {
        chrono::NaiveTime::parse_and_remainder(s, fmt)
            .ok()
            .map(|(t, rem)| (t.into(), rem.to_string()))
    }

    /// Construct from hour, minute, second, and microsecond. Returns `None` if invalid.
    pub fn from_hms_micro_opt(hour: u32, minute: u32, second: u32, micro: u32) -> Option<Self> {
        chrono::NaiveTime::from_hms_micro_opt(hour, minute, second, micro).map(Into::into)
    }

    /// Construct from hour, minute, second, and millisecond. Returns `None` if invalid.
    pub fn from_hms_milli_opt(hour: u32, minute: u32, second: u32, milli: u32) -> Option<Self> {
        chrono::NaiveTime::from_hms_milli_opt(hour, minute, second, milli).map(Into::into)
    }

    /// Construct from the number of non-leap seconds since midnight plus nanoseconds.
    /// Returns `None` if out of range.
    pub fn from_num_seconds_from_midnight_opt(secs: u32, nano: u32) -> Option<Self> {
        chrono::NaiveTime::from_num_seconds_from_midnight_opt(secs, nano).map(Into::into)
    }
}

impl std::ops::AddAssign<crate::Duration> for NaiveTime {
    fn add_assign(&mut self, rhs: crate::Duration) {
        self.0 = std::sync::Arc::new(*self.0 + *rhs);
    }
}

impl std::ops::SubAssign<crate::Duration> for NaiveTime {
    fn sub_assign(&mut self, rhs: crate::Duration) {
        self.0 = std::sync::Arc::new(*self.0 - *rhs);
    }
}

mod emit_impls {
    use super::NaiveTime;
    use chrono::Timelike;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for NaiveTime {
        fn to_code_literal(&self) -> TokenStream {
            let h = self.0.hour();
            let m = self.0.minute();
            let s = self.0.second();
            let ns = self.0.nanosecond();
            quote::quote! {
                ::elicit_chrono::NaiveTime::from_hms_nano_opt(#h, #m, #s, #ns)
                    .expect("valid NaiveTime")
            }
        }
    }
}

impl elicitation::ElicitComplete for NaiveTime {}
