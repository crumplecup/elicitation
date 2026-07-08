# Citation Worksheet: `src/contracts/instant.rs`

This worksheet stages exhaustive coverage review for the mixed-authority
instant-semantics surface in `src/contracts/instant.rs`.

## Source set

- `../iso-8601-1-2019.*`
- `../../public/iso-wd-8601-1-2016.txt`
- `../../public/rfc3339.txt`
- `../../public/rfc9557.txt`

## Instant contract map

| Contract symbol | Current topic label | Exact clause | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `TimestampHasExplicitUtcOffset` | `timestamp carries explicit UTC relationship` | `RFC 3339 §4.4; §5.6` | `../../public/rfc3339.txt lines 262-274; 417-422` | `yes` |
| `OffsetDateTimeIdentifiesSingleInstant` | `offset date-time identifies single instant` | `ISO 8601-1:2019 5.4.2; 5.4.3; RFC 3339 §5.6` | `../iso-8601-1-2019.sample.txt lines 191-194; ../../public/iso-wd-8601-1-2016.txt lines 1548-1568; RFC 3339 lines 417-422` | `yes` |
| `LocalDateTimeRequiresZoneOrOffsetForInstant` | `local date-time requires zone or offset authority before instant identification` | `RFC 3339 §4.4` | `../../public/rfc3339.txt lines 262-274` | `yes` |
| `LocalDateTimeMayBeAmbiguousAtZoneTransition` | `local date-time may be ambiguous at zone transition` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 221-224` | `yes` |
| `LocalDateTimeMayFallInZoneTransitionGap` | `local date-time may fall in zone-transition gap` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 221-224` | `yes` |
| `UtcTimelineOrderingAppliesToFixedInstants` | `UTC timeline ordering applies to fixed instants` | `RFC 3339 §5.1` | `../../public/rfc3339.txt lines 299-307` | `yes` |

## Instant Coverage Audit

| Clause concept | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `ISO 8601-1:2019 5.4.2; 5.4.3 date-time identifies a time point with UTC relationship` | `covered` | `OffsetDateTimeIdentifiesSingleInstant` | The instant-identifying semantics for offset date-times are explicit. |
| `RFC 3339 §4.4 unqualified local time is unacceptable for interoperable timestamp exchange` | `covered` | `TimestampHasExplicitUtcOffset`, `LocalDateTimeRequiresZoneOrOffsetForInstant` | The surface distinguishes timestamp forms that already carry an explicit UTC relationship from local forms that still require authority. |
| `RFC 3339 §5.1 lexical ordering is meaningful only for fixed-instants under uniform encoding assumptions` | `covered` | `UtcTimelineOrderingAppliesToFixedInstants` | The deeper uniform-encoding preconditions are intentionally modeled in `rfc3339.rs`, not duplicated here. |
| `RFC 3339 §5.6 full-time requires a time-offset` | `covered` | `TimestampHasExplicitUtcOffset`, `OffsetDateTimeIdentifiesSingleInstant` | The grammar-level UTC relationship requirement and the instant-level consequence are both explicit. |
| `RFC 9557 §1.2 local-time-to-instant conversion may have zero or multiple solutions near zone transitions` | `covered` | `LocalDateTimeMayBeAmbiguousAtZoneTransition`, `LocalDateTimeMayFallInZoneTransitionGap` | Transition ambiguity and gap cases are both first-class. |

## Notes

- This file is intentionally mixed-authority: ISO 8601-1 supplies the core
  offset-date-time instant semantics, RFC 3339 supplies Internet-profile UTC
  relationship and ordering semantics, and RFC 9557 supplies zone-transition
  ambiguity semantics for local date-times.
- The aggregate proof carriers for this surface live in
  `src/contracts/proof_composition.rs` as `FixedInstantEvidence` and
  `LocalTimestampSemanticsEvidence`; those sidecars are tracked centrally in
  `contracts_proof_composition.rs.md`.
- No additional first-order instant proposition is currently required for the
  cited clauses. The remaining adjacent semantics about named-zone identity,
  inconsistency handling, and transition resolution authority intentionally
  live in `zone.rs`.

## Remaining Coverage Checklist

- [x] Replace the old ISO-only staging note with a full mixed-authority audit.
- [x] Map every first-order proposition in `src/contracts/instant.rs` to its
  governing clause family.
- [x] Separate instant-level semantics from adjacent zone-identity and
  inconsistency-handling semantics to avoid duplicating `zone.rs`.
