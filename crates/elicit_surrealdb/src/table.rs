//! [`surrealdb_types::Table`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(surrealdb_types::Table, as Table, forward_serde);
elicit_newtype_traits!(Table, surrealdb_types::Table, [eq, display]);

#[reflect_methods]
impl Table {
    /// Return the table name as a string.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> String {
        self.0.as_str().to_string()
    }

    /// Consume the wrapper and return the table name as an owned string.
    #[tracing::instrument(skip(self))]
    pub fn into_string(self) -> String {
        let inner = std::sync::Arc::try_unwrap(self.0).unwrap_or_else(|arc| (*arc).clone());
        inner.into_string()
    }
}

impl elicitation::ElicitComplete for Table {}

mod emit_impls {
    use super::Table;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Table {
        fn to_code_literal(&self) -> TokenStream {
            let name = self.0.as_str().to_string();
            quote::quote! {
                ::surrealdb_types::Table::new(#name.to_string()).into()
            }
        }
    }
}
