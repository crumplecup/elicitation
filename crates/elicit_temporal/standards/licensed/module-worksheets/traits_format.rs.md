# Citation Worksheet: `src/traits/format.rs`

This worksheet tracks exact standards citations for the formatter trait seams in
`src/traits/format.rs`.

## Source set

- `../iso-8601-1-2019.*`
- `../iso-8601-2-2019.*`
- `../../public/calconnect-cc-18011-2018.xml`
- `../../public/iso-wd-8601-1-2016.txt`

## Standards seam map

| Surface item | Current topic label | Exact clause | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `TemporalFormatter` | `basic and extended forms; qualified and extended temporal forms` | `ISO 8601-1:2019 2.3.3; 2.3.4; 5.2; 5.3; 5.4; 5.5; 5.6; ISO 8601-2:2019 4.3.5; 4.3.6; 4.7.2; 4.7.3; 4.7.4; 4.8.1; 4.8.2; 4.8.3; 5.1; 5.2; 5.3; 5.4; 6.1; 6.2; 6.3; 6.4; 6.5; 6.6; 8.2.1; 8.2.2; 8.2.3; 8.4.4; 8.4.5; 8.4.6; 8.5; 9.2.1; 9.2.2; 9.3; 10.2; 14.1; 14.2; 14.3; 14.4; CalConnect CC 18011:2018 §4.3; §5; §8` | `iso-8601-1-2019.sample.txt lines 147-214; iso-8601-2-2019.sample.txt lines 142-214, 275-360, 437-441; ../../public/calconnect-cc-18011-2018.xml lines 796-936, 1013-1048, 1307-1444` | `yes` |
| `format_calendar_date_extended` | `extended representation` | `ISO 8601-1:2019 5.2.2` | `iso-8601-1-2019.sample.txt lines 156-156` | `yes` |
| `format_calendar_date_basic` | `basic representation` | `ISO 8601-1:2019 5.2.2` | `iso-8601-1-2019.sample.txt lines 156-156` | `yes` |
| `format_reduced_calendar_date_extended` | `reduced precision calendar date representation` | `ISO 8601-1:2019 5.2.2` | `iso-8601-1-2019.sample.txt lines 156-156` | `yes` |
| `format_reduced_calendar_date_basic` | `reduced precision calendar date representation` | `ISO 8601-1:2019 5.2.2` | `iso-8601-1-2019.sample.txt lines 156-156` | `yes` |
| `format_ordinal_date_extended` | `ordinal date representation` | `ISO 8601-1:2019 5.2.3` | `iso-8601-1-2019.sample.txt lines 158-158` | `yes` |
| `format_ordinal_date_basic` | `ordinal date representation` | `ISO 8601-1:2019 5.2.3` | `iso-8601-1-2019.sample.txt lines 158-158` | `yes` |
| `format_week_date_extended` | `week date representation` | `ISO 8601-1:2019 5.2.4` | `iso-8601-1-2019.sample.txt lines 160-160` | `yes` |
| `format_week_date_basic` | `week date representation` | `ISO 8601-1:2019 5.2.4` | `iso-8601-1-2019.sample.txt lines 160-160` | `yes` |
| `format_local_time_extended` | `local time representation` | `ISO 8601-1:2019 5.3.1; ISO 8601-1:2019/Amd 1:2022 5.3.1.4; 5.3.2` | `iso-8601-1-2019.sample.txt lines 164-172; iso-8601-1-2019-amd1-2022.sample.txt lines 202-216` | `yes` |
| `format_local_time_basic` | `local time representation` | `ISO 8601-1:2019 5.3.1; ISO 8601-1:2019/Amd 1:2022 5.3.1.4; 5.3.2` | `iso-8601-1-2019.sample.txt lines 164-172; iso-8601-1-2019-amd1-2022.sample.txt lines 202-216` | `yes` |
| `format_utc_offset_extended` | `UTC offset representation` | `ISO 8601-1:2019 5.3.4; open-text cross-check ISO/WD 8601-1:2016(E) 4.2.5.1; 4.2.5.2` | `iso-8601-1-2019.sample.txt lines 170-170; ../../public/iso-wd-8601-1-2016.txt lines 1500-1574` | `yes` |
| `format_utc_offset_basic` | `UTC offset representation` | `ISO 8601-1:2019 5.3.4; open-text cross-check ISO/WD 8601-1:2016(E) 4.2.5.1; 4.2.5.2` | `iso-8601-1-2019.sample.txt lines 170-170; ../../public/iso-wd-8601-1-2016.txt lines 1500-1574` | `yes` |
| `format_local_date_time_extended` | `combined date-time representation` | `ISO 8601-1:2019 5.4.2; 5.4.3` | `iso-8601-1-2019.sample.txt lines 191-193; ../../public/iso-wd-8601-1-2016.txt lines 1547-1646` | `yes` |
| `format_local_date_time_basic` | `combined date-time representation` | `ISO 8601-1:2019 5.4.2; 5.4.3` | `iso-8601-1-2019.sample.txt lines 191-193; ../../public/iso-wd-8601-1-2016.txt lines 1547-1646` | `yes` |
| `format_date_with_shift` | `date with shift` | `CC 18011:2018 §4.3 - Date with shift` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `format_time_of_day_with_shift` | `time of day with time shift` | `CC 18011:2018 §4.3 - Time of day with time shift` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `format_extended_year` | `letter-prefixed, negative, exponential, and significant-digit year forms` | `ISO 8601-2:2019 4.7.2; 4.7.3; 4.7.4` | `iso-8601-2-2019.sample.txt lines 146-150, 961-972` | `yes` |
| `format_decade` | `decade component; decade representations` | `ISO 8601-1:2019/Amd 1:2022 4.3.11; ISO 8601-2:2019 4.3.5` | `iso-8601-1-2019-amd1-2022.sample.txt lines 184-191; iso-8601-2-2019.sample.txt lines 1026-1027` | `yes` |
| `format_century` | `century component; century representations` | `ISO 8601-1:2019/Amd 1:2022 4.3.12; ISO 8601-2:2019 4.3.6` | `iso-8601-1-2019-amd1-2022.sample.txt lines 193-200; iso-8601-2-2019.sample.txt lines 1029-1030` | `yes` |
| `format_qualified_temporal_value` | `qualification of temporal expressions` | `ISO 8601-2:2019 8.2.1; 8.2.2; 8.2.3; 8.4.4; 8.4.5; 8.4.6; 8.5` | `iso-8601-2-2019.sample.txt lines 280-345` | `yes` |
| `format_seasonal_temporal_expression` | `seasons and seasonal temporal expressions` | `ISO 8601-2:2019 4.8.1; 4.8.2; 4.8.3` | `iso-8601-2-2019.sample.txt lines 154-158, 873-875` | `yes` |
| `format_sub_year_grouping_expression` | `sub-year groupings` | `ISO 8601-2:2019 4.8.1; 4.8.2; 4.8.3` | `iso-8601-2-2019.sample.txt lines 154-158, 873-875` | `yes` |
| `format_unspecified_component_expression` | `unspecified digits and unspecified components` | `ISO 8601-2:2019 9.2.1; 9.2.2; 9.3` | `iso-8601-2-2019.sample.txt lines 350-356` | `yes` |
| `format_temporal_set` | `temporal sets and set representation refinements` | `ISO 8601-2:2019 6.1; 6.2; 6.3; 6.4; 6.5; 6.6` | `iso-8601-2-2019.sample.txt lines 201-212` | `yes` |
| `format_grouped_time_scale_unit` | `grouped time scale units` | `ISO 8601-2:2019 5.1; 5.2; 5.3; 5.4; 5.4.2` | `iso-8601-2-2019.sample.txt lines 167-177, 935-942` | `yes` |
| `format_date_time_formula` | `date and time arithmetic` | `ISO 8601-2:2019 14.1; 14.2; 14.3; 14.4` | `iso-8601-2-2019.sample.txt lines 438-441` | `yes` |
| `format_duration` | `duration representations` | `4.4.2 b); 4.4.3` | `../../public/iso-wd-8601-1-2016.txt` lines 1704-1768 | `yes` |
| `format_recurring_interval` | `recurring interval representations` | `4.5.1; 4.5.2; 4.5.3; 4.5.4` | `../../public/iso-wd-8601-1-2016.txt` lines 1949-2011 | `yes` |
| `format_time_interval` | `time interval representations; open and unknown interval boundaries` | `4.4.1; 4.4.2; 4.4.4; 4.4.5; ISO 8601-2:2019 10.2` | `../../public/iso-wd-8601-1-2016.txt` lines 1688-1943; `iso-8601-2-2019.sample.txt` line 360 | `yes` |

## Formatter Seam Coverage Audit

| Surface item | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `format_utc_offset_*` | `covered` | `format_utc_offset_extended`, `format_utc_offset_basic`, `UtcOffsetEvidence` | Repo-local WD text now anchors the open-text clause cross-check. |
| `format_local_date_time_*` | `covered` | `format_local_date_time_extended`, `format_local_date_time_basic`, `LocalDateTimeEvidence` | Combined date-time clause coverage is explicit and stable. |
| `format_duration` | `partially covered` | `format_duration`, `DurationFormEvidence` | Formatter coverage inherits the open alternative-duration distinction tracked in `traits_interval.rs.md`. |
| `format_recurring_interval` | `partially covered` | `format_recurring_interval`, `RecurringIntervalEvidence` | Complete versus other-than-complete recurring families remain flattened. |
| `format_time_interval` | `partially covered` | `format_time_interval`, `TimeIntervalEvidence`, `ExtendedIntervalBoundaryEvidence` | Formatter coverage inherits the open `4.4.5` substitution semantics tracked in `traits_interval.rs.md`. |

## Checklist

- [ ] Keep formatter interval seams aligned with the interval worksheet once the
  remaining interval-form distinctions land.

## Notes

- RFC 3339 and RFC 9557 formatter seams in this file are already section-cited
  and are intentionally excluded from this worksheet.
