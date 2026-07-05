//! `Month` — shadow for `chrono::Month`.

use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(elicitation::MonthSelect, as Month, serde);
elicitation::elicit_newtype_traits!(Month, elicitation::MonthSelect, [cmp, from_str]);

impl TryFrom<u8> for Month {
    type Error = String;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        chrono::Month::try_from(n)
            .map(|m| elicitation::MonthSelect::from(m).into())
            .map_err(|e| e.to_string())
    }
}

impl Month {
    /// Converts a 1-indexed integer (1–12) to a `Month`, or `None` if out of range.
    #[instrument]
    pub fn from_u32(n: u32) -> Option<Month> {
        u8::try_from(n)
            .ok()
            .and_then(|b| chrono::Month::try_from(b).ok())
            .map(|m| elicitation::MonthSelect::from(m).into())
    }

    /// Converts an `i64` (1–12) to a `Month`, or `None` if out of range.
    #[instrument]
    pub fn from_i64(n: i64) -> Option<Month> {
        u8::try_from(n)
            .ok()
            .and_then(|b| chrono::Month::try_from(b).ok())
            .map(|m| elicitation::MonthSelect::from(m).into())
    }

    /// Converts a `u64` (1–12) to a `Month`, or `None` if out of range.
    #[instrument]
    pub fn from_u64(n: u64) -> Option<Month> {
        u8::try_from(n)
            .ok()
            .and_then(|b| chrono::Month::try_from(b).ok())
            .map(|m| elicitation::MonthSelect::from(m).into())
    }
}

#[reflect_methods]
impl Month {
    /// Returns the English name of this month.
    #[instrument(skip(self))]
    pub fn name(&self) -> &'static str {
        (*self.0).into_inner().name()
    }

    /// Returns the 1-indexed number of this month (January = 1).
    #[instrument(skip(self))]
    pub fn number_from_month(&self) -> u32 {
        (*self.0).into_inner().number_from_month()
    }

    /// Returns the next month (December wraps to January).
    #[instrument(skip(self))]
    pub fn succ(&self) -> Month {
        elicitation::MonthSelect::from((*self.0).into_inner().succ()).into()
    }

    /// Returns the previous month (January wraps to December).
    #[instrument(skip(self))]
    pub fn pred(&self) -> Month {
        elicitation::MonthSelect::from((*self.0).into_inner().pred()).into()
    }

    /// Returns the number of days in this month for the given year.
    #[instrument(skip(self))]
    pub fn num_days(&self, year: i32) -> Option<u8> {
        (*self.0).into_inner().num_days(year)
    }
}
