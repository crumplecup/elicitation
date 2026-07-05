//! Free functions mirroring `chrono::format::parse` and `chrono::format::parse_and_remainder`.

use tracing::instrument;

/// Parses `s` into `parsed` using the provided format items.
///
/// Returns `Ok(())` when the entire string is consumed.  On failure the
/// `parsed` state is indeterminate and must not be used.
///
/// Mirrors [`chrono::format::parse`].
#[instrument(skip(parsed, items))]
pub fn parse(
    parsed: &mut crate::Parsed,
    s: &str,
    items: impl Iterator<Item = crate::Item>,
) -> Result<(), crate::ParseError> {
    let chrono_items = items.map(|it| it.0.into_item());
    chrono::format::parse(&mut parsed.0, s, chrono_items)
        .map_err(|e| crate::ParseError(elicitation::ParseErrorWrap::from(e)))
}

/// Parses `s` into `parsed` using the provided format items, returning the
/// unconsumed remainder of the string on success.
///
/// Mirrors [`chrono::format::parse_and_remainder`].
#[instrument(skip(parsed, items))]
pub fn parse_and_remainder<'b>(
    parsed: &mut crate::Parsed,
    s: &'b str,
    items: impl Iterator<Item = crate::Item>,
) -> Result<&'b str, crate::ParseError> {
    let chrono_items = items.map(|it| it.0.into_item());
    chrono::format::parse_and_remainder(&mut parsed.0, s, chrono_items)
        .map_err(|e| crate::ParseError(elicitation::ParseErrorWrap::from(e)))
}
