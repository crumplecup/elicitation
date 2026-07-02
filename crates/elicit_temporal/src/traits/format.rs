//! Formatting traits for standards-conformant temporal emission.

use elicitation::Established;

use crate::{
    CalendarDateDescriptor, CalendarDateValid, CenturyDescriptor, CenturyValid,
    DateTimeFormulaDescriptor, DateTimeFormulaEvaluationSemanticsValid, DateTimeFormulaValid,
    DateWithShiftDescriptor, DateWithShiftValid, DecadeDescriptor, DecadeValid, DurationDescriptor,
    DurationFormValid, ExtendedYearDescriptor, ExtendedYearValid, FormattedCenturyResult,
    FormattedDateTimeFormulaResult, FormattedDateWithShiftResult, FormattedDecadeResult,
    FormattedDurationResult, FormattedExtendedYearResult, FormattedGroupedTimeScaleUnitResult,
    FormattedIxdtfTimestampResult, FormattedQualifiedTemporalValueResult,
    FormattedRecurringIntervalResult, FormattedSeasonalTemporalExpressionResult,
    FormattedSubYearGroupingExpressionResult, FormattedTemporalSetResult,
    FormattedTimeIntervalResult, FormattedTimeOfDayWithShiftResult,
    FormattedUnspecifiedComponentExpressionResult, GroupedTimeScaleUnitDescriptor,
    GroupedTimeScaleUnitValid, Iso8601BasicFormUsesCompactRepresentation,
    Iso8601ExtendedFormUsesSeparators, IxdtfSerializationCarriesNamedZoneAnnotation,
    IxdtfTimestampDescriptor, IxdtfTimestampValid, LocalDateTimeDescriptor,
    LocalDateTimeDoesNotIdentifyFixedInstant, LocalDateTimeValid, LocalTimeDescriptor,
    LocalTimeValid, OffsetConsistentWithNamedZone, OffsetDateTimeDescriptor, OffsetDateTimeValid,
    OrdinalDateDescriptor, OrdinalDateValid, QualifiedTemporalValueDescriptor,
    QualifiedTemporalValueValid, RecurringIntervalDescriptor, RecurringIntervalFormValid,
    ReducedCalendarDateDescriptor, ReducedCalendarDateValid, ReducedLocalTimeDescriptor,
    ReducedLocalTimeValid, Rfc3339TimestampValid, SeasonalTemporalExpressionDescriptor,
    SeasonalTemporalExpressionValid, SerializationCarriesExplicitUtcRelationship,
    SubYearGroupingExpressionDescriptor, SubYearGroupingExpressionValid, TemporalResult,
    TemporalSetDescriptor, TemporalSetExpressionValid, TemporalSetRangeSemanticsValid,
    TimeIntervalDescriptor, TimeIntervalValid, TimeOfDayWithShiftDescriptor,
    TimeOfDayWithShiftValid, TimestampRepresentsFixedInstant,
    UnspecifiedComponentExpressionDescriptor, UnspecifiedComponentExpressionValid,
    UtcOffsetDescriptor, UtcOffsetValid, WeekDateDescriptor, WeekDateValid,
    ZonedDateTimeDescriptor, ZonedDateTimeHasNamedZone,
};

/// Emit validated ISO 8601, ISO 8601-2, and RFC-profiled descriptors into wire forms.
///
/// Normative sources: ISO 8601-1:2019 - basic and extended forms;
/// ISO 8601-2:2019 - qualified and extended temporal forms;
/// CalConnect CC 18011:2018 - explicit forms and formula-oriented extensions.
/// Informative cross-checks: RFC 3339 §5.6; RFC 9557 §3.1.
pub trait TemporalFormatter: Send + Sync {
    /// Emit a calendar date using ISO 8601 extended separators.
    ///
    /// Normative source: ISO 8601-1:2019 - extended representation.
    fn format_calendar_date_extended(
        &self,
        date: &CalendarDateDescriptor,
        date_proof: Established<CalendarDateValid>,
    ) -> TemporalResult<(String, Established<Iso8601ExtendedFormUsesSeparators>)>;

    /// Emit a calendar date using ISO 8601 basic compact form.
    ///
    /// Normative source: ISO 8601-1:2019 - basic representation.
    fn format_calendar_date_basic(
        &self,
        date: &CalendarDateDescriptor,
        date_proof: Established<CalendarDateValid>,
    ) -> TemporalResult<(
        String,
        Established<Iso8601BasicFormUsesCompactRepresentation>,
    )>;

    /// Emit a reduced-precision calendar date using ISO 8601 extended separators.
    ///
    /// Normative source: ISO 8601-1:2019 - reduced precision calendar date representation.
    fn format_reduced_calendar_date_extended(
        &self,
        date: &ReducedCalendarDateDescriptor,
        date_proof: Established<ReducedCalendarDateValid>,
    ) -> TemporalResult<(String, Established<Iso8601ExtendedFormUsesSeparators>)>;

    /// Emit a reduced-precision calendar date using ISO 8601 basic compact form.
    ///
    /// Normative source: ISO 8601-1:2019 - reduced precision calendar date representation.
    fn format_reduced_calendar_date_basic(
        &self,
        date: &ReducedCalendarDateDescriptor,
        date_proof: Established<ReducedCalendarDateValid>,
    ) -> TemporalResult<(
        String,
        Established<Iso8601BasicFormUsesCompactRepresentation>,
    )>;

    /// Emit an ordinal date using ISO 8601 extended separators.
    ///
    /// Normative source: ISO 8601-1:2019 - ordinal date representation.
    fn format_ordinal_date_extended(
        &self,
        date: &OrdinalDateDescriptor,
        date_proof: Established<OrdinalDateValid>,
    ) -> TemporalResult<(String, Established<Iso8601ExtendedFormUsesSeparators>)>;

    /// Emit an ordinal date using ISO 8601 basic compact form.
    ///
    /// Normative source: ISO 8601-1:2019 - ordinal date representation.
    fn format_ordinal_date_basic(
        &self,
        date: &OrdinalDateDescriptor,
        date_proof: Established<OrdinalDateValid>,
    ) -> TemporalResult<(
        String,
        Established<Iso8601BasicFormUsesCompactRepresentation>,
    )>;

    /// Emit a week date using ISO 8601 extended separators.
    ///
    /// Normative source: ISO 8601-1:2019 - week date representation.
    fn format_week_date_extended(
        &self,
        date: &WeekDateDescriptor,
        date_proof: Established<WeekDateValid>,
    ) -> TemporalResult<(String, Established<Iso8601ExtendedFormUsesSeparators>)>;

    /// Emit a week date using ISO 8601 basic compact form.
    ///
    /// Normative source: ISO 8601-1:2019 - week date representation.
    fn format_week_date_basic(
        &self,
        date: &WeekDateDescriptor,
        date_proof: Established<WeekDateValid>,
    ) -> TemporalResult<(
        String,
        Established<Iso8601BasicFormUsesCompactRepresentation>,
    )>;

    /// Emit a local time-of-day using ISO 8601 extended separators.
    ///
    /// Normative source: ISO 8601-1:2019 - local time representation.
    fn format_local_time_extended(
        &self,
        time: &LocalTimeDescriptor,
        time_proof: Established<LocalTimeValid>,
    ) -> TemporalResult<(String, Established<Iso8601ExtendedFormUsesSeparators>)>;

    /// Emit a local time-of-day using ISO 8601 basic compact form.
    ///
    /// Normative source: ISO 8601-1:2019 - local time representation.
    fn format_local_time_basic(
        &self,
        time: &LocalTimeDescriptor,
        time_proof: Established<LocalTimeValid>,
    ) -> TemporalResult<(
        String,
        Established<Iso8601BasicFormUsesCompactRepresentation>,
    )>;

    /// Emit a reduced-accuracy local time-of-day using ISO 8601 extended separators.
    ///
    /// Normative source: ISO 8601-1:2019 - reduced accuracy local time representation.
    fn format_reduced_local_time_extended(
        &self,
        time: &ReducedLocalTimeDescriptor,
        time_proof: Established<ReducedLocalTimeValid>,
    ) -> TemporalResult<(String, Established<Iso8601ExtendedFormUsesSeparators>)>;

    /// Emit a reduced-accuracy local time-of-day using ISO 8601 basic compact form.
    ///
    /// Normative source: ISO 8601-1:2019 - reduced accuracy local time representation.
    fn format_reduced_local_time_basic(
        &self,
        time: &ReducedLocalTimeDescriptor,
        time_proof: Established<ReducedLocalTimeValid>,
    ) -> TemporalResult<(
        String,
        Established<Iso8601BasicFormUsesCompactRepresentation>,
    )>;

    /// Emit a numeric UTC offset using ISO 8601 extended separators.
    ///
    /// Normative source: ISO 8601-1:2019 - UTC offset representation.
    /// Informative cross-check: RFC 3339 §4.2-§4.4.
    fn format_utc_offset_extended(
        &self,
        offset: &UtcOffsetDescriptor,
        offset_proof: Established<UtcOffsetValid>,
    ) -> TemporalResult<(String, Established<Iso8601ExtendedFormUsesSeparators>)>;

    /// Emit a numeric UTC offset using ISO 8601 basic compact form.
    ///
    /// Normative source: ISO 8601-1:2019 - UTC offset representation.
    /// Informative cross-check: RFC 3339 §4.2-§4.4.
    fn format_utc_offset_basic(
        &self,
        offset: &UtcOffsetDescriptor,
        offset_proof: Established<UtcOffsetValid>,
    ) -> TemporalResult<(
        String,
        Established<Iso8601BasicFormUsesCompactRepresentation>,
    )>;

    /// Emit a combined local date-time using ISO 8601 extended separators.
    ///
    /// The local-semantics proof sidecar remains explicit at the formatter seam:
    /// the representation is structurally valid, but it does not identify a
    /// fixed instant without additional zone or offset law.
    ///
    /// Normative source: ISO 8601-1:2019 - combined date-time representation.
    fn format_local_date_time_extended(
        &self,
        timestamp: &LocalDateTimeDescriptor,
        timestamp_proof: Established<LocalDateTimeValid>,
        local_semantics: Established<LocalDateTimeDoesNotIdentifyFixedInstant>,
    ) -> TemporalResult<(String, Established<Iso8601ExtendedFormUsesSeparators>)>;

    /// Emit a combined local date-time using ISO 8601 basic compact form.
    ///
    /// The local-semantics proof sidecar remains explicit at the formatter seam:
    /// the representation is structurally valid, but it does not identify a
    /// fixed instant without additional zone or offset law.
    ///
    /// Normative source: ISO 8601-1:2019 - combined date-time representation.
    fn format_local_date_time_basic(
        &self,
        timestamp: &LocalDateTimeDescriptor,
        timestamp_proof: Established<LocalDateTimeValid>,
        local_semantics: Established<LocalDateTimeDoesNotIdentifyFixedInstant>,
    ) -> TemporalResult<(
        String,
        Established<Iso8601BasicFormUsesCompactRepresentation>,
    )>;

    /// Emit an offset date-time using ISO 8601 extended separators.
    ///
    /// The fixed-instant proof sidecar stays explicit at the seam so downstream
    /// consumers can rely on the represented instant without re-deriving that
    /// semantic fact from the lexical offset.
    ///
    /// Normative source: ISO 8601-1:2019 - date-time with UTC relationship.
    fn format_offset_date_time_extended(
        &self,
        timestamp: &OffsetDateTimeDescriptor,
        timestamp_proof: Established<OffsetDateTimeValid>,
        fixed_instant: Established<TimestampRepresentsFixedInstant>,
    ) -> TemporalResult<(
        String,
        Established<Iso8601ExtendedFormUsesSeparators>,
        Established<SerializationCarriesExplicitUtcRelationship>,
    )>;

    /// Emit an offset date-time using ISO 8601 basic compact form.
    ///
    /// The fixed-instant proof sidecar stays explicit at the seam so downstream
    /// consumers can rely on the represented instant without re-deriving that
    /// semantic fact from the lexical offset.
    ///
    /// Normative source: ISO 8601-1:2019 - date-time with UTC relationship.
    fn format_offset_date_time_basic(
        &self,
        timestamp: &OffsetDateTimeDescriptor,
        timestamp_proof: Established<OffsetDateTimeValid>,
        fixed_instant: Established<TimestampRepresentsFixedInstant>,
    ) -> TemporalResult<(
        String,
        Established<Iso8601BasicFormUsesCompactRepresentation>,
        Established<SerializationCarriesExplicitUtcRelationship>,
    )>;

    /// Emit a complete explicit-form date carrying a time shift.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3 - Date with shift.
    fn format_date_with_shift(
        &self,
        value: &DateWithShiftDescriptor,
        value_proof: Established<DateWithShiftValid>,
    ) -> FormattedDateWithShiftResult;

    /// Emit a complete explicit-form time of day carrying a time shift.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3 - Time of day with time shift.
    fn format_time_of_day_with_shift(
        &self,
        value: &TimeOfDayWithShiftDescriptor,
        value_proof: Established<TimeOfDayWithShiftValid>,
    ) -> FormattedTimeOfDayWithShiftResult;

    /// Emit an ISO 8601-2 extended year form.
    ///
    /// Normative source: ISO 8601-2:2019 - letter-prefixed, negative, exponential, and significant-digit year forms.
    /// Informative cross-check: public LOC EDTF Level 1 - Letter-prefixed calendar year; Negative calendar year. Level 2 - Exponential year; Significant digits.
    fn format_extended_year(
        &self,
        year: &ExtendedYearDescriptor,
        year_proof: Established<ExtendedYearValid>,
    ) -> FormattedExtendedYearResult;

    /// Emit an ISO 8601 decade representation from its neutral ordinal descriptor.
    ///
    /// Normative sources: ISO 8601-1:2019/Amd 1:2022 - decade component;
    /// ISO 8601-2:2019 - decade representations.
    fn format_decade(
        &self,
        decade: &DecadeDescriptor,
        decade_proof: Established<DecadeValid>,
    ) -> FormattedDecadeResult;

    /// Emit an ISO 8601 century representation from its neutral ordinal descriptor.
    ///
    /// Normative sources: ISO 8601-1:2019/Amd 1:2022 - century component;
    /// ISO 8601-2:2019 - century representations.
    fn format_century(
        &self,
        century: &CenturyDescriptor,
        century_proof: Established<CenturyValid>,
    ) -> FormattedCenturyResult;

    /// Emit a qualified temporal value with explicit uncertainty and/or approximation semantics.
    ///
    /// Normative source: ISO 8601-2:2019 - qualification of temporal expressions.
    fn format_qualified_temporal_value(
        &self,
        value: &QualifiedTemporalValueDescriptor,
        value_proof: Established<QualifiedTemporalValueValid>,
    ) -> FormattedQualifiedTemporalValueResult;

    /// Emit a fixed-instant timestamp using the RFC 3339 profile.
    ///
    /// Normative source: RFC 3339 §5.6 - Internet timestamp profile.
    fn format_rfc3339_timestamp(
        &self,
        timestamp: &OffsetDateTimeDescriptor,
        timestamp_proof: Established<Rfc3339TimestampValid>,
        fixed_instant: Established<TimestampRepresentsFixedInstant>,
    ) -> TemporalResult<(
        String,
        Established<SerializationCarriesExplicitUtcRelationship>,
    )>;

    /// Emit a named-zone timestamp using an IXDTF zone annotation.
    ///
    /// Normative source: RFC 9557 §3.1 - named time-zone annotations.
    fn format_ixdtf_zoned_timestamp(
        &self,
        timestamp: &ZonedDateTimeDescriptor,
        fixed_instant: Established<TimestampRepresentsFixedInstant>,
        zone_proof: Established<ZonedDateTimeHasNamedZone>,
        consistency_proof: Established<OffsetConsistentWithNamedZone>,
    ) -> TemporalResult<(
        String,
        Established<IxdtfSerializationCarriesNamedZoneAnnotation>,
    )>;

    /// Emit a full IXDTF timestamp, preserving explicit suffix-proof branches.
    ///
    /// The returned exchange shape keeps named-zone identity, offset-zone
    /// compatibility semantics, preferred-calendar annotations, and
    /// additional-information semantics as separate proof branches rather than
    /// collapsing them into one aggregate suffix token.
    ///
    /// Normative source: RFC 9557 §3.1-§3.3 - time-zone and additional-information syntax.
    fn format_ixdtf_timestamp(
        &self,
        timestamp: &IxdtfTimestampDescriptor,
        timestamp_proof: Established<IxdtfTimestampValid>,
        fixed_instant: Established<TimestampRepresentsFixedInstant>,
    ) -> FormattedIxdtfTimestampResult;

    /// Emit an ISO 8601-2 seasonal temporal expression.
    ///
    /// Normative source: ISO 8601-2:2019 - seasons and seasonal temporal expressions.
    /// Informative cross-check: public LOC EDTF Level 1 - Seasons.
    fn format_seasonal_temporal_expression(
        &self,
        expression: &SeasonalTemporalExpressionDescriptor,
        expression_proof: Established<SeasonalTemporalExpressionValid>,
    ) -> FormattedSeasonalTemporalExpressionResult;

    /// Emit an ISO 8601-2 Level 2 sub-year grouping expression.
    ///
    /// Normative source: ISO 8601-2:2019 - sub-year groupings.
    /// Informative cross-check: public LOC EDTF Level 2 - Sub-year groupings.
    fn format_sub_year_grouping_expression(
        &self,
        expression: &SubYearGroupingExpressionDescriptor,
        expression_proof: Established<SubYearGroupingExpressionValid>,
    ) -> FormattedSubYearGroupingExpressionResult;

    /// Emit an ISO 8601-2 unspecified-component temporal expression.
    ///
    /// Normative source: ISO 8601-2:2019 - unspecified digits and unspecified components.
    /// Informative cross-check: public LOC EDTF Level 1 - Unspecified digit(s) from the right; Level 2 - Unspecified Digit.
    fn format_unspecified_component_expression(
        &self,
        expression: &UnspecifiedComponentExpressionDescriptor,
        expression_proof: Established<UnspecifiedComponentExpressionValid>,
    ) -> FormattedUnspecifiedComponentExpressionResult;

    /// Emit an ISO 8601-2 temporal set expression including range refinements.
    ///
    /// Normative source: ISO 8601-2:2019 - temporal sets and set representation refinements.
    /// Informative cross-check: public LOC EDTF Level 2 - Set representation.
    fn format_temporal_set(
        &self,
        expression: &TemporalSetDescriptor,
        expression_proof: Established<TemporalSetExpressionValid>,
        range_proof: Established<TemporalSetRangeSemanticsValid>,
    ) -> FormattedTemporalSetResult;

    /// Emit a grouped time scale unit expression through the neutral ISO 8601-2 seam.
    ///
    /// Normative source: ISO 8601-2:2019 - grouped time scale units.
    /// Informative cross-check: CalConnect CC 18011:2018 §5 - Grouped time scale units.
    fn format_grouped_time_scale_unit(
        &self,
        grouped: &GroupedTimeScaleUnitDescriptor,
        grouped_proof: Established<GroupedTimeScaleUnitValid>,
    ) -> FormattedGroupedTimeScaleUnitResult;

    /// Emit a date-time formula through the neutral ISO 8601-2 seam.
    ///
    /// Normative source: ISO 8601-2:2019 - date and time arithmetic.
    /// Informative cross-check: CalConnect CC 18011:2018 §8 - Evaluation of date and time with duration.
    fn format_date_time_formula(
        &self,
        formula: &DateTimeFormulaDescriptor,
        formula_proof: Established<DateTimeFormulaValid>,
        semantics_proof: Established<DateTimeFormulaEvaluationSemanticsValid>,
    ) -> FormattedDateTimeFormulaResult;

    /// Emit an ISO 8601 duration representation.
    ///
    /// Normative sources: ISO 8601-1:2019, 4.4.2 b); 4.4.3.
    fn format_duration(
        &self,
        duration: &DurationDescriptor,
        duration_proof: Established<DurationFormValid>,
    ) -> FormattedDurationResult;

    /// Emit an ISO 8601 recurring interval representation.
    ///
    /// Normative sources: ISO 8601-1:2019, 4.5.1; 4.5.2; 4.5.3; 4.5.4.
    fn format_recurring_interval(
        &self,
        interval: &RecurringIntervalDescriptor,
        interval_proof: Established<RecurringIntervalFormValid>,
    ) -> FormattedRecurringIntervalResult;

    /// Emit an ISO 8601 interval representation including extended-boundary semantics.
    ///
    /// Normative sources: ISO 8601-1:2019, 4.4.1; 4.4.2; 4.4.4; 4.4.5;
    /// ISO 8601-2:2019, 10.2.
    fn format_time_interval(
        &self,
        interval: &TimeIntervalDescriptor,
        interval_proof: Established<TimeIntervalValid>,
    ) -> FormattedTimeIntervalResult;
}
