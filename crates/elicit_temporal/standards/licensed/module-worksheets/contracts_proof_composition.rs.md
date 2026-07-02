# Citation Worksheet: `src/contracts/proof_composition.rs`

This worksheet tracks exact standards citations for the aggregate evidence bundles in
`src/contracts/proof_composition.rs`.

## Source set

- `../iso-8601-1-2019.*`
- `../iso-8601-1-2019-amd1-2022.*`
- `../iso-8601-2-2019.*`
- `../../public/calconnect-cc-18011-2018.xml`

## Standards evidence map

| Evidence struct | Current topic label | Exact clause | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `CalendarDateEvidence` | `calendar date interchange core` | | | `no` |
| `DecadeEvidence` | `decade component ordinal range; decade representations` | | | `no` |
| `CenturyEvidence` | `century component ordinal range; century representations` | | | `no` |
| `OrdinalDateEvidence` | `ordinal date interchange core` | | | `no` |
| `WeekDateEvidence` | `week date interchange core` | | | `no` |
| `LocalTimeEvidence` | `time-of-day interchange core; end-of-day technical correction` | | | `no` |
| `UtcOffsetEvidence` | `UTC offset interchange core` | | | `no` |
| `LocalDateTimeEvidence` | `combined date and time interchange core` | | | `no` |
| `OffsetDateTimeEvidence` | `date-time with UTC relationship` | | | `no` |
| `CompleteDateEvidence` | `explicit complete date families` | `CC 18011:2018 §4.3 - Date` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `DateWithShiftEvidence` | `date with shift` | `CC 18011:2018 §4.3 - Date with shift` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `TimeOfDayWithShiftEvidence` | `time of day with time shift` | `CC 18011:2018 §4.3 - Time of day with time shift` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `FixedInstantEvidence` | `date-time with UTC relationship` | | | `no` |
| `PrecisionPreservationEvidence` | `decimal fractions` | | | `no` |
| `BackendConversionEvidence` | `representation changes across equivalent forms` | | | `no` |
| `LossyConversionAuthorityEvidence` | `reduced-precision and rounding semantics` | | | `no` |
| `LosslessConversionEvidence` | `reduced-precision semantics` | | | `no` |
| `SubsecondTruncationEvidence` | `reduced-precision and rounding semantics` | | | `no` |
| `DurationFormEvidence` | `duration representations` | `4.4.2 b); 4.4.3.2` | `contracts_interval.rs.md` | `yes` |
| `TimeIntervalEvidence` | `time interval representations` | `4.4.1; 4.4.2 a)` | `contracts_interval.rs.md` | `yes` |
| `IntervalEndpointOrderingEvidence` | `interval ordering and duration semantics` | `3.1.1.6; 3.1.1.8` | `contracts_interval.rs.md` | `yes` |
| `RecurringIntervalEvidence` | `recurring interval representations` | `4.5.1; 4.5.2` | `contracts_interval.rs.md` | `yes` |
| `QualifiedTemporalExpressionEvidence` | `uncertain and approximate temporal expressions` | | | `no` |
| `ExtendedIntervalBoundaryEvidence` | `open and unknown interval boundaries` | | | `no` |
| `SeasonalTemporalExpressionEvidence` | `seasons and named seasonal temporal expressions` | | | `no` |
| `UnspecifiedComponentExpressionEvidence` | `unspecified digits and unspecified components` | | | `no` |
| `TemporalSetExpressionEvidence` | `temporal set expressions` | | | `no` |
| `TemporalSetRangeSemanticsEvidence` | `set representation` | | | `no` |

## Notes

- RFC-governed evidence bundles in this file are already section-cited and are
  intentionally excluded from this worksheet.
- `LocalTimeEvidence` must keep amendment-backed end-of-day references tied
  explicitly to `iso-8601-1-2019-amd1-2022.*`.
