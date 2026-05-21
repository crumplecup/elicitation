//! [`ContextualFactory`] — runtime-bounded dynamic tool registration.
//!
//! Unlike [`AnyToolFactory`](super::AnyToolFactory), which is object-safe and
//! discovered via `inventory` at compile time, `ContextualFactory` is
//! monomorphized at the call site and registered explicitly with runtime context.
//!
//! # Motivation
//!
//! `register_type::<T>(prefix)` generates tool schemas from `schemars::schema_for!(T)` —
//! a static macro with no access to runtime values.  There is no way to reflect a
//! runtime constraint (e.g., `"maximum": player_bankroll`) into the schema.
//!
//! `ContextualFactory` solves this: the factory receives a typed `Context` at
//! registration time and can embed any runtime value into the schema or the set
//! of tools it generates.
//!
//! # Example
//!
//! ```rust,ignore
//! use elicitation::dynamic::{ContextualFactory, DynamicToolDescriptor};
//! use rmcp::ErrorData;
//! use serde_json::json;
//! use std::sync::Arc;
//! use futures::FutureExt;
//!
//! pub struct BetConstraints { pub min: u64, pub max: u64 }
//!
//! pub struct BetAmountFactory;
//!
//! impl ContextualFactory for BetAmountFactory {
//!     type Context = BetConstraints;
//!
//!     fn instantiate(
//!         &self,
//!         prefix: &str,
//!         ctx: &BetConstraints,
//!     ) -> Result<Vec<DynamicToolDescriptor>, ErrorData> {
//!         let max = ctx.max;
//!         let name = format!("{prefix}__place");
//!         Ok(vec![DynamicToolDescriptor {
//!             name: name.clone(),
//!             description: format!("Place a bet (max {max})"),
//!             schema: json!({
//!                 "type": "object",
//!                 "properties": {
//!                     "amount": { "type": "integer", "minimum": ctx.min, "maximum": max }
//!                 },
//!                 "required": ["amount"]
//!             }),
//!             handler: Arc::new(move |args| {
//!                 Box::pin(async move {
//!                     // validate and handle ...
//!                     Ok(rmcp::model::CallToolResult::success(vec![]))
//!                 })
//!             }),
//!         }])
//!     }
//! }
//!
//! // At betting phase start — re-register when bankroll changes:
//! let registry = registry.register_contextual(
//!     "bet",
//!     BetAmountFactory,
//!     BetConstraints { min: 1, max: player.bankroll() },
//! );
//! ```
//!
//! # Re-registration on phase transitions
//!
//! Calling `register_contextual` a second time with the same prefix **replaces**
//! the previously generated tools.  Pair this with
//! [`DynamicToolRegistry::notify_tool_list_changed`](crate::DynamicToolRegistry::notify_tool_list_changed) to push the updated tool
//! list to connected agents immediately.
//!
//! # Relationship to `AnyToolFactory`
//!
//! | Property | `AnyToolFactory` | `ContextualFactory` |
//! |---|---|---|
//! | Discovery | `inventory` (compile time) | Explicit (registration time) |
//! | Object safe | Yes (`&'static dyn`) | No (monomorphized) |
//! | Context | None | Typed associated type |
//! | Schema bounds | `schemars::schema_for!` | Fully runtime |
//! | `Context = ()` | — | Degenerates to no-op context |

use rmcp::ErrorData;
use tracing::instrument;

use super::DynamicToolDescriptor;

/// A tool factory that requires typed runtime context to instantiate tools.
///
/// Implement this trait for factories whose tool schemas or tool sets depend on
/// values not available at compile time (e.g., per-session limits, database
/// records, user permissions).
///
/// Use `type Context = ()` for factories that need no runtime data — the
/// compiler optimises away the unit reference and the call site is identical.
pub trait ContextualFactory {
    /// Runtime context needed to build tools.
    ///
    /// Use `()` when no runtime data is required.  Any type that is `Send +
    /// Sync + 'static` is accepted — typically a small config struct.
    type Context: Send + Sync + 'static;

    /// Produce [`DynamicToolDescriptor`]s for the given prefix and context.
    ///
    /// Called immediately by [`DynamicToolRegistry::register_contextual`](crate::DynamicToolRegistry::register_contextual).
    /// The returned descriptors replace any tools previously registered under
    /// the same prefix.
    ///
    /// # Errors
    ///
    /// Return an [`ErrorData`] if the context is invalid (e.g., `min > max`).
    fn instantiate(
        &self,
        prefix: &str,
        context: &Self::Context,
    ) -> Result<Vec<DynamicToolDescriptor>, ErrorData>;
}

/// Erased contextual entry stored in the registry.
///
/// Holds the pre-instantiated descriptors produced at registration time.
/// Re-registration replaces the entry entirely.
pub(super) struct ContextualEntry {
    pub prefix: String,
    pub descriptors: Vec<DynamicToolDescriptor>,
}

impl ContextualEntry {
    /// Instantiate a new entry from a factory and its context.
    #[instrument(skip(factory, context), fields(prefix))]
    pub fn new<F: ContextualFactory>(
        prefix: String,
        factory: &F,
        context: &F::Context,
    ) -> Result<Self, ErrorData> {
        tracing::Span::current().record("prefix", prefix.as_str());
        let descriptors = factory.instantiate(&prefix, context)?;
        tracing::debug!(
            count = descriptors.len(),
            %prefix,
            "Contextual factory instantiated tools"
        );
        Ok(Self {
            prefix,
            descriptors,
        })
    }
}
