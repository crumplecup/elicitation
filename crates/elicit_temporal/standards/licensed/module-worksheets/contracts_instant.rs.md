# Citation Worksheet: `src/contracts/instant.rs`

This worksheet isolates the remaining ISO citation debt inside the mixed
authority surface of `src/contracts/instant.rs`.

## Source set

- `../iso-8601-1-2019.*`
- `../../public/iso-wd-8601-1-2016.txt`

## ISO contract map

| Contract symbol | Current topic label | Exact clause | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `OffsetDateTimeIdentifiesSingleInstant` | `date-time with UTC relationship` | `ISO 8601-1:2019 5.4.2; 5.4.3; RFC 3339 §5.6` | `iso-8601-1-2019.sample.txt` lines 191-193; `../../public/iso-wd-8601-1-2016.txt` lines 1575-1646; RFC 3339 | `yes` |

## Notes

- RFC-governed instant contracts in this file are already section-cited and are
  intentionally excluded from this worksheet.

## Instant Coverage Audit

| Clause concept | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `5.4.2; 5.4.3 date-time with UTC relationship` | `covered` | `OffsetDateTimeIdentifiesSingleInstant` | The instant-identifying semantics for offset date-times are explicit. |
