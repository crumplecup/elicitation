//! RFC 9557 IXDTF propositions.
//!
//! Normative source: RFC 9557, *Date and Time on the Internet: Timestamps with additional information*.
//! All section references are to RFC 9557.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — RFC 9557 contract */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — RFC 9557 contract */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — RFC 9557 contract */ }
                }
            }
        };
    }

    /// An IXDTF timestamp appends additional information to an RFC 3339 timestamp.
    ///
    /// Normative source: RFC 9557 §4.1 — ABNF
    pub struct IxdtfSuffixFollowsRfc3339Timestamp;
    structural_prop!(
        IxdtfSuffixFollowsRfc3339Timestamp,
        "IxdtfSuffixFollowsRfc3339Timestamp"
    );

    /// A time-zone annotation uses bracketed content naming a zone or offset identifier.
    ///
    /// Normative source: RFC 9557 §4.1 — ABNF
    pub struct IxdtfTimeZoneSuffixUsesBracketedNameOrOffset;
    structural_prop!(
        IxdtfTimeZoneSuffixUsesBracketedNameOrOffset,
        "IxdtfTimeZoneSuffixUsesBracketedNameOrOffset"
    );

    /// A critical annotation, when present, begins with `!`.
    ///
    /// Normative sources: RFC 9557 §3.3 — Optional Generation and Elective vs. Critical Consumption; §4.1 — ABNF
    pub struct IxdtfCriticalFlagIsLeadingExclamationWhenPresent;
    structural_prop!(
        IxdtfCriticalFlagIsLeadingExclamationWhenPresent,
        "IxdtfCriticalFlagIsLeadingExclamationWhenPresent"
    );

    /// Additional-information keys are lowercase.
    ///
    /// Normative source: RFC 9557 §3.1 — Format of Extended Information
    pub struct IxdtfSuffixKeysAreLowercase;
    structural_prop!(IxdtfSuffixKeysAreLowercase, "IxdtfSuffixKeysAreLowercase");

    /// A suffix tag uses bracketed key-value form with `=` between key and value.
    ///
    /// Normative sources: RFC 9557 §3.1 — Format of Extended Information; §4.1 — ABNF
    pub struct IxdtfSuffixTagsUseBracketedKeyValueForm;
    structural_prop!(
        IxdtfSuffixTagsUseBracketedKeyValueForm,
        "IxdtfSuffixTagsUseBracketedKeyValueForm"
    );

    /// A suffix value uses one or more hyphen-delimited items.
    ///
    /// Normative sources: RFC 9557 §3.1 — Format of Extended Information; §4.1 — ABNF
    pub struct IxdtfSuffixValuesUseHyphenDelimitedItems;
    structural_prop!(
        IxdtfSuffixValuesUseHyphenDelimitedItems,
        "IxdtfSuffixValuesUseHyphenDelimitedItems"
    );

    /// Suffix values are case-sensitive unless a key specification says otherwise.
    ///
    /// Normative source: RFC 9557 §3.1 — Format of Extended Information
    pub struct IxdtfSuffixValuesAreCaseSensitiveUnlessOtherwiseSpecified;
    structural_prop!(
        IxdtfSuffixValuesAreCaseSensitiveUnlessOtherwiseSpecified,
        "IxdtfSuffixValuesAreCaseSensitiveUnlessOtherwiseSpecified"
    );

    /// Generators may omit suffix tags entirely.
    ///
    /// Normative source: RFC 9557 §3.3 — Optional Generation and Elective vs. Critical Consumption
    pub struct IxdtfGeneratorsMayOmitSuffixTags;
    structural_prop!(
        IxdtfGeneratorsMayOmitSuffixTags,
        "IxdtfGeneratorsMayOmitSuffixTags"
    );

    /// Recipients may ignore elective suffix tags.
    ///
    /// Normative source: RFC 9557 §3.3 — Optional Generation and Elective vs. Critical Consumption
    pub struct IxdtfRecipientsMayIgnoreElectiveSuffixTags;
    structural_prop!(
        IxdtfRecipientsMayIgnoreElectiveSuffixTags,
        "IxdtfRecipientsMayIgnoreElectiveSuffixTags"
    );

    /// Critical suffix tags require processing or explicit error handling.
    ///
    /// Normative source: RFC 9557 §3.3 — Optional Generation and Elective vs. Critical Consumption
    pub struct IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling;
    structural_prop!(
        IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling,
        "IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling"
    );

    /// Duplicate elective suffix keys use the first occurrence when no additional inconsistency handling is performed.
    ///
    /// Normative source: RFC 9557 §3.3 — Optional Generation and Elective vs. Critical Consumption
    pub struct IxdtfDuplicateElectiveSuffixUsesFirstOccurrence;
    structural_prop!(
        IxdtfDuplicateElectiveSuffixUsesFirstOccurrence,
        "IxdtfDuplicateElectiveSuffixUsesFirstOccurrence"
    );

    /// Experimental suffix keys use a leading underscore.
    ///
    /// Normative sources: RFC 9557 §3.2 — Registering Keys for Extended Information Tags; §4.1 — ABNF
    pub struct IxdtfExperimentalSuffixKeysUseLeadingUnderscore;
    structural_prop!(
        IxdtfExperimentalSuffixKeysUseLeadingUnderscore,
        "IxdtfExperimentalSuffixKeysUseLeadingUnderscore"
    );

    /// Experimental suffix keys are not valid for general interchange.
    ///
    /// Normative source: RFC 9557 §3.2 — Registering Keys for Extended Information Tags
    pub struct IxdtfExperimentalSuffixKeysAreNotForInterchange;
    structural_prop!(
        IxdtfExperimentalSuffixKeysAreNotForInterchange,
        "IxdtfExperimentalSuffixKeysAreNotForInterchange"
    );

    /// Recipients must reject experimental suffix keys when not configured for the experiment.
    ///
    /// Normative source: RFC 9557 §3.2 — Registering Keys for Extended Information Tags
    pub struct IxdtfRecipientsMustRejectUnconfiguredExperimentalSuffixKeys;
    structural_prop!(
        IxdtfRecipientsMustRejectUnconfiguredExperimentalSuffixKeys,
        "IxdtfRecipientsMustRejectUnconfiguredExperimentalSuffixKeys"
    );

    /// Experimental suffix keys cannot be registered.
    ///
    /// Normative source: RFC 9557 §3.2 — Registering Keys for Extended Information Tags
    pub struct IxdtfExperimentalSuffixKeysCannotBeRegistered;
    structural_prop!(
        IxdtfExperimentalSuffixKeysCannotBeRegistered,
        "IxdtfExperimentalSuffixKeysCannotBeRegistered"
    );

    /// A registered suffix key carries a key identifier field.
    ///
    /// Normative source: RFC 9557 §3.2 — Registering Keys for Extended Information Tags
    pub struct IxdtfRegisteredSuffixKeyCarriesKeyIdentifier;
    structural_prop!(
        IxdtfRegisteredSuffixKeyCarriesKeyIdentifier,
        "IxdtfRegisteredSuffixKeyCarriesKeyIdentifier"
    );

    /// A registered suffix key carries a registration-status field.
    ///
    /// Normative source: RFC 9557 §3.2 — Registering Keys for Extended Information Tags
    pub struct IxdtfRegisteredSuffixKeyCarriesRegistrationStatus;
    structural_prop!(
        IxdtfRegisteredSuffixKeyCarriesRegistrationStatus,
        "IxdtfRegisteredSuffixKeyCarriesRegistrationStatus"
    );

    /// A registered suffix key uses either provisional or permanent registration status.
    ///
    /// Normative source: RFC 9557 §3.2 — Registering Keys for Extended Information Tags
    pub struct IxdtfRegisteredSuffixKeyStatusIsProvisionalOrPermanent;
    structural_prop!(
        IxdtfRegisteredSuffixKeyStatusIsProvisionalOrPermanent,
        "IxdtfRegisteredSuffixKeyStatusIsProvisionalOrPermanent"
    );

    /// A registered suffix key carries a short description field.
    ///
    /// Normative source: RFC 9557 §3.2 — Registering Keys for Extended Information Tags
    pub struct IxdtfRegisteredSuffixKeyCarriesDescription;
    structural_prop!(
        IxdtfRegisteredSuffixKeyCarriesDescription,
        "IxdtfRegisteredSuffixKeyCarriesDescription"
    );

    /// A registered suffix key carries a change-controller field.
    ///
    /// Normative source: RFC 9557 §3.2 — Registering Keys for Extended Information Tags
    pub struct IxdtfRegisteredSuffixKeyCarriesChangeController;
    structural_prop!(
        IxdtfRegisteredSuffixKeyCarriesChangeController,
        "IxdtfRegisteredSuffixKeyCarriesChangeController"
    );

    /// A registered suffix key carries a reference field.
    ///
    /// Normative source: RFC 9557 §3.2 — Registering Keys for Extended Information Tags
    pub struct IxdtfRegisteredSuffixKeyCarriesReference;
    structural_prop!(
        IxdtfRegisteredSuffixKeyCarriesReference,
        "IxdtfRegisteredSuffixKeyCarriesReference"
    );

    /// A permanent registered suffix key includes a full specification in its reference material.
    ///
    /// Normative source: RFC 9557 §3.2 — Registering Keys for Extended Information Tags
    pub struct IxdtfPermanentRegisteredSuffixKeyRequiresFullSpecificationReference;
    structural_prop!(
        IxdtfPermanentRegisteredSuffixKeyRequiresFullSpecificationReference,
        "IxdtfPermanentRegisteredSuffixKeyRequiresFullSpecificationReference"
    );

    /// A provisional registered suffix key carries at least some reference information.
    ///
    /// Normative source: RFC 9557 §3.2 — Registering Keys for Extended Information Tags
    pub struct IxdtfProvisionalRegisteredSuffixKeyRequiresReferenceInformation;
    structural_prop!(
        IxdtfProvisionalRegisteredSuffixKeyRequiresReferenceInformation,
        "IxdtfProvisionalRegisteredSuffixKeyRequiresReferenceInformation"
    );

    /// A provisional registered suffix key is expected to improve its reference information over time.
    ///
    /// Normative source: RFC 9557 §3.2 — Registering Keys for Extended Information Tags
    pub struct IxdtfProvisionalRegisteredSuffixKeyReferenceExpectedToImproveOverTime;
    structural_prop!(
        IxdtfProvisionalRegisteredSuffixKeyReferenceExpectedToImproveOverTime,
        "IxdtfProvisionalRegisteredSuffixKeyReferenceExpectedToImproveOverTime"
    );

    /// A calendar-awareness annotation is present in the IXDTF suffix.
    ///
    /// Normative source: RFC 9557 §5 — The u-ca Suffix Key: Calendar Awareness
    pub struct IxdtfCalendarAnnotationPresent;
    structural_prop!(
        IxdtfCalendarAnnotationPresent,
        "IxdtfCalendarAnnotationPresent"
    );

    /// The calendar-awareness suffix key is `u-ca`.
    ///
    /// Normative source: RFC 9557 §5 — The u-ca Suffix Key: Calendar Awareness
    pub struct IxdtfCalendarKeyUsesUCa;
    structural_prop!(IxdtfCalendarKeyUsesUCa, "IxdtfCalendarKeyUsesUCa");

    /// The calendar-awareness suffix value uses a Unicode Calendar Identifier.
    ///
    /// Normative source: RFC 9557 §5 — The u-ca Suffix Key: Calendar Awareness
    pub struct IxdtfCalendarValueUsesUnicodeCalendarIdentifier;
    structural_prop!(
        IxdtfCalendarValueUsesUnicodeCalendarIdentifier,
        "IxdtfCalendarValueUsesUnicodeCalendarIdentifier"
    );

    /// The calendar-awareness suffix declares the preferred calendar for presentation.
    ///
    /// Normative source: RFC 9557 §5 — The u-ca Suffix Key: Calendar Awareness
    pub struct IxdtfCalendarAnnotationDeclaresPreferredPresentationCalendar;
    structural_prop!(
        IxdtfCalendarAnnotationDeclaresPreferredPresentationCalendar,
        "IxdtfCalendarAnnotationDeclaresPreferredPresentationCalendar"
    );

    /// The initial Timestamp Suffix Tag Keys registry contains a `u-ca` entry.
    ///
    /// Normative source: RFC 9557 §6 — IANA Considerations
    pub struct IxdtfRegistryInitiallyContainsUCaEntry;
    structural_prop!(
        IxdtfRegistryInitiallyContainsUCaEntry,
        "IxdtfRegistryInitiallyContainsUCaEntry"
    );

    /// The initial `u-ca` registry entry is permanent.
    ///
    /// Normative source: RFC 9557 §6 — IANA Considerations
    pub struct IxdtfUCaRegistryEntryIsPermanent;
    structural_prop!(
        IxdtfUCaRegistryEntryIsPermanent,
        "IxdtfUCaRegistryEntryIsPermanent"
    );

    /// The initial `u-ca` registry entry uses the description "Preferred Calendar for Presentation".
    ///
    /// Normative source: RFC 9557 §6 — IANA Considerations
    pub struct IxdtfUCaRegistryEntryUsesPreferredCalendarForPresentationDescription;
    structural_prop!(
        IxdtfUCaRegistryEntryUsesPreferredCalendarForPresentationDescription,
        "IxdtfUCaRegistryEntryUsesPreferredCalendarForPresentationDescription"
    );

    /// The initial `u-ca` registry entry uses `IETF` as its change controller.
    ///
    /// Normative source: RFC 9557 §6 — IANA Considerations
    pub struct IxdtfUCaRegistryEntryUsesIetfChangeController;
    structural_prop!(
        IxdtfUCaRegistryEntryUsesIetfChangeController,
        "IxdtfUCaRegistryEntryUsesIetfChangeController"
    );

    /// The initial `u-ca` registry entry references Section 5 of RFC 9557.
    ///
    /// Normative source: RFC 9557 §6 — IANA Considerations
    pub struct IxdtfUCaRegistryEntryReferencesSectionFive;
    structural_prop!(
        IxdtfUCaRegistryEntryReferencesSectionFive,
        "IxdtfUCaRegistryEntryReferencesSectionFive"
    );

    /// Permanent Timestamp Suffix Tag Keys use the Specification Required registration policy.
    ///
    /// Normative source: RFC 9557 §6 — IANA Considerations
    pub struct IxdtfPermanentEntriesUseSpecificationRequiredPolicy;
    structural_prop!(
        IxdtfPermanentEntriesUseSpecificationRequiredPolicy,
        "IxdtfPermanentEntriesUseSpecificationRequiredPolicy"
    );

    /// Provisional Timestamp Suffix Tag Keys use the Expert Review registration policy.
    ///
    /// Normative source: RFC 9557 §6 — IANA Considerations
    pub struct IxdtfProvisionalEntriesUseExpertReviewPolicy;
    structural_prop!(
        IxdtfProvisionalEntriesUseExpertReviewPolicy,
        "IxdtfProvisionalEntriesUseExpertReviewPolicy"
    );

    /// Expert review for provisional Timestamp Suffix Tag Keys ascertains that a basic specification exists.
    ///
    /// Normative source: RFC 9557 §6 — IANA Considerations
    pub struct IxdtfExpertReviewAscertainsBasicSpecificationExists;
    structural_prop!(
        IxdtfExpertReviewAscertainsBasicSpecificationExists,
        "IxdtfExpertReviewAscertainsBasicSpecificationExists"
    );

    /// Expert review keeps concise identifiers with generally applicable semantics in reserve.
    ///
    /// Normative source: RFC 9557 §6 — IANA Considerations
    pub struct IxdtfExpertReviewReservesConciseGenerallyApplicableKeys;
    structural_prop!(
        IxdtfExpertReviewReservesConciseGenerallyApplicableKeys,
        "IxdtfExpertReviewReservesConciseGenerallyApplicableKeys"
    );

    /// Experts may initiate registration to avert future key-identifier collisions.
    ///
    /// Normative source: RFC 9557 §6 — IANA Considerations
    pub struct IxdtfExpertsMayInitiateRegistrationToAvoidFutureCollisions;
    structural_prop!(
        IxdtfExpertsMayInitiateRegistrationToAvoidFutureCollisions,
        "IxdtfExpertsMayInitiateRegistrationToAvoidFutureCollisions"
    );
}

pub use emit_impls::{
    IxdtfCalendarAnnotationDeclaresPreferredPresentationCalendar, IxdtfCalendarAnnotationPresent,
    IxdtfCalendarKeyUsesUCa, IxdtfCalendarValueUsesUnicodeCalendarIdentifier,
    IxdtfCriticalFlagIsLeadingExclamationWhenPresent,
    IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling,
    IxdtfDuplicateElectiveSuffixUsesFirstOccurrence,
    IxdtfExperimentalSuffixKeysAreNotForInterchange, IxdtfExperimentalSuffixKeysCannotBeRegistered,
    IxdtfExperimentalSuffixKeysUseLeadingUnderscore,
    IxdtfExpertReviewAscertainsBasicSpecificationExists,
    IxdtfExpertReviewReservesConciseGenerallyApplicableKeys,
    IxdtfExpertsMayInitiateRegistrationToAvoidFutureCollisions, IxdtfGeneratorsMayOmitSuffixTags,
    IxdtfPermanentEntriesUseSpecificationRequiredPolicy,
    IxdtfPermanentRegisteredSuffixKeyRequiresFullSpecificationReference,
    IxdtfProvisionalEntriesUseExpertReviewPolicy,
    IxdtfProvisionalRegisteredSuffixKeyReferenceExpectedToImproveOverTime,
    IxdtfProvisionalRegisteredSuffixKeyRequiresReferenceInformation,
    IxdtfRecipientsMayIgnoreElectiveSuffixTags,
    IxdtfRecipientsMustRejectUnconfiguredExperimentalSuffixKeys,
    IxdtfRegisteredSuffixKeyCarriesChangeController, IxdtfRegisteredSuffixKeyCarriesDescription,
    IxdtfRegisteredSuffixKeyCarriesKeyIdentifier, IxdtfRegisteredSuffixKeyCarriesReference,
    IxdtfRegisteredSuffixKeyCarriesRegistrationStatus,
    IxdtfRegisteredSuffixKeyStatusIsProvisionalOrPermanent, IxdtfRegistryInitiallyContainsUCaEntry,
    IxdtfSuffixFollowsRfc3339Timestamp, IxdtfSuffixKeysAreLowercase,
    IxdtfSuffixTagsUseBracketedKeyValueForm,
    IxdtfSuffixValuesAreCaseSensitiveUnlessOtherwiseSpecified,
    IxdtfSuffixValuesUseHyphenDelimitedItems, IxdtfTimeZoneSuffixUsesBracketedNameOrOffset,
    IxdtfUCaRegistryEntryIsPermanent, IxdtfUCaRegistryEntryReferencesSectionFive,
    IxdtfUCaRegistryEntryUsesIetfChangeController,
    IxdtfUCaRegistryEntryUsesPreferredCalendarForPresentationDescription,
};
