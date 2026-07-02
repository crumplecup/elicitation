//! Temporal-conversion propositions.
//!
//! Sources:
//! - ISO 8601-1:2019 — representation changes across equivalent forms
//! - ISO 8601-2:2019 — reduced precision and extension semantics
//! - RFC 3339 §5.1, §5.6

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — conversion contract */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — conversion contract */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — conversion contract */ }
                }
            }
        };
    }

    /// The source temporal semantic kind is explicitly declared.
    ///
    /// Normative source: ISO 8601-1:2019 — distinct temporal representation families
    pub struct ConversionSourceSemanticKindDeclared;
    structural_prop!(
        ConversionSourceSemanticKindDeclared,
        "ConversionSourceSemanticKindDeclared"
    );

    /// The target temporal semantic kind is explicitly declared.
    ///
    /// Normative source: ISO 8601-1:2019 — distinct temporal representation families
    pub struct ConversionTargetSemanticKindDeclared;
    structural_prop!(
        ConversionTargetSemanticKindDeclared,
        "ConversionTargetSemanticKindDeclared"
    );

    /// A conversion preserves the represented fixed instant.
    ///
    /// Normative source: RFC 3339 §5.1 — Ordering
    pub struct ConversionPreservesRepresentedInstant;
    structural_prop!(
        ConversionPreservesRepresentedInstant,
        "ConversionPreservesRepresentedInstant"
    );

    /// A conversion preserves temporal ordering on the UTC timeline.
    ///
    /// Normative source: RFC 3339 §5.1 — Ordering
    pub struct ConversionPreservesTemporalOrdering;
    structural_prop!(
        ConversionPreservesTemporalOrdering,
        "ConversionPreservesTemporalOrdering"
    );

    /// A conversion drops named-zone identity.
    ///
    /// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
    pub struct ConversionDropsNamedZoneIdentity;
    structural_prop!(
        ConversionDropsNamedZoneIdentity,
        "ConversionDropsNamedZoneIdentity"
    );

    /// A conversion drops subsecond precision.
    ///
    /// Normative source: ISO 8601-2:2019 — reduced precision semantics
    pub struct ConversionDropsSubsecondPrecision;
    structural_prop!(
        ConversionDropsSubsecondPrecision,
        "ConversionDropsSubsecondPrecision"
    );

    /// A lossy conversion requires explicit authority rather than silent coercion.
    ///
    /// Normative source: ISO 8601-2:2019 — extended and reduced-precision semantics
    pub struct ConversionRequiresExplicitAuthorityWhenLossy;
    structural_prop!(
        ConversionRequiresExplicitAuthorityWhenLossy,
        "ConversionRequiresExplicitAuthorityWhenLossy"
    );
}

pub use emit_impls::{
    ConversionDropsNamedZoneIdentity, ConversionDropsSubsecondPrecision,
    ConversionPreservesRepresentedInstant, ConversionPreservesTemporalOrdering,
    ConversionRequiresExplicitAuthorityWhenLossy, ConversionSourceSemanticKindDeclared,
    ConversionTargetSemanticKindDeclared,
};
