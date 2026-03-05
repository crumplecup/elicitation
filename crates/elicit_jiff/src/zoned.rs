//! `Zoned` — elicitation-enabled wrapper around `jiff::Zoned`.

use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(jiff::Zoned, as Zoned, serde);

#[reflect_methods]
impl Zoned {
    /// Returns the year component.
    #[instrument(skip(self))]
    pub fn year(&self) -> i16 {
        self.0.year()
    }

    /// Returns the month (1 = January … 12 = December).
    #[instrument(skip(self))]
    pub fn month(&self) -> i8 {
        self.0.month()
    }

    /// Returns the day of the month (1–31).
    #[instrument(skip(self))]
    pub fn day(&self) -> i8 {
        self.0.day()
    }

    /// Returns the hour (0–23).
    #[instrument(skip(self))]
    pub fn hour(&self) -> i8 {
        self.0.hour()
    }

    /// Returns the minute (0–59).
    #[instrument(skip(self))]
    pub fn minute(&self) -> i8 {
        self.0.minute()
    }

    /// Returns the second (0–59).
    #[instrument(skip(self))]
    pub fn second(&self) -> i8 {
        self.0.second()
    }

    /// Returns the nanosecond component (0–999).
    #[instrument(skip(self))]
    pub fn nanosecond(&self) -> i16 {
        self.0.nanosecond()
    }

    /// Returns the total subsecond nanosecond offset (0–999_999_999).
    #[instrument(skip(self))]
    pub fn subsec_nanosecond(&self) -> i32 {
        self.0.subsec_nanosecond()
    }

    /// Returns the day of the year (1–366).
    #[instrument(skip(self))]
    pub fn day_of_year(&self) -> i16 {
        self.0.day_of_year()
    }

    /// Returns the number of days in the current month.
    #[instrument(skip(self))]
    pub fn days_in_month(&self) -> i8 {
        self.0.days_in_month()
    }

    /// Returns the weekday name (e.g. `"Monday"`).
    #[instrument(skip(self))]
    pub fn weekday(&self) -> String {
        format!("{:?}", self.0.weekday())
    }

    /// Returns the IANA timezone name (e.g. `"America/New_York"`), or `"UTC"` for UTC.
    #[instrument(skip(self))]
    pub fn timezone_name(&self) -> String {
        self.0.time_zone().iana_name().unwrap_or("UTC").to_string()
    }

    /// Returns the Unix timestamp in whole seconds.
    #[instrument(skip(self))]
    pub fn timestamp_seconds(&self) -> i64 {
        self.0.timestamp().as_second()
    }

    /// Converts to the named timezone. Returns the RFC 9557 string of the result, or `None`.
    ///
    /// Example: `dt.in_tz("Europe/London")` → `Some("2024-01-15T12:30:00+00:00[Europe/London]")`
    #[instrument(skip(self))]
    pub fn in_tz(&self, name: String) -> Option<String> {
        self.0.in_tz(&name).ok().map(|z| z.to_string())
    }
}

impl Zoned {
    /// Returns the current date-time in the system timezone.
    pub fn now() -> Self {
        jiff::Zoned::now().into()
    }

    /// Parse an RFC 9557 / ISO 8601 string. Returns `None` if invalid.
    pub fn parse(s: &str) -> Option<Self> {
        s.parse::<jiff::Zoned>()
            .ok()
            .map(|z| std::sync::Arc::new(z).into())
    }
}
