//! [`bevy::ecs::entity::Entity`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use std::sync::Arc;

elicit_newtype!(bevy::ecs::entity::Entity, as Entity);
elicit_newtype_traits!(Entity, bevy::ecs::entity::Entity, [cmp]);

impl serde::Serialize for Entity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Bevy serializes Entity as its u64 bit representation.
        (*self.0).serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Entity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        bevy::ecs::entity::Entity::deserialize(deserializer).map(|e| Entity(Arc::new(e)))
    }
}

/// Unwrap the Arc back to an owned `bevy::ecs::entity::Entity`.
///
/// Entity is `Copy`, so dereferencing the Arc is sufficient.
impl From<Entity> for bevy::ecs::entity::Entity {
    fn from(v: Entity) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Entity {
    /// Returns the index of this entity within the allocator.
    #[tracing::instrument(skip(self))]
    pub fn index_u32(&self) -> u32 {
        self.0.index_u32()
    }

    /// Returns the generation counter of this entity as a raw `u32`.
    ///
    /// The generation distinguishes recycled entities that share the same index.
    #[tracing::instrument(skip(self))]
    pub fn generation(&self) -> u32 {
        self.0.generation().to_bits()
    }

    /// Returns the full 64-bit representation: `generation << 32 | index`.
    #[tracing::instrument(skip(self))]
    pub fn bits(&self) -> u64 {
        self.0.to_bits()
    }

    /// Returns `true` if this entity is the [`PLACEHOLDER`](bevy::ecs::entity::Entity::PLACEHOLDER).
    ///
    /// Placeholder entities are never valid ECS entities and are used as
    /// sentinels in data structures.
    #[tracing::instrument(skip(self))]
    pub fn is_placeholder(&self) -> bool {
        *self.0 == bevy::ecs::entity::Entity::PLACEHOLDER
    }

    /// Reconstruct an [`Entity`] from its `to_bits()` representation.
    ///
    /// The `&self` receiver is ignored; this is a constructor exposed as an
    /// instance method so that agents can call it on any existing `Entity`.
    #[tracing::instrument(skip(self))]
    pub fn from_bits_constructor(&self, bits: u64) -> Entity {
        Entity(Arc::new(bevy::ecs::entity::Entity::from_bits(bits)))
    }

    /// Construct an [`Entity`] from a raw index with generation zero.
    ///
    /// Returns `None` if `index` equals `u32::MAX`, which is reserved.
    ///
    /// The `&self` receiver is ignored; this is a factory constructor.
    #[tracing::instrument(skip(self))]
    pub fn from_raw_constructor(&self, index: u32) -> Option<Entity> {
        bevy::ecs::entity::Entity::from_raw_u32(index).map(|e| Entity(Arc::new(e)))
    }
}

// ── ElicitComplete + ToCodeLiteral ───────────────────────────────────────────

mod emit_impls {
    use super::Entity;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Entity {
        fn to_code_literal(&self) -> TokenStream {
            let bits = self.0.to_bits();
            quote::quote! {
                ::elicit_bevy::Entity::from(
                    ::bevy::ecs::entity::Entity::from_bits(#bits)
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for Entity {}
