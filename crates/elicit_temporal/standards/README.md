# Temporal Standards Corpus

This directory is the canonical local standards corpus for
`crates/elicit_temporal`.

Its purpose is operational, not decorative:

- exact contract citations must be grounded in stable local source material
- public cross-check artifacts may support meaning, but they are not normative
- licensed ISO clause-tightening work must proceed from local copies stored
  here, not from ephemeral browser tabs or discarded `/tmp` files

## Layout

- `public/`
  - public source-text, profile, or catalog artifacts that can be preserved in
    the repo without pretending they are the final ISO clause authority
- `licensed/`
  - the local working corpus for licensed ISO text or faithful extracted text
    used to tighten exact clause and subclause references
  - `CLAUSE_MAP_TEMPLATE.md` for clause-to-contract extraction
  - `module-worksheets/` for module-specific citation closure
  - `module-worksheets/INDEX.md` for staged coverage tracking

## Current public artifacts

- `public/iso-wd-8601-1-2016.txt`
  - clause-addressable text extracted from the staged public `ISO/WD 8601-1`
    working-draft PDF
- `public/iso-wd-8601-2-2016.txt`
  - clause-addressable text extracted from the staged public `ISO/WD 8601-2`
    working-draft PDF
- `public/rfc3339.txt`
  - public RFC source text for the base Internet timestamp profile
- `public/rfc9557.txt`
  - public RFC source text for IXDTF suffix, time-zone, and calendar
    annotation semantics
- `public/loc-edtf-2019.html`
  - Library of Congress EDTF specification page used as an informative
    cross-check for ISO 8601-2-aligned heading names
- `public/calconnect-cc-18011-2018.xml`
  - public machine-readable CalConnect source text for explicit temporal
    representations, grouped units, explicit durations, and date-time formulas
- `public/calconnect-cc-18012-2018.xml`
  - public machine-readable CalConnect source text for recurrence and
    selection-rule representations
- `public/iso-8601-1-2019-iso-catalog.html`
  - ISO catalog metadata page for `ISO 8601-1:2019`, including title, product
    identifier `70907`, review state, and amendment links
- `public/iso-8601-2-2019-iso-catalog.html`
  - ISO catalog metadata page for `ISO 8601-2:2019`, including title, product
    identifier `70908`, review state, and amendment links
- `public/iso-8601-1-2019-amd1-2022-iso-catalog.html`
  - ISO catalog or product metadata page for amendment `81801`
- `public/iso-8601-1-2019-amd1-2022-iso-sample-shell.html`
  - ISO sample-shell HTML bootstrap with no usable clause text

## Acquisition state

The public-source acquisition pass is no longer blocked. The repo now carries:

- public clause-addressable working-draft text for `ISO/WD 8601-1:2016(E)`
- public clause-addressable working-draft text for `ISO/WD 8601-2:2016(E)`
- a public clause-addressable text for the RFC 3339 Internet timestamp profile
- a public clause-addressable text for the RFC 9557 IXDTF extension profile
- a public clause-addressable text for CalConnect explicit temporal extensions
- a public clause-addressable text for CalConnect recurrence semantics
- a public EDTF profile for ISO 8601-2-aligned headings
- ISO product metadata pages that pin exact document identities and amendments

## Remaining ISO-specific gap

The remaining gap is no longer clause-addressable text in the abstract. The
repo now has open working-draft text that is sufficient for worksheet-driven
coverage audits and source-anchored gap analysis.

What still remains unavailable locally is the final published clause authority
for the exact `ISO 8601-1:2019`, `ISO 8601-1:2019/Amd 1:2022`, and
`ISO 8601-2:2019` text. The remaining local corpus target is:

- `licensed/iso-8601-1-2019.*`
- `licensed/iso-8601-1-2019-amd1-2022.*`
- `licensed/iso-8601-2-2019.*`

The exact extension is less important than the content. A file is usable only
if it exposes searchable clause or subclause numbering with enough fidelity to
distinguish neighboring provisions without guesswork.

The CalConnect XML artifacts already satisfy this standard for their own source
families. The staged working-draft ISO texts are also sufficient for coverage
audits, worksheet checklists, and next-best open-source line anchors. Neither
substitutes for the exact governing published ISO clause numbers where a
contract is explicitly claimed as a final `ISO 8601-1` or `ISO 8601-2` mirror.

## Extraction workflow

Use `licensed/CLAUSE_MAP_TEMPLATE.md` to record the authoritative clause map
before editing Rust doc comments. Then complete the corresponding worksheet
under `licensed/module-worksheets/`, update
`licensed/module-worksheets/INDEX.md`, and only afterwards mark the module
complete in `STANDARDS_LEDGER.md`.
