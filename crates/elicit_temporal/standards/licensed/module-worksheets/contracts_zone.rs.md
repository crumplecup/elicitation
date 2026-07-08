# Citation Worksheet: `src/contracts/zone.rs`

This worksheet stages exhaustive coverage review for the mixed-authority
named-zone surface in `src/contracts/zone.rs`.

## Source set

- `../../public/rfc9557.txt`
- `../../public/tzdb-theory.html`
- BCP 175 governance context for the IANA Time Zone Database, where useful

## Zone contract map

| Contract symbol | Current topic label | Exact section | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `NamedTimeZoneAnnotationPresent` | `named time-zone annotation presence` | `RFC 9557 §4.1` | `../../public/rfc9557.txt lines 529-589` | `yes` |
| `NamedTimeZoneUsesIanaIdentifier` | `IANA time-zone identifier` | `RFC 9557 §1.2; §4.1` | `../../public/rfc9557.txt lines 235-245; 575-579` | `yes` |
| `NamedTimeZoneIdentifierExcludesDotSegments` | `named time-zone identifier excludes "." and ".." segments` | `RFC 9557 §4.1` | `../../public/rfc9557.txt lines 572-573` | `yes` |
| `OffsetTimeZoneAnnotationPresent` | `offset time-zone annotation presence` | `RFC 9557 §1.2; §4.1` | `../../public/rfc9557.txt lines 247-266; 529-560` | `yes` |
| `NamedTimeZoneIdentifierIsCaseSensitive` | `case sensitivity of named time-zone identifiers` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 235-245; ../../public/tzdb-theory.html lines 211-214` | `yes` |
| `NamedTimeZoneIsNotNumericOffsetAlias` | `named zone not equivalent to bare numeric offset` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 219-266` | `yes` |
| `NumericOffsetDoesNotIdentifyNamedZone` | `numeric offset does not identify named zone` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 219-266` | `yes` |
| `OffsetTimeZoneRepeatsTimestampOffset` | `offset annotation repeats timestamp offset` | `RFC 9557 §1.2; §3.4` | `../../public/rfc9557.txt lines 247-255; 466-527` | `yes` |
| `OffsetTimeZoneUseIsStronglyDiscouraged` | `offset time-zone use discouraged` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 257-271` | `yes` |
| `OffsetTimeZoneMustNotBeSynthesizedFromTimestampOffset` | `forbid synthesized offset annotations` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 259-271` | `yes` |
| `NamedTimeZoneRetainsCivilRuleIdentity` | `named zone retains civil-rule identity` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 219-245; ../../public/tzdb-theory.html lines 104-112; 908-913` | `yes` |
| `ZoneOffsetResolvedForRepresentedInstant` | `zone rules resolve offset for represented instant` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 219-232` | `yes` |
| `NamedTimeZoneMeaningUsesCurrentTzdbRules` | `named zone meaning tracks current TZDB rules` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 241-245; ../../public/tzdb-theory.html lines 104-112` | `yes` |
| `UnknownNamedTimeZoneIdentifierTreatedAsInconsistency` | `unknown named zone treated as inconsistency` | `RFC 9557 §4.1` | `../../public/rfc9557.txt lines 575-579` | `yes` |
| `CriticalTimeZoneSuffixInconsistencyRequiresAction` | `critical time-zone inconsistency requires action` | `RFC 9557 §3.4` | `../../public/rfc9557.txt lines 481-487` | `yes` |
| `ElectiveTimeZoneSuffixInconsistencyMayBeHandled` | `elective time-zone inconsistency may be handled` | `RFC 9557 §3.4` | `../../public/rfc9557.txt lines 481-487` | `yes` |
| `ZuluTimeZoneSuffixAvoidsOffsetInconsistency` | `Z-based timestamp avoids offset inconsistency with time-zone suffix` | `RFC 9557 §2.2; §3.4` | `../../public/rfc9557.txt lines 304-316; 504-516` | `yes` |
| `ZoneTransitionAmbiguityDeclared` | `transition ambiguity declaration` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 221-223` | `yes` |
| `ZoneTransitionGapDeclared` | `transition gap declaration` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 221-223` | `yes` |
| `ZoneTransitionDisambiguationAuthorityDeclared` | `ambiguity disambiguation authority` | `RFC 9557 §1.1; §3.4` | `../../public/rfc9557.txt lines 149-162; 483-487` | `yes` |
| `ZoneTransitionGapHandlingAuthorityDeclared` | `gap-handling authority` | `RFC 9557 §1.1; §3.4` | `../../public/rfc9557.txt lines 149-162; 483-487` | `yes` |

## RFC 9557 Zone Coverage Audit

| Section | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `1.1 Scope` | `covered` | `ZoneTransitionDisambiguationAuthorityDeclared`, `ZoneTransitionGapHandlingAuthorityDeclared`, `ZoneTransitionResolutionAuthorityEvidence`, `LocalTimeZoneResolutionProofBranch` | Out-of-band resolution authority is now explicit both as first-order propositions and as exchange-carried proof composition for ambiguous and skipped local times. |
| `1.2 Definitions` | `covered` | `NamedTimeZoneUsesIanaIdentifier`, `OffsetTimeZoneAnnotationPresent`, `NamedTimeZoneRetainsCivilRuleIdentity`, `ZoneOffsetResolvedForRepresentedInstant`, `NamedTimeZoneMeaningUsesCurrentTzdbRules`, transition gap and ambiguity contracts | This is now the primary RFC anchor for named-zone ontology and local-time ambiguity semantics. |
| `3.4 Inconsistent time-offset and Time Zone Information` | `covered` | `OffsetTimeZoneRepeatsTimestampOffset`, `CriticalTimeZoneSuffixInconsistencyRequiresAction`, `ElectiveTimeZoneSuffixInconsistencyMayBeHandled`, `ZuluTimeZoneSuffixAvoidsOffsetInconsistency`, `UnknownNamedTimeZoneIdentifierTreatedAsInconsistency`, `ZoneTransitionDisambiguationAuthorityDeclared`, `ZoneTransitionGapHandlingAuthorityDeclared` | The `MUST` versus `MAY` inconsistency split and the `Z` non-inconsistency case are now explicit rather than inferred from generic suffix rules. |
| `4.1 ABNF` | `covered` | `NamedTimeZoneAnnotationPresent`, `NamedTimeZoneUsesIanaIdentifier`, `NamedTimeZoneIdentifierExcludesDotSegments`, `OffsetTimeZoneAnnotationPresent`, `UnknownNamedTimeZoneIdentifierTreatedAsInconsistency` | The special-case time-zone syntax now has exact published anchors, including the explicit `"."` and `".."` exclusion. |
| `BCP 175 / TZDB operational semantics` | `covered` | `NamedTimeZoneIdentifierIsCaseSensitive`, `NamedTimeZoneRetainsCivilRuleIdentity`, `NamedTimeZoneMeaningUsesCurrentTzdbRules` | Exact public TZDB operational text is now staged locally via `../../public/tzdb-theory.html`. |

## Notes

- This file is intentionally mixed-authority: RFC 9557 is normative for IXDTF
  named-zone annotation semantics, while TZDB and BCP material act as
  operational cross-checks where named-zone meaning depends on rule evolution.
- The BCP 175 info page is useful mainly as governance context pointing to
  RFC 6557; the exact public operational wording used for proposition
  concordance in this worksheet now comes from `../../public/tzdb-theory.html`.
- The mixed worksheet should remain unified: the TZDB/BCP material in this
  surface does not currently introduce an independent exchange family, only
  informative cross-checks on the same named-zone propositions.
- The main fidelity improvement here is shifting from draft-era `§3.1` blanket
  citations to the actual published RFC 9557 sections that carry the rule.
- Zone-transition ambiguity and gap semantics are not confined to `zone.rs`;
  they are intentionally mirrored across `src/contracts/instant.rs`,
  `src/contracts/proof_composition.rs`, and the `LocalTimeZoneResolutionProofBranch`
  exchange surface in `src/types.rs`.
- RFC 9557 `§3.4` now has dedicated first-order propositions for critical
  inconsistency handling, elective inconsistency handling, and the `Z`-based
  non-inconsistency path, instead of relying only on the generic
  critical/elective suffix consumption laws in `rfc9557.rs`.

## Remaining Coverage Checklist

- [x] Stage the public RFC 9557 source text locally for stable worksheet references.
- [x] Re-anchor the worksheet to the published RFC 9557 sections instead of blanket `§3.1` citations.
- [x] Decide whether the mixed RFC and TZDB surface should remain in one worksheet or split into separate RFC and operational-semantics worksheets.
- [x] Stage public TZDB or BCP source text locally if exact proposition-to-source concordance is required for non-RFC zone behavior.
- [x] Audit whether zone-transition ambiguity and gap semantics belong solely in `zone.rs` or also need mirrored contracts in adjacent instant and proof-composition surfaces.
