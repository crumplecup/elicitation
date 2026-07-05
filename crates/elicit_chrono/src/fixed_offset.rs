//! `FixedOffset` — shadow for `chrono::FixedOffset`.

use elicitation_derive::reflect_methods;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// Shadow for [`chrono::FixedOffset`].
///
/// Serializes and deserializes as seconds east of UTC (negative = west).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct FixedOffset(pub elicitation::FixedOffsetWrap);

impl From<chrono::FixedOffset> for FixedOffset {
    fn from(o: chrono::FixedOffset) -> Self {
        Self(elicitation::FixedOffsetWrap::from(o))
    }
}

impl std::fmt::Display for FixedOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            seconds_to_offset_string(self.local_minus_utc_raw())
        )
    }
}

impl std::str::FromStr for FixedOffset {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<chrono::FixedOffset>()
            .map(Self::from)
            .map_err(|e| e.to_string())
    }
}

fn seconds_to_offset_string(secs: i32) -> String {
    let sign = if secs >= 0 { '+' } else { '-' };
    let abs = secs.unsigned_abs();
    let h = abs / 3600;
    let m = (abs % 3600) / 60;
    let s = abs % 60;
    if s == 0 {
        format!("{sign}{h:02}:{m:02}")
    } else {
        format!("{sign}{h:02}:{m:02}:{s:02}")
    }
}

impl FixedOffset {
    fn local_minus_utc_raw(&self) -> i32 {
        match self.0.into_inner() {
            Some(o) => o.local_minus_utc(),
            None => 0,
        }
    }
}

impl FixedOffset {
    /// Constructs a `FixedOffset` for the given number of seconds east of UTC.
    ///
    /// Returns `None` if `secs` is outside `(-86400, 86400)`.
    #[instrument]
    pub fn east_opt(secs: i32) -> Option<FixedOffset> {
        chrono::FixedOffset::east_opt(secs).map(Self::from)
    }

    /// Constructs a `FixedOffset` for the given number of seconds west of UTC.
    ///
    /// Returns `None` if `secs` is outside `(-86400, 86400)`.
    #[instrument]
    pub fn west_opt(secs: i32) -> Option<FixedOffset> {
        chrono::FixedOffset::west_opt(secs).map(Self::from)
    }

    /// Constructs a `FixedOffset` east of UTC (deprecated; use `east_opt`).
    #[instrument]
    pub fn east(secs: i32) -> Option<FixedOffset> {
        FixedOffset::east_opt(secs)
    }

    /// Constructs a `FixedOffset` west of UTC (deprecated; use `west_opt`).
    #[instrument]
    pub fn west(secs: i32) -> Option<FixedOffset> {
        FixedOffset::west_opt(secs)
    }
}

#[reflect_methods]
impl FixedOffset {
    /// Returns this offset as a formatted string (e.g. `"+05:30"`).
    #[instrument(skip(self))]
    pub fn fix(&self) -> String {
        seconds_to_offset_string(self.local_minus_utc_raw())
    }

    /// Returns seconds east of UTC (positive east, negative west).
    #[instrument(skip(self))]
    pub fn local_minus_utc(&self) -> i32 {
        self.local_minus_utc_raw()
    }

    /// Returns seconds west of UTC (positive west, negative east).
    #[instrument(skip(self))]
    pub fn utc_minus_local(&self) -> i32 {
        -self.local_minus_utc_raw()
    }

    /// Returns `self` (a `FixedOffset` is its own offset).
    #[instrument(skip(self))]
    pub fn from_offset(&self) -> FixedOffset {
        *self
    }

    /// Returns the formatted offset for the given local date (always `fix()`).
    #[instrument(skip(self))]
    pub fn offset_from_local_date(&self, _date: crate::NaiveDate) -> String {
        self.fix()
    }

    /// Returns the formatted offset for the given local datetime (always `fix()`).
    #[instrument(skip(self))]
    pub fn offset_from_local_datetime(&self, _dt: crate::NaiveDateTime) -> String {
        self.fix()
    }

    /// Returns the formatted offset for the given UTC date (always `fix()`).
    #[instrument(skip(self))]
    pub fn offset_from_utc_date(&self, _date: crate::NaiveDate) -> String {
        self.fix()
    }

    /// Returns the formatted offset for the given UTC datetime (always `fix()`).
    #[instrument(skip(self))]
    pub fn offset_from_utc_datetime(&self, _dt: crate::NaiveDateTime) -> String {
        self.fix()
    }
}

impl chrono::Offset for FixedOffset {
    fn fix(&self) -> chrono::FixedOffset {
        match self.0.into_inner() {
            Some(o) => o,
            None => chrono::Utc.fix(),
        }
    }
}

impl chrono::TimeZone for FixedOffset {
    type Offset = FixedOffset;

    fn from_offset(offset: &FixedOffset) -> FixedOffset {
        *offset
    }

    fn offset_from_local_date(
        &self,
        _local: &chrono::NaiveDate,
    ) -> chrono::MappedLocalTime<FixedOffset> {
        chrono::MappedLocalTime::Single(*self)
    }

    fn offset_from_local_datetime(
        &self,
        _local: &chrono::NaiveDateTime,
    ) -> chrono::MappedLocalTime<FixedOffset> {
        chrono::MappedLocalTime::Single(*self)
    }

    fn offset_from_utc_date(&self, _utc: &chrono::NaiveDate) -> FixedOffset {
        *self
    }

    fn offset_from_utc_datetime(&self, _utc: &chrono::NaiveDateTime) -> FixedOffset {
        *self
    }
}
