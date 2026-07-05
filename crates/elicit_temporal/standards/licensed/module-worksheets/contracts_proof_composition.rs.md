# Citation Worksheet: `src/contracts/proof_composition.rs`

This worksheet tracks exact standards citations for the aggregate evidence bundles in
`src/contracts/proof_composition.rs`.

## Source set

- `../iso-8601-1-2019.*`
- `../iso-8601-1-2019-amd1-2022.*`
- `../iso-8601-2-2019.*`
- `../../public/calconnect-cc-18011-2018.xml`
- `../../public/iso-wd-8601-1-2016.txt`
- `../../public/iso-wd-8601-2-2016.txt`

## Standards evidence map

| Evidence struct | Current topic label | Exact clause | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `CalendarDateEvidence` | `calendar date interchange core` | `ISO 8601-1:2019 5.2.2` | `iso-8601-1-2019.sample.txt lines 156-156` | `yes` |
| `DecadeEvidence` | `decade component ordinal range; decade representations` | `ISO 8601-1:2019/Amd 1:2022 4.3.11; ISO 8601-2:2019 4.3.5` | `iso-8601-1-2019-amd1-2022.sample.txt lines 184-191; iso-8601-2-2019.sample.txt lines 1026-1027` | `yes` |
| `CenturyEvidence` | `century component ordinal range; century representations` | `ISO 8601-1:2019/Amd 1:2022 4.3.12; ISO 8601-2:2019 4.3.6` | `iso-8601-1-2019-amd1-2022.sample.txt lines 193-200; iso-8601-2-2019.sample.txt lines 1029-1030` | `yes` |
| `OrdinalDateEvidence` | `ordinal date interchange core` | `ISO 8601-1:2019 5.2.3` | `iso-8601-1-2019.sample.txt lines 158-158` | `yes` |
| `WeekDateEvidence` | `week date interchange core` | `ISO 8601-1:2019 5.2.4` | `iso-8601-1-2019.sample.txt lines 160-160` | `yes` |
| `LocalTimeEvidence` | `time-of-day interchange core; end-of-day technical correction` | `ISO 8601-1:2019 5.3.1; ISO 8601-1:2019/Amd 1:2022 5.3.1.4; 5.3.2` | `iso-8601-1-2019.sample.txt lines 164-172; iso-8601-1-2019-amd1-2022.sample.txt lines 202-216` | `yes` |
| `UtcOffsetEvidence` | `UTC offset interchange core` | `ISO 8601-1:2019 5.3.4; open-text cross-check ISO/WD 8601-1:2016(E) 4.2.5.1` | `iso-8601-1-2019.sample.txt lines 170-170; ../../public/iso-wd-8601-1-2016.txt lines 1500-1546` | `yes` |
| `LocalDateTimeEvidence` | `combined date and time interchange core` | `ISO 8601-1:2019 5.4.2; 5.4.3` | `iso-8601-1-2019.sample.txt lines 191-193; ../../public/iso-wd-8601-1-2016.txt lines 1547-1646` | `yes` |
| `OffsetDateTimeEvidence` | `date-time with UTC relationship` | `ISO 8601-1:2019 5.3.4; 5.4.2; 5.4.3` | `iso-8601-1-2019.sample.txt lines 170-170, 191-193; ../../public/iso-wd-8601-1-2016.txt lines 1500-1646` | `yes` |
| `CompleteDateEvidence` | `explicit complete date families` | `CC 18011:2018 §4.3 - Date` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `DateWithShiftEvidence` | `date with shift` | `CC 18011:2018 §4.3 - Date with shift` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `TimeOfDayWithShiftEvidence` | `time of day with time shift` | `CC 18011:2018 §4.3 - Time of day with time shift` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `FixedInstantEvidence` | `date-time with UTC relationship` | `ISO 8601-1:2019 5.3.4; 5.4.2; 5.4.3; RFC 3339 §5.6` | `iso-8601-1-2019.sample.txt lines 170-170, 191-193; ../../public/iso-wd-8601-1-2016.txt lines 1500-1646; RFC 3339` | `yes` |
| `PrecisionPreservationEvidence` | `decimal fractions` | `ISO 8601-1:2019/Amd 1:2022 5.3.1.4; RFC 3339 §5.6` | `iso-8601-1-2019-amd1-2022.sample.txt lines 205-208; RFC 3339` | `yes` |
| `BackendConversionEvidence` | `representation changes across equivalent forms` | `ISO 8601-1:2019 3.1.3; 5.2; 5.3; 5.4; 5.5; 5.6; RFC 3339 §5.1` | `iso-8601-1-2019.sample.txt lines 75-97, 147-214; RFC 3339` | `yes` |
| `LossyConversionAuthorityEvidence` | `reduced-precision and rounding semantics` | `ISO 8601-2:2019 7.13; 14.2; 14.3; 14.4` | `iso-8601-2-2019.sample.txt lines 261, 437-441` | `yes` |
| `LosslessConversionEvidence` | `reduced-precision semantics` | `ISO 8601-2:2019 7.11; 7.12; 7.13; RFC 3339 §5.1; §5.6` | `iso-8601-2-2019.sample.txt lines 259-261; RFC 3339` | `yes` |
| `SubsecondTruncationEvidence` | `reduced-precision and rounding semantics` | `ISO 8601-2:2019 7.13; 14.2; 14.3; 14.4` | `iso-8601-2-2019.sample.txt lines 261, 437-441` | `yes` |
| `DurationDesignatorRepresentationEvidence` | `designator-based duration representation family` | `4.4.3.2` | `../../public/iso-wd-8601-1-2016.txt` lines 1515-1524 | `yes` |
| `DurationFormEvidence` | `duration representations` | `4.4.2 b); 4.4.3.2; 4.4.3.3` | `contracts_interval.rs.md` | `yes` |
| `TimeIntervalEvidence` | `time interval representations` | `4.4.1; 4.4.2 a)` | `contracts_interval.rs.md` | `yes` |
| `InheritedIntervalEndComponentsEvidence` | `interval end-component inheritance semantics` | `4.4.5` | `contracts_interval.rs.md` | `yes` |
| `InheritedIntervalZoneEvidence` | `interval trailing-zone inheritance semantics` | `4.4.5` | `contracts_interval.rs.md` | `yes` |
| `IntervalEndpointOrderingEvidence` | `interval ordering and duration semantics` | `3.1.1.6; 3.1.1.8` | `contracts_interval.rs.md` | `yes` |
| `RecurringIntervalEvidence` | `recurring interval representations` | `4.5.1; 4.5.2` | `contracts_interval.rs.md` | `yes` |
| `CompleteRecurringIntervalRepresentationEvidence` | `complete recurring interval representation family` | `4.5.3` | `contracts_interval.rs.md` | `yes` |
| `OtherThanCompleteRecurringIntervalRepresentationEvidence` | `other-than-complete recurring interval representation family` | `4.5.4` | `contracts_interval.rs.md` | `yes` |
| `CompleteTimePointDateRepresentationEvidence` | `complete interval date substitution families` | `4.4.4.5` | `../../public/iso-wd-8601-1-2016.txt` lines 1655-1657 | `yes` |
| `CompleteTimePointTimeRepresentationEvidence` | `complete interval time substitution families` | `4.4.4.5` | `../../public/iso-wd-8601-1-2016.txt` lines 1659-1661 | `yes` |
| `CompleteTimePointRepresentationEvidence` | `complete interval time-point substitution bundle` | `4.4.4.5` | `../../public/iso-wd-8601-1-2016.txt` lines 1655-1661 | `yes` |
| `CompleteIntervalDurationRepresentationEvidence` | `complete interval duration substitution families` | `4.4.4.5` | `../../public/iso-wd-8601-1-2016.txt` lines 1663-1664 | `yes` |
| `CompleteStartEndIntervalSubstitutionEvidence` | `complete start/end interval substitution semantics` | `4.4.4.1; 4.4.4.5` | `../../public/iso-wd-8601-1-2016.txt` lines 1548-1568, 1651-1664 | `yes` |
| `CompleteStartDurationIntervalSubstitutionEvidence` | `complete start/duration interval substitution semantics` | `4.4.4.3; 4.4.4.5` | `../../public/iso-wd-8601-1-2016.txt` lines 1595-1616, 1651-1664 | `yes` |
| `CompleteDurationEndIntervalSubstitutionEvidence` | `complete duration/end interval substitution semantics` | `4.4.4.4; 4.4.4.5` | `../../public/iso-wd-8601-1-2016.txt` lines 1626-1647, 1651-1664 | `yes` |
| `QualifiedTemporalExpressionEvidence` | `uncertain and approximate temporal expressions` | `ISO 8601-2:2019 8.2.1; 8.2.2; 8.2.3; 8.4.4; 8.4.5; 8.4.6; 8.5` | `iso-8601-2-2019.sample.txt lines 280-345` | `yes` |
| `ExtendedIntervalBoundaryEvidence` | `open and unknown interval boundaries` | `ISO 8601-2:2019 10.2` | `iso-8601-2-2019.sample.txt lines 360-360` | `yes` |
| `EnhancedIntervalLevelOneEvidence` | `enhanced interval Level 1 qualification semantics` | `ISO 8601-2:2019 4.5.1; open-text cross-check ISO/WD 8601-2:2016(E) 4.5.1` | `../../public/iso-wd-8601-2-2016.txt lines 446-447` | `yes` |
| `EnhancedIntervalLevelTwoEvidence` | `enhanced interval Level 2 boundary qualification semantics` | `ISO 8601-2:2019 4.5.2; open-text cross-check ISO/WD 8601-2:2016(E) 4.5.2` | `../../public/iso-wd-8601-2-2016.txt lines 474-476` | `yes` |
| `SeasonalTemporalExpressionEvidence` | `seasons and named seasonal temporal expressions` | `ISO 8601-2:2019 4.8.1; 4.8.2; 4.8.3` | `iso-8601-2-2019.sample.txt lines 154-158, 873-875` | `yes` |
| `UnspecifiedComponentExpressionEvidence` | `unspecified digits and unspecified components` | `ISO 8601-2:2019 9.2.1; 9.2.2; 9.3` | `iso-8601-2-2019.sample.txt lines 350-356` | `yes` |
| `TemporalSetExpressionEvidence` | `temporal set expressions` | `ISO 8601-2:2019 6.1; 6.2; 6.3; 6.4` | `iso-8601-2-2019.sample.txt lines 201-209` | `yes` |
| `TemporalSetRangeSemanticsEvidence` | `set representation` | `ISO 8601-2:2019 6.3; 6.4` | `iso-8601-2-2019.sample.txt lines 206-209` | `yes` |

## Notes

- RFC-governed evidence bundles in this file are already section-cited and are
  intentionally excluded from this worksheet.
- `LocalTimeEvidence` must keep amendment-backed end-of-day references tied
  explicitly to `iso-8601-1-2019-amd1-2022.*`.
- The lossy-conversion bundles are accord-level sidecars rather than
  standard-named concepts, so the worksheet records the clauses they enforce.

## Proof Composition Coverage Audit

| Evidence family | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `core ISO 8601-1 bundles` | `covered` | `CalendarDateEvidence`, `OrdinalDateEvidence`, `WeekDateEvidence`, `UtcOffsetEvidence`, `LocalDateTimeEvidence`, `OffsetDateTimeEvidence` | Repo-local WD text now anchors the open-text cross-checks. |
| `interval and duration bundles` | `covered` | `DurationDesignatorRepresentationEvidence`, `DurationFormEvidence`, `DurationRepresentationEvidence`, `DurationAlternativeFormEvidence`, `DurationWeekFormEvidence`, `TimeIntervalEvidence`, `InheritedIntervalEndComponentsEvidence`, `InheritedIntervalZoneEvidence`, `CompleteTimePointDateRepresentationEvidence`, `CompleteTimePointTimeRepresentationEvidence`, `CompleteTimePointRepresentationEvidence`, `CompleteIntervalDurationRepresentationEvidence`, `CompleteStartEndIntervalSubstitutionEvidence`, `CompleteStartDurationIntervalSubstitutionEvidence`, `CompleteDurationEndIntervalSubstitutionEvidence`, `RecurringIntervalEvidence`, `CompleteRecurringIntervalRepresentationEvidence`, `OtherThanCompleteRecurringIntervalRepresentationEvidence`, `ExtendedIntervalBoundaryEvidence` | Designator and alternative duration families, `4.4.5` inheritance, `4.4.4.5` substitution branches, and recurring-form families are all explicit in the proof graph. |
| `extended temporal bundles` | `covered` | `QualifiedTemporalExpressionEvidence`, `EnhancedIntervalLevelOneEvidence`, `EnhancedIntervalLevelTwoEvidence`, `SeasonalTemporalExpressionEvidence`, `UnspecifiedComponentExpressionEvidence`, `TemporalSetExpressionEvidence` | Extended expression families, including enhanced-interval Level 1/Level 2 semantics, remain explicitly bundled. |
| `precision and conversion bundles` | `covered` | `PrecisionPreservationEvidence`, `LossyConversionAuthorityEvidence`, `LosslessConversionEvidence`, `SubsecondTruncationEvidence` | Accord-level sidecars remain explicit and clause-backed. |

## Checklist

- [x] Revisit recurring-interval proof bundles once the interval worksheet
  decides whether complete and other-than-complete recurring forms need
  distinct sidecars.
- [x] Add explicit proof bundles for the `4.4.4.5` complete-interval
  substitution families.
