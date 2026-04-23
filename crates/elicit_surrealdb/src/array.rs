//! [`surrealdb_types::Array`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

use crate::Value;

elicit_newtype!(surrealdb_types::Array, as Array, forward_serde);
elicit_newtype_traits!(Array, surrealdb_types::Array, [eq]);

/// Unwrap the Arc back to an owned `surrealdb_types::Array`.
impl From<Array> for surrealdb_types::Array {
    fn from(val: Array) -> Self {
        std::sync::Arc::try_unwrap(val.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl Array {
    /// Returns the number of elements in the array.
    #[tracing::instrument(skip(self))]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the array contains no elements.
    #[tracing::instrument(skip(self))]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the first element, if any.
    #[tracing::instrument(skip(self))]
    pub fn first(&self) -> Option<Value> {
        self.0.first().cloned().map(Into::into)
    }

    /// Returns the last element, if any.
    #[tracing::instrument(skip(self))]
    pub fn last(&self) -> Option<Value> {
        self.0.last().cloned().map(Into::into)
    }

    /// Returns the element at `index`, or `None` if out of bounds.
    #[tracing::instrument(skip(self))]
    pub fn get(&self, index: usize) -> Option<Value> {
        self.0.get(index).cloned().map(Into::into)
    }

    /// Returns all elements as a `Vec<Value>`.
    #[tracing::instrument(skip(self))]
    pub fn to_vec(&self) -> Vec<Value> {
        self.0.iter().cloned().map(Into::into).collect()
    }
}

impl elicitation::ElicitComplete for Array {}

mod emit_impls {
    use super::Array;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Array {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(self.0.as_ref()).expect("Array is Serialize");
            quote::quote! {
                ::serde_json::from_str::<::surrealdb_types::Array>(#json)
                    .expect("valid Array JSON")
                    .into()
            }
        }
    }
}
