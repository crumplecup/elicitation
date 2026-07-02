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
