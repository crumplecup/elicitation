//! Named-zone and zone-identity propositions.
//!
//! Sources:
//! - RFC 9557 §3.1, §3.3
//! - BCP 175 / IANA Time Zone Database, where named-zone semantics matter

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — zone contract */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — zone contract */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — zone contract */ }
                }
            }
        };
    }

    /// An IXDTF timestamp carries a named time-zone annotation.
    ///
    /// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
    pub struct NamedTimeZoneAnnotationPresent;
    structural_prop!(
        NamedTimeZoneAnnotationPresent,
        "NamedTimeZoneAnnotationPresent"
    );

    /// A named time-zone annotation uses an IANA time-zone identifier.
    ///
    /// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
    pub struct NamedTimeZoneUsesIanaIdentifier;
    structural_prop!(
        NamedTimeZoneUsesIanaIdentifier,
        "NamedTimeZoneUsesIanaIdentifier"
    );

    /// An IXDTF timestamp may carry an offset time-zone annotation.
    ///
    /// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
    pub struct OffsetTimeZoneAnnotationPresent;
    structural_prop!(
        OffsetTimeZoneAnnotationPresent,
        "OffsetTimeZoneAnnotationPresent"
    );

    /// Named time-zone identifiers are case-sensitive.
    ///
    /// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
    pub struct NamedTimeZoneIdentifierIsCaseSensitive;
    structural_prop!(
        NamedTimeZoneIdentifierIsCaseSensitive,
        "NamedTimeZoneIdentifierIsCaseSensitive"
    );

    /// A named time zone is not equivalent to a bare numeric UTC offset.
    ///
    /// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
    pub struct NamedTimeZoneIsNotNumericOffsetAlias;
    structural_prop!(
        NamedTimeZoneIsNotNumericOffsetAlias,
        "NamedTimeZoneIsNotNumericOffsetAlias"
    );

    /// A bare numeric UTC offset does not identify a named time zone.
    ///
    /// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
    pub struct NumericOffsetDoesNotIdentifyNamedZone;
    structural_prop!(
        NumericOffsetDoesNotIdentifyNamedZone,
        "NumericOffsetDoesNotIdentifyNamedZone"
    );

    /// An offset time-zone annotation repeats the RFC 3339 timestamp offset.
    ///
    /// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
    pub struct OffsetTimeZoneRepeatsTimestampOffset;
    structural_prop!(
        OffsetTimeZoneRepeatsTimestampOffset,
        "OffsetTimeZoneRepeatsTimestampOffset"
    );

    /// Use of offset time zones is strongly discouraged.
    ///
    /// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
    pub struct OffsetTimeZoneUseIsStronglyDiscouraged;
    structural_prop!(
        OffsetTimeZoneUseIsStronglyDiscouraged,
        "OffsetTimeZoneUseIsStronglyDiscouraged"
    );

    /// Programs must not synthesize an offset time-zone annotation by copying the timestamp offset.
    ///
    /// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
    pub struct OffsetTimeZoneMustNotBeSynthesizedFromTimestampOffset;
    structural_prop!(
        OffsetTimeZoneMustNotBeSynthesizedFromTimestampOffset,
        "OffsetTimeZoneMustNotBeSynthesizedFromTimestampOffset"
    );

    /// A named time zone retains civil-time rule identity beyond the current offset.
    ///
    /// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
    /// Informative cross-check: BCP 175 / IANA TZDB semantics
    pub struct NamedTimeZoneRetainsCivilRuleIdentity;
    structural_prop!(
        NamedTimeZoneRetainsCivilRuleIdentity,
        "NamedTimeZoneRetainsCivilRuleIdentity"
    );

    /// The named-zone rules resolve an offset for the represented instant.
    ///
    /// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
    pub struct ZoneOffsetResolvedForRepresentedInstant;
    structural_prop!(
        ZoneOffsetResolvedForRepresentedInstant,
        "ZoneOffsetResolvedForRepresentedInstant"
    );

    /// A named IANA time zone is interpreted using the rules current at the time of interpretation.
    ///
    /// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
    pub struct NamedTimeZoneMeaningUsesCurrentTzdbRules;
    structural_prop!(
        NamedTimeZoneMeaningUsesCurrentTzdbRules,
        "NamedTimeZoneMeaningUsesCurrentTzdbRules"
    );

    /// An unrecognized IANA time-zone name due to TZDB revision skew is treated as an inconsistency.
    ///
    /// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
    pub struct UnknownNamedTimeZoneIdentifierTreatedAsInconsistency;
    structural_prop!(
        UnknownNamedTimeZoneIdentifierTreatedAsInconsistency,
        "UnknownNamedTimeZoneIdentifierTreatedAsInconsistency"
    );

    /// A local timestamp explicitly declares when a zone transition introduces ambiguity.
    ///
    /// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
    pub struct ZoneTransitionAmbiguityDeclared;
    structural_prop!(
        ZoneTransitionAmbiguityDeclared,
        "ZoneTransitionAmbiguityDeclared"
    );

    /// A local timestamp explicitly declares when a zone transition creates a gap.
    ///
    /// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
    pub struct ZoneTransitionGapDeclared;
    structural_prop!(ZoneTransitionGapDeclared, "ZoneTransitionGapDeclared");

    /// Resolution of an ambiguous local timestamp uses explicit disambiguation authority.
    ///
    /// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
    pub struct ZoneTransitionDisambiguationAuthorityDeclared;
    structural_prop!(
        ZoneTransitionDisambiguationAuthorityDeclared,
        "ZoneTransitionDisambiguationAuthorityDeclared"
    );

    /// Resolution of a skipped local timestamp uses explicit gap-handling authority.
    ///
    /// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
    pub struct ZoneTransitionGapHandlingAuthorityDeclared;
    structural_prop!(
        ZoneTransitionGapHandlingAuthorityDeclared,
        "ZoneTransitionGapHandlingAuthorityDeclared"
    );
}

pub use emit_impls::{
    NamedTimeZoneAnnotationPresent, NamedTimeZoneIdentifierIsCaseSensitive,
    NamedTimeZoneIsNotNumericOffsetAlias, NamedTimeZoneMeaningUsesCurrentTzdbRules,
    NamedTimeZoneRetainsCivilRuleIdentity, NamedTimeZoneUsesIanaIdentifier,
    NumericOffsetDoesNotIdentifyNamedZone, OffsetTimeZoneAnnotationPresent,
    OffsetTimeZoneMustNotBeSynthesizedFromTimestampOffset, OffsetTimeZoneRepeatsTimestampOffset,
    OffsetTimeZoneUseIsStronglyDiscouraged, UnknownNamedTimeZoneIdentifierTreatedAsInconsistency,
    ZoneOffsetResolvedForRepresentedInstant, ZoneTransitionAmbiguityDeclared,
    ZoneTransitionDisambiguationAuthorityDeclared, ZoneTransitionGapDeclared,
    ZoneTransitionGapHandlingAuthorityDeclared,
};
