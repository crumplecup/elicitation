//! `Local` — shadow for `chrono::Local` timezone marker.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// Shadow for [`chrono::Local`].
///
/// Provides local-time construction methods that return [`crate::DateTime`] instead
/// of the raw `chrono::DateTime<chrono::Local>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Local;

impl Local {
    /// Returns the current local time.
    #[instrument]
    pub fn now() -> crate::DateTime {
        chrono::Local::now().with_timezone(&chrono::Utc).into()
    }

    /// Returns the current local date (deprecated; use `now().date_naive()`).
    #[instrument]
    pub fn today() -> String {
        chrono::Local::now().format("%Y-%m-%d").to_string()
    }

    /// Returns a `Local` instance (the only valid Local offset is itself).
    #[instrument]
    pub fn from_offset() -> Self {
        Local
    }

    /// Returns the local offset that applies to the given naive date, as a formatted string.
    ///
    /// Uses noon on that date to avoid DST-gap ambiguity.
    #[instrument]
    pub fn offset_from_local_date(date: crate::NaiveDate) -> String {
        use chrono::TimeZone;
        let nd: chrono::NaiveDate = *date.0;
        let ndt = nd.and_hms_opt(12, 0, 0).unwrap_or_default();
        chrono::Local
            .offset_from_local_datetime(&ndt)
            .earliest()
            .map(|o| o.to_string())
            .unwrap_or_else(|| "invalid".to_string())
    }

    /// Returns the local offset for the given naive datetime, as a formatted string.
    #[instrument]
    pub fn offset_from_local_datetime(dt: crate::NaiveDateTime) -> String {
        use chrono::TimeZone;
        let ndt: chrono::NaiveDateTime = *dt.0;
        chrono::Local
            .offset_from_local_datetime(&ndt)
            .earliest()
            .map(|o| o.to_string())
            .unwrap_or_else(|| "invalid".to_string())
    }

    /// Returns the local offset applied to the given UTC date, as a formatted string.
    #[instrument]
    pub fn offset_from_utc_date(date: crate::NaiveDate) -> String {
        use chrono::TimeZone;
        let nd: chrono::NaiveDate = *date.0;
        let ndt = nd.and_hms_opt(0, 0, 0).unwrap_or_default();
        chrono::Local.offset_from_utc_datetime(&ndt).to_string()
    }

    /// Returns the local offset applied to the given UTC datetime, as a formatted string.
    #[instrument]
    pub fn offset_from_utc_datetime(dt: crate::NaiveDateTime) -> String {
        use chrono::TimeZone;
        let ndt: chrono::NaiveDateTime = *dt.0;
        chrono::Local.offset_from_utc_datetime(&ndt).to_string()
    }
}

// ── chrono timezone trait ──────────────────────────────────────────────────────

impl chrono::TimeZone for Local {
    type Offset = crate::FixedOffset;

    fn from_offset(_offset: &crate::FixedOffset) -> Local {
        Local
    }

    fn offset_from_local_date(
        &self,
        local: &chrono::NaiveDate,
    ) -> chrono::MappedLocalTime<crate::FixedOffset> {
        chrono::Local
            .offset_from_local_datetime(&local.and_time(chrono::NaiveTime::default()))
            .map(crate::FixedOffset::from)
    }

    fn offset_from_local_datetime(
        &self,
        local: &chrono::NaiveDateTime,
    ) -> chrono::MappedLocalTime<crate::FixedOffset> {
        chrono::Local
            .offset_from_local_datetime(local)
            .map(crate::FixedOffset::from)
    }

    fn offset_from_utc_date(&self, utc: &chrono::NaiveDate) -> crate::FixedOffset {
        chrono::Local
            .offset_from_utc_datetime(&utc.and_time(chrono::NaiveTime::default()))
            .into()
    }

    fn offset_from_utc_datetime(&self, utc: &chrono::NaiveDateTime) -> crate::FixedOffset {
        chrono::Local.offset_from_utc_datetime(utc).into()
    }
}
