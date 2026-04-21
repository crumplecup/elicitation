//! [`surrealdb_types::Kind`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(surrealdb_types::Kind, as Kind, forward_serde);
elicit_newtype_traits!(Kind, surrealdb_types::Kind, [eq, display]);

/// Unwrap the Arc back to an owned `surrealdb_types::Kind`.
impl From<Kind> for surrealdb_types::Kind {
    fn from(val: Kind) -> Self {
        std::sync::Arc::try_unwrap(val.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl Kind {
    /// Flatten an `Either` kind into a flat list of constituent kinds.
    #[tracing::instrument(skip(self))]
    pub fn flatten(self) -> Vec<Kind> {
        let inner = std::sync::Arc::try_unwrap(self.0).unwrap_or_else(|arc| (*arc).clone());
        inner.flatten().into_iter().map(Into::into).collect()
    }
}

impl elicitation::ElicitComplete for Kind {}

mod emit_impls {
    use super::Kind;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Kind {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(self.0.as_ref()).expect("Kind is Serialize");
            quote::quote! {
                ::serde_json::from_str::<::surrealdb_types::Kind>(#json)
                    .expect("valid Kind JSON")
                    .into()
            }
        }
    }
}
