# Citation Worksheet: `src/contracts/serialization.rs`

This worksheet tracks exact ISO citations for the serialization-form contracts
in `src/contracts/serialization.rs`.

## Source set

- `../iso-8601-1-2019.*`
- `../../public/iso-wd-8601-1-2016.txt`

## ISO contract map

| Contract symbol | Current topic label | Exact clause | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `Iso8601BasicFormUsesCompactRepresentation` | `basic format` | `ISO 8601-1:2019 3.1.3; 5.1; ISO/WD 8601-1:2016(E) 2.3.3` | `iso-8601-1-2019.sample.txt` lines 82, 150; `../../public/iso-wd-8601-1-2016.txt` lines 605-611 | `yes` |
| `Iso8601ExtendedFormUsesSeparators` | `extended format` | `ISO 8601-1:2019 3.1.3; 5.1; ISO/WD 8601-1:2016(E) 2.3.4` | `iso-8601-1-2019.sample.txt` lines 82, 150; `../../public/iso-wd-8601-1-2016.txt` lines 613-615 | `yes` |
| `SerializationProfileDeclared` | `basic and extended forms` | `ISO 8601-1:2019 3.1.3; 5.1; ISO/WD 8601-1:2016(E) 2.3.3; 2.3.4` | `iso-8601-1-2019.sample.txt` lines 82, 150; `../../public/iso-wd-8601-1-2016.txt` lines 605-615 | `yes` |

## Notes

- RFC 3339 and RFC 9557 serialization contracts in this file are already
  section-cited and are intentionally excluded from this worksheet.

## Serialization Coverage Audit

| Clause concept | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `2.3.3 basic format` | `covered` | `Iso8601BasicFormUsesCompactRepresentation` | Compact/basic form is explicit. |
| `2.3.4 extended format` | `covered` | `Iso8601ExtendedFormUsesSeparators` | Separator-bearing extended form is explicit. |
| `5.1 representation profile declaration` | `covered` | `SerializationProfileDeclared` | Serialization-family choice remains explicit. |
