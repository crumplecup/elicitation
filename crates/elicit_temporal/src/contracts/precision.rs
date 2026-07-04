//! Precision and subsecond-retention propositions.
//!
//! Sources:
//! - ISO 8601-1:2019/Amd 1:2022, 5.3.1.4
//! - ISO 8601-2:2019, 7.11, 7.12, 7.13, 14.2, 14.3, 14.4
//! - CalConnect CC 18011:2018, representations-precision,
//!   representations-decimal, representations-reduced-precision, §8.2, §8.3-§8.5
//! - RFC 3339 §5.6

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — precision contract */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — precision contract */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — precision contract */ }
                }
            }
        };
    }

    /// Fractional-second precision is explicitly declared when subseconds are present.
    ///
    /// Normative basis: ISO 8601-1:2019/Amd 1:2022, 5.3.1.4.
    /// Informative cross-check: RFC 3339 §5.6.
    pub struct FractionalSecondPrecisionDeclared;
    structural_prop!(
        FractionalSecondPrecisionDeclared,
        "FractionalSecondPrecisionDeclared"
    );

    /// Fractional-second digits form a contiguous decimal suffix.
    ///
    /// Normative basis: ISO 8601-1:2019/Amd 1:2022, 5.3.1.4.
    /// Informative cross-check: RFC 3339 §5.6.
    pub struct FractionalSecondDigitsAreContiguous;
    structural_prop!(
        FractionalSecondDigitsAreContiguous,
        "FractionalSecondDigitsAreContiguous"
    );

    /// A precision reduction is explicitly declared rather than implicit.
    ///
    /// Normative basis: ISO 8601-2:2019, 7.11 and 7.13.
    /// Informative cross-check: CalConnect CC 18011:2018,
    /// representations-precision and representations-reduced-precision.
    pub struct PrecisionReductionDeclared;
    structural_prop!(PrecisionReductionDeclared, "PrecisionReductionDeclared");

    /// A rounding mode is explicitly declared when precision is reduced.
    ///
    /// Normative basis: ISO 8601-2:2019, 7.13, 14.2, 14.3, and 14.4.
    /// Informative cross-check: CalConnect CC 18011:2018 §8.2 and §8.3-§8.5.
    /// The accord requires this policy to remain explicit rather than implicit.
    pub struct RoundingModeDeclared;
    structural_prop!(RoundingModeDeclared, "RoundingModeDeclared");

    /// Subsecond digits are preserved across an exchange.
    ///
    /// Normative source: RFC 3339 §5.6 — fractional seconds
    pub struct SubsecondDigitsPreserved;
    structural_prop!(SubsecondDigitsPreserved, "SubsecondDigitsPreserved");
}

pub use emit_impls::{
    FractionalSecondDigitsAreContiguous, FractionalSecondPrecisionDeclared,
    PrecisionReductionDeclared, RoundingModeDeclared, SubsecondDigitsPreserved,
};
