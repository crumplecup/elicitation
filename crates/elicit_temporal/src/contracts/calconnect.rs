//! CalConnect temporal extension propositions.
//!
//! Public authoritative sources:
//! - CalConnect CC 18011:2018, *Date and time - Explicit representation*
//! - CalConnect CC 18012:2018, *Date and time - General recurrence representation*
//!
//! Unlike the ISO topic-label placeholders elsewhere in this crate, the
//! CalConnect families below already carry direct public section references.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — CalConnect temporal contract */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — CalConnect temporal contract */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — CalConnect temporal contract */ }
                }
            }
        };
    }

    /// An explicit temporal form uses designator symbols to delimit time scale components.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3 — Explicit forms
    pub struct ExplicitTemporalFormUsesDesignatorSymbols;
    structural_prop!(
        ExplicitTemporalFormUsesDesignatorSymbols,
        "ExplicitTemporalFormUsesDesignatorSymbols"
    );

    /// An explicit form may omit zero-valued components when the resulting representation stays valid.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3.6 — Omission of zero valued components
    pub struct ExplicitTemporalFormMayOmitZeroValuedComponents;
    structural_prop!(
        ExplicitTemporalFormMayOmitZeroValuedComponents,
        "ExplicitTemporalFormMayOmitZeroValuedComponents"
    );

    /// The lowest denoted explicit component declares the representation's precision.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3.7 — Indication of precision
    pub struct ExplicitTemporalPrecisionUsesLowestDenotedComponent;
    structural_prop!(
        ExplicitTemporalPrecisionUsesLowestDenotedComponent,
        "ExplicitTemporalPrecisionUsesLowestDenotedComponent"
    );

    /// An explicit UTC relationship uses either `Z` or a signed time shift.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3.5 — Explicit time shift
    pub struct ExplicitUtcRelationshipUsesZuluOrSignedShift;
    structural_prop!(
        ExplicitUtcRelationshipUsesZuluOrSignedShift,
        "ExplicitUtcRelationshipUsesZuluOrSignedShift"
    );

    /// An explicit local time of day uses the leading `T` designator.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3.3 — Local time of day
    pub struct ExplicitTimeOfDayUsesTimeDesignator;
    structural_prop!(
        ExplicitTimeOfDayUsesTimeDesignator,
        "ExplicitTimeOfDayUsesTimeDesignator"
    );

    /// An explicit local time of day uses hour, minute, and second unit designators.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3.3 — Local time of day
    pub struct ExplicitTimeOfDayUsesHourMinuteSecondUnitDesignators;
    structural_prop!(
        ExplicitTimeOfDayUsesHourMinuteSecondUnitDesignators,
        "ExplicitTimeOfDayUsesHourMinuteSecondUnitDesignators"
    );

    /// Explicit local time of day has no end-of-day representation.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3.3.2 — Beginning of the day
    pub struct ExplicitTimeOfDayForbidsEndOfDayRepresentation;
    structural_prop!(
        ExplicitTimeOfDayForbidsEndOfDayRepresentation,
        "ExplicitTimeOfDayForbidsEndOfDayRepresentation"
    );

    /// An explicit time shift uses the leading `Z` designator.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3.4 — Time shift
    pub struct ExplicitTimeShiftUsesZuluDesignator;
    structural_prop!(
        ExplicitTimeShiftUsesZuluDesignator,
        "ExplicitTimeShiftUsesZuluDesignator"
    );

    /// A leading minus sign is used only when the explicit time shift is behind UTC.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3.4 — Time shift
    pub struct ExplicitTimeShiftUsesLeadingMinusOnlyWhenBehindUtc;
    structural_prop!(
        ExplicitTimeShiftUsesLeadingMinusOnlyWhenBehindUtc,
        "ExplicitTimeShiftUsesLeadingMinusOnlyWhenBehindUtc"
    );

    /// A non-empty explicit time-shift payload uses the explicit local-time-of-day family.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3.4 — Time shift
    pub struct ExplicitTimeShiftPayloadUsesExplicitTimeOfDay;
    structural_prop!(
        ExplicitTimeShiftPayloadUsesExplicitTimeOfDay,
        "ExplicitTimeShiftPayloadUsesExplicitTimeOfDay"
    );

    /// A bare `Z` explicit time shift denotes UTC with zero shift.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3.4 — Time shift
    pub struct ExplicitTimeShiftBareZuluRepresentsUtcZero;
    structural_prop!(
        ExplicitTimeShiftBareZuluRepresentsUtcZero,
        "ExplicitTimeShiftBareZuluRepresentsUtcZero"
    );

    /// An explicit date-time concatenates a complete explicit date before the explicit time.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3.4.3 — Date and time only
    pub struct ExplicitDateTimeUsesDateThenTimeConcatenation;
    structural_prop!(
        ExplicitDateTimeUsesDateThenTimeConcatenation,
        "ExplicitDateTimeUsesDateThenTimeConcatenation"
    );

    /// The time portion of an explicit date-time may use reduced precision.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3.4.3 — Date and time of day
    pub struct ExplicitDateTimeTimePortionMayBeReducedPrecision;
    structural_prop!(
        ExplicitDateTimeTimePortionMayBeReducedPrecision,
        "ExplicitDateTimeTimePortionMayBeReducedPrecision"
    );

    /// An explicit date with shift concatenates a complete explicit date before the explicit time shift.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3.4.1 — Date with shift
    pub struct ExplicitDateWithShiftUsesDateThenShiftConcatenation;
    structural_prop!(
        ExplicitDateWithShiftUsesDateThenShiftConcatenation,
        "ExplicitDateWithShiftUsesDateThenShiftConcatenation"
    );

    /// An explicit time of day with shift concatenates an explicit time before the explicit time shift.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3.4.2 — Time of day with time shift
    pub struct ExplicitTimeOfDayWithShiftUsesTimeThenShiftConcatenation;
    structural_prop!(
        ExplicitTimeOfDayWithShiftUsesTimeThenShiftConcatenation,
        "ExplicitTimeOfDayWithShiftUsesTimeThenShiftConcatenation"
    );

    /// An explicit date-time with shift concatenates an explicit date-time before the explicit time shift.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3.4.3 — Date and time with shift
    pub struct ExplicitDateTimeWithShiftUsesDateTimeThenShiftConcatenation;
    structural_prop!(
        ExplicitDateTimeWithShiftUsesDateTimeThenShiftConcatenation,
        "ExplicitDateTimeWithShiftUsesDateTimeThenShiftConcatenation"
    );

    /// A complete explicit time interval uses the CalConnect `[datetimeE]/[datetimeE]` endpoint family.
    ///
    /// Normative source: CalConnect CC 18011:2018 §6 — Time interval, General
    pub struct ExplicitTimeIntervalUsesDateTimeEndpointFamily;
    structural_prop!(
        ExplicitTimeIntervalUsesDateTimeEndpointFamily,
        "ExplicitTimeIntervalUsesDateTimeEndpointFamily"
    );

    /// An explicit time interval may substitute an explicit duration for either boundary when the missing endpoint remains inferable.
    ///
    /// Normative source: CalConnect CC 18011:2018 §6 — Time interval, Duration substitution
    pub struct ExplicitTimeIntervalDurationSubstitutionInfersMissingBoundary;
    structural_prop!(
        ExplicitTimeIntervalDurationSubstitutionInfersMissingBoundary,
        "ExplicitTimeIntervalDurationSubstitutionInfersMissingBoundary"
    );

    /// An explicit time interval may omit higher-order trailing-end components when inheritance from the start stays unambiguous.
    ///
    /// Normative source: CalConnect CC 18011:2018 §6 — Time interval, Time scale component order
    pub struct ExplicitTimeIntervalTrailingEndMayInheritHigherOrderComponents;
    structural_prop!(
        ExplicitTimeIntervalTrailingEndMayInheritHigherOrderComponents,
        "ExplicitTimeIntervalTrailingEndMayInheritHigherOrderComponents"
    );

    /// A leading explicit time shift propagates across the interval separator unless the trailing component supplies an alternative.
    ///
    /// Normative source: CalConnect CC 18011:2018 §6 — Time interval, Time shift indication
    pub struct ExplicitTimeIntervalLeadingShiftAppliesToTrailingComponentUnlessOverridden;
    structural_prop!(
        ExplicitTimeIntervalLeadingShiftAppliesToTrailingComponentUnlessOverridden,
        "ExplicitTimeIntervalLeadingShiftAppliesToTrailingComponentUnlessOverridden"
    );

    /// A grouped time scale unit uses the `G...U` grouping delimiters.
    ///
    /// Normative source: CalConnect CC 18011:2018 §5.1 — Unit definition
    pub struct GroupedTimeScaleUnitUsesGroupingDesignators;
    structural_prop!(
        GroupedTimeScaleUnitUsesGroupingDesignators,
        "GroupedTimeScaleUnitUsesGroupingDesignators"
    );

    /// A grouped time scale unit carries one or more positive duration components.
    ///
    /// Normative source: CalConnect CC 18011:2018 §5.1 — Unit definition
    pub struct GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits;
    structural_prop!(
        GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits,
        "GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits"
    );

    /// Adjacent grouped units define a continuous interval without gaps.
    ///
    /// Normative source: CalConnect CC 18011:2018 §5.1 — Unit definition
    pub struct GroupedTimeScaleUnitDefinitionIsContinuous;
    structural_prop!(
        GroupedTimeScaleUnitDefinitionIsContinuous,
        "GroupedTimeScaleUnitDefinitionIsContinuous"
    );

    /// A grouped-unit value carries an explicit coefficient.
    ///
    /// Normative source: CalConnect CC 18011:2018 §5.2 — Unit value
    pub struct GroupedTimeScaleUnitValueCarriesExplicitCoefficient;
    structural_prop!(
        GroupedTimeScaleUnitValueCarriesExplicitCoefficient,
        "GroupedTimeScaleUnitValueCarriesExplicitCoefficient"
    );

    /// Lower-order units remain within the bounds set by the grouped unit.
    ///
    /// Normative source: CalConnect CC 18011:2018 §5.3.2 — Adherence to grouped unit boundaries
    pub struct GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds;
    structural_prop!(
        GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds,
        "GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds"
    );

    /// Out-of-bounds grouped-unit remainder is truncated at the original boundary.
    ///
    /// Normative source: CalConnect CC 18011:2018 §5.3.4 — Truncation of partial units
    pub struct GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder;
    structural_prop!(
        GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder,
        "GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder"
    );

    /// A grouped-unit date-time may append an explicit time shift.
    ///
    /// Normative source: CalConnect CC 18011:2018 §5.3 — Use of grouped units, Representation with time shift
    pub struct GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift;
    structural_prop!(
        GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift,
        "GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift"
    );

    /// Grouped-unit expressions can be converted into time-interval semantics.
    ///
    /// Normative source: CalConnect CC 18011:2018 §5.3.3 — Conversion to basic time scale units
    pub struct GroupedTimeScaleUnitConvertsToTimeInterval;
    structural_prop!(
        GroupedTimeScaleUnitConvertsToTimeInterval,
        "GroupedTimeScaleUnitConvertsToTimeInterval"
    );

    /// An explicit duration is represented with durational unit designators.
    ///
    /// Normative source: CalConnect CC 18011:2018 §7.2 — Durational units
    pub struct ExplicitDurationUsesDurationalUnitDesignators;
    structural_prop!(
        ExplicitDurationUsesDurationalUnitDesignators,
        "ExplicitDurationUsesDurationalUnitDesignators"
    );

    /// An explicit duration declares whether it uses simple, composite, or precedence representation.
    ///
    /// Normative source: CalConnect CC 18011:2018 §7.3 — Representations
    pub struct ExplicitDurationRepresentationKindDeclared;
    structural_prop!(
        ExplicitDurationRepresentationKindDeclared,
        "ExplicitDurationRepresentationKindDeclared"
    );

    /// An explicit duration may use the composite representation family.
    ///
    /// Normative source: CalConnect CC 18011:2018 §7.4 — Composite representation
    pub struct ExplicitDurationCompositeRepresentationDeclared;
    structural_prop!(
        ExplicitDurationCompositeRepresentationDeclared,
        "ExplicitDurationCompositeRepresentationDeclared"
    );

    /// A precedence duration preserves the declared component evaluation order.
    ///
    /// Normative source: CalConnect CC 18011:2018 §7.5 — Precedence representation
    pub struct ExplicitDurationPrecedenceRepresentationCarriesEvaluationOrder;
    structural_prop!(
        ExplicitDurationPrecedenceRepresentationCarriesEvaluationOrder,
        "ExplicitDurationPrecedenceRepresentationCarriesEvaluationOrder"
    );

    /// An explicit duration may be signed negative.
    ///
    /// Normative source: CalConnect CC 18011:2018 §7.6 — Negative duration
    pub struct ExplicitDurationMayBeNegative;
    structural_prop!(
        ExplicitDurationMayBeNegative,
        "ExplicitDurationMayBeNegative"
    );

    /// An explicit duration may carry a fractional lowest-order unit.
    ///
    /// Normative source: CalConnect CC 18011:2018 §7.7 — Fractional duration
    pub struct ExplicitDurationMayUseFractionalLowestOrderUnit;
    structural_prop!(
        ExplicitDurationMayUseFractionalLowestOrderUnit,
        "ExplicitDurationMayUseFractionalLowestOrderUnit"
    );

    /// Exact-duration semantics are explicitly declared when that family is used.
    ///
    /// Normative source: CalConnect CC 18011:2018 §7.8 — Exact duration
    pub struct ExactDurationSemanticsDeclared;
    structural_prop!(
        ExactDurationSemanticsDeclared,
        "ExactDurationSemanticsDeclared"
    );

    /// Context-dependent-duration semantics are explicitly declared when that family is used.
    ///
    /// Normative source: CalConnect CC 18011:2018 §7.9 — Context-dependent duration
    pub struct ContextDependentDurationSemanticsDeclared;
    structural_prop!(
        ContextDependentDurationSemanticsDeclared,
        "ContextDependentDurationSemanticsDeclared"
    );

    /// Speculative-duration semantics are explicitly declared when that family is used.
    ///
    /// Normative source: CalConnect CC 18011:2018 §7.10 — Speculative duration
    pub struct SpeculativeDurationSemanticsDeclared;
    structural_prop!(
        SpeculativeDurationSemanticsDeclared,
        "SpeculativeDurationSemanticsDeclared"
    );

    /// A date-time formula combines an explicit temporal value with a duration.
    ///
    /// Normative source: CalConnect CC 18011:2018 §8 — Evaluation of date and time with duration
    pub struct DateTimeFormulaCombinesTemporalValueWithDuration;
    structural_prop!(
        DateTimeFormulaCombinesTemporalValueWithDuration,
        "DateTimeFormulaCombinesTemporalValueWithDuration"
    );

    /// Date-time formula evaluation carries overflow across component boundaries.
    ///
    /// Normative source: CalConnect CC 18011:2018 §8.1 — Carry-over of overflow
    pub struct DateTimeFormulaUsesCarryOverSemantics;
    structural_prop!(
        DateTimeFormulaUsesCarryOverSemantics,
        "DateTimeFormulaUsesCarryOverSemantics"
    );

    /// Date-time formula evaluation truncates values at time scale component boundaries.
    ///
    /// Normative source: CalConnect CC 18011:2018 §8.2 — Truncation at time scale component boundaries
    pub struct DateTimeFormulaTruncatesAtComponentBoundaries;
    structural_prop!(
        DateTimeFormulaTruncatesAtComponentBoundaries,
        "DateTimeFormulaTruncatesAtComponentBoundaries"
    );

    /// A date-time formula declares whether evaluation follows simple, composite, or precedence duration rules.
    ///
    /// Normative source: CalConnect CC 18011:2018 §8.3-§8.5 — Simple, composite, and precedence duration
    pub struct DateTimeFormulaEvaluationModeDeclared;
    structural_prop!(
        DateTimeFormulaEvaluationModeDeclared,
        "DateTimeFormulaEvaluationModeDeclared"
    );

    /// A selection expression is delimited by the `L...N` selection markers.
    ///
    /// Normative source: CalConnect CC 18012:2018 §5.1 — General
    pub struct SelectionExpressionUsesSelectionDelimiters;
    structural_prop!(
        SelectionExpressionUsesSelectionDelimiters,
        "SelectionExpressionUsesSelectionDelimiters"
    );

    /// A selection expression uses the standard selection-rule vocabulary.
    ///
    /// Normative source: CalConnect CC 18012:2018 §5.2 — Selection rules
    pub struct SelectionExpressionUsesRecognizedSelectionRuleVocabulary;
    structural_prop!(
        SelectionExpressionUsesRecognizedSelectionRuleVocabulary,
        "SelectionExpressionUsesRecognizedSelectionRuleVocabulary"
    );

    /// A month-selection rule uses the `[monthE]` rule family.
    ///
    /// Normative source: CalConnect CC 18012:2018 §5.2 — Selection of calendar month of year
    pub struct SelectionRuleMonthUsesMonthExpression;
    structural_prop!(
        SelectionRuleMonthUsesMonthExpression,
        "SelectionRuleMonthUsesMonthExpression"
    );

    /// A week-selection rule uses the `[weekE]` rule family.
    ///
    /// Normative source: CalConnect CC 18012:2018 §5.2 — Selection of calendar week of year
    pub struct SelectionRuleWeekUsesWeekExpression;
    structural_prop!(
        SelectionRuleWeekUsesWeekExpression,
        "SelectionRuleWeekUsesWeekExpression"
    );

    /// A day-of-month selection rule uses the `[dayE]` rule family.
    ///
    /// Normative source: CalConnect CC 18012:2018 §5.2 — Selection of calendar day of month
    pub struct SelectionRuleDayOfMonthUsesDayExpression;
    structural_prop!(
        SelectionRuleDayOfMonthUsesDayExpression,
        "SelectionRuleDayOfMonthUsesDayExpression"
    );

    /// A weekday selection rule uses the `[daykE]` rule family.
    ///
    /// Normative source: CalConnect CC 18012:2018 §5.2 — Selection of week days
    pub struct SelectionRuleWeekDayUsesDayOfWeekExpression;
    structural_prop!(
        SelectionRuleWeekDayUsesDayOfWeekExpression,
        "SelectionRuleWeekDayUsesDayOfWeekExpression"
    );

    /// An ordinal-day-of-year selection rule uses the `[dayoE(m)]` rule family.
    ///
    /// Normative source: CalConnect CC 18012:2018 §5.2 — Selection of ordinal days in calendar year
    pub struct SelectionRuleOrdinalDayOfYearUsesOrdinalDayExpression;
    structural_prop!(
        SelectionRuleOrdinalDayOfYearUsesOrdinalDayExpression,
        "SelectionRuleOrdinalDayOfYearUsesOrdinalDayExpression"
    );

    /// An hour-selection rule uses the `[hourE]` rule family.
    ///
    /// Normative source: CalConnect CC 18012:2018 §5.2 — Selection of hours
    pub struct SelectionRuleHourUsesHourExpression;
    structural_prop!(
        SelectionRuleHourUsesHourExpression,
        "SelectionRuleHourUsesHourExpression"
    );

    /// A minute-selection rule uses the `[minE]` rule family.
    ///
    /// Normative source: CalConnect CC 18012:2018 §5.2 — Selection of minutes
    pub struct SelectionRuleMinuteUsesMinuteExpression;
    structural_prop!(
        SelectionRuleMinuteUsesMinuteExpression,
        "SelectionRuleMinuteUsesMinuteExpression"
    );

    /// A second-selection rule uses the `[secE]` rule family.
    ///
    /// Normative source: CalConnect CC 18012:2018 §5.2 — Selection of seconds
    pub struct SelectionRuleSecondUsesSecondExpression;
    structural_prop!(
        SelectionRuleSecondUsesSecondExpression,
        "SelectionRuleSecondUsesSecondExpression"
    );

    /// A position-selection rule uses an integer followed by the instance designator.
    ///
    /// Normative source: CalConnect CC 18012:2018 §5.2 — Selection of position
    pub struct SelectionRulePositionUsesInstanceDesignatorSuffix;
    structural_prop!(
        SelectionRulePositionUsesInstanceDesignatorSuffix,
        "SelectionRulePositionUsesInstanceDesignatorSuffix"
    );

    /// Selection rules apply within the results selected by prior components.
    ///
    /// Normative source: CalConnect CC 18012:2018 §5.3 — Application within representations
    pub struct SelectionRulesApplyWithinSelectedResults;
    structural_prop!(
        SelectionRulesApplyWithinSelectedResults,
        "SelectionRulesApplyWithinSelectedResults"
    );

    /// A selection expression may select a single instance by using the instance designator.
    ///
    /// Normative source: CalConnect CC 18012:2018 §5.1 — General
    pub struct SelectionExpressionMaySelectSingleInstance;
    structural_prop!(
        SelectionExpressionMaySelectSingleInstance,
        "SelectionExpressionMaySelectSingleInstance"
    );

    /// Position-based selection is applied after the preceding selection rules.
    ///
    /// Normative source: CalConnect CC 18012:2018 §5.2 — Selection rules
    pub struct SelectionRulePositionAppliesLast;
    structural_prop!(
        SelectionRulePositionAppliesLast,
        "SelectionRulePositionAppliesLast"
    );

    /// Selection-with-duration extends a selection component by an explicit duration suffix.
    ///
    /// Normative source: CalConnect CC 18012:2018 §5.2 — Selection rules
    pub struct SelectionWithDurationUsesDurationSuffix;
    structural_prop!(
        SelectionWithDurationUsesDurationSuffix,
        "SelectionWithDurationUsesDurationSuffix"
    );

    /// A repeat rule uses the frequency designator `F`.
    ///
    /// Normative source: CalConnect CC 18012:2018 §6.3.1 — General
    pub struct RepeatRuleUsesFrequencyDesignator;
    structural_prop!(
        RepeatRuleUsesFrequencyDesignator,
        "RepeatRuleUsesFrequencyDesignator"
    );

    /// A repeat rule explicitly declares its eligible time intervals.
    ///
    /// Normative source: CalConnect CC 18012:2018 §6.3.2 — Eligible time intervals
    pub struct RepeatRuleDeclaresEligibleTimeIntervals;
    structural_prop!(
        RepeatRuleDeclaresEligibleTimeIntervals,
        "RepeatRuleDeclaresEligibleTimeIntervals"
    );

    /// Repeat-rule selection applies within the eligible intervals of the repeating cycle.
    ///
    /// Normative source: CalConnect CC 18012:2018 §6.3.3 — Selection part and selection rules
    pub struct RepeatRuleSelectionAppliesWithinEligibleIntervals;
    structural_prop!(
        RepeatRuleSelectionAppliesWithinEligibleIntervals,
        "RepeatRuleSelectionAppliesWithinEligibleIntervals"
    );

    /// Repeat-rule evaluation inherits time-scale component information from the initial start date.
    ///
    /// Normative source: CalConnect CC 18012:2018 §6.5 — Evaluation of repeat rule
    pub struct RepeatRuleEvaluationInheritsInitialStartComponentInformation;
    structural_prop!(
        RepeatRuleEvaluationInheritsInitialStartComponentInformation,
        "RepeatRuleEvaluationInheritsInitialStartComponentInformation"
    );

    /// A recurring representation with a repeat rule uses the complete `R.../.../...` form.
    ///
    /// Normative source: CalConnect CC 18012:2018 §6.4 — Complete representation
    pub struct RecurringIntervalWithRepeatRuleUsesCompleteRepresentation;
    structural_prop!(
        RecurringIntervalWithRepeatRuleUsesCompleteRepresentation,
        "RecurringIntervalWithRepeatRuleUsesCompleteRepresentation"
    );
}

pub use emit_impls::{
    ContextDependentDurationSemanticsDeclared, DateTimeFormulaCombinesTemporalValueWithDuration,
    DateTimeFormulaEvaluationModeDeclared, DateTimeFormulaTruncatesAtComponentBoundaries,
    DateTimeFormulaUsesCarryOverSemantics, ExactDurationSemanticsDeclared,
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
    ExplicitTimeIntervalDurationSubstitutionInfersMissingBoundary,
    ExplicitTimeIntervalLeadingShiftAppliesToTrailingComponentUnlessOverridden,
    ExplicitTimeIntervalTrailingEndMayInheritHigherOrderComponents,
    ExplicitTimeIntervalUsesDateTimeEndpointFamily, ExplicitTimeOfDayForbidsEndOfDayRepresentation,
    ExplicitTimeOfDayUsesHourMinuteSecondUnitDesignators, ExplicitTimeOfDayUsesTimeDesignator,
    ExplicitTimeOfDayWithShiftUsesTimeThenShiftConcatenation,
    ExplicitTimeShiftBareZuluRepresentsUtcZero, ExplicitTimeShiftPayloadUsesExplicitTimeOfDay,
    ExplicitTimeShiftUsesLeadingMinusOnlyWhenBehindUtc, ExplicitTimeShiftUsesZuluDesignator,
    ExplicitUtcRelationshipUsesZuluOrSignedShift,
    GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits, GroupedTimeScaleUnitConvertsToTimeInterval,
    GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift,
    GroupedTimeScaleUnitDefinitionIsContinuous,
    GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds,
    GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder, GroupedTimeScaleUnitUsesGroupingDesignators,
    GroupedTimeScaleUnitValueCarriesExplicitCoefficient,
    RecurringIntervalWithRepeatRuleUsesCompleteRepresentation,
    RepeatRuleDeclaresEligibleTimeIntervals,
    RepeatRuleEvaluationInheritsInitialStartComponentInformation,
    RepeatRuleSelectionAppliesWithinEligibleIntervals, RepeatRuleUsesFrequencyDesignator,
    SelectionExpressionMaySelectSingleInstance,
    SelectionExpressionUsesRecognizedSelectionRuleVocabulary,
    SelectionExpressionUsesSelectionDelimiters, SelectionRuleDayOfMonthUsesDayExpression,
    SelectionRuleHourUsesHourExpression, SelectionRuleMinuteUsesMinuteExpression,
    SelectionRuleMonthUsesMonthExpression, SelectionRuleOrdinalDayOfYearUsesOrdinalDayExpression,
    SelectionRulePositionAppliesLast, SelectionRulePositionUsesInstanceDesignatorSuffix,
    SelectionRuleSecondUsesSecondExpression, SelectionRuleWeekDayUsesDayOfWeekExpression,
    SelectionRuleWeekUsesWeekExpression, SelectionRulesApplyWithinSelectedResults,
    SelectionWithDurationUsesDurationSuffix, SpeculativeDurationSemanticsDeclared,
};
