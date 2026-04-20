//! Shadow types for `surrealdb::opt::auth` credential structs.
//!
//! These are plain parameter containers used by [`SurrealConnectionPlugin`] tools to
//! generate sign-in code snippets.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Root-level credentials (highest privilege).
///
/// Maps to `surrealdb::opt::auth::Root`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct Root {
    /// Username.
    pub username: String,
    /// Password.
    pub password: String,
}

/// Namespace-scoped credentials.
///
/// Maps to `surrealdb::opt::auth::Namespace`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct Namespace {
    /// Namespace to authenticate against.
    pub namespace: String,
    /// Username.
    pub username: String,
    /// Password.
    pub password: String,
}

/// Database-scoped credentials.
///
/// Maps to `surrealdb::opt::auth::Database`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct Database {
    /// Namespace containing the database.
    pub namespace: String,
    /// Database to authenticate against.
    pub database: String,
    /// Username.
    pub username: String,
    /// Password.
    pub password: String,
}

/// JWT / opaque token credential.
///
/// Maps to `surrealdb::opt::auth::Token`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct Token {
    /// Raw JWT or opaque token string.
    pub token: String,
}
