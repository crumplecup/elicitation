//! Elicitation-enabled SurrealDB type wrappers.
//!
//! Provides MCP tools for SurrealDB 3.x authoring:
//!
//! - [`SurrealSchemaPlugin`] — DDL tools emitting SurrealQL `DEFINE`/`REMOVE` statements
//! - [`SurrealCrudPlugin`] — DML tools emitting SurrealQL queries and Rust SDK snippets
//! - [`SurrealConnectionPlugin`] — connection/auth setup code generation
//! - [`SurrealSelectPlugin`] — stateful `SELECT` query builder
//! - [`SurrealTransactionPlugin`] — stateful transaction block builder

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod auth;
mod config;
mod connection_plugin;
mod crud_plugin;
mod schema_plugin;
mod select_plugin;
mod trait_factories;
mod txn_plugin;
mod types;

pub use auth::{Database, Namespace, Root, Token};
pub use config::{Capabilities, Config, ExperimentalFeature, PlannerStrategy};
pub use connection_plugin::SurrealConnectionPlugin;
pub use crud_plugin::SurrealCrudPlugin;
pub use schema_plugin::SurrealSchemaPlugin;
pub use select_plugin::SurrealSelectPlugin;
pub use trait_factories::prime_surreal_value;
pub use txn_plugin::SurrealTransactionPlugin;
pub use types::{Datetime, Duration, Geometry, Kind, Number, PatchOp, RecordId, Table, Value};
