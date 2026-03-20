//! Elicitation implementations for sqlx types.
//!
//! Provides [`Elicitation`](crate::Elicitation) for sqlx types that can be
//! interactively constructed, plus serializable bridge types for crossing
//! the MCP boundary.
//!
//! # Select enums (choose one variant)
//!
//! - [`sqlx::error::ErrorKind`] — database constraint violation category
//! - [`sqlx::any::AnyTypeInfoKind`] — SQL column type category (Null, Bool, SmallInt, etc.)
//! - [`ColumnValue`] — owned, serializable SQL value (mirrors AnyValueKind)
//! - [`SqlTypeKind`] — owned, serializable column type category (mirrors AnyTypeInfoKind)
//!
//! # Survey types (multi-field construction)
//!
//! - [`sqlx::any::AnyTypeInfo`] — wraps AnyTypeInfoKind
//! - [`sqlx::any::AnyQueryResult`] — rows_affected + last_insert_id
//! - [`ColumnDescriptor`] — serializable column metadata (name + ordinal + type_kind)
//! - [`ColumnEntry`] — serializable column name/value pair
//! - [`RowData`] — serializable SQL row (Vec<ColumnEntry>)
//!
//! # Enabled by the `sqlx-types` feature

mod any_query_result;
mod any_type_info;
mod column_descriptor;
mod column_value;
mod error_kind;
mod row_data;
mod sql_type_kind;
mod type_info_kind;

pub use any_query_result::AnyQueryResultStyle;
pub use any_type_info::AnyTypeInfoStyle;
pub use column_descriptor::{ColumnDescriptor, ColumnDescriptorStyle};
pub use column_value::{ColumnValue, ColumnValueStyle};
pub use error_kind::SqlxErrorKindStyle;
pub use row_data::{ColumnEntry, ColumnEntryStyle, RowData, RowDataStyle};
pub use sql_type_kind::{SqlTypeKind, SqlTypeKindStyle};
pub use type_info_kind::AnyTypeInfoKindStyle;
