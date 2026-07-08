//! CalConnect-authored explicit-form and recurrence extension traits.
//!
//! Shared ISO-neutral families such as qualified temporal values, grouped time
//! scale units, and date-time formulas are also mirrored on
//! [`TemporalParser`](crate::TemporalParser) and
//! [`TemporalFormatter`](crate::TemporalFormatter). This trait retains the
//! directly section-cited CalConnect doorway plus formula evaluation and the
//! recurrence-specific extension families.

use elicitation::Established;

use crate::{
    DateTimeFormulaDescriptor, DateTimeFormulaEvaluationSemanticsValid, DateTimeFormulaValid,
    EvaluatedDateTimeFormulaResult, ExplicitDurationDescriptor, ExplicitDurationMayBeNegative,
    ExplicitDurationMayUseFractionalLowestOrderUnit, ExplicitDurationRepresentationEvidence,
    ExplicitDurationSemanticEvidence, ExplicitDurationUsesDurationalUnitDesignators,
    ExplicitDurationValid, ExplicitIntervalDurationSubstitutionProofBranch,
    ExplicitIntervalEndComponentInheritanceProofBranch,
    ExplicitIntervalShiftPropagationProofBranch, ExplicitTemporalFormDescriptor,
    ExplicitTemporalFormMayOmitZeroValuedComponents, ExplicitTemporalFormUsesDesignatorSymbols,
    ExplicitTemporalFormValid, ExplicitTemporalPrecisionUsesLowestDenotedComponent,
    ExplicitTimeIntervalDescriptor, ExplicitTimeIntervalValid,
    ExplicitUtcRelationshipUsesZuluOrSignedShift, FormattedDateTimeFormulaResult,
    FormattedExplicitDurationResult, FormattedExplicitTemporalFormResult,
    FormattedExplicitTimeIntervalResult, FormattedGroupedTimeScaleUnitResult,
    FormattedQualifiedTemporalValueResult, FormattedRecurringIntervalWithRepeatRuleResult,
    FormattedRepeatRuleResult, FormattedSelectionExpressionResult,
    GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits, GroupedTimeScaleUnitConvertsToTimeInterval,
    GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift,
    GroupedTimeScaleUnitDefinitionIsContinuous, GroupedTimeScaleUnitDescriptor,
    GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds,
    GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder, GroupedTimeScaleUnitUsesGroupingDesignators,
    GroupedTimeScaleUnitValid, GroupedTimeScaleUnitValueCarriesExplicitCoefficient,
    ParsedDateTimeFormulaResult, ParsedExplicitDurationResult, ParsedExplicitTemporalFormResult,
    ParsedExplicitTimeIntervalResult, ParsedGroupedTimeScaleUnitResult,
    ParsedQualifiedTemporalValueResult, ParsedRecurringIntervalWithRepeatRuleResult,
    ParsedRepeatRuleResult, ParsedSelectionExpressionResult, QualificationPlacementEvidence,
    QualifiedTemporalExpressionValid, QualifiedTemporalValueDescriptor,
    QualifiedTemporalValueValid, RecurringIntervalWithRepeatRuleDescriptor,
    RecurringIntervalWithRepeatRuleIntervalProofBranch, RecurringIntervalWithRepeatRuleValid,
    RepeatRuleDeclaresEligibleTimeIntervals, RepeatRuleDescriptor,
    RepeatRuleEvaluationInheritsInitialStartComponentInformation,
    RepeatRuleSelectionAppliesWithinEligibleIntervals, RepeatRuleUsesFrequencyDesignator,
    RepeatRuleValid, SelectionExpressionDescriptor, SelectionExpressionMaySelectSingleInstance,
    SelectionExpressionUsesRecognizedSelectionRuleVocabulary,
    SelectionExpressionUsesSelectionDelimiters, SelectionExpressionValid,
    SelectionRuleDayOfMonthUsesDayExpression, SelectionRuleHourUsesHourExpression,
    SelectionRuleMinuteUsesMinuteExpression, SelectionRuleMonthUsesMonthExpression,
    SelectionRuleOrdinalDayOfYearUsesOrdinalDayExpression, SelectionRulePositionAppliesLast,
    SelectionRulePositionUsesInstanceDesignatorSuffix, SelectionRuleSecondUsesSecondExpression,
    SelectionRuleWeekDayUsesDayOfWeekExpression, SelectionRuleWeekUsesWeekExpression,
    SelectionRulesApplyWithinSelectedResults, SelectionWithDurationUsesDurationSuffix,
};

/// Parse, format, and evaluate the CalConnect extensions layered on top of ISO 8601.
///
/// Normative sources:
/// - CalConnect CC 18011:2018 — explicit forms, grouped units, and date-time formulas
/// - CalConnect CC 18012:2018 — selection rules and repeat rules
pub trait TemporalCalConnectFactory: Send + Sync {
    /// Parse a top-level temporal value carrying an explicit qualification sidecar.
    ///
    /// The returned exchange shape keeps the qualification-law sidecars
    /// explicit: qualified-value validity, qualification-expression validity,
    /// and placement-family proof all cross the seam together.
    ///
    /// Normative source: ISO 8601-2:2019, 8.2.1, 8.2.2, 8.2.3, 8.4.4, 8.4.5,
    /// 8.4.6, and 8.5.
    fn parse_qualified_temporal_value(&self, input: &str) -> ParsedQualifiedTemporalValueResult;

    /// Emit a qualified temporal value.
    ///
    /// The caller must supply the same qualification-law sidecars that the
    /// parser produces, so higher-order seams cannot silently depend on hidden
    /// placement or scope semantics.
    ///
    /// Normative source: ISO 8601-2:2019, 8.2.1, 8.2.2, 8.2.3, 8.4.4, 8.4.5,
    /// 8.4.6, and 8.5.
    fn format_qualified_temporal_value(
        &self,
        value: &QualifiedTemporalValueDescriptor,
        value_proof: Established<QualifiedTemporalValueValid>,
        qualification_proof: Established<QualifiedTemporalExpressionValid>,
        placement_proof: QualificationPlacementEvidence,
    ) -> FormattedQualifiedTemporalValueResult;

    /// Parse a CalConnect explicit temporal form.
    ///
    /// The returned exchange shape carries the explicit-form law directly:
    /// designator use, zero-omission authority, precision declaration, and
    /// UTC-relationship syntax all cross the seam as separate proof sidecars.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3 — Explicit forms.
    fn parse_explicit_temporal_form(&self, input: &str) -> ParsedExplicitTemporalFormResult;

    /// Emit a CalConnect explicit temporal form.
    ///
    /// The caller must supply explicit-form law sidecars for designator use,
    /// zero-omission authority, precision declaration, and UTC-relationship
    /// syntax rather than relying on a single aggregate validity token.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3 — Explicit forms.
    fn format_explicit_temporal_form(
        &self,
        form: &ExplicitTemporalFormDescriptor,
        form_proof: Established<ExplicitTemporalFormValid>,
        designators: Established<ExplicitTemporalFormUsesDesignatorSymbols>,
        zero_omission: Established<ExplicitTemporalFormMayOmitZeroValuedComponents>,
        precision: Established<ExplicitTemporalPrecisionUsesLowestDenotedComponent>,
        utc_relationship: Established<ExplicitUtcRelationshipUsesZuluOrSignedShift>,
    ) -> FormattedExplicitTemporalFormResult;

    /// Parse a CalConnect explicit duration.
    ///
    /// The returned exchange shape keeps the explicit-duration law visible:
    /// unit-designator usage, representation family, negative-sign authority,
    /// fractional-lowest-unit authority, and exactness-family semantics all
    /// cross the seam explicitly.
    ///
    /// Normative source: CalConnect CC 18011:2018 §7 — Explicit duration.
    fn parse_explicit_duration(&self, input: &str) -> ParsedExplicitDurationResult;

    /// Parse a CalConnect explicit time interval.
    ///
    /// Normative source: CalConnect CC 18011:2018 §6 — Time interval.
    fn parse_explicit_time_interval(&self, input: &str) -> ParsedExplicitTimeIntervalResult;

    /// Emit a CalConnect explicit duration.
    ///
    /// The caller must supply the explicit-duration law sidecars directly
    /// rather than relying on one aggregate validity token.
    ///
    /// Normative source: CalConnect CC 18011:2018 §7 — Explicit duration.
    fn format_explicit_duration(
        &self,
        duration: &ExplicitDurationDescriptor,
        duration_proof: Established<ExplicitDurationValid>,
        units: Established<ExplicitDurationUsesDurationalUnitDesignators>,
        representation: ExplicitDurationRepresentationEvidence,
        sign: Established<ExplicitDurationMayBeNegative>,
        fractional: Established<ExplicitDurationMayUseFractionalLowestOrderUnit>,
        semantics: ExplicitDurationSemanticEvidence,
    ) -> FormattedExplicitDurationResult;

    /// Emit a CalConnect explicit time interval.
    ///
    /// Normative source: CalConnect CC 18011:2018 §6 — Time interval.
    fn format_explicit_time_interval(
        &self,
        interval: &ExplicitTimeIntervalDescriptor,
        interval_proof: Established<ExplicitTimeIntervalValid>,
        duration_substitution: ExplicitIntervalDurationSubstitutionProofBranch,
        end_component_inheritance: ExplicitIntervalEndComponentInheritanceProofBranch,
        shift_propagation: ExplicitIntervalShiftPropagationProofBranch,
    ) -> FormattedExplicitTimeIntervalResult;

    /// Parse a grouped time scale unit expression.
    ///
    /// The returned exchange shape carries grouped-unit law explicitly:
    /// delimiter syntax, non-empty unit carriage, continuity, coefficient
    /// declaration, lower-order bounds, explicit time-shift authority,
    /// truncation semantics, and interval-conversion semantics all cross the
    /// seam as proof sidecars.
    ///
    /// Normative source: CalConnect CC 18011:2018 §5 — Grouped time scale units.
    fn parse_grouped_time_scale_unit(&self, input: &str) -> ParsedGroupedTimeScaleUnitResult;

    /// Emit a grouped time scale unit expression.
    ///
    /// The caller must supply the same grouped-unit law sidecars directly
    /// rather than relying on one aggregate validity token.
    ///
    /// Normative source: CalConnect CC 18011:2018 §5 — Grouped time scale units.
    fn format_grouped_time_scale_unit(
        &self,
        grouped: &GroupedTimeScaleUnitDescriptor,
        grouped_proof: Established<GroupedTimeScaleUnitValid>,
        designators: Established<GroupedTimeScaleUnitUsesGroupingDesignators>,
        units: Established<GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits>,
        continuity: Established<GroupedTimeScaleUnitDefinitionIsContinuous>,
        coefficient: Established<GroupedTimeScaleUnitValueCarriesExplicitCoefficient>,
        bounds: Established<GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds>,
        explicit_time_shift: Established<GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift>,
        truncation: Established<GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder>,
        interval_semantics: Established<GroupedTimeScaleUnitConvertsToTimeInterval>,
    ) -> FormattedGroupedTimeScaleUnitResult;

    /// Parse a date-time formula.
    ///
    /// Normative source: CalConnect CC 18011:2018 §8 — Evaluation of date and time with duration.
    fn parse_date_time_formula(&self, input: &str) -> ParsedDateTimeFormulaResult;

    /// Emit a date-time formula.
    ///
    /// Normative source: CalConnect CC 18011:2018 §8 — Evaluation of date and time with duration.
    fn format_date_time_formula(
        &self,
        formula: &DateTimeFormulaDescriptor,
        formula_proof: Established<DateTimeFormulaValid>,
        semantics_proof: Established<DateTimeFormulaEvaluationSemanticsValid>,
    ) -> FormattedDateTimeFormulaResult;

    /// Evaluate a date-time formula into its resolved explicit form.
    ///
    /// The returned exchange shape preserves evaluation-result provenance
    /// explicitly: downstream consumers receive not only a valid explicit
    /// temporal form, but also a dedicated proof that this form was lawfully
    /// produced by formula evaluation under the declared CalConnect semantics.
    ///
    /// Normative source: CalConnect CC 18011:2018 §8 — Evaluation of date and time with duration.
    fn evaluate_date_time_formula(
        &self,
        formula: &DateTimeFormulaDescriptor,
        formula_proof: Established<DateTimeFormulaValid>,
        semantics_proof: Established<DateTimeFormulaEvaluationSemanticsValid>,
    ) -> EvaluatedDateTimeFormulaResult;

    /// Parse a selection expression.
    ///
    /// The returned exchange shape keeps the selection-law surface explicit:
    /// delimiters, recognized rule vocabulary, component-specific rule
    /// families, nesting semantics, single-instance authority, positional
    /// selection syntax and ordering, and duration-window semantics all cross
    /// the seam as proof sidecars.
    ///
    /// Normative source: CalConnect CC 18012:2018 §5 — Selection.
    fn parse_selection_expression(&self, input: &str) -> ParsedSelectionExpressionResult;

    /// Emit a selection expression.
    ///
    /// The caller must supply the selection-law sidecars directly rather than
    /// relying on one aggregate validity token.
    ///
    /// Normative source: CalConnect CC 18012:2018 §5 — Selection.
    fn format_selection_expression(
        &self,
        selection: &SelectionExpressionDescriptor,
        selection_proof: Established<SelectionExpressionValid>,
        delimiters: Established<SelectionExpressionUsesSelectionDelimiters>,
        vocabulary: Established<SelectionExpressionUsesRecognizedSelectionRuleVocabulary>,
        month_rule: Established<SelectionRuleMonthUsesMonthExpression>,
        week_rule: Established<SelectionRuleWeekUsesWeekExpression>,
        day_of_month_rule: Established<SelectionRuleDayOfMonthUsesDayExpression>,
        weekday_rule: Established<SelectionRuleWeekDayUsesDayOfWeekExpression>,
        ordinal_day_of_year_rule: Established<
            SelectionRuleOrdinalDayOfYearUsesOrdinalDayExpression,
        >,
        hour_rule: Established<SelectionRuleHourUsesHourExpression>,
        minute_rule: Established<SelectionRuleMinuteUsesMinuteExpression>,
        second_rule: Established<SelectionRuleSecondUsesSecondExpression>,
        nesting: Established<SelectionRulesApplyWithinSelectedResults>,
        single_instance: Established<SelectionExpressionMaySelectSingleInstance>,
        position_syntax: Established<SelectionRulePositionUsesInstanceDesignatorSuffix>,
        position: Established<SelectionRulePositionAppliesLast>,
        duration_window: Established<SelectionWithDurationUsesDurationSuffix>,
    ) -> FormattedSelectionExpressionResult;

    /// Parse a repeat rule.
    ///
    /// The returned exchange shape keeps repeat-law sidecars explicit:
    /// frequency syntax, eligible-interval declaration, embedded selection
    /// semantics, and inheritance from the initial start date all cross the
    /// seam directly.
    ///
    /// Normative source: CalConnect CC 18012:2018 §6.3 — Repeat rule.
    fn parse_repeat_rule(&self, input: &str) -> ParsedRepeatRuleResult;

    /// Emit a repeat rule.
    ///
    /// The caller must supply the repeat-law sidecars directly rather than
    /// relying on one aggregate validity token.
    ///
    /// Normative source: CalConnect CC 18012:2018 §6.3 — Repeat rule.
    fn format_repeat_rule(
        &self,
        rule: &RepeatRuleDescriptor,
        rule_proof: Established<RepeatRuleValid>,
        frequency: Established<RepeatRuleUsesFrequencyDesignator>,
        eligible_intervals: Established<RepeatRuleDeclaresEligibleTimeIntervals>,
        selection: Established<RepeatRuleSelectionAppliesWithinEligibleIntervals>,
        inheritance: Established<RepeatRuleEvaluationInheritsInitialStartComponentInformation>,
    ) -> FormattedRepeatRuleResult;

    /// Parse a complete recurring interval representation with a repeat rule.
    ///
    /// The returned exchange shape preserves both embedded-law families:
    /// interval-family proof branches remain explicit, and the attached repeat
    /// rule re-issues frequency, eligible-interval, embedded-selection, and
    /// initial-start inheritance sidecars rather than collapsing them back
    /// into coarse repeat-rule validity.
    ///
    /// Normative source: CalConnect CC 18012:2018 §6.4 — Complete representation.
    fn parse_recurring_interval_with_repeat_rule(
        &self,
        input: &str,
    ) -> ParsedRecurringIntervalWithRepeatRuleResult;

    /// Emit a complete recurring interval representation with a repeat rule.
    ///
    /// The caller must supply both embedded-law families explicitly: the
    /// interval-family branch and the attached repeat-rule sidecars.
    ///
    /// Normative source: CalConnect CC 18012:2018 §6.4 — Complete representation.
    fn format_recurring_interval_with_repeat_rule(
        &self,
        interval: &RecurringIntervalWithRepeatRuleDescriptor,
        interval_proof: Established<RecurringIntervalWithRepeatRuleValid>,
        interval_family: RecurringIntervalWithRepeatRuleIntervalProofBranch,
        frequency: Established<RepeatRuleUsesFrequencyDesignator>,
        eligible_intervals: Established<RepeatRuleDeclaresEligibleTimeIntervals>,
        selection: Established<RepeatRuleSelectionAppliesWithinEligibleIntervals>,
        inheritance: Established<RepeatRuleEvaluationInheritsInitialStartComponentInformation>,
    ) -> FormattedRecurringIntervalWithRepeatRuleResult;
}
