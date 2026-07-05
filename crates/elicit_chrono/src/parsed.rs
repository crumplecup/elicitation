//! `Parsed` — shadow for `chrono::format::Parsed`.

use elicitation_derive::reflect_methods;
use schemars::{Schema, SchemaGenerator};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tracing::instrument;

/// Shadow for [`chrono::format::Parsed`] — a collection of parsed date/time fields.
///
/// Serializes and deserializes via [`elicitation::ParsedWrap`] (JSON object with optional fields).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Parsed(pub chrono::format::Parsed);

impl Serialize for Parsed {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        elicitation::ParsedWrap::from(self.0.clone()).serialize(s)
    }
}

impl<'de> Deserialize<'de> for Parsed {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        elicitation::ParsedWrap::deserialize(d).map(|w| Self(w.into_inner()))
    }
}

impl schemars::JsonSchema for Parsed {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Parsed".into()
    }
    fn json_schema(schema_gen: &mut SchemaGenerator) -> Schema {
        elicitation::ParsedWrap::json_schema(schema_gen)
    }
}

impl std::hash::Hash for Parsed {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.year.hash(state);
        self.0.month.hash(state);
        self.0.day.hash(state);
        self.0.hour_div_12.hash(state);
        self.0.hour_mod_12.hash(state);
        self.0.minute.hash(state);
        self.0.second.hash(state);
        self.0.nanosecond.hash(state);
        self.0.timestamp.hash(state);
    }
}

impl Parsed {
    /// Creates a new, empty `Parsed`.
    #[instrument]
    pub fn new() -> Parsed {
        Parsed(chrono::format::Parsed::new())
    }
}

#[reflect_methods]
impl Parsed {
    // ── Setters ───────────────────────────────────────────────────────────────

    /// Sets the full year. Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_year(&self, year: i64) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_year(year)
            .map(|()| Parsed(p))
            .map_err(|e| e.to_string())
    }

    /// Sets the year modulo 100 (two-digit year). Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_year_div_100(&self, v: i64) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_year_div_100(v)
            .map(|()| Parsed(p))
            .map_err(|e| e.to_string())
    }

    /// Sets the year divided by 100 (century). Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_year_mod_100(&self, v: i64) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_year_mod_100(v)
            .map(|()| Parsed(p))
            .map_err(|e| e.to_string())
    }

    /// Sets the ISO year. Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_isoyear(&self, v: i64) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_isoyear(v)
            .map(|()| Parsed(p))
            .map_err(|e| e.to_string())
    }

    /// Sets the ISO year divided by 100. Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_isoyear_div_100(&self, v: i64) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_isoyear_div_100(v)
            .map(|()| Parsed(p))
            .map_err(|e| e.to_string())
    }

    /// Sets the ISO year modulo 100. Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_isoyear_mod_100(&self, v: i64) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_isoyear_mod_100(v)
            .map(|()| Parsed(p))
            .map_err(|e| e.to_string())
    }

    /// Sets the month (1–12). Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_month(&self, v: i64) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_month(v)
            .map(|()| Parsed(p))
            .map_err(|e| e.to_string())
    }

    /// Sets the quarter (1–4). Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_quarter(&self, v: i64) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_quarter(v)
            .map(|()| Parsed(p))
            .map_err(|e| e.to_string())
    }

    /// Sets the week number from Sunday (0–53). Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_week_from_sun(&self, v: i64) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_week_from_sun(v)
            .map(|()| Parsed(p))
            .map_err(|e| e.to_string())
    }

    /// Sets the week number from Monday (0–53). Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_week_from_mon(&self, v: i64) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_week_from_mon(v)
            .map(|()| Parsed(p))
            .map_err(|e| e.to_string())
    }

    /// Sets the ISO week number (1–53). Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_isoweek(&self, v: i64) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_isoweek(v)
            .map(|()| Parsed(p))
            .map_err(|e| e.to_string())
    }

    /// Sets the weekday. Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_weekday(&self, day: crate::Weekday) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_weekday(*day.0)
            .map(|()| Parsed(p))
            .map_err(|e| e.to_string())
    }

    /// Sets the day of year (1–366). Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_ordinal(&self, v: i64) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_ordinal(v)
            .map(|()| Parsed(p))
            .map_err(|e| e.to_string())
    }

    /// Sets the day of month (1–31). Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_day(&self, v: i64) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_day(v).map(|()| Parsed(p)).map_err(|e| e.to_string())
    }

    /// Sets AM/PM (false = AM, true = PM). Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_ampm(&self, ampm: bool) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_ampm(ampm)
            .map(|()| Parsed(p))
            .map_err(|e| e.to_string())
    }

    /// Sets the 12-hour clock component (1–12). Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_hour12(&self, v: i64) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_hour12(v)
            .map(|()| Parsed(p))
            .map_err(|e| e.to_string())
    }

    /// Sets the 24-hour clock component (0–23). Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_hour(&self, v: i64) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_hour(v).map(|()| Parsed(p)).map_err(|e| e.to_string())
    }

    /// Sets the minute (0–59). Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_minute(&self, v: i64) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_minute(v)
            .map(|()| Parsed(p))
            .map_err(|e| e.to_string())
    }

    /// Sets the second (0–60). Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_second(&self, v: i64) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_second(v)
            .map(|()| Parsed(p))
            .map_err(|e| e.to_string())
    }

    /// Sets the nanosecond within a second. Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_nanosecond(&self, v: i64) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_nanosecond(v)
            .map(|()| Parsed(p))
            .map_err(|e| e.to_string())
    }

    /// Sets the Unix timestamp in seconds. Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_timestamp(&self, v: i64) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_timestamp(v)
            .map(|()| Parsed(p))
            .map_err(|e| e.to_string())
    }

    /// Sets the UTC offset in seconds. Returns an error if conflicting.
    #[instrument(skip(self))]
    pub fn set_offset(&self, v: i64) -> Result<Parsed, String> {
        let mut p = self.0.clone();
        p.set_offset(v)
            .map(|()| Parsed(p))
            .map_err(|e| e.to_string())
    }

    // ── Getters ───────────────────────────────────────────────────────────────

    /// Returns the full year, if set.
    #[instrument(skip(self))]
    pub fn year(&self) -> Option<i32> {
        self.0.year
    }

    /// Returns the year divided by 100, if set.
    #[instrument(skip(self))]
    pub fn year_div_100(&self) -> Option<i32> {
        self.0.year_div_100
    }

    /// Returns the year modulo 100, if set.
    #[instrument(skip(self))]
    pub fn year_mod_100(&self) -> Option<i32> {
        self.0.year_mod_100
    }

    /// Returns the ISO year, if set.
    #[instrument(skip(self))]
    pub fn isoyear(&self) -> Option<i32> {
        self.0.isoyear
    }

    /// Returns the ISO year divided by 100, if set.
    #[instrument(skip(self))]
    pub fn isoyear_div_100(&self) -> Option<i32> {
        self.0.isoyear_div_100
    }

    /// Returns the ISO year modulo 100, if set.
    #[instrument(skip(self))]
    pub fn isoyear_mod_100(&self) -> Option<i32> {
        self.0.isoyear_mod_100
    }

    /// Returns the month (1–12), if set.
    #[instrument(skip(self))]
    pub fn month(&self) -> Option<u32> {
        self.0.month
    }

    /// Returns the quarter (1–4), if set.
    #[instrument(skip(self))]
    pub fn quarter(&self) -> Option<u32> {
        self.0.quarter
    }

    /// Returns the week from Sunday, if set.
    #[instrument(skip(self))]
    pub fn week_from_sun(&self) -> Option<u32> {
        self.0.week_from_sun
    }

    /// Returns the week from Monday, if set.
    #[instrument(skip(self))]
    pub fn week_from_mon(&self) -> Option<u32> {
        self.0.week_from_mon
    }

    /// Returns the ISO week number, if set.
    #[instrument(skip(self))]
    pub fn isoweek(&self) -> Option<u32> {
        self.0.isoweek
    }

    /// Returns the weekday, if set.
    #[instrument(skip(self))]
    pub fn weekday(&self) -> Option<crate::Weekday> {
        self.0.weekday.map(crate::Weekday::from)
    }

    /// Returns the ordinal day (1–366), if set.
    #[instrument(skip(self))]
    pub fn ordinal(&self) -> Option<u32> {
        self.0.ordinal
    }

    /// Returns the day of month (1–31), if set.
    #[instrument(skip(self))]
    pub fn day(&self) -> Option<u32> {
        self.0.day
    }

    /// Returns the half-day flag (0 = AM, 1 = PM), if set.
    #[instrument(skip(self))]
    pub fn hour_div_12(&self) -> Option<u32> {
        self.0.hour_div_12
    }

    /// Returns the 12-hour clock component (1–12), if set.
    #[instrument(skip(self))]
    pub fn hour_mod_12(&self) -> Option<u32> {
        self.0.hour_mod_12
    }

    /// Returns the minute (0–59), if set.
    #[instrument(skip(self))]
    pub fn minute(&self) -> Option<u32> {
        self.0.minute
    }

    /// Returns the second (0–60), if set.
    #[instrument(skip(self))]
    pub fn second(&self) -> Option<u32> {
        self.0.second
    }

    /// Returns the nanosecond, if set.
    #[instrument(skip(self))]
    pub fn nanosecond(&self) -> Option<u32> {
        self.0.nanosecond
    }

    /// Returns the Unix timestamp in seconds, if set.
    #[instrument(skip(self))]
    pub fn timestamp(&self) -> Option<i64> {
        self.0.timestamp
    }

    /// Returns the UTC offset in seconds, if set.
    #[instrument(skip(self))]
    pub fn offset(&self) -> Option<i32> {
        self.0.offset
    }

    // ── Converters ────────────────────────────────────────────────────────────

    /// Converts to a [`crate::NaiveDate`], or returns an error string.
    #[instrument(skip(self))]
    pub fn to_naive_date(&self) -> Result<crate::NaiveDate, String> {
        self.0
            .to_naive_date()
            .map(crate::NaiveDate::from)
            .map_err(|e| e.to_string())
    }

    /// Converts to a [`crate::NaiveTime`], or returns an error string.
    #[instrument(skip(self))]
    pub fn to_naive_time(&self) -> Result<crate::NaiveTime, String> {
        self.0
            .to_naive_time()
            .map(crate::NaiveTime::from)
            .map_err(|e| e.to_string())
    }

    /// Converts to a [`crate::NaiveDateTime`] using the stored offset, or returns an error string.
    #[instrument(skip(self))]
    pub fn to_naive_datetime_with_offset(&self) -> Result<crate::NaiveDateTime, String> {
        self.0
            .to_naive_datetime_with_offset(self.0.offset.unwrap_or(0))
            .map(crate::NaiveDateTime::from)
            .map_err(|e| e.to_string())
    }

    /// Converts to a [`crate::FixedOffset`], or returns an error string.
    #[instrument(skip(self))]
    pub fn to_fixed_offset(&self) -> Result<crate::FixedOffset, String> {
        self.0
            .to_fixed_offset()
            .map(crate::FixedOffset::from)
            .map_err(|e| e.to_string())
    }

    /// Converts to a [`crate::DateTime`] (UTC), or returns an error string.
    #[instrument(skip(self))]
    pub fn to_datetime(&self) -> Result<crate::DateTime, String> {
        self.0
            .to_datetime()
            .map(|dt| crate::DateTime::from(dt.with_timezone(&chrono::Utc)))
            .map_err(|e| e.to_string())
    }

    /// Converts to a [`crate::DateTime`] using the given UTC offset in seconds.
    ///
    /// Returns the result in UTC.
    #[instrument(skip(self))]
    pub fn to_datetime_with_timezone(&self, offset_secs: i32) -> Result<crate::DateTime, String> {
        let tz = chrono::FixedOffset::east_opt(offset_secs)
            .ok_or_else(|| format!("offset {offset_secs} out of range"))?;
        self.0
            .to_datetime_with_timezone(&tz)
            .map(|dt| crate::DateTime::from(dt.with_timezone(&chrono::Utc)))
            .map_err(|e| e.to_string())
    }
}
