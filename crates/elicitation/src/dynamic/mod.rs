//! [`DynamicToolRegistry`] — runtime middleware between inventory and the MCP server.
//!
//! The registry bridges two worlds:
//!
//! - **Compile-time**: [`AnyToolFactory`] instances submitted to inventory via
//!   `inventory::submit!`.
//! - **Runtime**: concrete types registered at startup via
//!   [`DynamicToolRegistry::register_type`].
//!
//! When an agent calls a factory meta-tool (e.g. `"instantiate_my_trait"`),
//! the registry instantiates [`DynamicToolDescriptor`]s for the requested type
//! and fires `notify_tool_list_changed` so the agent can immediately see the
//! new tools.
//!
//! # Lifecycle
//!
//! ```text
//! COMPILE TIME   inventory::submit!(ToolFactoryRegistration { ... })
//!                    (macro generates factory + submission)
//!
//! STARTUP TIME   registry.register_type::<User>("user")
//!                    (monomorphization: TypedSlot<User> created + stored)
//!
//! REQUEST TIME   agent calls "instantiate_my_trait" { prefix: "user" }
//!                    → factory.instantiate(slot) → Vec<DynamicToolDescriptor>
//!                    → peer.notify_tool_list_changed()
//!                    → agent re-calls list_tools, sees "user__my_method"
//! ```

pub mod factory;
pub mod meta_tool;
pub mod slot;

pub use factory::{AnyToolFactory, ToolFactoryRegistration};
pub use slot::{AnyToolSlot, TypedSlot};

use std::{
    collections::HashMap,
    sync::{Arc, OnceLock, RwLock},
};

use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::{Peer, RequestContext},
};
use schemars::JsonSchema;
use serde::{Serialize, de::DeserializeOwned};
use tracing::{debug, instrument, warn};

use crate::{plugin::ElicitPlugin, rmcp::RoleServer, traits::Elicitation};

// ── DynamicToolDescriptor ──────────────────────────────────────────────────────

/// A tool whose handler is created at runtime via a factory.
///
/// Unlike static [`ToolDescriptor`](crate::plugin::ToolDescriptor)s, these are
/// generated when an agent calls a factory meta-tool.  All type safety is
/// enforced at registration time inside [`DynamicToolRegistry::register_type`].
pub struct DynamicToolDescriptor {
    /// MCP tool name (e.g. `"user__insert"`).
    pub name: String,
    /// Description shown to the agent.
    pub description: String,
    /// JSON Schema for this tool's parameters.
    pub schema: serde_json::Value,
    /// Type-erased async handler.
    ///
    /// Receives a `serde_json::Value` of the tool's arguments object and returns
    /// an MCP `CallToolResult`.  All bounds (`Serialize`, `DeserializeOwned`,
    /// `JsonSchema`, `Elicit`) are enforced when the closure is created inside
    /// `register_type::<T>()`.
    pub handler: Arc<
        dyn Fn(serde_json::Value) -> BoxFuture<'static, Result<CallToolResult, ErrorData>>
            + Send
            + Sync,
    >,
}

impl std::fmt::Debug for DynamicToolDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicToolDescriptor")
            .field("name", &self.name)
            .field("description", &self.description)
            .finish()
    }
}

impl DynamicToolDescriptor {
    /// Convert to an rmcp [`Tool`] (schema + metadata, no handler).
    pub fn as_tool(&self) -> Tool {
        let schema_obj = match &self.schema {
            serde_json::Value::Object(m) => Arc::new(m.clone()),
            _ => Arc::new(Default::default()),
        };
        Tool::new(self.name.clone(), self.description.clone(), schema_obj)
    }
}

// ── DynamicToolRegistry ────────────────────────────────────────────────────────

/// Runtime middleware that manages factory meta-tools and instantiated dynamic tools.
///
/// Implements [`ElicitPlugin`] so it drops into [`PluginRegistry`](crate::PluginRegistry)
/// alongside static plugins.
///
/// # Usage
///
/// ```rust,ignore
/// use elicitation::dynamic::DynamicToolRegistry;
/// use schemars::JsonSchema;
/// use serde::{Deserialize, Serialize};
///
/// // T must impl Serialize + DeserializeOwned + JsonSchema + Elicitation
/// #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
/// struct MyType { value: String }
///
/// // impl Elicitation for MyType { ... }
///
/// let registry = DynamicToolRegistry::new()
///     .register_type::<MyType>("my_type");
/// ```
#[derive(Clone)]
pub struct DynamicToolRegistry {
    /// Factories from inventory, collected once at construction.
    factories: Vec<&'static dyn AnyToolFactory>,
    /// Registered type slots, keyed by user-provided prefix.
    slots: Arc<RwLock<HashMap<String, Box<dyn AnyToolSlot>>>>,
    /// Instantiated dynamic tools — populated by factory meta-tool calls.
    dynamic_tools: Arc<RwLock<Vec<DynamicToolDescriptor>>>,
    /// rmcp peer handle for `notify_tool_list_changed`.
    ///
    /// Injected by [`PluginRegistry`](crate::PluginRegistry) after the server
    /// starts (future work).  When absent, instantiation still works but the
    /// agent must re-call `list_tools` manually.
    peer: Arc<OnceLock<Peer<RoleServer>>>,
}

impl DynamicToolRegistry {
    /// Collect all factories from inventory and create an empty registry.
    #[instrument]
    pub fn new() -> Self {
        let factories: Vec<&'static dyn AnyToolFactory> =
            inventory::iter::<ToolFactoryRegistration>
                .into_iter()
                .map(|r| r.factory)
                .collect();

        debug!(
            count = factories.len(),
            "Collected tool factories from inventory"
        );

        Self {
            factories,
            slots: Arc::new(RwLock::new(HashMap::new())),
            dynamic_tools: Arc::new(RwLock::new(Vec::new())),
            peer: Arc::new(OnceLock::new()),
        }
    }

    /// Register a concrete type `T` under the given prefix.
    ///
    /// Creates a [`TypedSlot<T>`] and stores it.  Monomorphization of handler
    /// closures happens later in each factory's `instantiate()` method.
    ///
    /// # Panics
    ///
    /// Panics if `prefix` was already registered.
    #[instrument(skip(self), fields(prefix))]
    pub fn register_type<T>(self, prefix: impl Into<String>) -> Self
    where
        T: Serialize + DeserializeOwned + JsonSchema + Elicitation + Send + Sync + 'static,
    {
        let prefix = prefix.into();
        tracing::Span::current().record("prefix", prefix.as_str());
        let mut slots = self.slots.write().expect("slots lock poisoned");
        assert!(
            !slots.contains_key(&prefix),
            "prefix `{prefix}` already registered in DynamicToolRegistry"
        );
        let slot: Box<dyn AnyToolSlot> = Box::new(TypedSlot::<T>::new(prefix.clone()));
        slots.insert(prefix.clone(), slot);
        debug!(%prefix, type_name = std::any::type_name::<T>(), "Registered type slot");
        drop(slots);
        self
    }

    /// Inject the rmcp peer so `notify_tool_list_changed` can be sent.
    ///
    /// Called by the server setup after the connection is established.
    pub fn set_peer(&self, peer: Peer<RoleServer>) {
        let _ = self.peer.set(peer);
    }

    /// Instantiate tools for the given trait + prefix combination.
    ///
    /// Creates [`DynamicToolDescriptor`]s for the named factory and registered
    /// prefix, then fires `notify_tool_list_changed` if a peer is set.
    ///
    /// Calling this again for the same `prefix` is idempotent — existing tools
    /// for that prefix are replaced.
    ///
    /// # Errors
    ///
    /// Returns an error if no factory is registered for `trait_name`, or if no
    /// type was registered under `prefix`.
    #[instrument(skip(self))]
    pub async fn instantiate(
        &self,
        trait_name: &str,
        prefix: &str,
    ) -> Result<CallToolResult, ErrorData> {
        self.instantiate_for(trait_name, prefix).await
    }

    /// Instantiate tools for the given trait + prefix combination.
    #[instrument(skip(self))]
    async fn instantiate_for(
        &self,
        trait_name: &str,
        prefix: &str,
    ) -> Result<CallToolResult, ErrorData> {
        // Find the factory (from Vec, no lock needed)
        let factory = self
            .factories
            .iter()
            .find(|f| f.trait_name() == trait_name)
            .ok_or_else(|| {
                ErrorData::invalid_params(
                    format!("no factory registered for trait `{trait_name}`"),
                    None,
                )
            })?;

        // Acquire slots lock, instantiate descriptors, then drop lock before any await
        let (new_descriptors, new_names) = {
            let slots = self.slots.read().expect("slots lock poisoned");
            let slot = slots.get(prefix).ok_or_else(|| {
                ErrorData::invalid_params(
                    format!(
                        "no type registered under prefix `{prefix}`. \
                         Call register_type::<T>(\"{prefix}\") at startup."
                    ),
                    None,
                )
            })?;
            let descriptors = factory.instantiate(slot.as_ref())?;
            let names: Vec<String> = descriptors.iter().map(|d| d.name.clone()).collect();
            (descriptors, names)
            // slots guard dropped here
        };

        debug!(
            trait_name,
            prefix,
            count = new_descriptors.len(),
            "Instantiated dynamic tools"
        );

        // Store descriptors — acquire, mutate, drop before await
        {
            let mut tools = self
                .dynamic_tools
                .write()
                .expect("dynamic_tools lock poisoned");
            tools.retain(|d| !d.name.starts_with(&format!("{prefix}__")));
            tools.extend(new_descriptors);
            // tools guard dropped here
        }

        // Notify the agent that the tool list changed (await is safe here — no locks held)
        if let Some(peer) = self.peer.get() {
            if let Err(e) = peer.notify_tool_list_changed().await {
                warn!(error = ?e, "Failed to send notify_tool_list_changed");
            }
        } else {
            debug!("No peer set — agent must re-call list_tools manually");
        }

        let summary = format!(
            "Instantiated {} tools for `{prefix}`: {}",
            new_names.len(),
            new_names.join(", ")
        );
        Ok(CallToolResult::success(vec![Content::text(summary)]))
    }

    /// Invoke a named dynamic tool with the given JSON argument object.
    ///
    /// Returns `None` if no tool with that name has been instantiated yet.
    /// Returns `Some(Err(...))` if the tool handler returns an error.
    ///
    /// Useful for testing and for programmatic invocation without a live MCP
    /// connection (no [`rmcp::RequestContext`] required).
    pub async fn invoke_dynamic(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> Option<Result<CallToolResult, ErrorData>> {
        let handler = {
            let tools = self
                .dynamic_tools
                .read()
                .expect("dynamic_tools lock poisoned");
            tools.iter().find(|d| d.name == name)?.handler.clone()
        };
        Some(handler(args).await)
    }

    /// Return all factory meta-tools as [`Tool`] entries.
    fn factory_meta_tools(&self) -> Vec<Tool> {
        self.factories
            .iter()
            .map(|f| meta_tool::make_meta_tool(*f))
            .collect()
    }
}

impl Default for DynamicToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ElicitPlugin for DynamicToolRegistry {
    fn name(&self) -> &'static str {
        "dynamic"
    }

    fn list_tools(&self) -> Vec<Tool> {
        let mut tools = self.factory_meta_tools();
        let dynamic = self
            .dynamic_tools
            .read()
            .expect("dynamic_tools lock poisoned");
        tools.extend(dynamic.iter().map(|d| d.as_tool()));
        tools
    }

    #[instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        let tool_name = params.name.to_string();
        let args = params.arguments.clone().unwrap_or_default();

        Box::pin(async move {
            // Check if it's a factory meta-tool call: "instantiate_{snake_trait}"
            if let Some(trait_name) = self
                .factories
                .iter()
                .find(|f| meta_tool::meta_tool_name(f.trait_name()) == tool_name)
                .map(|f| f.trait_name())
            {
                let prefix = args
                    .get("prefix")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ErrorData::invalid_params("missing `prefix` argument", None))?
                    .to_string();

                return self.instantiate_for(trait_name, &prefix).await;
            }

            // Route to a live dynamic tool — clone handler inside block so guard is dropped
            let handler = {
                let tools = self
                    .dynamic_tools
                    .read()
                    .expect("dynamic_tools lock poisoned");
                tools
                    .iter()
                    .find(|d| d.name == tool_name)
                    .ok_or_else(|| {
                        ErrorData::invalid_params(
                            format!("tool `{tool_name}` not found in dynamic registry"),
                            None,
                        )
                    })?
                    .handler
                    .clone()
                // guard dropped here
            };

            let value = serde_json::Value::Object(args);
            handler(value).await
        })
    }
}
