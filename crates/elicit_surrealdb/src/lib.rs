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
//! | [`Array`] | `surrealdb_types::Array` |
//! | [`Object`] | `surrealdb_types::Object` |
//! | [`Bytes`] | `surrealdb_types::Bytes` |
//! | [`Uuid`] | `surrealdb_types::Uuid` |
//! | [`Set`] | `surrealdb_types::Set` |
//! | [`Range`] | `surrealdb_types::Range` |
//! | [`RecordIdKeyRange`] | `surrealdb_types::RecordIdKeyRange` |
//! | [`RecordId`] | `surrealdb_types::RecordId` |
//! | [`RecordIdKey`] | `surrealdb_types::RecordIdKey` |
//! | [`Regex`] | `surrealdb_types::Regex` |
//! | [`File`] | `surrealdb_types::File` |
//! | [`Kind`] | `surrealdb_types::Kind` |
//! | [`Number`] | `surrealdb_types::Number` |
//! | [`Datetime`] | `surrealdb_types::Datetime` |
//! | [`Duration`] | `surrealdb_types::Duration` |
//! | [`Table`] | `surrealdb_types::Table` |
//! | [`Geometry`] | `surrealdb_types::Geometry` |
//! | [`Notification`] | `surrealdb_types::Notification` |
//! | [`Action`] | `surrealdb_types::Action` |
//!
//! # Trait factories
//!
//! [`trait_factories`] exposes [`surrealdb_types::SurrealValue`] derive-trait methods
//! as MCP tools for any user type that implements the trait.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod admin_plugin;
mod array;
mod auth;
mod bytes_type;
mod config;
mod connection_plugin;
mod constructors;
mod control_plugin;
mod crud_plugin;
mod datetime;
mod duration;
mod file_type;
mod geometry;
mod info_plugin;
mod kind;
mod live_plugin;
mod notification;
mod number;
mod object;
mod range;
mod record_id;
mod record_id_key;
mod regex_type;
mod schema_plugin;
mod select_plugin;
mod set;
mod table;
pub mod trait_factories;
mod txn_plugin;
mod uuid_type;
mod value;

pub use admin_plugin::SurrealAdminPlugin;
pub use array::Array;
pub use auth::{AuthDatabase, AuthNamespace, AuthRecord, AuthRoot};
pub use bytes_type::Bytes;
pub use config::{ExperimentalFeature, PlannerStrategy, SurrealCapabilities, SurrealConfig};
pub use connection_plugin::SurrealConnectionPlugin;
pub use constructors::SurrealConstructorsPlugin;
pub use control_plugin::SurrealControlPlugin;
pub use crud_plugin::SurrealCrudPlugin;
pub use datetime::Datetime;
pub use duration::Duration;
pub use file_type::File;
pub use geometry::Geometry;
pub use info_plugin::SurrealInfoPlugin;
pub use kind::Kind;
pub use live_plugin::SurrealLivePlugin;
pub use notification::{Action, Notification};
pub use number::Number;
pub use object::Object;
pub use range::{Range, RecordIdKeyRange};
pub use record_id::RecordId;
pub use record_id_key::RecordIdKey;
pub use regex_type::Regex;
pub use schema_plugin::SurrealSchemaPlugin;
pub use select_plugin::SurrealSelectPlugin;
pub use set::Set;
pub use table::Table;
pub use trait_factories::SurrealValueFactory;
pub use txn_plugin::SurrealTransactionPlugin;
pub use uuid_type::Uuid;
pub use value::Value;
