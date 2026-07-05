//! `LocalResult` — shadow for `chrono::offset::LocalResult`.
//!
//! The upstream type is generic over `T`; here it is concretised to
//! `crate::DateTime` (UTC) so that it can be serialized and queried via MCP.

use elicitation_derive::reflect_methods;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// Shadow for [`chrono::offset::LocalResult<DateTime>`], concretised to UTC.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", content = "value")]
pub enum LocalResult {
    /// The local time does not exist (e.g. during a DST gap).
    None,
    /// The local time maps uniquely to the contained UTC instant.
    Single(crate::DateTime),
    /// The local time is ambiguous (e.g. during a DST fold); both instants given.
    Ambiguous(crate::DateTime, crate::DateTime),
}

impl LocalResult {
    fn from_chrono(r: chrono::offset::LocalResult<chrono::DateTime<chrono::Utc>>) -> Self {
        match r {
            chrono::offset::LocalResult::None => LocalResult::None,
            chrono::offset::LocalResult::Single(dt) => LocalResult::Single(dt.into()),
            chrono::offset::LocalResult::Ambiguous(a, b) => {
                LocalResult::Ambiguous(a.into(), b.into())
            }
        }
    }
}

#[reflect_methods]
impl LocalResult {
    /// Returns the single value if this is `Single`, otherwise `None`.
    #[instrument(skip(self))]
    pub fn single(&self) -> Option<crate::DateTime> {
        match self {
            LocalResult::Single(dt) => Some(dt.clone()),
            _ => None,
        }
    }

    /// Returns the first (earliest) matching instant, or `None`.
    #[instrument(skip(self))]
    pub fn earliest(&self) -> Option<crate::DateTime> {
        match self {
            LocalResult::None => None,
            LocalResult::Single(dt) => Some(dt.clone()),
            LocalResult::Ambiguous(a, _) => Some(a.clone()),
        }
    }

    /// Returns the last (latest) matching instant, or `None`.
    #[instrument(skip(self))]
    pub fn latest(&self) -> Option<crate::DateTime> {
        match self {
            LocalResult::None => None,
            LocalResult::Single(dt) => Some(dt.clone()),
            LocalResult::Ambiguous(_, b) => Some(b.clone()),
        }
    }

    /// Applies a function to the inner `DateTime`, returning a new `LocalResult`.
    #[instrument(skip(self))]
    pub fn map(&self, offset_secs: i32) -> LocalResult {
        let shift = match chrono::TimeDelta::try_seconds(offset_secs as i64) {
            Some(d) => d,
            None => return LocalResult::None,
        };
        match self {
            LocalResult::None => LocalResult::None,
            LocalResult::Single(dt) => {
                let inner: chrono::DateTime<chrono::Utc> = **dt;
                LocalResult::Single((inner + shift).into())
            }
            LocalResult::Ambiguous(a, b) => {
                let ia: chrono::DateTime<chrono::Utc> = **a;
                let ib: chrono::DateTime<chrono::Utc> = **b;
                LocalResult::Ambiguous((ia + shift).into(), (ib + shift).into())
            }
        }
    }

    /// Returns the single value, or returns an error string on `None` or `Ambiguous`.
    #[instrument(skip(self))]
    pub fn unwrap(&self) -> Result<crate::DateTime, String> {
        match self {
            LocalResult::Single(dt) => Ok(dt.clone()),
            LocalResult::None => Err("LocalResult is None".to_string()),
            LocalResult::Ambiguous(_, _) => Err("LocalResult is Ambiguous".to_string()),
        }
    }

    /// Constructs a `LocalResult` from a UTC `DateTime` and an hour/minute/second/microsecond.
    ///
    /// Delegates to chrono's `and_hms_micro_opt` on the underlying UTC date.
    #[instrument(skip(self))]
    pub fn and_hms_micro_opt(
        &self,
        hour: u32,
        min: u32,
        sec: u32,
        micro: u32,
    ) -> Option<crate::DateTime> {
        let base = match self {
            LocalResult::Single(dt) => dt.clone(),
            LocalResult::Ambiguous(dt, _) => dt.clone(),
            LocalResult::None => return None,
        };
        let inner: chrono::DateTime<chrono::Utc> = *base;
        inner
            .date_naive()
            .and_hms_micro_opt(hour, min, sec, micro)
            .map(|ndt| ndt.and_utc().into())
    }

    /// Like `and_hms_micro_opt` but with millisecond precision.
    #[instrument(skip(self))]
    pub fn and_hms_milli_opt(
        &self,
        hour: u32,
        min: u32,
        sec: u32,
        milli: u32,
    ) -> Option<crate::DateTime> {
        let base = match self {
            LocalResult::Single(dt) => dt.clone(),
            LocalResult::Ambiguous(dt, _) => dt.clone(),
            LocalResult::None => return None,
        };
        let inner: chrono::DateTime<chrono::Utc> = *base;
        inner
            .date_naive()
            .and_hms_milli_opt(hour, min, sec, milli)
            .map(|ndt| ndt.and_utc().into())
    }

    /// Like `and_hms_micro_opt` but with nanosecond precision.
    #[instrument(skip(self))]
    pub fn and_hms_nano_opt(
        &self,
        hour: u32,
        min: u32,
        sec: u32,
        nano: u32,
    ) -> Option<crate::DateTime> {
        let base = match self {
            LocalResult::Single(dt) => dt.clone(),
            LocalResult::Ambiguous(dt, _) => dt.clone(),
            LocalResult::None => return None,
        };
        let inner: chrono::DateTime<chrono::Utc> = *base;
        inner
            .date_naive()
            .and_hms_nano_opt(hour, min, sec, nano)
            .map(|ndt| ndt.and_utc().into())
    }

    /// Combines this result's date with the given `NaiveTime`.
    #[instrument(skip(self))]
    pub fn and_time(&self, time: crate::NaiveTime) -> Option<crate::DateTime> {
        let base = match self {
            LocalResult::Single(dt) => dt.clone(),
            LocalResult::Ambiguous(dt, _) => dt.clone(),
            LocalResult::None => return None,
        };
        let inner: chrono::DateTime<chrono::Utc> = *base;
        Some(inner.date_naive().and_time(*time).and_utc().into())
    }

    /// Like `and_hms_micro_opt` but only hour/minute/second.
    #[instrument(skip(self))]
    pub fn and_hms_opt(&self, hour: u32, min: u32, sec: u32) -> Option<crate::DateTime> {
        self.and_hms_micro_opt(hour, min, sec, 0)
    }
}

impl LocalResult {
    /// Builds a `LocalResult::Single` wrapping the given UTC `DateTime`.
    #[instrument]
    pub fn from_single(dt: crate::DateTime) -> LocalResult {
        LocalResult::Single(dt)
    }

    /// Converts a `LocalResultWrap<DateTime>` to `LocalResult`.
    #[instrument]
    pub fn from_wrap(wrap: elicitation::LocalResultWrap<crate::DateTime>) -> LocalResult {
        match wrap {
            elicitation::LocalResultWrap::None => LocalResult::None,
            elicitation::LocalResultWrap::Single(dt) => LocalResult::Single(dt),
            elicitation::LocalResultWrap::Ambiguous(a, b) => LocalResult::Ambiguous(a, b),
        }
    }
}

impl From<chrono::offset::LocalResult<chrono::DateTime<chrono::Utc>>> for LocalResult {
    fn from(r: chrono::offset::LocalResult<chrono::DateTime<chrono::Utc>>) -> Self {
        LocalResult::from_chrono(r)
    }
}
