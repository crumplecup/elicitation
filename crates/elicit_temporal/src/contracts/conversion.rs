//! Temporal-conversion propositions.
//!
//! Sources:
//! - ISO 8601-1:2019, 3.1.3, 5.2, 5.3, 5.4, 5.5, 5.6
//! - ISO 8601-2:2019, 7.11, 7.12, 7.13, 14.2, 14.3, 14.4
//! - CalConnect CC 18011:2018, representations-precision,
//!   representations-decimal, representations-reduced-precision, §8.2, §8.3-§8.5
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
    /// Normative basis: ISO 8601-1:2019, 3.1.3, 5.2, 5.3, 5.4, 5.5, and 5.6.
    pub struct ConversionSourceSemanticKindDeclared;
    structural_prop!(
        ConversionSourceSemanticKindDeclared,
        "ConversionSourceSemanticKindDeclared"
    );

    /// The target temporal semantic kind is explicitly declared.
    ///
    /// Normative basis: ISO 8601-1:2019, 3.1.3, 5.2, 5.3, 5.4, 5.5, and 5.6.
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
    /// Normative source: RFC 9557 §1.2 — Definitions
    pub struct ConversionDropsNamedZoneIdentity;
    structural_prop!(
        ConversionDropsNamedZoneIdentity,
        "ConversionDropsNamedZoneIdentity"
    );

    /// A conversion drops subsecond precision.
    ///
    /// Normative basis: ISO 8601-2:2019, 7.11, 7.12, and 7.13.
    /// Informative cross-check: CalConnect CC 18011:2018,
    /// representations-precision, representations-decimal,
    /// and representations-reduced-precision.
    pub struct ConversionDropsSubsecondPrecision;
    structural_prop!(
        ConversionDropsSubsecondPrecision,
        "ConversionDropsSubsecondPrecision"
    );

    /// A lossy conversion requires explicit authority rather than silent coercion.
    ///
    /// Normative basis: ISO 8601-2:2019, 7.13, 14.2, 14.3, and 14.4.
    /// Informative cross-check: CalConnect CC 18011:2018 §8.2 and §8.3-§8.5.
    /// Silent precision-reducing coercions are forbidden by the accord.
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
