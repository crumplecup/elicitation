//! Elicitation bridge types for [`surrealdb_types`] values.
//!
//! Each type in this module is a trenchcoat wrapper that adds
//! [`schemars::JsonSchema`] (required for MCP boundary crossing) to the
//! corresponding upstream `surrealdb-types` type.
//!
//! # Types
//!
//! - [`Value`] — shadow of `surrealdb_types::Value`
//! - [`RecordId`] — shadow of `surrealdb_types::RecordId`
//! - [`Number`] — shadow of `surrealdb_types::Number`
//! - [`Geometry`] — shadow of `surrealdb_types::Geometry`
//! - [`Datetime`] — ISO 8601 newtype around `surrealdb_types::Datetime`
//! - [`Duration`] — SurrealDB duration string newtype
//! - [`Kind`] — shadow of `surrealdb_types::Kind`
//! - [`Table`] — string newtype for table names
//! - [`PatchOp`] — shadow of JSON Patch operations
//!
//! # Enabled by the `surreal-types` feature

mod datetime;
mod duration;
mod geometry;
mod kind;
mod number;
mod patch_op;
mod record_id;
mod table;
mod value;

pub use datetime::Datetime;
pub use duration::Duration;
pub use geometry::Geometry;
pub use kind::Kind;
pub use number::Number;
pub use patch_op::PatchOp;
pub use record_id::RecordId;
pub use table::Table;
pub use value::Value;
