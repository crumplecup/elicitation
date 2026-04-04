//! `AnyColumn` — elicitation-enabled wrapper around [`sqlx_core::any::AnyColumn`].

use elicitation::{SqlTypeKind, elicit_newtype};
use elicitation_derive::reflect_methods;
use sqlx::Column as _;
use sqlx::TypeInfo as _;
use tracing::instrument;

elicit_newtype!(sqlx_core::any::AnyColumn, as AnyColumn);

#[reflect_methods]
impl AnyColumn {
    /// Returns the zero-based ordinal position of this column.
    #[instrument(skip(self))]
    pub fn ordinal(&self) -> usize {
        self.0.ordinal()
    }

    /// Returns the column name.
    #[instrument(skip(self))]
    pub fn name(&self) -> String {
        self.0.name().to_string()
    }

    /// Returns the SQL type kind for this column.
    #[instrument(skip(self))]
    pub fn type_kind(&self) -> SqlTypeKind {
        SqlTypeKind::from(self.0.type_info().kind)
    }

    /// Returns the database type name string (e.g. `"TEXT"`, `"BIGINT"`).
    #[instrument(skip(self))]
    pub fn type_name(&self) -> String {
        self.0.type_info().name().to_string()
    }
}

impl serde::Serialize for AnyColumn {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("ordinal", &self.ordinal())?;
        map.serialize_entry("name", &self.name())?;
        map.serialize_entry("type_name", &self.type_name())?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for AnyColumn {
    fn deserialize<D: serde::Deserializer<'de>>(_d: D) -> Result<Self, D::Error> {
        Err(serde::de::Error::custom(
            "sqlx_core::any::AnyColumn cannot be reconstructed from JSON",
        ))
    }
}

mod emit_impls {
    use super::AnyColumn;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for AnyColumn {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { { unimplemented!("AnyColumn cannot be reconstructed as a code literal") } }
        }
    }
}

impl elicitation::ElicitComplete for AnyColumn {}
