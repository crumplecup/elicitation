//! `OffsetDateTime` — elicitation-enabled wrapper around `time::OffsetDateTime`.

use std::sync::Arc;

use elicitation_derive::reflect_methods;
use schemars::{JsonSchema, SchemaGenerator, json_schema};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tracing::instrument;

/// Elicitation-enabled wrapper around `time::OffsetDateTime`.
///
/// Serializes to/from RFC 3339 strings (e.g. `"2024-01-15T12:30:00+00:00"`).
#[derive(Debug, Clone)]
pub struct OffsetDateTime(pub Arc<time::OffsetDateTime>);

impl JsonSchema for OffsetDateTime {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "OffsetDateTime".into()
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "string",
            "format": "date-time",
            "description": "RFC 3339 date-time string with UTC offset (e.g. \"2024-01-15T12:30:00+00:00\")"
        })
    }
}

impl Serialize for OffsetDateTime {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let formatted = self
            .0
            .format(&time::format_description::well_known::Rfc3339)
            .map_err(serde::ser::Error::custom)?;
        s.serialize_str(&formatted)
    }
}

impl<'de> Deserialize<'de> for OffsetDateTime {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        time::OffsetDateTime::parse(&s, &time::format_description::well_known::Rfc3339)
            .map(|dt| Arc::new(dt).into())
            .map_err(serde::de::Error::custom)
    }
}

impl std::ops::Deref for OffsetDateTime {
    type Target = time::OffsetDateTime;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::convert::AsRef<time::OffsetDateTime> for OffsetDateTime {
    fn as_ref(&self) -> &time::OffsetDateTime {
        &self.0
    }
}

impl From<time::OffsetDateTime> for OffsetDateTime {
    fn from(inner: time::OffsetDateTime) -> Self {
        Self(Arc::new(inner))
    }
}

impl From<Arc<time::OffsetDateTime>> for OffsetDateTime {
    fn from(arc: Arc<time::OffsetDateTime>) -> Self {
        Self(arc)
    }
}

#[reflect_methods]
impl OffsetDateTime {
    /// Returns the year component.
    #[instrument(skip(self))]
    pub fn year(&self) -> i32 {
        self.0.year()
    }

    /// Returns the month as a number (1 = January … 12 = December).
    #[instrument(skip(self))]
    pub fn month(&self) -> u8 {
        u8::from(self.0.month())
    }

    /// Returns the day of the month (1–31).
    #[instrument(skip(self))]
    pub fn day(&self) -> u8 {
        self.0.day()
    }

    /// Returns the hour (0–23).
    #[instrument(skip(self))]
    pub fn hour(&self) -> u8 {
        self.0.hour()
    }

    /// Returns the minute (0–59).
    #[instrument(skip(self))]
    pub fn minute(&self) -> u8 {
        self.0.minute()
    }

    /// Returns the second (0–59).
    #[instrument(skip(self))]
    pub fn second(&self) -> u8 {
        self.0.second()
    }

    /// Returns the nanosecond component (0–999_999_999).
    #[instrument(skip(self))]
    pub fn nanosecond(&self) -> u32 {
        self.0.nanosecond()
    }

    /// Returns the Unix timestamp (whole seconds since 1970-01-01T00:00:00Z).
    #[instrument(skip(self))]
    pub fn unix_timestamp(&self) -> i64 {
        self.0.unix_timestamp()
    }

    /// Returns the UTC offset as a string (e.g. `"+05:30"` or `"-08:00"`).
    #[instrument(skip(self))]
    pub fn utc_offset(&self) -> String {
        let off = self.0.offset();
        let (h, m, s) = off.as_hms();
        let sign = if off.is_negative() { '-' } else { '+' };
        format!("{sign}{h:02}:{m:02}:{s:02}")
    }

    /// Returns the RFC 3339 string representation.
    #[instrument(skip(self))]
    pub fn to_rfc3339(&self) -> String {
        self.0
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap_or_default()
    }
}

impl OffsetDateTime {
    /// Returns the current time in UTC.
    pub fn now_utc() -> Self {
        time::OffsetDateTime::now_utc().into()
    }

    /// Parse an RFC 3339 string. Returns `None` if the string is invalid.
    pub fn parse(s: &str) -> Option<Self> {
        time::OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339)
            .ok()
            .map(Into::into)
    }
}
