//! [`surrealdb_types::Bytes`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(surrealdb_types::Bytes, as Bytes, forward_serde);
elicit_newtype_traits!(Bytes, surrealdb_types::Bytes, [eq, display]);

/// Unwrap the Arc back to an owned `surrealdb_types::Bytes`.
impl From<Bytes> for surrealdb_types::Bytes {
    fn from(val: Bytes) -> Self {
        std::sync::Arc::try_unwrap(val.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl Bytes {
    /// Returns the number of bytes.
    #[tracing::instrument(skip(self))]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the byte slice is empty.
    #[tracing::instrument(skip(self))]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the bytes as an uppercase hex string (e.g. `"DEADBEEF"`).
    #[tracing::instrument(skip(self))]
    pub fn to_hex(&self) -> String {
        hex::encode_upper(self.0.as_ref().as_ref())
    }

    /// Returns the SurrealQL literal representation, e.g. `b"DEADBEEF"`.
    #[tracing::instrument(skip(self))]
    pub fn to_surreal_literal(&self) -> String {
        format!("b\"{}\"", hex::encode_upper(self.0.as_ref().as_ref()))
    }
}

impl elicitation::ElicitComplete for Bytes {}

mod emit_impls {
    use super::Bytes;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Bytes {
        fn to_code_literal(&self) -> TokenStream {
            let hex = hex::encode_upper(self.0.as_ref().as_ref());
            quote::quote! {
                ::surrealdb_types::Bytes::from(
                    ::hex::decode(#hex).expect("valid hex")
                )
                .into()
            }
        }
    }
}
