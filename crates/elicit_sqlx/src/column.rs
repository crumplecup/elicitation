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
