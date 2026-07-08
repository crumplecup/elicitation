# Citation Worksheet: `src/traits/zone.rs`

This worksheet stages the trait-surface audit for named-zone resolution and
attachment in `src/traits/zone.rs`.

## Source set

- `../../public/rfc9557.txt`
- `contracts_zone.rs.md`
- `contracts_proof_composition.rs.md`

## Trait coverage map

| Trait seam | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `resolve_named_zone` | `covered` | `ResolvedNamedTimeZoneResult`, `NamedTimeZoneIdentityValid`, `NamedTimeZoneIdentityEvidence` | Named-zone identity now crosses the seam as a generic zone-law bundle rather than a lone IANA-identifier token. |
| `confirm_local_time_zone_resolution_authority` | `covered` | `ConfirmedZoneTransitionResolutionAuthorityResult`, `ZoneTransitionResolutionAuthorityValid`, `ZoneTransitionResolutionAuthorityEvidence` | The authority descriptor now has an explicit proof-establishing doorway before higher-order local-time resolution consumes it. |
| `resolve_local_date_time` | `covered` | `ResolvedLocalDateTimeAtNamedZoneResult`, `NamedZoneAttachmentEvidence`, `OffsetConsistencyEvidence`, `ZoneTransitionResolutionAuthorityEvidence`, `LocalTimeZoneResolutionProofBranch` | Local-to-zone resolution now consumes named-zone identity and declared authority, then re-issues attachment, consistency, authority, and ambiguity-versus-gap semantics explicitly. |
| `attach_named_zone` | `covered` | `AttachedNamedZoneResult`, `NamedZoneAttachmentEvidence`, `OffsetConsistencyEvidence` | Generic named-zone attachment is now distinct from IXDTF-specific suffix annotation semantics. |
| `confirm_named_zone_revision` | `covered` | `ConfirmedNamedZoneRevisionResult`, `NamedTimeZoneRevisionEvidence` | Revision-aware named-zone interpretation now crosses the seam with explicit revision evidence rather than a lone aggregate token. |

## Notes

- This trait worksheet is intentionally separate from `contracts_zone.rs.md`:
  the contract worksheet tracks first-order law, while this file tracks which
  higher-order sidecars actually cross the zone-facing seams.
- Generic named-zone attachment evidence is distinct from RFC 9557 IXDTF
  annotation evidence. `ZonedTimestampEvidence` remains specific to IXDTF
  suffix-bearing strings, while `NamedZoneAttachmentEvidence` covers backend-
  neutral attachment of a named zone to a fixed instant.
- Zone-transition ambiguity and gap branches now carry their dedicated
  evidence bundles so the branch tells consumers not only which edge case
  happened, but also which standard-law family established it.

## Remaining Checklist

- [x] Separate generic named-zone identity and attachment proofs from IXDTF-specific suffix evidence.
- [x] Ensure local-to-zone resolution consumes declared authority and re-issues it on the result side.
- [x] Ensure ambiguity and gap branches carry explicit evidence bundles, not only aggregate propositions.
- [x] Give every zone-facing result shape a stable alias instead of anonymous tuples.
