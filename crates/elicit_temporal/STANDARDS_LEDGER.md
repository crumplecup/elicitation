# Temporal Standards Ledger

This ledger records the canonical standards sources for `elicit_temporal` and
the intended contract-mining order.

## Canonical standards

### ISO 8601-1:2019

- Title: *Date and time - Representations for information interchange - Part 1: Basic rules*
- Role: canonical interchange rules for calendar dates, ordinal dates, week
  dates, local times, UTC designators, numeric offsets, and combined
  date-time forms
- Coverage target: proposition-level mirror of the core syntax and structural
  validity rules
- Citation state: topic-level normative labels are embedded now; exact clause
  numbers can now be tightened incrementally against the preserved sample corpus
  in `standards/licensed/`, while truncated preview areas still rely on public
  cross-checks for topic sharpening

### ISO 8601-2:2019

- Title: *Date and time - Representations for information interchange - Part 2: Extensions*
- Role: extensions such as uncertain or approximate values, intervals, sets,
  seasons, and expanded temporal expressions
- Coverage target: second-wave contract expansion after the Part 1 interchange
  core is stable
- Citation state: normative topic labels plus public LOC EDTF Level 1/Level 2
  heading cross-checks are embedded in code comments now; exact ISO clause
  numbers can now be tightened incrementally against the preserved sample corpus
  in `standards/licensed/`, while truncated preview areas still rely on the
  public EDTF mirrors for topic sharpening

### CalConnect CC 18011:2018

- Title: *Date and time - Explicit representation*
- Role: public clause-addressable source text for explicit forms, grouped time
  scale units, explicit durations, and date-time formulas aligned to the ISO
  8601 family
- Coverage target: immediate mining source for extension contracts that do not
  need to wait for opaque browser shells or discarded temporary fetches
- Citation state: local XML source is preserved now and ready for clause-level
  extraction into contracts and trait-side intermediary descriptors

### CalConnect CC 18012:2018

- Title: *Date and time - General recurrence representation*
- Role: public clause-addressable source text for recurrence, selection rules,
  eligible time intervals, and repeat-rule evaluation
- Coverage target: staging source for recurrence-oriented contracts that will
  later compose cleanly with persistence and scheduling consumers
- Citation state: local XML source is preserved now and ready for clause-level
  extraction into recurrence descriptors and proof sidecars

### RFC 3339

- Title: *Date and Time on the Internet: Timestamps*
- Role: Internet profile of ISO 8601 for interoperable timestamps
- Coverage target: full-date, full-time, UTC relationship, secfrac, offset
  semantics, and generation guidance relevant to API and persistence seams
- Citation state: section-level citations available in code comments now

### RFC 9557

- Title: *Date and Time on the Internet: Timestamps with additional information*
- Role: IXDTF extensions for time-zone annotations and extra timestamp metadata
- Coverage target: bracketed zone annotations and criticality rules that matter
  for preserving zone identity across producers and persistence layers
- Citation state: section-level citations available in code comments now

## Citation fidelity model

Use these terms consistently across the temporal contract surface:

- `Normative source` means the governing standards text for the proposition or
  trait seam.
- `Informative cross-check` means a public heading or profile that mirrors the
  same concept and helps pin meaning while licensed ISO clause numbers remain
  unavailable in the repo.
- `Topic-level normative label` means the code names the narrow law topic
  honestly, but not yet the exact ISO clause or subclause.

Current fidelity boundary:

- RFC 3339 and RFC 9557 are section-cited directly in code comments.
- CalConnect CC 18011 and CC 18012 are now locally preserved in searchable XML
  and can be clause-cited directly when we intentionally mine those standards.
- ISO 8601-2 concepts now carry both normative topic labels and public LOC EDTF
  heading cross-checks where that public mirror exists.
- ISO 8601-1 and ISO 8601-2 now have preserved local sample-preview corpora
  under `standards/licensed/`.
- Exact ISO clause numbers may now be embedded where those saved previews expose
  the clause identifier with enough surrounding text to avoid guesswork.
- Topic labels and public cross-checks still remain necessary where the preview
  corpus truncates the normative body.

## Licensed-text extraction staging

This checklist decomposes the remaining clause-number extraction pass into
concrete module-sized slices. Check a module only when every ISO topic label in
that file has been tightened to the exact licensed-text clause or subclause.

### Current artifact inventory

- [x] `standards/public/loc-edtf-2019.html` is preserved as the public
  informative cross-check for EDTF headings already embedded in Part 2
  comments.
- [x] `standards/public/calconnect-cc-18011-2018.xml` is preserved as a public
  searchable clause-addressable source for explicit temporal extensions.
- [x] `standards/public/calconnect-cc-18012-2018.xml` is preserved as a public
  searchable clause-addressable source for recurrence semantics.
- [x] `standards/public/iso-8601-1-2019-iso-catalog.html` is preserved as the
  ISO catalog metadata page for product `70907`.
- [x] `standards/public/iso-8601-2-2019-iso-catalog.html` is preserved as the
  ISO catalog metadata page for product `70908`.
- [x] `standards/public/iso-8601-1-2019-amd1-2022-iso-catalog.html` is
  preserved, but it is only the ISO catalog or product page metadata for
  amendment `81801`, not the licensed amendment text.
- [x] `standards/public/iso-8601-1-2019-amd1-2022-iso-sample-shell.html` is
  preserved, but it is only a Vaadin bootstrap shell and does not expose
  searchable clause text.
- [x] Save usable licensed-text extracts for `ISO 8601-1:2019`,
  `ISO 8601-1:2019/Amd 1:2022`, and `ISO 8601-2:2019` under
  `standards/licensed/` before tightening any more ISO citations.
- [x] Preserve both PDF and searchable text forms for the current ISO preview
  corpus so clause extraction can proceed without repeated fetch-and-discard
  work.

### Acceptance criteria for a usable licensed-text extract

A saved artifact is usable for clause-tightening only when it satisfies all of
the following:

- searchable local text, OCR text, or HTML that exposes the actual clause or
  subclause numbering
- enough fidelity to distinguish nearby clauses without guessing from a heading
  summary alone
- a stable saved path under `standards/licensed/` so the extraction pass can
  proceed incrementally without repeated fetch-and-discard work

The preserved CalConnect XML artifacts already satisfy these criteria for their
own source families. The remaining acquisition gap is specific to official ISO
clause numbering breadth, not to local clause-bearing source preservation in
general.

## Post-acquisition surface gaps

The ISO sample corpus lets us audit the contract surface against the standards'
own clause family structure instead of guessing from memory.

- [x] Add a generic ISO 8601 offset date-time parse and format seam.
  The current trait surface exposes local date-time and profiled fixed-instant
  formats, but not the unprofiled ISO `date-time with UTC relationship` seam.
- [x] Add standalone decade and century descriptors, contracts, and trait
  methods.
  They now exist as first-class neutral descriptors, proof aggregates, and
  parser/formatter seams, and are also available through qualified-temporal and
  explicit-form value families.
- [x] Add dedicated branch-level descriptors and proof seams for explicit `date
  with shift` and `time of day with time shift`.
  They now exist as first-class descriptors and `ProvableFrom` exchanges.
  `date with shift` uses an explicit complete-date carrier
  (`calendar`/`ordinal`/`week`) rather than pretending the standard is
  calendar-date-only, and the shift branch now uses a CalConnect-specific
  explicit time-shift carrier rather than the narrower ISO numeric-offset seam.
- [x] Add explicit-only descriptors and proof seams for `timeE`, `dtE`, and
  `dtsE` rather than widening the generic ISO timestamp carriers.
  `ExplicitTimeOfDayDescriptor`, `ExplicitTimeShiftDescriptor`,
  `ExplicitDateTimeDescriptor`, and `ExplicitDateTimeWithShiftDescriptor` now
  carry CalConnect-specific reduced-precision time, bare-`Z` UTC, and
  second-granularity shift semantics without disturbing the narrower
  `LocalDateTimeDescriptor` / `OffsetDateTimeDescriptor` branch used by the
  ISO and RFC exchange seams.
- [x] Add explicit-only duration descriptors and proof seams for CalConnect
  §7 instead of widening the normalized ISO duration bag.
  `ExplicitDurationDescriptor` now preserves sign, ordered units,
  representation family, fractional lowest-order unit semantics, and
  exact/context-dependent/speculative classification. The CalConnect trait seam
  exposes dedicated parse/format exchanges for this family, and date-time
  formulas now consume the explicit-duration carrier directly.
- [x] Add explicit-only interval descriptors and proof seams for the remaining
  CalConnect §6 interval semantics.
  `ExplicitTimeIntervalDescriptor`, the dedicated CalConnect endpoint family,
  explicit interval proof aggregates, explicit interval proof branches, and
  parse/format exchange seams now carry duration substitution, omitted
  higher-order end-component inheritance, and time-shift propagation directly
  rather than collapsing them into the narrower ISO interval family.
- [x] Thread the CalConnect explicit-interval family through recurring
  interval-with-repeat-rule seams.
  `CC 18012:2018 §6.4` complete recurring representations now keep the ISO
  complete versus CalConnect explicit interval family visible at the interface
  boundary, together with the embedded interval proof branches.
- [x] Tighten exact ISO clause numbers in module worksheets and doc comments for
  the contracts already covered by the preserved preview corpus.

### Extraction workflow artifacts

- `standards/licensed/CLAUSE_MAP_TEMPLATE.md` is the canonical clause-to-contract
  extraction template.
- `standards/licensed/module-worksheets/INDEX.md` tracks staged worksheet
  coverage across contract and trait surfaces.
- `standards/licensed/module-worksheets/iso_8601.rs.md` is the first module
  worksheet and defines completion at the contract-symbol level.
- Future modules should receive the same worksheet treatment before any module
  is checked off below.

## Public CalConnect coverage status

- [x] `src/contracts/calconnect.rs` now represents CC 18011 §4.3 explicit
  forms, CC 18011 §5 grouped time scale units, CC 18011 §8 date-time formulas,
  CC 18012 §5 selection rules, and CC 18012 §6.3-§6.4 repeat-rule structure.
- [x] `src/contracts/proof_composition.rs` now composes these families through
  `QualifiedTemporalValueValid`, `ExplicitTemporalFormValid`,
  `GroupedTimeScaleUnitValid`, `DateTimeFormulaValid`,
  `DateTimeFormulaEvaluationSemanticsValid`, `SelectionExpressionValid`,
  `RepeatRuleValid`, and `RecurringIntervalWithRepeatRuleValid`.
- [x] `src/types.rs` now exposes neutral descriptors and result aliases for the
  same CalConnect families.
- [x] `src/traits/calconnect.rs` now defines the object-safe exchange seam for
  parsing, formatting, and evaluating those families.

### ISO 8601-1 staging

- [x] `src/contracts/iso_8601.rs`
- [x] `src/contracts/interval.rs`
- [x] `src/contracts/instant.rs`
- [x] `src/contracts/serialization.rs`
- [x] `src/contracts/precision.rs`
- [x] `src/contracts/conversion.rs`
- [x] `src/contracts/proof_composition.rs` for Part 1 evidence bundles
- [x] `src/traits/parse.rs` for Part 1 parsing seams
- [x] `src/traits/format.rs` for Part 1 formatting seams
- [x] `src/traits/interval.rs` for Part 1 interval seams

### ISO 8601-2 staging

- [x] `src/contracts/extended.rs`
- [x] `src/contracts/precision.rs` for reduced-precision and rounding semantics
- [x] `src/contracts/conversion.rs` for lossy-conversion authority semantics
- [x] `src/contracts/proof_composition.rs` for Part 2 evidence bundles
- [x] `src/traits/conversion.rs` for reduced-precision exchange seams
- [x] `src/traits/parse.rs` for Part 2 parsing seams
- [x] `src/traits/format.rs` for Part 2 formatting seams

## Extraction order

1. ISO 8601-1 core forms
2. RFC 3339 Internet profile
3. RFC 9557 IXDTF zone annotations
4. CalConnect CC 18011 explicit temporal extensions
5. CalConnect CC 18012 recurrence and selection rules
6. ISO 8601-2 extended temporal forms, then tighten to exact ISO clauses once
   official searchable text lands

## Contract-mining rules

Each mined contract should:

- be named as a crisp proposition, not a procedure
- cite the governing standard in the doc comment
- stand alone as a reusable proof token
- compose into higher-order propositions through explicit evidence bundles
- avoid backend-specific terms unless the standard itself requires them

## Initial aggregate targets

The first aggregate propositions in `proof_composition.rs` are:

- `CalendarDateValid`
- `OrdinalDateValid`
- `WeekDateValid`
- `LocalTimeValid`
- `UtcOffsetValid`
- `LocalDateTimeValid`
- `OffsetDateTimeValid`
- `Rfc3339TimestampValid`
- `IxdtfTimestampValid`

## Seed extraction entries

| Source | Clause / section | Paraphrase | Classification | Likely representation | Notes |
| --- | --- | --- | --- | --- | --- |
| ISO 8601-1:2019 | calendar date complete representation | Calendar dates carry year, month, and day. | structural | proposition type | Tighten to exact clause on licensed-text pass. |
| ISO 8601-1:2019 | ordinal date complete representation | Ordinal dates carry year and day-of-year. | structural | proposition type | Shared across parsing and persistence. |
| ISO 8601-1:2019 | week date complete representation | Week dates carry week-year, week number, and weekday. | structural | proposition type | Needed before week-date conversions. |
| ISO 8601-1:2019 | time-of-day component ranges | Hours, minutes, and seconds stay within legal ranges. | mathematical | proposition type, evidence field | Verifier-friendly arithmetic. |
| ISO 8601-1:2019 | UTC offset representation | Numeric offsets carry sign, hour, and minute. | structural | proposition type | Basis for offset-neutral consumers. |
| ISO 8601-1:2019 | combined date-time representation | Date and time are joined with the time designator. | structural | proposition type | Core to local date-time validity. |
| RFC 3339 | §4.3 | `-00:00` denotes unknown local offset convention. | authoritative / interpretive | proposition type, evidence field | Separates known-offset law from fixed-instant law. |
| RFC 3339 | §4.4 | Unqualified local time is forbidden. | structural | proposition type | Important for persistence and API boundaries. |
| RFC 3339 | §5.6 | RFC 3339 timestamps use `full-date` and `full-time`. | structural | proposition type, evidence field | Internet-profile narrowing of ISO 8601. |
| RFC 3339 | §5.6 | Fractional seconds use `.` as separator. | structural | proposition type | Profile-specific serialization law. |
| RFC 3339 | §5.1, §5.5, §5.6 | Lexical timestamp ordering depends on uniform UTC-relationship encoding, uniform fractional-second digit counts, and the profile's mandatory punctuation shape. | structural / interpretive | proposition type, evidence field | This is a comparison-law bundle rather than a single timestamp validity rule. |
| RFC 3339 | §5.3, §5.6, §5.7 | Fractional seconds are the only rarely used option; generators should prefer uppercase `T` and `Z`; inserted leap-second timestamps should not be generated before announcement. | authoritative / generation guidance | proposition type, evidence field | High-value producer-side law for API and persistence emitters. |
| RFC 9557 | §3.1 | Bracketed zone annotations carry named time-zone identity. | authoritative / interpretive | proposition type, evidence field | Zone identity is not reducible to offset alone. |
| RFC 9557 | §3.1 | Named IANA time zones are interpreted using the TZDB rules current at interpretation time; unknown names under revision skew are treated as inconsistencies. | authoritative / interpretive | proposition type, evidence field | Preserve rule-evolution semantics rather than freezing a zone name to one historic revision. |
| RFC 9557 | §3.1 | Offset time-zone annotations repeat the timestamp offset; their use is strongly discouraged and they must not be synthesized by copying the timestamp offset only to satisfy a suffix requirement. | authoritative / interpretive | proposition type, evidence field | Distinguish compatibility syntax from durable named-zone identity. |
| RFC 9557 | §3.2 | Critical annotations lead with `!`. | structural | proposition type | Useful for preserving strict parsing semantics. |
| RFC 9557 | §3.3 | IXDTF extends an RFC 3339 timestamp with additional information. | structural | proposition type, evidence field | Governs suffix layering. |
| RFC 9557 | §3.3, §4.1 | Additional-information tags use bracketed `key=value` form with one or more hyphen-delimited value items. | structural | proposition type, evidence field | Canonical tag syntax for suffix-level contracts. |
| RFC 9557 | §3.3 | Suffix tags are optional for generators and elective for recipients unless marked critical. | authoritative / interpretive | proposition type, evidence field | Separates generation freedom from recipient obligations. |
| RFC 9557 | §3.3 | Critical suffix tags require processing or explicit error handling, while experimental underscore-prefixed keys are restricted to controlled environments and rejected by unconfigured recipients. | authoritative / interpretive | proposition type, evidence field | Captures the main IXDTF recipient-side safety law for additional information. |
| RFC 9557 | §5 | The `u-ca` suffix key declares the preferred calendar for presentation using a Unicode calendar identifier. | structural | proposition type, evidence field | Keep this separate from fixed-instant semantics. |
| BCP 175 / IANA TZDB | named zone semantics | A named zone identifies civil-time rules, not just an offset. | authoritative / interpretive | proposition type, trait postcondition | Relevant once zoned traits are introduced. |
| ISO 8601-2:2019; RFC 3339 | reduced precision and ordering | Backend adapters should prove shared conversion semantics once, then separately prove either preserved subsecond precision or explicit lossy authority for truncation. | structural / interpretive | aggregate type, evidence bundle | This is an interface-layer composition law rather than a new wire syntax rule. |
| ISO 8601-1:2019; RFC 3339; RFC 9557 | interface exchange seam | Temporal parser, formatter, zone, conversion, and interval traits should exchange neutral descriptors together with explicit proof sidecars instead of backend-native values or ad hoc post-hoc validation. | structural / architectural | trait signature, descriptor type | Initial object-safe trait surface now exists; extend it incrementally as the standards mirror deepens. |
| ISO 8601-1:2019 | duration representation | Durations begin with `P`; time units follow `T`; week form is exclusive. | structural | proposition type, evidence field | Seed for neutral duration law. |
| ISO 8601-1:2019 | time interval representation | Intervals use `/` and declare an endpoint-or-duration form. | structural | proposition type, evidence field | Needed before persistence traits can speak interval law. |
| ISO 8601-1:2019 | interval semantics | Interval ordering is from earlier to later; durations are non-negative. | mathematical | proposition type, evidence field | Suitable for verifier-oriented arithmetic later. |
| ISO 8601-1:2019 | recurring interval representation | Recurring intervals begin with `R`, have positive bounded counts, and carry an interval component. | structural | proposition type, evidence field | Natural bridge to schedulers and persistence backends. |
| CalConnect CC 18011:2018 | §4.3 `Explicit forms` | Explicit forms use designator symbols to delimit time scale components and admit zero-valued omission beyond fixed-length ISO 8601 forms. | structural | proposition type, evidence field, descriptor, trait seam | Represented in `src/contracts/calconnect.rs`, `src/contracts/proof_composition.rs`, `src/types.rs`, and `src/traits/calconnect.rs`. |
| CalConnect CC 18011:2018 | §5 `Grouped time scale units` | Grouped time scale units package one or more durational units into reusable grouped units. | structural | proposition type, evidence field, descriptor, trait seam | Represented as a first-class intermediary contract before backend-native duration arithmetic. |
| CalConnect CC 18011:2018 | §8 `Evaluation of date and time with duration` | Date-time formulas define evaluation of a date or time expression with explicit duration mechanics. | mathematical | proposition type, evidence field, descriptor, trait seam | Represented, including a separate evaluation-semantics aggregate for proof-sidecar exchange. |
| CalConnect CC 18012:2018 | §5.2 `Selection rules` | Selection rules restrict time scale component values over eligible time intervals. | structural | proposition type, evidence field, descriptor, trait seam | Represented as a standalone contract family for recurrence consumers and persistence query layers. |
| CalConnect CC 18012:2018 | §6.3 `Repeat rule`; §6.4 `Complete representation` | Recurring time intervals with repeat rules combine repeating cycles, eligible intervals, and selection criteria. | structural | proposition type, evidence field, descriptor, trait seam | Represented, including a complete recurring-interval-plus-repeat-rule exchange seam. |
| ISO 8601-1:2019/Amd 1:2022 | Amendment 1: Technical corrections | End-of-day midnight is representable with hour `24`, but only as the terminal `24:00:00` form. | structural | proposition type, evidence field | Reconciliation pass aligned the local-time contract surface to this amendment-compatible end-of-day law; tighten to clause-level citation on the licensed-text pass. |
| ISO 8601-2:2019 | uncertain and approximate expressions | Extended expressions may explicitly qualify uncertainty, approximation, and their combination. | structural | proposition type, evidence field | Keep the neutral law broader than any one parser profile. |
| ISO 8601-2:2019 | qualification scope | A qualification may apply to a whole expression or one of its components. | structural | proposition type, evidence field | Important once descriptors model component-level uncertainty. |
| ISO 8601-2:2019 | open and unknown interval boundaries | Interval boundaries may be explicitly open or unknown. | authoritative / interpretive | proposition type, evidence field | Needed before interval consumers can distinguish absence from explicit openness. |
| ISO 8601-2:2019 | seasons and seasonal temporal expressions | A seasonal temporal expression uses a year-and-season form with the season code occupying the month position of a year-month-shaped representation. | structural | proposition type, evidence field | Representation shape inferred conservatively from the public LOC EDTF profile while citations remain anchored at the ISO seasonal extension. |
| ISO 8601-2:2019 | named seasonal temporal expressions | Season codes declare named seasonal semantics and their scope, distinguishing location-independent seasons from hemisphere-qualified seasonal expressions. | authoritative / interpretive | proposition type, evidence field | Public LOC EDTF Level 1 `Seasons` and Level 2 `Sub-year groupings` headings are embedded in code comments now; exact ISO clause numbers still await licensed text. |
| ISO 8601-2:2019 | unspecified digits and unspecified components | Uppercase `X` denotes unspecified digits or whole calendar components while preserving the declared expression precision. | structural / interpretive | proposition type, evidence field | Public LOC EDTF Level 1 `Unspecified digit(s) from the right` and Level 2 `Unspecified Digit` headings are embedded in code comments now; exact ISO clause numbers still await licensed text. |
| ISO 8601-2:2019 | advanced unspecified-digit placement | Level 1 constrains unspecified digits to rightmost positions, while Level 2 allows `X` anywhere within a component. | structural | proposition type, evidence field | Public LOC EDTF Level 1 `Unspecified digit(s) from the right` and Level 2 `Unspecified Digit` headings are embedded in code comments now; exact ISO clause numbers still await licensed text. |
| ISO 8601-2:2019 | temporal-set delimiters and membership semantics | Square brackets declare single-choice semantics, while curly braces declare inclusive all-members semantics. | structural / interpretive | proposition type | Public LOC EDTF Level 2 `Set representation` is embedded in code comments now; exact ISO clause numbers still await licensed text. |
| ISO 8601-2:2019 | temporal-set range and precision refinements | Temporal-set ranges use inclusive `..`, open-ended boundary `..`, forbid internal whitespace, and require shared precision across range neighborhoods. | structural | proposition type, evidence field | Public LOC EDTF Level 2 `Set representation` is embedded in code comments now; exact ISO clause numbers still await licensed text. |
| ISO 8601-2:2019 | temporal sets | Extended expressions may describe sets or alternatives rather than one fixed temporal value. | structural | proposition type, evidence field | Natural precursor to set-valued descriptors and query interfaces. |

## Completed mining sweep checklist

This checklist records the standards-mining sweep already completed across the
current contract catalog. It does not mean all standards citations have been
tightened to licensed ISO clause numbers; remaining citation debt is tracked
separately below.

- [x] RFC 9557 calendar identity annotations
- [x] RFC 9557 richer zone-identity annotations beyond the current bracketed-form seed
- [x] RFC 9557 additional-information field semantics as first-class contracts
- [x] ISO 8601-2 seasons and named seasonal temporal expressions
- [x] ISO 8601-2 masked precision and unspecified-component semantics
- [x] ISO 8601-2 richer set refinements beyond the current alternative-members seed
- [x] ISO 8601-1:2019/Amd 1:2022 reconciliation pass against existing Part 1 contracts
- [x] RFC 3339 remaining profile clauses, especially ordering and generation guidance
- [x] RFC and public-source citation tightening pass across the mined surface

## Contract-surface gap audit

This checklist is the current code-vs-local-corpus audit. Unlike the staging
checklists above, these items track actual representation gaps or seam
asymmetries, not just citation-tightening debt.

### Confirmed descriptor and proposition gaps

- [x] Add a first-class reduced-precision calendar value family for plain
  `year` and `year-month` expressions. Public LOC EDTF interval and set
  examples rely on these forms, but `TemporalValueDescriptor` currently only
  carries complete calendar dates, ordinal dates, week dates, date-times,
  seasonal expressions, and unspecified-component expressions.
- [x] Add a first-class expanded-year family for ISO 8601-2 year forms:
  letter-prefixed years, negative years as an explicit lexical law, exponential
  years, and significant-digit years. The public local corpus exposes these
  headings directly, but no current contract, descriptor, or trait seam names
  them.
- [x] Add sub-year grouping contracts and descriptors beyond the current
  seasonal set: quarter, quadrimester, and semestral forms exposed in the
  public `Sub-year groupings` heading are not yet represented.
- [x] Add explicit qualification-placement laws beyond the coarse
  `QualificationScopeDeclared` token. The public `Qualification` heading
  distinguishes immediate-right group qualification from immediate-left
  component-only qualification, and that lexical law is not yet mirrored as its
  own proposition family.

### Confirmed descriptor expressivity gaps

- [x] Allow interval endpoints to carry qualified temporal values, not only
  bare `TemporalValueDescriptor` values. `TimeIntervalEndpoint::Value(...)`
  currently cannot represent a plain approximate or uncertain calendar date
  boundary such as the public Level 2 interval examples.
- [x] Allow temporal-set members and range endpoints to carry the same richer
  reduced-precision and qualified value forms required by the standard examples.
  `TemporalSetMemberDescriptor` currently inherits the same limits as
  `TemporalValueDescriptor`.

### Confirmed trait-surface asymmetries

- [x] Add formatter seam parity for already-modeled core Part 1 values:
  ordinal dates, week dates, local times, UTC offsets, and local date-times
  now have matching formatter methods in basic and extended form, with local
  date-time emission continuing to require the explicit
  `LocalDateTimeDoesNotIdentifyFixedInstant` sidecar at the seam.

### Source-family alignment gaps

- [x] Mirror ISO 8601-2 qualification exchange seams onto the neutral parser
  and formatter surfaces. `parse_qualified_temporal_value` and
  `format_qualified_temporal_value` no longer live only behind the
  `TemporalCalConnectFactory` doorway.
- [x] Mirror grouped time scale units onto ISO-neutral parser and formatter
  seams. `TemporalParser` and `TemporalFormatter` now expose the grouped-unit
  family directly, while `TemporalCalConnectFactory` remains as the
  section-cited public cross-check seam.
- [x] Mirror date and time arithmetic onto ISO-neutral parser and formatter
  seams. `TemporalParser` and `TemporalFormatter` now expose neutral
  date-time-formula exchanges directly, while
  `TemporalCalConnectFactory::evaluate_date_time_formula` remains the
  operational extension seam.

## Next extraction focus

The next contract-mining pass should deepen:

- richer IXDTF consumer-side laws around suffix-key registries, critical-tag handling, and profile-specific downgrade paths
- richer interval and conversion trait coverage for persistence-oriented consumers

## Remaining citation fidelity gaps

This is citation debt, not contract-surface debt. The licensed-text extraction
item above remains the sole open blocker: the currently saved ISO artifacts
under `standards/public/` are metadata or bootstrap shells, not clause text.

- [x] Complete the ISO 8601-1 staging checklist above.
- [x] Complete the ISO 8601-2 staging checklist above.

## Post-review fidelity remediation checklist

Track the remaining contract-surface fidelity gaps here, but execute them from
the ordered checklist in [ELICIT_TEMPORAL_PLAN.md](../../ELICIT_TEMPORAL_PLAN.md).

- [x] Reconcile citation-state claims with the actual standards ledger
- [x] Promote mined ISO 8601-2 forms into neutral exchange descriptors
- [x] Add parser and formatter seams for the new ISO 8601-2 descriptors
- [x] Decompress IXDTF exchange results into explicit proof branches
- [x] Add lawful zone-resolution seams for ambiguous and gap local times
- [x] Replace stringly serialization metadata with typed annotation descriptors
- [x] Expand conversion and interval seams to match the mined lawbook
- [x] Run a final `ProvableFrom` closure pass over every new exchange
