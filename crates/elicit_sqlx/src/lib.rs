//! Elicitation-enabled sqlx type wrappers.
//!
//! Provides newtypes for sqlx types with [`JsonSchema`], MCP reflect methods,
//! runtime database tools, and fragment tools for sqlx compile-time macros.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod args;
mod column;
mod context;
pub mod drivers;
mod error;
mod frag_plugin;
pub mod fragments;
mod plugin;
mod query_result;
mod row;
mod type_info;

pub use args::{ToSqlxArgs, ToSqlxArgsFactory};
pub use column::AnyColumn;
pub use context::{SqlxContext, connect};
pub use drivers::{SqlxMySqlPlugin, SqlxPgPlugin, SqlxSqlitePlugin};
pub use error::SqlxError;
pub use frag_plugin::SqlxFragPlugin;
pub use fragments::{MigrateParams, QueryAsParams, QueryParams, QueryScalarParams};
pub use plugin::SqlxPlugin;
pub use query_result::{AnyQueryResult, QueryResultData};
pub use row::AnyRow;
pub use type_info::AnyTypeInfo;
