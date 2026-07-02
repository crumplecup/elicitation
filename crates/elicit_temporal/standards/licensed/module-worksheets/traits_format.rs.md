# Citation Worksheet: `src/traits/format.rs`

This worksheet tracks exact standards citations for the formatter trait seams in
`src/traits/format.rs`.

## Source set

- `../iso-8601-1-2019.*`
- `../iso-8601-2-2019.*`
- `../../public/calconnect-cc-18011-2018.xml`

## Standards seam map

| Surface item | Current topic label | Exact clause | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `TemporalFormatter` | `basic and extended forms; qualified and extended temporal forms` | | | `no` |
| `format_calendar_date_extended` | `extended representation` | | | `no` |
| `format_calendar_date_basic` | `basic representation` | | | `no` |
| `format_reduced_calendar_date_extended` | `reduced precision calendar date representation` | | | `no` |
| `format_reduced_calendar_date_basic` | `reduced precision calendar date representation` | | | `no` |
| `format_ordinal_date_extended` | `ordinal date representation` | | | `no` |
| `format_ordinal_date_basic` | `ordinal date representation` | | | `no` |
| `format_week_date_extended` | `week date representation` | | | `no` |
| `format_week_date_basic` | `week date representation` | | | `no` |
| `format_local_time_extended` | `local time representation` | | | `no` |
| `format_local_time_basic` | `local time representation` | | | `no` |
| `format_utc_offset_extended` | `UTC offset representation` | | | `no` |
| `format_utc_offset_basic` | `UTC offset representation` | | | `no` |
| `format_local_date_time_extended` | `combined date-time representation` | | | `no` |
| `format_local_date_time_basic` | `combined date-time representation` | | | `no` |
| `format_date_with_shift` | `date with shift` | `CC 18011:2018 §4.3 - Date with shift` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `format_time_of_day_with_shift` | `time of day with time shift` | `CC 18011:2018 §4.3 - Time of day with time shift` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `format_extended_year` | `letter-prefixed, negative, exponential, and significant-digit year forms` | | | `no` |
| `format_decade` | `decade component; decade representations` | | | `no` |
| `format_century` | `century component; century representations` | | | `no` |
| `format_qualified_temporal_value` | `qualification of temporal expressions` | | | `no` |
| `format_seasonal_temporal_expression` | `seasons and seasonal temporal expressions` | | | `no` |
| `format_sub_year_grouping_expression` | `sub-year groupings` | | | `no` |
| `format_unspecified_component_expression` | `unspecified digits and unspecified components` | | | `no` |
| `format_temporal_set` | `temporal sets and set representation refinements` | | | `no` |
| `format_grouped_time_scale_unit` | `grouped time scale units` | | | `no` |
| `format_date_time_formula` | `date and time arithmetic` | | | `no` |
| `format_duration` | `duration representations` | `4.4.2 b); 4.4.3` | ISO/WD 8601-1:2016(E) lines 1704-1768 | `yes` |
| `format_recurring_interval` | `recurring interval representations` | `4.5.1; 4.5.2; 4.5.3; 4.5.4` | ISO/WD 8601-1:2016(E) lines 1949-2011 | `yes` |
| `format_time_interval` | `time interval representations; open and unknown interval boundaries` | `4.4.1; 4.4.2; 4.4.4; 4.4.5; ISO 8601-2:2019 10.2` | ISO/WD 8601-1:2016(E) lines 1688-1943; ISO 8601-2:2019 sample TOC | `yes` |

## Notes

- RFC 3339 and RFC 9557 formatter seams in this file are already section-cited
  and are intentionally excluded from this worksheet.
