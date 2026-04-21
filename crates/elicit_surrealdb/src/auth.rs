//! Shadow types for `surrealdb::opt::auth`.
//!
//! Provides elicitation-complete credential types for SurrealDB authentication.
//! Each type mirrors the upstream struct field-for-field and implements
//! [`elicitation::ElicitComplete`] so user types can `#[derive(Elicit)]`
//! with these as fields.

use elicitation::Elicit;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Credentials for the root user.
///
/// Mirrors `surrealdb::opt::auth::Root`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct AuthRoot {
    /// The username of the root user.
    pub username: String,
    /// The password of the root user.
    pub password: String,
}

/// Credentials for a namespace user.
///
/// Mirrors `surrealdb::opt::auth::Namespace`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct AuthNamespace {
    /// The namespace the user has access to.
    pub namespace: String,
    /// The username of the namespace user.
    pub username: String,
    /// The password of the namespace user.
    pub password: String,
}

/// Credentials for a database user.
///
/// Mirrors `surrealdb::opt::auth::Database`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct AuthDatabase {
    /// The namespace the user has access to.
    pub namespace: String,
    /// The database the user has access to.
    pub database: String,
    /// The username of the database user.
    pub username: String,
    /// The password of the database user.
    pub password: String,
}

/// Credentials for a record user (scope-based authentication).
///
/// The upstream `surrealdb::opt::auth::Record<P>` is generic over `P:
/// SurrealValue`. This shadow type uses a JSON string for `params` so it
/// remains concretely [`elicitation::ElicitComplete`] without a generic.
/// Serialize the params value to JSON before placing it here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct AuthRecord {
    /// The namespace the user has access to.
    pub namespace: String,
    /// The database the user has access to.
    pub database: String,
    /// The access method name.
    pub access: String,
    /// Additional params as a JSON string (parsed and flattened into the payload).
    pub params_json: String,
}
