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
| `ReducedCalendarDatePrecisionEvidence` | `reduced-precision calendar date branch` | `ISO 8601-1:2019 5.2.2; open-text cross-check ISO/WD 8601-1:2016(E) 4.1.2.3` | `iso-8601-1-2019.sample.txt lines 156-157; ../../public/iso-wd-8601-1-2016.txt lines 1039-1055` | `yes` |
| `ReducedCalendarDateEvidence` | `reduced-precision calendar date bundle` | `ISO 8601-1:2019 5.2.2; open-text cross-check ISO/WD 8601-1:2016(E) 4.1.2.3` | `iso-8601-1-2019.sample.txt lines 156-157; ../../public/iso-wd-8601-1-2016.txt lines 1039-1055` | `yes` |
| `DecadeEvidence` | `decade component ordinal range; decade representations` | `ISO 8601-1:2019/Amd 1:2022 4.3.11; ISO 8601-2:2019 4.3.5` | `iso-8601-1-2019-amd1-2022.sample.txt lines 184-191; iso-8601-2-2019.sample.txt lines 1026-1027` | `yes` |
| `CenturyEvidence` | `century component ordinal range; century representations` | `ISO 8601-1:2019/Amd 1:2022 4.3.12; ISO 8601-2:2019 4.3.6` | `iso-8601-1-2019-amd1-2022.sample.txt lines 193-200; iso-8601-2-2019.sample.txt lines 1029-1030` | `yes` |
| `ExtendedYearSignificantDigitsEvidence` | `expanded calendar year significant-digits suffix` | `ISO 8601-2:2019 4.4.3; 4.7.4; open-text cross-check ISO/WD 8601-2:2016(E) 4.7; 4.7.2` | `iso-8601-2-2019.sample.txt lines 124-151; ../../public/iso-wd-8601-2-2016.txt lines 524-546` | `yes` |
| `ExtendedYearBaseEvidence` | `negative, letter-prefixed, and exponential year forms` | `ISO 8601-2:2019 4.4.1; 4.4.2; 4.7.2; 4.7.3; open-text cross-check ISO/WD 8601-2:2016(E) 4.6.1; 4.6.2` | `iso-8601-2-2019.sample.txt lines 124-149; ../../public/iso-wd-8601-2-2016.txt lines 489-514` | `yes` |
| `ExtendedYearEvidence` | `expanded calendar year bundle with significant digits` | `ISO 8601-2:2019 4.4.1; 4.4.2; 4.4.3; 4.7.2; 4.7.3; 4.7.4; open-text cross-check ISO/WD 8601-2:2016(E) 4.6.1; 4.6.2; 4.7` | `iso-8601-2-2019.sample.txt lines 124-151; ../../public/iso-wd-8601-2-2016.txt lines 489-546` | `yes` |
| `OrdinalDateEvidence` | `ordinal date interchange core` | `ISO 8601-1:2019 5.2.3` | `iso-8601-1-2019.sample.txt lines 158-158` | `yes` |
| `WeekDateEvidence` | `week date interchange core` | `ISO 8601-1:2019 5.2.4` | `iso-8601-1-2019.sample.txt lines 160-160` | `yes` |
| `DateEvidence` | `date representation families` | `ISO 8601-1:2019 5.2.1; 5.2.2; 5.2.3; 5.2.4` | `iso-8601-1-2019.sample.txt lines 154-160` | `yes` |
| `LocalTimeEvidence` | `time-of-day interchange core; end-of-day technical correction` | `ISO 8601-1:2019 5.3.1; ISO 8601-1:2019/Amd 1:2022 5.3.1.4; 5.3.2` | `iso-8601-1-2019.sample.txt lines 164-172; iso-8601-1-2019-amd1-2022.sample.txt lines 202-216` | `yes` |
| `ReducedLocalTimePrecisionEvidence` | `reduced-accuracy local time branch` | `ISO 8601-1:2019 5.3.1; open-text cross-check ISO/WD 8601-1:2016(E) 4.2.2.3` | `iso-8601-1-2019.sample.txt lines 164-165; ../../public/iso-wd-8601-1-2016.txt lines 1217-1228` | `yes` |
| `ReducedLocalTimeEvidence` | `reduced-accuracy and decimal-fraction local time bundle` | `ISO 8601-1:2019 5.3.1; 5.3.2; ISO 8601-1:2019/Amd 1:2022 5.3.1.4; open-text cross-check ISO/WD 8601-1:2016(E) 4.2.2.3; 4.2.2.4` | `iso-8601-1-2019.sample.txt lines 164-167; iso-8601-1-2019-amd1-2022.sample.txt lines 202-216; ../../public/iso-wd-8601-1-2016.txt lines 1217-1257` | `yes` |
| `UtcOffsetEvidence` | `UTC offset interchange core` | `ISO 8601-1:2019 5.3.4; open-text cross-check ISO/WD 8601-1:2016(E) 4.2.5.1` | `iso-8601-1-2019.sample.txt lines 170-170; ../../public/iso-wd-8601-1-2016.txt lines 1500-1546` | `yes` |
| `UtcTimeScaleEvidence` | `UTC reference time scale` | `ISO 8601-1:2019 3.1.1.12` | `iso_8601.rs.md` | `yes` |
| `UtcOfDayEvidence` | `UTC-of-day bundle` | `ISO 8601-1:2019 3.1.1.13; open-text cross-check ISO/WD 8601-1:2016(E) 4.2.4` | `iso_8601.rs.md` | `yes` |
| `StandardTimeEvidence` | `standard time bundle` | `ISO 8601-1:2019 3.1.1.14; 3.1.1.25` | `iso_8601.rs.md` | `yes` |
| `StandardTimeOfDayEvidence` | `standard-time-of-day bundle` | `ISO/WD 8601-1:2016(E) 2.1.15` | `iso_8601.rs.md` | `yes` |
| `LocalTimeScaleEvidence` | `local time-scale bundle` | `ISO 8601-1:2019 3.1.1.15` | `iso_8601.rs.md` | `yes` |
| `LocalTimeSemanticsEvidence` | `local time semantics bundle` | `ISO 8601-1:2019 3.1.1.15; 3.1.1.17` | `iso_8601.rs.md` | `yes` |
| `TimeEvidence` | `time representation families` | `ISO 8601-1:2019 3.1.1.2; 3.1.1.13; 3.1.1.16; 5.3.1; 5.3.3` | `iso_8601.rs.md` | `yes` |
| `LocalDateTimeEvidence` | `combined date and time interchange core` | `ISO 8601-1:2019 5.4.2; 5.4.3` | `iso-8601-1-2019.sample.txt lines 191-193; ../../public/iso-wd-8601-1-2016.txt lines 1547-1646` | `yes` |
| `OffsetDateTimeEvidence` | `date-time with UTC relationship` | `ISO 8601-1:2019 5.3.4; 5.4.2; 5.4.3` | `iso-8601-1-2019.sample.txt lines 170-170, 191-193; ../../public/iso-wd-8601-1-2016.txt lines 1500-1646` | `yes` |
| `CombinedDateTimeDateEvidence` | `combined date-time date branch families` | `ISO 8601-1:2019 5.4.2; open-text cross-check ISO/WD 8601-1:2016(E) 4.3.2 a); 4.3.2 b); 4.3.2 c)` | `iso_8601.rs.md` | `yes` |
| `CompleteDateEvidence` | `explicit complete date families` | `CC 18011:2018 §4.3 - Date` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitTimeOfDayEvidence` | `explicit local time of day` | `CC 18011:2018 §4.3.3` | `contracts_calconnect.rs.md` | `yes` |
| `ExplicitTimeShiftEvidence` | `explicit time shift` | `CC 18011:2018 §4.3.4` | `contracts_calconnect.rs.md` | `yes` |
| `ExplicitDateTimeEvidence` | `explicit date and time of day` | `CC 18011:2018 §4.3.2; §4.3.3` | `contracts_calconnect.rs.md` | `yes` |
| `DateWithShiftEvidence` | `date with shift` | `CC 18011:2018 §4.3 - Date with shift` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `TimeOfDayWithShiftEvidence` | `time of day with time shift` | `CC 18011:2018 §4.3 - Time of day with time shift` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitDateTimeWithShiftEvidence` | `explicit date and time with shift` | `CC 18011:2018 §4.3.4.3` | `contracts_calconnect.rs.md` | `yes` |
| `QualifiedTemporalValueEvidence` | `qualification sidecar exchange` | `ISO 8601-2:2019 8.2.1; 8.2.2; 8.2.3; 8.4.4; 8.4.5; 8.4.6; 8.5` | `iso-8601-2-2019.sample.txt lines 280-345` | `yes` |
| `MutualAgreementAuthorityScopeEvidence` | `agreement-governed authority scope` | `ISO/WD 8601-1:2016(E) 3.2.1; 4.1.2.1; 4.1.2.4` | `iso_8601.rs.md` | `yes` |
| `MutualAgreementAuthorityEvidence` | `explicit mutual-agreement authority` | `ISO/WD 8601-1:2016(E) 3.2.1; 4.1.2.1; 4.1.2.4` | `iso_8601.rs.md` | `yes` |
| `QualificationPlacementEvidence` | `group versus individual qualification placement` | `ISO 8601-2:2019 8.4.4; 8.4.5` | `contracts_extended.rs.md` | `yes` |
| `ExplicitTemporalFormEvidence` | `explicit forms` | `CC 18011:2018 §4.3` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitDurationRepresentationEvidence` | `explicit duration representation branch` | `CC 18011:2018 §7.3-§7.5` | `contracts_calconnect.rs.md` | `yes` |
| `ExplicitDurationSemanticEvidence` | `exact versus nominal duration semantics` | `CC 18011:2018 §7.1-§7.5` | `contracts_calconnect.rs.md` | `yes` |
| `ExplicitDurationEvidence` | `explicit duration` | `CC 18011:2018 §7` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitTimeIntervalEvidence` | `explicit time interval` | `CC 18011:2018 §6 - Time interval, General` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitIntervalDurationSubstitutionEvidence` | `explicit-interval duration substitution semantics` | `CC 18011:2018 §6 - Time interval, Duration substitution` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitIntervalEndComponentInheritanceEvidence` | `explicit-interval trailing-end inheritance semantics` | `CC 18011:2018 §6 - Time interval, Time scale component order` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `ExplicitIntervalShiftPropagationEvidence` | `explicit-interval leading-shift propagation semantics` | `CC 18011:2018 §6 - Time interval, Time shift indication` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `GroupedTimeScaleUnitEvidence` | `grouped time scale units` | `CC 18011:2018 §5` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `DateTimeFormulaEvidence` | `evaluation of date and time with duration` | `CC 18011:2018 §8` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `DateTimeFormulaEvaluationSemanticsEvidence` | `simple, composite, and precedence duration evaluation families` | `CC 18011:2018 §8.3-§8.5` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `DateTimeFormulaEvaluationResultEvidence` | `date-time formula evaluation result provenance` | `CC 18011:2018 §8` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `FixedInstantEvidence` | `date-time with UTC relationship` | `ISO 8601-1:2019 5.3.4; 5.4.2; 5.4.3; RFC 3339 §5.6` | `iso-8601-1-2019.sample.txt lines 170-170, 191-193; ../../public/iso-wd-8601-1-2016.txt lines 1500-1646; RFC 3339` | `yes` |
| `PrecisionPreservationEvidence` | `decimal fractions` | `ISO 8601-1:2019/Amd 1:2022 5.3.1.4; RFC 3339 §5.6` | `iso-8601-1-2019-amd1-2022.sample.txt lines 205-208; RFC 3339` | `yes` |
| `BackendConversionEvidence` | `representation changes across equivalent forms` | `ISO 8601-1:2019 3.1.3; 5.2; 5.3; 5.4; 5.5; 5.6; RFC 3339 §5.1` | `iso-8601-1-2019.sample.txt lines 75-97, 147-214; RFC 3339` | `yes` |
| `LossyConversionAuthorityEvidence` | `reduced-precision and rounding semantics` | `ISO 8601-2:2019 7.13; 14.2; 14.3; 14.4` | `iso-8601-2-2019.sample.txt lines 261, 437-441` | `yes` |
| `LosslessConversionEvidence` | `reduced-precision semantics` | `ISO 8601-2:2019 7.11; 7.12; 7.13; RFC 3339 §5.1; §5.6` | `iso-8601-2-2019.sample.txt lines 259-261; RFC 3339` | `yes` |
| `SubsecondTruncationEvidence` | `reduced-precision and rounding semantics` | `ISO 8601-2:2019 7.13; 14.2; 14.3; 14.4` | `iso-8601-2-2019.sample.txt lines 261, 437-441` | `yes` |
| `DurationDesignatorRepresentationEvidence` | `designator-based duration representation family` | `4.4.3.2` | `../../public/iso-wd-8601-1-2016.txt` lines 1515-1524 | `yes` |
| `DurationWeekFormEvidence` | `week-form duration semantics` | `ISO 8601-1:2019 4.4.2 b); 4.4.3.2; 4.4.3.3; informative cross-check ISO/WD 8601-1:2016(E) 4.4.4.2.2; 4.4.4.3; 4.4.4.4; 4.4.5` | `../../public/iso-wd-8601-1-2016.txt lines 1595-1647` | `yes` |
| `DurationAlternativeFormEvidence` | `alternative complete duration representation family` | `ISO 8601-1:2019 4.4.3.3; informative cross-check ISO/WD 8601-1:2016(E) 4.4.4.2.2; 4.4.4.3; 4.4.4.4; 4.4.5` | `../../public/iso-wd-8601-1-2016.txt lines 1570-1647` | `yes` |
| `DurationRepresentationEvidence` | `duration representation family branch` | `ISO 8601-1:2019 4.4.3.2; 4.4.3.3; informative cross-check ISO/WD 8601-1:2016(E) 4.4.4.2.2; 4.4.4.3; 4.4.4.4; 4.4.5` | `contracts_interval.rs.md` | `yes` |
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
| `SubYearGroupingKindEvidence` | `sub-year grouping kind branch` | `ISO 8601-2:2019 4.8.1; 4.8.2` | `contracts_extended.rs.md` | `yes` |
| `SubYearGroupingExpressionEvidence` | `sub-year grouping expression bundle` | `ISO 8601-2:2019 4.8.1; 4.8.2; 4.8.3` | `contracts_extended.rs.md` | `yes` |
| `UnspecifiedComponentExpressionEvidence` | `unspecified digits and unspecified components` | `ISO 8601-2:2019 9.2.1; 9.2.2; 9.3` | `iso-8601-2-2019.sample.txt lines 350-356` | `yes` |
| `TemporalSetExpressionEvidence` | `temporal set expressions` | `ISO 8601-2:2019 6.1; 6.2; 6.3; 6.4` | `iso-8601-2-2019.sample.txt lines 201-209` | `yes` |
| `TemporalSetRangeSemanticsEvidence` | `set representation` | `ISO 8601-2:2019 6.3; 6.4` | `iso-8601-2-2019.sample.txt lines 206-209` | `yes` |
| `SelectionExpressionEvidence` | `selection of date and time` | `CC 18012:2018 §5` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |
| `RepeatRuleEvidence` | `repeat rule` | `CC 18012:2018 §6.3` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |
| `RecurringIntervalWithRepeatRuleEvidence` | `complete recurring interval with repeat-rule refinement` | `CC 18012:2018 §6.4` | `../../public/calconnect-cc-18012-2018.xml` | `yes` |

## Intentionally Externalized Proof Carriers

These proof carriers are part of `src/contracts/proof_composition.rs`, but
their authoritative clause-by-clause treatment lives in the dedicated RFC or
zone worksheets named in the final column rather than being duplicated here.

| Evidence struct | Home standards surface | Governing source family | Home worksheet |
| --- | --- | --- | --- |
| `KnownUtcOffsetEvidence` | `known UTC-offset semantics` | `RFC 3339 / RFC 9557` | `contracts_rfc3339.rs.md` |
| `LocalTimestampSemanticsEvidence` | `local timestamp semantics` | `RFC 9557 / zone semantics` | `contracts_zone.rs.md` |
| `ZoneTransitionResolutionAuthorityEvidence` | `zone-transition resolution authority` | `RFC 9557 / zone semantics` | `contracts_zone.rs.md` |
| `NamedTimeZoneIdentityEvidence` | `generic named-zone identity semantics` | `RFC 9557 / zone semantics` | `contracts_zone.rs.md` |
| `ZoneTransitionAmbiguityEvidence` | `zone-transition ambiguity semantics` | `RFC 9557 / zone semantics` | `contracts_zone.rs.md` |
| `NamedZoneAttachmentEvidence` | `generic fixed-instant named-zone attachment` | `RFC 9557 / zone semantics` | `contracts_zone.rs.md` |
| `ZoneTransitionGapEvidence` | `zone-transition gap semantics` | `RFC 9557 / zone semantics` | `contracts_zone.rs.md` |
| `Rfc3339TimestampEvidence` | `RFC 3339 timestamp profile` | `RFC 3339 as updated by RFC 9557` | `contracts_rfc3339.rs.md` |
| `Rfc3339LexicalOrderingEvidence` | `RFC 3339 lexical-ordering semantics` | `RFC 3339` | `contracts_rfc3339.rs.md` |
| `Rfc3339GenerationGuidanceEvidence` | `RFC 3339 generation guidance` | `RFC 3339` | `contracts_rfc3339.rs.md` |
| `Rfc3339DisplayGuidanceEvidence` | `RFC 3339 display-localization guidance` | `RFC 3339` | `contracts_rfc3339.rs.md` |
| `IxdtfTimestampEvidence` | `IXDTF timestamp syntax` | `RFC 9557` | `contracts_rfc9557.rs.md` |
| `IxdtfSuffixKeyRegistryEntryEvidence` | `IXDTF suffix-key registry entry fields` | `RFC 9557` | `contracts_rfc9557.rs.md` |
| `IxdtfPermanentSuffixKeyRegistrationEvidence` | `permanent IXDTF suffix-key registration semantics` | `RFC 9557` | `contracts_rfc9557.rs.md` |
| `IxdtfProvisionalSuffixKeyRegistrationEvidence` | `provisional IXDTF suffix-key registration semantics` | `RFC 9557` | `contracts_rfc9557.rs.md` |
| `IxdtfSuffixKeyRegistryPolicyEvidence` | `IXDTF suffix-key registry policy` | `RFC 9557` | `contracts_rfc9557.rs.md` |
| `IxdtfCalendarAwareTimestampEvidence` | `IXDTF calendar-awareness semantics` | `RFC 9557` | `contracts_rfc9557.rs.md` |
| `IxdtfCalendarKeyRegistryEvidence` | `IXDTF calendar-key registry entry` | `RFC 9557` | `contracts_rfc9557.rs.md` |
| `IxdtfAdditionalInformationEvidence` | `IXDTF additional-information semantics` | `RFC 9557` | `contracts_rfc9557.rs.md` |
| `ZonedTimestampEvidence` | `named-zone timestamp semantics` | `RFC 9557 / TZDB operational cross-checks` | `contracts_zone.rs.md` |
| `CriticalTimeZoneInconsistencyEvidence` | `critical time-zone inconsistency handling` | `RFC 9557 / zone semantics` | `contracts_zone.rs.md` |
| `ElectiveTimeZoneInconsistencyEvidence` | `elective time-zone inconsistency handling` | `RFC 9557 / zone semantics` | `contracts_zone.rs.md` |
| `OffsetTimeZoneAnnotationEvidence` | `offset time-zone annotation semantics` | `RFC 9557` | `contracts_zone.rs.md` |
| `NamedTimeZoneRevisionEvidence` | `TZDB revision-aware named-zone semantics` | `RFC 9557 / TZDB operational cross-checks` | `contracts_zone.rs.md` |
| `OffsetConsistencyEvidence` | `offset and named-zone consistency` | `RFC 9557 / zone semantics` | `contracts_zone.rs.md` |
| `ZuluTimeZoneInconsistencyAvoidanceEvidence` | `Z-based time-zone non-inconsistency semantics` | `RFC 9557 / RFC 3339 update semantics` | `contracts_zone.rs.md` |
| `OffsetOnlySemanticsEvidence` | `offset-only semantics` | `RFC 9557` | `contracts_zone.rs.md` |
| `TemporalOrderingEvidence` | `ordering preservation semantics` | `RFC 3339` | `contracts_rfc3339.rs.md` |
| `UtcOffsetPrecisionEvidence` | `UTC-offset precision preservation` | `RFC 3339 / ISO 8601-1` | `contracts_rfc3339.rs.md` |

## Notes

- RFC-governed evidence bundles in this file are already section-cited in
  their dedicated worksheets and are intentionally kept out of the standards
  evidence map above; the externalized proof-carrier table records that
  ownership explicitly.
- RFC 9557 zone and named-time-zone bundles are staged in
  `contracts_zone.rs.md` and `contracts_rfc9557.rs.md` rather than duplicated
  here.
- `LocalTimeEvidence` must keep amendment-backed end-of-day references tied
  explicitly to `iso-8601-1-2019-amd1-2022.*`.
- The lossy-conversion bundles are accord-level sidecars rather than
  standard-named concepts, so the worksheet records the clauses they enforce.

## Proof Composition Coverage Audit

| Evidence family | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `core ISO 8601-1 bundles` | `covered` | `CalendarDateEvidence`, `OrdinalDateEvidence`, `WeekDateEvidence`, `UtcOffsetEvidence`, `LocalDateTimeEvidence`, `OffsetDateTimeEvidence` | Repo-local WD text now anchors the open-text cross-checks. |
| `interval and duration bundles` | `covered` | `DurationDesignatorRepresentationEvidence`, `DurationFormEvidence`, `DurationRepresentationEvidence`, `DurationAlternativeFormEvidence`, `DurationWeekFormEvidence`, `TimeIntervalEvidence`, `InheritedIntervalEndComponentsEvidence`, `InheritedIntervalZoneEvidence`, `CompleteTimePointDateRepresentationEvidence`, `CompleteTimePointTimeRepresentationEvidence`, `CompleteTimePointRepresentationEvidence`, `CompleteIntervalDurationRepresentationEvidence`, `CompleteStartEndIntervalSubstitutionEvidence`, `CompleteStartDurationIntervalSubstitutionEvidence`, `CompleteDurationEndIntervalSubstitutionEvidence`, `RecurringIntervalEvidence`, `CompleteRecurringIntervalRepresentationEvidence`, `OtherThanCompleteRecurringIntervalRepresentationEvidence`, `ExtendedIntervalBoundaryEvidence` | Designator and alternative duration families, `4.4.5` inheritance, `4.4.4.5` substitution branches, and recurring-form families are all explicit in the proof graph. |
| `CalConnect explicit-form and recurrence bundles` | `covered` | `QualifiedTemporalValueEvidence`, `ExplicitTemporalFormEvidence`, `ExplicitDurationEvidence`, `ExplicitTimeIntervalEvidence`, `ExplicitIntervalDurationSubstitutionEvidence`, `ExplicitIntervalEndComponentInheritanceEvidence`, `ExplicitIntervalShiftPropagationEvidence`, `GroupedTimeScaleUnitEvidence`, `DateTimeFormulaEvidence`, `DateTimeFormulaEvaluationSemanticsEvidence`, `DateTimeFormulaEvaluationResultEvidence`, `SelectionExpressionEvidence`, `RepeatRuleEvidence`, `RecurringIntervalWithRepeatRuleEvidence` | The proof graph now records explicit-form, explicit-duration, explicit-interval, grouped-unit, formula, formula-evaluation-result provenance, selection, and repeat-rule evidence families directly, and the recurring-with-repeat-rule bundle now embeds full repeat-rule sidecars rather than reducing that branch to coarse repeat-rule validity. |
| `extended temporal bundles` | `covered` | `QualifiedTemporalExpressionEvidence`, `EnhancedIntervalLevelOneEvidence`, `EnhancedIntervalLevelTwoEvidence`, `SeasonalTemporalExpressionEvidence`, `UnspecifiedComponentExpressionEvidence`, `TemporalSetExpressionEvidence` | Extended expression families, including enhanced-interval Level 1/Level 2 semantics, remain explicitly bundled. |
| `precision and conversion bundles` | `covered` | `PrecisionPreservationEvidence`, `LossyConversionAuthorityEvidence`, `LosslessConversionEvidence`, `SubsecondTruncationEvidence` | Accord-level sidecars remain explicit and clause-backed. |

## Checklist

- [x] Revisit recurring-interval proof bundles once the interval worksheet
  decides whether complete and other-than-complete recurring forms need
  distinct sidecars.
- [x] Add explicit proof bundles for the `4.4.4.5` complete-interval
  substitution families.
- [x] Add explicit proof-composition worksheet coverage for the CalConnect
  explicit-form, explicit-interval, selection, and repeat-rule evidence
  families.
