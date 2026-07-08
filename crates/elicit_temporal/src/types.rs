//! Neutral temporal descriptors used in trait signatures.
//!
//! These types are the branch-level accord vocabulary for temporal exchange.
//! Leaf crates map concrete backend values into these descriptors before
//! crossing architectural boundaries.

use derive_builder::Builder;
use derive_more::Display;
use elicitation::Established;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    BackendConversionEvidence, BackendConversionSemanticsValid, CenturyValid,
    CompleteDurationEndIntervalSubstitutionEvidence, CompleteIntervalSubstitutionSemanticsValid,
    CompleteRecurringIntervalRepresentationEvidence,
    CompleteRecurringIntervalRepresentationSemanticsValid,
    CompleteStartDurationIntervalSubstitutionEvidence,
    CompleteStartEndIntervalSubstitutionEvidence, ConversionDropsNamedZoneIdentity,
    ConversionLossless, ConversionPreservesRepresentedInstant, ConversionPreservesTemporalOrdering,
    ConversionTruncatesSubseconds, DateTimeFormulaEvaluationResultEvidence,
    DateTimeFormulaEvaluationResultValid, DateTimeFormulaEvaluationSemanticsEvidence,
    DateTimeFormulaEvaluationSemanticsValid, DateTimeFormulaEvidence, DateTimeFormulaValid,
    DateWithShiftValid, DecadeValid,
    DurationAlternativeFormEvidence, DurationDesignatorRepresentationEvidence, DurationFormEvidence,
    DurationFormValid, DurationRepresentationSemanticsValid, ExplicitDurationMayBeNegative,
    ExplicitDurationMayUseFractionalLowestOrderUnit, ExplicitDurationEvidence,
    ExplicitDurationRepresentationEvidence, ExplicitDurationSemanticEvidence,
    ExplicitDurationUsesDurationalUnitDesignators, ExplicitDurationValid,
    ExplicitIntervalDurationSubstitutionEvidence,
    ExplicitIntervalDurationSubstitutionSemanticsValid,
    ExplicitIntervalEndComponentInheritanceEvidence,
    ExplicitIntervalEndComponentInheritanceSemanticsValid,
    ExplicitIntervalShiftPropagationEvidence, ExplicitIntervalShiftPropagationSemanticsValid,
    ExplicitTemporalFormEvidence, ExplicitTemporalFormMayOmitZeroValuedComponents,
    ExplicitTemporalFormUsesDesignatorSymbols, ExplicitTemporalFormValid,
    ExplicitTemporalPrecisionUsesLowestDenotedComponent, ExplicitTimeIntervalEvidence,
    ExplicitTimeIntervalValid, ExplicitUtcRelationshipUsesZuluOrSignedShift,
    ExtendedIntervalBoundarySemanticsValid, ExtendedYearValid, FixedInstantEvidence,
    GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits, GroupedTimeScaleUnitConvertsToTimeInterval,
    GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift,
    GroupedTimeScaleUnitDefinitionIsContinuous, GroupedTimeScaleUnitEvidence,
    GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds,
    GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder, GroupedTimeScaleUnitUsesGroupingDesignators,
    GroupedTimeScaleUnitValid, GroupedTimeScaleUnitValueCarriesExplicitCoefficient,
    InheritedIntervalEndComponentsEvidence, InheritedIntervalEndComponentsSemanticsValid,
    InheritedIntervalZoneEvidence, InheritedIntervalZoneSemanticsValid,
    IntervalEndpointOrderingEvidence, IntervalEndpointsOrdered,
    IxdtfAdditionalInformationEvidence, IxdtfAdditionalInformationSemanticsValid,
    IxdtfCalendarAwareTimestampEvidence, IxdtfTimestampHasPreferredPresentationCalendar,
    IxdtfTimestampValid, LevelOneUnspecifiedDigitsOccupyRightmostPositions,
    LevelTwoUnspecifiedDigitsMayAppearWithinComponent, LocalDateTimeDoesNotIdentifyFixedInstant,
    LocalDateTimeEvidence, LocalDateTimeMayBeAmbiguousAtZoneTransition,
    LocalDateTimeMayFallInZoneTransitionGap, LocalDateTimeValid, LocalTimestampSemanticsEvidence,
    LosslessConversionEvidence, LossyConversionAuthorityEvidence,
    LossyConversionAuthorityValid, NamedTimeZoneIdentityEvidence, NamedTimeZoneIdentityValid,
    NamedTimeZoneInterpretationTracksTzdbRevision, NamedTimeZoneRevisionEvidence,
    NamedZoneAttachmentEvidence, OffsetConsistencyEvidence, OffsetConsistentWithNamedZone,
    OffsetDateTimeEvidence, OffsetDateTimeValid, OffsetTimeZoneAnnotationConsistentWithTimestamp,
    OffsetTimeZoneAnnotationEvidence, OtherThanCompleteRecurringIntervalRepresentationEvidence,
    OtherThanCompleteRecurringIntervalRepresentationSemanticsValid, QualificationPlacementEvidence,
    QualifiedTemporalExpressionEvidence, QualifiedTemporalExpressionValid,
    QualifiedTemporalValueEvidence, QualifiedTemporalValueValid, RecurringIntervalEvidence,
    RecurringIntervalFormValid, RecurringIntervalWithRepeatRuleValid, ReducedCalendarDateValid,
    ReducedLocalTimeValid,
    RepeatRuleDeclaresEligibleTimeIntervals,
    RepeatRuleEvaluationInheritsInitialStartComponentInformation,
    RepeatRuleSelectionAppliesWithinEligibleIntervals, RepeatRuleUsesFrequencyDesignator,
    RepeatRuleValid, Rfc3339TimestampValid, SeasonCodeDeclaresNamedSeason,
    SeasonCodeDeclaresSeasonScope, SeasonalExpressionUsesSeasonCodeInMonthSlot,
    SeasonalExpressionUsesYearAndSeasonForm, SeasonalTemporalExpressionValid,
    SelectionExpressionMaySelectSingleInstance,
    SelectionExpressionUsesRecognizedSelectionRuleVocabulary,
    SelectionExpressionUsesSelectionDelimiters, SelectionExpressionValid,
    SelectionRuleDayOfMonthUsesDayExpression, SelectionRuleHourUsesHourExpression,
    SelectionRuleMinuteUsesMinuteExpression, SelectionRuleMonthUsesMonthExpression,
    SelectionRuleOrdinalDayOfYearUsesOrdinalDayExpression, SelectionRulePositionAppliesLast,
    SelectionRulePositionUsesInstanceDesignatorSuffix, SelectionRuleSecondUsesSecondExpression,
    SelectionRuleWeekDayUsesDayOfWeekExpression, SelectionRuleWeekUsesWeekExpression,
    SelectionRulesApplyWithinSelectedResults, SelectionWithDurationUsesDurationSuffix,
    SubYearGroupingExpressionUsesGroupingCodeInMonthSlot,
    SubYearGroupingExpressionUsesYearAndGroupingForm, SubYearGroupingExpressionValid,
    SubYearGroupingKindEvidence, SubsecondTruncationEvidence, TemporalResult,
    TemporalSetExpressionEvidence, TemporalSetExpressionValid, TemporalSetRangeSemanticsEvidence,
    TemporalSetRangeSemanticsValid, TimeIntervalValid,
    TimeOfDayWithShiftValid, TimestampRepresentsFixedInstant, UnspecifiedComponentExpressionValid,
    UnspecifiedDigitUsesUppercaseXPlaceholder, UnspecifiedDigitsDeclareUnknownValue,
    TimeIntervalEvidence, ZoneTransitionAmbiguityEvidence,
    ZoneTransitionAmbiguitySemanticsValid, ZoneTransitionGapEvidence,
    ZoneTransitionGapSemanticsValid, ZoneTransitionResolutionAuthorityEvidence,
    ZoneTransitionResolutionAuthorityValid, ZonedDateTimeHasNamedZone, ZonedTimestampEvidence,
};

/// Smallest named temporal unit relevant to ISO 8601 precision rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display)]
pub enum TemporalComponent {
    /// Calendar year.
    #[display("year")]
    Year,
    /// Calendar decade.
    #[display("decade")]
    Decade,
    /// Calendar century.
    #[display("century")]
    Century,
    /// Calendar month.
    #[display("month")]
    Month,
    /// Calendar day.
    #[display("day")]
    Day,
    /// Clock hour.
    #[display("hour")]
    Hour,
    /// Clock minute.
    #[display("minute")]
    Minute,
    /// Clock second.
    #[display("second")]
    Second,
}

/// Time scale units recognized by the broader explicit and recurrence standards family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display)]
pub enum TimeScaleUnitDescriptor {
    /// Calendar year.
    #[display("year")]
    Year,
    /// Calendar decade.
    #[display("decade")]
    Decade,
    /// Calendar century.
    #[display("century")]
    Century,
    /// Calendar month.
    #[display("month")]
    Month,
    /// Calendar week.
    #[display("week")]
    Week,
    /// Calendar day.
    #[display("day")]
    Day,
    /// Clock hour.
    #[display("hour")]
    Hour,
    /// Clock minute.
    #[display("minute")]
    Minute,
    /// Clock second.
    #[display("second")]
    Second,
}

/// Lexical placement family for ISO 8601-2 qualification markers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display)]
pub enum QualificationPlacementDescriptor {
    /// The qualification marker appears immediately to the right of the marked component.
    #[display("group-right")]
    GroupRight,
    /// The qualification marker appears immediately to the left of the marked component.
    #[display("component-left")]
    ComponentLeft,
}

/// Declared scope for ISO 8601-2 qualification markers.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum QualificationScopeDescriptor {
    /// The qualification applies to the named component and all more significant components to its left.
    ThroughComponent(TemporalComponent),
    /// The qualification applies only to the named component.
    OnlyComponent(TemporalComponent),
}

/// Explicit ISO 8601-2 uncertainty or approximation qualification metadata.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct QualifiedTemporalExpressionDescriptor {
    /// Whether uncertainty is explicitly declared.
    pub uncertainty: bool,
    /// Whether approximation is explicitly declared.
    pub approximation: bool,
    /// Lexical placement family used by the qualification marker.
    pub placement: QualificationPlacementDescriptor,
    /// Scope to which the qualification applies.
    pub scope: QualificationScopeDescriptor,
}

/// Fractional second digits preserved as a lexical decimal suffix.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct FractionalSecondDescriptor {
    /// Decimal digits appearing after the fractional separator.
    pub digits: String,
}

/// Complete Gregorian calendar date descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct CalendarDateDescriptor {
    /// Signed calendar year.
    pub year: i32,
    /// Calendar month number.
    pub month: u8,
    /// Day of month.
    pub day: u8,
}

/// Reduced-precision Gregorian calendar date descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum ReducedCalendarDateDescriptor {
    /// Year-only calendar date form.
    Year {
        /// Signed calendar year.
        year: i32,
    },
    /// Year-month calendar date form.
    YearMonth {
        /// Signed calendar year.
        year: i32,
        /// Calendar month number.
        month: u8,
    },
}

/// Gregorian calendar decade descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct DecadeDescriptor {
    /// Three-digit ordinal number for the Gregorian calendar decade.
    pub ordinal: u16,
    /// Whether the explicit expression refers to a value before year one using the `B` suffix.
    #[builder(default)]
    pub before_year_one: bool,
}

/// Gregorian calendar century descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct CenturyDescriptor {
    /// Two-digit ordinal number for the Gregorian calendar century.
    pub ordinal: u8,
    /// Whether the explicit expression refers to a value before year one using the `B` suffix.
    #[builder(default)]
    pub before_year_one: bool,
}

/// Base lexical form for an ISO 8601-2 extended year expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum ExtendedYearBaseDescriptor {
    /// Four-digit year form, which may also carry ISO 8601-2 extensions such as
    /// negative-year or significant-digit semantics.
    FourDigit {
        /// Signed year value.
        year: i32,
    },
    /// Letter-prefixed year form using a leading `Y` designator.
    LetterPrefixed {
        /// Signed year value.
        year: i64,
    },
    /// Letter-prefixed exponential year form.
    Exponential {
        /// Signed significand before the `E` separator.
        significand: i64,
        /// Positive exponent after the `E` separator.
        exponent: u32,
    },
}

/// Neutral ISO 8601-2 extended year descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct ExtendedYearDescriptor {
    /// The base year form carried by the expression.
    pub base: ExtendedYearBaseDescriptor,
    /// Optional number of significant digits declared by a trailing `S` suffix.
    #[builder(default)]
    pub significant_digits: Option<u32>,
}

/// Complete ordinal date descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct OrdinalDateDescriptor {
    /// Signed calendar year.
    pub year: i32,
    /// Day number within the year.
    pub day_of_year: u16,
}

/// Complete week-date descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct WeekDateDescriptor {
    /// ISO week-based year.
    pub week_year: i32,
    /// ISO week number.
    pub week: u8,
    /// ISO weekday number (`1` = Monday, `7` = Sunday).
    pub weekday: u8,
}

/// Complete date representation family recognized by the explicit-form standards.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum CompleteDateDescriptor {
    /// Complete Gregorian calendar date.
    Calendar(CalendarDateDescriptor),
    /// Complete ordinal date.
    Ordinal(OrdinalDateDescriptor),
    /// Complete week date.
    Week(WeekDateDescriptor),
}

/// Date representation family for the ISO 8601 Clause 2 umbrella date concept.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum DateDescriptor {
    /// Gregorian calendar date.
    Calendar(CalendarDateDescriptor),
    /// Ordinal date.
    Ordinal(OrdinalDateDescriptor),
    /// Week date.
    Week(WeekDateDescriptor),
}

/// Local time-of-day descriptor.
///
/// Rust uses the shorter `LocalTime` name for the ISO 8601 local-time-of-day family.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct LocalTimeDescriptor {
    /// Hour of day.
    pub hour: u8,
    /// Minute of hour.
    pub minute: u8,
    /// Second of minute.
    pub second: u8,
    /// Fractional-second suffix when present.
    #[builder(default)]
    pub fractional_second: Option<FractionalSecondDescriptor>,
}

/// Reduced-accuracy local time-of-day descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum ReducedLocalTimeDescriptor {
    /// Hour-only local time form.
    Hour {
        /// Hour of day.
        hour: u8,
        /// Optional decimal fraction attached to the hour component.
        fractional_component: Option<TimeScaleUnitFractionDescriptor>,
    },
    /// Hour-minute local time form.
    HourMinute {
        /// Hour of day.
        hour: u8,
        /// Minute of hour.
        minute: u8,
        /// Optional decimal fraction attached to the minute component.
        fractional_component: Option<TimeScaleUnitFractionDescriptor>,
    },
}

/// Explicit-form local time-of-day descriptor used by the CalConnect `timeE` family.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct ExplicitTimeOfDayDescriptor {
    /// Hour of day.
    pub hour: u8,
    /// Minute of hour when the representation is at least minute precision.
    #[builder(default)]
    pub minute: Option<u8>,
    /// Second of minute when the representation is at least second precision.
    #[builder(default)]
    pub second: Option<u8>,
    /// Fractional-second suffix when present.
    #[builder(default)]
    pub fractional_second: Option<FractionalSecondDescriptor>,
    /// Lowest denoted time scale unit, which declares the explicit-form precision.
    pub precision: TimeScaleUnitDescriptor,
    /// Zero-valued time units intentionally omitted from the lexical representation.
    #[builder(default)]
    pub omitted_zero_components: Vec<TimeScaleUnitDescriptor>,
}

/// Sign of a numeric UTC offset.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, Default,
)]
pub enum UtcOffsetSign {
    /// Positive or east-of-UTC offset.
    #[default]
    #[display("+")]
    Positive,
    /// Negative or west-of-UTC offset.
    #[display("-")]
    Negative,
}

/// Semantic interpretation of a UTC relationship payload in an offset-aware timestamp.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, Default,
)]
pub enum UtcOffsetRelationship {
    /// The local offset is known.
    #[default]
    #[display("known")]
    Known,
    /// RFC 9557-updated RFC 3339 unknown-local-offset semantics.
    #[display("unknown-local-offset")]
    UnknownLocalOffset,
}

/// UTC-offset relationship descriptor for an offset-aware timestamp.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct UtcOffsetDescriptor {
    /// Offset sign when the UTC relationship is carried numerically.
    pub sign: UtcOffsetSign,
    /// Absolute hour component for numeric-offset forms.
    pub hours: u8,
    /// Absolute minute component when the offset is represented with hour-minute precision.
    ///
    /// `None` denotes the integral-hour form permitted by ISO 8601.
    #[builder(default)]
    pub minutes: Option<u8>,
    /// Whether the relationship is a known local offset or the RFC 9557-updated unknown-offset case.
    #[builder(default)]
    pub relationship: UtcOffsetRelationship,
}

/// Descriptor for the UTC reference time scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display)]
pub enum UtcTimeScaleDescriptor {
    /// Coordinated Universal Time.
    #[display("UTC")]
    Utc,
}

/// UTC-of-day descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct UtcOfDayDescriptor {
    /// Time-of-day payload carried on the UTC time scale.
    pub time: LocalTimeDescriptor,
    /// Explicit UTC time-scale identity.
    pub scale: UtcTimeScaleDescriptor,
}

/// Standard-time descriptor derived from UTC by a local shift.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct StandardTimeDescriptor {
    /// Explicit UTC reference time scale from which the standard time is derived.
    pub reference: UtcTimeScaleDescriptor,
    /// Constant local shift from UTC.
    pub shift: UtcOffsetDescriptor,
}

/// Standard-time-of-day descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct StandardTimeOfDayDescriptor {
    /// Local wall-clock time of day.
    pub time: LocalTimeDescriptor,
    /// Standard-time scale carried alongside that wall-clock time.
    pub scale: StandardTimeDescriptor,
}

/// Locally applicable time-scale family for a local wall-clock time.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum LocalTimeScaleDescriptor {
    /// Standard time derived from UTC by a local shift.
    Standard(StandardTimeDescriptor),
    /// Another locally applicable time scale not defined as a UTC-derived standard time.
    NonUtcBased,
}

/// Time-of-day representation family carried by the ISO 8601 exchange surface.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum TimeDescriptor {
    /// Local clock time.
    Local(LocalTimeDescriptor),
    /// Reduced-accuracy local clock time.
    ReducedLocal(ReducedLocalTimeDescriptor),
    /// UTC-of-day form.
    Utc(UtcOfDayDescriptor),
    /// Standard-time-of-day form.
    Standard(StandardTimeOfDayDescriptor),
}

/// Explicit-form time-shift descriptor used by the CalConnect `shiftE` family.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct ExplicitTimeShiftDescriptor {
    /// Shift sign. Positive or zero shifts omit an explicit plus sign in lexical form.
    #[builder(default)]
    pub sign: UtcOffsetSign,
    /// Optional explicit time-of-day payload after the leading `Z` designator.
    ///
    /// `None` denotes the bare `Z` form, which represents UTC with zero shift.
    #[builder(default)]
    pub time: Option<ExplicitTimeOfDayDescriptor>,
}

/// Combined complete ISO date and local clock time.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct LocalDateTimeDescriptor {
    /// Complete date component.
    pub date: CompleteDateDescriptor,
    /// Local clock-time component.
    pub time: LocalTimeDescriptor,
}

/// Offset-aware timestamp descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct OffsetDateTimeDescriptor {
    /// Local civil timestamp fields.
    pub local: LocalDateTimeDescriptor,
    /// UTC relationship carried by the timestamp.
    pub offset: UtcOffsetDescriptor,
}

/// Explicit-form complete date plus explicit-form local time of day.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct ExplicitDateTimeDescriptor {
    /// Complete date component.
    pub date: CompleteDateDescriptor,
    /// Local time-of-day component, which may carry reduced precision.
    pub time: ExplicitTimeOfDayDescriptor,
}

/// Explicit-form complete date plus local time of day and time shift.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct ExplicitDateTimeWithShiftDescriptor {
    /// Complete local date-and-time component.
    pub local: ExplicitDateTimeDescriptor,
    /// Explicit time-shift component.
    pub shift: ExplicitTimeShiftDescriptor,
}

/// Complete explicit-form date carrying a time shift.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct DateWithShiftDescriptor {
    /// Complete date component.
    pub date: CompleteDateDescriptor,
    /// Explicit time shift carried alongside the date.
    pub shift: ExplicitTimeShiftDescriptor,
}

/// Complete explicit-form local time of day carrying a time shift.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct TimeOfDayWithShiftDescriptor {
    /// Explicit local time-of-day component.
    pub time: ExplicitTimeOfDayDescriptor,
    /// Explicit time shift carried alongside the time of day.
    pub shift: ExplicitTimeShiftDescriptor,
}

/// CalConnect `[datetimeE]` endpoint family admitted at explicit-interval boundaries.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum ExplicitTimeIntervalEndpointDescriptor {
    /// Explicit local date and time.
    DateTime(ExplicitDateTimeDescriptor),
    /// Explicit local date and time with time shift.
    DateTimeWithShift(ExplicitDateTimeWithShiftDescriptor),
    /// Explicit date with time shift.
    DateWithShift(DateWithShiftDescriptor),
    /// Explicit time of day with time shift.
    TimeOfDayWithShift(TimeOfDayWithShiftDescriptor),
}

/// Top-level CalConnect explicit time-interval representation form.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum ExplicitTimeIntervalRepresentation {
    /// Explicit start and end boundaries.
    StartEnd {
        /// Interval start boundary.
        start: ExplicitTimeIntervalEndpointDescriptor,
        /// Interval end boundary.
        end: ExplicitTimeIntervalEndpointDescriptor,
    },
    /// Explicit start boundary and an explicit duration.
    StartDuration {
        /// Interval start boundary.
        start: ExplicitTimeIntervalEndpointDescriptor,
        /// Interval duration.
        duration: ExplicitDurationDescriptor,
    },
    /// Explicit duration followed by an explicit end boundary.
    DurationEnd {
        /// Interval duration.
        duration: ExplicitDurationDescriptor,
        /// Interval end boundary.
        end: ExplicitTimeIntervalEndpointDescriptor,
    },
}

/// Neutral CalConnect explicit time-interval descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct ExplicitTimeIntervalDescriptor {
    /// One of the legal CalConnect explicit interval top-level forms.
    pub representation: ExplicitTimeIntervalRepresentation,
}

/// Named time-zone identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct NamedTimeZoneDescriptor {
    /// IANA time-zone identifier.
    pub identifier: String,
    /// Optional TZDB revision the producer associated with the interpretation.
    #[builder(default)]
    pub tzdb_revision: Option<String>,
}

/// Declared authority for resolving an ambiguous repeated local wall-clock time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display)]
pub enum ZoneAmbiguityResolutionDescriptor {
    /// Prefer the earlier matching fixed instant.
    #[display("prefer-earlier")]
    PreferEarlier,
    /// Prefer the later matching fixed instant.
    #[display("prefer-later")]
    PreferLater,
}

/// Declared authority for resolving a skipped local wall-clock time inside a gap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display)]
pub enum ZoneGapResolutionDescriptor {
    /// Advance to the first valid instant after the gap.
    #[display("shift-forward")]
    ShiftForward,
    /// Retreat to the last valid instant before the gap.
    #[display("shift-backward")]
    ShiftBackward,
}

/// Explicit authority bundle for resolving local wall-clock timestamps against a named zone.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct LocalTimeZoneResolutionAuthorityDescriptor {
    /// Authority for repeated local times during backward transitions.
    pub ambiguity: ZoneAmbiguityResolutionDescriptor,
    /// Authority for skipped local times during forward transitions.
    pub gap: ZoneGapResolutionDescriptor,
}

/// Zoned timestamp descriptor pairing a fixed-instant form with a named zone.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct ZonedDateTimeDescriptor {
    /// Offset timestamp representation.
    pub timestamp: OffsetDateTimeDescriptor,
    /// Named-zone identity.
    pub zone: NamedTimeZoneDescriptor,
}

/// Additional IXDTF annotation carried after the base RFC 3339 timestamp.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct IxdtfAnnotationDescriptor {
    /// Annotation key.
    pub key: String,
    /// One or more hyphen-delimited annotation values.
    #[builder(default)]
    pub values: Vec<String>,
    /// Whether the annotation is marked critical with a leading `!`.
    #[builder(default)]
    pub critical: bool,
}

/// Preferred-presentation calendar annotation carried by RFC 9557 `u-ca`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct IxdtfCalendarAnnotationDescriptor {
    /// Unicode calendar identifier carried by the `u-ca` suffix key.
    pub identifier: String,
    /// Whether the annotation is marked critical with a leading `!`.
    #[builder(default)]
    pub critical: bool,
}

/// Fractional duration component represented lexically.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct DurationFractionDescriptor {
    /// Lowest-order component to which the fraction applies.
    pub component: TemporalComponent,
    /// Decimal digits appearing after the fractional separator.
    pub digits: String,
}

/// Fractional time scale unit represented lexically for explicit-duration forms.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct TimeScaleUnitFractionDescriptor {
    /// Lowest-order unit to which the fraction applies.
    pub unit: TimeScaleUnitDescriptor,
    /// Decimal digits appearing after the fractional separator.
    pub digits: String,
}

/// Neutral ISO 8601 duration descriptor.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder, Default,
)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct DurationDescriptor {
    /// Whole years component.
    #[builder(default)]
    pub years: u32,
    /// Whole months component.
    #[builder(default)]
    pub months: u32,
    /// Whole weeks component.
    #[builder(default)]
    pub weeks: u32,
    /// Whole days component.
    #[builder(default)]
    pub days: u32,
    /// Whole hours component.
    #[builder(default)]
    pub hours: u32,
    /// Whole minutes component.
    #[builder(default)]
    pub minutes: u32,
    /// Whole seconds component.
    #[builder(default)]
    pub seconds: u32,
    /// Fraction applied to the lowest-order declared component when present.
    #[builder(default)]
    pub fractional_component: Option<DurationFractionDescriptor>,
}

/// Signedness carried by a CalConnect explicit duration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display)]
pub enum ExplicitDurationSignDescriptor {
    /// The duration advances forward in time.
    #[display("positive")]
    Positive,
    /// The duration advances backward in time.
    #[display("negative")]
    Negative,
}

/// Representation family carried by a CalConnect explicit duration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display)]
pub enum ExplicitDurationRepresentationKindDescriptor {
    /// A simple duration expression with a single ordered component sequence.
    #[display("simple")]
    Simple,
    /// A composite duration expression.
    #[display("composite")]
    Composite,
    /// A precedence duration expression whose component order is semantically relevant.
    #[display("precedence")]
    Precedence,
}

/// Semantics family declared for a CalConnect explicit duration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display)]
pub enum ExplicitDurationSemanticKindDescriptor {
    /// The duration denotes an exact span independent of placement context.
    #[display("exact")]
    Exact,
    /// The duration's realized span depends on placement on the time scale.
    #[display("context-dependent")]
    ContextDependent,
    /// The duration's realized span depends on future knowledge such as leap-second announcements.
    #[display("speculative")]
    Speculative,
}

/// CalConnect explicit duration descriptor preserving sign, order, and semantic family.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct ExplicitDurationDescriptor {
    /// Whether the duration advances forward or backward in time.
    #[builder(default = "ExplicitDurationSignDescriptor::Positive")]
    pub sign: ExplicitDurationSignDescriptor,
    /// Whether the duration uses simple, composite, or precedence semantics.
    #[builder(default = "ExplicitDurationRepresentationKindDescriptor::Simple")]
    pub representation_kind: ExplicitDurationRepresentationKindDescriptor,
    /// Ordered durational units preserved exactly as exchanged.
    #[builder(default)]
    pub components: Vec<TimeScaleUnitValueDescriptor>,
    /// Fraction applied to the lowest-order declared unit when present.
    #[builder(default)]
    pub fractional_component: Option<TimeScaleUnitFractionDescriptor>,
    /// Optional exactness family declared for the duration.
    #[builder(default)]
    pub semantic_kind: Option<ExplicitDurationSemanticKindDescriptor>,
}

/// Named season carried by an ISO 8601-2 seasonal temporal expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display)]
pub enum NamedSeasonDescriptor {
    /// Spring season.
    #[display("spring")]
    Spring,
    /// Summer season.
    #[display("summer")]
    Summer,
    /// Autumn season.
    #[display("autumn")]
    Autumn,
    /// Winter season.
    #[display("winter")]
    Winter,
}

/// Scope declared by an ISO 8601-2 season code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display)]
pub enum SeasonScopeDescriptor {
    /// Named season without explicit hemisphere qualification.
    #[display("location-independent")]
    LocationIndependent,
    /// Named season qualified to the northern hemisphere.
    #[display("northern-hemisphere")]
    NorthernHemisphere,
    /// Named season qualified to the southern hemisphere.
    #[display("southern-hemisphere")]
    SouthernHemisphere,
}

/// Neutral ISO 8601-2 seasonal temporal expression descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct SeasonalTemporalExpressionDescriptor {
    /// Signed calendar year paired with the season code.
    pub year: i32,
    /// Named season declared by the expression.
    pub season: NamedSeasonDescriptor,
    /// Whether the season is location-independent or hemisphere-qualified.
    pub scope: SeasonScopeDescriptor,
    /// Optional uncertainty or approximation qualification metadata.
    #[builder(default)]
    pub qualification: Option<QualifiedTemporalExpressionDescriptor>,
}

/// Quarter grouping within a calendar year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display)]
pub enum QuarterOfYearDescriptor {
    /// First quarter of the year.
    #[display("Q1")]
    First,
    /// Second quarter of the year.
    #[display("Q2")]
    Second,
    /// Third quarter of the year.
    #[display("Q3")]
    Third,
    /// Fourth quarter of the year.
    #[display("Q4")]
    Fourth,
}

/// Quadrimester grouping within a calendar year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display)]
pub enum QuadrimesterOfYearDescriptor {
    /// First quadrimester of the year.
    #[display("quadrimester-1")]
    First,
    /// Second quadrimester of the year.
    #[display("quadrimester-2")]
    Second,
    /// Third quadrimester of the year.
    #[display("quadrimester-3")]
    Third,
}

/// Semestral grouping within a calendar year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display)]
pub enum SemestralHalfDescriptor {
    /// First semestral half of the year.
    #[display("semestral-1")]
    First,
    /// Second semestral half of the year.
    #[display("semestral-2")]
    Second,
}

/// Neutral vocabulary for ISO 8601-2 Level 2 sub-year groupings.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum SubYearGroupingDescriptor {
    /// Season grouping, including hemisphere scope when declared.
    Season {
        /// Named season declared by the code.
        season: NamedSeasonDescriptor,
        /// Whether the season is location-independent or hemisphere-qualified.
        scope: SeasonScopeDescriptor,
    },
    /// Quarter grouping.
    Quarter(QuarterOfYearDescriptor),
    /// Quadrimester grouping.
    Quadrimester(QuadrimesterOfYearDescriptor),
    /// Semestral grouping.
    Semestral(SemestralHalfDescriptor),
}

/// Neutral ISO 8601-2 Level 2 sub-year grouping descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct SubYearGroupingExpressionDescriptor {
    /// Signed calendar year paired with the grouping code.
    pub year: i32,
    /// Specific sub-year grouping declared by the expression.
    pub grouping: SubYearGroupingDescriptor,
    /// Optional uncertainty or approximation qualification metadata.
    #[builder(default)]
    pub qualification: Option<QualifiedTemporalExpressionDescriptor>,
}

/// One digit of a masked ISO 8601-2 numeric component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display)]
pub enum UnspecifiedDigitDescriptor {
    /// Concrete decimal digit.
    #[display("{_0}")]
    Digit(u8),
    /// Uppercase `X` placeholder for an unspecified digit.
    #[display("X")]
    Unspecified,
}

/// Neutral digit-vector descriptor for ISO 8601-2 masked numeric components.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct MaskedNumericComponentDescriptor {
    /// Component digits in lexical order, preserving any `X` placeholders.
    #[builder(default)]
    pub digits: Vec<UnspecifiedDigitDescriptor>,
}

/// Declared public-profile masking regime for unspecified-digit expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display)]
pub enum UnspecifiedPrecisionProfileDescriptor {
    /// Level 1 rightmost-only masking.
    #[display("level-1-rightmost")]
    LevelOneRightmost,
    /// Level 2 masking that may appear anywhere within a component.
    #[display("level-2-any-position")]
    LevelTwoAnyPosition,
}

/// Neutral shape vocabulary for ISO 8601-2 unspecified-component expressions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum UnspecifiedTemporalShapeDescriptor {
    /// Masked calendar-date expression.
    CalendarDate {
        /// Year component digits.
        year: MaskedNumericComponentDescriptor,
        /// Optional month component digits.
        month: Option<MaskedNumericComponentDescriptor>,
        /// Optional day component digits.
        day: Option<MaskedNumericComponentDescriptor>,
    },
    /// Masked ordinal-date expression.
    OrdinalDate {
        /// Year component digits.
        year: MaskedNumericComponentDescriptor,
        /// Ordinal day digits.
        day_of_year: MaskedNumericComponentDescriptor,
    },
    /// Masked week-date expression.
    WeekDate {
        /// Week-based year digits.
        week_year: MaskedNumericComponentDescriptor,
        /// Week number digits.
        week: MaskedNumericComponentDescriptor,
        /// Optional weekday digits.
        weekday: Option<MaskedNumericComponentDescriptor>,
    },
}

/// Neutral ISO 8601-2 unspecified-component expression descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct UnspecifiedComponentExpressionDescriptor {
    /// Underlying masked temporal shape.
    pub shape: UnspecifiedTemporalShapeDescriptor,
    /// Declared masking regime.
    pub profile: UnspecifiedPrecisionProfileDescriptor,
    /// Optional uncertainty or approximation qualification metadata.
    #[builder(default)]
    pub qualification: Option<QualifiedTemporalExpressionDescriptor>,
}

/// Temporal value forms that may appear at interval boundaries.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum TemporalValueDescriptor {
    /// Calendar date form.
    CalendarDate(CalendarDateDescriptor),
    /// Reduced-precision calendar date form.
    ReducedCalendarDate(ReducedCalendarDateDescriptor),
    /// Gregorian calendar decade form.
    Decade(DecadeDescriptor),
    /// Gregorian calendar century form.
    Century(CenturyDescriptor),
    /// ISO 8601-2 extended year form.
    ExtendedYear(ExtendedYearDescriptor),
    /// Ordinal date form.
    OrdinalDate(OrdinalDateDescriptor),
    /// Week date form.
    WeekDate(WeekDateDescriptor),
    /// Date with time shift form.
    DateWithShift(DateWithShiftDescriptor),
    /// Time-of-day with time shift form.
    TimeOfDayWithShift(TimeOfDayWithShiftDescriptor),
    /// Local date-time form.
    LocalDateTime(LocalDateTimeDescriptor),
    /// Offset date-time form.
    OffsetDateTime(OffsetDateTimeDescriptor),
    /// Zoned date-time form.
    ZonedDateTime(ZonedDateTimeDescriptor),
    /// ISO 8601-2 Level 2 sub-year grouping expression.
    SubYearGrouping(SubYearGroupingExpressionDescriptor),
    /// Seasonal temporal expression.
    Seasonal(SeasonalTemporalExpressionDescriptor),
    /// Unspecified-component temporal expression.
    Unspecified(UnspecifiedComponentExpressionDescriptor),
}

/// Top-level temporal value plus explicit ISO 8601-2 qualification sidecar.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct QualifiedTemporalValueDescriptor {
    /// Underlying temporal value.
    pub value: TemporalValueDescriptor,
    /// Explicit qualification metadata carried alongside the value.
    pub qualification: QualifiedTemporalExpressionDescriptor,
}

/// Temporal value carrier used where the standards permit either bare or explicitly qualified values.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum QualifiedOrBareTemporalValueDescriptor {
    /// A bare temporal value with no explicit qualification sidecar.
    Bare(TemporalValueDescriptor),
    /// A temporal value carrying an explicit qualification sidecar.
    Qualified(QualifiedTemporalValueDescriptor),
}

/// Explicit temporal values governed by CalConnect explicit-form rules.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum ExplicitTemporalValueDescriptor {
    /// Explicit calendar date.
    CalendarDate(CalendarDateDescriptor),
    /// Explicit ordinal date.
    OrdinalDate(OrdinalDateDescriptor),
    /// Explicit week date.
    WeekDate(WeekDateDescriptor),
    /// Explicit Gregorian calendar decade.
    Decade(DecadeDescriptor),
    /// Explicit Gregorian calendar century.
    Century(CenturyDescriptor),
    /// Explicit duration representation.
    Duration(ExplicitDurationDescriptor),
    /// Explicit local time of day.
    TimeOfDay(ExplicitTimeOfDayDescriptor),
    /// Explicit time shift.
    TimeShift(ExplicitTimeShiftDescriptor),
    /// Explicit local date and time.
    DateTime(ExplicitDateTimeDescriptor),
    /// Explicit local date and time with time shift.
    DateTimeWithShift(ExplicitDateTimeWithShiftDescriptor),
    /// Explicit date with time shift.
    DateWithShift(DateWithShiftDescriptor),
    /// Explicit time of day with time shift.
    TimeOfDayWithShift(TimeOfDayWithShiftDescriptor),
}

/// Explicit temporal form metadata preserved across CalConnect exchanges.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct ExplicitTemporalFormDescriptor {
    /// The explicit temporal value carried by the representation.
    pub value: ExplicitTemporalValueDescriptor,
    /// The lowest denoted component, which declares explicit precision.
    pub precision: TimeScaleUnitDescriptor,
    /// Zero-valued components intentionally omitted from the lexical form.
    #[builder(default)]
    pub omitted_zero_components: Vec<TimeScaleUnitDescriptor>,
}

/// One valued time scale unit within grouped-unit or recurrence semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct TimeScaleUnitValueDescriptor {
    /// The unit family being counted.
    pub unit: TimeScaleUnitDescriptor,
    /// Positive coefficient attached to that unit.
    pub value: u32,
}

/// Grouped time scale unit expression including its coefficient and trailing lower-order units.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct GroupedTimeScaleUnitDescriptor {
    /// Coefficient attached to the grouped unit value.
    pub coefficient: u32,
    /// One or more units inside the grouped `G...U` definition.
    #[builder(default)]
    pub grouped_units: Vec<TimeScaleUnitValueDescriptor>,
    /// Optional lower-order units that apply within the grouped unit.
    #[builder(default)]
    pub lower_order_units: Vec<TimeScaleUnitValueDescriptor>,
}

/// Declared evaluation family for a date-time formula.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display)]
pub enum DateTimeFormulaEvaluationKindDescriptor {
    /// Simple-duration evaluation.
    #[display("simple")]
    Simple,
    /// Composite-duration evaluation.
    #[display("composite")]
    Composite,
    /// Precedence-duration evaluation.
    #[display("precedence")]
    Precedence,
}

/// Explicit temporal value plus duration under the CalConnect formula model.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct DateTimeFormulaDescriptor {
    /// The explicit date or time value being resolved.
    pub value: ExplicitTemporalValueDescriptor,
    /// The duration applied to that value.
    pub duration: ExplicitDurationDescriptor,
    /// Declared evaluation family for the formula.
    pub evaluation_kind: DateTimeFormulaEvaluationKindDescriptor,
}

/// One CalConnect selection-rule component.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum SelectionRuleDescriptor {
    /// Restrict by month numbers.
    Months(Vec<u8>),
    /// Restrict by week numbers, allowing negative indexing from the end.
    Weeks(Vec<i32>),
    /// Restrict by day-of-month values, allowing negative indexing from the end.
    DaysOfMonth(Vec<i32>),
    /// Restrict by ISO weekday numbers.
    Weekdays(Vec<u8>),
    /// Restrict by ordinal day-of-year values, allowing negative indexing.
    OrdinalDaysOfYear(Vec<i32>),
    /// Restrict by hour values.
    Hours(Vec<u8>),
    /// Restrict by minute values.
    Minutes(Vec<u8>),
    /// Restrict by second values.
    Seconds(Vec<u8>),
    /// Restrict by positional selection within the prior result set.
    Position(Vec<i32>),
    /// Extend a selection component by an explicit duration window.
    DurationWindow {
        /// The selection component being extended.
        rule: Box<SelectionRuleDescriptor>,
        /// Duration window applied to the selected component.
        duration: DurationDescriptor,
    },
}

/// Full CalConnect selection expression with optional single-instance semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct SelectionExpressionDescriptor {
    /// Selection rules in lexical order.
    #[builder(default)]
    pub rules: Vec<SelectionRuleDescriptor>,
    /// Whether the selection denotes a single instance via the `I` designator.
    #[builder(default)]
    pub selects_single_instance: bool,
}

/// Eligible time intervals used as the repeating cycle for repeat-rule evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct EligibleTimeIntervalsDescriptor {
    /// One or more eligible interval unit declarations.
    #[builder(default)]
    pub units: Vec<TimeScaleUnitValueDescriptor>,
}

/// Repeat-rule payload joining eligible intervals to a selection expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct RepeatRuleDescriptor {
    /// Repeating cycle that defines eligible intervals.
    pub eligible_intervals: EligibleTimeIntervalsDescriptor,
    /// Selection rules applied within each eligible interval.
    pub selection: SelectionExpressionDescriptor,
}

/// Interval family admitted by a complete recurring interval with repeat rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum RecurringIntervalWithRepeatRuleIntervalDescriptor {
    /// A complete ISO 8601 interval representation.
    IsoComplete(TimeIntervalDescriptor),
    /// A CalConnect explicit interval representation.
    Explicit(ExplicitTimeIntervalDescriptor),
}

/// Complete recurring-interval representation extended with a repeat rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct RecurringIntervalWithRepeatRuleDescriptor {
    /// Optional bounded repetition count; `None` denotes unbounded recurrence.
    #[builder(default)]
    pub repetitions: Option<u32>,
    /// Base complete interval being repeated.
    pub interval: RecurringIntervalWithRepeatRuleIntervalDescriptor,
    /// Repeat-rule refinement applied to the recurrence.
    pub repeat_rule: RepeatRuleDescriptor,
}

/// Membership semantics for an ISO 8601-2 temporal set expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display)]
pub enum TemporalSetSemanticsDescriptor {
    /// Square-bracket one-of semantics.
    #[display("alternatives")]
    Alternatives,
    /// Curly-brace inclusive all-members semantics.
    #[display("inclusive")]
    Inclusive,
}

/// One member of an ISO 8601-2 temporal set expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum TemporalSetMemberDescriptor {
    /// A single member expression.
    Value(QualifiedOrBareTemporalValueDescriptor),
    /// A bounded or open-ended range member.
    Range {
        /// Inclusive range start when present.
        start: Option<QualifiedOrBareTemporalValueDescriptor>,
        /// Inclusive range end when present.
        end: Option<QualifiedOrBareTemporalValueDescriptor>,
    },
}

/// Neutral ISO 8601-2 temporal set descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct TemporalSetDescriptor {
    /// Whether the set denotes alternatives or inclusive membership.
    pub semantics: TemporalSetSemanticsDescriptor,
    /// Member expressions and range members in source order.
    #[builder(default)]
    pub members: Vec<TemporalSetMemberDescriptor>,
    /// Optional uncertainty or approximation qualification metadata.
    #[builder(default)]
    pub qualification: Option<QualifiedTemporalExpressionDescriptor>,
}

/// RFC 9557 time-zone annotation payload carried in an IXDTF suffix.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum IxdtfTimeZoneAnnotationDescriptor {
    /// Named IANA time-zone identifier.
    Named(NamedTimeZoneDescriptor),
    /// Offset time-zone annotation used for compatibility.
    Offset(UtcOffsetDescriptor),
}

/// Interval boundary representation including explicit open or unknown cases.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum TimeIntervalEndpoint {
    /// A concrete temporal value.
    Value(QualifiedOrBareTemporalValueDescriptor),
    /// Explicit open boundary.
    Open,
    /// Explicit unknown boundary.
    Unknown,
}

/// Top-level interval representation form.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum TimeIntervalRepresentation {
    /// Concrete start and end boundaries.
    StartEnd {
        /// Interval start boundary.
        start: TimeIntervalEndpoint,
        /// Interval end boundary.
        end: TimeIntervalEndpoint,
    },
    /// Concrete start boundary and a duration.
    StartDuration {
        /// Interval start boundary.
        start: TimeIntervalEndpoint,
        /// Interval duration.
        duration: DurationDescriptor,
    },
    /// Duration followed by a concrete end boundary.
    DurationEnd {
        /// Interval duration.
        duration: DurationDescriptor,
        /// Interval end boundary.
        end: TimeIntervalEndpoint,
    },
}

/// Neutral ISO 8601 interval descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct TimeIntervalDescriptor {
    /// One of the legal ISO 8601 interval top-level forms.
    pub representation: TimeIntervalRepresentation,
}

/// Neutral ISO 8601 recurring interval descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct RecurringIntervalDescriptor {
    /// Optional bounded repetition count; `None` denotes unbounded recurrence.
    #[builder(default)]
    pub repetitions: Option<u32>,
    /// Repeated interval payload.
    pub interval: TimeIntervalDescriptor,
}

/// Declared rounding mode for precision-reducing conversions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display)]
pub enum RoundingModeDescriptor {
    /// Truncate the discarded digits.
    #[display("truncate")]
    Truncate,
    /// Round to nearest, ties away from zero.
    #[display("half-up")]
    HalfUp,
    /// Round to nearest, ties to even.
    #[display("half-even")]
    HalfEven,
    /// Round toward positive infinity.
    #[display("ceiling")]
    Ceiling,
    /// Round toward negative infinity.
    #[display("floor")]
    Floor,
    /// Another implementation-defined rounding mode.
    #[display("other({})", _0)]
    Other(String),
}

/// Declared precision target for temporal conversions or serializations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct PrecisionDescriptor {
    /// Smallest retained temporal component.
    pub smallest_component: TemporalComponent,
    /// Fractional-digit precision within the smallest component when applicable.
    #[builder(default)]
    pub fractional_digits: Option<u8>,
    /// Declared rounding mode for precision reduction when present.
    #[builder(default)]
    pub rounding_mode: Option<RoundingModeDescriptor>,
}

/// Named serialization profiles exposed by the branch-level temporal accord.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display)]
pub enum SerializationProfile {
    /// ISO 8601 basic form without separators.
    #[display("ISO 8601 basic")]
    Iso8601Basic,
    /// ISO 8601 extended form with separators.
    #[display("ISO 8601 extended")]
    Iso8601Extended,
    /// RFC 3339 Internet timestamp profile.
    #[display("RFC 3339")]
    Rfc3339,
    /// RFC 9557 IXDTF profile.
    #[display("RFC 9557 IXDTF")]
    Ixdtf,
}

/// Neutral serialization descriptor for temporal emitters.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct TemporalSerializationDescriptor {
    /// Governing serialization profile.
    pub profile: SerializationProfile,
    /// Whether the representation uses basic rather than extended separators.
    #[builder(default)]
    pub uses_basic_form: bool,
    /// Whether the representation uses uppercase `T` between date and time.
    #[builder(default)]
    pub uppercase_time_designator: bool,
    /// Whether the representation uses uppercase `Z` for UTC.
    #[builder(default)]
    pub uppercase_utc_designator: bool,
    /// Fractional-second digit count when a fractional second is present.
    #[builder(default)]
    pub fractional_second_digits: Option<u8>,
    /// RFC 9557 time-zone annotation when the profile preserves one.
    #[builder(default)]
    pub time_zone_annotation: Option<IxdtfTimeZoneAnnotationDescriptor>,
    /// Preferred presentation calendar annotation when the profile preserves one.
    #[builder(default)]
    pub calendar_annotation: Option<IxdtfCalendarAnnotationDescriptor>,
    /// Additional IXDTF annotations carried by the serialization.
    #[builder(default)]
    pub additional_annotations: Vec<IxdtfAnnotationDescriptor>,
}

/// Neutral RFC 9557 IXDTF timestamp descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct IxdtfTimestampDescriptor {
    /// Base RFC 3339 timestamp payload.
    pub timestamp: OffsetDateTimeDescriptor,
    /// Optional RFC 9557 time-zone annotation.
    #[builder(default)]
    pub time_zone_annotation: Option<IxdtfTimeZoneAnnotationDescriptor>,
    /// Additional IXDTF suffix annotations in source order.
    #[builder(default)]
    pub additional_annotations: Vec<IxdtfAnnotationDescriptor>,
}

/// Explicit RFC 9557 time-zone annotation proof branch carried by IXDTF exchanges.
pub enum IxdtfTimeZoneAnnotationProofBranch {
    /// No RFC 9557 time-zone annotation is present.
    None,
    /// A named-zone annotation is present and carries civil-rule identity.
    Named {
        /// Aggregate proof that the timestamp carries named-zone identity.
        zoned: Established<ZonedDateTimeHasNamedZone>,
        /// Evidence bundle for the named-zone branch.
        evidence: ZonedTimestampEvidence,
    },
    /// An offset time-zone annotation is present and follows compatibility semantics.
    Offset {
        /// Aggregate proof that the offset annotation is consistent with the timestamp.
        semantics: Established<OffsetTimeZoneAnnotationConsistentWithTimestamp>,
        /// Evidence bundle for the offset-annotation branch.
        evidence: OffsetTimeZoneAnnotationEvidence,
    },
}

/// Explicit proof branch for local-to-zone resolution across transition edge cases.
pub enum LocalTimeZoneResolutionProofBranch {
    /// The local wall-clock time mapped to a single instant without transition special handling.
    Unambiguous,
    /// The local wall-clock time was ambiguous and required explicit disambiguation authority.
    Ambiguous {
        /// The local timestamp can be ambiguous at a zone transition.
        possibility: Established<LocalDateTimeMayBeAmbiguousAtZoneTransition>,
        /// Aggregate proof that ambiguity semantics were handled explicitly and lawfully.
        semantics: Established<ZoneTransitionAmbiguitySemanticsValid>,
        /// Evidence bundle for the ambiguity-resolution branch.
        evidence: ZoneTransitionAmbiguityEvidence,
    },
    /// The local wall-clock time fell inside a skipped transition gap.
    Gap {
        /// The local timestamp can fall inside a skipped zone-transition gap.
        possibility: Established<LocalDateTimeMayFallInZoneTransitionGap>,
        /// Aggregate proof that gap semantics were handled explicitly and lawfully.
        semantics: Established<ZoneTransitionGapSemanticsValid>,
        /// Evidence bundle for the gap-resolution branch.
        evidence: ZoneTransitionGapEvidence,
    },
}

/// Explicit RFC 9557 preferred-calendar proof branch carried by IXDTF exchanges.
pub enum IxdtfCalendarAnnotationProofBranch {
    /// No preferred-presentation calendar annotation is present.
    None,
    /// A preferred-presentation calendar annotation is present.
    Present {
        /// Aggregate proof that the timestamp declares a preferred presentation calendar.
        preferred_calendar: Established<IxdtfTimestampHasPreferredPresentationCalendar>,
        /// Evidence bundle for the calendar-awareness branch.
        evidence: IxdtfCalendarAwareTimestampEvidence,
    },
}

/// Explicit RFC 9557 additional-information proof branch carried by IXDTF exchanges.
pub enum IxdtfAdditionalInformationProofBranch {
    /// No additional-information annotations are present.
    None,
    /// Additional-information annotations are present and semantically validated.
    Present {
        /// Aggregate proof that the additional-information semantics are valid.
        semantics: Established<IxdtfAdditionalInformationSemanticsValid>,
        /// Evidence bundle for the additional-information branch.
        evidence: IxdtfAdditionalInformationEvidence,
    },
}

/// Explicit proof branch for inherited end-component interval semantics.
pub enum IntervalEndComponentInheritanceProofBranch {
    /// The interval does not rely on inherited higher-order end components.
    None,
    /// The trailing interval component inherits omitted higher-order components.
    Inherited {
        /// Aggregate proof that inherited end-component semantics are explicit and lawful.
        semantics: Established<InheritedIntervalEndComponentsSemanticsValid>,
        /// Evidence bundle for the inherited end-component branch.
        evidence: InheritedIntervalEndComponentsEvidence,
    },
}

/// Explicit proof branch for inherited trailing-zone interval semantics.
pub enum IntervalZoneInheritanceProofBranch {
    /// The interval does not rely on inherited trailing-zone semantics.
    None,
    /// The trailing interval component inherits omitted zone or UTC semantics.
    Inherited {
        /// Aggregate proof that inherited trailing-zone semantics are explicit and lawful.
        semantics: Established<InheritedIntervalZoneSemanticsValid>,
        /// Evidence bundle for the inherited trailing-zone branch.
        evidence: InheritedIntervalZoneEvidence,
    },
}

/// Explicit proof branch for CalConnect explicit-interval duration substitution semantics.
pub enum ExplicitIntervalDurationSubstitutionProofBranch {
    /// The explicit interval uses concrete start/end boundaries, so duration substitution does not apply.
    NotApplicable,
    /// The explicit interval uses a start boundary plus an explicit duration.
    StartDuration {
        /// Aggregate proof that duration-substitution semantics are explicit and lawful.
        semantics: Established<ExplicitIntervalDurationSubstitutionSemanticsValid>,
        /// Evidence bundle for the start/duration substitution branch.
        evidence: ExplicitIntervalDurationSubstitutionEvidence,
    },
    /// The explicit interval uses an explicit duration plus an end boundary.
    DurationEnd {
        /// Aggregate proof that duration-substitution semantics are explicit and lawful.
        semantics: Established<ExplicitIntervalDurationSubstitutionSemanticsValid>,
        /// Evidence bundle for the duration/end substitution branch.
        evidence: ExplicitIntervalDurationSubstitutionEvidence,
    },
}

/// Explicit proof branch for CalConnect explicit-interval trailing-end inheritance semantics.
pub enum ExplicitIntervalEndComponentInheritanceProofBranch {
    /// The explicit interval does not rely on trailing-end higher-order inheritance.
    None,
    /// The trailing explicit endpoint inherits omitted higher-order components from the start.
    Inherited {
        /// Aggregate proof that trailing-end inheritance semantics are explicit and lawful.
        semantics: Established<ExplicitIntervalEndComponentInheritanceSemanticsValid>,
        /// Evidence bundle for the trailing-end inheritance branch.
        evidence: ExplicitIntervalEndComponentInheritanceEvidence,
    },
}

/// Explicit proof branch for CalConnect explicit-interval leading-shift propagation semantics.
pub enum ExplicitIntervalShiftPropagationProofBranch {
    /// The explicit interval does not rely on leading-shift propagation.
    None,
    /// The leading explicit time shift applies to the trailing endpoint absent an explicit override.
    Propagated {
        /// Aggregate proof that leading-shift propagation semantics are explicit and lawful.
        semantics: Established<ExplicitIntervalShiftPropagationSemanticsValid>,
        /// Evidence bundle for the leading-shift propagation branch.
        evidence: ExplicitIntervalShiftPropagationEvidence,
    },
}

/// Explicit proof branch for duration representation family semantics.
pub enum DurationRepresentationProofBranch {
    /// The duration uses the designator-based representation family.
    Designator {
        /// Aggregate proof that duration representation-family semantics are explicit and lawful.
        semantics: Established<DurationRepresentationSemanticsValid>,
        /// Evidence bundle for the designator-based duration branch.
        evidence: DurationDesignatorRepresentationEvidence,
    },
    /// The duration uses the alternative complete representation family.
    Alternative {
        /// Aggregate proof that duration representation-family semantics are explicit and lawful.
        semantics: Established<DurationRepresentationSemanticsValid>,
        /// Evidence bundle for the alternative duration branch.
        evidence: DurationAlternativeFormEvidence,
    },
}

/// Explicit proof branch for complete-interval substitution semantics.
pub enum CompleteIntervalSubstitutionProofBranch {
    /// The interval is not a complete `4.4.4` representation, so `4.4.4.5` substitutions do not apply.
    NotApplicable,
    /// The interval is a complete start/end representation with explicit time-point substitution evidence.
    StartEnd {
        /// Aggregate proof that complete-interval substitution semantics are explicit and lawful.
        semantics: Established<CompleteIntervalSubstitutionSemanticsValid>,
        /// Evidence bundle for the complete start/end branch.
        evidence: CompleteStartEndIntervalSubstitutionEvidence,
    },
    /// The interval is a complete start/duration representation with explicit substitution evidence.
    StartDuration {
        /// Aggregate proof that complete-interval substitution semantics are explicit and lawful.
        semantics: Established<CompleteIntervalSubstitutionSemanticsValid>,
        /// Evidence bundle for the complete start/duration branch.
        evidence: CompleteStartDurationIntervalSubstitutionEvidence,
    },
    /// The interval is a complete duration/end representation with explicit substitution evidence.
    DurationEnd {
        /// Aggregate proof that complete-interval substitution semantics are explicit and lawful.
        semantics: Established<CompleteIntervalSubstitutionSemanticsValid>,
        /// Evidence bundle for the complete duration/end branch.
        evidence: CompleteDurationEndIntervalSubstitutionEvidence,
    },
}

/// Explicit proof branch for recurring-interval representation family semantics.
pub enum RecurringIntervalRepresentationProofBranch {
    /// The recurring interval embeds a complete time-interval representation.
    Complete {
        /// Aggregate proof that complete recurring-interval semantics are explicit and lawful.
        semantics: Established<CompleteRecurringIntervalRepresentationSemanticsValid>,
        /// Evidence bundle for the complete recurring-interval branch.
        evidence: CompleteRecurringIntervalRepresentationEvidence,
    },
    /// The recurring interval embeds an other-than-complete time-interval representation.
    OtherThanComplete {
        /// Aggregate proof that other-than-complete recurring-interval semantics are explicit and lawful.
        semantics: Established<OtherThanCompleteRecurringIntervalRepresentationSemanticsValid>,
        /// Evidence bundle for the other-than-complete recurring-interval branch.
        evidence: OtherThanCompleteRecurringIntervalRepresentationEvidence,
    },
}

/// Explicit proof branch for the embedded interval family of a recurring interval with repeat rule.
pub enum RecurringIntervalWithRepeatRuleIntervalProofBranch {
    /// The recurring representation embeds a complete ISO 8601 interval.
    IsoComplete {
        /// The embedded ISO interval is structurally valid.
        interval: Established<TimeIntervalValid>,
        /// The embedded complete ISO interval carries explicit substitution-law sidecars.
        substitution: CompleteIntervalSubstitutionProofBranch,
    },
    /// The recurring representation embeds a CalConnect explicit interval.
    Explicit {
        /// The embedded CalConnect explicit interval is structurally valid.
        interval: Established<ExplicitTimeIntervalValid>,
        /// The embedded explicit interval carries duration-substitution semantics.
        duration_substitution: ExplicitIntervalDurationSubstitutionProofBranch,
        /// The embedded explicit interval carries trailing-end inheritance semantics.
        end_component_inheritance: ExplicitIntervalEndComponentInheritanceProofBranch,
        /// The embedded explicit interval carries leading-shift propagation semantics.
        shift_propagation: ExplicitIntervalShiftPropagationProofBranch,
    },
}

/// Aggregate semantic bundle for one backend conversion exchange.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct BackendConversionSemanticBundle {
    /// Aggregate proof that the backend conversion semantics are valid.
    pub semantics: Established<BackendConversionSemanticsValid>,
    /// Evidence bundle backing the aggregate backend-conversion proof.
    pub evidence: BackendConversionEvidence,
}

/// Aggregate semantic bundle for one local date-time carrier.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct LocalDateTimeSemanticBundle {
    /// Aggregate proof that the local date-time descriptor is structurally valid.
    pub validity: Established<LocalDateTimeValid>,
    /// Evidence bundle for the valid local date-time structure.
    pub validity_evidence: LocalDateTimeEvidence,
    /// Aggregate proof that the local date-time remains civil and does not yet identify a fixed instant.
    pub local_semantics: Established<LocalDateTimeDoesNotIdentifyFixedInstant>,
    /// Evidence bundle for the local non-fixed-instant semantics.
    pub local_semantics_evidence: LocalTimestampSemanticsEvidence,
    /// Shared backend-conversion semantics for the descriptor/native exchange.
    pub backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one fixed-instant offset date-time carrier.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct OffsetDateTimeSemanticBundle {
    /// Aggregate proof that the offset date-time descriptor is structurally valid.
    pub validity: Established<OffsetDateTimeValid>,
    /// Evidence bundle for the valid offset date-time structure.
    pub validity_evidence: OffsetDateTimeEvidence,
    /// Aggregate proof that the timestamp identifies one fixed instant.
    pub fixed_instant: Established<TimestampRepresentsFixedInstant>,
    /// Evidence bundle for the fixed-instant semantics.
    pub fixed_instant_evidence: FixedInstantEvidence,
    /// Shared backend-conversion semantics for the descriptor/native exchange.
    pub backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one named time-zone carrier.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct NamedTimeZoneSemanticBundle {
    /// Aggregate proof that the named zone carries lawful IANA identity semantics.
    pub identity: Established<NamedTimeZoneIdentityValid>,
    /// Evidence bundle for the named-zone identity semantics.
    pub identity_evidence: NamedTimeZoneIdentityEvidence,
    /// Shared backend-conversion semantics for the descriptor/native exchange.
    pub backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one named-zone-attached fixed-instant carrier.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct ZonedDateTimeSemanticBundle {
    /// Aggregate proof that the timestamp identifies one fixed instant.
    pub fixed_instant: Established<TimestampRepresentsFixedInstant>,
    /// Evidence bundle for the fixed-instant semantics.
    pub fixed_instant_evidence: FixedInstantEvidence,
    /// Aggregate proof that the attached zone carries lawful IANA identity semantics.
    pub zone_identity: Established<NamedTimeZoneIdentityValid>,
    /// Evidence bundle for the named-zone identity semantics.
    pub zone_identity_evidence: NamedTimeZoneIdentityEvidence,
    /// Aggregate proof that the timestamp carries named-zone attachment semantics.
    pub zone_attachment: Established<ZonedDateTimeHasNamedZone>,
    /// Evidence bundle for the named-zone attachment semantics.
    pub zone_attachment_evidence: NamedZoneAttachmentEvidence,
    /// Aggregate proof that the carried offset is consistent with the named-zone rules for the instant.
    pub offset_consistency: Established<OffsetConsistentWithNamedZone>,
    /// Evidence bundle for the offset-versus-zone consistency semantics.
    pub offset_consistency_evidence: OffsetConsistencyEvidence,
    /// Shared backend-conversion semantics for the descriptor/native exchange.
    pub backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one duration carrier.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct DurationSemanticBundle {
    /// Aggregate proof that the duration form is structurally valid.
    pub validity: Established<DurationFormValid>,
    /// Evidence bundle for the valid duration form.
    pub validity_evidence: DurationFormEvidence,
    /// Explicit proof branch for the chosen duration representation family.
    pub representation: DurationRepresentationProofBranch,
    /// Shared backend-conversion semantics for the descriptor/native exchange.
    pub backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one time-interval carrier.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct TimeIntervalSemanticBundle {
    /// Aggregate proof that the interval form is structurally valid.
    pub validity: Established<TimeIntervalValid>,
    /// Evidence bundle for the valid interval form.
    pub validity_evidence: TimeIntervalEvidence,
    /// Aggregate proof that any enhanced interval-boundary semantics are explicit and lawful.
    pub extended_boundaries: Established<ExtendedIntervalBoundarySemanticsValid>,
    /// Explicit proof branch for inherited higher-order trailing-end components.
    pub end_component_inheritance: IntervalEndComponentInheritanceProofBranch,
    /// Explicit proof branch for inherited trailing-zone semantics.
    pub zone_inheritance: IntervalZoneInheritanceProofBranch,
    /// Explicit proof branch for complete-interval substitution semantics.
    pub substitution: CompleteIntervalSubstitutionProofBranch,
    /// Shared backend-conversion semantics for the descriptor/native exchange.
    pub backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one recurring-interval carrier.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct RecurringIntervalSemanticBundle {
    /// Aggregate proof that the recurring-interval wrapper is structurally valid.
    pub validity: Established<RecurringIntervalFormValid>,
    /// Evidence bundle for the valid recurring-interval wrapper.
    pub validity_evidence: RecurringIntervalEvidence,
    /// Explicit proof branch for the embedded interval representation family.
    pub representation: RecurringIntervalRepresentationProofBranch,
    /// Shared backend-conversion semantics for the descriptor/native exchange.
    pub backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one qualified temporal value carrier.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct QualifiedTemporalValueSemanticBundle {
    /// Aggregate proof that the qualified temporal value is structurally valid.
    pub validity: Established<QualifiedTemporalValueValid>,
    /// Evidence bundle for the valid qualified temporal value structure.
    pub validity_evidence: QualifiedTemporalValueEvidence,
    /// Aggregate proof that the attached qualification expression is lawful.
    pub qualification: Established<QualifiedTemporalExpressionValid>,
    /// Evidence bundle for the qualification-expression semantics.
    pub qualification_evidence: QualifiedTemporalExpressionEvidence,
    /// Explicit proof branch for qualification placement semantics.
    pub placement: QualificationPlacementEvidence,
    /// Shared backend-conversion semantics for the descriptor/native exchange.
    pub backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one explicit temporal form carrier.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct ExplicitTemporalFormSemanticBundle {
    /// Aggregate proof that the explicit temporal form is structurally valid.
    pub validity: Established<ExplicitTemporalFormValid>,
    /// Evidence bundle for the valid explicit temporal form structure.
    pub validity_evidence: ExplicitTemporalFormEvidence,
    /// Aggregate proof that the explicit form uses designator symbols lawfully.
    pub designators: Established<ExplicitTemporalFormUsesDesignatorSymbols>,
    /// Aggregate proof that omitted zero-valued components remain explicitly lawful.
    pub zero_omission: Established<ExplicitTemporalFormMayOmitZeroValuedComponents>,
    /// Aggregate proof that the declared precision uses the lowest denoted component.
    pub precision: Established<ExplicitTemporalPrecisionUsesLowestDenotedComponent>,
    /// Aggregate proof that UTC relationship syntax uses Zulu or signed shift semantics.
    pub utc_relationship: Established<ExplicitUtcRelationshipUsesZuluOrSignedShift>,
    /// Shared backend-conversion semantics for the descriptor/native exchange.
    pub backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one explicit duration carrier.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct ExplicitDurationSemanticBundle {
    /// Aggregate proof that the explicit duration is structurally valid.
    pub validity: Established<ExplicitDurationValid>,
    /// Evidence bundle for the valid explicit-duration structure.
    pub validity_evidence: ExplicitDurationEvidence,
    /// Aggregate proof that explicit durational unit designators are used lawfully.
    pub units: Established<ExplicitDurationUsesDurationalUnitDesignators>,
    /// Explicit proof branch for the declared explicit-duration representation family.
    pub representation: ExplicitDurationRepresentationEvidence,
    /// Aggregate proof that any negative sign usage is lawful.
    pub sign: Established<ExplicitDurationMayBeNegative>,
    /// Aggregate proof that any fractional lowest-order unit usage is lawful.
    pub fractional: Established<ExplicitDurationMayUseFractionalLowestOrderUnit>,
    /// Explicit proof branch for the declared exactness or context semantics family.
    pub semantics: ExplicitDurationSemanticEvidence,
    /// Shared backend-conversion semantics for the descriptor/native exchange.
    pub backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one explicit time-interval carrier.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct ExplicitTimeIntervalSemanticBundle {
    /// Aggregate proof that the explicit time interval is structurally valid.
    pub validity: Established<ExplicitTimeIntervalValid>,
    /// Evidence bundle for the valid explicit time-interval structure.
    pub validity_evidence: ExplicitTimeIntervalEvidence,
    /// Explicit proof branch for duration-substitution semantics.
    pub duration_substitution: ExplicitIntervalDurationSubstitutionProofBranch,
    /// Explicit proof branch for trailing-end component inheritance semantics.
    pub end_component_inheritance: ExplicitIntervalEndComponentInheritanceProofBranch,
    /// Explicit proof branch for leading-shift propagation semantics.
    pub shift_propagation: ExplicitIntervalShiftPropagationProofBranch,
    /// Shared backend-conversion semantics for the descriptor/native exchange.
    pub backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one grouped time-scale-unit carrier.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct GroupedTimeScaleUnitSemanticBundle {
    /// Aggregate proof that the grouped time-scale-unit expression is structurally valid.
    pub validity: Established<GroupedTimeScaleUnitValid>,
    /// Evidence bundle for the valid grouped-unit structure.
    pub validity_evidence: GroupedTimeScaleUnitEvidence,
    /// Aggregate proof that grouping designators are used lawfully.
    pub designators: Established<GroupedTimeScaleUnitUsesGroupingDesignators>,
    /// Aggregate proof that one or more duration units are carried lawfully.
    pub units: Established<GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits>,
    /// Aggregate proof that the grouped-unit definition is continuous.
    pub continuity: Established<GroupedTimeScaleUnitDefinitionIsContinuous>,
    /// Aggregate proof that the grouped value carries an explicit coefficient.
    pub coefficient: Established<GroupedTimeScaleUnitValueCarriesExplicitCoefficient>,
    /// Aggregate proof that lower-order units remain within lawful group bounds.
    pub bounds: Established<GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds>,
    /// Aggregate proof that explicit time-shift carriage is lawful when present.
    pub explicit_time_shift: Established<GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift>,
    /// Aggregate proof that out-of-bounds remainder truncation semantics are explicit.
    pub truncation: Established<GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder>,
    /// Aggregate proof that the grouped unit converts to an interval lawfully.
    pub interval_semantics: Established<GroupedTimeScaleUnitConvertsToTimeInterval>,
    /// Shared backend-conversion semantics for the descriptor/native exchange.
    pub backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one temporal set carrier.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct TemporalSetSemanticBundle {
    /// Aggregate proof that the temporal set expression is structurally valid.
    pub expression: Established<TemporalSetExpressionValid>,
    /// Evidence bundle for the valid temporal-set expression structure.
    pub expression_evidence: TemporalSetExpressionEvidence,
    /// Aggregate proof that the temporal set carries lawful range semantics.
    pub range_semantics: Established<TemporalSetRangeSemanticsValid>,
    /// Evidence bundle for the temporal-set range semantics.
    pub range_semantics_evidence: TemporalSetRangeSemanticsEvidence,
    /// Shared backend-conversion semantics for the descriptor/native exchange.
    pub backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one date-time formula carrier.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct DateTimeFormulaSemanticBundle {
    /// Aggregate proof that the date-time formula is structurally valid.
    pub validity: Established<DateTimeFormulaValid>,
    /// Evidence bundle for the valid date-time-formula structure.
    pub validity_evidence: DateTimeFormulaEvidence,
    /// Aggregate proof that the declared evaluation semantics are lawful.
    pub evaluation_semantics: Established<DateTimeFormulaEvaluationSemanticsValid>,
    /// Evidence bundle for the declared formula-evaluation semantics.
    pub evaluation_semantics_evidence: DateTimeFormulaEvaluationSemanticsEvidence,
    /// Shared backend-conversion semantics for the descriptor/native exchange.
    pub backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one date-time-formula evaluation result.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct DateTimeFormulaEvaluationResultBundle {
    /// Aggregate proof that the explicit temporal form was lawfully produced by formula evaluation.
    pub semantics: Established<DateTimeFormulaEvaluationResultValid>,
    /// Evidence bundle for the formula-evaluation result provenance.
    pub evidence: DateTimeFormulaEvaluationResultEvidence,
}

/// Aggregate semantic bundle for explicit local-to-zone resolution authority.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct ZoneTransitionResolutionAuthorityBundle {
    /// Aggregate proof that the carried local-to-zone resolution authority is lawful.
    pub semantics: Established<ZoneTransitionResolutionAuthorityValid>,
    /// Evidence bundle describing the explicit resolution authority.
    pub evidence: ZoneTransitionResolutionAuthorityEvidence,
}

/// Aggregate semantic bundle for explicit lossy-conversion authority.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct LossyConversionAuthorityBundle {
    /// Aggregate proof that lossy conversion was lawfully authorized.
    pub semantics: Established<LossyConversionAuthorityValid>,
    /// Evidence bundle describing the lossiness authority and rounding regime.
    pub evidence: LossyConversionAuthorityEvidence,
}

/// Aggregate semantic bundle for one lossless conversion result.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct LosslessConversionBundle {
    /// Aggregate proof that the conversion was lossless.
    pub semantics: Established<ConversionLossless>,
    /// Evidence bundle for the lossless-conversion semantics.
    pub evidence: LosslessConversionEvidence,
}

/// Aggregate semantic bundle for one subsecond truncation result.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct SubsecondTruncationBundle {
    /// Aggregate proof that subsecond truncation occurred.
    pub semantics: Established<ConversionTruncatesSubseconds>,
    /// Evidence bundle for the truncation semantics.
    pub evidence: SubsecondTruncationEvidence,
}

/// Aggregate semantic bundle for one named-zone revision interpretation result.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct NamedTimeZoneRevisionBundle {
    /// Aggregate proof that named-zone interpretation tracks the applicable TZDB revision.
    pub semantics: Established<NamedTimeZoneInterpretationTracksTzdbRevision>,
    /// Evidence bundle for the revision-sensitive named-zone semantics.
    pub evidence: NamedTimeZoneRevisionEvidence,
}

/// Aggregate semantic bundle for one ordered fixed-instant interval result.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct IntervalEndpointOrderingBundle {
    /// Aggregate proof that the endpoints are ordered lawfully.
    pub semantics: Established<IntervalEndpointsOrdered>,
    /// Evidence bundle for the endpoint-ordering semantics.
    pub evidence: IntervalEndpointOrderingEvidence,
}

/// Generic user-facing wrapper pairing a native backend carrier with its aggregate semantic bundle.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct ProvenTemporalCarrier<T, S> {
    /// The backend-owned runtime carrier.
    pub carrier: T,
    /// Aggregate semantics carried alongside the runtime value.
    pub semantics: S,
}

/// User-facing proven local date-time carrier.
pub type ProvenLocalDateTimeCarrier<T> = ProvenTemporalCarrier<T, LocalDateTimeSemanticBundle>;

/// User-facing proven fixed-instant offset date-time carrier.
pub type ProvenOffsetDateTimeCarrier<T> = ProvenTemporalCarrier<T, OffsetDateTimeSemanticBundle>;

/// User-facing proven named time-zone carrier.
pub type ProvenNamedTimeZoneCarrier<T> = ProvenTemporalCarrier<T, NamedTimeZoneSemanticBundle>;

/// User-facing proven named-zone-attached fixed-instant carrier.
pub type ProvenZonedDateTimeCarrier<T> = ProvenTemporalCarrier<T, ZonedDateTimeSemanticBundle>;

/// User-facing proven duration carrier.
pub type ProvenDurationCarrier<T> = ProvenTemporalCarrier<T, DurationSemanticBundle>;

/// User-facing proven time-interval carrier.
pub type ProvenTimeIntervalCarrier<T> = ProvenTemporalCarrier<T, TimeIntervalSemanticBundle>;

/// User-facing proven recurring-interval carrier.
pub type ProvenRecurringIntervalCarrier<T> =
    ProvenTemporalCarrier<T, RecurringIntervalSemanticBundle>;

/// User-facing proven qualified temporal value carrier.
pub type ProvenQualifiedTemporalValueCarrier<T> =
    ProvenTemporalCarrier<T, QualifiedTemporalValueSemanticBundle>;

/// User-facing proven explicit temporal form carrier.
pub type ProvenExplicitTemporalFormCarrier<T> =
    ProvenTemporalCarrier<T, ExplicitTemporalFormSemanticBundle>;

/// User-facing proven explicit duration carrier.
pub type ProvenExplicitDurationCarrier<T> =
    ProvenTemporalCarrier<T, ExplicitDurationSemanticBundle>;

/// User-facing proven explicit time-interval carrier.
pub type ProvenExplicitTimeIntervalCarrier<T> =
    ProvenTemporalCarrier<T, ExplicitTimeIntervalSemanticBundle>;

/// User-facing proven grouped time-scale-unit carrier.
pub type ProvenGroupedTimeScaleUnitCarrier<T> =
    ProvenTemporalCarrier<T, GroupedTimeScaleUnitSemanticBundle>;

/// User-facing proven temporal set carrier.
pub type ProvenTemporalSetCarrier<T> = ProvenTemporalCarrier<T, TemporalSetSemanticBundle>;

/// User-facing proven date-time formula carrier.
pub type ProvenDateTimeFormulaCarrier<T> =
    ProvenTemporalCarrier<T, DateTimeFormulaSemanticBundle>;

/// Result shape for parsing an RFC 3339 timestamp into a fixed-instant descriptor.
pub type ParsedRfc3339TimestampResult = TemporalResult<(
    OffsetDateTimeDescriptor,
    Established<OffsetDateTimeValid>,
    Established<Rfc3339TimestampValid>,
    Established<TimestampRepresentsFixedInstant>,
)>;

/// Result shape for parsing a generic ISO 8601 offset date-time.
pub type ParsedOffsetDateTimeResult = TemporalResult<(
    OffsetDateTimeDescriptor,
    Established<OffsetDateTimeValid>,
    Established<TimestampRepresentsFixedInstant>,
)>;

/// Result shape for parsing a complete explicit-form date with shift.
pub type ParsedDateWithShiftResult =
    TemporalResult<(DateWithShiftDescriptor, Established<DateWithShiftValid>)>;

/// Result shape for parsing a complete explicit-form time of day with shift.
pub type ParsedTimeOfDayWithShiftResult = TemporalResult<(
    TimeOfDayWithShiftDescriptor,
    Established<TimeOfDayWithShiftValid>,
)>;

/// Result shape for parsing a reduced-precision ISO 8601 calendar date.
pub type ParsedReducedCalendarDateResult = TemporalResult<(
    ReducedCalendarDateDescriptor,
    Established<ReducedCalendarDateValid>,
)>;

/// Result shape for parsing a reduced-accuracy ISO 8601 local time.
pub type ParsedReducedLocalTimeResult = TemporalResult<(
    ReducedLocalTimeDescriptor,
    Established<ReducedLocalTimeValid>,
)>;

/// Result shape for parsing an ISO 8601 decade representation.
pub type ParsedDecadeResult = TemporalResult<(DecadeDescriptor, Established<DecadeValid>)>;

/// Result shape for parsing an ISO 8601 century representation.
pub type ParsedCenturyResult = TemporalResult<(CenturyDescriptor, Established<CenturyValid>)>;

/// Result shape for parsing an ISO 8601-2 extended year form.
pub type ParsedExtendedYearResult =
    TemporalResult<(ExtendedYearDescriptor, Established<ExtendedYearValid>)>;

/// Result shape for parsing an ISO 8601 duration.
pub type ParsedDurationResult = TemporalResult<(
    DurationDescriptor,
    Established<DurationFormValid>,
    DurationRepresentationProofBranch,
)>;

/// Result shape for parsing a CalConnect explicit duration.
pub type ParsedExplicitDurationResult = TemporalResult<(
    ExplicitDurationDescriptor,
    Established<ExplicitDurationValid>,
    Established<ExplicitDurationUsesDurationalUnitDesignators>,
    ExplicitDurationRepresentationEvidence,
    Established<ExplicitDurationMayBeNegative>,
    Established<ExplicitDurationMayUseFractionalLowestOrderUnit>,
    ExplicitDurationSemanticEvidence,
)>;

/// Result shape for parsing a CalConnect explicit time interval.
pub type ParsedExplicitTimeIntervalResult = TemporalResult<(
    ExplicitTimeIntervalDescriptor,
    Established<ExplicitTimeIntervalValid>,
    ExplicitIntervalDurationSubstitutionProofBranch,
    ExplicitIntervalEndComponentInheritanceProofBranch,
    ExplicitIntervalShiftPropagationProofBranch,
)>;

/// Result shape for parsing an ISO 8601 recurring interval.
pub type ParsedRecurringIntervalResult = TemporalResult<(
    RecurringIntervalDescriptor,
    Established<RecurringIntervalFormValid>,
    RecurringIntervalRepresentationProofBranch,
)>;

/// Result shape for parsing a qualified temporal value.
pub type ParsedQualifiedTemporalValueResult = TemporalResult<(
    QualifiedTemporalValueDescriptor,
    Established<QualifiedTemporalValueValid>,
    Established<QualifiedTemporalExpressionValid>,
    QualificationPlacementEvidence,
)>;

/// Result shape for parsing an explicit temporal form.
pub type ParsedExplicitTemporalFormResult = TemporalResult<(
    ExplicitTemporalFormDescriptor,
    Established<ExplicitTemporalFormValid>,
    Established<ExplicitTemporalFormUsesDesignatorSymbols>,
    Established<ExplicitTemporalFormMayOmitZeroValuedComponents>,
    Established<ExplicitTemporalPrecisionUsesLowestDenotedComponent>,
    Established<ExplicitUtcRelationshipUsesZuluOrSignedShift>,
)>;

/// Result shape for parsing a grouped time scale unit expression.
pub type ParsedGroupedTimeScaleUnitResult = TemporalResult<(
    GroupedTimeScaleUnitDescriptor,
    Established<GroupedTimeScaleUnitValid>,
    Established<GroupedTimeScaleUnitUsesGroupingDesignators>,
    Established<GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits>,
    Established<GroupedTimeScaleUnitDefinitionIsContinuous>,
    Established<GroupedTimeScaleUnitValueCarriesExplicitCoefficient>,
    Established<GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds>,
    Established<GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift>,
    Established<GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder>,
    Established<GroupedTimeScaleUnitConvertsToTimeInterval>,
)>;

/// Result shape for parsing a date-time formula and its evaluation semantics.
pub type ParsedDateTimeFormulaResult = TemporalResult<(
    DateTimeFormulaDescriptor,
    Established<DateTimeFormulaValid>,
    Established<DateTimeFormulaEvaluationSemanticsValid>,
)>;

/// Result shape for evaluating a date-time formula into an explicit temporal value.
///
/// The returned provenance token distinguishes "valid explicit form" from
/// "valid explicit form lawfully produced by the declared formula-evaluation
/// semantics."
pub type EvaluatedDateTimeFormulaResult = TemporalResult<(
    ExplicitTemporalValueDescriptor,
    Established<ExplicitTemporalFormValid>,
    Established<DateTimeFormulaEvaluationResultValid>,
)>;

/// Result shape for parsing a selection expression.
pub type ParsedSelectionExpressionResult = TemporalResult<(
    SelectionExpressionDescriptor,
    Established<SelectionExpressionValid>,
    Established<SelectionExpressionUsesSelectionDelimiters>,
    Established<SelectionExpressionUsesRecognizedSelectionRuleVocabulary>,
    Established<SelectionRuleMonthUsesMonthExpression>,
    Established<SelectionRuleWeekUsesWeekExpression>,
    Established<SelectionRuleDayOfMonthUsesDayExpression>,
    Established<SelectionRuleWeekDayUsesDayOfWeekExpression>,
    Established<SelectionRuleOrdinalDayOfYearUsesOrdinalDayExpression>,
    Established<SelectionRuleHourUsesHourExpression>,
    Established<SelectionRuleMinuteUsesMinuteExpression>,
    Established<SelectionRuleSecondUsesSecondExpression>,
    Established<SelectionRulesApplyWithinSelectedResults>,
    Established<SelectionExpressionMaySelectSingleInstance>,
    Established<SelectionRulePositionUsesInstanceDesignatorSuffix>,
    Established<SelectionRulePositionAppliesLast>,
    Established<SelectionWithDurationUsesDurationSuffix>,
)>;

/// Result shape for parsing a repeat rule.
pub type ParsedRepeatRuleResult = TemporalResult<(
    RepeatRuleDescriptor,
    Established<RepeatRuleValid>,
    Established<RepeatRuleUsesFrequencyDesignator>,
    Established<RepeatRuleDeclaresEligibleTimeIntervals>,
    Established<RepeatRuleSelectionAppliesWithinEligibleIntervals>,
    Established<RepeatRuleEvaluationInheritsInitialStartComponentInformation>,
)>;

/// Result shape for parsing a recurring interval with an attached repeat rule.
pub type ParsedRecurringIntervalWithRepeatRuleResult = TemporalResult<(
    RecurringIntervalWithRepeatRuleDescriptor,
    Established<RecurringIntervalWithRepeatRuleValid>,
    RecurringIntervalWithRepeatRuleIntervalProofBranch,
    Established<RepeatRuleUsesFrequencyDesignator>,
    Established<RepeatRuleDeclaresEligibleTimeIntervals>,
    Established<RepeatRuleSelectionAppliesWithinEligibleIntervals>,
    Established<RepeatRuleEvaluationInheritsInitialStartComponentInformation>,
)>;

/// Result shape for resolving a named zone into the neutral descriptor plus identity sidecars.
pub type ResolvedNamedTimeZoneResult = TemporalResult<(
    NamedTimeZoneDescriptor,
    Established<NamedTimeZoneIdentityValid>,
    NamedTimeZoneIdentityEvidence,
)>;

/// Result shape for confirming explicit local-to-zone resolution authority.
pub type ConfirmedZoneTransitionResolutionAuthorityResult = TemporalResult<(
    Established<ZoneTransitionResolutionAuthorityValid>,
    ZoneTransitionResolutionAuthorityEvidence,
)>;

/// Result shape for realizing a validated local date-time descriptor as a proven native carrier.
pub type RealizedProvenLocalDateTimeResult<T> = TemporalResult<ProvenLocalDateTimeCarrier<T>>;

/// Result shape for reflecting a proven native local date-time carrier back into the descriptor accord.
pub type ReflectedProvenLocalDateTimeResult =
    TemporalResult<(LocalDateTimeDescriptor, LocalDateTimeSemanticBundle)>;

/// Result shape for realizing a validated offset date-time descriptor as a proven native carrier.
pub type RealizedProvenOffsetDateTimeResult<T> = TemporalResult<ProvenOffsetDateTimeCarrier<T>>;

/// Result shape for reflecting a proven native offset date-time carrier back into the descriptor accord.
pub type ReflectedProvenOffsetDateTimeResult =
    TemporalResult<(OffsetDateTimeDescriptor, OffsetDateTimeSemanticBundle)>;

/// Result shape for realizing a validated named-zone descriptor as a proven native carrier.
pub type RealizedProvenNamedTimeZoneResult<T> = TemporalResult<ProvenNamedTimeZoneCarrier<T>>;

/// Result shape for reflecting a proven native named-zone carrier back into the descriptor accord.
pub type ReflectedProvenNamedTimeZoneResult =
    TemporalResult<(NamedTimeZoneDescriptor, NamedTimeZoneSemanticBundle)>;

/// Result shape for realizing a validated zoned date-time descriptor as a proven native carrier.
pub type RealizedProvenZonedDateTimeResult<T> = TemporalResult<ProvenZonedDateTimeCarrier<T>>;

/// Result shape for reflecting a proven native zoned date-time carrier back into the descriptor accord.
pub type ReflectedProvenZonedDateTimeResult =
    TemporalResult<(ZonedDateTimeDescriptor, ZonedDateTimeSemanticBundle)>;

/// Result shape for realizing a validated duration descriptor as a proven native carrier.
pub type RealizedProvenDurationResult<T> = TemporalResult<ProvenDurationCarrier<T>>;

/// Result shape for reflecting a proven native duration carrier back into the descriptor accord.
pub type ReflectedProvenDurationResult =
    TemporalResult<(DurationDescriptor, DurationSemanticBundle)>;

/// Result shape for realizing a validated interval descriptor as a proven native carrier.
pub type RealizedProvenTimeIntervalResult<T> = TemporalResult<ProvenTimeIntervalCarrier<T>>;

/// Result shape for reflecting a proven native interval carrier back into the descriptor accord.
pub type ReflectedProvenTimeIntervalResult =
    TemporalResult<(TimeIntervalDescriptor, TimeIntervalSemanticBundle)>;

/// Result shape for realizing a validated recurring-interval descriptor as a proven native carrier.
pub type RealizedProvenRecurringIntervalResult<T> =
    TemporalResult<ProvenRecurringIntervalCarrier<T>>;

/// Result shape for reflecting a proven native recurring-interval carrier back into the descriptor accord.
pub type ReflectedProvenRecurringIntervalResult =
    TemporalResult<(RecurringIntervalDescriptor, RecurringIntervalSemanticBundle)>;

/// Result shape for realizing a validated qualified temporal value descriptor as a proven native carrier.
pub type RealizedProvenQualifiedTemporalValueResult<T> =
    TemporalResult<ProvenQualifiedTemporalValueCarrier<T>>;

/// Result shape for reflecting a proven native qualified temporal value carrier back into the descriptor accord.
pub type ReflectedProvenQualifiedTemporalValueResult =
    TemporalResult<(QualifiedTemporalValueDescriptor, QualifiedTemporalValueSemanticBundle)>;

/// Result shape for realizing a validated explicit temporal form descriptor as a proven native carrier.
pub type RealizedProvenExplicitTemporalFormResult<T> =
    TemporalResult<ProvenExplicitTemporalFormCarrier<T>>;

/// Result shape for reflecting a proven native explicit temporal form carrier back into the descriptor accord.
pub type ReflectedProvenExplicitTemporalFormResult =
    TemporalResult<(ExplicitTemporalFormDescriptor, ExplicitTemporalFormSemanticBundle)>;

/// Result shape for realizing a validated explicit duration descriptor as a proven native carrier.
pub type RealizedProvenExplicitDurationResult<T> =
    TemporalResult<ProvenExplicitDurationCarrier<T>>;

/// Result shape for reflecting a proven native explicit duration carrier back into the descriptor accord.
pub type ReflectedProvenExplicitDurationResult =
    TemporalResult<(ExplicitDurationDescriptor, ExplicitDurationSemanticBundle)>;

/// Result shape for realizing a validated explicit time-interval descriptor as a proven native carrier.
pub type RealizedProvenExplicitTimeIntervalResult<T> =
    TemporalResult<ProvenExplicitTimeIntervalCarrier<T>>;

/// Result shape for reflecting a proven native explicit time-interval carrier back into the descriptor accord.
pub type ReflectedProvenExplicitTimeIntervalResult =
    TemporalResult<(ExplicitTimeIntervalDescriptor, ExplicitTimeIntervalSemanticBundle)>;

/// Result shape for realizing a validated grouped time-scale-unit descriptor as a proven native carrier.
pub type RealizedProvenGroupedTimeScaleUnitResult<T> =
    TemporalResult<ProvenGroupedTimeScaleUnitCarrier<T>>;

/// Result shape for reflecting a proven native grouped time-scale-unit carrier back into the descriptor accord.
pub type ReflectedProvenGroupedTimeScaleUnitResult =
    TemporalResult<(GroupedTimeScaleUnitDescriptor, GroupedTimeScaleUnitSemanticBundle)>;

/// Result shape for realizing a validated temporal-set descriptor as a proven native carrier.
pub type RealizedProvenTemporalSetResult<T> = TemporalResult<ProvenTemporalSetCarrier<T>>;

/// Result shape for reflecting a proven native temporal-set carrier back into the descriptor accord.
pub type ReflectedProvenTemporalSetResult =
    TemporalResult<(TemporalSetDescriptor, TemporalSetSemanticBundle)>;

/// Result shape for realizing a validated date-time formula descriptor as a proven native carrier.
pub type RealizedProvenDateTimeFormulaResult<T> =
    TemporalResult<ProvenDateTimeFormulaCarrier<T>>;

/// Result shape for reflecting a proven native date-time formula carrier back into the descriptor accord.
pub type ReflectedProvenDateTimeFormulaResult =
    TemporalResult<(DateTimeFormulaDescriptor, DateTimeFormulaSemanticBundle)>;

/// Result shape for evaluating a proven native date-time formula into a proven explicit temporal form.
pub type NativeEvaluatedProvenDateTimeFormulaResult<T> = TemporalResult<(
    ProvenExplicitTemporalFormCarrier<T>,
    DateTimeFormulaEvaluationResultBundle,
)>;

/// Result shape for resolving a proven native local date-time against a proven named zone.
pub type NativeResolvedProvenLocalDateTimeAtNamedZoneResult<T> = TemporalResult<(
    ProvenZonedDateTimeCarrier<T>,
    ZoneTransitionResolutionAuthorityBundle,
    LocalTimeZoneResolutionProofBranch,
)>;

/// Result shape for attaching a proven named zone to a proven fixed-instant timestamp.
pub type NativeAttachedProvenNamedZoneResult<T> = TemporalResult<ProvenZonedDateTimeCarrier<T>>;

/// Result shape for normalizing a proven native offset timestamp to UTC.
pub type NativeNormalizedProvenUtcTimestampResult<T> = TemporalResult<(
    ProvenOffsetDateTimeCarrier<T>,
    LosslessConversionBundle,
)>;

/// Result shape for dropping named-zone identity from a proven native zoned timestamp.
pub type NativeStrippedProvenNamedZoneTimestampResult<T> =
    TemporalResult<ProvenOffsetDateTimeCarrier<T>>;

/// Result shape for reducing proven native timestamp subsecond precision under explicit authority.
pub type NativeTruncatedProvenSubsecondsTimestampResult<T> = TemporalResult<(
    ProvenOffsetDateTimeCarrier<T>,
    SubsecondTruncationBundle,
)>;

/// Result shape for adjusting proven native timestamp precision without losing information.
pub type NativeLosslessPrecisionAdjustedProvenTimestampResult<T> = TemporalResult<(
    ProvenOffsetDateTimeCarrier<T>,
    LosslessConversionBundle,
)>;

/// Result shape for confirming chronological ordering between two proven native fixed-instant endpoints.
pub type NativeOrderedProvenOffsetEndpointsResult =
    TemporalResult<IntervalEndpointOrderingBundle>;

/// Result shape for confirming that a proven zoned timestamp is interpreted under a concrete TZDB revision.
pub type ConfirmedProvenNamedZoneRevisionResult =
    TemporalResult<NamedTimeZoneRevisionBundle>;

/// Result shape for realizing a validated local date-time descriptor as a native carrier.
pub type RealizedLocalDateTimeResult<T> = TemporalResult<(
    T,
    Established<LocalDateTimeDoesNotIdentifyFixedInstant>,
    LocalTimestampSemanticsEvidence,
    Established<BackendConversionSemanticsValid>,
    BackendConversionEvidence,
)>;

/// Result shape for reflecting a native local date-time carrier into the neutral descriptor accord.
pub type ReflectedLocalDateTimeResult = TemporalResult<(
    LocalDateTimeDescriptor,
    Established<LocalDateTimeValid>,
    LocalDateTimeEvidence,
    Established<LocalDateTimeDoesNotIdentifyFixedInstant>,
    LocalTimestampSemanticsEvidence,
    Established<BackendConversionSemanticsValid>,
    BackendConversionEvidence,
)>;

/// Result shape for realizing a validated offset date-time descriptor as a native carrier.
pub type RealizedOffsetDateTimeResult<T> = TemporalResult<(
    T,
    Established<TimestampRepresentsFixedInstant>,
    FixedInstantEvidence,
    Established<BackendConversionSemanticsValid>,
    BackendConversionEvidence,
)>;

/// Result shape for reflecting a native offset date-time carrier into the neutral descriptor accord.
pub type ReflectedOffsetDateTimeResult = TemporalResult<(
    OffsetDateTimeDescriptor,
    Established<OffsetDateTimeValid>,
    OffsetDateTimeEvidence,
    Established<TimestampRepresentsFixedInstant>,
    FixedInstantEvidence,
    Established<BackendConversionSemanticsValid>,
    BackendConversionEvidence,
)>;

/// Result shape for realizing a validated named-zone descriptor as a native carrier.
pub type RealizedNamedTimeZoneResult<T> = TemporalResult<(
    T,
    Established<NamedTimeZoneIdentityValid>,
    NamedTimeZoneIdentityEvidence,
    Established<BackendConversionSemanticsValid>,
    BackendConversionEvidence,
)>;

/// Result shape for reflecting a native named-zone carrier into the neutral descriptor accord.
pub type ReflectedNamedTimeZoneResult = TemporalResult<(
    NamedTimeZoneDescriptor,
    Established<NamedTimeZoneIdentityValid>,
    NamedTimeZoneIdentityEvidence,
    Established<BackendConversionSemanticsValid>,
    BackendConversionEvidence,
)>;

/// Result shape for realizing a validated zoned date-time descriptor as a native carrier.
pub type RealizedZonedDateTimeResult<T> = TemporalResult<(
    T,
    Established<TimestampRepresentsFixedInstant>,
    FixedInstantEvidence,
    Established<NamedTimeZoneIdentityValid>,
    Established<ZonedDateTimeHasNamedZone>,
    NamedZoneAttachmentEvidence,
    Established<OffsetConsistentWithNamedZone>,
    OffsetConsistencyEvidence,
    Established<BackendConversionSemanticsValid>,
    BackendConversionEvidence,
)>;

/// Result shape for reflecting a native zoned date-time carrier into the neutral descriptor accord.
pub type ReflectedZonedDateTimeResult = TemporalResult<(
    ZonedDateTimeDescriptor,
    Established<TimestampRepresentsFixedInstant>,
    FixedInstantEvidence,
    Established<NamedTimeZoneIdentityValid>,
    Established<ZonedDateTimeHasNamedZone>,
    NamedZoneAttachmentEvidence,
    Established<OffsetConsistentWithNamedZone>,
    OffsetConsistencyEvidence,
    Established<BackendConversionSemanticsValid>,
    BackendConversionEvidence,
)>;

/// Result shape for resolving a native local date-time against a native named zone.
pub type NativeResolvedLocalDateTimeAtNamedZoneResult<T> = TemporalResult<(
    T,
    Established<TimestampRepresentsFixedInstant>,
    FixedInstantEvidence,
    Established<NamedTimeZoneIdentityValid>,
    Established<ZonedDateTimeHasNamedZone>,
    NamedZoneAttachmentEvidence,
    Established<OffsetConsistentWithNamedZone>,
    OffsetConsistencyEvidence,
    Established<ZoneTransitionResolutionAuthorityValid>,
    ZoneTransitionResolutionAuthorityEvidence,
    LocalTimeZoneResolutionProofBranch,
)>;

/// Result shape for attaching a native named zone to a native fixed-instant timestamp.
pub type NativeAttachedNamedZoneResult<T> = TemporalResult<(
    T,
    Established<TimestampRepresentsFixedInstant>,
    FixedInstantEvidence,
    Established<NamedTimeZoneIdentityValid>,
    Established<ZonedDateTimeHasNamedZone>,
    NamedZoneAttachmentEvidence,
    Established<OffsetConsistentWithNamedZone>,
    OffsetConsistencyEvidence,
)>;

/// Result shape for normalizing a native offset timestamp to UTC.
pub type NativeNormalizedUtcTimestampResult<T> = TemporalResult<(
    T,
    Established<TimestampRepresentsFixedInstant>,
    FixedInstantEvidence,
    Established<ConversionPreservesRepresentedInstant>,
    Established<ConversionPreservesTemporalOrdering>,
    Established<BackendConversionSemanticsValid>,
    BackendConversionEvidence,
    Established<ConversionLossless>,
    LosslessConversionEvidence,
)>;

/// Result shape for dropping named-zone identity from a native zoned timestamp.
pub type NativeStrippedNamedZoneTimestampResult<T> = TemporalResult<(
    T,
    Established<TimestampRepresentsFixedInstant>,
    FixedInstantEvidence,
    Established<ConversionDropsNamedZoneIdentity>,
    Established<ConversionPreservesRepresentedInstant>,
    Established<ConversionPreservesTemporalOrdering>,
    Established<BackendConversionSemanticsValid>,
    BackendConversionEvidence,
)>;

/// Result shape for reducing native timestamp subsecond precision under explicit authority.
pub type NativeTruncatedSubsecondsTimestampResult<T> = TemporalResult<(
    T,
    Established<TimestampRepresentsFixedInstant>,
    FixedInstantEvidence,
    Established<BackendConversionSemanticsValid>,
    BackendConversionEvidence,
    Established<ConversionTruncatesSubseconds>,
    SubsecondTruncationEvidence,
)>;

/// Result shape for adjusting native timestamp precision without losing information.
pub type NativeLosslessPrecisionAdjustedTimestampResult<T> = TemporalResult<(
    T,
    Established<TimestampRepresentsFixedInstant>,
    FixedInstantEvidence,
    Established<BackendConversionSemanticsValid>,
    BackendConversionEvidence,
    Established<ConversionLossless>,
    LosslessConversionEvidence,
)>;

/// Result shape for realizing a validated duration descriptor as a native carrier.
pub type RealizedDurationResult<T> = TemporalResult<(
    T,
    DurationRepresentationProofBranch,
    Established<BackendConversionSemanticsValid>,
    BackendConversionEvidence,
)>;

/// Result shape for reflecting a native duration carrier into the neutral descriptor accord.
pub type ReflectedDurationResult = TemporalResult<(
    DurationDescriptor,
    Established<DurationFormValid>,
    DurationRepresentationProofBranch,
    Established<BackendConversionSemanticsValid>,
    BackendConversionEvidence,
)>;

/// Result shape for realizing a validated interval descriptor as a native carrier.
pub type RealizedTimeIntervalResult<T> = TemporalResult<(
    T,
    Established<ExtendedIntervalBoundarySemanticsValid>,
    IntervalEndComponentInheritanceProofBranch,
    IntervalZoneInheritanceProofBranch,
    CompleteIntervalSubstitutionProofBranch,
    Established<BackendConversionSemanticsValid>,
    BackendConversionEvidence,
)>;

/// Result shape for reflecting a native interval carrier into the neutral descriptor accord.
pub type ReflectedTimeIntervalResult = TemporalResult<(
    TimeIntervalDescriptor,
    Established<TimeIntervalValid>,
    Established<ExtendedIntervalBoundarySemanticsValid>,
    IntervalEndComponentInheritanceProofBranch,
    IntervalZoneInheritanceProofBranch,
    CompleteIntervalSubstitutionProofBranch,
    Established<BackendConversionSemanticsValid>,
    BackendConversionEvidence,
)>;

/// Result shape for realizing a validated recurring-interval descriptor as a native carrier.
pub type RealizedRecurringIntervalResult<T> = TemporalResult<(
    T,
    RecurringIntervalRepresentationProofBranch,
    Established<BackendConversionSemanticsValid>,
    BackendConversionEvidence,
)>;

/// Result shape for reflecting a native recurring-interval carrier into the neutral descriptor accord.
pub type ReflectedRecurringIntervalResult = TemporalResult<(
    RecurringIntervalDescriptor,
    Established<RecurringIntervalFormValid>,
    RecurringIntervalRepresentationProofBranch,
    Established<BackendConversionSemanticsValid>,
    BackendConversionEvidence,
)>;

/// Result shape for ordering two native fixed-instant endpoints chronologically.
pub type NativeOrderedOffsetEndpointsResult = TemporalResult<Established<IntervalEndpointsOrdered>>;

/// Result shape for resolving a local wall-clock timestamp against a named zone.
pub type ResolvedLocalDateTimeAtNamedZoneResult = TemporalResult<(
    ZonedDateTimeDescriptor,
    Established<TimestampRepresentsFixedInstant>,
    Established<ZonedDateTimeHasNamedZone>,
    NamedZoneAttachmentEvidence,
    Established<OffsetConsistentWithNamedZone>,
    OffsetConsistencyEvidence,
    Established<ZoneTransitionResolutionAuthorityValid>,
    ZoneTransitionResolutionAuthorityEvidence,
    LocalTimeZoneResolutionProofBranch,
)>;

/// Result shape for attaching named-zone identity to a fixed-instant timestamp.
pub type AttachedNamedZoneResult = TemporalResult<(
    ZonedDateTimeDescriptor,
    Established<ZonedDateTimeHasNamedZone>,
    NamedZoneAttachmentEvidence,
    Established<OffsetConsistentWithNamedZone>,
    OffsetConsistencyEvidence,
)>;

/// Result shape for confirming revision-aware named-zone interpretation semantics.
pub type ConfirmedNamedZoneRevisionResult = TemporalResult<(
    Established<NamedTimeZoneInterpretationTracksTzdbRevision>,
    NamedTimeZoneRevisionEvidence,
)>;

/// Result shape for parsing an RFC 9557 IXDTF timestamp and its suffix semantics.
pub type ParsedIxdtfTimestampResult = TemporalResult<(
    IxdtfTimestampDescriptor,
    Established<IxdtfTimestampValid>,
    Established<TimestampRepresentsFixedInstant>,
    IxdtfTimeZoneAnnotationProofBranch,
    IxdtfCalendarAnnotationProofBranch,
    IxdtfAdditionalInformationProofBranch,
)>;

/// Result shape for parsing an ISO 8601-2 seasonal temporal expression.
pub type ParsedSeasonalTemporalExpressionResult = TemporalResult<(
    SeasonalTemporalExpressionDescriptor,
    Established<SeasonalTemporalExpressionValid>,
    Established<SeasonalExpressionUsesYearAndSeasonForm>,
    Established<SeasonalExpressionUsesSeasonCodeInMonthSlot>,
    Established<SeasonCodeDeclaresNamedSeason>,
    Established<SeasonCodeDeclaresSeasonScope>,
)>;

/// Result shape for parsing an ISO 8601-2 Level 2 sub-year grouping expression.
pub type ParsedSubYearGroupingExpressionResult = TemporalResult<(
    SubYearGroupingExpressionDescriptor,
    Established<SubYearGroupingExpressionValid>,
    Established<SubYearGroupingExpressionUsesYearAndGroupingForm>,
    Established<SubYearGroupingExpressionUsesGroupingCodeInMonthSlot>,
    SubYearGroupingKindEvidence,
)>;

/// Result shape for parsing an ISO 8601-2 unspecified-component expression.
pub type ParsedUnspecifiedComponentExpressionResult = TemporalResult<(
    UnspecifiedComponentExpressionDescriptor,
    Established<UnspecifiedComponentExpressionValid>,
    Established<UnspecifiedDigitUsesUppercaseXPlaceholder>,
    Established<UnspecifiedDigitsDeclareUnknownValue>,
    Established<LevelOneUnspecifiedDigitsOccupyRightmostPositions>,
    Established<LevelTwoUnspecifiedDigitsMayAppearWithinComponent>,
)>;

/// Result shape for parsing an ISO 8601-2 temporal set expression.
pub type ParsedTemporalSetResult = TemporalResult<(
    TemporalSetDescriptor,
    Established<TemporalSetExpressionValid>,
    Established<TemporalSetRangeSemanticsValid>,
)>;

/// Result shape for parsing an ISO 8601 interval including extended-boundary semantics.
pub type ParsedTimeIntervalResult = TemporalResult<(
    TimeIntervalDescriptor,
    Established<TimeIntervalValid>,
    Established<ExtendedIntervalBoundarySemanticsValid>,
    IntervalEndComponentInheritanceProofBranch,
    IntervalZoneInheritanceProofBranch,
    CompleteIntervalSubstitutionProofBranch,
)>;

/// Result shape for normalizing an offset timestamp to UTC.
pub type NormalizedUtcTimestampResult = TemporalResult<(
    OffsetDateTimeDescriptor,
    Established<OffsetDateTimeValid>,
    Established<ConversionPreservesRepresentedInstant>,
    Established<ConversionPreservesTemporalOrdering>,
)>;

/// Result shape for dropping named-zone identity from a zoned timestamp.
pub type StrippedNamedZoneTimestampResult = TemporalResult<(
    OffsetDateTimeDescriptor,
    Established<OffsetDateTimeValid>,
    Established<ConversionDropsNamedZoneIdentity>,
    Established<ConversionPreservesRepresentedInstant>,
    Established<ConversionPreservesTemporalOrdering>,
)>;

/// Result shape for reducing timestamp subsecond precision under explicit authority.
pub type TruncatedSubsecondsTimestampResult = TemporalResult<(
    OffsetDateTimeDescriptor,
    Established<OffsetDateTimeValid>,
    Established<BackendConversionSemanticsValid>,
    Established<ConversionTruncatesSubseconds>,
)>;

/// Result shape for adjusting timestamp precision without losing information.
pub type LosslessPrecisionAdjustedTimestampResult = TemporalResult<(
    OffsetDateTimeDescriptor,
    Established<OffsetDateTimeValid>,
    Established<ConversionLossless>,
)>;

/// Result shape for emitting an IXDTF timestamp together with suffix-semantics confirmation.
pub type FormattedIxdtfTimestampResult = TemporalResult<(
    String,
    Established<IxdtfTimestampValid>,
    IxdtfTimeZoneAnnotationProofBranch,
    IxdtfCalendarAnnotationProofBranch,
    IxdtfAdditionalInformationProofBranch,
)>;

/// Result shape for emitting an ISO 8601-2 extended year form.
pub type FormattedExtendedYearResult = TemporalResult<(String, Established<ExtendedYearValid>)>;

/// Result shape for emitting an ISO 8601 decade representation.
pub type FormattedDecadeResult = TemporalResult<(String, Established<DecadeValid>)>;

/// Result shape for emitting an ISO 8601 century representation.
pub type FormattedCenturyResult = TemporalResult<(String, Established<CenturyValid>)>;

/// Result shape for emitting a complete explicit-form date with shift.
pub type FormattedDateWithShiftResult = TemporalResult<(String, Established<DateWithShiftValid>)>;

/// Result shape for emitting a complete explicit-form time of day with shift.
pub type FormattedTimeOfDayWithShiftResult =
    TemporalResult<(String, Established<TimeOfDayWithShiftValid>)>;

/// Result shape for emitting an ISO 8601-2 seasonal temporal expression.
pub type FormattedSeasonalTemporalExpressionResult = TemporalResult<(
    String,
    Established<SeasonalTemporalExpressionValid>,
    Established<SeasonalExpressionUsesYearAndSeasonForm>,
    Established<SeasonalExpressionUsesSeasonCodeInMonthSlot>,
    Established<SeasonCodeDeclaresNamedSeason>,
    Established<SeasonCodeDeclaresSeasonScope>,
)>;

/// Result shape for emitting an ISO 8601-2 Level 2 sub-year grouping expression.
pub type FormattedSubYearGroupingExpressionResult = TemporalResult<(
    String,
    Established<SubYearGroupingExpressionValid>,
    Established<SubYearGroupingExpressionUsesYearAndGroupingForm>,
    Established<SubYearGroupingExpressionUsesGroupingCodeInMonthSlot>,
    SubYearGroupingKindEvidence,
)>;

/// Result shape for emitting an ISO 8601-2 unspecified-component expression.
pub type FormattedUnspecifiedComponentExpressionResult = TemporalResult<(
    String,
    Established<UnspecifiedComponentExpressionValid>,
    Established<UnspecifiedDigitUsesUppercaseXPlaceholder>,
    Established<UnspecifiedDigitsDeclareUnknownValue>,
    Established<LevelOneUnspecifiedDigitsOccupyRightmostPositions>,
    Established<LevelTwoUnspecifiedDigitsMayAppearWithinComponent>,
)>;

/// Result shape for emitting an ISO 8601-2 temporal set expression.
pub type FormattedTemporalSetResult = TemporalResult<(
    String,
    Established<TemporalSetExpressionValid>,
    Established<TemporalSetRangeSemanticsValid>,
)>;

/// Result shape for emitting an ISO 8601 duration.
pub type FormattedDurationResult = TemporalResult<(
    String,
    Established<DurationFormValid>,
    DurationRepresentationProofBranch,
)>;

/// Result shape for emitting a CalConnect explicit duration.
pub type FormattedExplicitDurationResult = TemporalResult<(
    String,
    Established<ExplicitDurationValid>,
    Established<ExplicitDurationUsesDurationalUnitDesignators>,
    ExplicitDurationRepresentationEvidence,
    Established<ExplicitDurationMayBeNegative>,
    Established<ExplicitDurationMayUseFractionalLowestOrderUnit>,
    ExplicitDurationSemanticEvidence,
)>;

/// Result shape for emitting a CalConnect explicit time interval.
pub type FormattedExplicitTimeIntervalResult = TemporalResult<(
    String,
    Established<ExplicitTimeIntervalValid>,
    ExplicitIntervalDurationSubstitutionProofBranch,
    ExplicitIntervalEndComponentInheritanceProofBranch,
    ExplicitIntervalShiftPropagationProofBranch,
)>;

/// Result shape for emitting a qualified temporal value.
pub type FormattedQualifiedTemporalValueResult = TemporalResult<(
    String,
    Established<QualifiedTemporalValueValid>,
    Established<QualifiedTemporalExpressionValid>,
    QualificationPlacementEvidence,
)>;

/// Result shape for emitting an explicit temporal form.
pub type FormattedExplicitTemporalFormResult = TemporalResult<(
    String,
    Established<ExplicitTemporalFormValid>,
    Established<ExplicitTemporalFormUsesDesignatorSymbols>,
    Established<ExplicitTemporalFormMayOmitZeroValuedComponents>,
    Established<ExplicitTemporalPrecisionUsesLowestDenotedComponent>,
    Established<ExplicitUtcRelationshipUsesZuluOrSignedShift>,
)>;

/// Result shape for emitting a grouped time scale unit expression.
pub type FormattedGroupedTimeScaleUnitResult = TemporalResult<(
    String,
    Established<GroupedTimeScaleUnitValid>,
    Established<GroupedTimeScaleUnitUsesGroupingDesignators>,
    Established<GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits>,
    Established<GroupedTimeScaleUnitDefinitionIsContinuous>,
    Established<GroupedTimeScaleUnitValueCarriesExplicitCoefficient>,
    Established<GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds>,
    Established<GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift>,
    Established<GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder>,
    Established<GroupedTimeScaleUnitConvertsToTimeInterval>,
)>;

/// Result shape for emitting a date-time formula and its evaluation semantics.
pub type FormattedDateTimeFormulaResult = TemporalResult<(
    String,
    Established<DateTimeFormulaValid>,
    Established<DateTimeFormulaEvaluationSemanticsValid>,
)>;

/// Result shape for emitting a selection expression.
pub type FormattedSelectionExpressionResult = TemporalResult<(
    String,
    Established<SelectionExpressionValid>,
    Established<SelectionExpressionUsesSelectionDelimiters>,
    Established<SelectionExpressionUsesRecognizedSelectionRuleVocabulary>,
    Established<SelectionRuleMonthUsesMonthExpression>,
    Established<SelectionRuleWeekUsesWeekExpression>,
    Established<SelectionRuleDayOfMonthUsesDayExpression>,
    Established<SelectionRuleWeekDayUsesDayOfWeekExpression>,
    Established<SelectionRuleOrdinalDayOfYearUsesOrdinalDayExpression>,
    Established<SelectionRuleHourUsesHourExpression>,
    Established<SelectionRuleMinuteUsesMinuteExpression>,
    Established<SelectionRuleSecondUsesSecondExpression>,
    Established<SelectionRulesApplyWithinSelectedResults>,
    Established<SelectionExpressionMaySelectSingleInstance>,
    Established<SelectionRulePositionUsesInstanceDesignatorSuffix>,
    Established<SelectionRulePositionAppliesLast>,
    Established<SelectionWithDurationUsesDurationSuffix>,
)>;

/// Result shape for emitting a repeat rule.
pub type FormattedRepeatRuleResult = TemporalResult<(
    String,
    Established<RepeatRuleValid>,
    Established<RepeatRuleUsesFrequencyDesignator>,
    Established<RepeatRuleDeclaresEligibleTimeIntervals>,
    Established<RepeatRuleSelectionAppliesWithinEligibleIntervals>,
    Established<RepeatRuleEvaluationInheritsInitialStartComponentInformation>,
)>;

/// Result shape for emitting an ISO 8601 recurring interval.
pub type FormattedRecurringIntervalResult = TemporalResult<(
    String,
    Established<RecurringIntervalFormValid>,
    RecurringIntervalRepresentationProofBranch,
)>;

/// Result shape for emitting a recurring interval with an attached repeat rule.
pub type FormattedRecurringIntervalWithRepeatRuleResult = TemporalResult<(
    String,
    Established<RecurringIntervalWithRepeatRuleValid>,
    RecurringIntervalWithRepeatRuleIntervalProofBranch,
    Established<RepeatRuleUsesFrequencyDesignator>,
    Established<RepeatRuleDeclaresEligibleTimeIntervals>,
    Established<RepeatRuleSelectionAppliesWithinEligibleIntervals>,
    Established<RepeatRuleEvaluationInheritsInitialStartComponentInformation>,
)>;

/// Result shape for emitting an ISO 8601 interval including extended-boundary semantics.
pub type FormattedTimeIntervalResult = TemporalResult<(
    String,
    Established<TimeIntervalValid>,
    Established<ExtendedIntervalBoundarySemanticsValid>,
    IntervalEndComponentInheritanceProofBranch,
    IntervalZoneInheritanceProofBranch,
    CompleteIntervalSubstitutionProofBranch,
)>;
