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
    EvaluatedDateTimeFormulaResult, ExplicitDurationDescriptor, ExplicitDurationValid,
    ExplicitTemporalFormDescriptor, ExplicitTemporalFormValid, FormattedDateTimeFormulaResult,
    FormattedExplicitDurationResult, FormattedExplicitTemporalFormResult,
    FormattedGroupedTimeScaleUnitResult, FormattedQualifiedTemporalValueResult,
    FormattedRecurringIntervalWithRepeatRuleResult, FormattedRepeatRuleResult,
    FormattedSelectionExpressionResult, GroupedTimeScaleUnitDescriptor, GroupedTimeScaleUnitValid,
    ParsedDateTimeFormulaResult, ParsedExplicitDurationResult, ParsedExplicitTemporalFormResult,
    ParsedGroupedTimeScaleUnitResult, ParsedQualifiedTemporalValueResult,
    ParsedRecurringIntervalWithRepeatRuleResult, ParsedRepeatRuleResult,
    ParsedSelectionExpressionResult, QualifiedTemporalValueDescriptor, QualifiedTemporalValueValid,
    RecurringIntervalWithRepeatRuleDescriptor, RecurringIntervalWithRepeatRuleValid,
    RepeatRuleDescriptor, RepeatRuleValid, SelectionExpressionDescriptor, SelectionExpressionValid,
};

/// Parse, format, and evaluate the CalConnect extensions layered on top of ISO 8601.
///
/// Normative sources:
/// - CalConnect CC 18011:2018 — explicit forms, grouped units, and date-time formulas
/// - CalConnect CC 18012:2018 — selection rules and repeat rules
pub trait TemporalCalConnectFactory: Send + Sync {
    /// Parse a top-level temporal value carrying an explicit qualification sidecar.
    ///
    /// Normative source: ISO 8601-2:2019 — qualification of temporal expressions.
    fn parse_qualified_temporal_value(&self, input: &str) -> ParsedQualifiedTemporalValueResult;

    /// Emit a qualified temporal value.
    ///
    /// Normative source: ISO 8601-2:2019 — qualification of temporal expressions.
    fn format_qualified_temporal_value(
        &self,
        value: &QualifiedTemporalValueDescriptor,
        value_proof: Established<QualifiedTemporalValueValid>,
    ) -> FormattedQualifiedTemporalValueResult;

    /// Parse a CalConnect explicit temporal form.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3 — Explicit forms.
    fn parse_explicit_temporal_form(&self, input: &str) -> ParsedExplicitTemporalFormResult;

    /// Emit a CalConnect explicit temporal form.
    ///
    /// Normative source: CalConnect CC 18011:2018 §4.3 — Explicit forms.
    fn format_explicit_temporal_form(
        &self,
        form: &ExplicitTemporalFormDescriptor,
        form_proof: Established<ExplicitTemporalFormValid>,
    ) -> FormattedExplicitTemporalFormResult;

    /// Parse a CalConnect explicit duration.
    ///
    /// Normative source: CalConnect CC 18011:2018 §7 — Explicit duration.
    fn parse_explicit_duration(&self, input: &str) -> ParsedExplicitDurationResult;

    /// Emit a CalConnect explicit duration.
    ///
    /// Normative source: CalConnect CC 18011:2018 §7 — Explicit duration.
    fn format_explicit_duration(
        &self,
        duration: &ExplicitDurationDescriptor,
        duration_proof: Established<ExplicitDurationValid>,
    ) -> FormattedExplicitDurationResult;

    /// Parse a grouped time scale unit expression.
    ///
    /// Normative source: CalConnect CC 18011:2018 §5 — Grouped time scale units.
    fn parse_grouped_time_scale_unit(&self, input: &str) -> ParsedGroupedTimeScaleUnitResult;

    /// Emit a grouped time scale unit expression.
    ///
    /// Normative source: CalConnect CC 18011:2018 §5 — Grouped time scale units.
    fn format_grouped_time_scale_unit(
        &self,
        grouped: &GroupedTimeScaleUnitDescriptor,
        grouped_proof: Established<GroupedTimeScaleUnitValid>,
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
    /// Normative source: CalConnect CC 18011:2018 §8 — Evaluation of date and time with duration.
    fn evaluate_date_time_formula(
        &self,
        formula: &DateTimeFormulaDescriptor,
        formula_proof: Established<DateTimeFormulaValid>,
        semantics_proof: Established<DateTimeFormulaEvaluationSemanticsValid>,
    ) -> EvaluatedDateTimeFormulaResult;

    /// Parse a selection expression.
    ///
    /// Normative source: CalConnect CC 18012:2018 §5 — Selection.
    fn parse_selection_expression(&self, input: &str) -> ParsedSelectionExpressionResult;

    /// Emit a selection expression.
    ///
    /// Normative source: CalConnect CC 18012:2018 §5 — Selection.
    fn format_selection_expression(
        &self,
        selection: &SelectionExpressionDescriptor,
        selection_proof: Established<SelectionExpressionValid>,
    ) -> FormattedSelectionExpressionResult;

    /// Parse a repeat rule.
    ///
    /// Normative source: CalConnect CC 18012:2018 §6.3 — Repeat rule.
    fn parse_repeat_rule(&self, input: &str) -> ParsedRepeatRuleResult;

    /// Emit a repeat rule.
    ///
    /// Normative source: CalConnect CC 18012:2018 §6.3 — Repeat rule.
    fn format_repeat_rule(
        &self,
        rule: &RepeatRuleDescriptor,
        rule_proof: Established<RepeatRuleValid>,
    ) -> FormattedRepeatRuleResult;

    /// Parse a complete recurring interval representation with a repeat rule.
    ///
    /// Normative source: CalConnect CC 18012:2018 §6.4 — Complete representation.
    fn parse_recurring_interval_with_repeat_rule(
        &self,
        input: &str,
    ) -> ParsedRecurringIntervalWithRepeatRuleResult;

    /// Emit a complete recurring interval representation with a repeat rule.
    ///
    /// Normative source: CalConnect CC 18012:2018 §6.4 — Complete representation.
    fn format_recurring_interval_with_repeat_rule(
        &self,
        interval: &RecurringIntervalWithRepeatRuleDescriptor,
        interval_proof: Established<RecurringIntervalWithRepeatRuleValid>,
    ) -> FormattedRecurringIntervalWithRepeatRuleResult;
}
