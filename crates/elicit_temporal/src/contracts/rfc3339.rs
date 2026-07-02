//! RFC 3339 timestamp propositions.
//!
//! Normative source: RFC 3339, *Date and Time on the Internet: Timestamps*.
//! All section references are to RFC 3339.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — RFC 3339 contract */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — RFC 3339 contract */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — RFC 3339 contract */ }
                }
            }
        };
    }

    /// An RFC 3339 timestamp uses a four-digit year.
    ///
    /// Normative source: RFC 3339 §5.6 — Internet Date/Time Format
    pub struct Rfc3339UsesFourDigitYear;
    structural_prop!(Rfc3339UsesFourDigitYear, "Rfc3339UsesFourDigitYear");

    /// An RFC 3339 timestamp uses the `full-date` production.
    ///
    /// Normative source: RFC 3339 §5.6 — Internet Date/Time Format
    pub struct Rfc3339UsesExtendedCalendarDate;
    structural_prop!(
        Rfc3339UsesExtendedCalendarDate,
        "Rfc3339UsesExtendedCalendarDate"
    );

    /// An RFC 3339 timestamp uses the `full-time` production.
    ///
    /// Normative source: RFC 3339 §5.6 — Internet Date/Time Format
    pub struct Rfc3339UsesFullTime;
    structural_prop!(Rfc3339UsesFullTime, "Rfc3339UsesFullTime");

    /// An RFC 3339 timestamp includes an explicit relationship to UTC.
    ///
    /// Normative source: RFC 3339 §4.4 — Unqualified Local Time
    pub struct Rfc3339RequiresUtcRelationship;
    structural_prop!(
        Rfc3339RequiresUtcRelationship,
        "Rfc3339RequiresUtcRelationship"
    );

    /// Unqualified local time is forbidden in RFC 3339 timestamps.
    ///
    /// Normative source: RFC 3339 §4.4 — Unqualified Local Time
    pub struct Rfc3339UnqualifiedLocalTimeForbidden;
    structural_prop!(
        Rfc3339UnqualifiedLocalTimeForbidden,
        "Rfc3339UnqualifiedLocalTimeForbidden"
    );

    /// Fractional seconds use `.` as the separator.
    ///
    /// Normative source: RFC 3339 §5.6 — Internet Date/Time Format
    pub struct Rfc3339FractionUsesDotSeparator;
    structural_prop!(
        Rfc3339FractionUsesDotSeparator,
        "Rfc3339FractionUsesDotSeparator"
    );

    /// The UTC relationship is expressed either as `Z` or as a numeric offset.
    ///
    /// Normative source: RFC 3339 §5.6 — Internet Date/Time Format
    pub struct Rfc3339OffsetIsUtcOrNumeric;
    structural_prop!(Rfc3339OffsetIsUtcOrNumeric, "Rfc3339OffsetIsUtcOrNumeric");

    /// The unknown local-offset convention is encoded as `-00:00`.
    ///
    /// Normative source: RFC 3339 §4.3 — Unknown Local Offset Convention
    pub struct Rfc3339UnknownLocalOffsetUsesNegativeZero;
    structural_prop!(
        Rfc3339UnknownLocalOffsetUsesNegativeZero,
        "Rfc3339UnknownLocalOffsetUsesNegativeZero"
    );

    /// The timestamp does not use the unknown local-offset convention.
    ///
    /// Normative source: RFC 3339 §4.3 — Unknown Local Offset Convention
    pub struct Rfc3339LocalOffsetNotUnknown;
    structural_prop!(Rfc3339LocalOffsetNotUnknown, "Rfc3339LocalOffsetNotUnknown");

    /// Lexical ordering requires timestamps in the compared set to use the same UTC-relationship string form.
    ///
    /// Normative source: RFC 3339 §5.1 — Ordering
    pub struct Rfc3339LexicalOrderingRequiresUniformUtcRelationshipEncoding;
    structural_prop!(
        Rfc3339LexicalOrderingRequiresUniformUtcRelationshipEncoding,
        "Rfc3339LexicalOrderingRequiresUniformUtcRelationshipEncoding"
    );

    /// Lexical ordering requires timestamps in the compared set to use the same number of fractional-second digits.
    ///
    /// Normative source: RFC 3339 §5.1 — Ordering
    pub struct Rfc3339LexicalOrderingRequiresUniformFractionalSecondDigits;
    structural_prop!(
        Rfc3339LexicalOrderingRequiresUniformFractionalSecondDigits,
        "Rfc3339LexicalOrderingRequiresUniformFractionalSecondDigits"
    );

    /// The RFC 3339 profile achieves simplicity by making most fields and punctuation mandatory.
    ///
    /// Normative sources: RFC 3339 §5.5 — Simplicity; §5.6 — Internet Date/Time Format
    pub struct Rfc3339ProfileMakesMostFieldsAndPunctuationMandatory;
    structural_prop!(
        Rfc3339ProfileMakesMostFieldsAndPunctuationMandatory,
        "Rfc3339ProfileMakesMostFieldsAndPunctuationMandatory"
    );

    /// Fractions of a second are the only rarely used option in the RFC 3339 profile.
    ///
    /// Normative source: RFC 3339 §5.3 — Rarely Used Options
    pub struct Rfc3339FractionalSecondsAreOnlyRarelyUsedOption;
    structural_prop!(
        Rfc3339FractionalSecondsAreOnlyRarelyUsedOption,
        "Rfc3339FractionalSecondsAreOnlyRarelyUsedOption"
    );

    /// Generators should use uppercase `T` and `Z`.
    ///
    /// Normative source: RFC 3339 §5.6 — Internet Date/Time Format
    pub struct Rfc3339GeneratorsShouldUseUppercaseTAndZ;
    structural_prop!(
        Rfc3339GeneratorsShouldUseUppercaseTAndZ,
        "Rfc3339GeneratorsShouldUseUppercaseTAndZ"
    );

    /// Applications should not generate inserted leap-second timestamps before the leap second is announced.
    ///
    /// Normative source: RFC 3339 §5.7 — Restrictions
    pub struct Rfc3339LeapSecondGenerationRequiresPriorAnnouncement;
    structural_prop!(
        Rfc3339LeapSecondGenerationRequiresPriorAnnouncement,
        "Rfc3339LeapSecondGenerationRequiresPriorAnnouncement"
    );
}

pub use emit_impls::{
    Rfc3339FractionUsesDotSeparator, Rfc3339FractionalSecondsAreOnlyRarelyUsedOption,
    Rfc3339GeneratorsShouldUseUppercaseTAndZ, Rfc3339LeapSecondGenerationRequiresPriorAnnouncement,
    Rfc3339LexicalOrderingRequiresUniformFractionalSecondDigits,
    Rfc3339LexicalOrderingRequiresUniformUtcRelationshipEncoding, Rfc3339LocalOffsetNotUnknown,
    Rfc3339OffsetIsUtcOrNumeric, Rfc3339ProfileMakesMostFieldsAndPunctuationMandatory,
    Rfc3339RequiresUtcRelationship, Rfc3339UnknownLocalOffsetUsesNegativeZero,
    Rfc3339UnqualifiedLocalTimeForbidden, Rfc3339UsesExtendedCalendarDate,
    Rfc3339UsesFourDigitYear, Rfc3339UsesFullTime,
};
