# Citation Worksheet: `src/contracts/precision.rs`

This worksheet tracks exact ISO citations for the precision and rounding
contracts in `src/contracts/precision.rs`.

## Source set

- `../iso-8601-1-2019.*`
- `../iso-8601-2-2019.*`
- `../../public/calconnect-cc-18011-2018.xml`

## ISO contract map

| Contract symbol | Current topic label | Exact clause | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `FractionalSecondPrecisionDeclared` | `decimal fractions` | `ISO 8601-1:2019/Amd 1:2022 5.3.1.4; RFC 3339 §5.6` | `iso-8601-1-2019-amd1-2022.sample.txt lines 205-206; RFC 3339` | `yes` |
| `FractionalSecondDigitsAreContiguous` | `decimal fractions` | `ISO 8601-1:2019/Amd 1:2022 5.3.1.4; RFC 3339 §5.6` | `iso-8601-1-2019-amd1-2022.sample.txt lines 205-206; RFC 3339` | `yes` |
| `PrecisionReductionDeclared` | `reduced precision and truncation semantics` | `ISO 8601-2:2019 7.11; 7.13; CalConnect CC 18011:2018 representations-precision; representations-reduced-precision` | `iso-8601-2-2019.sample.txt lines 259-261; ../../public/calconnect-cc-18011-2018.xml lines 1013-1048` | `yes` |
| `RoundingModeDeclared` | `reduced precision and rounding semantics` | `ISO 8601-2:2019 7.13; 14.2; 14.3; 14.4; CalConnect CC 18011:2018 §8.2; §8.3-§8.5` | `iso-8601-2-2019.sample.txt lines 261, 437-441; ../../public/calconnect-cc-18011-2018.xml lines 1041-1048, 1363-1444` | `yes` |

## Notes

- The RFC 3339 `SubsecondDigitsPreserved` contract is already section-cited and
  is intentionally excluded from this worksheet.
- `RoundingModeDeclared` is an accord-level sidecar law rather than a
  standard-named token, so the worksheet tracks the clauses that require the
  policy to remain explicit.

## Precision Coverage Audit

| Clause concept | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `fractional-second precision declaration` | `covered` | `FractionalSecondPrecisionDeclared`, `FractionalSecondDigitsAreContiguous` | Fractional precision and digit contiguity are explicit. |
| `reduced precision and truncation semantics` | `covered` | `PrecisionReductionDeclared`, `RoundingModeDeclared` | Accord-level rounding authority remains explicit and clause-backed. |
