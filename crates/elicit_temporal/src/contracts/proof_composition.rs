//! Temporal proof composition — `ProvableFrom` dependency chains.
//!
//! This module declares how higher-order temporal propositions are minted from
//! lower-order standards evidence. None of this is runtime validation logic; it
//! is the proof-sidecar graph that producers and consumers exchange.

use elicitation::contracts::Prop;
use elicitation::proc_macro2::TokenStream;
use elicitation::quote::quote;
use elicitation::{Established, contracts::ProvableFrom};

use crate::{
    ApproximationQualificationDeclared, BeforeYearOneValueUsesTrailingBSuffix,
    CalendarDateHasYearMonthDay, CalendarDateUsesGregorianCalendar, CalendarDayWithinMonthBounds,
    CalendarMonthInRangeOneToTwelve, CalendarYearThrough1582RequiresMutualAgreement,
    CenturyOrdinalInRangeZeroToNinetyNine, CombinedDateTimeDateComponentMustNotUseReducedAccuracy,
    CombinedDateTimePermitsCalendarDateComponent, CombinedDateTimePermitsOrdinalDateComponent,
    CombinedDateTimePermitsWeekDateComponent,
    CombinedDateTimeUsesSingleFormatAcrossDateAndTimeComponents,
    CombinedDateTimeUsesTimeDesignator, ComponentQualificationAppliesOnlyToMarkedComponent,
    ComponentQualificationUsesImmediateLeftPlacement, ContextDependentDurationSemanticsDeclared,
    ConversionDropsSubsecondPrecision, ConversionPreservesRepresentedInstant,
    ConversionPreservesTemporalOrdering, ConversionRequiresExplicitAuthorityWhenLossy,
    ConversionSourceSemanticKindDeclared, ConversionTargetSemanticKindDeclared,
    DateIdentifiesPositionWithinCalendar, DateTimeFormulaCombinesTemporalValueWithDuration,
    DateTimeFormulaEvaluationModeDeclared, DateTimeFormulaTruncatesAtComponentBoundaries,
    DateTimeFormulaUsesCarryOverSemantics, DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
    DurationAlternativeFormCarriesCompleteCalendarAndClockComponents,
    DurationAlternativeFormRequiresPartnerAgreement,
    DurationAlternativeFormUsesDateAndTimeComponentSlots,
    DurationTimeComponentsFollowTimeDesignator, DurationUsesPeriodDesignator,
    DurationWeekFormNotMixedWithCalendarOrClockUnits, DurationWeekFormUsesSingleWeekUnit,
    ExactDurationSemanticsDeclared, ExpandedRepresentationRequiresAdditionalAgreement,
    ExplicitDateTimeTimePortionMayBeReducedPrecision,
    ExplicitDateTimeUsesDateThenTimeConcatenation,
    ExplicitDateTimeWithShiftUsesDateTimeThenShiftConcatenation,
    ExplicitDateWithShiftUsesDateThenShiftConcatenation,
    ExplicitDurationCompositeRepresentationDeclared, ExplicitDurationMayBeNegative,
    ExplicitDurationMayUseFractionalLowestOrderUnit,
    ExplicitDurationPrecedenceRepresentationCarriesEvaluationOrder,
    ExplicitDurationRepresentationKindDeclared, ExplicitDurationUsesDurationalUnitDesignators,
    ExplicitTemporalFormMayOmitZeroValuedComponents, ExplicitTemporalFormUsesDesignatorSymbols,
    ExplicitTemporalPrecisionUsesLowestDenotedComponent,
    ExplicitTimeOfDayForbidsEndOfDayRepresentation,
    ExplicitTimeOfDayUsesHourMinuteSecondUnitDesignators, ExplicitTimeOfDayUsesTimeDesignator,
    ExplicitTimeOfDayWithShiftUsesTimeThenShiftConcatenation,
    ExplicitTimeShiftBareZuluRepresentsUtcZero, ExplicitTimeShiftPayloadUsesExplicitTimeOfDay,
    ExplicitTimeShiftUsesLeadingMinusOnlyWhenBehindUtc, ExplicitTimeShiftUsesZuluDesignator,
    ExplicitUtcRelationshipUsesZuluOrSignedShift, ExponentialYearExponentIsPositiveInteger,
    ExponentialYearUsesPowerOfTenNotation, FractionAppliesToLowestOrderComponent,
    FractionUsesDecimalSign, FractionalSecondDigitsAreContiguous,
    FractionalSecondPrecisionDeclared,
    GroupQualificationAppliesToMarkedAndMoreSignificantComponents,
    GroupQualificationUsesImmediateRightPlacement,
    GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits, GroupedTimeScaleUnitConvertsToTimeInterval,
    GroupedTimeScaleUnitDefinitionIsContinuous,
    GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds,
    GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder, GroupedTimeScaleUnitUsesGroupingDesignators,
    GroupedTimeScaleUnitValueCarriesExplicitCoefficient, HourInRangeZeroToTwentyFour,
    IntervalDurationIsNonNegative, IntervalStartPrecedesEnd,
    IxdtfCalendarAnnotationDeclaresPreferredPresentationCalendar, IxdtfCalendarAnnotationPresent,
    IxdtfCalendarKeyUsesUCa, IxdtfCalendarValueUsesUnicodeCalendarIdentifier,
    IxdtfCriticalFlagIsLeadingExclamationWhenPresent,
    IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling,
    IxdtfExperimentalSuffixKeysAreNotForInterchange,
    IxdtfExperimentalSuffixKeysUseLeadingUnderscore, IxdtfGeneratorsMayOmitSuffixTags,
    IxdtfRecipientsMayIgnoreElectiveSuffixTags,
    IxdtfRecipientsMustRejectUnconfiguredExperimentalSuffixKeys,
    IxdtfSuffixFollowsRfc3339Timestamp, IxdtfSuffixKeysAreLowercase,
    IxdtfSuffixTagsUseBracketedKeyValueForm,
    IxdtfSuffixValuesAreCaseSensitiveUnlessOtherwiseSpecified,
    IxdtfSuffixValuesUseHyphenDelimitedItems, IxdtfTimeZoneSuffixUsesBracketedNameOrOffset,
    LeapDayOccursOnlyInLeapYear, LeapSecondOccursOnlyAtUtcBoundary,
    LetterPrefixedCalendarYearMagnitudeExceedsFourDigits,
    LetterPrefixedCalendarYearUsesLeadingYDesignator,
    LevelOneUnspecifiedDigitsOccupyRightmostPositions,
    LevelTwoUnspecifiedDigitsMayAppearWithinComponent, LocalDateTimeMayBeAmbiguousAtZoneTransition,
    LocalDateTimeMayFallInZoneTransitionGap, LocalDateTimeRequiresZoneOrOffsetForInstant,
    LocalTimeHasHourMinuteSecond, LocalTimeScaleMayBeStandardOrNonUtcBased,
    LocalTimeUsesLocallyApplicableTimeScale, MinuteInRangeZeroToFiftyNine,
    NamedTimeZoneAnnotationPresent, NamedTimeZoneIsNotNumericOffsetAlias,
    NamedTimeZoneMeaningUsesCurrentTzdbRules, NamedTimeZoneRetainsCivilRuleIdentity,
    NamedTimeZoneUsesIanaIdentifier, NegativeCalendarYearUsesLeadingMinusSign,
    NumericOffsetDoesNotIdentifyNamedZone, OffsetDateTimeIdentifiesSingleInstant,
    OffsetTimeZoneAnnotationPresent, OffsetTimeZoneMustNotBeSynthesizedFromTimestampOffset,
    OffsetTimeZoneRepeatsTimestampOffset, OffsetTimeZoneUseIsStronglyDiscouraged,
    OpenIntervalBoundaryDeclared, OrdinalDateHasYearAndDayOfYear,
    OrdinalDayInRangeOneToThreeHundredSixtySix, PrecisionReductionDeclared,
    ProlepticGregorianDatesBefore1583RequireMutualAgreement, QualificationScopeDeclared,
    RecurringIntervalCarriesIntervalComponent, RecurringIntervalCountIsNonNegativeWhenBounded,
    RecurringIntervalOmittedCountDenotesUnboundedOccurrences,
    RecurringIntervalUsesRepeatDesignator,
    RecurringIntervalWithRepeatRuleUsesCompleteRepresentation,
    ReducedAccuracyLocalTimeUsesHourMinuteRepresentation,
    ReducedAccuracyLocalTimeUsesHourOnlyRepresentation, ReducedCalendarDateHasYearComponent,
    ReducedCalendarDateUsesYearMonthRepresentation, ReducedCalendarDateUsesYearOnlyRepresentation,
    RepeatRuleDeclaresEligibleTimeIntervals,
    RepeatRuleEvaluationInheritsInitialStartComponentInformation,
    RepeatRuleSelectionAppliesWithinEligibleIntervals, RepeatRuleUsesFrequencyDesignator,
    Rfc3339FractionUsesDotSeparator, Rfc3339FractionalSecondsAreOnlyRarelyUsedOption,
    Rfc3339GeneratorsShouldUseUppercaseTAndZ, Rfc3339LeapSecondGenerationRequiresPriorAnnouncement,
    Rfc3339LexicalOrderingRequiresUniformFractionalSecondDigits,
    Rfc3339LexicalOrderingRequiresUniformUtcRelationshipEncoding, Rfc3339LocalOffsetNotUnknown,
    Rfc3339OffsetIsUtcOrNumeric, Rfc3339ProfileMakesMostFieldsAndPunctuationMandatory,
    Rfc3339RequiresUtcRelationship, Rfc3339UnknownLocalOffsetUsesNegativeZero,
    Rfc3339UnqualifiedLocalTimeForbidden, Rfc3339UsesExtendedCalendarDate,
    Rfc3339UsesFourDigitYear, Rfc3339UsesFullTime, RoundingModeDeclared,
    SeasonCodeDeclaresNamedSeason, SeasonCodeDeclaresSeasonScope,
    SeasonalExpressionUsesSeasonCodeInMonthSlot, SeasonalExpressionUsesYearAndSeasonForm,
    SecondInRangeZeroToSixty, SelectionExpressionMaySelectSingleInstance,
    SelectionExpressionUsesRecognizedSelectionRuleVocabulary,
    SelectionExpressionUsesSelectionDelimiters, SelectionRulePositionAppliesLast,
    SelectionRulesApplyWithinSelectedResults, SelectionWithDurationUsesDurationSuffix,
    SignificantDigitYearCountIsPositiveInteger, SignificantDigitYearUsesTrailingSSuffix,
    SpeculativeDurationSemanticsDeclared, StandardTimeDerivedFromUtcByLocalShift,
    StandardTimeOfDayUsesStandardTimeScale, SubYearGroupingCodeDeclaresQuadrimester,
    SubYearGroupingCodeDeclaresQuarter, SubYearGroupingCodeDeclaresSemestral,
    SubYearGroupingExpressionUsesGroupingCodeInMonthSlot,
    SubYearGroupingExpressionUsesYearAndGroupingForm, SubsecondDigitsPreserved,
    TemporalSetCarriesMultipleMembers, TemporalSetForbidsInternalWhitespace,
    TemporalSetMemberSeparatorDeclared, TemporalSetOpenRangeUsesBoundaryDoubleDot,
    TemporalSetRangeNeighborhoodSharesPrecision, TemporalSetRangeUsesInclusiveDoubleDotSemantics,
    TimeIntervalBoundaryOrDurationFormDeclared, TimeIntervalHasTwoComponents,
    TimeIntervalUsesSolidusSeparator, TimeOfDayOccursWithinCalendarDay,
    TimeShiftIsConstantDurationBetweenTimeScales, TimestampHasExplicitUtcOffset,
    TwentyFourHourRequiresZeroMinuteSecondAndFraction, TwentyFourHourReservedForEndOfDay,
    UncertaintyAndApproximationMayBeCombined, UncertaintyQualificationDeclared,
    UnknownIntervalBoundaryDeclared, UnknownNamedTimeZoneIdentifierTreatedAsInconsistency,
    UnspecifiedDigitUsesUppercaseXPlaceholder, UnspecifiedDigitsDeclareUnknownValue,
    UtcDesignatorIsUppercaseZ, UtcDifferenceMinutesOmittedOnlyForIntegralHourOffsets,
    UtcIsReferenceTimeScale, UtcOfDayIdentifiesTimeWithinUtcCalendarDay,
    UtcOfDayUsesTrailingZuluDesignatorImmediately, UtcOffsetCarriesSignHourAndOptionalMinute,
    UtcOffsetHourInRangeZeroToTwentyThree, UtcOffsetMinuteInRangeZeroToFiftyNine,
    UtcTimelineOrderingAppliesToFixedInstants, WeekDateHasWeekYearWeekAndWeekday,
    WeekNumberInRangeOneToFiftyThree, WeekdayInRangeOneToSeven,
    ZoneOffsetResolvedForRepresentedInstant, ZoneTransitionAmbiguityDeclared,
    ZoneTransitionDisambiguationAuthorityDeclared, ZoneTransitionGapDeclared,
    ZoneTransitionGapHandlingAuthorityDeclared,
};

macro_rules! structural_prop {
    ($t:ty, $name:literal) => {
        impl Prop for $t {
            fn kani_proof() -> TokenStream {
                quote! { /* structural: #name — temporal proof composition */ }
            }
            fn verus_proof() -> TokenStream {
                quote! { /* structural: #name — temporal proof composition */ }
            }
            fn creusot_proof() -> TokenStream {
                quote! { /* structural: #name — temporal proof composition */ }
            }
        }
    };
}

/// Aggregate proof that a complete calendar date is structurally valid.
pub struct CalendarDateValid;
structural_prop!(CalendarDateValid, "CalendarDateValid");

/// Aggregate proof that a reduced-precision calendar date is structurally valid.
pub struct ReducedCalendarDateValid;
structural_prop!(ReducedCalendarDateValid, "ReducedCalendarDateValid");

/// Aggregate proof that an ISO 8601 date representation is structurally valid.
pub struct DateValid;
structural_prop!(DateValid, "DateValid");

/// Aggregate proof that an ISO 8601 time-of-day representation is structurally valid.
pub struct TimeValid;
structural_prop!(TimeValid, "TimeValid");

/// Aggregate proof that a Gregorian calendar decade representation is structurally valid.
pub struct DecadeValid;
structural_prop!(DecadeValid, "DecadeValid");

/// Aggregate proof that a Gregorian calendar century representation is structurally valid.
pub struct CenturyValid;
structural_prop!(CenturyValid, "CenturyValid");

/// Aggregate proof that an ISO 8601-2 extended year form is structurally valid.
pub struct ExtendedYearValid;
structural_prop!(ExtendedYearValid, "ExtendedYearValid");

/// Aggregate proof that a complete ordinal date is structurally valid.
pub struct OrdinalDateValid;
structural_prop!(OrdinalDateValid, "OrdinalDateValid");

/// Aggregate proof that a complete week date is structurally valid.
pub struct WeekDateValid;
structural_prop!(WeekDateValid, "WeekDateValid");

/// Aggregate proof that a local time representation is structurally valid.
pub struct LocalTimeValid;
structural_prop!(LocalTimeValid, "LocalTimeValid");

/// Aggregate proof that a reduced-accuracy local time representation is structurally valid.
pub struct ReducedLocalTimeValid;
structural_prop!(ReducedLocalTimeValid, "ReducedLocalTimeValid");

/// Aggregate proof that the UTC reference time scale semantics are established.
pub struct UtcTimeScaleValid;
structural_prop!(UtcTimeScaleValid, "UtcTimeScaleValid");

/// Aggregate proof that a UTC-of-day representation is structurally valid.
pub struct UtcOfDayValid;
structural_prop!(UtcOfDayValid, "UtcOfDayValid");

/// Aggregate proof that a numeric UTC offset is structurally valid.
pub struct UtcOffsetValid;
structural_prop!(UtcOffsetValid, "UtcOffsetValid");

/// Aggregate proof that the local offset is known rather than `-00:00`.
pub struct UtcOffsetKnown;
structural_prop!(UtcOffsetKnown, "UtcOffsetKnown");

/// Aggregate proof that standard-time semantics are established.
pub struct StandardTimeValid;
structural_prop!(StandardTimeValid, "StandardTimeValid");

/// Aggregate proof that a standard-time-of-day representation is structurally valid.
pub struct StandardTimeOfDayValid;
structural_prop!(StandardTimeOfDayValid, "StandardTimeOfDayValid");

/// Aggregate proof that a local time-scale interpretation is established.
pub struct LocalTimeScaleValid;
structural_prop!(LocalTimeScaleValid, "LocalTimeScaleValid");

/// Aggregate proof that local-time semantics are established.
pub struct LocalTimeSemanticsValid;
structural_prop!(LocalTimeSemanticsValid, "LocalTimeSemanticsValid");

/// Aggregate proof that a combined local date-time representation is structurally valid.
pub struct LocalDateTimeValid;
structural_prop!(LocalDateTimeValid, "LocalDateTimeValid");

/// Aggregate proof that an offset date-time representation is structurally valid.
pub struct OffsetDateTimeValid;
structural_prop!(OffsetDateTimeValid, "OffsetDateTimeValid");

/// Aggregate proof that a complete explicit-form date with shift is structurally valid.
pub struct DateWithShiftValid;
structural_prop!(DateWithShiftValid, "DateWithShiftValid");

/// Aggregate proof that a complete explicit-form time of day with shift is structurally valid.
pub struct TimeOfDayWithShiftValid;
structural_prop!(TimeOfDayWithShiftValid, "TimeOfDayWithShiftValid");

/// Aggregate proof that a timestamp denotes a single fixed instant.
pub struct TimestampRepresentsFixedInstant;
structural_prop!(
    TimestampRepresentsFixedInstant,
    "TimestampRepresentsFixedInstant"
);

/// Aggregate proof that a local date-time does not, by itself, identify a fixed instant.
pub struct LocalDateTimeDoesNotIdentifyFixedInstant;
structural_prop!(
    LocalDateTimeDoesNotIdentifyFixedInstant,
    "LocalDateTimeDoesNotIdentifyFixedInstant"
);

/// Aggregate proof that a standards-governed mutual-agreement authority is explicit and lawful.
pub struct MutualAgreementAuthorityValid;
structural_prop!(
    MutualAgreementAuthorityValid,
    "MutualAgreementAuthorityValid"
);

/// Aggregate proof that explicit local-to-zone resolution authority is fully declared.
pub struct ZoneTransitionResolutionAuthorityValid;
structural_prop!(
    ZoneTransitionResolutionAuthorityValid,
    "ZoneTransitionResolutionAuthorityValid"
);

/// Aggregate proof that ambiguous local-time resolution semantics are explicit and lawful.
pub struct ZoneTransitionAmbiguitySemanticsValid;
structural_prop!(
    ZoneTransitionAmbiguitySemanticsValid,
    "ZoneTransitionAmbiguitySemanticsValid"
);

/// Aggregate proof that skipped local-time gap semantics are explicit and lawful.
pub struct ZoneTransitionGapSemanticsValid;
structural_prop!(
    ZoneTransitionGapSemanticsValid,
    "ZoneTransitionGapSemanticsValid"
);

/// Aggregate proof that an RFC 3339 timestamp is structurally valid.
pub struct Rfc3339TimestampValid;
structural_prop!(Rfc3339TimestampValid, "Rfc3339TimestampValid");

/// Aggregate proof that RFC 3339 lexical ordering preconditions are structurally valid.
pub struct Rfc3339LexicalOrderingSemanticsValid;
structural_prop!(
    Rfc3339LexicalOrderingSemanticsValid,
    "Rfc3339LexicalOrderingSemanticsValid"
);

/// Aggregate proof that RFC 3339 generation guidance is structurally valid.
pub struct Rfc3339GenerationGuidanceValid;
structural_prop!(
    Rfc3339GenerationGuidanceValid,
    "Rfc3339GenerationGuidanceValid"
);

/// Aggregate proof that an RFC 9557 IXDTF timestamp is structurally valid.
pub struct IxdtfTimestampValid;
structural_prop!(IxdtfTimestampValid, "IxdtfTimestampValid");

/// Aggregate proof that an IXDTF timestamp declares a preferred presentation calendar.
pub struct IxdtfTimestampHasPreferredPresentationCalendar;
structural_prop!(
    IxdtfTimestampHasPreferredPresentationCalendar,
    "IxdtfTimestampHasPreferredPresentationCalendar"
);

/// Aggregate proof that IXDTF additional-information semantics are structurally valid.
pub struct IxdtfAdditionalInformationSemanticsValid;
structural_prop!(
    IxdtfAdditionalInformationSemanticsValid,
    "IxdtfAdditionalInformationSemanticsValid"
);

/// Aggregate proof that a timestamp carries named-zone identity.
pub struct ZonedDateTimeHasNamedZone;
structural_prop!(ZonedDateTimeHasNamedZone, "ZonedDateTimeHasNamedZone");

/// Aggregate proof that the timestamp offset agrees with the named-zone rules.
pub struct OffsetConsistentWithNamedZone;
structural_prop!(
    OffsetConsistentWithNamedZone,
    "OffsetConsistentWithNamedZone"
);

/// Aggregate proof that offset-only semantics remain weaker than named-zone semantics.
pub struct OffsetOnlyZoneSemanticsLimited;
structural_prop!(
    OffsetOnlyZoneSemanticsLimited,
    "OffsetOnlyZoneSemanticsLimited"
);

/// Aggregate proof that an offset time-zone annotation is consistent with the timestamp.
pub struct OffsetTimeZoneAnnotationConsistentWithTimestamp;
structural_prop!(
    OffsetTimeZoneAnnotationConsistentWithTimestamp,
    "OffsetTimeZoneAnnotationConsistentWithTimestamp"
);

/// Aggregate proof that named-zone interpretation tracks current TZDB revision semantics.
pub struct NamedTimeZoneInterpretationTracksTzdbRevision;
structural_prop!(
    NamedTimeZoneInterpretationTracksTzdbRevision,
    "NamedTimeZoneInterpretationTracksTzdbRevision"
);

/// Aggregate proof that subsecond precision was preserved.
pub struct PrecisionPreserved;
structural_prop!(PrecisionPreserved, "PrecisionPreserved");

/// Aggregate proof that a conversion preserved ordering on the UTC timeline.
pub struct TemporalOrderingPreserved;
structural_prop!(TemporalOrderingPreserved, "TemporalOrderingPreserved");

/// Aggregate proof that backend-conversion semantics are explicitly declared.
pub struct BackendConversionSemanticsValid;
structural_prop!(
    BackendConversionSemanticsValid,
    "BackendConversionSemanticsValid"
);

/// Aggregate proof that lossy conversion authority is explicit and fully described.
pub struct LossyConversionAuthorityValid;
structural_prop!(
    LossyConversionAuthorityValid,
    "LossyConversionAuthorityValid"
);

/// Aggregate proof that a conversion is lossless.
pub struct ConversionLossless;
structural_prop!(ConversionLossless, "ConversionLossless");

/// Aggregate proof that a conversion truncates subseconds.
pub struct ConversionTruncatesSubseconds;
structural_prop!(
    ConversionTruncatesSubseconds,
    "ConversionTruncatesSubseconds"
);

/// Aggregate proof that a duration representation is structurally valid.
pub struct DurationFormValid;
structural_prop!(DurationFormValid, "DurationFormValid");

/// Aggregate proof that a time interval representation is structurally valid.
pub struct TimeIntervalValid;
structural_prop!(TimeIntervalValid, "TimeIntervalValid");

/// Aggregate proof that interval endpoints are chronologically ordered.
pub struct IntervalEndpointsOrdered;
structural_prop!(IntervalEndpointsOrdered, "IntervalEndpointsOrdered");

/// Aggregate proof that a recurring interval representation is structurally valid.
pub struct RecurringIntervalFormValid;
structural_prop!(RecurringIntervalFormValid, "RecurringIntervalFormValid");

/// Aggregate proof that an extended temporal qualification is structurally valid.
pub struct QualifiedTemporalExpressionValid;
structural_prop!(
    QualifiedTemporalExpressionValid,
    "QualifiedTemporalExpressionValid"
);

/// Aggregate proof that a temporal value and its qualification sidecar form a lawful exchange.
pub struct QualifiedTemporalValueValid;
structural_prop!(QualifiedTemporalValueValid, "QualifiedTemporalValueValid");

/// Aggregate proof that a CalConnect explicit form is structurally valid.
pub struct ExplicitTemporalFormValid;
structural_prop!(ExplicitTemporalFormValid, "ExplicitTemporalFormValid");

/// Aggregate proof that a CalConnect explicit local-time-of-day form is structurally valid.
pub struct ExplicitTimeOfDayValid;
structural_prop!(ExplicitTimeOfDayValid, "ExplicitTimeOfDayValid");

/// Aggregate proof that a CalConnect explicit time-shift form is structurally valid.
pub struct ExplicitTimeShiftValid;
structural_prop!(ExplicitTimeShiftValid, "ExplicitTimeShiftValid");

/// Aggregate proof that a CalConnect explicit date-time form is structurally valid.
pub struct ExplicitDateTimeValid;
structural_prop!(ExplicitDateTimeValid, "ExplicitDateTimeValid");

/// Aggregate proof that a CalConnect explicit date-time-with-shift form is structurally valid.
pub struct ExplicitDateTimeWithShiftValid;
structural_prop!(
    ExplicitDateTimeWithShiftValid,
    "ExplicitDateTimeWithShiftValid"
);

/// Aggregate proof that a CalConnect explicit duration form is structurally valid.
pub struct ExplicitDurationValid;
structural_prop!(ExplicitDurationValid, "ExplicitDurationValid");

/// Aggregate proof that extended interval boundary semantics are structurally valid.
pub struct ExtendedIntervalBoundarySemanticsValid;
structural_prop!(
    ExtendedIntervalBoundarySemanticsValid,
    "ExtendedIntervalBoundarySemanticsValid"
);

/// Aggregate proof that a grouped time scale unit expression is structurally valid.
pub struct GroupedTimeScaleUnitValid;
structural_prop!(GroupedTimeScaleUnitValid, "GroupedTimeScaleUnitValid");

/// Aggregate proof that a date-time formula is structurally valid.
pub struct DateTimeFormulaValid;
structural_prop!(DateTimeFormulaValid, "DateTimeFormulaValid");

/// Aggregate proof that date-time formula evaluation semantics are fully declared.
pub struct DateTimeFormulaEvaluationSemanticsValid;
structural_prop!(
    DateTimeFormulaEvaluationSemanticsValid,
    "DateTimeFormulaEvaluationSemanticsValid"
);

/// Aggregate proof that a temporal set expression is structurally valid.
pub struct TemporalSetExpressionValid;
structural_prop!(TemporalSetExpressionValid, "TemporalSetExpressionValid");

/// Aggregate proof that a seasonal temporal expression is structurally valid.
pub struct SeasonalTemporalExpressionValid;
structural_prop!(
    SeasonalTemporalExpressionValid,
    "SeasonalTemporalExpressionValid"
);

/// Aggregate proof that a Level 2 sub-year grouping expression is structurally valid.
pub struct SubYearGroupingExpressionValid;
structural_prop!(
    SubYearGroupingExpressionValid,
    "SubYearGroupingExpressionValid"
);

/// Aggregate proof that masked precision and unspecified-component semantics are structurally valid.
pub struct UnspecifiedComponentExpressionValid;
structural_prop!(
    UnspecifiedComponentExpressionValid,
    "UnspecifiedComponentExpressionValid"
);

/// Aggregate proof that refined temporal-set range semantics are structurally valid.
pub struct TemporalSetRangeSemanticsValid;
structural_prop!(
    TemporalSetRangeSemanticsValid,
    "TemporalSetRangeSemanticsValid"
);

/// Aggregate proof that a selection expression is structurally valid.
pub struct SelectionExpressionValid;
structural_prop!(SelectionExpressionValid, "SelectionExpressionValid");

/// Aggregate proof that a repeat rule is structurally valid.
pub struct RepeatRuleValid;
structural_prop!(RepeatRuleValid, "RepeatRuleValid");

/// Aggregate proof that a recurring interval with repeat-rule refinement is structurally valid.
pub struct RecurringIntervalWithRepeatRuleValid;
structural_prop!(
    RecurringIntervalWithRepeatRuleValid,
    "RecurringIntervalWithRepeatRuleValid"
);

/// Evidence bundle for a valid calendar date.
///
/// Normative source: ISO 8601-1:2019, 5.2.2.
pub struct CalendarDateEvidence {
    /// The representation is a Gregorian calendar date.
    pub calendar: Established<CalendarDateUsesGregorianCalendar>,
    /// The representation carries year, month, and day fields.
    pub shape: Established<CalendarDateHasYearMonthDay>,
    /// The month is within the legal ISO 8601 range.
    pub month: Established<CalendarMonthInRangeOneToTwelve>,
    /// The day is legal for the specific month and year.
    pub day: Established<CalendarDayWithinMonthBounds>,
    /// Leap-day usage satisfies the Gregorian leap-year rule.
    pub leap_day: Established<LeapDayOccursOnlyInLeapYear>,
}

/// Evidence branch for the declared reduced calendar-date precision.
///
/// Normative source: ISO 8601-1:2019 — reduced precision calendar date representation
pub enum ReducedCalendarDatePrecisionEvidence {
    /// The reduced calendar date uses the year-only form.
    YearOnly {
        /// The representation carries a calendar year component.
        year: Established<ReducedCalendarDateHasYearComponent>,
        /// The representation uses the year-only lexical form.
        form: Established<ReducedCalendarDateUsesYearOnlyRepresentation>,
    },
    /// The reduced calendar date uses the year-month form.
    YearMonth {
        /// The representation carries a calendar year component.
        year: Established<ReducedCalendarDateHasYearComponent>,
        /// The representation uses the year-month lexical form.
        form: Established<ReducedCalendarDateUsesYearMonthRepresentation>,
        /// The month is within the legal ISO 8601 range.
        month: Established<CalendarMonthInRangeOneToTwelve>,
    },
}

/// Evidence bundle for a valid reduced-precision calendar date.
///
/// Normative source: ISO 8601-1:2019 — reduced precision calendar date representation
pub struct ReducedCalendarDateEvidence {
    /// The representation is a Gregorian calendar date.
    pub calendar: Established<CalendarDateUsesGregorianCalendar>,
    /// The declared precision branch is structurally valid.
    pub precision: ReducedCalendarDatePrecisionEvidence,
}

/// Evidence bundle for a valid Gregorian calendar decade.
///
/// Normative sources: ISO 8601-1:2019/Amd 1:2022, 4.3.11; ISO 8601-2:2019,
/// 4.3.5.
pub struct DecadeEvidence {
    /// The decade ordinal is within the legal ISO range.
    pub ordinal: Established<DecadeOrdinalInRangeZeroToNineHundredNinetyNine>,
    /// Before-year-one syntax, when used, is declared by a trailing `B` suffix.
    pub before_year_one: Option<Established<BeforeYearOneValueUsesTrailingBSuffix>>,
}

/// Evidence bundle for a valid Gregorian calendar century.
///
/// Normative sources: ISO 8601-1:2019/Amd 1:2022, 4.3.12; ISO 8601-2:2019,
/// 4.3.6.
pub struct CenturyEvidence {
    /// The century ordinal is within the legal ISO range.
    pub ordinal: Established<CenturyOrdinalInRangeZeroToNinetyNine>,
    /// Before-year-one syntax, when used, is declared by a trailing `B` suffix.
    pub before_year_one: Option<Established<BeforeYearOneValueUsesTrailingBSuffix>>,
}

/// Evidence branch for the significant-digits suffix on an ISO 8601-2 year form.
///
/// Normative source: ISO 8601-2:2019 — significant digits
/// Informative cross-check: public LOC EDTF Level 2 — Significant digits
pub struct ExtendedYearSignificantDigitsEvidence {
    /// The year form uses a trailing uppercase `S` suffix.
    pub suffix: Established<SignificantDigitYearUsesTrailingSSuffix>,
    /// The significant-digit count is a positive integer.
    pub count: Established<SignificantDigitYearCountIsPositiveInteger>,
}

/// Evidence branch for the base lexical form of an ISO 8601-2 year extension.
///
/// Normative source: ISO 8601-2:2019 — letter-prefixed, negative, and exponential year forms
/// Informative cross-check: public LOC EDTF Level 1 — Letter-prefixed calendar year;
/// Negative calendar year. Level 2 — Exponential year
pub enum ExtendedYearBaseEvidence {
    /// A four-digit year form, optionally carrying the negative-year lexical law.
    FourDigit {
        /// The representation uses an explicit leading minus sign when negative.
        negative: Option<Established<NegativeCalendarYearUsesLeadingMinusSign>>,
    },
    /// A letter-prefixed year form with an integer year payload.
    LetterPrefixed {
        /// The representation uses the leading `Y` designator.
        prefix: Established<LetterPrefixedCalendarYearUsesLeadingYDesignator>,
        /// The absolute year magnitude exceeds four digits.
        magnitude: Established<LetterPrefixedCalendarYearMagnitudeExceedsFourDigits>,
        /// The representation uses an explicit leading minus sign when negative.
        negative: Option<Established<NegativeCalendarYearUsesLeadingMinusSign>>,
    },
    /// A letter-prefixed year form with exponential notation.
    Exponential {
        /// The representation uses the leading `Y` designator.
        prefix: Established<LetterPrefixedCalendarYearUsesLeadingYDesignator>,
        /// The representation uses `E` power-of-ten notation.
        exponential: Established<ExponentialYearUsesPowerOfTenNotation>,
        /// The exponent is a positive integer.
        exponent: Established<ExponentialYearExponentIsPositiveInteger>,
        /// The representation uses an explicit leading minus sign when negative.
        negative: Option<Established<NegativeCalendarYearUsesLeadingMinusSign>>,
    },
}

/// Evidence bundle for a valid ISO 8601-2 extended year form.
///
/// Normative source: ISO 8601-2:2019 — letter-prefixed, negative, exponential, and significant-digit year forms
/// Informative cross-check: public LOC EDTF Level 1 — Letter-prefixed calendar year;
/// Negative calendar year. Level 2 — Exponential year; Significant digits
pub struct ExtendedYearEvidence {
    /// The base year form is structurally valid.
    pub base: ExtendedYearBaseEvidence,
    /// Optional significant-digit semantics attached to the base form.
    pub significant_digits: Option<ExtendedYearSignificantDigitsEvidence>,
}

/// Evidence bundle for a valid ordinal date.
///
/// Normative source: ISO 8601-1:2019, 5.2.3.
pub struct OrdinalDateEvidence {
    /// The representation carries year and day-of-year fields.
    pub shape: Established<OrdinalDateHasYearAndDayOfYear>,
    /// The ordinal day is in range for the target year.
    pub day: Established<OrdinalDayInRangeOneToThreeHundredSixtySix>,
}

/// Evidence bundle for a valid week date.
///
/// Normative source: ISO 8601-1:2019, 5.2.4.
pub struct WeekDateEvidence {
    /// The representation carries week-year, week, and weekday fields.
    pub shape: Established<WeekDateHasWeekYearWeekAndWeekday>,
    /// The week number is in the legal ISO 8601 range.
    pub week: Established<WeekNumberInRangeOneToFiftyThree>,
    /// The weekday is in the legal ISO 8601 range.
    pub weekday: Established<WeekdayInRangeOneToSeven>,
}

/// Evidence branch for an ISO 8601 date representation.
///
/// Normative source: ISO 8601-1:2019 — date
pub enum DateEvidence {
    /// Gregorian calendar-date representation.
    Calendar {
        /// The calendar-date representation is structurally valid.
        date: Established<CalendarDateValid>,
        /// A date identifies a position within the calendar.
        semantics: Established<DateIdentifiesPositionWithinCalendar>,
    },
    /// Ordinal-date representation.
    Ordinal {
        /// The ordinal-date representation is structurally valid.
        date: Established<OrdinalDateValid>,
        /// A date identifies a position within the calendar.
        semantics: Established<DateIdentifiesPositionWithinCalendar>,
    },
    /// Week-date representation.
    Week {
        /// The week-date representation is structurally valid.
        date: Established<WeekDateValid>,
        /// A date identifies a position within the calendar.
        semantics: Established<DateIdentifiesPositionWithinCalendar>,
    },
}

/// Evidence bundle for a valid local time.
///
/// Normative sources: ISO 8601-1:2019, 5.3.1; ISO 8601-1:2019/Amd 1:2022,
/// 5.3.1.4 and 5.3.2.
pub struct LocalTimeEvidence {
    /// The representation carries hour, minute, and second fields.
    pub shape: Established<LocalTimeHasHourMinuteSecond>,
    /// The hour value is in range.
    pub hour: Established<HourInRangeZeroToTwentyFour>,
    /// The minute value is in range.
    pub minute: Established<MinuteInRangeZeroToFiftyNine>,
    /// The second value is in range.
    pub second: Established<SecondInRangeZeroToSixty>,
    /// End-of-day usage of hour 24 is legal.
    pub end_of_day: Established<TwentyFourHourReservedForEndOfDay>,
    /// End-of-day hour 24 uses terminal zero minute, second, and fraction values only.
    pub terminal_end_of_day: Established<TwentyFourHourRequiresZeroMinuteSecondAndFraction>,
    /// Leap-second usage is legal.
    pub leap_second: Established<LeapSecondOccursOnlyAtUtcBoundary>,
    /// Fraction syntax uses an ISO 8601 decimal sign.
    pub fraction_sign: Established<FractionUsesDecimalSign>,
    /// Any fraction attaches to the lowest-order present component.
    pub fraction_target: Established<FractionAppliesToLowestOrderComponent>,
}

/// Evidence branch for the declared reduced local-time precision.
///
/// Normative source: ISO 8601-1:2019 — reduced accuracy local time representation
pub enum ReducedLocalTimePrecisionEvidence {
    /// The reduced local time uses the hour-only form.
    HourOnly {
        /// The representation uses the hour-only lexical form.
        form: Established<ReducedAccuracyLocalTimeUsesHourOnlyRepresentation>,
    },
    /// The reduced local time uses the hour-minute form.
    HourMinute {
        /// The representation uses the hour-minute lexical form.
        form: Established<ReducedAccuracyLocalTimeUsesHourMinuteRepresentation>,
        /// The minute is within the legal ISO 8601 range.
        minute: Established<MinuteInRangeZeroToFiftyNine>,
    },
}

/// Evidence bundle for a valid reduced-accuracy local time.
///
/// Normative sources: ISO 8601-1:2019 — reduced-accuracy and decimal-fraction local time representations;
/// ISO 8601-1:2019/Amd 1:2022 — end-of-day technical correction
pub struct ReducedLocalTimeEvidence {
    /// The hour value is in range.
    pub hour: Established<HourInRangeZeroToTwentyFour>,
    /// The declared precision branch is structurally valid.
    pub precision: ReducedLocalTimePrecisionEvidence,
    /// End-of-day usage of hour 24 is legal.
    pub end_of_day: Established<TwentyFourHourReservedForEndOfDay>,
    /// End-of-day hour 24 uses terminal zero minute, second, and fraction values only.
    pub terminal_end_of_day: Established<TwentyFourHourRequiresZeroMinuteSecondAndFraction>,
    /// Fraction syntax uses an ISO 8601 decimal sign.
    pub fraction_sign: Established<FractionUsesDecimalSign>,
    /// Any fraction attaches to the lowest-order present component.
    pub fraction_target: Established<FractionAppliesToLowestOrderComponent>,
}

/// Evidence branch for the declared numeric UTC-offset precision.
///
/// Normative source: ISO 8601-1:2019, 5.3.4.
/// Open-text cross-check: ISO/WD 8601-1:2016(E), 4.2.5.1.
pub enum UtcOffsetPrecisionEvidence {
    /// The offset is expressed with hours only.
    HourOnly {
        /// Minute omission is legal only for integral-hour offsets.
        omitted_minutes: Established<UtcDifferenceMinutesOmittedOnlyForIntegralHourOffsets>,
    },
    /// The offset is expressed with hour-and-minute precision.
    HourMinute {
        /// The offset minute is in range.
        minute: Established<UtcOffsetMinuteInRangeZeroToFiftyNine>,
    },
}

/// Evidence bundle for a valid numeric UTC offset.
///
/// Normative source: ISO 8601-1:2019, 5.3.4.
/// Open-text cross-check: ISO/WD 8601-1:2016(E), 4.2.5.1.
pub struct UtcOffsetEvidence {
    /// The offset includes a sign, an hour component, and optional minute precision.
    pub shape: Established<UtcOffsetCarriesSignHourAndOptionalMinute>,
    /// The offset hour is in range.
    pub hour: Established<UtcOffsetHourInRangeZeroToTwentyThree>,
    /// The declared precision branch is structurally valid.
    pub precision: UtcOffsetPrecisionEvidence,
}

/// Evidence bundle for the UTC reference time scale.
///
/// Normative source: ISO 8601-1:2019, 3.1.1.12
pub struct UtcTimeScaleEvidence {
    /// UTC acts as the reference time scale.
    pub reference: Established<UtcIsReferenceTimeScale>,
}

/// Evidence bundle for a valid UTC of day.
///
/// Normative source: ISO 8601-1:2019, 3.1.1.13
pub struct UtcOfDayEvidence {
    /// The carried time-of-day representation is structurally valid.
    pub time: Established<LocalTimeValid>,
    /// The representation is anchored to the UTC reference time scale.
    pub scale: Established<UtcTimeScaleValid>,
    /// The value identifies a position within a UTC calendar day.
    pub position: Established<UtcOfDayIdentifiesTimeWithinUtcCalendarDay>,
    /// The UTC time-of-day payload is followed immediately by the `Z` designator.
    pub designator: Established<UtcOfDayUsesTrailingZuluDesignatorImmediately>,
    /// The UTC designator uses uppercase `Z`.
    pub uppercase_z: Established<UtcDesignatorIsUppercaseZ>,
}

/// Evidence bundle for a known local offset.
///
/// Normative source: RFC 3339 §4.3 — Unknown Local Offset Convention
pub struct KnownUtcOffsetEvidence {
    /// The timestamp already carries a valid UTC-offset representation.
    pub offset: Established<UtcOffsetValid>,
    /// The timestamp does not use the unknown local-offset convention.
    pub known_convention: Established<Rfc3339LocalOffsetNotUnknown>,
}

/// Evidence bundle for standard-time semantics derived from UTC.
///
/// Normative source: ISO 8601-1:2019, 3.1.1.14
pub struct StandardTimeEvidence {
    /// The reference time scale is UTC.
    pub reference: Established<UtcTimeScaleValid>,
    /// The local shift from UTC is carried explicitly.
    pub shift: Established<UtcOffsetValid>,
    /// Standard time is derived from UTC by an applicable local shift.
    pub derivation: Established<StandardTimeDerivedFromUtcByLocalShift>,
    /// The shift is a constant duration between the time scales.
    pub constant_shift: Established<TimeShiftIsConstantDurationBetweenTimeScales>,
}

/// Evidence bundle for a valid standard time of day.
///
/// Normative source: ISO/WD 8601-1:2016(E), 2.1.15
pub struct StandardTimeOfDayEvidence {
    /// The wall-clock time-of-day representation is structurally valid.
    pub time: Established<LocalTimeValid>,
    /// The governing standard-time scale is established.
    pub scale: Established<StandardTimeValid>,
    /// The time-of-day interpretation uses standard time.
    pub semantics: Established<StandardTimeOfDayUsesStandardTimeScale>,
}

/// Evidence branch for the locally applicable time scale carried by a local time.
///
/// Normative source: ISO 8601-1:2019, 3.1.1.15
pub enum LocalTimeScaleEvidence {
    /// The local time uses a UTC-derived standard-time scale.
    Standard {
        /// The standard-time interpretation is established explicitly.
        standard_time: Established<StandardTimeValid>,
        /// The standard permits local time scales to be UTC-derived standard time.
        locality: Established<LocalTimeScaleMayBeStandardOrNonUtcBased>,
    },
    /// The local time uses another non-UTC-based local time scale.
    NonUtcBased {
        /// The standard permits local time scales to be non-UTC-based.
        locality: Established<LocalTimeScaleMayBeStandardOrNonUtcBased>,
    },
}

/// Evidence bundle for local-time semantics.
///
/// The Rust `LocalTime*` family models ISO 8601-1:2019 local time of day.
///
/// Normative source: ISO 8601-1:2019, 3.1.1.17
pub struct LocalTimeSemanticsEvidence {
    /// The wall-clock time-of-day representation is structurally valid.
    pub time: Established<LocalTimeValid>,
    /// The locally applicable time scale is established explicitly.
    pub scale: Established<LocalTimeScaleValid>,
    /// The time is interpreted against a locally applicable time scale.
    pub semantics: Established<LocalTimeUsesLocallyApplicableTimeScale>,
}

/// Evidence branch for an ISO 8601 time-of-day representation.
///
/// Normative source: ISO 8601-1:2019, 3.1.1.16
pub enum TimeEvidence {
    /// Local clock-time representation.
    Local {
        /// The local clock-time representation is structurally valid.
        time: Established<LocalTimeValid>,
        /// A time of day occurs within a calendar day.
        semantics: Established<TimeOfDayOccursWithinCalendarDay>,
    },
    /// Reduced-accuracy local clock-time representation.
    ReducedLocal {
        /// The reduced local-time representation is structurally valid.
        time: Established<ReducedLocalTimeValid>,
        /// A time of day occurs within a calendar day.
        semantics: Established<TimeOfDayOccursWithinCalendarDay>,
    },
    /// UTC-of-day representation.
    Utc {
        /// The UTC-of-day representation is structurally valid.
        time: Established<UtcOfDayValid>,
        /// A time of day occurs within a calendar day.
        semantics: Established<TimeOfDayOccursWithinCalendarDay>,
    },
    /// Standard-time-of-day representation.
    Standard {
        /// The standard-time-of-day representation is structurally valid.
        time: Established<StandardTimeOfDayValid>,
        /// A time of day occurs within a calendar day.
        semantics: Established<TimeOfDayOccursWithinCalendarDay>,
    },
}

/// Evidence branch for the complete date family carried by a combined date-time representation.
///
/// Normative source: ISO 8601-1:2019, 5.4.2 and 5.4.3.
pub enum CombinedDateTimeDateEvidence {
    /// The combined representation uses the calendar-date family.
    Calendar {
        /// The calendar-date component is structurally valid.
        date: Established<CalendarDateValid>,
        /// Clause 4.3 permits the calendar-date family in combined date-time forms.
        family: Established<CombinedDateTimePermitsCalendarDateComponent>,
    },
    /// The combined representation uses the ordinal-date family.
    Ordinal {
        /// The ordinal-date component is structurally valid.
        date: Established<OrdinalDateValid>,
        /// Clause 4.3 permits the ordinal-date family in combined date-time forms.
        family: Established<CombinedDateTimePermitsOrdinalDateComponent>,
    },
    /// The combined representation uses the week-date family.
    Week {
        /// The week-date component is structurally valid.
        date: Established<WeekDateValid>,
        /// Clause 4.3 permits the week-date family in combined date-time forms.
        family: Established<CombinedDateTimePermitsWeekDateComponent>,
    },
}

/// Evidence bundle for a valid combined local date-time.
///
/// Normative source: ISO 8601-1:2019, 5.4.2 and 5.4.3.
pub struct LocalDateTimeEvidence {
    /// The date component is valid and belongs to a lawful combined date-time family.
    pub date: CombinedDateTimeDateEvidence,
    /// The time component is valid.
    pub time: Established<LocalTimeValid>,
    /// The date and time parts are joined with the ISO designator.
    pub separator: Established<CombinedDateTimeUsesTimeDesignator>,
    /// The date branch is complete rather than reduced-accuracy.
    pub date_precision: Established<CombinedDateTimeDateComponentMustNotUseReducedAccuracy>,
    /// The date and time branches use a single ISO format family across the expression.
    pub component_format: Established<CombinedDateTimeUsesSingleFormatAcrossDateAndTimeComponents>,
}

/// Evidence bundle for a valid offset date-time.
///
/// Normative sources: ISO 8601-1:2019, 5.3.4, 5.4.2, and 5.4.3.
pub struct OffsetDateTimeEvidence {
    /// The local date-time component is valid.
    pub local: Established<LocalDateTimeValid>,
    /// The UTC relationship is represented by a valid numeric offset.
    pub offset: Established<UtcOffsetValid>,
    /// The UTC designator, when used instead of a numeric offset, is uppercase `Z`.
    pub utc_designator: Established<UtcDesignatorIsUppercaseZ>,
}

/// Evidence branch for the complete date family carried by an explicit date-with-shift value.
///
/// Normative source: CalConnect CC 18011:2018 §4.3 — Date
pub enum CompleteDateEvidence {
    /// The complete date uses the calendar-date family.
    Calendar(Established<CalendarDateValid>),
    /// The complete date uses the ordinal-date family.
    Ordinal(Established<OrdinalDateValid>),
    /// The complete date uses the week-date family.
    Week(Established<WeekDateValid>),
}

/// Evidence bundle for a valid explicit-form local time of day.
///
/// Normative source: CalConnect CC 18011:2018 §4.3.3 — Local time of day
pub struct ExplicitTimeOfDayEvidence {
    /// The representation uses the leading `T` designator.
    pub designator: Established<ExplicitTimeOfDayUsesTimeDesignator>,
    /// The representation uses explicit hour, minute, and second unit designators.
    pub units: Established<ExplicitTimeOfDayUsesHourMinuteSecondUnitDesignators>,
    /// The hour value is in range.
    pub hour: Established<HourInRangeZeroToTwentyFour>,
    /// The minute value is in range when present.
    pub minute: Option<Established<MinuteInRangeZeroToFiftyNine>>,
    /// The second value is in range when present.
    pub second: Option<Established<SecondInRangeZeroToSixty>>,
    /// Fraction syntax uses an ISO 8601 decimal sign when present.
    pub fraction_sign: Option<Established<FractionUsesDecimalSign>>,
    /// Any fraction attaches to the lowest-order present component.
    pub fraction_target: Option<Established<FractionAppliesToLowestOrderComponent>>,
    /// Zero-valued components may be omitted from the lexical form.
    pub zero_omission: Established<ExplicitTemporalFormMayOmitZeroValuedComponents>,
    /// The lowest denoted component declares the precision.
    pub precision: Established<ExplicitTemporalPrecisionUsesLowestDenotedComponent>,
    /// Explicit local time of day does not use an end-of-day representation.
    pub no_end_of_day: Established<ExplicitTimeOfDayForbidsEndOfDayRepresentation>,
}

/// Evidence bundle for a valid explicit-form time shift.
///
/// Normative source: CalConnect CC 18011:2018 §4.3.4 — Time shift
pub struct ExplicitTimeShiftEvidence {
    /// The representation uses the leading `Z` designator.
    pub designator: Established<ExplicitTimeShiftUsesZuluDesignator>,
    /// A leading minus sign appears only when the shift is behind UTC.
    pub sign: Established<ExplicitTimeShiftUsesLeadingMinusOnlyWhenBehindUtc>,
    /// Any non-empty payload uses the explicit local-time-of-day family.
    pub payload_shape: Established<ExplicitTimeShiftPayloadUsesExplicitTimeOfDay>,
    /// A bare `Z` denotes UTC with zero shift.
    pub bare_utc: Established<ExplicitTimeShiftBareZuluRepresentsUtcZero>,
    /// The explicit time payload when present.
    pub time: Option<Established<ExplicitTimeOfDayValid>>,
}

/// Evidence bundle for a valid explicit-form date-time.
///
/// Normative source: CalConnect CC 18011:2018 §4.3.4.3 — Date and time only
pub struct ExplicitDateTimeEvidence {
    /// The date branch is one of the lawful explicit complete-date families.
    pub date: CompleteDateEvidence,
    /// The time branch is a lawful explicit local-time-of-day form.
    pub time: Established<ExplicitTimeOfDayValid>,
    /// The explicit date precedes the explicit time in the combined representation.
    pub concatenation: Established<ExplicitDateTimeUsesDateThenTimeConcatenation>,
    /// The time branch may use reduced precision.
    pub reduced_precision: Established<ExplicitDateTimeTimePortionMayBeReducedPrecision>,
}

/// Evidence bundle for a valid complete explicit-form date with shift.
///
/// Normative source: CalConnect CC 18011:2018 §4.3 — Date with shift
pub struct DateWithShiftEvidence {
    /// The date branch is one of the lawful explicit complete-date families.
    pub date: CompleteDateEvidence,
    /// The explicit date precedes the explicit time shift.
    pub concatenation: Established<ExplicitDateWithShiftUsesDateThenShiftConcatenation>,
    /// The attached shift is a lawful explicit-form time shift.
    pub shift: Established<ExplicitTimeShiftValid>,
}

/// Evidence bundle for a valid complete explicit-form time of day with shift.
///
/// Normative source: CalConnect CC 18011:2018 §4.3 — Time of day with time shift
pub struct TimeOfDayWithShiftEvidence {
    /// The explicit local-time-of-day branch is structurally valid.
    pub time: Established<ExplicitTimeOfDayValid>,
    /// The explicit time precedes the explicit time shift.
    pub concatenation: Established<ExplicitTimeOfDayWithShiftUsesTimeThenShiftConcatenation>,
    /// The attached shift is a lawful explicit-form time shift.
    pub shift: Established<ExplicitTimeShiftValid>,
}

/// Evidence bundle for a valid complete explicit-form date-time with shift.
///
/// Normative source: CalConnect CC 18011:2018 §4.3.4.3 — Date and time with shift
pub struct ExplicitDateTimeWithShiftEvidence {
    /// The explicit local date-time branch is structurally valid.
    pub local: Established<ExplicitDateTimeValid>,
    /// The explicit date-time precedes the explicit time shift.
    pub concatenation: Established<ExplicitDateTimeWithShiftUsesDateTimeThenShiftConcatenation>,
    /// The attached shift is a lawful explicit-form time shift.
    pub shift: Established<ExplicitTimeShiftValid>,
}

/// Evidence bundle for a fixed instant.
///
/// Normative sources: ISO 8601-1:2019, 5.3.4, 5.4.2, and 5.4.3.
/// Informative cross-check: RFC 3339 §5.6
pub struct FixedInstantEvidence {
    /// The offset date-time representation is structurally valid.
    pub offset_date_time: Established<OffsetDateTimeValid>,
    /// The timestamp carries an explicit UTC relationship.
    pub explicit_offset: Established<TimestampHasExplicitUtcOffset>,
    /// The representation identifies a single instant on the UTC timeline.
    pub single_instant: Established<OffsetDateTimeIdentifiesSingleInstant>,
}

/// Evidence bundle for local timestamp semantics.
///
/// Normative source: RFC 3339 §4.4 — Unqualified Local Time
pub struct LocalTimestampSemanticsEvidence {
    /// The local date-time representation is structurally valid.
    pub local_date_time: Established<LocalDateTimeValid>,
    /// Additional zone or offset authority is required to identify an instant.
    pub requires_authority: Established<LocalDateTimeRequiresZoneOrOffsetForInstant>,
}

/// Evidence branch for a standards-governed mutual-agreement authority scope.
///
/// Normative source: ISO 8601-1:2019 — agreement-governed temporal representations
pub enum MutualAgreementAuthorityScopeEvidence {
    /// Mutual agreement explicitly covers non-expanded calendar years through 1582.
    CalendarYearThrough1582 {
        /// The governed calendar-year restriction is established.
        clause: Established<CalendarYearThrough1582RequiresMutualAgreement>,
    },
    /// Mutual agreement explicitly covers proleptic Gregorian dates before 1583.
    ProlepticGregorianBefore1583 {
        /// The governed proleptic-date restriction is established.
        clause: Established<ProlepticGregorianDatesBefore1583RequireMutualAgreement>,
    },
    /// Mutual agreement explicitly covers expanded representations.
    ExpandedRepresentation {
        /// The governed expanded-representation restriction is established.
        clause: Established<ExpandedRepresentationRequiresAdditionalAgreement>,
    },
}

/// Evidence bundle for explicit mutual-agreement authority.
///
/// Normative source: ISO 8601-1:2019 — agreement-governed temporal representations
pub struct MutualAgreementAuthorityEvidence {
    /// The agreement-governed scope carried by the authority sidecar.
    pub scope: MutualAgreementAuthorityScopeEvidence,
}

/// Evidence bundle for explicit local-to-zone resolution authority.
///
/// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
pub struct ZoneTransitionResolutionAuthorityEvidence {
    /// The named zone is carried by an IANA time-zone identifier.
    pub zone_identifier: Established<NamedTimeZoneUsesIanaIdentifier>,
    /// Ambiguous local times use explicit disambiguation authority.
    pub disambiguation: Established<ZoneTransitionDisambiguationAuthorityDeclared>,
    /// Skipped local times use explicit gap-handling authority.
    pub gap_handling: Established<ZoneTransitionGapHandlingAuthorityDeclared>,
}

/// Evidence bundle for ambiguous local-time resolution semantics.
///
/// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
pub struct ZoneTransitionAmbiguityEvidence {
    /// The local date-time is still awaiting zone authority to identify an instant.
    pub local_semantics: Established<LocalDateTimeDoesNotIdentifyFixedInstant>,
    /// The local date-time can be ambiguous at a transition boundary.
    pub ambiguity_possibility: Established<LocalDateTimeMayBeAmbiguousAtZoneTransition>,
    /// The producer explicitly declares that a transition ambiguity is in play.
    pub ambiguity_declared: Established<ZoneTransitionAmbiguityDeclared>,
    /// The selected disambiguation and gap-handling policy is explicit.
    pub resolution_authority: Established<ZoneTransitionResolutionAuthorityValid>,
}

/// Evidence bundle for skipped local-time gap semantics.
///
/// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
pub struct ZoneTransitionGapEvidence {
    /// The local date-time is still awaiting zone authority to identify an instant.
    pub local_semantics: Established<LocalDateTimeDoesNotIdentifyFixedInstant>,
    /// The local date-time can fall inside a skipped transition gap.
    pub gap_possibility: Established<LocalDateTimeMayFallInZoneTransitionGap>,
    /// The producer explicitly declares that a transition gap is in play.
    pub gap_declared: Established<ZoneTransitionGapDeclared>,
    /// The selected disambiguation and gap-handling policy is explicit.
    pub resolution_authority: Established<ZoneTransitionResolutionAuthorityValid>,
}

/// Evidence bundle for a valid RFC 3339 timestamp.
///
/// Normative sources: RFC 3339 §4.3, §4.4, §5.6
pub struct Rfc3339TimestampEvidence {
    /// The timestamp rests on a valid offset date-time representation.
    pub offset_date_time: Established<OffsetDateTimeValid>,
    /// The year uses the RFC 3339 four-digit profile.
    pub year: Established<Rfc3339UsesFourDigitYear>,
    /// The date uses the RFC 3339 `full-date` profile.
    pub full_date: Established<Rfc3339UsesExtendedCalendarDate>,
    /// The time uses the RFC 3339 `full-time` profile.
    pub full_time: Established<Rfc3339UsesFullTime>,
    /// The timestamp carries an explicit relationship to UTC.
    pub utc_relationship: Established<Rfc3339RequiresUtcRelationship>,
    /// Local time without an offset or `Z` is forbidden.
    pub no_unqualified_local: Established<Rfc3339UnqualifiedLocalTimeForbidden>,
    /// Fractional seconds use `.` rather than the broader ISO 8601 decimal options.
    pub secfrac_separator: Established<Rfc3339FractionUsesDotSeparator>,
    /// The UTC relationship is encoded as `Z` or a numeric offset.
    pub offset_encoding: Established<Rfc3339OffsetIsUtcOrNumeric>,
    /// The unknown local-offset convention uses `-00:00`.
    pub unknown_offset: Established<Rfc3339UnknownLocalOffsetUsesNegativeZero>,
}

/// Evidence bundle for RFC 3339 lexical-ordering semantics.
///
/// Normative sources: RFC 3339 §5.1, §5.5, §5.6
pub struct Rfc3339LexicalOrderingEvidence {
    /// The underlying timestamp shape is a valid RFC 3339 timestamp.
    pub timestamp: Established<Rfc3339TimestampValid>,
    /// Compared timestamps use the same UTC-relationship string form.
    pub uniform_utc_relationship:
        Established<Rfc3339LexicalOrderingRequiresUniformUtcRelationshipEncoding>,
    /// Compared timestamps use the same number of fractional-second digits.
    pub uniform_fractional_digits:
        Established<Rfc3339LexicalOrderingRequiresUniformFractionalSecondDigits>,
    /// Most fields and punctuation are mandatory in the profile.
    pub mandatory_shape: Established<Rfc3339ProfileMakesMostFieldsAndPunctuationMandatory>,
}

/// Evidence bundle for RFC 3339 generation guidance.
///
/// Normative sources: RFC 3339 §5.3, §5.6, §5.7
pub struct Rfc3339GenerationGuidanceEvidence {
    /// Fractional seconds are the only rarely used profile option.
    pub secfrac_rarity: Established<Rfc3339FractionalSecondsAreOnlyRarelyUsedOption>,
    /// Generators should prefer uppercase `T` and `Z`.
    pub uppercase_tz: Established<Rfc3339GeneratorsShouldUseUppercaseTAndZ>,
    /// Inserted leap-second timestamps are not generated before announcement.
    pub leap_second_announcement: Established<Rfc3339LeapSecondGenerationRequiresPriorAnnouncement>,
}

/// Evidence bundle for a valid RFC 9557 IXDTF timestamp.
///
/// Normative sources: RFC 9557 §3.1, §3.2, §3.3
pub struct IxdtfTimestampEvidence {
    /// The base timestamp is a valid RFC 3339 timestamp.
    pub base: Established<Rfc3339TimestampValid>,
    /// Additional information is appended to an RFC 3339 timestamp.
    pub suffix: Established<IxdtfSuffixFollowsRfc3339Timestamp>,
    /// The time-zone annotation uses the RFC 9557 bracketed form.
    pub time_zone: Established<IxdtfTimeZoneSuffixUsesBracketedNameOrOffset>,
    /// Criticality is expressed with a leading `!` when present.
    pub critical: Established<IxdtfCriticalFlagIsLeadingExclamationWhenPresent>,
    /// Additional-information keys obey RFC 9557 casing rules.
    pub key_case: Established<IxdtfSuffixKeysAreLowercase>,
}

/// Evidence bundle for IXDTF preferred-presentation calendar semantics.
///
/// Normative source: RFC 9557 §5 — The u-ca Suffix Key: Calendar Awareness
pub struct IxdtfCalendarAwareTimestampEvidence {
    /// The underlying timestamp is a valid IXDTF timestamp.
    pub timestamp: Established<IxdtfTimestampValid>,
    /// A calendar-awareness annotation is present.
    pub calendar_annotation: Established<IxdtfCalendarAnnotationPresent>,
    /// The calendar-awareness key uses the RFC 9557 `u-ca` token.
    pub calendar_key: Established<IxdtfCalendarKeyUsesUCa>,
    /// The calendar value uses a Unicode calendar identifier.
    pub calendar_identifier: Established<IxdtfCalendarValueUsesUnicodeCalendarIdentifier>,
    /// The annotation declares the preferred calendar for presentation.
    pub preferred_presentation:
        Established<IxdtfCalendarAnnotationDeclaresPreferredPresentationCalendar>,
}

/// Evidence bundle for IXDTF additional-information field semantics.
///
/// Normative sources: RFC 9557 §3.3, §4.1
pub struct IxdtfAdditionalInformationEvidence {
    /// The underlying timestamp is a valid IXDTF timestamp.
    pub timestamp: Established<IxdtfTimestampValid>,
    /// Suffix tags use the bracketed key-value form.
    pub tag_form: Established<IxdtfSuffixTagsUseBracketedKeyValueForm>,
    /// Suffix values use one or more hyphen-delimited items.
    pub value_items: Established<IxdtfSuffixValuesUseHyphenDelimitedItems>,
    /// Suffix keys remain lowercase.
    pub key_case: Established<IxdtfSuffixKeysAreLowercase>,
    /// Suffix values are case-sensitive unless a key says otherwise.
    pub value_case: Established<IxdtfSuffixValuesAreCaseSensitiveUnlessOtherwiseSpecified>,
    /// Generators may omit suffix tags entirely.
    pub optional_generation: Established<IxdtfGeneratorsMayOmitSuffixTags>,
    /// Elective suffix tags may be ignored by recipients.
    pub elective_consumption: Established<IxdtfRecipientsMayIgnoreElectiveSuffixTags>,
    /// Criticality is expressed with a leading `!` when present.
    pub critical_flag: Established<IxdtfCriticalFlagIsLeadingExclamationWhenPresent>,
    /// Critical suffix tags require processing or explicit error handling.
    pub critical_consumption: Established<IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling>,
    /// Experimental suffix keys use a leading underscore.
    pub experimental_key: Established<IxdtfExperimentalSuffixKeysUseLeadingUnderscore>,
    /// Experimental suffix keys are not valid for general interchange.
    pub experimental_interchange: Established<IxdtfExperimentalSuffixKeysAreNotForInterchange>,
    /// Unconfigured recipients reject experimental suffix keys.
    pub experimental_rejection:
        Established<IxdtfRecipientsMustRejectUnconfiguredExperimentalSuffixKeys>,
}

/// Evidence bundle for named-zone identity.
///
/// Normative sources: RFC 9557 §3.1, §3.3
pub struct ZonedTimestampEvidence {
    /// The underlying timestamp is a valid IXDTF timestamp.
    pub timestamp: Established<IxdtfTimestampValid>,
    /// The timestamp carries a named-zone annotation.
    pub zone_annotation: Established<NamedTimeZoneAnnotationPresent>,
    /// The zone annotation uses an IANA zone identifier.
    pub zone_identifier: Established<NamedTimeZoneUsesIanaIdentifier>,
    /// The annotation is not merely a numeric offset alias.
    pub named_zone: Established<NamedTimeZoneIsNotNumericOffsetAlias>,
    /// The named zone preserves civil-time rule identity.
    pub civil_rules: Established<NamedTimeZoneRetainsCivilRuleIdentity>,
}

/// Evidence bundle for offset time-zone annotation semantics.
///
/// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
pub struct OffsetTimeZoneAnnotationEvidence {
    /// The underlying timestamp is a valid IXDTF timestamp.
    pub timestamp: Established<IxdtfTimestampValid>,
    /// The annotation uses an offset time-zone form.
    pub offset_zone: Established<OffsetTimeZoneAnnotationPresent>,
    /// The suffix offset repeats the RFC 3339 timestamp offset.
    pub repeated_offset: Established<OffsetTimeZoneRepeatsTimestampOffset>,
    /// Offset time zones are only a discouraged compatibility form.
    pub discouraged_use: Established<OffsetTimeZoneUseIsStronglyDiscouraged>,
    /// The offset-zone annotation was not synthesized by copying the timestamp offset.
    pub not_synthesized: Established<OffsetTimeZoneMustNotBeSynthesizedFromTimestampOffset>,
}

/// Evidence bundle for TZDB-revision-aware named-zone semantics.
///
/// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
pub struct NamedTimeZoneRevisionEvidence {
    /// The timestamp already carries named-zone identity.
    pub zoned: Established<ZonedDateTimeHasNamedZone>,
    /// Interpretation follows the TZDB rules current at interpretation time.
    pub current_rules: Established<NamedTimeZoneMeaningUsesCurrentTzdbRules>,
    /// Unknown names under revision skew are treated as inconsistencies.
    pub unknown_name: Established<UnknownNamedTimeZoneIdentifierTreatedAsInconsistency>,
}

/// Evidence bundle for offset and named-zone consistency.
///
/// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
pub struct OffsetConsistencyEvidence {
    /// The timestamp denotes a fixed instant.
    pub fixed_instant: Established<TimestampRepresentsFixedInstant>,
    /// The timestamp carries named-zone identity.
    pub zoned: Established<ZonedDateTimeHasNamedZone>,
    /// The named-zone rules resolve the represented offset.
    pub resolved_offset: Established<ZoneOffsetResolvedForRepresentedInstant>,
}

/// Evidence bundle for offset-only semantics.
///
/// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
pub struct OffsetOnlySemanticsEvidence {
    /// The timestamp carries a valid numeric UTC offset.
    pub offset: Established<UtcOffsetValid>,
    /// A numeric offset alone does not identify a named zone.
    pub not_a_zone: Established<NumericOffsetDoesNotIdentifyNamedZone>,
}

/// Evidence bundle for precision preservation.
///
/// Normative sources: ISO 8601-1:2019/Amd 1:2022, 5.3.1.4;
/// RFC 3339 §5.6.
pub struct PrecisionPreservationEvidence {
    /// The source subsecond precision is explicitly declared.
    pub precision: Established<FractionalSecondPrecisionDeclared>,
    /// The source fraction digits form a contiguous decimal suffix.
    pub contiguous_digits: Established<FractionalSecondDigitsAreContiguous>,
    /// The exchange preserved all subsecond digits.
    pub preserved_digits: Established<SubsecondDigitsPreserved>,
}

/// Evidence bundle for ordering preservation.
///
/// Normative source: RFC 3339 §5.1 — Ordering
pub struct TemporalOrderingEvidence {
    /// The source timestamp denotes a fixed instant.
    pub fixed_instant: Established<TimestampRepresentsFixedInstant>,
    /// Ordering is evaluated on the UTC timeline.
    pub utc_timeline: Established<UtcTimelineOrderingAppliesToFixedInstants>,
    /// The conversion preserved temporal ordering.
    pub preserved_ordering: Established<ConversionPreservesTemporalOrdering>,
}

/// Evidence bundle for backend-conversion semantics.
///
/// Normative sources: ISO 8601-1:2019, 3.1.3, 5.2, 5.3, 5.4, 5.5, and 5.6;
/// RFC 3339 §5.1.
pub struct BackendConversionEvidence {
    /// The source semantic kind is explicitly declared.
    pub source_kind: Established<ConversionSourceSemanticKindDeclared>,
    /// The target semantic kind is explicitly declared.
    pub target_kind: Established<ConversionTargetSemanticKindDeclared>,
    /// The conversion preserves the represented instant.
    pub instant: Established<ConversionPreservesRepresentedInstant>,
    /// The conversion preserves temporal ordering.
    pub ordering: Established<TemporalOrderingPreserved>,
}

/// Evidence bundle for explicit lossy-conversion authority.
///
/// Normative basis: ISO 8601-2:2019, 7.13, 14.2, 14.3, and 14.4.
/// The authority sidecar is an accord law layered on top of the standard
/// reduced-precision and truncation regimes.
pub struct LossyConversionAuthorityEvidence {
    /// Lossy conversion was explicitly authorized.
    pub explicit_lossy_authority: Established<ConversionRequiresExplicitAuthorityWhenLossy>,
    /// Precision reduction was declared.
    pub precision_reduction: Established<PrecisionReductionDeclared>,
    /// The rounding mode was declared.
    pub rounding: Established<RoundingModeDeclared>,
    /// The conversion dropped subsecond precision.
    pub dropped_digits: Established<ConversionDropsSubsecondPrecision>,
}

/// Evidence bundle for a lossless conversion.
///
/// Normative sources: RFC 3339 §5.1 and §5.6; ISO 8601-2:2019, 7.11, 7.12,
/// and 7.13.
pub struct LosslessConversionEvidence {
    /// Shared backend-conversion semantics are established.
    pub conversion: Established<BackendConversionSemanticsValid>,
    /// The conversion preserves subsecond precision.
    pub precision: Established<PrecisionPreserved>,
}

/// Evidence bundle for subsecond truncation.
///
/// Normative basis: ISO 8601-2:2019, 7.13, 14.2, 14.3, and 14.4.
pub struct SubsecondTruncationEvidence {
    /// Shared backend-conversion semantics are established.
    pub conversion: Established<BackendConversionSemanticsValid>,
    /// Lossy conversion authority is explicit and fully described.
    pub lossy_authority: Established<LossyConversionAuthorityValid>,
}

/// Evidence bundle for a valid duration form.
///
/// Normative sources: ISO 8601-1:2019, 4.4.2 b); 4.4.3.2; 4.4.3.3
/// Informative cross-checks: CalConnect CC 18011:2018 §7.3 — Representations;
/// ISO/WD 8601-1:2016(E), 4.4.4.2.2; 4.4.4.3; 4.4.4.4; 4.4.5
pub struct DurationWeekFormEvidence {
    /// Week-form durations use the week-unit shape.
    pub form: Established<DurationWeekFormUsesSingleWeekUnit>,
    /// Week-form durations are not mixed with calendar or clock units.
    pub exclusive: Established<DurationWeekFormNotMixedWithCalendarOrClockUnits>,
}

/// Evidence bundle for the alternative complete duration representation.
///
/// Normative source: ISO 8601-1:2019, 4.4.3.3
/// Informative cross-checks: ISO/WD 8601-1:2016(E), 4.4.4.2.2; 4.4.4.3;
/// 4.4.4.4; 4.4.5
pub struct DurationAlternativeFormEvidence {
    /// Use of the alternative duration form is explicitly agreed by the parties.
    pub agreement: Established<DurationAlternativeFormRequiresPartnerAgreement>,
    /// The payload uses calendar-date and time-of-day component slots.
    pub slots: Established<DurationAlternativeFormUsesDateAndTimeComponentSlots>,
    /// The payload carries a complete calendar-and-clock component set.
    pub complete: Established<DurationAlternativeFormCarriesCompleteCalendarAndClockComponents>,
}

/// Evidence branch for the chosen duration representation family.
///
/// Normative sources: ISO 8601-1:2019, 4.4.3.2; 4.4.3.3
/// Informative cross-checks: CalConnect CC 18011:2018 §7.3 — Representations;
/// ISO/WD 8601-1:2016(E), 4.4.4.2.2; 4.4.4.3; 4.4.4.4; 4.4.5
pub enum DurationRepresentationEvidence {
    /// The duration uses the designator-based representation.
    Designator {
        /// Time components, when present, are placed after the `T` designator.
        time_components: Established<DurationTimeComponentsFollowTimeDesignator>,
        /// Week-form semantics when the duration is expressed as `PnnW`.
        week_form: Option<DurationWeekFormEvidence>,
    },
    /// The duration uses the alternative complete representation.
    Alternative(DurationAlternativeFormEvidence),
}

/// Evidence bundle for a valid duration form.
///
/// Normative sources: ISO 8601-1:2019, 4.4.2 b); 4.4.3.2; 4.4.3.3
/// Informative cross-checks: CalConnect CC 18011:2018 §7.3 — Representations;
/// ISO/WD 8601-1:2016(E), 4.4.4.2.2; 4.4.4.3; 4.4.4.4; 4.4.5
pub struct DurationFormEvidence {
    /// The representation begins with the duration designator.
    pub period: Established<DurationUsesPeriodDesignator>,
    /// The concrete representation family and its associated laws.
    pub representation: DurationRepresentationEvidence,
}

/// Evidence bundle for a valid time interval form.
///
/// Normative sources: ISO 8601-1:2019, 4.4.1; 4.4.2 a)
/// Informative cross-check: CalConnect CC 18011:2018 §6.14 — Time interval
pub struct TimeIntervalEvidence {
    /// The interval uses the solidus separator between its top-level components.
    pub separator: Established<TimeIntervalUsesSolidusSeparator>,
    /// The interval has exactly two top-level components.
    pub components: Established<TimeIntervalHasTwoComponents>,
    /// The interval declares its endpoint-or-duration form.
    pub form: Established<TimeIntervalBoundaryOrDurationFormDeclared>,
}

/// Evidence bundle for ordered interval endpoints.
///
/// Normative sources: ISO 8601-1:2019, 3.1.1.6; 3.1.1.8
/// Informative cross-check: CalConnect CC 18011:2018 §6.14 — Time interval;
/// §8 — Evaluation of date and time with duration
pub struct IntervalEndpointOrderingEvidence {
    /// The interval representation is structurally valid.
    pub interval: Established<TimeIntervalValid>,
    /// The start endpoint precedes the end endpoint.
    pub ordering: Established<IntervalStartPrecedesEnd>,
    /// Any duration component denotes a non-negative span.
    pub duration: Established<IntervalDurationIsNonNegative>,
}

/// Evidence bundle for a valid recurring interval form.
///
/// Normative sources: ISO 8601-1:2019, 4.5.1; 4.5.2
/// Informative cross-check: CalConnect CC 18012:2018 §6.3 — Repeat rule;
/// §6.4 — Complete representation
pub struct RecurringIntervalEvidence {
    /// The representation begins with the repetition designator.
    pub repeat_designator: Established<RecurringIntervalUsesRepeatDesignator>,
    /// Any bounded recurrence count is non-negative.
    pub repeat_count: Established<RecurringIntervalCountIsNonNegativeWhenBounded>,
    /// Omitting the recurrence count denotes an unbounded occurrence stream.
    pub omitted_count: Established<RecurringIntervalOmittedCountDenotesUnboundedOccurrences>,
    /// The repetition prefix is followed by an interval component.
    pub interval_component: Established<RecurringIntervalCarriesIntervalComponent>,
}

/// Evidence bundle for a qualified temporal expression.
///
/// Normative source: ISO 8601-2:2019, 8.2.1, 8.2.2, 8.2.3, 8.4.4, 8.4.5,
/// 8.4.6, and 8.5.
/// Informative cross-check: public LOC EDTF Level 1 — Qualification of a date (complete);
/// Level 2 — Qualification
pub enum QualificationPlacementEvidence {
    /// The qualification marker appears immediately to the right and propagates leftward.
    GroupRight {
        /// The qualification marker is placed immediately to the right of the marked component.
        placement: Established<GroupQualificationUsesImmediateRightPlacement>,
        /// The qualification applies to the marked component and all more significant components.
        propagation: Established<GroupQualificationAppliesToMarkedAndMoreSignificantComponents>,
    },
    /// The qualification marker appears immediately to the left and applies only to that component.
    ComponentLeft {
        /// The qualification marker is placed immediately to the left of the marked component.
        placement: Established<ComponentQualificationUsesImmediateLeftPlacement>,
        /// The qualification applies only to the marked component.
        propagation: Established<ComponentQualificationAppliesOnlyToMarkedComponent>,
    },
}

/// Evidence bundle for a qualified temporal expression.
///
/// Normative source: ISO 8601-2:2019, 8.2.1, 8.2.2, 8.2.3, 8.4.4, 8.4.5,
/// 8.4.6, and 8.5.
/// Informative cross-check: public LOC EDTF Level 1 — Qualification of a date (complete);
/// Level 2 — Qualification
pub struct QualifiedTemporalExpressionEvidence {
    /// The expression explicitly declares uncertainty qualification.
    pub uncertainty: Established<UncertaintyQualificationDeclared>,
    /// The expression explicitly declares approximation qualification.
    pub approximation: Established<ApproximationQualificationDeclared>,
    /// The qualification scope is explicitly declared.
    pub scope: Established<QualificationScopeDeclared>,
    /// The placement law for the qualification marker is explicitly carried.
    pub placement: QualificationPlacementEvidence,
    /// Combined uncertainty and approximation semantics are available when needed.
    pub combined: Established<UncertaintyAndApproximationMayBeCombined>,
}

/// Evidence bundle for a qualified temporal value exchange.
///
/// Normative source: ISO 8601-2:2019 — qualification of temporal expressions
/// Informative cross-check: public LOC EDTF Level 1 — Qualification of a date (complete);
/// Level 2 — Qualification
pub struct QualifiedTemporalValueEvidence {
    /// The qualification sidecar is structurally valid.
    pub qualification: Established<QualifiedTemporalExpressionValid>,
}

/// Evidence bundle for a CalConnect explicit temporal form.
///
/// Normative source: CalConnect CC 18011:2018 §4.3 — Explicit forms
pub struct ExplicitTemporalFormEvidence {
    /// Explicit forms use designator symbols.
    pub designators: Established<ExplicitTemporalFormUsesDesignatorSymbols>,
    /// Zero-valued components may be omitted when the result remains valid.
    pub zero_omission: Established<ExplicitTemporalFormMayOmitZeroValuedComponents>,
    /// The lowest denoted component declares precision.
    pub precision: Established<ExplicitTemporalPrecisionUsesLowestDenotedComponent>,
    /// UTC relationship syntax uses `Z` or a signed shift.
    pub utc_relationship: Established<ExplicitUtcRelationshipUsesZuluOrSignedShift>,
}

/// Representation-specific evidence branch for a CalConnect explicit duration.
///
/// Normative source: CalConnect CC 18011:2018 §7.3-§7.5 — Representations
pub enum ExplicitDurationRepresentationEvidence {
    /// Simple representation semantics.
    Simple {
        /// The duration explicitly declares its representation family.
        representation_kind: Established<ExplicitDurationRepresentationKindDeclared>,
    },
    /// Composite representation semantics.
    Composite {
        /// The duration explicitly declares its representation family.
        representation_kind: Established<ExplicitDurationRepresentationKindDeclared>,
        /// The duration uses the composite representation family.
        composite: Established<ExplicitDurationCompositeRepresentationDeclared>,
    },
    /// Precedence representation semantics.
    Precedence {
        /// The duration explicitly declares its representation family.
        representation_kind: Established<ExplicitDurationRepresentationKindDeclared>,
        /// The representation preserves the declared evaluation order.
        precedence: Established<ExplicitDurationPrecedenceRepresentationCarriesEvaluationOrder>,
    },
}

/// Exactness-specific evidence branch for a CalConnect explicit duration.
///
/// Normative source: CalConnect CC 18011:2018 §7.8-§7.10 — Exact, context-dependent, and speculative duration
pub enum ExplicitDurationSemanticEvidence {
    /// No exactness family is explicitly declared.
    None,
    /// Exact-duration semantics are declared.
    Exact {
        /// The duration is explicitly classified as exact.
        exact: Established<ExactDurationSemanticsDeclared>,
    },
    /// Context-dependent-duration semantics are declared.
    ContextDependent {
        /// The duration is explicitly classified as context-dependent.
        context_dependent: Established<ContextDependentDurationSemanticsDeclared>,
    },
    /// Speculative-duration semantics are declared.
    Speculative {
        /// The duration is explicitly classified as speculative.
        speculative: Established<SpeculativeDurationSemanticsDeclared>,
    },
}

/// Evidence bundle for a CalConnect explicit duration.
///
/// Normative source: CalConnect CC 18011:2018 §7 — Explicit duration
pub struct ExplicitDurationEvidence {
    /// Durational unit designators are used.
    pub units: Established<ExplicitDurationUsesDurationalUnitDesignators>,
    /// The explicit duration declares its representation family.
    pub representation: ExplicitDurationRepresentationEvidence,
    /// Signed negative-duration semantics are explicitly available.
    pub sign: Established<ExplicitDurationMayBeNegative>,
    /// Fractional lowest-order unit semantics are explicitly available.
    pub fractional: Established<ExplicitDurationMayUseFractionalLowestOrderUnit>,
    /// Exactness-family semantics are explicitly carried when needed.
    pub semantics: ExplicitDurationSemanticEvidence,
}

/// Evidence bundle for extended interval boundary semantics.
///
/// Normative source: ISO 8601-2:2019, 10.2.
/// Informative cross-check: public LOC EDTF Level 1 — Extended Interval
pub struct ExtendedIntervalBoundaryEvidence {
    /// The underlying interval representation is structurally valid.
    pub interval: Established<TimeIntervalValid>,
    /// Open-boundary semantics are explicitly declared.
    pub open_boundary: Established<OpenIntervalBoundaryDeclared>,
    /// Unknown-boundary semantics are explicitly declared.
    pub unknown_boundary: Established<UnknownIntervalBoundaryDeclared>,
}

/// Evidence bundle for a grouped time scale unit expression.
///
/// Normative source: CalConnect CC 18011:2018 §5 — Grouped time scale units
pub struct GroupedTimeScaleUnitEvidence {
    /// Grouped units use the `G...U` delimiters.
    pub designators: Established<GroupedTimeScaleUnitUsesGroupingDesignators>,
    /// Grouped units carry one or more duration components.
    pub units: Established<GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits>,
    /// Grouped-unit definitions are continuous.
    pub continuity: Established<GroupedTimeScaleUnitDefinitionIsContinuous>,
    /// Grouped-unit values carry explicit coefficients.
    pub coefficient: Established<GroupedTimeScaleUnitValueCarriesExplicitCoefficient>,
    /// Lower-order units remain within the grouped-unit bounds.
    pub bounds: Established<GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds>,
    /// Out-of-bounds remainder truncates at the original boundary.
    pub truncation: Established<GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder>,
    /// Grouped-unit expressions convert into time-interval semantics.
    pub interval_semantics: Established<GroupedTimeScaleUnitConvertsToTimeInterval>,
}

/// Evidence bundle for a date-time formula.
///
/// Normative source: CalConnect CC 18011:2018 §8 — Evaluation of date and time with duration
pub struct DateTimeFormulaEvidence {
    /// The formula combines an explicit temporal value with a duration.
    pub combination: Established<DateTimeFormulaCombinesTemporalValueWithDuration>,
    /// Overflow is carried across component boundaries.
    pub carry_over: Established<DateTimeFormulaUsesCarryOverSemantics>,
    /// Evaluation truncates at component boundaries when required.
    pub truncation: Established<DateTimeFormulaTruncatesAtComponentBoundaries>,
}

/// Evidence bundle for date-time formula evaluation semantics.
///
/// Normative source: CalConnect CC 18011:2018 §8.3-§8.5 — Simple, composite, and precedence duration
pub struct DateTimeFormulaEvaluationSemanticsEvidence {
    /// The underlying formula is structurally valid.
    pub formula: Established<DateTimeFormulaValid>,
    /// The formula declares its evaluation family.
    pub evaluation_mode: Established<DateTimeFormulaEvaluationModeDeclared>,
}

/// Evidence bundle for a seasonal temporal expression.
///
/// Normative source: ISO 8601-2:2019, 4.8.1, 4.8.2, and 4.8.3.
/// Informative cross-check: public LOC EDTF Level 1 — Seasons; Level 2 — Sub-year groupings
pub struct SeasonalTemporalExpressionEvidence {
    /// The expression uses a year-and-season form.
    pub form: Established<SeasonalExpressionUsesYearAndSeasonForm>,
    /// The season code occupies the month slot of a year-month-shaped representation.
    pub month_slot: Established<SeasonalExpressionUsesSeasonCodeInMonthSlot>,
    /// The season code declares a named season.
    pub named_season: Established<SeasonCodeDeclaresNamedSeason>,
    /// The season code declares its season scope.
    pub season_scope: Established<SeasonCodeDeclaresSeasonScope>,
}

/// Branch evidence for the specific Level 2 sub-year grouping family in use.
///
/// Normative source: ISO 8601-2:2019, 4.8.1, 4.8.2, and 4.8.3.
/// Informative cross-check: public LOC EDTF Level 2 — Sub-year groupings
pub enum SubYearGroupingKindEvidence {
    /// Seasonal grouping semantics, including hemisphere scope when declared.
    Season {
        /// The code declares a named season.
        named_season: Established<SeasonCodeDeclaresNamedSeason>,
        /// The code declares its season scope.
        season_scope: Established<SeasonCodeDeclaresSeasonScope>,
    },
    /// Quarter grouping semantics.
    Quarter {
        /// The code declares a quarter grouping.
        quarter: Established<SubYearGroupingCodeDeclaresQuarter>,
    },
    /// Quadrimester grouping semantics.
    Quadrimester {
        /// The code declares a quadrimester grouping.
        quadrimester: Established<SubYearGroupingCodeDeclaresQuadrimester>,
    },
    /// Semestral grouping semantics.
    Semestral {
        /// The code declares a semestral grouping.
        semestral: Established<SubYearGroupingCodeDeclaresSemestral>,
    },
}

/// Evidence bundle for a Level 2 sub-year grouping expression.
///
/// Normative source: ISO 8601-2:2019, 4.8.1, 4.8.2, and 4.8.3.
/// Informative cross-check: public LOC EDTF Level 2 — Sub-year groupings
pub struct SubYearGroupingExpressionEvidence {
    /// The expression uses a year-and-grouping form.
    pub form: Established<SubYearGroupingExpressionUsesYearAndGroupingForm>,
    /// The grouping code occupies the month slot of a year-month-shaped representation.
    pub month_slot: Established<SubYearGroupingExpressionUsesGroupingCodeInMonthSlot>,
    /// The specific grouping family declared by the code.
    pub grouping: SubYearGroupingKindEvidence,
}

/// Evidence bundle for masked precision and unspecified-component semantics.
///
/// Normative source: ISO 8601-2:2019, 9.2.1, 9.2.2, and 9.3.
/// Informative cross-check: public LOC EDTF Level 1 — Unspecified digit(s) from the right;
/// Level 2 — Unspecified Digit
pub struct UnspecifiedComponentExpressionEvidence {
    /// Unspecified digits use the uppercase `X` placeholder.
    pub placeholder: Established<UnspecifiedDigitUsesUppercaseXPlaceholder>,
    /// Each `X` placeholder denotes an unspecified digit or component value.
    pub unspecified_value: Established<UnspecifiedDigitsDeclareUnknownValue>,
    /// Level 1 masking occupies one or more rightmost positions.
    pub level_one_tail_masking: Established<LevelOneUnspecifiedDigitsOccupyRightmostPositions>,
    /// Level 2 masking may occur within a component.
    pub level_two_component_masking: Established<LevelTwoUnspecifiedDigitsMayAppearWithinComponent>,
}

/// Evidence bundle for a temporal set expression.
///
/// Normative source: ISO 8601-2:2019, 6.1, 6.2, 6.3, and 6.4.
/// Informative cross-check: public LOC EDTF Level 2 — Set representation
pub struct TemporalSetExpressionEvidence {
    /// Member expressions are explicitly separated.
    pub separator: Established<TemporalSetMemberSeparatorDeclared>,
    /// The set carries multiple temporal members.
    pub members: Established<TemporalSetCarriesMultipleMembers>,
}

/// Evidence bundle for refined temporal-set range semantics.
///
/// Normative source: ISO 8601-2:2019, 6.3 and 6.4.
/// Informative cross-check: public LOC EDTF Level 2 — Set representation
pub struct TemporalSetRangeSemanticsEvidence {
    /// The underlying temporal-set expression is structurally valid.
    pub set: Established<TemporalSetExpressionValid>,
    /// No internal whitespace appears within the expression.
    pub no_whitespace: Established<TemporalSetForbidsInternalWhitespace>,
    /// `..` denotes the inclusive values between bounded range endpoints.
    pub inclusive_range: Established<TemporalSetRangeUsesInclusiveDoubleDotSemantics>,
    /// Leading or trailing `..` denotes an open-ended boundary.
    pub open_range: Established<TemporalSetOpenRangeUsesBoundaryDoubleDot>,
    /// Values adjacent to a range share the same precision as the range expansion.
    pub precision: Established<TemporalSetRangeNeighborhoodSharesPrecision>,
}

/// Evidence bundle for a selection expression.
///
/// Normative source: CalConnect CC 18012:2018 §5 — Selection
pub struct SelectionExpressionEvidence {
    /// Selection expressions use the `L...N` delimiters.
    pub delimiters: Established<SelectionExpressionUsesSelectionDelimiters>,
    /// Selection expressions use the standard rule vocabulary.
    pub vocabulary: Established<SelectionExpressionUsesRecognizedSelectionRuleVocabulary>,
    /// Subsequent components apply within previously selected results.
    pub nesting: Established<SelectionRulesApplyWithinSelectedResults>,
    /// Single-instance selection is explicitly modeled.
    pub single_instance: Established<SelectionExpressionMaySelectSingleInstance>,
    /// Position rules apply after earlier selection rules.
    pub position: Established<SelectionRulePositionAppliesLast>,
    /// Selection-with-duration uses an explicit duration suffix.
    pub duration_window: Established<SelectionWithDurationUsesDurationSuffix>,
}

/// Evidence bundle for a repeat rule.
///
/// Normative source: CalConnect CC 18012:2018 §6.3 — Repeat rule
pub struct RepeatRuleEvidence {
    /// Repeat rules use the frequency designator.
    pub frequency: Established<RepeatRuleUsesFrequencyDesignator>,
    /// Eligible time intervals are explicitly declared.
    pub eligible_intervals: Established<RepeatRuleDeclaresEligibleTimeIntervals>,
    /// Selection rules apply within the eligible intervals.
    pub selection: Established<RepeatRuleSelectionAppliesWithinEligibleIntervals>,
    /// Evaluation inherits component information from the initial start date.
    pub inheritance: Established<RepeatRuleEvaluationInheritsInitialStartComponentInformation>,
}

/// Evidence bundle for a recurring interval with repeat-rule refinement.
///
/// Normative source: CalConnect CC 18012:2018 §6.4 — Complete representation
pub struct RecurringIntervalWithRepeatRuleEvidence {
    /// The recurring-interval prefix and interval component are structurally valid.
    pub recurring_interval: Established<RecurringIntervalFormValid>,
    /// The attached repeat rule is structurally valid.
    pub repeat_rule: Established<RepeatRuleValid>,
    /// The combined representation uses the complete recurring form.
    pub complete_representation:
        Established<RecurringIntervalWithRepeatRuleUsesCompleteRepresentation>,
}

impl ProvableFrom<CalendarDateEvidence> for CalendarDateValid {}
impl ProvableFrom<ReducedCalendarDateEvidence> for ReducedCalendarDateValid {}
impl ProvableFrom<DateEvidence> for DateValid {}
impl ProvableFrom<TimeEvidence> for TimeValid {}
impl ProvableFrom<DecadeEvidence> for DecadeValid {}
impl ProvableFrom<CenturyEvidence> for CenturyValid {}
impl ProvableFrom<ExtendedYearEvidence> for ExtendedYearValid {}
impl ProvableFrom<OrdinalDateEvidence> for OrdinalDateValid {}
impl ProvableFrom<WeekDateEvidence> for WeekDateValid {}
impl ProvableFrom<LocalTimeEvidence> for LocalTimeValid {}
impl ProvableFrom<ReducedLocalTimeEvidence> for ReducedLocalTimeValid {}
impl ProvableFrom<UtcTimeScaleEvidence> for UtcTimeScaleValid {}
impl ProvableFrom<UtcOfDayEvidence> for UtcOfDayValid {}
impl ProvableFrom<UtcOffsetEvidence> for UtcOffsetValid {}
impl ProvableFrom<KnownUtcOffsetEvidence> for UtcOffsetKnown {}
impl ProvableFrom<StandardTimeEvidence> for StandardTimeValid {}
impl ProvableFrom<StandardTimeOfDayEvidence> for StandardTimeOfDayValid {}
impl ProvableFrom<LocalTimeScaleEvidence> for LocalTimeScaleValid {}
impl ProvableFrom<LocalTimeSemanticsEvidence> for LocalTimeSemanticsValid {}
impl ProvableFrom<LocalDateTimeEvidence> for LocalDateTimeValid {}
impl ProvableFrom<OffsetDateTimeEvidence> for OffsetDateTimeValid {}
impl ProvableFrom<DateWithShiftEvidence> for DateWithShiftValid {}
impl ProvableFrom<TimeOfDayWithShiftEvidence> for TimeOfDayWithShiftValid {}
impl ProvableFrom<FixedInstantEvidence> for TimestampRepresentsFixedInstant {}
impl ProvableFrom<LocalTimestampSemanticsEvidence> for LocalDateTimeDoesNotIdentifyFixedInstant {}
impl ProvableFrom<MutualAgreementAuthorityEvidence> for MutualAgreementAuthorityValid {}
impl ProvableFrom<ZoneTransitionResolutionAuthorityEvidence>
    for ZoneTransitionResolutionAuthorityValid
{
}
impl ProvableFrom<ZoneTransitionAmbiguityEvidence> for ZoneTransitionAmbiguitySemanticsValid {}
impl ProvableFrom<ZoneTransitionGapEvidence> for ZoneTransitionGapSemanticsValid {}
impl ProvableFrom<Rfc3339TimestampEvidence> for Rfc3339TimestampValid {}
impl ProvableFrom<Rfc3339LexicalOrderingEvidence> for Rfc3339LexicalOrderingSemanticsValid {}
impl ProvableFrom<Rfc3339GenerationGuidanceEvidence> for Rfc3339GenerationGuidanceValid {}
impl ProvableFrom<IxdtfTimestampEvidence> for IxdtfTimestampValid {}
impl ProvableFrom<IxdtfAdditionalInformationEvidence> for IxdtfAdditionalInformationSemanticsValid {}
impl ProvableFrom<IxdtfCalendarAwareTimestampEvidence>
    for IxdtfTimestampHasPreferredPresentationCalendar
{
}
impl ProvableFrom<ZonedTimestampEvidence> for ZonedDateTimeHasNamedZone {}
impl ProvableFrom<OffsetTimeZoneAnnotationEvidence>
    for OffsetTimeZoneAnnotationConsistentWithTimestamp
{
}
impl ProvableFrom<NamedTimeZoneRevisionEvidence> for NamedTimeZoneInterpretationTracksTzdbRevision {}
impl ProvableFrom<OffsetConsistencyEvidence> for OffsetConsistentWithNamedZone {}
impl ProvableFrom<OffsetOnlySemanticsEvidence> for OffsetOnlyZoneSemanticsLimited {}
impl ProvableFrom<PrecisionPreservationEvidence> for PrecisionPreserved {}
impl ProvableFrom<TemporalOrderingEvidence> for TemporalOrderingPreserved {}
impl ProvableFrom<BackendConversionEvidence> for BackendConversionSemanticsValid {}
impl ProvableFrom<LossyConversionAuthorityEvidence> for LossyConversionAuthorityValid {}
impl ProvableFrom<LosslessConversionEvidence> for ConversionLossless {}
impl ProvableFrom<SubsecondTruncationEvidence> for ConversionTruncatesSubseconds {}
impl ProvableFrom<DurationFormEvidence> for DurationFormValid {}
impl ProvableFrom<TimeIntervalEvidence> for TimeIntervalValid {}
impl ProvableFrom<IntervalEndpointOrderingEvidence> for IntervalEndpointsOrdered {}
impl ProvableFrom<RecurringIntervalEvidence> for RecurringIntervalFormValid {}
impl ProvableFrom<QualifiedTemporalExpressionEvidence> for QualifiedTemporalExpressionValid {}
impl ProvableFrom<QualifiedTemporalValueEvidence> for QualifiedTemporalValueValid {}
impl ProvableFrom<ExplicitTemporalFormEvidence> for ExplicitTemporalFormValid {}
impl ProvableFrom<ExplicitTimeOfDayEvidence> for ExplicitTimeOfDayValid {}
impl ProvableFrom<ExplicitTimeShiftEvidence> for ExplicitTimeShiftValid {}
impl ProvableFrom<ExplicitDateTimeEvidence> for ExplicitDateTimeValid {}
impl ProvableFrom<ExplicitDateTimeWithShiftEvidence> for ExplicitDateTimeWithShiftValid {}
impl ProvableFrom<ExplicitDurationEvidence> for ExplicitDurationValid {}
impl ProvableFrom<ExtendedIntervalBoundaryEvidence> for ExtendedIntervalBoundarySemanticsValid {}
impl ProvableFrom<GroupedTimeScaleUnitEvidence> for GroupedTimeScaleUnitValid {}
impl ProvableFrom<DateTimeFormulaEvidence> for DateTimeFormulaValid {}
impl ProvableFrom<DateTimeFormulaEvaluationSemanticsEvidence>
    for DateTimeFormulaEvaluationSemanticsValid
{
}
impl ProvableFrom<SeasonalTemporalExpressionEvidence> for SeasonalTemporalExpressionValid {}
impl ProvableFrom<SubYearGroupingExpressionEvidence> for SubYearGroupingExpressionValid {}
impl ProvableFrom<UnspecifiedComponentExpressionEvidence> for UnspecifiedComponentExpressionValid {}
impl ProvableFrom<TemporalSetExpressionEvidence> for TemporalSetExpressionValid {}
impl ProvableFrom<TemporalSetRangeSemanticsEvidence> for TemporalSetRangeSemanticsValid {}
impl ProvableFrom<SelectionExpressionEvidence> for SelectionExpressionValid {}
impl ProvableFrom<RepeatRuleEvidence> for RepeatRuleValid {}
impl ProvableFrom<RecurringIntervalWithRepeatRuleEvidence>
    for RecurringIntervalWithRepeatRuleValid
{
}
