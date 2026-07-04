# Citation Worksheet: `src/contracts/interval.rs`

This worksheet tracks exact licensed-text citations for the ISO 8601 interval
and duration contracts in `src/contracts/interval.rs`.

## Source set

- `../iso-8601-1-2019.*`
- `../../public/iso-wd-8601-1-2016.txt`

## Contract map

| Contract symbol | Current topic label | Exact clause | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `DurationUsesPeriodDesignator` | `duration representation` | `4.4.2 b)` | `../../public/iso-wd-8601-1-2016.txt` lines 1704-1707 | `yes` |
| `DurationTimeComponentsFollowTimeDesignator` | `duration date/time component separation` | `4.4.3.2` | `../../public/iso-wd-8601-1-2016.txt` lines 1743-1748, 1768 | `yes` |
| `DurationWeekFormUsesSingleWeekUnit` | `duration week representation` | `4.4.3.2` | `../../public/iso-wd-8601-1-2016.txt` lines 1749-1750 | `yes` |
| `DurationWeekFormNotMixedWithCalendarOrClockUnits` | `duration form exclusivity` | `4.4.3.2` | `../../public/iso-wd-8601-1-2016.txt` lines 1749-1750 | `yes` |
| `DurationAlternativeFormRequiresPartnerAgreement` | `alternative duration interchange precondition` | `4.4.3.3` | `../../public/iso-wd-8601-1-2016.txt` lines 1799-1802 | `yes` |
| `DurationAlternativeFormUsesDateAndTimeComponentSlots` | `alternative duration component layout` | `4.4.3.3` | `../../public/iso-wd-8601-1-2016.txt` lines 1803-1806, 1817-1818, 1834-1835, 1867-1868 | `yes` |
| `DurationAlternativeFormCarriesCompleteCalendarAndClockComponents` | `alternative complete duration representation` | `4.4.3.3` | `../../public/iso-wd-8601-1-2016.txt` lines 1803-1806, 1817-1818, 1834-1835, 1867-1868 | `yes` |
| `TimeIntervalUsesSolidusSeparator` | `time interval representation` | `4.4.2 a)` | `../../public/iso-wd-8601-1-2016.txt` lines 1704-1705 | `yes` |
| `TimeIntervalHasTwoComponents` | `interval component structure` | `4.4.1; 4.4.2 a)` | `../../public/iso-wd-8601-1-2016.txt` lines 1688-1696, 1704-1705 | `yes` |
| `TimeIntervalBoundaryOrDurationFormDeclared` | `interval forms` | `4.4.1` | `../../public/iso-wd-8601-1-2016.txt` lines 1688-1696 | `yes` |
| `IntervalStartPrecedesEnd` | `interval endpoint ordering` | `3.1.1.6; 3.1.1.8` | ISO 8601-1:2019 sample text lines 458-484 | `yes` |
| `IntervalDurationIsNonNegative` | `duration semantics` | `3.1.1.8` | ISO 8601-1:2019 sample text lines 481-484 | `yes` |
| `RecurringIntervalUsesRepeatDesignator` | `recurring time interval representation` | `4.5.2` | `../../public/iso-wd-8601-1-2016.txt` lines 1969-1974 | `yes` |
| `RecurringIntervalCountIsNonNegativeWhenBounded` | `recurring interval recurrence count` | `4.5.1` | `../../public/iso-wd-8601-1-2016.txt` lines 1950-1968 | `yes` |
| `RecurringIntervalOmittedCountDenotesUnboundedOccurrences` | `recurring interval omitted recurrence count semantics` | `4.5.1` | `../../public/iso-wd-8601-1-2016.txt` lines 1951-1968 | `yes` |
| `RecurringIntervalCarriesIntervalComponent` | `recurring interval component structure` | `4.5.1; 4.5.2` | `../../public/iso-wd-8601-1-2016.txt` lines 1950-1974 | `yes` |

## ISO WD 8601-1 Clause Coverage Audit

| Clause concept | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `4.4.1 Means of specifying time intervals` | `covered` | `TimeIntervalHasTwoComponents`, `TimeIntervalBoundaryOrDurationFormDeclared` | The four interval families are staged as first-class interval-form vocabulary. |
| `4.4.2 Separators and designators` | `covered` | `TimeIntervalUsesSolidusSeparator`, `DurationUsesPeriodDesignator` | The solidus and duration-designator laws are explicit. |
| `4.4.3 Duration` | `covered` | `DurationUsesPeriodDesignator`, `DurationTimeComponentsFollowTimeDesignator`, `DurationWeekFormUsesSingleWeekUnit`, `DurationWeekFormNotMixedWithCalendarOrClockUnits`, `DurationAlternativeFormRequiresPartnerAgreement`, `DurationAlternativeFormUsesDateAndTimeComponentSlots`, `DurationAlternativeFormCarriesCompleteCalendarAndClockComponents`, `IntervalDurationIsNonNegative` | The designator and alternative duration families are now split into first-class contracts. |
| `4.4.4 Complete representations` | `partially covered` | `TimeIntervalHasTwoComponents`, `TimeIntervalBoundaryOrDurationFormDeclared`, `IntervalStartPrecedesEnd`, `DurationAlternativeFormRequiresPartnerAgreement`, `DurationAlternativeFormUsesDateAndTimeComponentSlots`, `DurationAlternativeFormCarriesCompleteCalendarAndClockComponents` | The start/end, start/duration, and duration/end families are recognized, and the alternative duration branch is explicit; the substitution families in `4.4.4.5` are still not represented separately. |
| `4.4.5 Representations other than complete` | `partially covered` | `TimeIntervalHasTwoComponents`, `TimeIntervalBoundaryOrDurationFormDeclared` | Inherited higher-order end components and inherited zone or UTC semantics after the solidus are not yet first-class propositions. |
| `4.5 Recurring time interval` | `partially covered` | `RecurringIntervalUsesRepeatDesignator`, `RecurringIntervalCountIsNonNegativeWhenBounded`, `RecurringIntervalOmittedCountDenotesUnboundedOccurrences`, `RecurringIntervalCarriesIntervalComponent` | Complete versus other-than-complete recurring families remain flattened into general recurrence laws. |

## Checklist

- [x] Add first-class coverage for the alternative duration representation used
  in complete interval forms.
- [ ] Add explicit inherited-end-component and inherited-zone semantics for
  `4.4.5` interval representations.
- [ ] Decide whether recurring complete and other-than-complete forms need
  separate propositions or proof sidecars.
