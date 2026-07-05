//! Parsing traits for core temporal interchange forms.
//!
//! These methods stand at the leaf edge of the temporal accord: raw strings
//! come in, neutral descriptors plus proof sidecars come out.

use elicitation::Established;

use crate::{
    CalendarDateDescriptor, CalendarDateValid, LocalDateTimeDescriptor,
    LocalDateTimeDoesNotIdentifyFixedInstant, LocalDateTimeValid, LocalTimeDescriptor,
    LocalTimeValid, OrdinalDateDescriptor, OrdinalDateValid, ParsedCenturyResult,
    ParsedDateTimeFormulaResult, ParsedDateWithShiftResult, ParsedDecadeResult,
    ParsedExtendedYearResult, ParsedGroupedTimeScaleUnitResult, ParsedIxdtfTimestampResult,
    ParsedOffsetDateTimeResult, ParsedQualifiedTemporalValueResult,
    ParsedReducedCalendarDateResult, ParsedReducedLocalTimeResult, ParsedRfc3339TimestampResult,
    ParsedSeasonalTemporalExpressionResult, ParsedSubYearGroupingExpressionResult,
    ParsedTemporalSetResult, ParsedTimeIntervalResult, ParsedTimeOfDayWithShiftResult,
    ParsedUnspecifiedComponentExpressionResult, TemporalResult, UtcOffsetDescriptor,
    UtcOffsetValid, WeekDateDescriptor, WeekDateValid,
};

/// Parse standards-governed temporal forms into neutral descriptors.
///
/// Normative sources: ISO 8601-1:2019, 2.3.3, 2.3.4, 5.2, 5.3, 5.4, 5.5, and
/// 5.6; ISO 8601-2:2019, 4.3.5, 4.3.6, 4.7.2, 4.7.3, 4.7.4, 4.8.1, 4.8.2,
/// 4.8.3, 5.1, 5.2, 5.3, 5.4, 6.1, 6.2, 6.3, 6.4, 6.5, 6.6, 8.2.1, 8.2.2,
/// 8.2.3, 8.4.4, 8.4.5, 8.4.6, 8.5, 9.2.1, 9.2.2, 9.3, 10.2, 14.1, 14.2,
/// 14.3, and 14.4; CalConnect CC 18011:2018 §4.3, §5, and §8.
/// Informative cross-check: RFC 3339 §5.6.
pub trait TemporalParser: Send + Sync {
    /// Parse a complete ISO 8601 calendar date (`YYYY-MM-DD` or equivalent).
    ///
    /// Normative source: ISO 8601-1:2019, 5.2.2.
    fn parse_calendar_date(
        &self,
        input: &str,
    ) -> TemporalResult<(CalendarDateDescriptor, Established<CalendarDateValid>)>;

    /// Parse a reduced-precision ISO 8601 calendar date (`YYYY` or `YYYY-MM`).
    ///
    /// Normative source: ISO 8601-1:2019, 5.2.2.
    fn parse_reduced_calendar_date(&self, input: &str) -> ParsedReducedCalendarDateResult;

    /// Parse an ISO 8601-2 extended year form.
    ///
    /// Normative source: ISO 8601-2:2019, 4.7.2, 4.7.3, and 4.7.4.
    /// Informative cross-check: public LOC EDTF Level 1 - Letter-prefixed calendar year; Negative calendar year. Level 2 - Exponential year; Significant digits.
    fn parse_extended_year(&self, input: &str) -> ParsedExtendedYearResult;

    /// Parse an ISO 8601 decade representation into its neutral ordinal descriptor.
    ///
    /// Normative sources: ISO 8601-1:2019/Amd 1:2022, 4.3.11; ISO 8601-2:2019,
    /// 4.3.5.
    fn parse_decade(&self, input: &str) -> ParsedDecadeResult;

    /// Parse an ISO 8601 century representation into its neutral ordinal descriptor.
    ///
    /// Normative sources: ISO 8601-1:2019/Amd 1:2022, 4.3.12; ISO 8601-2:2019,
    /// 4.3.6.
    fn parse_century(&self, input: &str) -> ParsedCenturyResult;

    /// Parse a qualified temporal value carrying explicit uncertainty and/or approximation semantics.
    ///
    /// Normative source: ISO 8601-2:2019, 8.2.1, 8.2.2, 8.2.3, 8.4.4, 8.4.5,
    /// 8.4.6, and 8.5.
    fn parse_qualified_temporal_value(&self, input: &str) -> ParsedQualifiedTemporalValueResult;

    /// Parse a complete ISO 8601 ordinal date.
    ///
    /// Normative source: ISO 8601-1:2019, 5.2.3.
    fn parse_ordinal_date(
        &self,
        input: &str,
    ) -> TemporalResult<(OrdinalDateDescriptor, Established<OrdinalDateValid>)>;

    /// Parse a complete ISO 8601 week date.
    ///
    /// Normative source: ISO 8601-1:2019, 5.2.4.
    fn parse_week_date(
        &self,
        input: &str,
    ) -> TemporalResult<(WeekDateDescriptor, Established<WeekDateValid>)>;

    /// Parse a local time-of-day representation.
    ///
    /// Normative sources: ISO 8601-1:2019, 5.3.1; ISO 8601-1:2019/Amd 1:2022,
    /// 5.3.1.4 and 5.3.2.
    fn parse_local_time(
        &self,
        input: &str,
    ) -> TemporalResult<(LocalTimeDescriptor, Established<LocalTimeValid>)>;

    /// Parse a reduced-accuracy local time-of-day representation.
    ///
    /// Normative sources: ISO 8601-1:2019, 5.3.1; ISO 8601-1:2019/Amd 1:2022,
    /// 5.3.1.4 and 5.3.2.
    fn parse_reduced_local_time(&self, input: &str) -> ParsedReducedLocalTimeResult;

    /// Parse a numeric UTC offset representation.
    ///
    /// Normative source: ISO 8601-1:2019, 5.3.4.
    /// Open-text cross-check: ISO/WD 8601-1:2016(E), 4.2.5.1 and 4.2.5.2.
    /// Informative cross-check: RFC 3339 §4.2-§4.4.
    fn parse_utc_offset(
        &self,
        input: &str,
    ) -> TemporalResult<(UtcOffsetDescriptor, Established<UtcOffsetValid>)>;

    /// Parse a combined local date-time representation.
    ///
    /// The returned proof sidecars make the local semantics explicit: the
    /// representation is structurally valid, but it does not identify a fixed
    /// instant without additional zone or offset law.
    ///
    /// Normative source: ISO 8601-1:2019, 5.4.2 and 5.4.3.
    fn parse_local_date_time(
        &self,
        input: &str,
    ) -> TemporalResult<(
        LocalDateTimeDescriptor,
        Established<LocalDateTimeValid>,
        Established<LocalDateTimeDoesNotIdentifyFixedInstant>,
    )>;

    /// Parse a combined date-time representation carrying an explicit UTC relationship.
    ///
    /// The returned proof sidecar preserves the fixed-instant semantics
    /// required by the proof sidecar pattern: an offset-bearing timestamp is
    /// not merely lexically well formed, it also identifies a single instant on
    /// the UTC timeline.
    ///
    /// Normative sources: ISO 8601-1:2019, 5.3.4, 5.4.2, and 5.4.3.
    fn parse_offset_date_time(&self, input: &str) -> ParsedOffsetDateTimeResult;

    /// Parse a complete explicit-form date carrying a time shift.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3 - Date with shift.
    fn parse_date_with_shift(&self, input: &str) -> ParsedDateWithShiftResult;

    /// Parse a complete explicit-form time of day carrying a time shift.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3 - Time of day with time shift.
    fn parse_time_of_day_with_shift(&self, input: &str) -> ParsedTimeOfDayWithShiftResult;

    /// Parse an RFC 3339 timestamp into an offset date-time descriptor.
    ///
    /// This seam remains strict to the `date-time = full-date "T" full-time`
    /// profile. The Section `5.6` readability note about using a space
    /// separator is treated as an application-level policy layer, not as part
    /// of the core parser contract.
    ///
    /// Normative source: RFC 3339 §5.6 - Internet timestamp profile.
    fn parse_rfc3339_timestamp(&self, input: &str) -> ParsedRfc3339TimestampResult;

    /// Parse an RFC 9557 IXDTF timestamp with explicit suffix-proof branches.
    ///
    /// The returned exchange shape keeps named-zone identity, offset-zone
    /// compatibility semantics, preferred-calendar annotations, and
    /// additional-information semantics as separate proof branches rather than
    /// collapsing them into one aggregate suffix token.
    ///
    /// Normative sources: RFC 9557 §1.2, §3.3, §3.4, and §4.1.
    fn parse_ixdtf_timestamp(&self, input: &str) -> ParsedIxdtfTimestampResult;

    /// Parse an ISO 8601-2 seasonal temporal expression.
    ///
    /// Normative source: ISO 8601-2:2019, 4.8.1, 4.8.2, and 4.8.3.
    /// Informative cross-check: public LOC EDTF Level 1 - Seasons.
    fn parse_seasonal_temporal_expression(
        &self,
        input: &str,
    ) -> ParsedSeasonalTemporalExpressionResult;

    /// Parse an ISO 8601-2 Level 2 sub-year grouping expression.
    ///
    /// Normative source: ISO 8601-2:2019, 4.8.1, 4.8.2, and 4.8.3.
    /// Informative cross-check: public LOC EDTF Level 2 - Sub-year groupings.
    fn parse_sub_year_grouping_expression(
        &self,
        input: &str,
    ) -> ParsedSubYearGroupingExpressionResult;

    /// Parse an ISO 8601-2 unspecified-component temporal expression.
    ///
    /// Normative source: ISO 8601-2:2019, 9.2.1, 9.2.2, and 9.3.
    /// Informative cross-check: public LOC EDTF Level 1 - Unspecified digit(s) from the right; Level 2 - Unspecified Digit.
    fn parse_unspecified_component_expression(
        &self,
        input: &str,
    ) -> ParsedUnspecifiedComponentExpressionResult;

    /// Parse an ISO 8601-2 temporal set expression, including range refinements.
    ///
    /// Normative source: ISO 8601-2:2019, 6.1, 6.2, 6.3, 6.4, 6.5, and 6.6.
    /// Informative cross-check: public LOC EDTF Level 2 - Set representation.
    fn parse_temporal_set(&self, input: &str) -> ParsedTemporalSetResult;

    /// Parse a grouped time scale unit expression into its neutral grouped-unit descriptor.
    ///
    /// Normative source: ISO 8601-2:2019, 5.1, 5.2, 5.3, 5.4, and 5.4.2.
    /// Informative cross-check: CalConnect CC 18011:2018 §5 - Grouped time scale units.
    fn parse_grouped_time_scale_unit(&self, input: &str) -> ParsedGroupedTimeScaleUnitResult;

    /// Parse a date-time formula into its neutral formula descriptor and semantics sidecar.
    ///
    /// Normative source: ISO 8601-2:2019, 14.1, 14.2, 14.3, and 14.4.
    /// Informative cross-check: CalConnect CC 18011:2018 §8 - Evaluation of date and time with duration.
    fn parse_date_time_formula(&self, input: &str) -> ParsedDateTimeFormulaResult;

    /// Parse an ISO 8601 time interval representation.
    ///
    /// The returned descriptor preserves explicit open or unknown boundaries
    /// without collapsing them into ordinary concrete endpoints.
    ///
    /// Normative sources: ISO 8601-1:2019, 4.4.1; 4.4.2; 4.4.4; 4.4.5;
    /// ISO 8601-2:2019, 10.2.
    /// Informative cross-check: CalConnect CC 18011:2018 §6.14 - Time interval.
    fn parse_time_interval(&self, input: &str) -> ParsedTimeIntervalResult;
}
