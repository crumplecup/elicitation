//! `AnyTypeInfo` — elicitation-enabled wrapper around [`sqlx::any::AnyTypeInfo`].

use elicitation::{SqlTypeKind, elicit_newtype};
use elicitation_derive::reflect_methods;
use sqlx::TypeInfo as _;
use tracing::instrument;

elicit_newtype!(sqlx::any::AnyTypeInfo, as AnyTypeInfo);

#[reflect_methods]
impl AnyTypeInfo {
    /// Returns the SQL type kind for this type info.
    #[instrument(skip(self))]
    pub fn kind(&self) -> SqlTypeKind {
        SqlTypeKind::from(self.0.kind)
    }

    /// Returns the database type name string.
    #[instrument(skip(self))]
    pub fn name(&self) -> String {
        self.0.name().to_string()
    }

    /// Returns `true` if this type represents SQL NULL.
    #[instrument(skip(self))]
    pub fn is_null(&self) -> bool {
        use sqlx::any::AnyTypeInfoKind;
        self.0.kind == AnyTypeInfoKind::Null
    }
}

impl serde::Serialize for AnyTypeInfo {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("kind", &self.kind().to_string())?;
        map.serialize_entry("name", &self.name())?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for AnyTypeInfo {
    fn deserialize<D: serde::Deserializer<'de>>(_d: D) -> Result<Self, D::Error> {
        Err(serde::de::Error::custom(
            "sqlx::any::AnyTypeInfo cannot be reconstructed from JSON",
        ))
    }
}

mod emit_impls {
    use super::AnyTypeInfo;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for AnyTypeInfo {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { { unimplemented!("AnyTypeInfo cannot be reconstructed as a code literal") } }
        }
    }
}

impl elicitation::ElicitComplete for AnyTypeInfo {}
