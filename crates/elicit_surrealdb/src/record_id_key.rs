//! [`surrealdb_types::RecordIdKey`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(surrealdb_types::RecordIdKey, as RecordIdKey, forward_serde);
elicit_newtype_traits!(RecordIdKey, surrealdb_types::RecordIdKey, [eq]);

/// Unwrap the Arc back to an owned `surrealdb_types::RecordIdKey`.
impl From<RecordIdKey> for surrealdb_types::RecordIdKey {
    fn from(val: RecordIdKey) -> Self {
        std::sync::Arc::try_unwrap(val.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl RecordIdKey {
    /// Returns `true` if this key is a range key.
    #[tracing::instrument(skip(self))]
    pub fn is_range(&self) -> bool {
        self.0.is_range()
    }

    /// Returns `true` if this key is a numeric key.
    #[tracing::instrument(skip(self))]
    pub fn is_number(&self) -> bool {
        matches!(self.0.as_ref(), surrealdb_types::RecordIdKey::Number(_))
    }

    /// Returns `true` if this key is a string key.
    #[tracing::instrument(skip(self))]
    pub fn is_string(&self) -> bool {
        matches!(self.0.as_ref(), surrealdb_types::RecordIdKey::String(_))
    }

    /// Returns `true` if this key is a UUID key.
    #[tracing::instrument(skip(self))]
    pub fn is_uuid(&self) -> bool {
        matches!(self.0.as_ref(), surrealdb_types::RecordIdKey::Uuid(_))
    }

    /// Returns `true` if this key is an array key.
    #[tracing::instrument(skip(self))]
    pub fn is_array(&self) -> bool {
        matches!(self.0.as_ref(), surrealdb_types::RecordIdKey::Array(_))
    }

    /// Returns `true` if this key is an object key.
    #[tracing::instrument(skip(self))]
    pub fn is_object(&self) -> bool {
        matches!(self.0.as_ref(), surrealdb_types::RecordIdKey::Object(_))
    }
}

impl elicitation::ElicitComplete for RecordIdKey {}

mod emit_impls {
    use super::RecordIdKey;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for RecordIdKey {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(self.0.as_ref()).expect("RecordIdKey is Serialize");
            quote::quote! {
                ::serde_json::from_str::<::surrealdb_types::RecordIdKey>(#json)
                    .expect("valid RecordIdKey JSON")
                    .into()
            }
        }
    }
}
