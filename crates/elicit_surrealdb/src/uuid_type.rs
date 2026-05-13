//! [`surrealdb_types::Uuid`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(surrealdb_types::Uuid, as Uuid, forward_serde);
elicit_newtype_traits!(Uuid, surrealdb_types::Uuid, [eq, display]);

/// Unwrap the Arc back to an owned `surrealdb_types::Uuid`.
impl From<Uuid> for surrealdb_types::Uuid {
    fn from(val: Uuid) -> Self {
        std::sync::Arc::try_unwrap(val.0).unwrap_or_else(|arc| *arc)
    }
}

#[reflect_methods]
impl Uuid {
    /// Returns the UUID as a hyphenated string (e.g. `"550e8400-e29b-41d4-a716-446655440000"`).
    #[tracing::instrument(skip(self))]
    pub fn to_uuid_string(&self) -> String {
        self.0.to_string()
    }

    /// Returns `true` if this UUID is the nil UUID (all zeros).
    #[tracing::instrument(skip(self))]
    pub fn is_nil(&self) -> bool {
        self.0.as_ref() == &surrealdb_types::Uuid::default()
    }

    /// Returns the SurrealQL literal representation, e.g. `u'550e8400-…'`.
    #[tracing::instrument(skip(self))]
    pub fn to_surreal_literal(&self) -> String {
        format!("u'{}'", self.0)
    }
}

impl elicitation::ElicitComplete for Uuid {}

mod emit_impls {
    use super::Uuid;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Uuid {
        fn to_code_literal(&self) -> TokenStream {
            let s = self.0.to_string();
            quote::quote! {
                #s.parse::<::surrealdb_types::Uuid>()
                    .expect("valid UUID string")
                    .into()
            }
        }
    }
}
