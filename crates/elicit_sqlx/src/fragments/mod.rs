//! Fragment tools for sqlx compile-time macros.
//!
//! Each tool emits a Rust source fragment wrapping a `sqlx` macro invocation.
//! The emitted code requires `DATABASE_URL` to be set in the build environment
//! of the consuming binary.

mod migrate;
mod query;
mod query_as;
mod query_scalar;

pub use migrate::MigrateParams;
pub use query::QueryParams;
pub use query_as::QueryAsParams;
pub use query_scalar::QueryScalarParams;
