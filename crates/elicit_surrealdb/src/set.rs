//! [`surrealdb_types::Set`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

use crate::Value;

elicit_newtype!(surrealdb_types::Set, as Set, forward_serde);
elicit_newtype_traits!(Set, surrealdb_types::Set, [eq]);

/// Unwrap the Arc back to an owned `surrealdb_types::Set`.
impl From<Set> for surrealdb_types::Set {
    fn from(val: Set) -> Self {
        std::sync::Arc::try_unwrap(val.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl Set {
    /// Returns the number of unique elements.
    #[tracing::instrument(skip(self))]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the set is empty.
    #[tracing::instrument(skip(self))]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns all elements as a sorted `Vec<Value>`.
    #[tracing::instrument(skip(self))]
    pub fn to_vec(&self) -> Vec<Value> {
        self.0.iter().cloned().map(Into::into).collect()
    }
}

impl elicitation::ElicitComplete for Set {}

mod emit_impls {
    use super::Set;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Set {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(self.0.as_ref()).expect("Set is Serialize");
            quote::quote! {
                ::serde_json::from_str::<::surrealdb_types::Set>(#json)
                    .expect("valid Set JSON")
                    .into()
            }
        }
    }
}
