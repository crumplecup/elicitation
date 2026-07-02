//! Interval and duration parsing traits.

use elicitation::Established;

use crate::{
    IntervalEndpointsOrdered, OffsetDateTimeDescriptor, ParsedDurationResult,
    ParsedRecurringIntervalResult, ParsedTimeIntervalResult, TemporalResult,
    TimestampRepresentsFixedInstant,
};

/// Parse and compare ISO 8601 duration and interval forms.
///
/// Normative sources: ISO 8601-1:2019, 4.4; 4.5; ISO 8601-2:2019, 10.2.
/// Informative cross-check: CalConnect CC 18011:2018 §6.14, §7.3;
/// CalConnect CC 18012:2018 §6.3, §6.4.
pub trait TemporalIntervalFactory: Send + Sync {
    /// Parse an ISO 8601 duration representation.
    ///
    /// Normative sources: ISO 8601-1:2019, 4.4.2 b); 4.4.3.
    /// Informative cross-check: CalConnect CC 18011:2018 §7.3 - Representations.
    fn parse_duration(&self, input: &str) -> ParsedDurationResult;

    /// Parse an ISO 8601 recurring interval representation.
    ///
    /// Normative sources: ISO 8601-1:2019, 4.5.1; 4.5.2; 4.5.3; 4.5.4.
    /// Informative cross-check: CalConnect CC 18012:2018 §6.3 - Repeat rule;
    /// §6.4 - Complete representation.
    fn parse_recurring_interval(&self, input: &str) -> ParsedRecurringIntervalResult;

    /// Confirm chronological ordering between two fixed-instant endpoints.
    ///
    /// Normative sources: ISO 8601-1:2019, 3.1.1.6; 3.1.1.8.
    fn order_offset_endpoints(
        &self,
        start: &OffsetDateTimeDescriptor,
        end: &OffsetDateTimeDescriptor,
        start_proof: Established<TimestampRepresentsFixedInstant>,
        end_proof: Established<TimestampRepresentsFixedInstant>,
    ) -> TemporalResult<Established<IntervalEndpointsOrdered>>;

    /// Parse an ISO 8601 interval representation.
    ///
    /// The returned proof sidecars keep extended open or unknown boundary
    /// semantics distinct from the base interval-form validity token.
    ///
    /// Normative sources: ISO 8601-1:2019, 4.4.1; 4.4.2; 4.4.4; 4.4.5;
    /// ISO 8601-2:2019, 10.2.
    /// Informative cross-check: CalConnect CC 18011:2018 §6.14 - Time interval;
    /// public LOC EDTF Level 1 - Extended Interval.
    fn parse_interval(&self, input: &str) -> ParsedTimeIntervalResult;
}
