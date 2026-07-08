# Citation Worksheet: `src/contracts/calconnect.rs`

This worksheet records exhaustive coverage review for the CalConnect contract
surface in `src/contracts/calconnect.rs`.

## Source set

- `../../public/calconnect-cc-18011-2018.xml`
- `../../public/calconnect-cc-18012-2018.xml`

## CalConnect contract map

| Contract symbol | Current topic label | Exact section | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `ExplicitTemporalFormUsesDesignatorSymbols` | `explicit forms` | `CC 18011:2018 §4.3` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitTemporalFormMayOmitZeroValuedComponents` | `omission of zero valued components` | `CC 18011:2018 §4.3.6` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitTemporalPrecisionUsesLowestDenotedComponent` | `indication of precision` | `CC 18011:2018 §4.3.7` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitUtcRelationshipUsesZuluOrSignedShift` | `explicit time shift` | `CC 18011:2018 §4.3.5` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitTimeOfDayUsesTimeDesignator` | `local time of day` | `CC 18011:2018 §4.3.3` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitTimeOfDayUsesHourMinuteSecondUnitDesignators` | `local time of day` | `CC 18011:2018 §4.3.3` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitTimeOfDayForbidsEndOfDayRepresentation` | `beginning of the day` | `CC 18011:2018 §4.3.3.2` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitTimeShiftUsesZuluDesignator` | `time shift` | `CC 18011:2018 §4.3.4` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitTimeShiftUsesLeadingMinusOnlyWhenBehindUtc` | `time shift sign semantics` | `CC 18011:2018 §4.3.4` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitTimeShiftPayloadUsesExplicitTimeOfDay` | `time shift payload family` | `CC 18011:2018 §4.3.4` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitTimeShiftBareZuluRepresentsUtcZero` | `bare Z zero-shift semantics` | `CC 18011:2018 §4.3.4` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitDateTimeUsesDateThenTimeConcatenation` | `date and time only` | `CC 18011:2018 §4.3.4.3` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitDateTimeTimePortionMayBeReducedPrecision` | `date and time of day` | `CC 18011:2018 §4.3.4.3` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitDateWithShiftUsesDateThenShiftConcatenation` | `date with shift` | `CC 18011:2018 §4.3.4.1` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitTimeOfDayWithShiftUsesTimeThenShiftConcatenation` | `time of day with time shift` | `CC 18011:2018 §4.3.4.2` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitDateTimeWithShiftUsesDateTimeThenShiftConcatenation` | `date and time with shift` | `CC 18011:2018 §4.3.4.3` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitTimeIntervalUsesDateTimeEndpointFamily` | `time interval endpoint family` | `CC 18011:2018 §6` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitTimeIntervalDurationSubstitutionInfersMissingBoundary` | `time interval duration substitution` | `CC 18011:2018 §6` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitTimeIntervalTrailingEndMayInheritHigherOrderComponents` | `time interval trailing endpoint inheritance` | `CC 18011:2018 §6` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitTimeIntervalLeadingShiftAppliesToTrailingComponentUnlessOverridden` | `time interval leading shift propagation` | `CC 18011:2018 §6` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `GroupedTimeScaleUnitUsesGroupingDesignators` | `unit definition` | `CC 18011:2018 §5.1` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits` | `unit definition` | `CC 18011:2018 §5.1` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `GroupedTimeScaleUnitDefinitionIsContinuous` | `unit definition continuity` | `CC 18011:2018 §5.1` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `GroupedTimeScaleUnitValueCarriesExplicitCoefficient` | `unit value` | `CC 18011:2018 §5.2` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds` | `group boundary adherence` | `CC 18011:2018 §5.3.2` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder` | `truncation of partial units` | `CC 18011:2018 §5.3.4` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift` | `representation with time shift` | `CC 18011:2018 §5.3` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `GroupedTimeScaleUnitConvertsToTimeInterval` | `conversion to basic time scale units` | `CC 18011:2018 §5.3.3` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitDurationUsesDurationalUnitDesignators` | `durational units` | `CC 18011:2018 §7.2` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitDurationRepresentationKindDeclared` | `duration representation kind` | `CC 18011:2018 §7.3` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitDurationCompositeRepresentationDeclared` | `composite representation` | `CC 18011:2018 §7.4` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitDurationPrecedenceRepresentationCarriesEvaluationOrder` | `precedence representation` | `CC 18011:2018 §7.5` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitDurationMayBeNegative` | `negative duration` | `CC 18011:2018 §7.6` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitDurationMayUseFractionalLowestOrderUnit` | `fractional duration` | `CC 18011:2018 §7.7` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExactDurationSemanticsDeclared` | `exact duration` | `CC 18011:2018 §7.8` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ContextDependentDurationSemanticsDeclared` | `context-dependent duration` | `CC 18011:2018 §7.9` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `SpeculativeDurationSemanticsDeclared` | `speculative duration` | `CC 18011:2018 §7.10` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `DateTimeFormulaCombinesTemporalValueWithDuration` | `evaluation of date and time with duration` | `CC 18011:2018 §8` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `DateTimeFormulaUsesCarryOverSemantics` | `carry-over of overflow` | `CC 18011:2018 §8.1` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `DateTimeFormulaTruncatesAtComponentBoundaries` | `truncation at time scale component boundaries` | `CC 18011:2018 §8.2` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `DateTimeFormulaEvaluationModeDeclared` | `simple, composite, and precedence duration evaluation` | `CC 18011:2018 §8.3-§8.5` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `SelectionExpressionUsesSelectionDelimiters` | `selection expression delimiters` | `CC 18012:2018 §5.1` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |
| `SelectionExpressionUsesRecognizedSelectionRuleVocabulary` | `selection rules vocabulary` | `CC 18012:2018 §5.2` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |
| `SelectionRuleMonthUsesMonthExpression` | `month selection rule family` | `CC 18012:2018 §5.2` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |
| `SelectionRuleWeekUsesWeekExpression` | `week selection rule family` | `CC 18012:2018 §5.2` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |
| `SelectionRuleDayOfMonthUsesDayExpression` | `day-of-month selection rule family` | `CC 18012:2018 §5.2` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |
| `SelectionRuleWeekDayUsesDayOfWeekExpression` | `weekday selection rule family` | `CC 18012:2018 §5.2` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |
| `SelectionRuleOrdinalDayOfYearUsesOrdinalDayExpression` | `ordinal-day-of-year selection rule family` | `CC 18012:2018 §5.2` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |
| `SelectionRuleHourUsesHourExpression` | `hour selection rule family` | `CC 18012:2018 §5.2` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |
| `SelectionRuleMinuteUsesMinuteExpression` | `minute selection rule family` | `CC 18012:2018 §5.2` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |
| `SelectionRuleSecondUsesSecondExpression` | `second selection rule family` | `CC 18012:2018 §5.2` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |
| `SelectionRulesApplyWithinSelectedResults` | `selection rules application scope` | `CC 18012:2018 §5.3` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |
| `SelectionExpressionMaySelectSingleInstance` | `selection expression single-instance semantics` | `CC 18012:2018 §5.1` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |
| `SelectionRulePositionUsesInstanceDesignatorSuffix` | `position rule syntax` | `CC 18012:2018 §5.2` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |
| `SelectionRulePositionAppliesLast` | `selection rule position ordering` | `CC 18012:2018 §5.2` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |
| `SelectionWithDurationUsesDurationSuffix` | `selection with duration syntax` | `CC 18012:2018 §5.2` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |
| `RepeatRuleUsesFrequencyDesignator` | `repeat rule frequency designator` | `CC 18012:2018 §6.3.1` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |
| `RepeatRuleDeclaresEligibleTimeIntervals` | `eligible time intervals` | `CC 18012:2018 §6.3.2` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |
| `RepeatRuleSelectionAppliesWithinEligibleIntervals` | `selection part and selection rules` | `CC 18012:2018 §6.3.3` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |
| `RepeatRuleEvaluationInheritsInitialStartComponentInformation` | `evaluation of repeat rule` | `CC 18012:2018 §6.5` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |
| `RecurringIntervalWithRepeatRuleUsesCompleteRepresentation` | `complete representation for recurring interval with repeat rule` | `CC 18012:2018 §6.4` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |

## Notes

- This file already carries direct public section citations in code comments.
- Exhaustive CalConnect clause coverage is complete for the current public XML
  source set staged in this workspace.
- The `Source artifact` column points at local public XML corpora already staged
  in `standards/public`.
- `calconnect.rs` remains the right home for primitive CalConnect
  propositions. Proof-graph bundling belongs in
  `contracts_proof_composition.rs.md`, and consumer-facing method concordance
  belongs in the trait worksheets, so no proposition migration is currently
  warranted.

## CalConnect Clause Coverage Audit

| Source clause family | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `CC 18011:2018 §4.3 explicit forms` | `covered` | `ExplicitTemporalFormUsesDesignatorSymbols`, explicit time-of-day, time-shift, date-with-shift, date-time-with-shift, omission, precision, and reduced-time contracts | Negative-value, decade, and century explicit-form details remain intentionally shared with the broader ISO and extended surfaces. |
| `CC 18011:2018 §5 grouped time scale units` | `covered` | `GroupedTimeScaleUnitUsesGroupingDesignators`, `GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits`, `GroupedTimeScaleUnitDefinitionIsContinuous`, `GroupedTimeScaleUnitValueCarriesExplicitCoefficient`, `GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds`, `GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift`, `GroupedTimeScaleUnitConvertsToTimeInterval`, and `GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder` | Grouped-unit representation-with-shift semantics now have a first-class token. |
| `CC 18011:2018 §6 explicit time interval` | `covered` | `ExplicitTimeIntervalUsesDateTimeEndpointFamily`, `ExplicitTimeIntervalDurationSubstitutionInfersMissingBoundary`, `ExplicitTimeIntervalTrailingEndMayInheritHigherOrderComponents`, `ExplicitTimeIntervalLeadingShiftAppliesToTrailingComponentUnlessOverridden` | Subclause-level numbering still depends on a stronger public section breakdown than the XML headings expose directly. |
| `CC 18011:2018 §7 explicit duration` | `covered` | `ExplicitDurationUsesDurationalUnitDesignators`, representation-kind, composite, precedence, negative, fractional, exact, context-dependent, and speculative duration contracts | No further clause-level proposition is currently required. |
| `CC 18011:2018 §8 evaluation of date and time with duration` | `covered` | `DateTimeFormulaCombinesTemporalValueWithDuration`, `DateTimeFormulaUsesCarryOverSemantics`, `DateTimeFormulaTruncatesAtComponentBoundaries`, `DateTimeFormulaEvaluationModeDeclared` | Higher-detail evaluation proofs may later belong in proof-composition worksheets. |
| `CC 18012:2018 §5 selection rules` | `covered` | `SelectionExpressionUsesSelectionDelimiters`, `SelectionExpressionUsesRecognizedSelectionRuleVocabulary`, `SelectionRuleMonthUsesMonthExpression`, `SelectionRuleWeekUsesWeekExpression`, `SelectionRuleDayOfMonthUsesDayExpression`, `SelectionRuleWeekDayUsesDayOfWeekExpression`, `SelectionRuleOrdinalDayOfYearUsesOrdinalDayExpression`, `SelectionRuleHourUsesHourExpression`, `SelectionRuleMinuteUsesMinuteExpression`, `SelectionRuleSecondUsesSecondExpression`, `SelectionRulesApplyWithinSelectedResults`, `SelectionExpressionMaySelectSingleInstance`, `SelectionRulePositionUsesInstanceDesignatorSuffix`, `SelectionRulePositionAppliesLast`, `SelectionWithDurationUsesDurationSuffix` | The per-rule grammar vocabulary is now represented explicitly; RFC-compatibility notes beyond the surface grammar remain intentionally unstaged. |
| `CC 18012:2018 §6 recurring time intervals with repeat rules` | `covered` | `RepeatRuleUsesFrequencyDesignator`, `RepeatRuleDeclaresEligibleTimeIntervals`, `RepeatRuleSelectionAppliesWithinEligibleIntervals`, `RecurringIntervalWithRepeatRuleUsesCompleteRepresentation`, `RepeatRuleEvaluationInheritsInitialStartComponentInformation` | The public seam now preserves whether the embedded interval uses the ISO complete or CalConnect explicit family; repeat-rule compatibility commentary beyond the core representation remains intentionally unstaged. |

- [x] Walk CalConnect CC 18011 clause-by-clause and mark which substantive clauses are intentionally represented elsewhere in `elicit_temporal`.
- [x] Walk CalConnect CC 18012 clause-by-clause and do the same for recurrence and selection semantics.
- [x] Decide whether any CalConnect propositions currently embedded in `calconnect.rs` should move into separate proof-composition or trait worksheets for better concordance.
