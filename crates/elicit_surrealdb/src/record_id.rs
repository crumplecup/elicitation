//! [`surrealdb_types::RecordId`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(surrealdb_types::RecordId, as RecordId, forward_serde);
elicit_newtype_traits!(RecordId, surrealdb_types::RecordId, [eq]);

/// Unwrap the Arc back to an owned `surrealdb_types::RecordId`.
impl From<RecordId> for surrealdb_types::RecordId {
    fn from(val: RecordId) -> Self {
        std::sync::Arc::try_unwrap(val.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl RecordId {
    /// Return the table name of this record ID.
    #[tracing::instrument(skip(self))]
    pub fn table(&self) -> String {
        self.0.table.to_string()
    }

    /// Returns `true` if this record ID's table is one of the given table names.
    #[tracing::instrument(skip(self))]
    pub fn is_table_type(&self, tables: Vec<String>) -> bool {
        let tables: Vec<surrealdb_types::Table> = tables
            .into_iter()
            .map(surrealdb_types::Table::new)
            .collect();
        self.0.is_table_type(&tables)
    }
}

impl elicitation::ElicitComplete for RecordId {}

mod emit_impls {
    use super::RecordId;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for RecordId {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(self.0.as_ref()).expect("RecordId is Serialize");
            quote::quote! {
                ::serde_json::from_str::<::surrealdb_types::RecordId>(#json)
                    .expect("valid RecordId JSON")
                    .into()
            }
        }
    }
}
