//! Bevy scene wrappers.
//!
//! ## Note on `Scene` and `DynamicScene`
//!
//! `bevy::scene::Scene` implements [`bevy::asset::Asset`]
//! and is loaded / accessed via `Handle<Scene>`. These
//! handles are managed by the `AssetServer` and cannot be directly instantiated
//! as MCP values.
//!
//! For scene spawning, attach `bevy::scene::SceneRoot` as a component on an
//! entity. The
//! [`WorldInstanceReady`] event fires on the parent entity once the scene has
//! fully spawned.
//!
//! ## WorldInstanceReady
//!
//! [`WorldInstanceReady`] is an entity event that carries the spawned entity
//! and its instance ID. It is wrapped here so agents can inspect the payload
//! when it arrives over MCP.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use std::sync::Arc;

// ── WorldInstanceReady ────────────────────────────────────────────────────────

elicit_newtype!(bevy::world_serialization::WorldInstanceReady, as WorldInstanceReady);
elicit_newtype_traits!(
    WorldInstanceReady,
    bevy::world_serialization::WorldInstanceReady,
    [eq]
);

impl serde::Serialize for WorldInstanceReady {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("entity_bits", &self.0.entity.to_bits())?;
        // InstanceId has no Display impl; use Debug representation.
        map.serialize_entry("instance_id", &format!("{:?}", self.0.instance_id))?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for WorldInstanceReady {
    fn deserialize<D: serde::Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
        // WorldInstanceReady is an ECS event produced by Bevy's scene spawner.
        // It cannot be constructed externally (InstanceId::new is private).
        Err(serde::de::Error::custom(
            "WorldInstanceReady cannot be deserialized; observe it via the ECS event system",
        ))
    }
}

impl From<WorldInstanceReady> for bevy::world_serialization::WorldInstanceReady {
    fn from(v: WorldInstanceReady) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| *arc)
    }
}

#[reflect_methods]
impl WorldInstanceReady {
    /// Returns the bits of the entity whose scene instance is ready.
    ///
    /// Pass this value to `Entity::from_bits_constructor` to reconstruct the entity.
    #[tracing::instrument(skip(self))]
    pub fn entity_bits(&self) -> u64 {
        self.0.entity.to_bits()
    }

    /// Returns the instance ID as its debug string.
    ///
    /// Useful for correlating the ready event with the spawned scene instance.
    #[tracing::instrument(skip(self))]
    pub fn instance_id_string(&self) -> String {
        format!("{:?}", self.0.instance_id)
    }
}

// ── ElicitComplete + ToCodeLiteral ───────────────────────────────────────────

mod emit_impls {
    use super::WorldInstanceReady;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for WorldInstanceReady {
        fn to_code_literal(&self) -> TokenStream {
            let bits = self.0.entity.to_bits();
            quote::quote! {
                // WorldInstanceReady is an ECS event, not constructed directly.
                // This literal reconstructs the entity reference only.
                compile_error!(
                    "WorldInstanceReady cannot be constructed as a code literal; \
                     observe it via the ECS event system instead."
                );
                let _ = #bits;
            }
        }
    }
}

impl elicitation::ElicitComplete for WorldInstanceReady {}

// ── Name ──────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ecs::name::Name, as Name, forward_serde);
elicit_newtype_traits!(Name, bevy::ecs::name::Name, []);

impl From<Name> for bevy::ecs::name::Name {
    fn from(v: Name) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl Name {
    /// Returns the entity name as a `&str`.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

mod emit_name {
    use super::Name;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Name {
        fn to_code_literal(&self) -> TokenStream {
            let s = self.0.as_str();
            quote::quote! { ::bevy::ecs::name::Name::new(#s) }
        }
    }
}

impl elicitation::ElicitComplete for Name {}
