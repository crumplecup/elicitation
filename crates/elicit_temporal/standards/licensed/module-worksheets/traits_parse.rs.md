# Citation Worksheet: `src/traits/parse.rs`

This worksheet tracks exact standards citations for the parser trait seams in
`src/traits/parse.rs`.

## Source set

- `../iso-8601-1-2019.*`
- `../iso-8601-2-2019.*`
- `../../public/calconnect-cc-18011-2018.xml`
- `../../public/iso-wd-8601-1-2016.txt`

## Standards seam map

| Surface item | Current topic label | Exact clause | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `TemporalParser` | `core date/time representations; qualified and extended temporal forms` | `ISO 8601-1:2019 2.3.3; 2.3.4; 5.2; 5.3; 5.4; 5.5; 5.6; ISO 8601-2:2019 4.3.5; 4.3.6; 4.7.2; 4.7.3; 4.7.4; 4.8.1; 4.8.2; 4.8.3; 5.1; 5.2; 5.3; 5.4; 6.1; 6.2; 6.3; 6.4; 6.5; 6.6; 8.2.1; 8.2.2; 8.2.3; 8.4.4; 8.4.5; 8.4.6; 8.5; 9.2.1; 9.2.2; 9.3; 10.2; 14.1; 14.2; 14.3; 14.4; CalConnect CC 18011:2018 §4.3; §5; §8` | `iso-8601-1-2019.sample.txt lines 147-214; iso-8601-2-2019.sample.txt lines 142-214, 275-360, 437-441; ../../public/calconnect-cc-18011-2018.xml lines 796-936, 1013-1048, 1307-1444` | `yes` |
| `parse_calendar_date` | `calendar date representation` | `ISO 8601-1:2019 5.2.2` | `iso-8601-1-2019.sample.txt lines 156-156` | `yes` |
| `parse_reduced_calendar_date` | `reduced precision calendar date representation` | `ISO 8601-1:2019 5.2.2` | `iso-8601-1-2019.sample.txt lines 156-156` | `yes` |
| `parse_extended_year` | `letter-prefixed, negative, exponential, and significant-digit year forms` | `ISO 8601-2:2019 4.7.2; 4.7.3; 4.7.4` | `iso-8601-2-2019.sample.txt lines 146-150, 961-972` | `yes` |
| `parse_decade` | `decade component; decade representations` | `ISO 8601-1:2019/Amd 1:2022 4.3.11; ISO 8601-2:2019 4.3.5` | `iso-8601-1-2019-amd1-2022.sample.txt lines 184-191; iso-8601-2-2019.sample.txt lines 1026-1027` | `yes` |
| `parse_century` | `century component; century representations` | `ISO 8601-1:2019/Amd 1:2022 4.3.12; ISO 8601-2:2019 4.3.6` | `iso-8601-1-2019-amd1-2022.sample.txt lines 193-200; iso-8601-2-2019.sample.txt lines 1029-1030` | `yes` |
| `parse_qualified_temporal_value` | `qualification of temporal expressions` | `ISO 8601-2:2019 8.2.1; 8.2.2; 8.2.3; 8.4.4; 8.4.5; 8.4.6; 8.5` | `iso-8601-2-2019.sample.txt lines 280-345` | `yes` |
| `parse_ordinal_date` | `ordinal date representation` | `ISO 8601-1:2019 5.2.3` | `iso-8601-1-2019.sample.txt lines 158-158` | `yes` |
| `parse_week_date` | `week date representation` | `ISO 8601-1:2019 5.2.4` | `iso-8601-1-2019.sample.txt lines 160-160` | `yes` |
| `parse_local_time` | `local time representation` | `ISO 8601-1:2019 5.3.1; ISO 8601-1:2019/Amd 1:2022 5.3.1.4; 5.3.2` | `iso-8601-1-2019.sample.txt lines 164-172; iso-8601-1-2019-amd1-2022.sample.txt lines 202-216` | `yes` |
| `parse_utc_offset` | `UTC offset representation` | `ISO 8601-1:2019 5.3.4; open-text cross-check ISO/WD 8601-1:2016(E) 4.2.5.1; 4.2.5.2` | `iso-8601-1-2019.sample.txt lines 170-170; ../../public/iso-wd-8601-1-2016.txt lines 1500-1574` | `yes` |
| `parse_local_date_time` | `combined date-time representation` | `ISO 8601-1:2019 5.4.2; 5.4.3` | `iso-8601-1-2019.sample.txt lines 191-193; ../../public/iso-wd-8601-1-2016.txt lines 1547-1646` | `yes` |
| `parse_date_with_shift` | `date with shift` | `CC 18011:2018 §4.3 - Date with shift` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `parse_time_of_day_with_shift` | `time of day with time shift` | `CC 18011:2018 §4.3 - Time of day with time shift` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `parse_seasonal_temporal_expression` | `seasons and seasonal temporal expressions` | `ISO 8601-2:2019 4.8.1; 4.8.2; 4.8.3` | `iso-8601-2-2019.sample.txt lines 154-158, 873-875` | `yes` |
| `parse_sub_year_grouping_expression` | `sub-year groupings` | `ISO 8601-2:2019 4.8.1; 4.8.2; 4.8.3` | `iso-8601-2-2019.sample.txt lines 154-158, 873-875` | `yes` |
| `parse_unspecified_component_expression` | `unspecified digits and unspecified components` | `ISO 8601-2:2019 9.2.1; 9.2.2; 9.3` | `iso-8601-2-2019.sample.txt lines 350-356` | `yes` |
| `parse_temporal_set` | `temporal sets and set representation refinements` | `ISO 8601-2:2019 6.1; 6.2; 6.3; 6.4; 6.5; 6.6` | `iso-8601-2-2019.sample.txt lines 201-212` | `yes` |
| `parse_grouped_time_scale_unit` | `grouped time scale units` | `ISO 8601-2:2019 5.1; 5.2; 5.3; 5.4; 5.4.2` | `iso-8601-2-2019.sample.txt lines 167-177, 935-942` | `yes` |
| `parse_date_time_formula` | `date and time arithmetic` | `ISO 8601-2:2019 14.1; 14.2; 14.3; 14.4` | `iso-8601-2-2019.sample.txt lines 438-441` | `yes` |
| `parse_time_interval` | `time interval representations; open and unknown interval boundaries` | `4.4.1; 4.4.2; 4.4.4; 4.4.5; ISO 8601-2:2019 10.2` | `../../public/iso-wd-8601-1-2016.txt` lines 1460-1697; `iso-8601-2-2019.sample.txt` line 360 | `yes` |

## Parser Seam Coverage Audit

| Surface item | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `parse_utc_offset` | `covered` | `parse_utc_offset`, `UtcOffsetEvidence` | Repo-local WD text now anchors the open-text clause cross-check. |
| `parse_local_date_time` | `covered` | `parse_local_date_time`, `LocalDateTimeEvidence` | Combined date-time clause coverage is explicit and stable. |
| `parse_time_interval` | `covered` | `parse_time_interval`, `TimeIntervalEvidence`, `ExtendedIntervalBoundaryEvidence`, `InheritedIntervalEndComponentsEvidence`, `InheritedIntervalZoneEvidence`, `CompleteIntervalSubstitutionProofBranch`, `DurationRepresentationProofBranch` | Interval parsing now carries explicit `4.4.5` inherited sidecars and `4.4.4.5` substitution semantics, while the parser seam family now exposes designator versus alternative duration branches directly. |
| `parse_qualified_temporal_value` | `covered` | `parse_qualified_temporal_value`, `ParsedQualifiedTemporalValueResult`, `QualifiedTemporalExpressionEvidence`, `QualificationPlacementEvidence`, `QualifiedTemporalValueEvidence` | The parser seam now returns the qualified-value proof, the qualification-expression proof, and the placement-family branch explicitly rather than collapsing qualification law into a single coarse token. |
| `parse_grouped_time_scale_unit` | `covered` | `parse_grouped_time_scale_unit`, `ParsedGroupedTimeScaleUnitResult`, `GroupedTimeScaleUnitEvidence` | The parser seam now returns delimiter, unit-carriage, continuity, coefficient, bounds, explicit-time-shift, truncation, and interval-conversion sidecars explicitly rather than flattening grouped-unit law into one aggregate validity token. |
| `parse_temporal_set` | `covered` | `parse_temporal_set`, `TemporalSetExpressionEvidence`, `TemporalSetRangeSemanticsEvidence` | Set-family semantics are explicit. |
| `parse_seasonal_temporal_expression` | `covered` | `parse_seasonal_temporal_expression`, `ParsedSeasonalTemporalExpressionResult`, `SeasonalTemporalExpressionEvidence` | The parser seam now returns year-and-season form, month-slot encoding, named-season declaration, and season-scope declaration as explicit sidecars instead of flattening seasonal law into coarse validity alone. |
| `parse_sub_year_grouping_expression` | `covered` | `parse_sub_year_grouping_expression`, `ParsedSubYearGroupingExpressionResult`, `SubYearGroupingExpressionEvidence`, `SubYearGroupingKindEvidence` | The parser seam now returns year-and-grouping form, month-slot encoding, and the grouping-family branch explicitly, preserving quarter, quadrimester, semestral, and seasonal distinctions. |
| `parse_unspecified_component_expression` | `covered` | `parse_unspecified_component_expression`, `ParsedUnspecifiedComponentExpressionResult`, `UnspecifiedComponentExpressionEvidence` | The parser seam now returns placeholder syntax, unknown-value semantics, Level 1 right-tail masking, and Level 2 internal-component masking as explicit sidecars rather than flattening masking law into coarse validity alone. |

## Checklist

- [x] Decide whether parser seams should expose designator versus alternative
  complete-duration branches as dedicated result sidecars.

## Notes

- RFC 3339 and RFC 9557 parser seams in this file are already section-cited and
  are intentionally excluded from this worksheet.
