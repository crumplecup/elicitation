//! `elicit_surrealdb` — elicitation-enabled newtype wrappers for `surrealdb-types`.
//!
//! Each wrapper:
//! - Implements [`schemars::JsonSchema`] so the type can appear in MCP tool schemas
//! - Provides transparent [`Deref`]/[`DerefMut`] access to the inner type
//! - Exposes public API methods via `#[reflect_methods]` as MCP tools
//! - Implements [`elicitation::ElicitComplete`] so it can be used in `#[derive(Elicit)]` structs
//!
//! # Supported types
//!
//! | Wrapper | Inner type |
//! |---------|-----------|
//! | [`Value`] | `surrealdb_types::Value` |
//! | [`Kind`] | `surrealdb_types::Kind` |
//! | [`Number`] | `surrealdb_types::Number` |
//! | [`Datetime`] | `surrealdb_types::Datetime` |
//! | [`Duration`] | `surrealdb_types::Duration` |
//! | [`Table`] | `surrealdb_types::Table` |
//! | [`RecordId`] | `surrealdb_types::RecordId` |
//! | [`Geometry`] | `surrealdb_types::Geometry` |
//!
//! # Trait factories
//!
//! [`trait_factories`] exposes [`surrealdb_types::SurrealValue`] derive-trait methods
//! as MCP tools for any user type that implements the trait.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod auth;
mod config;
mod connection_plugin;
mod constructors;
mod crud_plugin;
mod datetime;
mod duration;
mod geometry;
mod kind;
mod number;
mod record_id;
mod schema_plugin;
mod select_plugin;
mod table;
pub mod trait_factories;
mod txn_plugin;
mod value;

pub use auth::{AuthDatabase, AuthNamespace, AuthRecord, AuthRoot};
pub use config::{ExperimentalFeature, PlannerStrategy, SurrealCapabilities, SurrealConfig};
pub use connection_plugin::SurrealConnectionPlugin;
pub use constructors::SurrealConstructorsPlugin;
pub use crud_plugin::SurrealCrudPlugin;
pub use datetime::Datetime;
pub use duration::Duration;
pub use geometry::Geometry;
pub use kind::Kind;
pub use number::Number;
pub use record_id::RecordId;
pub use schema_plugin::SurrealSchemaPlugin;
pub use select_plugin::SurrealSelectPlugin;
pub use table::Table;
pub use trait_factories::SurrealValueFactory;
pub use txn_plugin::SurrealTransactionPlugin;
pub use value::Value;
