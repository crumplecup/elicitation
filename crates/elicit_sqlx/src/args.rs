//! `ToSqlxArgs` — bridge trait for converting any [`Elicitation`] type into
//! positional SQL arguments.
//!
//! # How it works
//!
//! Any type `T` that derives [`Elicitation`] (which implies `Serialize +
//! DeserializeOwned + JsonSchema`) automatically implements [`ToSqlxArgs`] via
//! a blanket impl.  The impl serializes `T` to JSON and extracts the field
//! values in declaration order as a flat `Vec<serde_json::Value>`.
//!
//! The resulting vector can be passed directly as `args` to any driver plugin
//! tool that accepts positional parameters:
//!
//! ```text
//! my_type__to_sqlx_args { target: { name: "Alice", age: 30 } }
//!     → { result: ["Alice", 30] }
//!
//! pg__execute { pool_id: "…", sql: "INSERT INTO users(name,age) VALUES($1,$2)",
//!               args: ["Alice", 30] }
//!     → { rows_affected: 1 }
//! ```
//!
//! # Field ordering
//!
//! Fields are emitted in the order they appear in the JSON serialization of
//! `T`.  For types that derive `serde::Serialize`, this matches the struct
//! declaration order.  Renamed or skipped fields follow serde attribute rules.
//!
//! For non-object JSON values (e.g. a newtype wrapping a single scalar),
//! the scalar itself is wrapped in a single-element `Vec`.
//!
//! # Factory usage
//!
//! Register a user type at server startup to get a typed binding tool:
//!
//! ```rust,ignore
//! prime_to_sqlx_args::<CreateUser>();
//! registry.register_type::<CreateUser>("create_user");
//! // Agent can now call: create_user__to_sqlx_args { target: { … } }
//! ```

use elicitation_derive::reflect_trait;
use serde::Serialize;

/// Convert `self` into a flat list of positional SQL argument values.
///
/// Implemented automatically for all types that derive [`Elicitation`].
/// The return value is suitable for direct use as the `args` field in
/// `*__execute`, `*__fetch_all`, `*__fetch_one`, and `*__fetch_optional`
/// driver tools.
pub trait ToSqlxArgs {
    /// Serialize `self` into ordered positional SQL argument values.
    fn to_sqlx_args(&self) -> Vec<serde_json::Value>;
}

impl<T> ToSqlxArgs for T
where
    T: elicitation::Elicitation + Serialize,
{
    fn to_sqlx_args(&self) -> Vec<serde_json::Value> {
        match serde_json::to_value(self).unwrap_or(serde_json::Value::Null) {
            serde_json::Value::Object(map) => map.into_values().collect(),
            other => vec![other],
        }
    }
}

/// Expose [`ToSqlxArgs`] as an agent-callable MCP tool factory.
///
/// For each registered type `T`, contributes one tool:
/// - `{prefix}__to_sqlx_args` — convert a `T` value into positional SQL args
///
/// The `target` parameter must be the JSON representation of the `T` value.
/// The output `Vec<serde_json::Value>` can be passed directly as `args` to
/// any driver plugin execute or fetch tool.
#[reflect_trait(crate::ToSqlxArgs)]
pub trait ToSqlxArgsTools {
    /// Serialize this value to a list of positional SQL argument values.
    fn to_sqlx_args(&self) -> Vec<serde_json::Value>;
}
