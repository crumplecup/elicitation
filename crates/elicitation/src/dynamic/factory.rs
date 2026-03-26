//! [`AnyToolFactory`] — object-safe factory submitted to inventory at compile time.
//!
//! Each `#[reflect_trait(their::Trait)]` invocation generates one factory and
//! submits it via `inventory::submit!(ToolFactoryRegistration { ... })`.
//!
//! At runtime, [`DynamicToolRegistry`](super::DynamicToolRegistry) collects all
//! registrations from inventory and uses them to:
//!
//! 1. Expose factory meta-tools in `list_tools` (always visible).
//! 2. Instantiate [`DynamicToolDescriptor`](super::DynamicToolDescriptor)s when
//!    an agent calls the meta-tool.

use super::{DynamicToolDescriptor, slot::AnyToolSlot};
use rmcp::ErrorData;

/// Object-safe factory that knows how to produce tools for one third-party trait.
///
/// # Object Safety
///
/// This trait is intentionally object-safe: no generic methods, no `Self`
/// requirements.  Factories live behind `&'static dyn AnyToolFactory`.
///
/// # Implementation
///
/// Implement this trait manually (or via `#[reflect_trait]` once the macro
/// exists) and submit to inventory:
///
/// ```rust,ignore
/// inventory::submit!(ToolFactoryRegistration {
///     trait_name: "my_crate::MyTrait",
///     factory: &MyToolFactory,
/// });
/// ```
pub trait AnyToolFactory: Send + Sync + 'static {
    /// Fully-qualified name of the third-party trait this factory wraps.
    ///
    /// Used as the unique key for the factory meta-tool name and for routing.
    fn trait_name(&self) -> &'static str;

    /// Human-readable description shown to agents in the factory meta-tool.
    fn factory_description(&self) -> &'static str;

    /// Names of the individual tools this factory can produce.
    fn method_names(&self) -> &'static [&'static str];

    /// Produce [`DynamicToolDescriptor`]s for the given type slot.
    ///
    /// The factory should call [`TypedSlot::downcast_ref`](super::slot::TypedSlot::downcast_ref)
    /// to confirm the slot holds the type it expects, then generate one
    /// descriptor per method.
    ///
    /// Returns an error if the slot's type is incompatible with this factory.
    fn instantiate(&self, slot: &dyn AnyToolSlot) -> Result<Vec<DynamicToolDescriptor>, ErrorData>;
}

/// Inventory key connecting a static factory to the global registry.
///
/// Submit via `inventory::submit!` in the crate that defines the factory.
/// Collected by [`DynamicToolRegistry::new`](super::DynamicToolRegistry::new)
/// at server startup.
pub struct ToolFactoryRegistration {
    /// Trait name — must match [`AnyToolFactory::trait_name`].
    pub trait_name: &'static str,
    /// Reference to the static factory singleton.
    pub factory: &'static dyn AnyToolFactory,
}

inventory::collect!(ToolFactoryRegistration);
