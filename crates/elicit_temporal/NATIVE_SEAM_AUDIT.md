# Native Seam Audit

This audit closes Step 2 of the associated native trait-family refactor plan:
inventory every current `String`-returning or string-mediated seam in
`elicit_temporal`, then classify each one as either a lawful text boundary or
an architectural refactor target.

## Decision rule

`String` or `&str` is lawful when it carries one of these roles:

- wire text entering the accord
- wire text leaving the accord
- stable identifiers or metadata that are text by nature
- lexical payload preserved inside a neutral descriptor

`String` or `&str` is a refactor target only when it is being used as the de
facto runtime carrier for a temporal value that should instead live in a native
associated-type family.

## Result

No current trait seam in `elicit_temporal` uses `String` as the de facto
non-wire runtime carrier for dates, times, offsets, durations, intervals, or
zoned timestamps.

That means the string problem is narrower than it first appeared. The actual
remaining debt is not "remove `String` from semantic traits"; it is "add native
descriptor-bridge trait families so higher-order semantics stop terminating at
neutral descriptors."

## Lawful text boundaries

### Parse entry seams

These are lawful text-entry boundaries and should remain text-based:

- `TemporalParser` methods taking `input: &str`
- `TemporalCalConnectFactory` parse methods taking `input: &str`
- `TemporalIntervalFactory` parse methods taking `input: &str`
- `TemporalZoneFactory::resolve_named_zone(&self, identifier: &str)`

Why lawful:

- the caller is supplying standards-governed wire text or a zone identifier
- the seam is explicitly lexical-to-descriptor, not semantic native-to-native

### Format exit seams

These are lawful text-exit boundaries and should remain string-based:

- `TemporalFormatter` methods returning `TemporalResult<(String, ...)>`
- all `Formatted*Result` aliases in `src/types.rs`

Why lawful:

- the purpose of the seam is emission of ISO 8601, RFC 3339, RFC 9557, or
  CalConnect wire text
- the returned `String` is the serialized artifact itself, not an ersatz
  temporal runtime value

### Metadata/reporting seams

These are lawful metadata strings:

- `TemporalReporter::supported_ixdtf_annotation_keys(&self) -> Vec<String>`
- `TemporalReporter::current_tzdb_revision(&self) -> Option<String>`

Why lawful:

- suffix keys and TZDB revisions are textual metadata, not temporal runtime
  carriers

## Lawful lexical descriptor fields

The following `String` fields are also lawful because the descriptor is
preserving lexical content mandated or permitted by the standards:

- fractional digits in `FractionalSecondDescriptor`
- named-zone identifier and optional TZDB revision in `NamedTimeZoneDescriptor`
- IXDTF annotation keys and values in `IxdtfAnnotationDescriptor`
- preferred calendar identifier in `IxdtfCalendarAnnotationDescriptor`
- duration and grouped-unit fraction digits
- implementation-defined rounding mode payload in `PrecisionRoundingMode::Other`

These are part of the neutral descriptor accord. Replacing them with native
runtime time types would be category error.

## Actual refactor targets

The real architectural targets exposed by this audit are these:

- descriptor-only semantic seams remain the dominant runtime abstraction
- no native constructor family yet realizes validated descriptors into backend
  carriers such as `chrono`, `time`, or `jiff` values
- no native projection family yet reflects backend carriers back into neutral
  descriptors with explicit proofs
- higher-order zone, conversion, duration, interval, and recurrence work still
  terminates at descriptors instead of continuing through native associated
  families
- the compatibility aggregate `TemporalBackend` still overstates the importance
  of the descriptor-oriented doorway

## First implementation target after this audit

The next vertical slice should remain:

- local date-time
- offset date-time
- named time zone
- zoned date-time

That slice addresses the highest-value semantic runtime path first without
confusing lawful wire text with native temporal carriers.
