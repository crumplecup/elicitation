//! Trait factory for `surrealdb::types::SurrealValue`.
//!
//! Provides [`prime_surreal_value`] which registers a user type that implements
//! `surrealdb::types::SurrealValue` (the serialization trait) into the elicitation
//! type registry, making it available as a parameter type in MCP tools.

use elicitation::ElicitComplete;

/// Register a user type `T` that satisfies [`ElicitComplete`] into the
/// elicitation type registry so it can be used as an MCP parameter type.
///
/// The bound is intentionally kept to `ElicitComplete` so that the codegen
/// crate does not need to link against the SurrealDB SDK. To verify that `T`
/// also implements `surrealdb::types::SurrealValue`, add that bound in your
/// own crate that already depends on `surrealdb`.
///
/// # Example
///
/// ```rust,ignore
/// use elicit_surrealdb::prime_surreal_value;
///
/// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
/// #[derive(elicitation::ElicitComplete)]
/// struct MyRecord { id: String }
///
/// prime_surreal_value::<MyRecord>();
/// ```
pub fn prime_surreal_value<T: ElicitComplete>() {
    // Registration is handled by `#[derive(ElicitComplete)]` inventory submission.
    // This function acts as a compile-time proof that `T` satisfies the bound
    // and as a visible call site for documentation purposes.
    let _ = std::marker::PhantomData::<T>;
}
