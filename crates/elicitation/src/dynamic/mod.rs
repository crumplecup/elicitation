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

pub mod contextual;
pub mod factory;
pub mod meta_tool;
pub mod slot;

pub use contextual::ContextualFactory;
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
use contextual::ContextualEntry;

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
    /// Contextual entries registered via `register_contextual`.
    ///
    /// Stored separately so re-registration replaces the old entry without
    /// touching inventory-instantiated tools.
    contextual_entries: Arc<RwLock<Vec<ContextualEntry>>>,
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
            contextual_entries: Arc::new(RwLock::new(Vec::new())),
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

    /// Register a serde-mediated type-to-type conversion tool.
    ///
    /// Creates a concrete `DynamicToolDescriptor` whose input schema is `T`'s
    /// JSON Schema and whose handler deserializes the params as `T`, re-encodes
    /// via `serde_json`, then deserializes as `U`.  This works whenever `T` and
    /// `U` are structurally compatible in serde's data model (e.g. schema
    /// migration, newtype unwrapping, field renaming via `#[serde]` attributes).
    ///
    /// The tool is named `convert__{t}__to__{u}` where `t` / `u` are the
    /// [`type_name`](std::any::type_name) leaf segments converted to snake_case.
    /// These names are intentionally back-of-house; use them as building blocks
    /// when constructing agent workflows.
    ///
    /// # Panics
    ///
    /// Panics if a conversion tool with the same auto-generated name is already
    /// registered.
    #[instrument(skip(self), fields(tool_name))]
    pub fn register_convert<T, U>(self) -> Self
    where
        T: Serialize + DeserializeOwned + JsonSchema + Send + Sync + 'static,
        U: Serialize + DeserializeOwned + JsonSchema + Send + Sync + 'static,
    {
        let t_seg = type_leaf_snake::<T>();
        let u_seg = type_leaf_snake::<U>();
        let tool_name = format!("convert__{t_seg}__to__{u_seg}");
        tracing::Span::current().record("tool_name", tool_name.as_str());

        let schema = serde_json::to_value(schemars::schema_for!(T)).unwrap_or_default();
        let t_name = std::any::type_name::<T>();
        let u_name = std::any::type_name::<U>();
        let description =
            format!("Convert a `{t_name}` value to `{u_name}` via serde structural mapping.");

        let handler: Arc<
            dyn Fn(serde_json::Value) -> BoxFuture<'static, Result<CallToolResult, ErrorData>>
                + Send
                + Sync,
        > = Arc::new(move |params| {
            Box::pin(async move {
                let t: T = serde_json::from_value(params).map_err(|e| {
                    ErrorData::invalid_params(format!("failed to deserialize {t_name}: {e}"), None)
                })?;
                let intermediate = serde_json::to_value(&t).map_err(|e| {
                    ErrorData::internal_error(format!("failed to serialize {t_name}: {e}"), None)
                })?;
                let u: U = serde_json::from_value(intermediate).map_err(|e| {
                    ErrorData::invalid_params(
                        format!("conversion from {t_name} to {u_name} failed: {e}"),
                        None,
                    )
                })?;
                let text = serde_json::to_string(&u).map_err(|e| {
                    ErrorData::internal_error(format!("failed to serialize {u_name}: {e}"), None)
                })?;
                Ok(CallToolResult::success(vec![Content::text(text)]))
            })
        });

        let descriptor = DynamicToolDescriptor {
            name: tool_name.clone(),
            description,
            schema,
            handler,
        };
        let mut tools = self
            .dynamic_tools
            .write()
            .expect("dynamic_tools lock poisoned");
        assert!(
            !tools.iter().any(|d| d.name == tool_name),
            "convert tool `{tool_name}` already registered"
        );
        tools.push(descriptor);
        debug!(tool_name, "Registered convert tool");
        drop(tools);
        self
    }

    /// Register a contextual factory with runtime data.
    ///
    /// Tools are instantiated immediately by calling
    /// [`ContextualFactory::instantiate`] with the supplied `context`.  The
    /// resulting [`DynamicToolDescriptor`]s are visible in `list_tools` right
    /// away.
    ///
    /// Calling this again with the same `prefix` **replaces** the previously
    /// generated tools for that prefix — old descriptors are dropped.  Pair
    /// with [`notify_tool_list_changed`](Self::notify_tool_list_changed) to
    /// push the updated list to connected agents.
    ///
    /// # Errors
    ///
    /// Returns `self` unchanged if the factory returns an error.  Errors are
    /// logged at `warn` level so the server can continue running.
    #[instrument(skip(self, factory, context), fields(prefix))]
    pub fn register_contextual<F>(
        self,
        prefix: impl Into<String>,
        factory: F,
        context: F::Context,
    ) -> Self
    where
        F: ContextualFactory,
    {
        let prefix = prefix.into();
        tracing::Span::current().record("prefix", prefix.as_str());
        match ContextualEntry::new(prefix.clone(), &factory, &context) {
            Ok(entry) => {
                let mut entries = self
                    .contextual_entries
                    .write()
                    .expect("contextual_entries lock poisoned");
                entries.retain(|e| e.prefix != prefix);
                entries.push(entry);
                debug!(%prefix, "Registered contextual factory");
            }
            Err(e) => {
                tracing::warn!(%prefix, error = ?e, "Contextual factory instantiation failed");
            }
        }
        self
    }

    /// Send `notify_tool_list_changed` to the connected agent.
    ///
    /// Call this after re-registering a contextual factory on a phase
    /// transition so the agent immediately sees the updated tool list.
    ///
    /// No-op if no peer is set (the agent must re-call `list_tools` manually).
    pub async fn notify_tool_list_changed(&self) {
        if let Some(peer) = self.peer.get() {
            if let Err(e) = peer.notify_tool_list_changed().await {
                warn!(error = ?e, "Failed to send notify_tool_list_changed");
            }
        } else {
            debug!("No peer set — agent must re-call list_tools manually");
        }
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
        self.notify_tool_list_changed().await;

        let summary = format!(
            "Instantiated {} tools for `{prefix}`: {}",
            new_names.len(),
            new_names.join(", ")
        );
        Ok(CallToolResult::success(vec![Content::text(summary)]))
    }

    /// Invoke a named dynamic tool with the given JSON argument object.
    ///
    /// Searches both inventory-instantiated tools and contextual entries.
    /// Returns `None` if no tool with that name has been instantiated yet.
    /// Returns `Some(Err(...))` if the tool handler returns an error.
    ///
    /// Useful for testing and for programmatic invocation without a live MCP
    /// connection (no `rmcp::RequestContext` required).
    pub async fn invoke_dynamic(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> Option<Result<CallToolResult, ErrorData>> {
        let handler = {
            // Search inventory-instantiated tools first
            let tools = self
                .dynamic_tools
                .read()
                .expect("dynamic_tools lock poisoned");
            if let Some(d) = tools.iter().find(|d| d.name == name) {
                d.handler.clone()
            } else {
                drop(tools);
                // Then search contextual entries
                let entries = self
                    .contextual_entries
                    .read()
                    .expect("contextual_entries lock poisoned");
                entries
                    .iter()
                    .flat_map(|e| e.descriptors.iter())
                    .find(|d| d.name == name)?
                    .handler
                    .clone()
            }
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
        drop(dynamic);
        let contextual = self
            .contextual_entries
            .read()
            .expect("contextual_entries lock poisoned");
        for entry in contextual.iter() {
            tools.extend(entry.descriptors.iter().map(|d| d.as_tool()));
        }
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
                if let Some(d) = tools.iter().find(|d| d.name == tool_name) {
                    d.handler.clone()
                } else {
                    drop(tools);
                    let entries = self
                        .contextual_entries
                        .read()
                        .expect("contextual_entries lock poisoned");
                    entries
                        .iter()
                        .flat_map(|e| e.descriptors.iter())
                        .find(|d| d.name == tool_name)
                        .ok_or_else(|| {
                            ErrorData::invalid_params(
                                format!("tool `{tool_name}` not found in dynamic registry"),
                                None,
                            )
                        })?
                        .handler
                        .clone()
                }
                // guard dropped here
            };

            let value = serde_json::Value::Object(args);
            handler(value).await
        })
    }
}

// ── Naming helpers ─────────────────────────────────────────────────────────────

/// Returns the last path segment of `type_name::<T>()` converted to snake_case.
///
/// `type_name::<MyStruct>()` → `"my_struct"`.
/// For generics like `Vec<T>`, angle brackets are stripped: `"vec"`.
fn type_leaf_snake<T: 'static>() -> String {
    let full = std::any::type_name::<T>();
    // Strip any generic angle-bracket suffix: "Vec<T>" → "Vec"
    let without_generics = full.split('<').next().unwrap_or(full);
    let leaf = without_generics
        .rsplit("::")
        .next()
        .unwrap_or(without_generics);
    camel_to_snake_rt(leaf)
}

/// Runtime CamelCase → snake_case conversion (mirrors the proc-macro version).
fn camel_to_snake_rt(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i != 0 {
            out.push('_');
        }
        out.extend(ch.to_lowercase());
    }
    out
}
