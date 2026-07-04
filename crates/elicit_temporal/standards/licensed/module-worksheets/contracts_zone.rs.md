# Citation Worksheet: `src/contracts/zone.rs`

This worksheet stages exhaustive coverage review for the mixed-authority
named-zone surface in `src/contracts/zone.rs`.

## Source set

- `../../public/rfc9557.txt`
- BCP 175 / IANA Time Zone Database semantics, where named-zone operational
  meaning matters

## Zone contract map

| Contract symbol | Current topic label | Exact section | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `NamedTimeZoneAnnotationPresent` | `named time-zone annotation presence` | `RFC 9557 §4.1` | `../../public/rfc9557.txt lines 529-589` | `yes` |
| `NamedTimeZoneUsesIanaIdentifier` | `IANA time-zone identifier` | `RFC 9557 §1.2; §4.1` | `../../public/rfc9557.txt lines 235-245; 575-579` | `yes` |
| `OffsetTimeZoneAnnotationPresent` | `offset time-zone annotation presence` | `RFC 9557 §1.2; §4.1` | `../../public/rfc9557.txt lines 247-266; 529-560` | `yes` |
| `NamedTimeZoneIdentifierIsCaseSensitive` | `case sensitivity of named time-zone identifiers` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 235-245; BCP 175 / IANA TZDB naming semantics cross-check` | `yes` |
| `NamedTimeZoneIsNotNumericOffsetAlias` | `named zone not equivalent to bare numeric offset` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 219-266` | `yes` |
| `NumericOffsetDoesNotIdentifyNamedZone` | `numeric offset does not identify named zone` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 219-266` | `yes` |
| `OffsetTimeZoneRepeatsTimestampOffset` | `offset annotation repeats timestamp offset` | `RFC 9557 §1.2; §3.4` | `../../public/rfc9557.txt lines 247-255; 466-527` | `yes` |
| `OffsetTimeZoneUseIsStronglyDiscouraged` | `offset time-zone use discouraged` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 257-271` | `yes` |
| `OffsetTimeZoneMustNotBeSynthesizedFromTimestampOffset` | `forbid synthesized offset annotations` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 259-271` | `yes` |
| `NamedTimeZoneRetainsCivilRuleIdentity` | `named zone retains civil-rule identity` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 219-245; BCP 175 / IANA TZDB semantics cross-check` | `yes` |
| `ZoneOffsetResolvedForRepresentedInstant` | `zone rules resolve offset for represented instant` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 219-232` | `yes` |
| `NamedTimeZoneMeaningUsesCurrentTzdbRules` | `named zone meaning tracks current TZDB rules` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 241-245` | `yes` |
| `UnknownNamedTimeZoneIdentifierTreatedAsInconsistency` | `unknown named zone treated as inconsistency` | `RFC 9557 §4.1` | `../../public/rfc9557.txt lines 575-579` | `yes` |
| `ZoneTransitionAmbiguityDeclared` | `transition ambiguity declaration` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 221-223` | `yes` |
| `ZoneTransitionGapDeclared` | `transition gap declaration` | `RFC 9557 §1.2` | `../../public/rfc9557.txt lines 221-223` | `yes` |
| `ZoneTransitionDisambiguationAuthorityDeclared` | `ambiguity disambiguation authority` | `RFC 9557 §1.1; §3.4` | `../../public/rfc9557.txt lines 149-162; 483-487` | `yes` |
| `ZoneTransitionGapHandlingAuthorityDeclared` | `gap-handling authority` | `RFC 9557 §1.1; §3.4` | `../../public/rfc9557.txt lines 149-162; 483-487` | `yes` |

## RFC 9557 Zone Coverage Audit

| Section | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `1.1 Scope` | `partially covered` | `ZoneTransitionDisambiguationAuthorityDeclared`, `ZoneTransitionGapHandlingAuthorityDeclared` | The out-of-band resolution language is represented, but still somewhat interpretive at the proposition level. |
| `1.2 Definitions` | `covered` | `NamedTimeZoneUsesIanaIdentifier`, `OffsetTimeZoneAnnotationPresent`, `NamedTimeZoneRetainsCivilRuleIdentity`, `ZoneOffsetResolvedForRepresentedInstant`, `NamedTimeZoneMeaningUsesCurrentTzdbRules`, transition gap and ambiguity contracts | This is now the primary RFC anchor for named-zone ontology and local-time ambiguity semantics. |
| `3.4 Inconsistent time-offset and Time Zone Information` | `covered` | `OffsetTimeZoneRepeatsTimestampOffset`, `UnknownNamedTimeZoneIdentifierTreatedAsInconsistency`, `ZoneTransitionDisambiguationAuthorityDeclared`, `ZoneTransitionGapHandlingAuthorityDeclared` | Revisit if we split inconsistency handling into a dedicated contract module. |
| `4.1 ABNF` | `covered` | `NamedTimeZoneAnnotationPresent`, `NamedTimeZoneUsesIanaIdentifier`, `OffsetTimeZoneAnnotationPresent`, `UnknownNamedTimeZoneIdentifierTreatedAsInconsistency` | The special-case time-zone syntax now has exact published anchors. |
| `BCP 175 / TZDB operational semantics` | `partially covered` | `NamedTimeZoneIdentifierIsCaseSensitive`, `NamedTimeZoneRetainsCivilRuleIdentity`, `NamedTimeZoneMeaningUsesCurrentTzdbRules` | Stage local BCP 175 or TZDB naming text if we need exact non-RFC line-based concordance for operational cross-checks. |

## Notes

- This file is intentionally mixed-authority: RFC 9557 is normative for IXDTF
  named-zone annotation semantics, while TZDB and BCP material act as
  operational cross-checks where named-zone meaning depends on rule evolution.
- The main fidelity improvement here is shifting from draft-era `§3.1` blanket
  citations to the actual published RFC 9557 sections that carry the rule.

## Remaining Coverage Checklist

- [x] Stage the public RFC 9557 source text locally for stable worksheet references.
- [x] Re-anchor the worksheet to the published RFC 9557 sections instead of blanket `§3.1` citations.
- [ ] Decide whether the mixed RFC and TZDB surface should remain in one worksheet or split into separate RFC and operational-semantics worksheets.
- [ ] Stage public TZDB or BCP source text locally if exact proposition-to-source concordance is required for non-RFC zone behavior.
- [ ] Audit whether zone-transition ambiguity and gap semantics belong solely in `zone.rs` or also need mirrored contracts in adjacent instant and proof-composition surfaces.
