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
