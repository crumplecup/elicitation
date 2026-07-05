# Citation Worksheet: `src/contracts/interval.rs`

This worksheet tracks exact licensed-text citations for the ISO 8601 interval
and duration contracts in `src/contracts/interval.rs`.

## Source set

- `../iso-8601-1-2019.*`
- `../../public/iso-wd-8601-1-2016.txt`

## Contract map

| Contract symbol | Current topic label | Exact clause | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `DurationUsesPeriodDesignator` | `duration representation` | `4.4.2 b)` | `../../public/iso-wd-8601-1-2016.txt` lines 1483-1484 | `yes` |
| `DurationTimeComponentsFollowTimeDesignator` | `duration date/time component separation` | `4.4.3.2` | `../../public/iso-wd-8601-1-2016.txt` lines 1515-1524, 1544 | `yes` |
| `DurationWeekFormUsesSingleWeekUnit` | `duration week representation` | `4.4.3.2` | `../../public/iso-wd-8601-1-2016.txt` lines 1516-1523 | `yes` |
| `DurationWeekFormNotMixedWithCalendarOrClockUnits` | `duration form exclusivity` | `4.4.3.2` | `../../public/iso-wd-8601-1-2016.txt` lines 1516-1523 | `yes` |
| `DurationAlternativeFormRequiresPartnerAgreement` | `alternative duration interchange precondition` | `4.4.3.3` | `../../public/iso-wd-8601-1-2016.txt` lines 1584-1588 | `yes` |
| `DurationAlternativeFormUsesDateAndTimeComponentSlots` | `alternative duration component layout` | `4.4.3.3` | `../../public/iso-wd-8601-1-2016.txt` lines 1586-1591 | `yes` |
| `IntervalEndOmittedHigherOrderComponentsInheritFromStart` | `inherited higher-order end components after the solidus` | `4.4.5` | `../../public/iso-wd-8601-1-2016.txt` lines 1689-1693 | `yes` |
| `IntervalTrailingComponentInheritsZoneOrUtcFromLeadingComponentWhenOmitted` | `inherited trailing zone or UTC semantics after the solidus` | `4.4.5` | `../../public/iso-wd-8601-1-2016.txt` lines 1695-1697 | `yes` |
| `DurationAlternativeFormCarriesCompleteCalendarAndClockComponents` | `alternative complete duration representation` | `4.4.3.3` | `../../public/iso-wd-8601-1-2016.txt` lines 1586-1593 | `yes` |
| `TimeIntervalUsesSolidusSeparator` | `time interval representation` | `4.4.2 a)` | `../../public/iso-wd-8601-1-2016.txt` lines 1479-1482 | `yes` |
| `TimeIntervalHasTwoComponents` | `interval component structure` | `4.4.1; 4.4.2 a)` | `../../public/iso-wd-8601-1-2016.txt` lines 1462-1470, 1479-1482 | `yes` |
| `TimeIntervalBoundaryOrDurationFormDeclared` | `interval forms` | `4.4.1` | `../../public/iso-wd-8601-1-2016.txt` lines 1460-1470 | `yes` |
| `IntervalStartPrecedesEnd` | `interval endpoint ordering` | `3.1.1.6; 3.1.1.8` | ISO 8601-1:2019 sample text lines 458-484 | `yes` |
| `IntervalDurationIsNonNegative` | `duration semantics` | `3.1.1.8` | ISO 8601-1:2019 sample text lines 481-484 | `yes` |
| `CompleteIntervalCalendarDateMayBeSubstitutedByOrdinalDate` | `complete interval ordinal-date substitution` | `4.4.4.5` | `../../public/iso-wd-8601-1-2016.txt` lines 1655-1657 | `yes` |
| `CompleteIntervalCalendarDateMayBeSubstitutedByWeekDate` | `complete interval week-date substitution` | `4.4.4.5` | `../../public/iso-wd-8601-1-2016.txt` lines 1655-1657 | `yes` |
| `CompleteIntervalLocalTimeMayBeSubstitutedByUtcOfDay` | `complete interval UTC-of-day substitution` | `4.4.4.5` | `../../public/iso-wd-8601-1-2016.txt` lines 1659-1661 | `yes` |
| `CompleteIntervalLocalTimeMayBeSubstitutedByLocalTimeAndUtcDifference` | `complete interval local-time-plus-offset substitution` | `4.4.4.5` | `../../public/iso-wd-8601-1-2016.txt` lines 1659-1661 | `yes` |
| `CompleteIntervalDurationMaySubstituteWeekForm` | `complete interval week-duration substitution` | `4.4.4.5` | `../../public/iso-wd-8601-1-2016.txt` lines 1663-1664 | `yes` |
| `RecurringIntervalUsesRepeatDesignator` | `recurring time interval representation` | `4.5.2` | `../../public/iso-wd-8601-1-2016.txt` lines 1727-1734 | `yes` |
| `RecurringIntervalCountIsNonNegativeWhenBounded` | `recurring interval recurrence count` | `4.5.1` | `../../public/iso-wd-8601-1-2016.txt` lines 1704-1725 | `yes` |
| `RecurringIntervalOmittedCountDenotesUnboundedOccurrences` | `recurring interval omitted recurrence count semantics` | `4.5.1` | `../../public/iso-wd-8601-1-2016.txt` lines 1708-1725 | `yes` |
| `RecurringIntervalCarriesIntervalComponent` | `recurring interval component structure` | `4.5.1; 4.5.2` | `../../public/iso-wd-8601-1-2016.txt` lines 1704-1734 | `yes` |
| `RecurringIntervalUsesCompleteTimeIntervalRepresentation` | `complete recurring interval representation family` | `4.5.3` | `../../public/iso-wd-8601-1-2016.txt` lines 1736-1765 | `yes` |
| `RecurringIntervalUsesOtherThanCompleteTimeIntervalRepresentation` | `other-than-complete recurring interval representation family` | `4.5.4` | `../../public/iso-wd-8601-1-2016.txt` lines 1767-1770 | `yes` |

## ISO WD 8601-1 Clause Coverage Audit

| Clause concept | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `4.4.1 Means of specifying time intervals` | `covered` | `TimeIntervalHasTwoComponents`, `TimeIntervalBoundaryOrDurationFormDeclared` | The four interval families are staged as first-class interval-form vocabulary. |
| `4.4.2 Separators and designators` | `covered` | `TimeIntervalUsesSolidusSeparator`, `DurationUsesPeriodDesignator` | The solidus and duration-designator laws are explicit. |
| `4.4.3 Duration` | `covered` | `DurationUsesPeriodDesignator`, `DurationTimeComponentsFollowTimeDesignator`, `DurationWeekFormUsesSingleWeekUnit`, `DurationWeekFormNotMixedWithCalendarOrClockUnits`, `DurationAlternativeFormRequiresPartnerAgreement`, `DurationAlternativeFormUsesDateAndTimeComponentSlots`, `DurationAlternativeFormCarriesCompleteCalendarAndClockComponents`, `IntervalDurationIsNonNegative` | The designator and alternative duration families are now split into first-class contracts. |
| `4.4.4 Complete representations` | `covered` | `TimeIntervalHasTwoComponents`, `TimeIntervalBoundaryOrDurationFormDeclared`, `IntervalStartPrecedesEnd`, `DurationAlternativeFormRequiresPartnerAgreement`, `DurationAlternativeFormUsesDateAndTimeComponentSlots`, `DurationAlternativeFormCarriesCompleteCalendarAndClockComponents`, `CompleteIntervalCalendarDateMayBeSubstitutedByOrdinalDate`, `CompleteIntervalCalendarDateMayBeSubstitutedByWeekDate`, `CompleteIntervalLocalTimeMayBeSubstitutedByUtcOfDay`, `CompleteIntervalLocalTimeMayBeSubstitutedByLocalTimeAndUtcDifference`, `CompleteIntervalDurationMaySubstituteWeekForm` | The complete interval families now include explicit `4.4.4.5` substitution contracts for date, time, and duration representation branches. |
| `4.4.5 Representations other than complete` | `covered` | `TimeIntervalHasTwoComponents`, `TimeIntervalBoundaryOrDurationFormDeclared`, `IntervalEndOmittedHigherOrderComponentsInheritFromStart`, `IntervalTrailingComponentInheritsZoneOrUtcFromLeadingComponentWhenOmitted` | The inherited higher-order end-component and trailing zone-or-UTC rules are now explicit first-class propositions. |
| `4.5 Recurring time interval` | `covered` | `RecurringIntervalUsesRepeatDesignator`, `RecurringIntervalCountIsNonNegativeWhenBounded`, `RecurringIntervalOmittedCountDenotesUnboundedOccurrences`, `RecurringIntervalCarriesIntervalComponent`, `RecurringIntervalUsesCompleteTimeIntervalRepresentation`, `RecurringIntervalUsesOtherThanCompleteTimeIntervalRepresentation` | The recurrence prefix, count semantics, interval payload, and complete versus other-than-complete representation families are explicit first-class contracts. |

## Checklist

- [x] Add first-class coverage for the alternative duration representation used
  in complete interval forms.
- [x] Add explicit inherited-end-component and inherited-zone semantics for
  `4.4.5` interval representations.
- [x] Add first-class coverage for the `4.4.4.5` complete-interval
  substitution families.
- [x] Decide whether recurring complete and other-than-complete forms need
  separate propositions or proof sidecars.
