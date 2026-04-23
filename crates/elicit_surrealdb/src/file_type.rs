//! [`surrealdb_types::File`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(surrealdb_types::File, as File, forward_serde);
elicit_newtype_traits!(File, surrealdb_types::File, [eq]);

/// Unwrap the Arc back to an owned `surrealdb_types::File`.
impl From<File> for surrealdb_types::File {
    fn from(val: File) -> Self {
        std::sync::Arc::try_unwrap(val.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl File {
    /// Returns the bucket name.
    #[tracing::instrument(skip(self))]
    pub fn bucket(&self) -> String {
        self.0.bucket().to_string()
    }

    /// Returns the file key (always begins with `/`).
    #[tracing::instrument(skip(self))]
    pub fn key(&self) -> String {
        self.0.key().to_string()
    }

    /// Returns the SurrealQL literal, e.g. `f"bucket:/key"`.
    #[tracing::instrument(skip(self))]
    pub fn to_surreal_literal(&self) -> String {
        format!("f\"{}:{}\"", self.0.bucket(), self.0.key())
    }
}

impl elicitation::ElicitComplete for File {}

mod emit_impls {
    use super::File;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for File {
        fn to_code_literal(&self) -> TokenStream {
            let bucket = self.0.bucket().to_string();
            let key = self.0.key().to_string();
            quote::quote! {
                ::surrealdb_types::File::new(#bucket, #key).into()
            }
        }
    }
}
