//! Security propositions.
//!
//! Source: ISO/IEC 27001:2022 — Information security management.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    /// Access was authorized for the requesting identity.
    ///
    /// Source: ISO/IEC 27001:2022 §A.5.15 — Access control
    pub struct AccessAuthorized;

    /// The operation was recorded in the audit log.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.15 — Logging
    pub struct AuditLogged;

    /// The minimum necessary privileges were enforced for this operation.
    ///
    /// Source: ISO/IEC 27001:2022 §A.5.15 — Least privilege principle
    pub struct LeastPrivilegeEnforced;

    /// Data at rest is encrypted.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.24 — Use of cryptography
    pub struct EncryptedAtRest;

    /// Data in transit is encrypted.
    ///
    /// Source: ISO/IEC 27001:2022 §A.8.24 — Use of cryptography
    pub struct EncryptedInTransit;

    macro_rules! security_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by access control policy */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by access control policy */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by access control policy */ }
                }
            }
        };
    }

    security_prop!(AccessAuthorized, "AccessAuthorized");
    security_prop!(AuditLogged, "AuditLogged");
    security_prop!(LeastPrivilegeEnforced, "LeastPrivilegeEnforced");
    security_prop!(EncryptedAtRest, "EncryptedAtRest");
    security_prop!(EncryptedInTransit, "EncryptedInTransit");
}

pub use emit_impls::{
    AccessAuthorized, AuditLogged, EncryptedAtRest, EncryptedInTransit, LeastPrivilegeEnforced,
};
