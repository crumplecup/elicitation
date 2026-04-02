//! `SqlxError` — elicitation-enabled wrapper around [`sqlx::Error`].

use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(sqlx::Error, as SqlxError);

#[reflect_methods]
impl SqlxError {
    /// Returns a human-readable label for the error kind.
    #[instrument(skip(self))]
    pub fn kind_label(&self) -> String {
        use sqlx::Error;
        match &*self.0 {
            Error::Configuration(_) => "Configuration".to_string(),
            Error::Database(_) => "Database".to_string(),
            Error::Io(_) => "Io".to_string(),
            Error::Tls(_) => "Tls".to_string(),
            Error::Protocol(_) => "Protocol".to_string(),
            Error::RowNotFound => "RowNotFound".to_string(),
            Error::TypeNotFound { .. } => "TypeNotFound".to_string(),
            Error::ColumnIndexOutOfBounds { .. } => "ColumnIndexOutOfBounds".to_string(),
            Error::ColumnNotFound(_) => "ColumnNotFound".to_string(),
            Error::ColumnDecode { .. } => "ColumnDecode".to_string(),
            Error::Decode(_) => "Decode".to_string(),
            Error::AnyDriverError(_) => "AnyDriverError".to_string(),
            Error::PoolTimedOut => "PoolTimedOut".to_string(),
            Error::PoolClosed => "PoolClosed".to_string(),
            Error::WorkerCrashed => "WorkerCrashed".to_string(),
            Error::Migrate(_) => "Migrate".to_string(),
            _ => "Other".to_string(),
        }
    }

    /// Returns the full error message string.
    #[instrument(skip(self))]
    pub fn message(&self) -> String {
        self.0.to_string()
    }
}

impl serde::Serialize for SqlxError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("kind", &self.kind_label())?;
        map.serialize_entry("message", &self.message())?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for SqlxError {
    fn deserialize<D: serde::Deserializer<'de>>(_d: D) -> Result<Self, D::Error> {
        Err(serde::de::Error::custom(
            "sqlx::Error cannot be reconstructed from JSON",
        ))
    }
}

mod emit_impls {
    use super::SqlxError;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for SqlxError {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { { unimplemented!("SqlxError cannot be reconstructed as a code literal") } }
        }
    }
}

impl elicitation::ElicitComplete for SqlxError {}
