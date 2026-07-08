# Native Trait Families

`elicit_temporal` must not collapse into `String` as the unofficial type of
time, and it must not reinvent a framework-owned time library merely to retain
object safety. The correct center of the architecture is therefore:

- standards-anchored descriptors and propositions as the shared accord
- associated native trait families for backend-owned temporal carriers
- explicit proof-sidecar exchanges between the two layers

The string-seam audit that accompanies this design note is recorded in
[NATIVE_SEAM_AUDIT.md](NATIVE_SEAM_AUDIT.md).

## Design rules

1. Descriptors remain the shared legal vocabulary across crates.
2. Native carrier traits are the primary semantic interface, even when that
   costs object safety.
3. Text is lawful only at wire seams such as parse, format, identifiers, and
   metadata reporting.
4. Every descriptor-to-native or native-to-descriptor exchange must carry
   argument-side and result-side proofs through the same `Established<P>` and
   `ProvableFrom` pattern as the rest of the crate.
5. Higher-order native traits should aggregate those low-level sidecars into
   named semantic bundles and proven-carrier wrappers so users do not juggle
   raw proof tuples directly.
6. We do not invent framework-owned concrete runtime time types merely to make
   `dyn` work.

## Root family

The umbrella associated-type family is `TemporalNativeProps`.

It is intentionally decomposed into narrower capability families so a backend
can implement only the lawful portions it actually supports:

- `TemporalCivilProps`
- `TemporalInstantProps`
- `TemporalZoneProps`
- `TemporalDurationProps`
- `TemporalTimeIntervalProps`
- `TemporalRecurringIntervalProps`
- `TemporalSpanProps`
- `TemporalQualifiedTemporalValueProps`
- `TemporalExplicitTemporalFormProps`
- `TemporalExplicitDurationProps`
- `TemporalExplicitTimeIntervalProps`
- `TemporalGroupedTimeScaleUnitProps`
- `TemporalSetProps`
- `TemporalDateTimeFormulaProps`
- `TemporalExtensionProps`

`TemporalNativeProps` is the blanket composition of those narrower families.
It should represent a fully-capable temporal backend, not the minimum bar for
every crate that touches time.

## Subfamily inventory

### `TemporalCivilProps`

Civil carriers that do not by themselves identify a unique instant:

- `CalendarDate`
- `ReducedCalendarDate`
- `OrdinalDate`
- `WeekDate`
- `LocalTime`
- `ReducedLocalTime`
- `LocalDateTime`

### `TemporalInstantProps`

Carriers that participate in fixed-instant semantics:

- `UtcOffset`
- `OffsetDateTime`
- `Instant`

`Instant` is kept distinct from `OffsetDateTime` because some upstream
libraries expose an absolute timestamp type that is not the same thing as an
offset-bearing civil representation.

### `TemporalZoneProps`

Carriers for named-zone identity and zone-attached timestamps:

- `NamedTimeZone`
- `ZonedDateTime`

This family is intentionally separate because not every backend has a lawful
upstream named-zone carrier. A backend that only supports offsets should not be
forced to invent a fake zone type.

### `TemporalDurationProps`

Carriers for elapsed or nominal durations:

- `Duration`

This family carries an important constraint: not every upstream "duration"
type is lawful for every span seam. A scalar elapsed-seconds type may be
insufficient to re-issue ISO nominal-duration structure or representation
branches. In that case the backend should leave the duration bridge
unimplemented and continue using the descriptor accord for that portion of the
law surface.

### `TemporalTimeIntervalProps`

Carriers for interval values:

- `TimeInterval`

This family is separate from duration because an upstream backend may have a
lawful interval carrier even when it lacks a lawful nominal-duration carrier.

### `TemporalRecurringIntervalProps`

Carriers for recurrence values:

- `RecurringInterval`

This family is separate because a backend may lawfully support interval
carriers without also exposing a stable native recurring-interval carrier.

### `TemporalSpanProps`

Aggregate family for backends that support all three span subfamilies:

- `TemporalDurationProps`
- `TemporalTimeIntervalProps`
- `TemporalRecurringIntervalProps`

### `TemporalExtensionProps`

Aggregate family for the currently modeled higher-order extension carriers:

- `TemporalQualifiedTemporalValueProps`
- `TemporalExplicitTemporalFormProps`
- `TemporalExplicitDurationProps`
- `TemporalExplicitTimeIntervalProps`
- `TemporalGroupedTimeScaleUnitProps`
- `TemporalSetProps`
- `TemporalDateTimeFormulaProps`

### `TemporalQualifiedTemporalValueProps`

Carriers for qualified temporal values:

- `QualifiedTemporalValue`

### `TemporalExplicitTemporalFormProps`

Carriers for explicit temporal forms:

- `ExplicitTemporalForm`

### `TemporalExplicitDurationProps`

Carriers for explicit durations:

- `ExplicitDuration`

### `TemporalExplicitTimeIntervalProps`

Carriers for explicit time intervals:

- `ExplicitTimeInterval`

### `TemporalGroupedTimeScaleUnitProps`

Carriers for grouped time scale units:

- `GroupedTimeScaleUnit`

### `TemporalSetProps`

Carriers for temporal sets:

- `TemporalSet`

### `TemporalDateTimeFormulaProps`

Carriers for date-time formulas:

- `DateTimeFormula`

### Extension families currently left descriptor-only

Higher-order ISO 8601-2 / CalConnect carriers where a backend has a meaningful
native representation are now modeled as native subfamilies. The following
forms remain descriptor-only for now because we do not yet have a stable,
cross-backend native carrier story for them:

- `SeasonalTemporalExpressionDescriptor`
- `SubYearGroupingExpressionDescriptor`
- `UnspecifiedComponentExpressionDescriptor`
- `SelectionExpressionDescriptor`
- `RepeatRuleDescriptor`
- `RecurringIntervalWithRepeatRuleDescriptor`

Where no lawful native carrier exists yet, the descriptor accord remains the
canonical representation and no fake runtime type should be invented.

## Interface direction

The intended layering is:

1. Parse traits remain lawful text-entry seams.
2. Format traits remain lawful text-exit seams.
3. Native constructor traits realize validated descriptors into backend-native
   carriers.
4. Native projection traits reflect backend-native carriers back into neutral
   descriptors with explicit proofs.
5. User-facing higher-order traits should operate on proven native carriers
   and named semantic bundles rather than loose `Established<P>` parameters.
6. Higher-order conversion, zone, interval, and recurrence traits should
   operate on native carriers once those families exist, not on string
   round-trips.

The first bridge traits now exist for that runtime seam:

- `TemporalCivilNativeBridge`
- `TemporalInstantNativeBridge`
- `TemporalZoneNativeBridge`
- `TemporalNativeBridge`

The first higher-order native traits now exist above those bridges:

- `TemporalNativeZoneFactory`
- `TemporalNativeConversionFactory`
- each now consumes proven carriers plus named authority bundles rather than
  loose proof tuples

The first native span traits now also exist:

- `TemporalDurationNativeBridge`
- `TemporalTimeIntervalNativeBridge`
- `TemporalRecurringIntervalNativeBridge`
- `TemporalNativeSpanBridge`
- `TemporalNativeIntervalFactory`
- span bridges now realize and reflect proven carriers carrying bundled
  duration, interval, and recurrence semantics

The first native extension traits now also exist:

- `TemporalQualifiedTemporalValueNativeBridge`
- `TemporalExplicitTemporalFormNativeBridge`
- `TemporalExplicitDurationNativeBridge`
- `TemporalExplicitTimeIntervalNativeBridge`
- `TemporalGroupedTimeScaleUnitNativeBridge`
- `TemporalSetNativeBridge`
- `TemporalDateTimeFormulaNativeBridge`
- `TemporalNativeExtensionBridge`
- `TemporalNativeDateTimeFormulaFactory`
- extension bridges now realize and reflect proven carriers carrying bundled
  higher-order semantics, and formula evaluation returns a proven explicit
  temporal form plus explicit evaluation-result provenance

## First implementation slice

The first vertical slice should be:

- `LocalDateTime`
- `OffsetDateTime`
- `NamedTimeZone`
- `ZonedDateTime`

This slice removes the most damaging string-mediated seams first: local wall
time, fixed-instant time, named-zone identity, and zone-attached time.

## Non-goals

- Making the entire semantic temporal surface object-safe
- Introducing framework-owned replacement time structs
- Collapsing fine-grained proof branches into coarse native validity tokens
