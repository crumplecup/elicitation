//! [`surrealdb_types::Object`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

use crate::Value;

elicit_newtype!(surrealdb_types::Object, as Object, forward_serde);
elicit_newtype_traits!(Object, surrealdb_types::Object, [eq]);

/// Unwrap the Arc back to an owned `surrealdb_types::Object`.
impl From<Object> for surrealdb_types::Object {
    fn from(val: Object) -> Self {
        std::sync::Arc::try_unwrap(val.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl Object {
    /// Returns the number of key-value pairs.
    #[tracing::instrument(skip(self))]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the object has no fields.
    #[tracing::instrument(skip(self))]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns all keys in sorted order.
    #[tracing::instrument(skip(self))]
    pub fn keys(&self) -> Vec<String> {
        self.0.keys().cloned().collect()
    }

    /// Returns the value for `key`, or `None` if the key is absent.
    #[tracing::instrument(skip(self))]
    pub fn get(&self, key: String) -> Option<Value> {
        self.0.get(&key).cloned().map(Into::into)
    }

    /// Returns `true` if the object contains the given key.
    #[tracing::instrument(skip(self))]
    pub fn contains_key(&self, key: String) -> bool {
        self.0.contains_key(&key)
    }

    /// Returns all values in key-sorted order.
    #[tracing::instrument(skip(self))]
    pub fn values(&self) -> Vec<Value> {
        self.0.values().cloned().map(Into::into).collect()
    }
}

impl elicitation::ElicitComplete for Object {}

mod emit_impls {
    use super::Object;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Object {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(self.0.as_ref()).expect("Object is Serialize");
            quote::quote! {
                ::serde_json::from_str::<::surrealdb_types::Object>(#json)
                    .expect("valid Object JSON")
                    .into()
            }
        }
    }
}
