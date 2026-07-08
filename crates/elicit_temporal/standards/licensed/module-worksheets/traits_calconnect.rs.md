# Citation Worksheet: `src/traits/calconnect.rs`

This worksheet tracks standards concordance for the CalConnect-facing temporal
extension doorway in `src/traits/calconnect.rs`.

## Source set

- `../iso-8601-2-2019.*`
- `../../public/calconnect-cc-18011-2018.xml`
- `../../public/calconnect-cc-18012-2018.xml`

## Standards seam map

| Surface item | Current topic label | Exact clause | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `TemporalCalConnectFactory` | `qualification, explicit forms, grouped time scale units, date-time formulas, selection rules, and repeat-rule recurrence` | `ISO 8601-2:2019 8.2.1; 8.2.2; 8.2.3; 8.4.4; 8.4.5; 8.4.6; 8.5; CC 18011:2018 §4.3; §5; §6; §7; §8; CC 18012:2018 §5; §6.3; §6.4` | `iso-8601-2-2019.sample.txt lines 280-345; ../../public/calconnect-cc-18011-2018.xml lines 276-1444; ../../public/calconnect-cc-18012-2018.xml lines 276-807` | `yes` |
| `parse_qualified_temporal_value` | `qualification of temporal expressions` | `ISO 8601-2:2019 8.2.1; 8.2.2; 8.2.3; 8.4.4; 8.4.5; 8.4.6; 8.5` | `iso-8601-2-2019.sample.txt lines 280-345` | `yes` |
| `format_qualified_temporal_value` | `qualification of temporal expressions` | `ISO 8601-2:2019 8.2.1; 8.2.2; 8.2.3; 8.4.4; 8.4.5; 8.4.6; 8.5` | `iso-8601-2-2019.sample.txt lines 280-345` | `yes` |
| `parse_explicit_temporal_form` | `explicit forms` | `CC 18011:2018 §4.3` | `../../public/calconnect-cc-18011-2018.xml lines 276-594` | `yes` |
| `format_explicit_temporal_form` | `explicit forms` | `CC 18011:2018 §4.3` | `../../public/calconnect-cc-18011-2018.xml lines 276-594` | `yes` |
| `parse_explicit_duration` | `explicit duration` | `CC 18011:2018 §7` | `../../public/calconnect-cc-18011-2018.xml lines 1097-1290` | `yes` |
| `format_explicit_duration` | `explicit duration` | `CC 18011:2018 §7` | `../../public/calconnect-cc-18011-2018.xml lines 1097-1290` | `yes` |
| `parse_explicit_time_interval` | `explicit time interval` | `CC 18011:2018 §6` | `../../public/calconnect-cc-18011-2018.xml lines 1053-1095` | `yes` |
| `format_explicit_time_interval` | `explicit time interval` | `CC 18011:2018 §6` | `../../public/calconnect-cc-18011-2018.xml lines 1053-1095` | `yes` |
| `parse_grouped_time_scale_unit` | `grouped time scale units` | `CC 18011:2018 §5` | `../../public/calconnect-cc-18011-2018.xml lines 595-1051` | `yes` |
| `format_grouped_time_scale_unit` | `grouped time scale units` | `CC 18011:2018 §5` | `../../public/calconnect-cc-18011-2018.xml lines 595-1051` | `yes` |
| `parse_date_time_formula` | `evaluation of date and time with duration` | `CC 18011:2018 §8` | `../../public/calconnect-cc-18011-2018.xml lines 1292-1444` | `yes` |
| `format_date_time_formula` | `evaluation of date and time with duration` | `CC 18011:2018 §8` | `../../public/calconnect-cc-18011-2018.xml lines 1292-1444` | `yes` |
| `evaluate_date_time_formula` | `evaluation of date and time with duration` | `CC 18011:2018 §8` | `../../public/calconnect-cc-18011-2018.xml lines 1292-1444` | `yes` |
| `parse_selection_expression` | `selection of date and time` | `CC 18012:2018 §5` | `../../public/calconnect-cc-18012-2018.xml lines 276-605` | `yes` |
| `format_selection_expression` | `selection of date and time` | `CC 18012:2018 §5` | `../../public/calconnect-cc-18012-2018.xml lines 276-605` | `yes` |
| `parse_repeat_rule` | `repeat rule` | `CC 18012:2018 §6.3` | `../../public/calconnect-cc-18012-2018.xml lines 629-699, 758-807` | `yes` |
| `format_repeat_rule` | `repeat rule` | `CC 18012:2018 §6.3` | `../../public/calconnect-cc-18012-2018.xml lines 629-699, 758-807` | `yes` |
| `parse_recurring_interval_with_repeat_rule` | `complete recurring interval representation with repeat rule` | `CC 18012:2018 §6.4` | `../../public/calconnect-cc-18012-2018.xml lines 701-751` | `yes` |
| `format_recurring_interval_with_repeat_rule` | `complete recurring interval representation with repeat rule` | `CC 18012:2018 §6.4` | `../../public/calconnect-cc-18012-2018.xml lines 701-751` | `yes` |

## CalConnect Trait Coverage Audit

| Surface item | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `qualified temporal value doorway` | `covered` | `parse_qualified_temporal_value`, `format_qualified_temporal_value`, `ParsedQualifiedTemporalValueResult`, `FormattedQualifiedTemporalValueResult`, `QualifiedTemporalExpressionEvidence`, `QualificationPlacementEvidence`, `QualifiedTemporalValueEvidence`, neutral parser/formatter mirrors | The CalConnect trait keeps the section-cited doorway while preserving the same explicit qualification-law sidecars as the ISO-neutral parser and formatter surfaces: qualified-value validity, qualification-expression validity, and placement-family proof all cross the seam together. |
| `explicit-form doorway` | `covered` | `parse_explicit_temporal_form`, `format_explicit_temporal_form`, `ParsedExplicitTemporalFormResult`, `FormattedExplicitTemporalFormResult`, `ExplicitTemporalFormEvidence` | Explicit-form designator use, zero-omission authority, precision declaration, and UTC-relationship syntax now cross the seam as separate proof sidecars rather than being flattened into one aggregate validity token. |
| `explicit-duration and explicit-interval doorway` | `covered` | `parse_explicit_duration`, `format_explicit_duration`, `ParsedExplicitDurationResult`, `FormattedExplicitDurationResult`, `ExplicitDurationEvidence`, `ExplicitDurationRepresentationEvidence`, `ExplicitDurationSemanticEvidence`, `parse_explicit_time_interval`, `format_explicit_time_interval`, explicit interval proof branches | Explicit-duration seams now carry unit-designator usage, representation family, negative-sign authority, fractional-lowest-unit authority, and exactness-family semantics explicitly, while explicit-interval seams continue to expose duration substitution, trailing-end inheritance, and leading-shift propagation directly. |
| `grouped-unit and formula doorway` | `covered` | `parse_grouped_time_scale_unit`, `format_grouped_time_scale_unit`, `ParsedGroupedTimeScaleUnitResult`, `FormattedGroupedTimeScaleUnitResult`, `GroupedTimeScaleUnitEvidence`, `parse_date_time_formula`, `format_date_time_formula`, `evaluate_date_time_formula`, `EvaluatedDateTimeFormulaResult`, `DateTimeFormulaEvaluationResultEvidence` | Grouped-unit seams now carry delimiter, unit-carriage, continuity, coefficient, bounds, explicit-time-shift, truncation, and interval-conversion sidecars explicitly, while formula seams now preserve both declared evaluation semantics and a dedicated evaluation-result provenance token so downstream consumers can distinguish a lawful formula result from an arbitrary explicit form. |
| `selection and repeat-rule doorway` | `covered` | `parse_selection_expression`, `format_selection_expression`, `ParsedSelectionExpressionResult`, `FormattedSelectionExpressionResult`, `SelectionExpressionEvidence`, `parse_repeat_rule`, `format_repeat_rule`, `ParsedRepeatRuleResult`, `FormattedRepeatRuleResult`, `RepeatRuleEvidence` | Selection seams now carry delimiter, vocabulary, component-rule, nesting, single-instance, positional, and duration-window sidecars explicitly, while repeat-rule seams now carry frequency, eligible-interval, embedded-selection, and initial-start inheritance sidecars explicitly rather than flattening those laws into coarse validity tokens. |
| `recurring interval with repeat rule doorway` | `covered` | `parse_recurring_interval_with_repeat_rule`, `format_recurring_interval_with_repeat_rule`, `ParsedRecurringIntervalWithRepeatRuleResult`, `FormattedRecurringIntervalWithRepeatRuleResult`, `RecurringIntervalWithRepeatRuleIntervalDescriptor`, `RecurringIntervalWithRepeatRuleIntervalProofBranch`, `RepeatRuleEvidence` | The seam now preserves both embedded-law families explicitly: the interval family remains split between ISO complete and CalConnect explicit interval representations, and the attached repeat rule re-issues frequency, eligible-interval, embedded-selection, and initial-start inheritance sidecars rather than collapsing them back into coarse repeat-rule validity. |

## Checklist

- [x] Add trait-level concordance for CalConnect explicit duration and explicit
  interval seams.
- [x] Add trait-level concordance for CalConnect selection and repeat-rule
  seams.
- [x] Record that the recurring repeat-rule seam preserves the embedded
  interval family and proof branch explicitly.
