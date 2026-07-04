# Citation Worksheet: `src/traits/conversion.rs`

This worksheet tracks exact ISO citations for the conversion trait seams in
`src/traits/conversion.rs`.

## Source set

- `../iso-8601-1-2019.*`
- `../iso-8601-1-2019-amd1-2022.*`
- `../iso-8601-2-2019.*`
- `../../public/calconnect-cc-18011-2018.xml`

## ISO seam map

| Surface item | Current topic label | Exact clause | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `adjust_precision_losslessly` | `reduced precision and rounding semantics` | `ISO 8601-1:2019/Amd 1:2022 5.3.1.4; ISO 8601-2:2019 7.11; 7.12; 7.13` | `iso-8601-1-2019-amd1-2022.sample.txt lines 205-208; iso-8601-2-2019.sample.txt lines 259-261` | `yes` |
| `truncate_subseconds` | `reduced precision and rounding semantics` | `ISO 8601-2:2019 7.13; 14.2; 14.3; 14.4; CalConnect CC 18011:2018 §8.2; §8.3-§8.5` | `iso-8601-2-2019.sample.txt lines 261, 437-441; ../../public/calconnect-cc-18011-2018.xml lines 1363-1444` | `yes` |

## Notes

- RFC-governed conversion seams in this file are already section-cited and are
  intentionally excluded from this worksheet.
- `truncate_subseconds` uses an accord-level authority sidecar layered on top
  of the cited truncation clauses.

## Conversion Seam Coverage Audit

| Surface item | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `adjust_precision_losslessly` | `covered` | `adjust_precision_losslessly`, `LosslessConversionEvidence` | Lossless precision adjustment remains explicit and clause-backed. |
| `truncate_subseconds` | `covered` | `truncate_subseconds`, `SubsecondTruncationEvidence`, `LossyConversionAuthorityEvidence` | Truncation authority remains explicit through proof sidecars. |
