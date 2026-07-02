//! Precision and subsecond-retention propositions.
//!
//! Sources:
//! - ISO 8601-1:2019 — decimal fractions
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
    /// Normative source: ISO 8601-1:2019 — decimal fractions
    /// Informative cross-check: RFC 3339 §5.6
    pub struct FractionalSecondPrecisionDeclared;
    structural_prop!(
        FractionalSecondPrecisionDeclared,
        "FractionalSecondPrecisionDeclared"
    );

    /// Fractional-second digits form a contiguous decimal suffix.
    ///
    /// Normative source: ISO 8601-1:2019 — decimal fractions
    /// Informative cross-check: RFC 3339 §5.6
    pub struct FractionalSecondDigitsAreContiguous;
    structural_prop!(
        FractionalSecondDigitsAreContiguous,
        "FractionalSecondDigitsAreContiguous"
    );

    /// A precision reduction is explicitly declared rather than implicit.
    ///
    /// Normative source: ISO 8601-2:2019 — reduced precision and truncation semantics
    pub struct PrecisionReductionDeclared;
    structural_prop!(PrecisionReductionDeclared, "PrecisionReductionDeclared");

    /// A rounding mode is explicitly declared when precision is reduced.
    ///
    /// Normative source: ISO 8601-2:2019 — reduced precision and rounding semantics
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
