//! `PrimitiveDateTime` — elicitation-enabled wrapper around `time::PrimitiveDateTime`.

use std::sync::Arc;

use elicitation_derive::reflect_methods;
use schemars::{JsonSchema, SchemaGenerator, json_schema};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tracing::instrument;

/// ISO 8601 local datetime format: `YYYY-MM-DDTHH:MM:SS`
const FORMAT: &[time::format_description::BorrowedFormatItem<'static>] =
    time::macros::format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");

/// Elicitation-enabled wrapper around `time::PrimitiveDateTime`.
///
/// Serializes to/from ISO 8601 local datetime strings without timezone
/// (e.g. `"2024-01-15T12:30:00"`).
#[derive(Debug, Clone)]
pub struct PrimitiveDateTime(pub Arc<time::PrimitiveDateTime>);

impl JsonSchema for PrimitiveDateTime {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "PrimitiveDateTime".into()
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "string",
            "description": "ISO 8601 local datetime without timezone (e.g. \"2024-01-15T12:30:00\")"
        })
    }
}

impl Serialize for PrimitiveDateTime {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let formatted = self.0.format(FORMAT).map_err(serde::ser::Error::custom)?;
        s.serialize_str(&formatted)
    }
}

impl<'de> Deserialize<'de> for PrimitiveDateTime {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        time::PrimitiveDateTime::parse(&s, FORMAT)
            .map(|dt| Arc::new(dt).into())
            .map_err(serde::de::Error::custom)
    }
}

impl std::ops::Deref for PrimitiveDateTime {
    type Target = time::PrimitiveDateTime;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::convert::AsRef<time::PrimitiveDateTime> for PrimitiveDateTime {
    fn as_ref(&self) -> &time::PrimitiveDateTime {
        &self.0
    }
}

impl From<time::PrimitiveDateTime> for PrimitiveDateTime {
    fn from(inner: time::PrimitiveDateTime) -> Self {
        Self(Arc::new(inner))
    }
}

impl From<Arc<time::PrimitiveDateTime>> for PrimitiveDateTime {
    fn from(arc: Arc<time::PrimitiveDateTime>) -> Self {
        Self(arc)
    }
}

#[reflect_methods]
impl PrimitiveDateTime {
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

    /// Returns the ISO 8601 string representation (no timezone).
    #[instrument(skip(self))]
    pub fn to_iso8601(&self) -> String {
        self.0.format(FORMAT).unwrap_or_default()
    }
}

impl PrimitiveDateTime {
    /// Parse an ISO 8601 local datetime string (`"YYYY-MM-DDTHH:MM:SS"`).
    /// Returns `None` if the string is invalid.
    pub fn parse(s: &str) -> Option<Self> {
        time::PrimitiveDateTime::parse(s, FORMAT)
            .ok()
            .map(Into::into)
    }
}
