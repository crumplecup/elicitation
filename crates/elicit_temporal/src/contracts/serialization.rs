//! Serialization-profile propositions.
//!
//! Sources:
//! - ISO 8601-1:2019 — basic and extended forms
//! - RFC 3339 §5.6
//! - RFC 9557 §3.3

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — serialization contract */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — serialization contract */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — serialization contract */ }
                }
            }
        };
    }

    /// ISO 8601 basic form uses a compact representation without separators.
    ///
    /// Normative source: ISO 8601-1:2019 — basic format
    pub struct Iso8601BasicFormUsesCompactRepresentation;
    structural_prop!(
        Iso8601BasicFormUsesCompactRepresentation,
        "Iso8601BasicFormUsesCompactRepresentation"
    );

    /// ISO 8601 extended form uses separators between major components.
    ///
    /// Normative source: ISO 8601-1:2019 — extended format
    pub struct Iso8601ExtendedFormUsesSeparators;
    structural_prop!(
        Iso8601ExtendedFormUsesSeparators,
        "Iso8601ExtendedFormUsesSeparators"
    );

    /// The chosen serialization profile is explicitly declared.
    ///
    /// Normative source: ISO 8601-1:2019 — basic and extended forms
    /// Informative cross-checks: RFC 3339 §5.6; RFC 9557 §3.3
    pub struct SerializationProfileDeclared;
    structural_prop!(SerializationProfileDeclared, "SerializationProfileDeclared");

    /// The serialization carries an explicit UTC relationship.
    ///
    /// Normative source: RFC 3339 §4.4, §5.6
    pub struct SerializationCarriesExplicitUtcRelationship;
    structural_prop!(
        SerializationCarriesExplicitUtcRelationship,
        "SerializationCarriesExplicitUtcRelationship"
    );

    /// IXDTF serialization carries a named-zone annotation when zone identity is preserved.
    ///
    /// Normative source: RFC 9557 §3.1, §3.3
    pub struct IxdtfSerializationCarriesNamedZoneAnnotation;
    structural_prop!(
        IxdtfSerializationCarriesNamedZoneAnnotation,
        "IxdtfSerializationCarriesNamedZoneAnnotation"
    );
}

pub use emit_impls::{
    Iso8601BasicFormUsesCompactRepresentation, Iso8601ExtendedFormUsesSeparators,
    IxdtfSerializationCarriesNamedZoneAnnotation, SerializationCarriesExplicitUtcRelationship,
    SerializationProfileDeclared,
};
