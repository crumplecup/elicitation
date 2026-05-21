//! [`AnyToolSlot`] — type-erased bounded container for a registered concrete type.
//!
//! A slot is created once per [`DynamicToolRegistry::register_type`](crate::DynamicToolRegistry::register_type) call.
//! It carries the type's prefix string and, in the concrete [`TypedSlot<T>`]
//! implementation, a monomorphized vtable that closures can capture.

use std::{any::Any, marker::PhantomData};

use schemars::JsonSchema;
use serde::{Serialize, de::DeserializeOwned};

use crate::traits::Elicitation;

/// Object-safe wrapper representing one registered concrete type.
///
/// Created via [`DynamicToolRegistry::register_type`](crate::DynamicToolRegistry::register_type).  Factories receive a
/// `&dyn AnyToolSlot` and call [`TypedSlot::downcast_ref`] to recover the
/// typed slot for their specific vtable.
pub trait AnyToolSlot: Any + Send + Sync + 'static {
    /// User-provided prefix (e.g. `"user"`, `"post"`).
    fn prefix(&self) -> &str;

    /// Rust type name, for diagnostics.
    fn type_name(&self) -> &'static str;

    /// JSON schema for the registered type, generated from `JsonSchema`.
    fn schema(&self) -> schemars::Schema;

    /// `std::any::TypeId` of the concrete `T`, for safe downcasting.
    fn slot_type_id(&self) -> std::any::TypeId;

    /// Upcast to `&dyn Any` for safe downcasting via `Any::downcast_ref`.
    fn as_any(&self) -> &dyn Any;
}

/// Concrete slot for a known `T`.
///
/// Only constructable when the bounds `T: Serialize + DeserializeOwned +
/// JsonSchema + Elicitation + Send + Sync + 'static` are satisfied.  These
/// bounds enforce that every registered type can be safely turned into MCP
/// tool parameters and return values.
pub struct TypedSlot<T>
where
    T: Serialize + DeserializeOwned + JsonSchema + Elicitation + Send + Sync + 'static,
{
    prefix: String,
    _phantom: PhantomData<fn() -> T>,
}

impl<T> TypedSlot<T>
where
    T: Serialize + DeserializeOwned + JsonSchema + Elicitation + Send + Sync + 'static,
{
    /// Create a new slot for `T` with the given prefix.
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            _phantom: PhantomData,
        }
    }

    /// Attempt to downcast a `&dyn AnyToolSlot` to `&TypedSlot<T>`.
    ///
    /// Returns `None` if the slot holds a different concrete type.
    pub fn downcast_ref(slot: &dyn AnyToolSlot) -> Option<&TypedSlot<T>> {
        slot.as_any().downcast_ref::<TypedSlot<T>>()
    }
}

impl<T> AnyToolSlot for TypedSlot<T>
where
    T: Serialize + DeserializeOwned + JsonSchema + Elicitation + Send + Sync + 'static,
{
    fn prefix(&self) -> &str {
        &self.prefix
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn schema(&self) -> schemars::Schema {
        schemars::schema_for!(T)
    }

    fn slot_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<T>()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
