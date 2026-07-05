//! `IsoWeek` and `NaiveWeek` — shadows for `chrono::IsoWeek` and `chrono::NaiveWeek`.

use elicitation_derive::reflect_methods;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── IsoWeek ───────────���────────────────────────────────��──────────────────────

/// Shadow for [`chrono::IsoWeek`].
///
/// Stores the ISO year and week number for serialization.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
pub struct IsoWeek {
    /// ISO year.
    pub year: i32,
    /// ISO week number (1–53).
    pub week: u32,
}

impl From<chrono::IsoWeek> for IsoWeek {
    fn from(w: chrono::IsoWeek) -> Self {
        Self {
            year: w.year(),
            week: w.week(),
        }
    }
}

#[reflect_methods]
impl IsoWeek {
    /// Returns the ISO year.
    #[instrument(skip(self))]
    pub fn year(&self) -> i32 {
        self.year
    }

    /// Returns the ISO week number (1–53).
    #[instrument(skip(self))]
    pub fn week(&self) -> u32 {
        self.week
    }

    /// Returns the ISO week number (0-indexed; 0 = week 1).
    #[instrument(skip(self))]
    pub fn week0(&self) -> u32 {
        self.week.saturating_sub(1)
    }
}

// ── NaiveWeek ─────────────────────────────────────────────────────────────────

/// Shadow for [`chrono::NaiveWeek`].
///
/// Stores the first day of the week as a [`crate::NaiveDate`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct NaiveWeek {
    /// First day of the week.
    pub first_day: crate::NaiveDate,
}

impl From<chrono::NaiveWeek> for NaiveWeek {
    fn from(w: chrono::NaiveWeek) -> Self {
        Self {
            first_day: w.first_day().into(),
        }
    }
}

#[reflect_methods]
impl NaiveWeek {
    /// Returns the first day of this week.
    #[instrument(skip(self))]
    pub fn first_day(&self) -> crate::NaiveDate {
        self.first_day.clone()
    }

    /// Returns the last day of this week.
    #[instrument(skip(self))]
    pub fn last_day(&self) -> crate::NaiveDate {
        let first: chrono::NaiveDate = *self.first_day;
        match first.checked_add_days(chrono::Days::new(6)) {
            Some(d) => d.into(),
            None => self.first_day.clone(),
        }
    }

    /// Returns the first day of this week, or `None` if it would overflow.
    #[instrument(skip(self))]
    pub fn checked_first_day(&self) -> Option<crate::NaiveDate> {
        Some(self.first_day.clone())
    }

    /// Returns the last day of this week, or `None` if it would overflow.
    #[instrument(skip(self))]
    pub fn checked_last_day(&self) -> Option<crate::NaiveDate> {
        let first: chrono::NaiveDate = *self.first_day;
        first.checked_add_days(chrono::Days::new(6)).map(Into::into)
    }

    /// Returns all days in this week as a list of dates.
    #[instrument(skip(self))]
    pub fn days(&self) -> Vec<crate::NaiveDate> {
        let first: chrono::NaiveDate = *self.first_day;
        (0u64..7)
            .filter_map(|i| {
                first
                    .checked_add_days(chrono::Days::new(i))
                    .map(crate::NaiveDate::from)
            })
            .collect()
    }

    /// Returns all days in this week, or `None` if any date would overflow.
    #[instrument(skip(self))]
    pub fn checked_days(&self) -> Option<Vec<crate::NaiveDate>> {
        let first: chrono::NaiveDate = *self.first_day;
        (0u64..7)
            .map(|i| {
                first
                    .checked_add_days(chrono::Days::new(i))
                    .map(crate::NaiveDate::from)
            })
            .collect()
    }
}
