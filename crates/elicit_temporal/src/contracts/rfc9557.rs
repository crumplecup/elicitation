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
    /// Normative source: RFC 9557 §3.3 — Full Syntax
    pub struct IxdtfSuffixFollowsRfc3339Timestamp;
    structural_prop!(
        IxdtfSuffixFollowsRfc3339Timestamp,
        "IxdtfSuffixFollowsRfc3339Timestamp"
    );

    /// A time-zone annotation uses bracketed content naming a zone or offset identifier.
    ///
    /// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
    pub struct IxdtfTimeZoneSuffixUsesBracketedNameOrOffset;
    structural_prop!(
        IxdtfTimeZoneSuffixUsesBracketedNameOrOffset,
        "IxdtfTimeZoneSuffixUsesBracketedNameOrOffset"
    );

    /// A critical annotation, when present, begins with `!`.
    ///
    /// Normative source: RFC 9557 §3.2 — Critical Flags
    pub struct IxdtfCriticalFlagIsLeadingExclamationWhenPresent;
    structural_prop!(
        IxdtfCriticalFlagIsLeadingExclamationWhenPresent,
        "IxdtfCriticalFlagIsLeadingExclamationWhenPresent"
    );

    /// Additional-information keys are lowercase.
    ///
    /// Normative source: RFC 9557 §3.3 — Full Syntax
    pub struct IxdtfSuffixKeysAreLowercase;
    structural_prop!(IxdtfSuffixKeysAreLowercase, "IxdtfSuffixKeysAreLowercase");

    /// A suffix tag uses bracketed key-value form with `=` between key and value.
    ///
    /// Normative sources: RFC 9557 §3.3 — Full Syntax; §4.1 — ABNF
    pub struct IxdtfSuffixTagsUseBracketedKeyValueForm;
    structural_prop!(
        IxdtfSuffixTagsUseBracketedKeyValueForm,
        "IxdtfSuffixTagsUseBracketedKeyValueForm"
    );

    /// A suffix value uses one or more hyphen-delimited items.
    ///
    /// Normative sources: RFC 9557 §3.3 — Full Syntax; §4.1 — ABNF
    pub struct IxdtfSuffixValuesUseHyphenDelimitedItems;
    structural_prop!(
        IxdtfSuffixValuesUseHyphenDelimitedItems,
        "IxdtfSuffixValuesUseHyphenDelimitedItems"
    );

    /// Suffix values are case-sensitive unless a key specification says otherwise.
    ///
    /// Normative source: RFC 9557 §3.3 — Full Syntax
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

    /// Experimental suffix keys use a leading underscore.
    ///
    /// Normative sources: RFC 9557 §3.3 — Suffix tag key registration and experimental use; §4.1 — ABNF
    pub struct IxdtfExperimentalSuffixKeysUseLeadingUnderscore;
    structural_prop!(
        IxdtfExperimentalSuffixKeysUseLeadingUnderscore,
        "IxdtfExperimentalSuffixKeysUseLeadingUnderscore"
    );

    /// Experimental suffix keys are not valid for general interchange.
    ///
    /// Normative source: RFC 9557 §3.3 — Suffix tag key registration and experimental use
    pub struct IxdtfExperimentalSuffixKeysAreNotForInterchange;
    structural_prop!(
        IxdtfExperimentalSuffixKeysAreNotForInterchange,
        "IxdtfExperimentalSuffixKeysAreNotForInterchange"
    );

    /// Recipients must reject experimental suffix keys when not configured for the experiment.
    ///
    /// Normative source: RFC 9557 §3.3 — Suffix tag key registration and experimental use
    pub struct IxdtfRecipientsMustRejectUnconfiguredExperimentalSuffixKeys;
    structural_prop!(
        IxdtfRecipientsMustRejectUnconfiguredExperimentalSuffixKeys,
        "IxdtfRecipientsMustRejectUnconfiguredExperimentalSuffixKeys"
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
}

pub use emit_impls::{
    IxdtfCalendarAnnotationDeclaresPreferredPresentationCalendar, IxdtfCalendarAnnotationPresent,
    IxdtfCalendarKeyUsesUCa, IxdtfCalendarValueUsesUnicodeCalendarIdentifier,
    IxdtfCriticalFlagIsLeadingExclamationWhenPresent,
    IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling,
    IxdtfExperimentalSuffixKeysAreNotForInterchange,
    IxdtfExperimentalSuffixKeysUseLeadingUnderscore, IxdtfGeneratorsMayOmitSuffixTags,
    IxdtfRecipientsMayIgnoreElectiveSuffixTags,
    IxdtfRecipientsMustRejectUnconfiguredExperimentalSuffixKeys,
    IxdtfSuffixFollowsRfc3339Timestamp, IxdtfSuffixKeysAreLowercase,
    IxdtfSuffixTagsUseBracketedKeyValueForm,
    IxdtfSuffixValuesAreCaseSensitiveUnlessOtherwiseSpecified,
    IxdtfSuffixValuesUseHyphenDelimitedItems, IxdtfTimeZoneSuffixUsesBracketedNameOrOffset,
};
