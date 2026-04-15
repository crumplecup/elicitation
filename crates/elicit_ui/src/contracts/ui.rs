//! UI-pipeline proof tokens (not tied to any external standard).

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by UI pipeline contract */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by UI pipeline contract */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by UI pipeline contract */ }
                }
            }
        };
    }

    /// Proposition: UI tree has been successfully rendered to a backend.
    pub struct RenderComplete;
    structural_prop!(RenderComplete, "RenderComplete");

    /// Proposition: The `VerifiedTree` passed to a renderer was produced by a
    /// canonical model's `to_verified_tree()` call, not constructed ad-hoc or
    /// bypassed.
    ///
    /// Any frontend renderer that requires this proof can only be invoked after
    /// `to_verified_tree()` has been called, giving all frontends contractually
    /// enforced equivalency at the type level.
    pub struct IrSourced;
    structural_prop!(IrSourced, "IrSourced");

    /// Proof that the [`accesskit`] tree was issued from a `Layout<Verified>`
    /// and all WCAG Level AA constraints have been satisfied by construction.
    pub struct WcagVerified;
    structural_prop!(WcagVerified, "WcagVerified");

    /// Proof that a frontend bridge method honoured all role guarantees carried
    /// into it — the produced widget preserves every invariant established
    /// during validation and dispatch.
    pub struct RolePreserved;
    structural_prop!(RolePreserved, "RolePreserved");
}

pub use emit_impls::{IrSourced, RenderComplete, RolePreserved, WcagVerified};
