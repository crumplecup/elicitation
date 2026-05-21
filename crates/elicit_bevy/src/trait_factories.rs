//! `#[reflect_trait]` factories and manual factories for core Bevy traits.
//!
//! These factories enable agents to work with any user type that implements a
//! core Bevy trait — without writing extra glue code.
//!
//! # Orphan rule note
//!
//! `ElicitProxy for bevy::X` cannot be written in `elicit_bevy` (orphan rule).
//! The `#[reflect_trait]` attribute uses `type_map(...)` to declare substitutions
//! using the `From` impls that `elicit_newtype!` already provides.
//!
//! # Marker traits
//!
//! `bevy::ecs::component::Component`, `bevy::ecs::system::Resource`,
//! `bevy::asset::Asset`, [`bevy::ecs::bundle::Bundle`],
//! `bevy::ecs::event::Event`, [`bevy::state::state::States`], and
//! [`bevy::app::Plugin`] are pure marker traits — they expose no MCP-safe
//! static methods.  The `#[reflect_trait]` macro requires at least one method
//! body, so these factories are written by hand.  Their `prime::<T>()` methods
//! register the type in a static map; `instantiate` returns an empty tool list.
//! The factory is still submitted to inventory so the dynamic registry knows
//! which Bevy role `T` fills.
//!
//! # Usage
//!
//! At server startup, for each user type `T` implementing the Bevy trait:
//!
//! ```rust,ignore
//! // For types implementing Default:
//! prime_std__default__default::<MyState>();
//!
//! // For each Bevy marker role:
//! prime_bevy_component::<MyComponent>();
//! prime_bevy_resource::<MyResource>();
//! prime_bevy_asset::<MyAsset>();
//! prime_bevy_event::<MyEvent>();
//! prime_bevy_states::<MyState>();
//!
//! registry.register_type::<MyState>("my_state").await;
//! ```

use elicitation_derive::reflect_trait;
use std::any::TypeId;
use std::collections::HashSet;
use std::sync::{LazyLock, RwLock};

// ── std::default::Default ─────────────────────────────────────────────────────

/// Expose [`Default`] as agent-callable MCP tools.
///
/// Agents can call `{prefix}__default()` to construct a default instance
/// of any type registered with this factory.
#[reflect_trait(std::default::Default)]
pub trait DefaultTools {
    /// Construct a default instance of this type.
    fn default() -> Self;
}

// ── Manual marker factory helpers ─────────────────────────────────────────────

/// Generate a manual `AnyToolFactory` for a Bevy marker trait.
///
/// Marker traits have no MCP-safe methods.  The factory exists so that the
/// dynamic registry can record which Bevy role a type fills and emit that
/// information to agents.
macro_rules! bevy_marker_factory {
    (
        factory = $factory:ident,
        prime_fn = $prime_fn:ident,
        trait_path = $trait_path:path,
        trait_name_str = $trait_name_str:expr,
        description = $description:expr $(,)?
    ) => {
        #[doc = $description]
        pub struct $factory;

        impl $factory {
            /// Prime this factory for concrete type `T`.
            ///
            /// Call this at server startup alongside
            /// `register_type::<T>(prefix)`.
            pub fn prime<T>()
            where
                T: $trait_path
                    + ::serde::Serialize
                    + ::serde::de::DeserializeOwned
                    + ::schemars::JsonSchema
                    + ::elicitation::Elicitation
                    + Send
                    + Sync
                    + 'static,
            {
                let type_id = TypeId::of::<T>();
                let mut set = $factory::vtable_set()
                    .write()
                    .expect("factory set lock poisoned");
                set.insert(type_id);
            }

            fn vtable_set() -> &'static RwLock<HashSet<TypeId>> {
                static SET: LazyLock<RwLock<HashSet<TypeId>>> =
                    LazyLock::new(|| RwLock::new(HashSet::new()));
                &SET
            }
        }

        impl ::elicitation::AnyToolFactory for $factory {
            fn trait_name(&self) -> &'static str {
                $trait_name_str
            }

            fn factory_description(&self) -> &'static str {
                $description
            }

            fn method_names(&self) -> &'static [&'static str] {
                &[]
            }

            fn instantiate(
                &self,
                _slot: &dyn ::elicitation::dynamic::slot::AnyToolSlot,
            ) -> ::std::result::Result<
                ::std::vec::Vec<::elicitation::DynamicToolDescriptor>,
                ::rmcp::ErrorData,
            > {
                Ok(vec![])
            }
        }

        ::inventory::submit!(::elicitation::ToolFactoryRegistration {
            trait_name: $trait_name_str,
            factory: &$factory,
        });

        #[doc = concat!("Prime the [`", stringify!($factory), "`] factory for type `T`.")]
        ///
        /// Call this at server startup alongside `register_type::<T>(prefix)`.
        pub fn $prime_fn<T>()
        where
            T: $trait_path
                + ::serde::Serialize
                + ::serde::de::DeserializeOwned
                + ::schemars::JsonSchema
                + ::elicitation::Elicitation
                + Send
                + Sync
                + 'static,
        {
            $factory::prime::<T>();
        }
    };
}

// ── bevy::ecs::component::Component ──────────────────────────────────────────

bevy_marker_factory! {
    factory = ComponentToolsFactory,
    prime_fn = prime_bevy_component,
    trait_path = bevy::ecs::component::Component,
    trait_name_str = "bevy::ecs::component::Component",
    description = "Marker factory — confirms T: Component is registered.",
}

// ── bevy::ecs::resource::Resource ──────────────────────────────────────────

bevy_marker_factory! {
    factory = ResourceToolsFactory,
    prime_fn = prime_bevy_resource,
    trait_path = bevy::ecs::resource::Resource,
    trait_name_str = "bevy::ecs::resource::Resource",
    description = "Marker factory — confirms T: Resource is registered.",
}

// ── bevy::asset::Asset ───────────────────────────────────────────────────────

bevy_marker_factory! {
    factory = AssetToolsFactory,
    prime_fn = prime_bevy_asset,
    trait_path = bevy::asset::Asset,
    trait_name_str = "bevy::asset::Asset",
    description = "Marker factory — confirms T: Asset is registered.",
}

// ── bevy::ecs::bundle::Bundle ─────────────────────────────────────────────────

bevy_marker_factory! {
    factory = BundleToolsFactory,
    prime_fn = prime_bevy_bundle,
    trait_path = bevy::ecs::bundle::Bundle,
    trait_name_str = "bevy::ecs::bundle::Bundle",
    description = "Marker factory — confirms T: Bundle is registered.",
}

// ── bevy::ecs::event::Event ───────────────────────────────────────────────────

bevy_marker_factory! {
    factory = EventToolsFactory,
    prime_fn = prime_bevy_event,
    trait_path = bevy::ecs::event::Event,
    trait_name_str = "bevy::ecs::event::Event",
    description = "Marker factory — confirms T: Event is registered.",
}

// ── bevy::state::state::States ────────────────────────────────────────────────

bevy_marker_factory! {
    factory = StatesToolsFactory,
    prime_fn = prime_bevy_states,
    trait_path = bevy::state::state::States,
    trait_name_str = "bevy::state::state::States",
    description = "Marker factory — confirms T: States is registered.",
}

// ── bevy::app::Plugin ────────────────────────────────────────────────────────

bevy_marker_factory! {
    factory = PluginToolsFactory,
    prime_fn = prime_bevy_plugin,
    trait_path = bevy::app::Plugin,
    trait_name_str = "bevy::app::Plugin",
    description = "Marker factory — confirms T: Plugin is registered.",
}

// Factories are already public via `pub struct` in the macro expansions above.
