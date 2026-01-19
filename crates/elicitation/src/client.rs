//! Client wrapper for style-aware elicitation.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use rmcp::service::{Peer, RoleClient};

use crate::{ElicitResult, Elicitation};

/// Client wrapper that carries style context.
///
/// Wraps an RMCP peer and maintains style selections for different types.
/// Each type can have its own style, allowing nested types to use different
/// styles independently.
///
/// # Example
///
/// ```rust,ignore
/// use elicitation::ElicitClient;
/// 
/// let client = ElicitClient::new(&peer);
/// let styled = client.with_style(ConfigStyle::Verbose);
/// let config = Config::elicit(&styled).await?;
/// ```
pub struct ElicitClient<'a> {
    peer: &'a Peer<RoleClient>,
    style_context: StyleContext,
}

impl<'a> ElicitClient<'a> {
    /// Create a new client wrapper from an RMCP peer.
    pub fn new(peer: &'a Peer<RoleClient>) -> Self {
        Self {
            peer,
            style_context: StyleContext::default(),
        }
    }

    /// Get the underlying RMCP peer for making tool calls.
    pub fn peer(&self) -> &Peer<RoleClient> {
        self.peer
    }

    /// Create a new client with a style set for a specific type.
    ///
    /// Returns a new `ElicitClient` with the style added to the context.
    /// The original client is unchanged.
    pub fn with_style<T: Elicitation + 'static>(&self, style: T::Style) -> Self
    where
        T::Style: Clone + Send + Sync + 'static,
    {
        let mut ctx = self.style_context.clone();
        ctx.set_style::<T>(style);
        Self {
            peer: self.peer,
            style_context: ctx,
        }
    }

    /// Get the current style for a type, or elicit it from the user.
    ///
    /// If a style has been set via `with_style()`, returns that.
    /// Otherwise, elicits the style from the user (auto-selection).
    #[tracing::instrument(skip(self))]
    pub async fn current_style<T: Elicitation + 'static>(&self) -> ElicitResult<T::Style>
    where
        T::Style: Clone + Send + Sync + 'static,
    {
        if let Some(style) = self.style_context.get_style::<T>() {
            tracing::debug!(type_name = std::any::type_name::<T>(), "Using pre-set style");
            return Ok(style);
        }

        tracing::debug!(type_name = std::any::type_name::<T>(), "Auto-selecting style");
        T::Style::elicit(self).await
    }
}

/// Storage for type-specific styles.
///
/// Uses `TypeId` to store different style enums for different types.
/// This allows each type to have its own style selection without interference.
/// Internally uses `Arc<RwLock<_>>` for efficient cloning.
#[derive(Clone, Default)]
struct StyleContext {
    styles: Arc<RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync>>>>,
}

impl StyleContext {
    /// Set the style for a specific type.
    fn set_style<T: 'static>(&mut self, style: T::Style)
    where
        T: Elicitation,
        T::Style: Clone + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<T>();
        let mut styles = self.styles.write().expect("Lock poisoned");
        styles.insert(type_id, Box::new(style));
    }

    /// Get the style for a specific type, if set.
    fn get_style<T: 'static>(&self) -> Option<T::Style>
    where
        T: Elicitation,
        T::Style: Clone + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<T>();
        let styles = self.styles.read().expect("Lock poisoned");
        styles
            .get(&type_id)
            .and_then(|any| any.downcast_ref::<T::Style>())
            .cloned()
    }
}
