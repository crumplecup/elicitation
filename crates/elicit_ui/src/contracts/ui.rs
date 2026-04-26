//! UI-pipeline proof tokens (not tied to any external standard).

mod emit_impls {
    use elicitation::contracts::{Prop, ProvableFrom};
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:path, $name:literal) => {
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
            #[cfg(kani)]
            impl kani::Arbitrary for $t {
                fn any() -> Self {
                    $t
                }
            }
        };
    }

    /// Marker: all 182 per-role `XxxNodeValid` proof types implement this.
    ///
    /// Enables blanket `ProvableFrom` impls: any role token can mint `RolePreserved`,
    /// and any role token can be minted from `Established<WcagVerified>`.
    pub trait NodeRoleProof: Prop {}

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

    use crate::VerifiedTree;
    use elicitation::contracts::Established;

    // `WcagVerified` is minted from a `VerifiedTree` — the tree is the credential.
    impl ProvableFrom<VerifiedTree> for WcagVerified {}

    // Any role proof can mint `RolePreserved` — the role token is the credential.
    impl<T: NodeRoleProof> ProvableFrom<Established<T>> for RolePreserved {}

    // `RenderComplete` is minted once the wcag-gated render pass finishes.
    impl ProvableFrom<Established<WcagVerified>> for RenderComplete {}
}

pub use emit_impls::{IrSourced, NodeRoleProof, RenderComplete, RolePreserved, WcagVerified};
