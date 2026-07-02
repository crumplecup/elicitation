# Public Temporal Standards Artifacts

This directory preserves the public-source side of the `elicit_temporal`
standards corpus.

These artifacts fall into three buckets:

- public source text we can mine directly for clauses and structure
- public profile or heading cross-check material
- public catalog metadata that pins exact document identity and amendment state

They are useful for contract design and citation discipline, but they do not
erase the remaining need for official searchable `ISO 8601-1` and
`ISO 8601-2` text when a Rust contract claims an exact ISO clause number.

## Public source text

- `calconnect-cc-18011-2018.xml`
  - Origin: [https://standards.calconnect.org/cc/cc-18011-2018.xml](https://standards.calconnect.org/cc/cc-18011-2018.xml)
  - Scope: explicit representations, grouped time scale units, explicit
    duration, date-time formulas
- `calconnect-cc-18012-2018.xml`
  - Origin: [https://standards.calconnect.org/cc/cc-18012-2018.xml](https://standards.calconnect.org/cc/cc-18012-2018.xml)
  - Scope: recurrence, selection rules, eligible time intervals, repeat rules

## Public profile and cross-check text

- `loc-edtf-2019.html`
  - Origin: [https://www.loc.gov/standards/datetime/edtf.html](https://www.loc.gov/standards/datetime/edtf.html)
  - Scope: EDTF Level 0 to Level 2 headings used as public cross-checks for
    `ISO 8601-2`-aligned concepts

## Public catalog metadata

- `iso-8601-1-2019-iso-catalog.html`
  - Origin: [https://www.iso.org/standard/70907.html](https://www.iso.org/standard/70907.html)
  - Scope: title, product identifier, review state, and amendment linkage for
    `ISO 8601-1:2019`
- `iso-8601-2-2019-iso-catalog.html`
  - Origin: [https://www.iso.org/standard/70908.html](https://www.iso.org/standard/70908.html)
  - Scope: title, product identifier, review state, and amendment linkage for
    `ISO 8601-2:2019`
- `iso-8601-1-2019-amd1-2022-iso-catalog.html`
  - Origin: [https://www.iso.org/standard/81801.html](https://www.iso.org/standard/81801.html)
  - Scope: amendment identity and metadata for `ISO 8601-1:2019/Amd 1:2022`
- `iso-8601-1-2019-amd1-2022-iso-sample-shell.html`
  - Origin: `https://www.iso.org/obp/ui/#iso:std:iso:8601:-1:ed-1:v1:en`
  - Scope: preserved evidence that the public ISO sample route currently lands
    in a browser shell without searchable clause text
