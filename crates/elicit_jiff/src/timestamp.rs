//! `Timestamp` — elicitation-enabled wrapper around `jiff::Timestamp`.

use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(jiff::Timestamp, as Timestamp, serde);

#[reflect_methods]
impl Timestamp {
    /// Returns the number of whole seconds since the Unix epoch.
    #[instrument(skip(self))]
    pub fn as_second(&self) -> i64 {
        self.0.as_second()
    }

    /// Returns the number of whole milliseconds since the Unix epoch.
    #[instrument(skip(self))]
    pub fn as_millisecond(&self) -> i64 {
        self.0.as_millisecond()
    }

    /// Returns the number of whole microseconds since the Unix epoch.
    #[instrument(skip(self))]
    pub fn as_microsecond(&self) -> i64 {
        self.0.as_microsecond()
    }

    /// Returns the subsecond nanosecond offset (0–999_999_999).
    #[instrument(skip(self))]
    pub fn subsec_nanosecond(&self) -> i32 {
        self.0.subsec_nanosecond()
    }

    /// Returns `true` if this timestamp is zero (the Unix epoch).
    #[instrument(skip(self))]
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Returns `1` if positive, `-1` if negative, `0` if zero.
    #[instrument(skip(self))]
    pub fn signum(&self) -> i8 {
        self.0.signum()
    }

    /// Converts to the named timezone. Returns the RFC 9557 string of the result, or `None`.
    ///
    /// Example: `ts.in_tz("UTC")` → `Some("2024-01-15T12:30:00+00:00[UTC]")`
    #[instrument(skip(self))]
    pub fn in_tz(&self, name: String) -> Option<String> {
        self.0.in_tz(&name).ok().map(|z| z.to_string())
    }
}

impl Timestamp {
    /// Returns the current timestamp.
    pub fn now() -> Self {
        jiff::Timestamp::now().into()
    }

    /// Create a timestamp from whole seconds since the Unix epoch.
    /// Returns `None` if out of range.
    pub fn from_second(second: i64) -> Option<Self> {
        jiff::Timestamp::from_second(second)
            .ok()
            .map(|t| std::sync::Arc::new(t).into())
    }

    /// Parse an ISO 8601 / RFC 3339 string. Returns `None` if invalid.
    pub fn parse(s: &str) -> Option<Self> {
        s.parse::<jiff::Timestamp>()
            .ok()
            .map(|t| std::sync::Arc::new(t).into())
    }
}
