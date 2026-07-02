# Citation Worksheet: `src/traits/parse.rs`

This worksheet tracks exact standards citations for the parser trait seams in
`src/traits/parse.rs`.

## Source set

- `../iso-8601-1-2019.*`
- `../iso-8601-2-2019.*`
- `../../public/calconnect-cc-18011-2018.xml`

## Standards seam map

| Surface item | Current topic label | Exact clause | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `TemporalParser` | `core date/time representations; qualified and extended temporal forms` | | | `no` |
| `parse_calendar_date` | `calendar date representation` | | | `no` |
| `parse_reduced_calendar_date` | `reduced precision calendar date representation` | | | `no` |
| `parse_extended_year` | `letter-prefixed, negative, exponential, and significant-digit year forms` | | | `no` |
| `parse_decade` | `decade component; decade representations` | | | `no` |
| `parse_century` | `century component; century representations` | | | `no` |
| `parse_qualified_temporal_value` | `qualification of temporal expressions` | | | `no` |
| `parse_ordinal_date` | `ordinal date representation` | | | `no` |
| `parse_week_date` | `week date representation` | | | `no` |
| `parse_local_time` | `local time representation` | | | `no` |
| `parse_utc_offset` | `UTC offset representation` | | | `no` |
| `parse_local_date_time` | `combined date-time representation` | | | `no` |
| `parse_date_with_shift` | `date with shift` | `CC 18011:2018 §4.3 - Date with shift` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `parse_time_of_day_with_shift` | `time of day with time shift` | `CC 18011:2018 §4.3 - Time of day with time shift` | `../../public/calconnect-cc-18011-2018.xml` | `yes` |
| `parse_seasonal_temporal_expression` | `seasons and seasonal temporal expressions` | | | `no` |
| `parse_sub_year_grouping_expression` | `sub-year groupings` | | | `no` |
| `parse_unspecified_component_expression` | `unspecified digits and unspecified components` | | | `no` |
| `parse_temporal_set` | `temporal sets and set representation refinements` | | | `no` |
| `parse_grouped_time_scale_unit` | `grouped time scale units` | | | `no` |
| `parse_date_time_formula` | `date and time arithmetic` | | | `no` |
| `parse_time_interval` | `time interval representations; open and unknown interval boundaries` | `4.4.1; 4.4.2; 4.4.4; 4.4.5; ISO 8601-2:2019 10.2` | ISO/WD 8601-1:2016(E) lines 1688-1943; ISO 8601-2:2019 sample TOC | `yes` |

## Notes

- RFC 3339 and RFC 9557 parser seams in this file are already section-cited and
  are intentionally excluded from this worksheet.
