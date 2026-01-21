//! Client wrapper for style-aware elicitation.

use rmcp::service::{Peer, RoleClient};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::{ElicitResult, Elicitation, ElicitationStyle};

/// Client wrapper that carries style context.
///
/// Wraps an RMCP peer and maintains style selections for different types.
/// Each type can have its own style, allowing nested types to use different
/// styles independently.
///
/// Users can provide custom style types for any type by implementing
/// [`ElicitationStyle`] and calling [`with_style`](Self::with_style).
///
/// # Example
///
/// ```rust,ignore
/// use elicitation::{ElicitClient, ElicitationStyle, Elicitation};
///
/// // Define custom style for i32
/// #[derive(Clone, Default)]
/// enum MyI32Style {
///     #[default]
///     Terse,
///     Verbose
/// }
///
/// impl ElicitationStyle for MyI32Style {}
///
/// // Use it
/// let client = ElicitClient::new(&peer);
/// let styled = client.with_style::<i32, _>(MyI32Style::Verbose);
/// let age = i32::elicit(&styled).await?;
/// ```
pub struct ElicitClient<'a> {
    peer: &'a Peer<RoleClient>,
    style_context: StyleContext,
}

impl<'a> ElicitClient<'a> {
    /// Create a new client wrapper from an RMCP peer.
    #[tracing::instrument(skip(peer))]
    pub fn new(peer: &'a Peer<RoleClient>) -> Self {
        tracing::debug!("Creating new ElicitClient");
        Self {
            peer,
            style_context: StyleContext::default(),
        }
    }

    /// Get the underlying RMCP peer for making tool calls.
    #[tracing::instrument(skip(self), level = "trace")]
    pub fn peer(&self) -> &Peer<RoleClient> {
        self.peer
    }

    /// Create a new client with a custom style for a specific type.
    ///
    /// Accepts any style type that implements [`ElicitationStyle`], allowing
    /// users to define custom styles for built-in types.
    ///
    /// Returns a new `ElicitClient` with the style added to the context.
    /// The original client is unchanged.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Use default style
    /// let client = client.with_style::<Config, _>(ConfigStyle::default());
    ///
    /// // Use custom style for i32
    /// let client = client.with_style::<i32, _>(MyI32Style::Verbose);
    /// ```
    #[tracing::instrument(skip(self, style))]
    pub fn with_style<T: Elicitation + 'static, S: ElicitationStyle>(&self, style: S) -> Self {
        let type_name = std::any::type_name::<T>();
        tracing::debug!(type_name, "Setting custom style");
        let mut ctx = self.style_context.clone();
        ctx.set_style::<T, S>(style);
        Self {
            peer: self.peer,
            style_context: ctx,
        }
    }

    /// Get the current style for a type, or use default if not set.
    ///
    /// This method checks if a custom style was set via `with_style()`.
    /// If a style was set, it returns that style. Otherwise, it returns
    /// the default style for the type.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Get style - uses custom if set, default otherwise
    /// let style = client.style_or_default::<Config>();
    /// ```
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
    /// the style from the user.
    ///
    /// This enables "auto-selection": styles are only elicited when needed.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Get style - uses custom if set, otherwise asks user
    /// let style = client.style_or_elicit::<Config>().await?;
    /// ```
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
            tracing::debug!(type_name = std::any::type_name::<T>(), "Eliciting style");
            T::Style::elicit(self).await
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
            .and_then(|any| any.downcast_ref::<S>())
            .cloned()
    }
}
