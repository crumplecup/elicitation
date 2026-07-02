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
    BackendConversionSemanticsValid, CenturyValid, ConversionDropsNamedZoneIdentity,
    ConversionLossless, ConversionPreservesRepresentedInstant, ConversionPreservesTemporalOrdering,
    ConversionTruncatesSubseconds, DateTimeFormulaEvaluationSemanticsValid, DateTimeFormulaValid,
    DateWithShiftValid, DecadeValid, DurationFormValid, ExplicitDurationValid,
    ExplicitTemporalFormValid, ExtendedIntervalBoundarySemanticsValid, ExtendedYearValid,
    GroupedTimeScaleUnitValid, IxdtfAdditionalInformationEvidence,
    IxdtfAdditionalInformationSemanticsValid, IxdtfCalendarAwareTimestampEvidence,
    IxdtfTimestampHasPreferredPresentationCalendar, IxdtfTimestampValid,
    LocalDateTimeMayBeAmbiguousAtZoneTransition, LocalDateTimeMayFallInZoneTransitionGap,
    OffsetConsistentWithNamedZone, OffsetDateTimeValid,
    OffsetTimeZoneAnnotationConsistentWithTimestamp, OffsetTimeZoneAnnotationEvidence,
    QualifiedTemporalValueValid, RecurringIntervalFormValid, RecurringIntervalWithRepeatRuleValid,
    ReducedCalendarDateValid, ReducedLocalTimeValid, RepeatRuleValid, Rfc3339TimestampValid,
    SeasonalTemporalExpressionValid, SelectionExpressionValid, SubYearGroupingExpressionValid,
    TemporalResult, TemporalSetExpressionValid, TemporalSetRangeSemanticsValid, TimeIntervalValid,
    TimeOfDayWithShiftValid, TimestampRepresentsFixedInstant, UnspecifiedComponentExpressionValid,
    ZoneTransitionAmbiguitySemanticsValid, ZoneTransitionGapSemanticsValid,
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

/// Semantic interpretation of a numeric UTC offset payload.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, Default,
)]
pub enum UtcOffsetRelationship {
    /// The local offset is known.
    #[default]
    #[display("known")]
    Known,
    /// RFC 3339 `-00:00` unknown-local-offset convention.
    #[display("unknown-local-offset")]
    UnknownLocalOffset,
}

/// Numeric UTC offset descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct UtcOffsetDescriptor {
    /// Offset sign.
    pub sign: UtcOffsetSign,
    /// Absolute hour component.
    pub hours: u8,
    /// Absolute minute component when the offset is represented with hour-minute precision.
    ///
    /// `None` denotes the integral-hour form permitted by ISO 8601.
    #[builder(default)]
    pub minutes: Option<u8>,
    /// Whether the offset is known or uses RFC 3339's unknown-local-offset convention.
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

/// Complete recurring-interval representation extended with a repeat rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct RecurringIntervalWithRepeatRuleDescriptor {
    /// Optional bounded repetition count; `None` denotes unbounded recurrence.
    #[builder(default)]
    pub repetitions: Option<u32>,
    /// Base time interval being repeated.
    pub interval: TimeIntervalDescriptor,
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
    },
    /// The local wall-clock time fell inside a skipped transition gap.
    Gap {
        /// The local timestamp can fall inside a skipped zone-transition gap.
        possibility: Established<LocalDateTimeMayFallInZoneTransitionGap>,
        /// Aggregate proof that gap semantics were handled explicitly and lawfully.
        semantics: Established<ZoneTransitionGapSemanticsValid>,
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
pub type ParsedDurationResult =
    TemporalResult<(DurationDescriptor, Established<DurationFormValid>)>;

/// Result shape for parsing a CalConnect explicit duration.
pub type ParsedExplicitDurationResult = TemporalResult<(
    ExplicitDurationDescriptor,
    Established<ExplicitDurationValid>,
)>;

/// Result shape for parsing an ISO 8601 recurring interval.
pub type ParsedRecurringIntervalResult = TemporalResult<(
    RecurringIntervalDescriptor,
    Established<RecurringIntervalFormValid>,
)>;

/// Result shape for parsing a qualified temporal value.
pub type ParsedQualifiedTemporalValueResult = TemporalResult<(
    QualifiedTemporalValueDescriptor,
    Established<QualifiedTemporalValueValid>,
)>;

/// Result shape for parsing an explicit temporal form.
pub type ParsedExplicitTemporalFormResult = TemporalResult<(
    ExplicitTemporalFormDescriptor,
    Established<ExplicitTemporalFormValid>,
)>;

/// Result shape for parsing a grouped time scale unit expression.
pub type ParsedGroupedTimeScaleUnitResult = TemporalResult<(
    GroupedTimeScaleUnitDescriptor,
    Established<GroupedTimeScaleUnitValid>,
)>;

/// Result shape for parsing a date-time formula and its evaluation semantics.
pub type ParsedDateTimeFormulaResult = TemporalResult<(
    DateTimeFormulaDescriptor,
    Established<DateTimeFormulaValid>,
    Established<DateTimeFormulaEvaluationSemanticsValid>,
)>;

/// Result shape for evaluating a date-time formula into an explicit temporal value.
pub type EvaluatedDateTimeFormulaResult = TemporalResult<(
    ExplicitTemporalValueDescriptor,
    Established<ExplicitTemporalFormValid>,
)>;

/// Result shape for parsing a selection expression.
pub type ParsedSelectionExpressionResult = TemporalResult<(
    SelectionExpressionDescriptor,
    Established<SelectionExpressionValid>,
)>;

/// Result shape for parsing a repeat rule.
pub type ParsedRepeatRuleResult =
    TemporalResult<(RepeatRuleDescriptor, Established<RepeatRuleValid>)>;

/// Result shape for parsing a recurring interval with an attached repeat rule.
pub type ParsedRecurringIntervalWithRepeatRuleResult = TemporalResult<(
    RecurringIntervalWithRepeatRuleDescriptor,
    Established<RecurringIntervalWithRepeatRuleValid>,
)>;

/// Result shape for resolving a local wall-clock timestamp against a named zone.
pub type ResolvedLocalDateTimeAtNamedZoneResult = TemporalResult<(
    ZonedDateTimeDescriptor,
    Established<TimestampRepresentsFixedInstant>,
    Established<ZonedDateTimeHasNamedZone>,
    Established<OffsetConsistentWithNamedZone>,
    Established<ZoneTransitionResolutionAuthorityValid>,
    LocalTimeZoneResolutionProofBranch,
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
)>;

/// Result shape for parsing an ISO 8601-2 Level 2 sub-year grouping expression.
pub type ParsedSubYearGroupingExpressionResult = TemporalResult<(
    SubYearGroupingExpressionDescriptor,
    Established<SubYearGroupingExpressionValid>,
)>;

/// Result shape for parsing an ISO 8601-2 unspecified-component expression.
pub type ParsedUnspecifiedComponentExpressionResult = TemporalResult<(
    UnspecifiedComponentExpressionDescriptor,
    Established<UnspecifiedComponentExpressionValid>,
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
pub type FormattedSeasonalTemporalExpressionResult =
    TemporalResult<(String, Established<SeasonalTemporalExpressionValid>)>;

/// Result shape for emitting an ISO 8601-2 Level 2 sub-year grouping expression.
pub type FormattedSubYearGroupingExpressionResult =
    TemporalResult<(String, Established<SubYearGroupingExpressionValid>)>;

/// Result shape for emitting an ISO 8601-2 unspecified-component expression.
pub type FormattedUnspecifiedComponentExpressionResult =
    TemporalResult<(String, Established<UnspecifiedComponentExpressionValid>)>;

/// Result shape for emitting an ISO 8601-2 temporal set expression.
pub type FormattedTemporalSetResult = TemporalResult<(
    String,
    Established<TemporalSetExpressionValid>,
    Established<TemporalSetRangeSemanticsValid>,
)>;

/// Result shape for emitting an ISO 8601 duration.
pub type FormattedDurationResult = TemporalResult<(String, Established<DurationFormValid>)>;

/// Result shape for emitting a CalConnect explicit duration.
pub type FormattedExplicitDurationResult =
    TemporalResult<(String, Established<ExplicitDurationValid>)>;

/// Result shape for emitting a qualified temporal value.
pub type FormattedQualifiedTemporalValueResult =
    TemporalResult<(String, Established<QualifiedTemporalValueValid>)>;

/// Result shape for emitting an explicit temporal form.
pub type FormattedExplicitTemporalFormResult =
    TemporalResult<(String, Established<ExplicitTemporalFormValid>)>;

/// Result shape for emitting a grouped time scale unit expression.
pub type FormattedGroupedTimeScaleUnitResult =
    TemporalResult<(String, Established<GroupedTimeScaleUnitValid>)>;

/// Result shape for emitting a date-time formula and its evaluation semantics.
pub type FormattedDateTimeFormulaResult = TemporalResult<(
    String,
    Established<DateTimeFormulaValid>,
    Established<DateTimeFormulaEvaluationSemanticsValid>,
)>;

/// Result shape for emitting a selection expression.
pub type FormattedSelectionExpressionResult =
    TemporalResult<(String, Established<SelectionExpressionValid>)>;

/// Result shape for emitting a repeat rule.
pub type FormattedRepeatRuleResult = TemporalResult<(String, Established<RepeatRuleValid>)>;

/// Result shape for emitting an ISO 8601 recurring interval.
pub type FormattedRecurringIntervalResult =
    TemporalResult<(String, Established<RecurringIntervalFormValid>)>;

/// Result shape for emitting a recurring interval with an attached repeat rule.
pub type FormattedRecurringIntervalWithRepeatRuleResult =
    TemporalResult<(String, Established<RecurringIntervalWithRepeatRuleValid>)>;

/// Result shape for emitting an ISO 8601 interval including extended-boundary semantics.
pub type FormattedTimeIntervalResult = TemporalResult<(
    String,
    Established<TimeIntervalValid>,
    Established<ExtendedIntervalBoundarySemanticsValid>,
)>;
