# ELICIT_TEMPORAL_PLAN.md

## Goal

Introduce `elicit_temporal` as a branch-level interface crate that owns the
shared contract language, proof propositions, evidence bundles, descriptor
types, and object-safe trait seams for temporal law across the workspace.

`elicit_temporal` sits below the current temporal leaf crates
(`elicit_time`, `elicit_chrono`, `elicit_jiff`) and below downstream consumer
branches and leaves (`elicit_db`, `elicit_sqlx`, `elicit_redb`,
`elicit_surrealdb`, `elicit_polars`, `elicit_bevy`, and others). Its job is
to transcribe recognized temporal standards into a proof-carrying accord that
all of those crates can implement or consume without inventing their own time
semantics.

The architectural standard is strict:

- contract language first
- third-party standards mined deeply and cited precisely
- trait methods defined only after the lawbook exists
- every lawful exchange expressed through `ProvableFrom`
- proof sidecars (`Established<P>`) used as the one true exchange pattern

The long-term payoff is correctness by construction: mathematically verified
temporal invariants are established once, carried forward as proof sidecars,
and reused across long pipelines without repeated post-hoc validation.

Initial landed runtime slice:

- neutral temporal descriptors, including composite IXDTF timestamp exchange
  types
- object-safe parser, formatter, zone, conversion, interval, and reporter
  traits
- named result aliases for the proof-carrying exchange shapes that would
  otherwise violate the workspace's type-complexity lint policy

## Current Closure Checklist

The current remaining work is narrow enough to close systematically rather than
continuing with open-ended audits.

- [x] Close the CalConnect CC 18011 explicit-interval surface in code.
  Dedicated explicit-interval contracts, descriptors, proof aggregates,
  proof branches, and parser/formatter seams now exist for duration
  substitution, trailing higher-order component inheritance, and
  leading-shift propagation.
- [x] Thread the explicit-interval carrier through the CalConnect repeat-rule
  recurrence seam without collapsing the embedded interval family.
  `CC 18012:2018 §6.4` complete recurring representations now preserve the
  choice between an ISO complete interval and a CalConnect explicit interval,
  and the exchange result carries the embedded interval proof branch family.
- [x] Extend proof-composition worksheet concordance for the CalConnect
  interval and recurrence bundles.
  `contracts_proof_composition.rs.md` now records the explicit temporal,
  explicit-duration, explicit-interval, selection, repeat-rule, and recurring
  interval-with-repeat-rule evidence families explicitly.
- [x] Add CalConnect trait worksheet concordance for the new interval and
  recurrence seams.
  The staged worksheet set now includes `traits_calconnect.rs.md`, tying the
  public doorway methods to their exact ISO 8601-2 and CalConnect clauses.
- [x] Tighten exact ISO clause citations where the preserved local preview
  corpus exposes authoritative numbering without guesswork.
- [ ] Re-walk the staged worksheets and only mark a module complete after the
  contract symbols, proof composition, trait seams, and doc comments all agree.

## Trait Connectivity Checklist

The contract surface is no longer the primary uncertainty. The remaining
architectural question is trait connectivity: does every meaningful contract
actually cross a trait seam as a proof sidecar, or are some contracts stranded
inside `proof_composition.rs` as dormant evidence types?

This checklist closes that gap methodically.

### Connectivity invariants

Every trait seam in `elicit_temporal` should satisfy all of the following:

- [ ] Leaf parse or construct methods return neutral descriptors plus explicit
  proof sidecars for every semantic branch established at that seam.
- [ ] Non-leaf methods require proof sidecars for every semantic precondition
  they rely on.
- [ ] Non-leaf methods return fresh proof sidecars for every semantic
  postcondition they establish.
- [ ] Every returned postcondition has an explicit `ProvableFrom` derivation
  path from a named evidence bundle.
- [ ] No method collapses a meaningful branch family into a single coarse
  `Established<...Valid>` token when downstream consumers may lawfully depend
  on the finer distinction.
- [ ] Every evidence bundle in `src/contracts/proof_composition.rs` is in one
  of three states only:
  - directly produced by a leaf seam
  - required or re-issued by a higher-order seam
  - explicitly staged for a future seam in this checklist
- [ ] No evidence bundle is left architecturally orphaned.

### Already-strong seams to preserve

These seams already reflect the desired proof-sidecar grammar and should be
treated as the reference shape for weaker families.

- [x] Local date-time parse and format keep local non-fixed-instant semantics
  explicit.
- [x] Offset date-time parse and format keep fixed-instant semantics explicit.
- [x] IXDTF parse and format keep suffix semantics split into named proof
  branches instead of collapsing them into one aggregate token.
- [x] ISO duration parse and format keep representation-family semantics split
  into proof branches.
- [x] ISO recurring interval parse and format keep complete versus
  other-than-complete representation semantics split into proof branches.
- [x] ISO interval parse and format keep extended-boundary, inherited
  end-component, inherited-zone, and substitution semantics explicit.
- [x] CalConnect explicit interval parse and format keep duration
  substitution, end-component inheritance, and shift propagation explicit.
- [x] Temporal set parse and format keep base expression validity separate
  from range semantics validity.
- [x] Date-time formula parse and format keep structural validity separate
  from evaluation semantics validity.
- [x] Conversion traits require argument-side proofs and emit result-side
  conversion proofs.

### Family-by-family orphan audit

For each family below, completion means:

1. The trait seam returns or requires the needed sidecars.
2. The result alias names the exchange shape explicitly.
3. The proof bundle has a visible `ProvableFrom` path.
4. The corresponding worksheet and doc comments agree with the final shape.

#### Priority A: flattened high-value families already modeled in proof composition

- [x] Qualified temporal value.
  Closed: `ParsedQualifiedTemporalValueResult` and
  `FormattedQualifiedTemporalValueResult` now carry the qualified-value proof,
  the qualification-expression proof, and the
  `QualificationPlacementEvidence` branch explicitly.

- [x] Explicit temporal form.
  Closed: `ParsedExplicitTemporalFormResult` and
  `FormattedExplicitTemporalFormResult` now carry explicit sidecars for
  designator use, zero-omission authority, lowest-denoted-component
  precision, and UTC-relationship syntax.

- [x] Explicit duration.
  Closed: `ParsedExplicitDurationResult` and
  `FormattedExplicitDurationResult` now carry unit-designator usage,
  representation family, negative-sign authority,
  fractional-lowest-order-unit authority, and exactness-family semantics as
  explicit sidecars.

- [x] Grouped time scale unit.
  Closed: `ParsedGroupedTimeScaleUnitResult` and
  `FormattedGroupedTimeScaleUnitResult` now carry delimiter syntax,
  non-empty unit carriage, continuity, coefficient declaration,
  lower-order bounds, explicit-time-shift authority, truncation semantics,
  and interval-conversion semantics as explicit sidecars.

- [x] Selection expression.
  Closed: `ParsedSelectionExpressionResult` and
  `FormattedSelectionExpressionResult` now carry delimiter, vocabulary,
  component-rule, nesting, single-instance, positional, and
  duration-window semantics as explicit sidecars.

- [x] Repeat rule.
  Closed: `ParsedRepeatRuleResult` and `FormattedRepeatRuleResult` now carry
  frequency, eligible-interval, embedded-selection, and
  initial-start inheritance semantics as explicit sidecars.

#### Priority B: ISO 8601-2 extension families that may still be too coarse

- [x] Seasonal temporal expression.
  Closed: `ParsedSeasonalTemporalExpressionResult` and
  `FormattedSeasonalTemporalExpressionResult` now carry year-and-season form,
  month-slot encoding, named-season declaration, and season-scope
  declaration as explicit sidecars.

- [x] Sub-year grouping expression.
  Closed: `ParsedSubYearGroupingExpressionResult` and
  `FormattedSubYearGroupingExpressionResult` now carry year-and-grouping
  form, month-slot encoding, and the `SubYearGroupingKindEvidence` branch
  explicitly across the seam.

- [x] Unspecified-component expression.
  Closed: `ParsedUnspecifiedComponentExpressionResult` and
  `FormattedUnspecifiedComponentExpressionResult` now carry placeholder
  syntax, unknown-value semantics, Level 1 right-tail masking, and Level 2
  internal-component masking as explicit sidecars.

#### Priority C: higher-order composition seams

- [x] Date-time formula evaluation result provenance.
  Closed: `EvaluatedDateTimeFormulaResult` now carries both
  `Established<ExplicitTemporalFormValid>` and a dedicated
  `Established<DateTimeFormulaEvaluationResultValid>` postcondition minted
  from formula validity, declared evaluation semantics, and the produced
  explicit-form proof.

- [x] Recurring interval with repeat rule.
  Closed: `ParsedRecurringIntervalWithRepeatRuleResult` and
  `FormattedRecurringIntervalWithRepeatRuleResult` now preserve both embedded
  law families: the interval-family proof branch and the attached repeat-rule
  sidecars for frequency, eligible intervals, embedded selection, and
  initial-start inheritance.

- [x] Zone-resolution and named-zone higher abstractions.
  Closed: `TemporalZoneFactory` now separates generic named-zone identity and
  fixed-instant attachment evidence from IXDTF-specific suffix evidence,
  establishes local-to-zone resolution authority through its own explicit seam,
  and re-issues attachment, consistency, authority, and ambiguity-versus-gap
  sidecars across the zone-resolution result family.

### Audit procedure per checklist item

Apply this exact sequence to each family before checking it off:

- [ ] Confirm the standard clauses and worksheet entries for the family.
- [ ] Inspect descriptor shape in `src/types.rs`.
- [ ] Inspect evidence bundle shape in `src/contracts/proof_composition.rs`.
- [ ] Inspect current trait parse, format, conversion, interval, zone, or
  CalConnect seam.
- [ ] Decide which semantics must cross the seam explicitly and which can
  remain internal.
- [ ] Add or refine result aliases so the exchange shape is named and stable.
- [ ] Thread the argument-side and result-side proof sidecars through the trait
  signatures.
- [ ] Update worksheet notes and doc comments to match the final exchange
  shape.
- [ ] Re-run `markdownlint-cli2` on the touched document only.

### Completion condition for the trait architecture

This phase is complete only when all of the following are true:

- [ ] Every meaningful evidence family has a non-orphaned exchange path.
- [ ] Every higher-order trait method composes lower proofs instead of
  bypassing them.
- [ ] The strongest families in the codebase all follow the same sidecar
  grammar, rather than mixing branch-rich and branch-poor exchange styles
  arbitrarily.
- [ ] Consumers can obtain complex guarantees by reading trait signatures and
  result aliases alone, without hidden semantic assumptions.

## Associated Native Trait Family Refactor Checklist

The current temporal accord is strong on neutral descriptors and proof
sidecars, but weak on native carrier integration. Parsers lawfully accept
`&str`, and formatters lawfully emit `String`, yet too many higher-order seams
still treat text as the unofficial type of time. That is the wrong long-term
shape for backends such as `elicit_chrono`, `elicit_jiff`, and `elicit_time`,
whose real working values are native temporal carriers rather than strings.

The refactor target is a two-layer architecture:

- the existing neutral descriptor accord remains the shared, object-safe law
  boundary across independent crates
- a new associated-type trait family carries backend-native temporal values at
  compile time, with descriptor/native exchanges governed by the same proof
  sidecar pattern

### Architectural invariants

- [ ] Keep the neutral descriptor accord as the canonical inter-crate contract
  language. Descriptors and propositions remain the shared legal vocabulary.
- [ ] Restrict `&str` and `String` to explicit wire-text seams only:
  parse, format, stable identifiers, standards reporting, and similar textual
  interchange boundaries.
- [ ] Do not use `String` as a stand-in for local date-times, offset
  date-times, durations, intervals, named zones, or other temporal values at
  non-wire seams.
- [x] Treat semantic native-carrier traits as the primary interface, even when
  that means surrendering object safety.
- [ ] Preserve object safety only for seams that are inherently textual or
  metadata-oriented, where `dyn` remains lawful without collapsing temporal
  semantics into ad hoc concrete wrapper types.
- [ ] Do not invent framework-owned concrete temporal runtime types merely to
  preserve object safety; that would turn `elicit_temporal` into a time
  library instead of a trait accord.
- [ ] Require every native/descriptive exchange to follow the one true
  sidecar pattern: arguments carry explicit `Established<P>` sidecars, and
  results return fresh `Established<P>` sidecars minted from named evidence
  bundles through `ProvableFrom`.

### Family design checklist

- [x] Introduce a root associated-type family trait for temporal native
  carriers. Working name to confirm during implementation:
  `TemporalNativeProps`.
- [x] Partition the native family into coherent subfamilies rather than one
  monolithic bag of types.
  Minimum expected partitions:
  - [x] civil date/time carriers
  - [x] fixed-instant and offset carriers
  - [x] named-zone and zone-resolution carriers
  - [x] duration, interval, and recurrence carriers
  - [x] ISO 8601-2 / CalConnect extension carriers
- [x] For each subfamily, define the minimum lawful associated carriers before
  any trait refactor begins.
  Minimum expected carrier inventory:
  - [ ] calendar date
  - [ ] reduced calendar date
  - [ ] ordinal date
  - [ ] week date
  - [ ] local time
  - [ ] reduced local time
  - [ ] UTC offset
  - [ ] local date-time
  - [ ] offset date-time / fixed instant
  - [ ] named time zone
  - [ ] zoned date-time
  - [x] duration
  - [x] interval
  - [x] recurring interval
  - [x] qualified temporal value / explicit temporal form where a backend has
    a meaningful native representation
- [x] Decide explicitly which extension contracts remain descriptor-only
  because no stable cross-backend native carrier exists yet, and document that
  choice instead of leaving it implicit.
  `NATIVE_TRAIT_FAMILIES.md` now records the currently descriptor-only
  extension leftovers: seasonal expressions, sub-year grouping expressions,
  unspecified-component expressions, selection expressions, repeat rules, and
  recurring intervals with repeat rules.

### Trait-family checklist

- [ ] Preserve descriptor-oriented parse traits as lawful text-entry seams.
- [ ] Preserve descriptor-oriented format traits as lawful text-exit seams.
- [ ] Reclassify descriptor-only dynamic traits as secondary compatibility
  doors rather than the primary temporal abstraction.
- [x] Add a native constructor family for lawful descriptor-to-native
  realization.
  Each method should consume a neutral descriptor plus the full required proof
  sidecars and return a native carrier plus fresh postcondition sidecars.
- [x] Add a native projection family for lawful native-to-descriptor
  reflection.
  Each method should consume a backend-native temporal carrier and return the
  corresponding descriptor plus proof sidecars that make the extracted
  semantics explicit.
- [x] Add native zone-resolution and named-zone attachment families so the
  zone layer works on backend-native local, offset, and zoned values instead
  of forcing callers through string round-trips.
- [x] Add native interval, recurrence, and duration families so persistence and
  analytics consumers can work with actual temporal carriers rather than
  reparsed text.
- [x] Add a first native interval-ordering seam over backend fixed-instant
  carriers.
- [x] Add conditional native duration, interval, and recurrence bridge traits
  for backends whose upstream span carriers preserve enough structure to
  re-issue the lawful descriptor-side branches.
- [x] Split the span associated-type family so duration, interval, and
  recurrence can be supported independently rather than as one forced bundle.
- [x] Split the extension associated-type family so higher-order carriers can
  be supported independently rather than as one forced bundle.
- [x] Add a user-facing proven-carrier layer above the raw native bridges so
  higher-order methods aggregate sidecars into named semantic bundles instead
  of loose proof tuples.
- [x] Add native extension bridges and a native date-time-formula evaluation
  seam for the currently modeled higher-order carriers.
- [ ] Keep `TemporalReporter` textual and metadata-oriented unless a concrete
  non-textual capability contract becomes necessary.
- [x] Re-evaluate the current `TemporalBackend` blanket supertrait: if it
  implies a fully object-safe aggregate backend as the architectural center,
  replace that framing with associated trait families plus narrower
  descriptor-text compatibility seams.

### Proof-sidecar checklist for native exchanges

- [x] For every native constructor seam, list the exact descriptor-side proof
  preconditions it requires.
- [x] For every native projection seam, list the exact native-side evidence
  bundle that authorizes re-issuing descriptor proofs.
- [x] For every native result, define named result aliases so the exchange
  shape stays readable and clippy-compliant.
- [ ] For every new postcondition, define a named evidence bundle in
  `src/contracts/proof_composition.rs`.
- [ ] For every new evidence bundle, add the `ProvableFrom` derivation path
  immediately rather than staging it implicitly.
- [ ] Do not mint coarse aggregate proofs where downstream consumers may
  lawfully depend on finer branch families.
- [x] Ensure native zone-resolution seams preserve ambiguity versus gap
  branches, attachment evidence, offset consistency, and revision authority as
  separate sidecars.

### Orphan-prevention checklist

- [ ] Audit existing propositions and evidence bundles for contracts that are
  currently descriptor-only but should also participate in native seams.
- [ ] Audit each new associated carrier to ensure it is connected to at least
  one constructor seam, one projection seam, or one higher-order composition
  seam.
- [ ] Reject any associated type that exists only for symmetry but carries no
  lawful contract exchange.
- [ ] Reject any proof proposition that becomes unreachable once native seams
  exist.

### Migration sequence

Apply the refactor in this order instead of scattering associated types
through the crate ad hoc:

- [x] Step 1: write the native-family design note and finalize the root trait
  and subfamily names.
- [x] Step 2: inventory every current `String`-returning or string-mediated
  non-wire seam and classify it as either lawful text boundary or refactor
  target.
- [x] Step 3: introduce the associated-type family traits and placeholder
  result aliases in `elicit_temporal` without removing the descriptor accord.
- [x] Step 4: redefine the architectural center around native associated trait
  families rather than an all-in-one object-safe backend supertrait.
- [x] Step 4: add descriptor-to-native and native-to-descriptor bridge traits
  for one narrow vertical slice first.
  Recommended first slice:
  - [x] local date-time
  - [x] offset date-time
  - [x] named zone
  - [x] zoned date-time
- [x] Step 5: thread proof sidecars and `ProvableFrom` bundles through that
  first slice until no string round-trips remain at the higher-order seam.
- [x] Step 6: extend the same pattern to duration, interval, and recurrence
  families.
- [ ] Step 7: extend the same pattern to ISO 8601-2 / CalConnect higher-order
  forms where a backend-native carrier is justified.
- [ ] Step 8: implement the new families in `elicit_time`, `elicit_chrono`,
  and `elicit_jiff`, using their real native carriers instead of strings.
- [ ] Step 9: update downstream consumers and persistence crates to depend on
  native seams where appropriate, while preserving the descriptor accord as
  the stable cross-crate law boundary.
- [ ] Step 10: remove temporary string-mediated helper seams only after the
  native trait families fully cover the same contract guarantees.

### Definition of done for this refactor

- [ ] No non-wire temporal seam uses `String` as the de facto native temporal
  value.
- [ ] Native backend crates can expose their real carrier types through
  associated trait families without sacrificing proof-sidecar discipline.
- [ ] The descriptor accord remains standards-anchored, and object safety is
  retained only where it does not force fake concrete time types.
- [ ] The native-family layer composes with the descriptor accord through
  explicit `ProvableFrom` exchanges only.
- [ ] No meaningful temporal contract is left orphaned from both the
  descriptor and native layers.

## Phase 0: Standards Corpus and Research Method

### 0.1 Establish the normative source hierarchy

Create a standards manifest in the new crate that classifies each source as
normative, profile, or backend-specific:

1. **Normative core**
   - `ISO 8601-1:2019` — basic rules for dates, times, UTC offsets, intervals
   - `ISO 8601-1:2019/Amd 1:2022` — technical corrections
   - `ISO 8601-2:2019` — extensions and richer notations
2. **Internet interchange profiles**
   - `RFC 3339` — Internet timestamp profile of ISO 8601
   - `RFC 9557` — IXDTF, RFC 3339 extension with additional information such
     as time zone names
3. **Support references for semantics that the above rely on**
   - IANA Time Zone Database / BCP 175 references where named zone semantics
     matter
   - leap-second authorities referenced by the standards where relevant
4. **Consumer refinement sources**
   - database backend timestamp semantics for `elicit_db`-adjacent work
   - crate-specific upstream docs for `time`, `chrono`, and `jiff`
   - analytics / rendering consumer requirements only after the neutral accord
     exists

### 0.2 Create a standards extraction ledger

Add a research document under the new crate, for example
`crates/elicit_temporal/STANDARDS_LEDGER.md`, with one entry per clause or
rule mined from the standards corpus. Each entry should include:

- source document
- exact clause or section identifier
- standard text paraphrase
- contract classification:
  - structural
  - mathematical
  - authoritative / interpretive
- likely representation:
  - descriptor field shape
  - proposition type
  - evidence bundle field
  - trait precondition
  - trait postcondition
- notes about backend drift or consumer-specific refinement

Do not start from trait names. Start from clause extraction.

### 0.3 Define citation discipline up front

Every proposition, descriptor, evidence bundle, and trait method that encodes
temporal law should eventually carry doc comments with standards references.
Adopt a consistent doc-comment format before implementation begins, for
example:

```rust
/// ISO 8601-1:2019 §4.3.2 b)
/// RFC 3339 §5.6
/// Timestamp has an explicit UTC offset and denotes a fixed instant.
pub struct TimestampHasUtcOffset;
```

Rules:

1. Cite the narrowest useful clause, not only the top-level standard.
2. Distinguish normative citations from informative notes.
3. If multiple sources constrain the same proposition, document the layering.
4. If a consumer refinement narrows the neutral law, place the refinement in
   the consumer crate, not in `elicit_temporal`.

## Phase 1: Contract Taxonomy Before Traits

### 1.1 Partition the temporal domain into contract families

Build the initial contract tree under `crates/elicit_temporal/src/contracts/`
with explicit subdomains:

- `calendar.rs`
  - Gregorian date structure
  - ordinal dates
  - week dates
  - leap-year relations
- `clock.rs`
  - 24-hour clock rules
  - subsecond precision
  - leap-second handling
- `offset.rs`
  - UTC offsets
  - explicit-zero offset semantics
  - offset bounds and formatting
- `zone.rs`
  - named time-zone semantics
  - IANA zone identity vs numeric offset
  - offset/zone consistency
- `instant.rs`
  - fixed timestamps
  - local vs offset-aware vs zoned distinctions
  - ambiguity / non-ambiguity of represented instants
- `interval.rs`
  - start/end intervals
  - duration-based intervals
  - recurring intervals
- `precision.rs`
  - precision retention
  - truncation / rounding law
  - lossy conversion boundaries
- `serialization.rs`
  - ISO 8601 forms
  - RFC 3339 profile forms
  - IXDTF forms
- `conversion.rs`
  - lawful conversions between descriptors
  - proof obligations for lossless vs lossy conversion
- `consistency.rs`
  - composed propositions over several subdomains
  - evidence bundles and aggregate proofs

The directory layout should reflect the standards corpus, not the current Rust
crate APIs.

### 1.2 Classify each proposition into proof tiers

For each contract family, decide whether a proposition is:

1. **Structural**
   - proven by descriptor shape and constructor restriction
   - example: a type that cannot exist without an explicit offset
2. **Mathematical**
   - verifier-friendly and suitable for Kani / Creusot / Verus harnesses
   - example: range checks, ordering, leap-year arithmetic, week-date bounds
3. **Authoritative / interpretive**
   - proven at an authority method via proof sidecar
   - example: backend attestation that a named time zone came from a trusted
     zone database snapshot, or that a storage round trip preserved semantics

Do not collapse these categories. The whole proof architecture depends on
keeping them distinct.

### 1.3 Draft proposition families aggressively

Expect a large contract surface. It is acceptable, and probably correct, for
`elicit_temporal` to define hundreds of propositions over time.

Initial high-value proposition families:

- `GregorianDateValid`
- `OrdinalDateValid`
- `WeekDateValid`
- `ClockTimeValid`
- `LeapSecondRepresentable`
- `UtcOffsetValid`
- `UtcOffsetKnown`
- `TimestampRepresentsFixedInstant`
- `LocalDateTimeDoesNotIdentifyFixedInstant`
- `ZonedDateTimeHasNamedZone`
- `OffsetConsistentWithNamedZone`
- `OffsetOnlyZoneSemanticsLimited`
- `Iso8601BasicFormValid`
- `Iso8601ExtendedFormValid`
- `Rfc3339TimestampValid`
- `IxdtfTimestampValid`
- `DurationFormValid`
- `IntervalEndpointsOrdered`
- `RecurringIntervalFormValid`
- `PrecisionPreserved`
- `ConversionLossless`
- `ConversionTruncatesSubseconds`
- `TemporalOrderingPreserved`

The purpose at this stage is coverage, not elegance.

## Phase 2: Descriptor and Evidence Design

### 2.1 Define neutral contract descriptors

Under `crates/elicit_temporal/src/types/`, define descriptor types that
express temporal law in a backend-neutral vocabulary. These are not wrappers
around `chrono`, `time`, or `jiff`; they are the accord types that those crates
must map into and out of.

Likely descriptor families:

- `CalendarDateDescriptor`
- `OrdinalDateDescriptor`
- `WeekDateDescriptor`
- `ClockTimeDescriptor`
- `UtcOffsetDescriptor`
- `LocalDateTimeDescriptor`
- `OffsetDateTimeDescriptor`
- `ZonedDateTimeDescriptor`
- `InstantDescriptor`
- `DurationDescriptor`
- `TimeIntervalDescriptor`
- `RecurringIntervalDescriptor`
- `PrecisionDescriptor`
- `TemporalSerializationDescriptor`

Rules for these descriptors:

1. Use builders, never struct literals in consumers.
2. Distinguish fixed-instant semantics from local-time semantics explicitly.
3. Model ambiguity directly; do not flatten local, offset-aware, and zoned time
   into one convenient bag.
4. Keep calendar identity explicit if RFC 9557 / IXDTF extensions require it.

### 2.2 Define evidence bundle types

Under `contracts/proof_composition.rs` or temporal submodules, define evidence
bundles that assemble leaf proofs into stronger compound claims. Examples:

- `FixedInstantEvidence`
  - `Established<GregorianDateValid>`
  - `Established<ClockTimeValid>`
  - `Established<UtcOffsetValid>`
  - `Established<TimestampRepresentsFixedInstant>`
- `ZonedTimestampEvidence`
  - `Established<FixedInstantRepresented>`
  - `Established<ZonedDateTimeHasNamedZone>`
  - `Established<OffsetConsistentWithNamedZone>`
- `LosslessConversionEvidence`
  - source validity
  - target validity
  - precision preservation
  - ordering preservation
- `IntervalValidityEvidence`
  - endpoint validity
  - endpoint ordering
  - representation form validity

These bundles are the raw material for `ProvableFrom` recursion.

### 2.3 Normalize ambiguous and lossy cases early

Do not defer the hard cases. Explicitly model:

- local timestamps that do not name a fixed instant
- repeated / skipped local times near zone transitions
- loss of named-zone information when converting to offset-only forms
- precision truncation across backends
- inability of a backend to round-trip leap seconds or certain ISO 8601 forms

Each of these should have named propositions and explicit conversion proofs or
conversion failures.

## Phase 3: Trait Interface Design

### 3.1 Define role families for the temporal accord

Once the contract language is stable enough, add object-safe trait families
under `crates/elicit_temporal/src/traits/`.

Suggested role split:

- **Role 1a — leaf temporal factories**
  - parse, construct, normalize, convert, project, compare
  - each lawful method returns `(Descriptor, Established<P>)` or richer tuples
- **Role 1b — proof composition**
  - evidence-bundle-driven `ProvableFrom` impls that mint aggregate proofs
- **Role 2 — temporal reporters / inspectors**
  - metadata queries that do not mint or consume proofs

Potential trait families:

- `TemporalParser`
- `TemporalFormatter`
- `TemporalInstantFactory`
- `TemporalZoneFactory`
- `TemporalConversionFactory`
- `TemporalIntervalFactory`
- `TemporalPrecisionFactory`
- `TemporalOrderingFactory`
- `TemporalReporter`

Initial landed slice:

- neutral descriptor module for core dates, times, offsets, zoned timestamps,
  durations, intervals, recurring intervals, precision, and serialization
- crate-local `TemporalError` / `TemporalResult`
- object-safe `TemporalParser`, `TemporalFormatter`, `TemporalZoneFactory`,
  `TemporalConversionFactory`, `TemporalIntervalFactory`, and
  `TemporalReporter` scaffolding

### 3.2 Make every method speak the proof-sidecar grammar

Method signatures should make lawful exchange explicit. Examples:

```rust
fn parse_rfc3339(
    &self,
    input: String,
) -> TemporalResult<(
    OffsetDateTimeDescriptor,
    Established<Rfc3339TimestampValid>,
    Established<TimestampRepresentsFixedInstant>,
)>;

fn attach_named_zone(
    &self,
    input: OffsetDateTimeDescriptor,
    zone: TimeZoneIdentifier,
    _pre: Established<TimestampRepresentsFixedInstant>,
) -> TemporalResult<(
    ZonedDateTimeDescriptor,
    Established<ZonedDateTimeHasNamedZone>,
    Established<OffsetConsistentWithNamedZone>,
)>;
```

Every signature should answer:

- what descriptor crosses the boundary
- which propositions must already hold
- which propositions are newly established
- whether the exchange is lossless, lossy, or ambiguous

### 3.3 Add intermediary proof steps instead of giant methods

Do not jump directly from raw strings to high-assurance aggregate proofs if
intermediate law matters. Introduce intermediary contracts where standards
have real semantic distinctions:

- parsed syntax validity
- calendar validity
- offset validity
- fixed-instant establishment
- named-zone consistency
- precision preservation
- storage compatibility

This keeps proof generation recursive and auditable.

## Phase 4: `ProvableFrom` Graph Construction

### 4.1 Encode the one true pattern explicitly

For every aggregate temporal proposition, define the evidence bundle and
`ProvableFrom` path that proves it. Avoid shortcut proofs except where the
proposition is genuinely structural.

Examples:

- `ProvableFrom<FixedInstantEvidence> for FixedInstantRepresented`
- `ProvableFrom<ZonedTimestampEvidence> for ZonedTimestampConsistent`
- `ProvableFrom<LosslessConversionEvidence> for ConversionLossless`
- `ProvableFrom<IntervalValidityEvidence> for TimeIntervalValid`

### 4.2 Make invalidation and refinement explicit

When a later step can invalidate or narrow a prior proposition, model that in
the API instead of assuming the old proof still applies.

Examples:

- truncating nanoseconds should not preserve `PrecisionPreserved`
- dropping the named zone should not preserve `ZonedDateTimeHasNamedZone`
- serializing to RFC 3339 should not preserve IXDTF-only properties

The proof graph must track these changes honestly.

### 4.3 Add proof composition modules early

Create `contracts/proof_composition.rs` early in the crate, even before all
leaf traits are implemented. This becomes the architectural backbone and the
review surface for aggregate claims.

## Phase 5: Verification Strategy

### 5.1 Add crate-scoped verification targets from the start

Extend the workspace justfile to support a scoped command for the new crate:

- `just check elicit_temporal`
- `just test-package elicit_temporal`

Prefer crate-scoped verification while the contract surface grows.

### 5.2 Partition verification work by proof tier

1. **Structural proofs**
   - encode in type shape and constructors
   - verify mostly through compile-time restrictions
2. **Kani**
   - finite arithmetic, parser normalization, lossless round trips within bounds
3. **Creusot**
   - compositional invariants over descriptors and conversions
4. **Verus**
   - stronger semantic proofs on core normalization and composition routines

Do not wait until the end to add harnesses. Add them alongside each family of
contracts so proof drift is caught immediately.

### 5.3 Focus the first verification wave

First-wave proofs should target the hardest high-value reusable invariants:

- Gregorian leap-year arithmetic
- ordinal / week-date conversion correctness
- UTC offset range and formatting rules
- fixed-instant preservation under lawful conversions
- precision-preserving vs precision-losing conversions
- interval endpoint ordering

These invariants will be reused across every leaf implementation.

## Phase 6: Leaf Crate Adoption

### 6.1 Keep current temporal crates as leaves

Treat the current crates as producer / consumer leaves:

- `elicit_time`
- `elicit_chrono`
- `elicit_jiff`

They should depend on `elicit_temporal`, implement its trait families where
appropriate, and mint proofs against upstream payloads.

### 6.2 Add explicit mapping layers

For each leaf crate:

1. map upstream types into neutral temporal descriptors
2. establish lawful propositions through parsing / construction methods
3. expose proof-carrying conversion methods back into upstream forms

Do not let leaf wrappers define private temporal law once `elicit_temporal`
exists.

### 6.3 Start with a constrained slice

A good first adoption slice:

- offset-aware timestamp parsing and formatting
- named-zone timestamp handling
- fixed-instant conversion between `time`, `chrono`, and `jiff`
- explicit proof of lossless vs lossy conversions

This is enough to validate the seam before tackling every interval and exotic
representation.

## Phase 7: Persistence and Downstream Integration

### 7.1 Let `elicit_db` refine, not replace, temporal law

Once `elicit_temporal` exists, `elicit_db` should add persistence-specific
temporal propositions on top of it, for example:

- `SqlTimestampRoundTripsLosslessly`
- `BackendPreservesUtcOffset`
- `BackendDropsNamedZone`
- `BackendNormalizesToUtc`
- `IndexedTemporalOrderingPreserved`

These should consume `elicit_temporal` descriptors and proofs rather than
redefining them.

### 7.2 Integrate non-database consumers explicitly

Plan separate adoption slices for:

- `elicit_polars`
  - columnar precision, ordering, nullability, time-unit semantics
- `elicit_bevy`
  - simulation / animation timelines, fixed-step vs wall-clock distinctions
- other consumers
  - only after they can depend on the neutral temporal accord

This keeps the branch crate neutral while still supporting real consumers.

## Phase 8: Documentation and Review Surface

### 8.1 Add top-level architecture docs immediately

The new crate should ship with:

- `README.md` — crate purpose, role in the workspace, trait taxonomy
- `STANDARDS_LEDGER.md` — clause extraction and contract registry
- `PROOF_ARCHITECTURE.md` or equivalent — evidence bundle and `ProvableFrom`
  map

### 8.2 Embed standards references into code comments continuously

Do not postpone citations until cleanup. During implementation:

- add doc comments on proposition types
- add doc comments on evidence bundles
- add doc comments on trait methods
- include clause-level references wherever practical

This keeps the code review surface auditable and supports future certification
work.

### 8.3 Review against a strict architectural checklist

Every PR in this workstream should be checked for:

- standards clause traced into a named contract
- contract correctly classified by proof tier
- neutral descriptor vocabulary preserved
- no consumer-specific policy leaking into `elicit_temporal`
- every aggregate proof backed by an explicit `ProvableFrom` path
- every exchange carrying typed proof sidecars
- no stringly typed error collapse or proof-shortcut APIs

### 8.4 Post-review fidelity remediation checklist

Address the remaining architectural gaps in this order so the crate closes the
highest-fidelity seams first instead of broadening unevenly.

- [x] Reconcile citation-state claims with the actual standards ledger
  Completion condition: `STANDARDS_LEDGER.md` no longer claims the
  clause-tightening pass is complete while licensed-text ISO extraction remains
  outstanding, and the active gap list distinguishes citation debt from
  contract-surface debt.
- [x] Promote mined ISO 8601-2 forms into neutral exchange descriptors
  Completion condition: seasonal expressions, unspecified-component
  expressions, and temporal sets each have descriptor vocabulary that can move
  across trait boundaries without collapsing back to strings or comments.
- [x] Add parser and formatter seams for the new ISO 8601-2 descriptors
  Completion condition: the object-safe trait surface can parse and emit the
  ISO 8601-2 forms already represented in the contract catalog, with named
  proof-carrying result aliases where needed.
- [x] Decompress IXDTF exchange results into explicit proof branches
  Completion condition: IXDTF parse and format exchanges can return explicit
  proof sidecars for named-zone identity, offset-zone annotation semantics,
  calendar annotations, and additional-information semantics rather than only
  coarse aggregate validity tokens.
- [x] Add lawful zone-resolution seams for ambiguous and gap local times
  Completion condition: consumers can submit a local wall-clock representation
  together with explicit disambiguation or gap-handling authority and receive
  typed results plus transition-specific proofs.
- [x] Replace stringly serialization metadata with typed annotation descriptors
  Completion condition: serialization-side zone and calendar metadata use
  neutral descriptor types that align with the IXDTF contract surface rather
  than raw `String` fields.
- [x] Expand conversion and interval seams to match the mined lawbook
  Completion condition: the runtime trait families cover the interval,
  precision, and downgrade paths already present in the proposition catalog,
  especially for persistence-oriented consumers.
- [x] Run a final `ProvableFrom` closure pass over every new exchange
  Completion condition: every added aggregate result or evidence bundle has an
  explicit proof derivation path and no new API returns validated data without
  the corresponding proof sidecars.

## Suggested Implementation Order

1. Create `crates/elicit_temporal/` with README and standards ledger.
2. Add the crate to the workspace and wire crate-scoped `just` checks.
3. Mine ISO 8601-1 / 8601-2, RFC 3339, and RFC 9557 into the ledger.
4. Draft the first contract families:
   - calendar
   - clock
   - offset
   - instant
   - serialization
5. Define neutral descriptors and builders for those families.
6. Add first-wave propositions and proof composition bundles.
7. Implement first-wave object-safe traits for parsing, formatting, and
   fixed-instant conversions.
8. Add Kani / Creusot / Verus coverage for the first-wave mathematical
   invariants.
9. Adapt `elicit_time`, `elicit_chrono`, and `elicit_jiff` to the new accord.
10. Add `elicit_db` temporal refinements as a separate follow-on phase.

## Success Criteria

`elicit_temporal` is on the right path when:

- the standards corpus is transcribed into a growing, cited contract registry
- time law is expressed in neutral descriptors rather than leaf-specific types
- leaf crates mint proofs against the shared accord instead of private rules
- every nontrivial temporal exchange returns proof sidecars
- aggregate temporal guarantees are assembled recursively through
  `ProvableFrom`
- verifier-friendly invariants are established once and reused downstream
  without repeated validation
- downstream crates can depend on temporal law without depending on a specific
  backend or storage engine
