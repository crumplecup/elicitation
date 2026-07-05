//! `NaiveDateDaysIterator` and `NaiveDateWeeksIterator` — shadows for the corresponding
//! `chrono::naive` iterator types.
//!
//! These types cannot carry real lazy iterators across the MCP boundary.
//! They wrap a cursor (current position) and implement `Iterator` /
//! `DoubleEndedIterator` so that the required method names exist in this crate's
//! public API surface.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// Shadow for [`chrono::naive::NaiveDateDaysIterator`].
///
/// Holds a current-date cursor; `next()` advances by one day, `next_back()` retreats.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
pub struct NaiveDateDaysIterator {
    /// Current position of the iterator.
    pub current: crate::NaiveDate,
}

impl NaiveDateDaysIterator {
    /// Creates a new days iterator starting at `start`.
    #[instrument]
    pub fn new(start: crate::NaiveDate) -> Self {
        Self { current: start }
    }
}

impl Iterator for NaiveDateDaysIterator {
    type Item = crate::NaiveDate;

    fn next(&mut self) -> Option<crate::NaiveDate> {
        let cur = self.current.clone();
        self.current = self.current.succ_opt()?;
        Some(cur)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

impl DoubleEndedIterator for NaiveDateDaysIterator {
    fn next_back(&mut self) -> Option<crate::NaiveDate> {
        let cur = self.current.clone();
        self.current = self.current.pred_opt()?;
        Some(cur)
    }
}

/// Shadow for [`chrono::naive::NaiveDateWeeksIterator`].
///
/// Holds a current-date cursor; `next()` advances by seven days, `next_back()` retreats.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
pub struct NaiveDateWeeksIterator {
    /// Current position of the iterator.
    pub current: crate::NaiveDate,
}

impl NaiveDateWeeksIterator {
    /// Creates a new weeks iterator starting at `start`.
    #[instrument]
    pub fn new(start: crate::NaiveDate) -> Self {
        Self { current: start }
    }
}

impl Iterator for NaiveDateWeeksIterator {
    type Item = crate::NaiveDate;

    fn next(&mut self) -> Option<crate::NaiveDate> {
        let cur = self.current.clone();
        self.current = self.current.checked_add_days(7u64)?;
        Some(cur)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

impl DoubleEndedIterator for NaiveDateWeeksIterator {
    fn next_back(&mut self) -> Option<crate::NaiveDate> {
        let cur = self.current.clone();
        self.current = self.current.checked_sub_days(7u64)?;
        Some(cur)
    }
}
