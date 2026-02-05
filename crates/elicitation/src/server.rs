//! Server wrapper for style-aware elicitation.
//!
//! This is the server-side equivalent of `ElicitClient`. It wraps a `Peer<RoleServer>`
//! and provides the same style management API, but uses server-to-client communication
//! via `peer.create_message()`.

use rmcp::service::{Peer, RoleServer};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::{ElicitResult, Elicitation, ElicitationStyle};

/// Server wrapper that carries style context.
///
/// Wraps an RMCP server peer and maintains style selections for different types.
/// This is the server-side equivalent of `ElicitClient` - it has the same API
/// but uses `Peer<RoleServer>` for server-to-client communication.
///
/// # Example
///
/// ```rust,ignore
/// use elicitation::{ElicitServer, ElicitationStyle, Elicitation};
///
/// // In a tool handler:
/// #[tool]
/// async fn my_tool(peer: Peer<RoleServer>) -> Result<Config, Error> {
///     let server = ElicitServer::new(peer);
///     let config = Config::elicit(&server).await?;
///     Ok(config)
/// }
/// ```
pub struct ElicitServer {
    peer: Peer<RoleServer>,
    style_context: StyleContext,
}

impl ElicitServer {
    /// Create a new server wrapper from an RMCP peer.
    #[tracing::instrument(skip(peer))]
    pub fn new(peer: Peer<RoleServer>) -> Self {
        tracing::debug!("Creating new ElicitServer");
        Self {
            peer,
            style_context: StyleContext::default(),
        }
    }

    /// Get the underlying RMCP peer for making requests to the client.
    #[tracing::instrument(skip(self), level = "trace")]
    pub fn peer(&self) -> &Peer<RoleServer> {
        &self.peer
    }

    /// Create a new server with a custom style for a specific type.
    ///
    /// Accepts any style type that implements [`ElicitationStyle`], allowing
    /// users to define custom styles for built-in types.
    ///
    /// Returns a new `ElicitServer` with the style added to the context.
    /// The original server is unchanged.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Use default style
    /// let server = server.with_style::<Config, _>(ConfigStyle::default());
    ///
    /// // Use custom style for i32
    /// let server = server.with_style::<i32, _>(MyI32Style::Verbose);
    /// ```
    #[tracing::instrument(skip(self, style))]
    pub fn with_style<T: Elicitation + 'static, S: ElicitationStyle>(&self, style: S) -> Self {
        let type_name = std::any::type_name::<T>();
        tracing::debug!(type_name, "Setting custom style");
        let mut ctx = self.style_context.clone();
        ctx.set_style::<T, S>(style);
        Self {
            peer: self.peer.clone(),
            style_context: ctx,
        }
    }

    /// Get the current style for a type, or use default if not set.
    ///
    /// This method checks if a custom style was set via `with_style()`.
    /// If a style was set, it returns that style. Otherwise, it returns
    /// the default style for the type.
    #[tracing::instrument(skip(self))]
    pub fn style_or_default<T: Elicitation + 'static>(&self) -> T::Style
    where
        T::Style: ElicitationStyle,
    {
        let type_name = std::any::type_name::<T>();
        let has_custom = self.style_context.get_style::<T, T::Style>().is_some();
        tracing::debug!(type_name, has_custom, "Getting style or default");
        self.style_context
            .get_style::<T, T::Style>()
            .unwrap_or_default()
    }

    /// Get the current style for a type, eliciting if not set.
    ///
    /// This method checks if a custom style was set via `with_style()`.
    /// If a style was set, it returns that style. Otherwise, it elicits
    /// the style from the client.
    ///
    /// This enables "auto-selection": styles are only elicited when needed.
    ///
    /// # TODO
    ///
    /// Currently stubbed - requires making Elicitation trait generic over
    /// ElicitClient/ElicitServer.
    #[tracing::instrument(skip(self))]
    pub async fn style_or_elicit<T: Elicitation + 'static>(&self) -> ElicitResult<T::Style>
    where
        T::Style: ElicitationStyle,
    {
        if let Some(style) = self.style_context.get_style::<T, T::Style>() {
            tracing::debug!(
                type_name = std::any::type_name::<T>(),
                "Using pre-set style"
            );
            Ok(style)
        } else {
            tracing::debug!(type_name = std::any::type_name::<T>(), "Eliciting style (TODO)");
            // TODO: T::Style::elicit(self).await
            Ok(T::Style::default())
        }
    }

    /// Get the current style for a type, or use the default.
    ///
    /// If a custom style has been set via `with_style()`, returns that.
    /// Otherwise, returns `T::Style::default()` as fallback.
    #[tracing::instrument(skip(self))]
    pub async fn current_style<T: Elicitation + 'static>(&self) -> ElicitResult<T::Style>
    where
        T::Style: Clone + Send + Sync + 'static,
    {
        // Try to get custom style first
        if let Some(style) = self.style_context.get_style::<T, T::Style>() {
            tracing::debug!(type_name = std::any::type_name::<T>(), "Using custom style");
            return Ok(style);
        }

        // Fall back to default
        tracing::debug!(
            type_name = std::any::type_name::<T>(),
            "Using default style"
        );
        Ok(T::Style::default())
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
    /// Set a custom style for a specific type.
    ///
    /// Accepts any style type S that implements ElicitationStyle.
    #[tracing::instrument(skip(self, style), level = "debug", fields(type_id = ?TypeId::of::<T>()))]
    fn set_style<T: 'static, S: ElicitationStyle>(&mut self, style: S) {
        let type_id = TypeId::of::<T>();
        let mut styles = self.styles.write().expect("Lock poisoned");
        styles.insert(type_id, Box::new(style));
    }

    /// Get the custom style for a specific type, if one was set.
    ///
    /// Returns None if no custom style was provided, allowing
    /// fallback to T::Style::default().
    #[tracing::instrument(skip(self), level = "debug", fields(type_id = ?TypeId::of::<T>()))]
    fn get_style<T: 'static, S: ElicitationStyle>(&self) -> Option<S> {
        let type_id = TypeId::of::<T>();
        let styles = self.styles.read().expect("Lock poisoned");
        styles
            .get(&type_id)
            .and_then(|boxed| boxed.downcast_ref::<S>())
            .cloned()
    }
}
