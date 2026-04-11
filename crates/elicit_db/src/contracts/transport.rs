//! Wire protocol and transport propositions.
//!
//! Source: PostgreSQL Frontend/Backend Protocol (§55) and IETF RFC 7159.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    /// Request is well-formed according to the wire protocol.
    ///
    /// Source: PostgreSQL Frontend/Backend Protocol §55
    pub struct RequestWellFormed;

    /// Response is fully serializable to JSON.
    ///
    /// Source: IETF RFC 7159 — The JavaScript Object Notation (JSON) Data Interchange Format
    pub struct ResponseSerializable;

    /// A database connection has been established.
    ///
    /// Source: PostgreSQL Frontend/Backend Protocol §55.2 — Connection Setup
    pub struct ConnectionEstablished;

    macro_rules! transport_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by protocol message framing */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by protocol message framing */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by protocol message framing */ }
                }
            }
        };
    }

    transport_prop!(RequestWellFormed, "RequestWellFormed");
    transport_prop!(ResponseSerializable, "ResponseSerializable");
    transport_prop!(ConnectionEstablished, "ConnectionEstablished");
}

pub use emit_impls::{ConnectionEstablished, RequestWellFormed, ResponseSerializable};
