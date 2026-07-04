# Citation Worksheet: `src/contracts/rfc9557.rs`

This worksheet stages exhaustive coverage review for the RFC 9557 contract
surface in `src/contracts/rfc9557.rs`.

## Source set

- `../../public/rfc9557.txt`

## RFC contract map

| Contract symbol | Current topic label | Exact section | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `IxdtfSuffixFollowsRfc3339Timestamp` | `IXDTF suffix follows RFC 3339 timestamp` | `RFC 9557 §4.1` | `../../public/rfc9557.txt lines 529-589` | `yes` |
| `IxdtfTimeZoneSuffixUsesBracketedNameOrOffset` | `time-zone annotation syntax` | `RFC 9557 §4.1` | `../../public/rfc9557.txt lines 529-589` | `yes` |
| `IxdtfCriticalFlagIsLeadingExclamationWhenPresent` | `critical flag syntax` | `RFC 9557 §3.3; §4.1` | `../../public/rfc9557.txt lines 386-464; 529-589` | `yes` |
| `IxdtfSuffixKeysAreLowercase` | `lowercase additional-information keys` | `RFC 9557 §3.1` | `../../public/rfc9557.txt lines 337-353` | `yes` |
| `IxdtfSuffixTagsUseBracketedKeyValueForm` | `bracketed key-value suffix tags` | `RFC 9557 §3.1; §4.1` | `../../public/rfc9557.txt lines 337-353; 529-589` | `yes` |
| `IxdtfSuffixValuesUseHyphenDelimitedItems` | `hyphen-delimited suffix values` | `RFC 9557 §3.1; §4.1` | `../../public/rfc9557.txt lines 337-353; 529-589` | `yes` |
| `IxdtfSuffixValuesAreCaseSensitiveUnlessOtherwiseSpecified` | `suffix value case sensitivity` | `RFC 9557 §3.1` | `../../public/rfc9557.txt lines 337-353` | `yes` |
| `IxdtfGeneratorsMayOmitSuffixTags` | `optional generation of suffix tags` | `RFC 9557 §3.3` | `../../public/rfc9557.txt lines 386-464` | `yes` |
| `IxdtfRecipientsMayIgnoreElectiveSuffixTags` | `elective suffix consumption` | `RFC 9557 §3.3` | `../../public/rfc9557.txt lines 386-464` | `yes` |
| `IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling` | `critical suffix consumption rule` | `RFC 9557 §3.3` | `../../public/rfc9557.txt lines 386-464` | `yes` |
| `IxdtfDuplicateElectiveSuffixUsesFirstOccurrence` | `duplicate elective suffix first-wins rule` | `RFC 9557 §3.3` | `../../public/rfc9557.txt lines 456-464` | `yes` |
| `IxdtfExperimentalSuffixKeysUseLeadingUnderscore` | `experimental suffix key syntax` | `RFC 9557 §3.2; §4.1` | `../../public/rfc9557.txt lines 355-384; 529-589` | `yes` |
| `IxdtfExperimentalSuffixKeysCannotBeRegistered` | `experimental suffix key registration prohibition` | `RFC 9557 §3.2` | `../../public/rfc9557.txt lines 378-384` | `yes` |
| `IxdtfExperimentalSuffixKeysAreNotForInterchange` | `experimental suffix interchange restriction` | `RFC 9557 §3.2` | `../../public/rfc9557.txt lines 378-384` | `yes` |
| `IxdtfRecipientsMustRejectUnconfiguredExperimentalSuffixKeys` | `experimental suffix rejection rule` | `RFC 9557 §3.2` | `../../public/rfc9557.txt lines 378-384` | `yes` |
| `IxdtfCalendarAnnotationPresent` | `calendar-awareness annotation presence` | `RFC 9557 §5` | `../../public/rfc9557.txt lines 631-641` | `yes` |
| `IxdtfCalendarKeyUsesUCa` | `u-ca key` | `RFC 9557 §5` | `../../public/rfc9557.txt lines 631-641` | `yes` |
| `IxdtfCalendarValueUsesUnicodeCalendarIdentifier` | `Unicode calendar identifier value` | `RFC 9557 §5` | `../../public/rfc9557.txt lines 631-641` | `yes` |
| `IxdtfCalendarAnnotationDeclaresPreferredPresentationCalendar` | `preferred presentation calendar semantics` | `RFC 9557 §5` | `../../public/rfc9557.txt lines 631-641` | `yes` |

## RFC 9557 Section Coverage Audit

| Section | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `1.1 Scope` | `covered elsewhere` | `IxdtfGeneratorsMayOmitSuffixTags`, `IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling`, `zone.rs` inconsistency contracts | Scope-level non-goals and out-of-band resolution semantics are split across `rfc9557.rs` and `zone.rs`. |
| `1.2 Definitions` | `covered elsewhere` | `zone.rs` named-zone and offset-zone contracts | Keep time-zone ontology in `zone.rs`, not the generic suffix-key module. |
| `2.2 Update to RFC 3339` | `gap` | `Rfc3339UnknownLocalOffsetUsesNegativeZero`, `Rfc3339LocalOffsetNotUnknown` in `rfc3339.rs` | Replace or quarantine the pre-update RFC 3339 unknown-offset law so the profile surface matches the updated standard stack. |
| `3.1 Format of Extended Information` | `covered` | `IxdtfSuffixKeysAreLowercase`, `IxdtfSuffixTagsUseBracketedKeyValueForm`, `IxdtfSuffixValuesUseHyphenDelimitedItems`, `IxdtfSuffixValuesAreCaseSensitiveUnlessOtherwiseSpecified` | No additional generic suffix proposition is currently required. |
| `3.2 Registering Keys for Extended Information Tags` | `partially covered` | `IxdtfExperimentalSuffixKeysUseLeadingUnderscore`, `IxdtfExperimentalSuffixKeysCannotBeRegistered`, `IxdtfExperimentalSuffixKeysAreNotForInterchange`, `IxdtfRecipientsMustRejectUnconfiguredExperimentalSuffixKeys` | Registry metadata fields and permanent-versus-provisional status remain unstaged. |
| `3.3 Optional Generation and Elective vs. Critical Consumption` | `covered` | `IxdtfGeneratorsMayOmitSuffixTags`, `IxdtfRecipientsMayIgnoreElectiveSuffixTags`, `IxdtfCriticalFlagIsLeadingExclamationWhenPresent`, `IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling`, `IxdtfDuplicateElectiveSuffixUsesFirstOccurrence` | Zone-specific inconsistency handling remains intentionally split into `zone.rs`. |
| `3.4 Inconsistent time-offset and Time Zone Information` | `covered elsewhere` | `zone.rs` inconsistency and authority contracts | Keep time-zone inconsistency handling in the zone worksheet instead of duplicating it here. |
| `4.1 ABNF` | `covered` | `IxdtfSuffixFollowsRfc3339Timestamp`, `IxdtfTimeZoneSuffixUsesBracketedNameOrOffset`, `IxdtfCriticalFlagIsLeadingExclamationWhenPresent`, `IxdtfExperimentalSuffixKeysUseLeadingUnderscore` | Decide later whether `time-zone-part` exclusion of `.` and `..` needs a first-class parser law. |
| `5 The u-ca Suffix Key: Calendar Awareness` | `covered` | `IxdtfCalendarAnnotationPresent`, `IxdtfCalendarKeyUsesUCa`, `IxdtfCalendarValueUsesUnicodeCalendarIdentifier`, `IxdtfCalendarAnnotationDeclaresPreferredPresentationCalendar` | No additional calendar-awareness proposition is currently required. |
| `6 IANA Considerations` | `partially covered` | experimental-key and `u-ca` allocation surface | Registry policy and expert-review mechanics remain unstaged until we need a first-class registry contract family. |

## Notes

- This worksheet now tracks the published RFC 9557 section structure rather
  than draft-era heading names.
- The public RFC text is staged locally under `standards/public`.

## Remaining Coverage Checklist

- [x] Stage the public RFC 9557 source text locally for stable worksheet references.
- [x] Audit RFC 9557 section-by-section and mark which substantive clauses are intentionally represented in `rfc9557.rs` versus `zone.rs`.
- [x] Add first-class contracts for duplicate elective suffix handling and experimental-key registration prohibition.
- [ ] Reconcile the RFC 9557 update to RFC 3339 Section `4.3` with the existing `rfc3339.rs` surface.
- [ ] Decide whether registry metadata fields and registry policy need first-class contracts beyond the currently mined suffix-key rules.
- [ ] Decide whether the ABNF exclusion of `.` and `..` in `time-zone-part` needs a parser-facing contract.
