//! Shadow types for `chrono::format` and simple error types.
//!
//! These types have only derived trait methods (clone, eq, fmt, hash) plus a few
//! explicit methods for error description and kind access.

use tracing::instrument;

// ── Format enum/struct types ──────────────────────────────────────────────────

/// Shadow for [`chrono::format::Colons`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Colons(pub elicitation::ColonsSelect);

/// Shadow for [`chrono::format::Pad`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pad(pub elicitation::PadSelect);

/// Shadow for [`chrono::format::Fixed`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Fixed(pub elicitation::FixedSelect);

/// Shadow for [`chrono::format::Numeric`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Numeric(pub elicitation::NumericSelect);

/// Shadow for [`chrono::format::OffsetPrecision`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OffsetPrecision(pub elicitation::OffsetPrecisionSelect);

/// Shadow for [`chrono::format::ParseErrorKind`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParseErrorKind(pub elicitation::ParseErrorKindSelect);

/// Shadow for [`chrono::SecondsFormat`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SecondsFormat(pub elicitation::SecondsFormatSelect);

/// Shadow for [`chrono::format::InternalFixed`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InternalFixed(pub elicitation::InternalFixedWrap);

/// Shadow for [`chrono::format::InternalNumeric`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InternalNumeric(pub elicitation::InternalNumericWrap);

/// Shadow for [`chrono::format::OffsetFormat`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OffsetFormat(pub elicitation::OffsetFormatWrap);

// ── Simple error types ────────────────────────────────────────────────────────

/// Shadow for [`chrono::OutOfRange`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OutOfRange(pub elicitation::OutOfRangeWrap);

/// Shadow for [`chrono::ParseMonthError`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseMonthError(pub elicitation::ParseMonthErrorWrap);

/// Shadow for [`chrono::ParseWeekdayError`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseWeekdayError(pub elicitation::ParseWeekdayErrorWrap);

/// Shadow for [`chrono::RoundingError`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoundingError(pub elicitation::RoundingErrorSelect);

impl RoundingError {
    /// Returns a description of the error (deprecated; use Display instead).
    #[instrument(skip(self))]
    pub fn description(&self) -> &'static str {
        "rounding error"
    }
}

/// Shadow for [`chrono::OutOfRangeError`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutOfRangeError(pub elicitation::OutOfRangeErrorWrap);

impl OutOfRangeError {
    /// Returns a description of the error (deprecated; use Display instead).
    #[instrument(skip(self))]
    pub fn description(&self) -> &'static str {
        "out of range"
    }
}

/// Shadow for [`chrono::ParseError`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, derive_more::Display, derive_more::Error)]
#[display("{}", _0)]
pub struct ParseError(pub elicitation::ParseErrorWrap);

impl ParseError {
    /// Returns a description of the error (deprecated; use Display instead).
    #[instrument(skip(self))]
    pub fn description(&self) -> &'static str {
        "input contains invalid characters"
    }

    /// Returns the kind of parse error.
    #[instrument(skip(self))]
    pub fn kind(&self) -> ParseErrorKind {
        ParseErrorKind(elicitation::ParseErrorKindSelect::from(
            self.0.into_inner().kind(),
        ))
    }
}
