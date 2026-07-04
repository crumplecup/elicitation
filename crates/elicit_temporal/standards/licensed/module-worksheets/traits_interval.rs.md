# Citation Worksheet: `src/traits/interval.rs`

This worksheet tracks exact ISO citations for the interval-factory trait seams
in `src/traits/interval.rs`.

## Source set

- `../iso-8601-1-2019.*`
- `../../public/iso-wd-8601-1-2016.txt`
- `../../public/iso-wd-8601-2-2016.txt`

## ISO seam map

| Surface item | Current topic label | Exact clause | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `TemporalIntervalFactory` | `duration, interval, and recurring-interval representations` | `4.4; 4.5; ISO 8601-2:2019 10.2` | `../../public/iso-wd-8601-1-2016.txt` lines 1688-1978; `../../public/iso-wd-8601-2-2016.txt` lines 425-487 | `yes` |
| `parse_duration` | `duration representations` | `4.4.2 b); 4.4.3` | `../../public/iso-wd-8601-1-2016.txt` lines 1704-1768 | `yes` |
| `parse_recurring_interval` | `recurring interval representations` | `4.5.1; 4.5.2; 4.5.3; 4.5.4` | `../../public/iso-wd-8601-1-2016.txt` lines 1949-2011 | `yes` |
| `order_offset_endpoints` | `interval ordering semantics` | `3.1.1.6; 3.1.1.8` | ISO 8601-1:2019 sample text lines 458-484 | `yes` |
| `parse_interval` | `time interval representations; open and unknown interval boundaries` | `4.4.1; 4.4.2; 4.4.4; 4.4.5; ISO 8601-2:2019 10.2` | `../../public/iso-wd-8601-1-2016.txt` lines 1688-1943; `../../public/iso-wd-8601-2-2016.txt` lines 425-487 | `yes` |

## Interval Trait Coverage Audit

| Surface item | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `parse_duration` | `partially covered` | `parse_duration`, `ParsedDurationResult`, duration contracts in `contracts_interval.rs` | The seam accepts ISO duration forms generically, but it does not yet distinguish designator versus alternative complete-duration representations. |
| `parse_interval` | `partially covered` | `parse_interval`, `ParsedTimeIntervalResult`, `OpenIntervalBoundaryDeclared`, `UnknownIntervalBoundaryDeclared` | Base interval validity and extended interval-boundary semantics are present, but inherited end-component and inherited zone semantics from `4.4.5` do not yet travel as dedicated proof sidecars. |
| `parse_recurring_interval` | `partially covered` | `parse_recurring_interval`, `ParsedRecurringIntervalResult`, recurring interval contracts in `contracts_interval.rs` | The seam covers recurrence prefixes and interval payloads, but it does not yet separate complete versus other-than-complete recurring interval families. |
| `order_offset_endpoints` | `covered` | `order_offset_endpoints`, `IntervalEndpointsOrdered`, fixed-instant proof sidecars | Chronological endpoint ordering is already expressed through proof-carrying inputs and output. |

## Checklist

- [ ] Decide whether duration parsing must distinguish designator and
  alternative complete-duration forms explicitly.
- [ ] Add dedicated interval result sidecars for inherited-end-component and
  inherited-zone semantics if `4.4.5` fidelity is required at consumer
  boundaries.
- [ ] Decide whether recurring interval parsing should expose complete versus
  other-than-complete form distinctions directly.
