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
| `IxdtfRegisteredSuffixKeyCarriesKeyIdentifier` | `registry key-identifier field` | `RFC 9557 §3.2` | `../../public/rfc9557.txt lines 362-362` | `yes` |
| `IxdtfRegisteredSuffixKeyCarriesRegistrationStatus` | `registry registration-status field` | `RFC 9557 §3.2` | `../../public/rfc9557.txt lines 364-364` | `yes` |
| `IxdtfRegisteredSuffixKeyStatusIsProvisionalOrPermanent` | `registry status domain` | `RFC 9557 §3.2` | `../../public/rfc9557.txt lines 364-364` | `yes` |
| `IxdtfRegisteredSuffixKeyCarriesDescription` | `registry description field` | `RFC 9557 §3.2` | `../../public/rfc9557.txt lines 366-366` | `yes` |
| `IxdtfRegisteredSuffixKeyCarriesChangeController` | `registry change-controller field` | `RFC 9557 §3.2` | `../../public/rfc9557.txt lines 368-371` | `yes` |
| `IxdtfRegisteredSuffixKeyCarriesReference` | `registry reference field` | `RFC 9557 §3.2` | `../../public/rfc9557.txt lines 373-377` | `yes` |
| `IxdtfPermanentRegisteredSuffixKeyRequiresFullSpecificationReference` | `permanent entry reference requirement` | `RFC 9557 §3.2` | `../../public/rfc9557.txt lines 373-374` | `yes` |
| `IxdtfProvisionalRegisteredSuffixKeyRequiresReferenceInformation` | `provisional entry minimum reference requirement` | `RFC 9557 §3.2` | `../../public/rfc9557.txt lines 374-376` | `yes` |
| `IxdtfProvisionalRegisteredSuffixKeyReferenceExpectedToImproveOverTime` | `provisional entry reference-improvement expectation` | `RFC 9557 §3.2` | `../../public/rfc9557.txt lines 375-377` | `yes` |
| `IxdtfCalendarAnnotationPresent` | `calendar-awareness annotation presence` | `RFC 9557 §5` | `../../public/rfc9557.txt lines 631-641` | `yes` |
| `IxdtfCalendarKeyUsesUCa` | `u-ca key` | `RFC 9557 §5` | `../../public/rfc9557.txt lines 631-641` | `yes` |
| `IxdtfCalendarValueUsesUnicodeCalendarIdentifier` | `Unicode calendar identifier value` | `RFC 9557 §5` | `../../public/rfc9557.txt lines 631-641` | `yes` |
| `IxdtfCalendarAnnotationDeclaresPreferredPresentationCalendar` | `preferred presentation calendar semantics` | `RFC 9557 §5` | `../../public/rfc9557.txt lines 631-641` | `yes` |
| `IxdtfRegistryInitiallyContainsUCaEntry` | `initial registry contents include u-ca` | `RFC 9557 §6` | `../../public/rfc9557.txt lines 648-660` | `yes` |
| `IxdtfUCaRegistryEntryIsPermanent` | `u-ca registration status` | `RFC 9557 §6` | `../../public/rfc9557.txt lines 655-657` | `yes` |
| `IxdtfUCaRegistryEntryUsesPreferredCalendarForPresentationDescription` | `u-ca registry description` | `RFC 9557 §6` | `../../public/rfc9557.txt lines 655-657` | `yes` |
| `IxdtfUCaRegistryEntryUsesIetfChangeController` | `u-ca change controller` | `RFC 9557 §6` | `../../public/rfc9557.txt lines 655-657` | `yes` |
| `IxdtfUCaRegistryEntryReferencesSectionFive` | `u-ca registry reference` | `RFC 9557 §6` | `../../public/rfc9557.txt lines 655-657` | `yes` |
| `IxdtfPermanentEntriesUseSpecificationRequiredPolicy` | `permanent-entry registration policy` | `RFC 9557 §6` | `../../public/rfc9557.txt lines 662-663` | `yes` |
| `IxdtfProvisionalEntriesUseExpertReviewPolicy` | `provisional-entry registration policy` | `RFC 9557 §6` | `../../public/rfc9557.txt lines 662-663` | `yes` |
| `IxdtfExpertReviewAscertainsBasicSpecificationExists` | `expert review basic-specification check` | `RFC 9557 §6` | `../../public/rfc9557.txt lines 663-665` | `yes` |
| `IxdtfExpertReviewReservesConciseGenerallyApplicableKeys` | `frugal concise-key allocation` | `RFC 9557 §6` | `../../public/rfc9557.txt lines 667-670` | `yes` |
| `IxdtfExpertsMayInitiateRegistrationToAvoidFutureCollisions` | `expert-initiated collision-avoidance registration` | `RFC 9557 §6` | `../../public/rfc9557.txt lines 670-673` | `yes` |

## RFC 9557 Section Coverage Audit

| Section | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `1.1 Scope` | `covered elsewhere` | `IxdtfGeneratorsMayOmitSuffixTags`, `IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling`, `zone.rs` inconsistency contracts | Scope-level non-goals and out-of-band resolution semantics are split across `rfc9557.rs` and `zone.rs`. |
| `1.2 Definitions` | `covered elsewhere` | `zone.rs` named-zone and offset-zone contracts | Keep time-zone ontology in `zone.rs`, not the generic suffix-key module. |
| `2.2 Update to RFC 3339` | `covered elsewhere` | `Rfc3339UnknownLocalOffsetUsesZuluDesignator`, `Rfc3339LocalOffsetNotUnknown`, `Rfc3339PositiveZeroOffsetDeclaresPreferredUtcReferencePoint` in `rfc3339.rs` | RFC 3339 Section `4.3` is now tracked in its RFC 9557-updated form in the RFC 3339 worksheet and contract surface. |
| `3.1 Format of Extended Information` | `covered` | `IxdtfSuffixKeysAreLowercase`, `IxdtfSuffixTagsUseBracketedKeyValueForm`, `IxdtfSuffixValuesUseHyphenDelimitedItems`, `IxdtfSuffixValuesAreCaseSensitiveUnlessOtherwiseSpecified` | No additional generic suffix proposition is currently required. |
| `3.2 Registering Keys for Extended Information Tags` | `covered` | `IxdtfRegisteredSuffixKeyCarriesKeyIdentifier`, `IxdtfRegisteredSuffixKeyCarriesRegistrationStatus`, `IxdtfRegisteredSuffixKeyStatusIsProvisionalOrPermanent`, `IxdtfRegisteredSuffixKeyCarriesDescription`, `IxdtfRegisteredSuffixKeyCarriesChangeController`, `IxdtfRegisteredSuffixKeyCarriesReference`, `IxdtfPermanentRegisteredSuffixKeyRequiresFullSpecificationReference`, `IxdtfProvisionalRegisteredSuffixKeyRequiresReferenceInformation`, `IxdtfProvisionalRegisteredSuffixKeyReferenceExpectedToImproveOverTime`, plus experimental-key contracts | Section 3.2 field schema, status split, and experimental-key restrictions are now all explicit. |
| `3.3 Optional Generation and Elective vs. Critical Consumption` | `covered` | `IxdtfGeneratorsMayOmitSuffixTags`, `IxdtfRecipientsMayIgnoreElectiveSuffixTags`, `IxdtfCriticalFlagIsLeadingExclamationWhenPresent`, `IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling`, `IxdtfDuplicateElectiveSuffixUsesFirstOccurrence` | Zone-specific inconsistency handling remains intentionally split into `zone.rs`. |
| `3.4 Inconsistent time-offset and Time Zone Information` | `covered elsewhere` | `zone.rs` inconsistency and authority contracts | Keep time-zone inconsistency handling in the zone worksheet instead of duplicating it here. |
| `4.1 ABNF` | `covered` | `IxdtfSuffixFollowsRfc3339Timestamp`, `IxdtfTimeZoneSuffixUsesBracketedNameOrOffset`, `IxdtfCriticalFlagIsLeadingExclamationWhenPresent`, `IxdtfExperimentalSuffixKeysUseLeadingUnderscore`, `NamedTimeZoneIdentifierExcludesDotSegments` in `zone.rs` | The `time-zone-part` exclusion of `.` and `..` is intentionally tracked in the zone-facing worksheet and contract surface. |
| `5 The u-ca Suffix Key: Calendar Awareness` | `covered` | `IxdtfCalendarAnnotationPresent`, `IxdtfCalendarKeyUsesUCa`, `IxdtfCalendarValueUsesUnicodeCalendarIdentifier`, `IxdtfCalendarAnnotationDeclaresPreferredPresentationCalendar` | No additional calendar-awareness proposition is currently required. |
| `6 IANA Considerations` | `covered` | `IxdtfRegistryInitiallyContainsUCaEntry`, `IxdtfUCaRegistryEntryIsPermanent`, `IxdtfUCaRegistryEntryUsesPreferredCalendarForPresentationDescription`, `IxdtfUCaRegistryEntryUsesIetfChangeController`, `IxdtfUCaRegistryEntryReferencesSectionFive`, `IxdtfPermanentEntriesUseSpecificationRequiredPolicy`, `IxdtfProvisionalEntriesUseExpertReviewPolicy`, `IxdtfExpertReviewAscertainsBasicSpecificationExists`, `IxdtfExpertReviewReservesConciseGenerallyApplicableKeys`, `IxdtfExpertsMayInitiateRegistrationToAvoidFutureCollisions` | Initial registry contents and policy mechanics are now first-class. |

## Notes

- This worksheet now tracks the published RFC 9557 section structure rather
  than draft-era heading names.
- The public RFC text is staged locally under `standards/public`.
- Registry-entry field semantics now live in `rfc9557.rs` as a first-class
  contract family rather than being left as worksheet-only audit debt.

## Remaining Coverage Checklist

- [x] Stage the public RFC 9557 source text locally for stable worksheet references.
- [x] Audit RFC 9557 section-by-section and mark which substantive clauses are intentionally represented in `rfc9557.rs` versus `zone.rs`.
- [x] Add first-class contracts for duplicate elective suffix handling and experimental-key registration prohibition.
- [x] Reconcile the RFC 9557 update to RFC 3339 Section `4.3` with the existing `rfc3339.rs` surface.
- [x] Decide whether registry metadata fields and registry policy need first-class contracts beyond the currently mined suffix-key rules.
- [x] Decide whether the ABNF exclusion of `.` and `..` in `time-zone-part` needs a parser-facing contract.
