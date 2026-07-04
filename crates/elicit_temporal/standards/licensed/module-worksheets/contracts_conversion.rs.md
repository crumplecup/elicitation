# Citation Worksheet: `src/contracts/conversion.rs`

This worksheet tracks exact ISO citations for the conversion-law contracts in
`src/contracts/conversion.rs`.

## Source set

- `../iso-8601-1-2019.*`
- `../iso-8601-2-2019.*`
- `../../public/calconnect-cc-18011-2018.xml`

## ISO contract map

| Contract symbol | Current topic label | Exact clause | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `ConversionSourceSemanticKindDeclared` | `distinct temporal representation families` | `ISO 8601-1:2019 3.1.3; 5.2; 5.3; 5.4; 5.5; 5.6` | `iso-8601-1-2019.sample.txt lines 75-97, 147-214` | `yes` |
| `ConversionTargetSemanticKindDeclared` | `distinct temporal representation families` | `ISO 8601-1:2019 3.1.3; 5.2; 5.3; 5.4; 5.5; 5.6` | `iso-8601-1-2019.sample.txt lines 75-97, 147-214` | `yes` |
| `ConversionDropsSubsecondPrecision` | `reduced precision semantics` | `ISO 8601-2:2019 7.11; 7.12; 7.13; CalConnect CC 18011:2018 representations-precision; representations-decimal; representations-reduced-precision` | `iso-8601-2-2019.sample.txt lines 259-261; ../../public/calconnect-cc-18011-2018.xml lines 1013-1048` | `yes` |
| `ConversionRequiresExplicitAuthorityWhenLossy` | `extended and reduced-precision semantics` | `ISO 8601-2:2019 7.13; 14.2; 14.3; 14.4; CalConnect CC 18011:2018 §8.2; §8.3-§8.5` | `iso-8601-2-2019.sample.txt lines 261, 437-441; ../../public/calconnect-cc-18011-2018.xml lines 1363-1444` | `yes` |

## Notes

- RFC 3339 and RFC 9557 conversion contracts in this file are already
  section-cited and are intentionally excluded from this worksheet.
- Explicit lossy-conversion authority is an accord-level law layered on top of
  the cited reduced-precision and truncation clauses.

## Conversion Coverage Audit

| Clause concept | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `representation-family distinction` | `covered` | `ConversionSourceSemanticKindDeclared`, `ConversionTargetSemanticKindDeclared` | Conversion endpoints stay explicit about source and target semantic kinds. |
| `reduced-precision conversion semantics` | `covered` | `ConversionDropsSubsecondPrecision`, `ConversionRequiresExplicitAuthorityWhenLossy` | Lossy conversion remains explicit and clause-backed. |
