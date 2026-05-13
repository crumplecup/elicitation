//! [`surrealdb_types::Value`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

use crate::Kind;

elicit_newtype!(surrealdb_types::Value, as Value, forward_serde);
elicit_newtype_traits!(Value, surrealdb_types::Value, [eq]);

/// Unwrap the Arc back to an owned `surrealdb_types::Value`.
impl From<Value> for surrealdb_types::Value {
    fn from(val: Value) -> Self {
        std::sync::Arc::try_unwrap(val.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl Value {
    /// Returns the [`Kind`] that describes this value's type.
    #[tracing::instrument(skip(self))]
    pub fn kind(&self) -> Kind {
        self.0.kind().into()
    }

    /// Returns the first element if this value is an array, otherwise `None`.
    #[tracing::instrument(skip(self))]
    pub fn first(&self) -> Option<Value> {
        self.0.first().map(Into::into)
    }

    /// Returns `true` if this value is `None` or `Null`.
    #[tracing::instrument(skip(self))]
    pub fn is_nullish(&self) -> bool {
        self.0.is_nullish()
    }

    /// Returns `true` if this value is empty (none, null, empty string, empty array, etc.).
    #[tracing::instrument(skip(self))]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if this value conforms to the given [`Kind`].
    #[tracing::instrument(skip(self))]
    pub fn is_kind(&self, kind: Kind) -> bool {
        self.0.is_kind(&surrealdb_types::Kind::from(kind))
    }
}

impl elicitation::ElicitComplete for Value {}

mod emit_impls {
    use super::Value;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Value {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(self.0.as_ref()).expect("Value is Serialize");
            quote::quote! {
                ::serde_json::from_str::<::surrealdb_types::Value>(#json)
                    .expect("valid Value JSON")
                    .into()
            }
        }
    }
}
