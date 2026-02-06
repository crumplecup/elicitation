//! Communication abstraction for elicitation.
//!
//! This module provides the `ElicitCommunicator` trait which abstracts over
//! client-side and server-side elicitation contexts. Both `ElicitClient` and
//! `ElicitServer` implement this trait, allowing the `Elicitation` trait to
//! work with either context seamlessly.

use crate::{ElicitResult, Elicitation, ElicitationStyle};
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Abstraction for elicitation communication.
///
/// This trait provides a unified interface for both client-side and server-side
/// elicitation. Implementations handle the details of sending prompts and
/// receiving responses in their respective contexts.
///
/// # Implementors
///
/// - `ElicitClient` - Client-side communication via `Peer<RoleClient>`
/// - `ElicitServer` - Server-side communication via `Peer<RoleServer>`
pub trait ElicitCommunicator: Clone + Send + Sync {
    /// Send a prompt and receive a text response.
    ///
    /// The implementation handles the details of formatting the prompt,
    /// sending it via MCP, and extracting the text response.
    ///
    /// # Arguments
    ///
    /// * `prompt` - The prompt text to send
    ///
    /// # Returns
    ///
    /// Returns the response text on success, or an error if communication fails.
    fn send_prompt(
        &self,
        prompt: &str,
    ) -> impl std::future::Future<Output = ElicitResult<String>> + Send;

    /// Call an MCP tool directly with given parameters.
    ///
    /// This is a low-level method used by validation types that need specific
    /// tool interactions beyond generic text prompts.
    ///
    /// # Arguments
    ///
    /// * `params` - The tool call parameters
    ///
    /// # Returns
    ///
    /// Returns the tool call result or an error.
    fn call_tool(
        &self,
        params: rmcp::model::CallToolRequestParams,
    ) -> impl std::future::Future<
        Output = Result<rmcp::model::CallToolResult, rmcp::service::ServiceError>,
    > + Send;

    /// Get the style context for type-specific styles.
    ///
    /// The style context maintains custom style selections for different types,
    /// allowing each type to have its own style independently.
    fn style_context(&self) -> &StyleContext;

    /// Create a new communicator with a style added for a specific type.
    ///
    /// Returns a new communicator with the style in the context. The original
    /// communicator is unchanged.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to set the style for
    /// * `S` - The style type (must implement `ElicitationStyle`)
    fn with_style<T: 'static, S: ElicitationStyle>(&self, style: S) -> Self;

    /// Get the current style for a type, or use default if not set.
    ///
    /// This method checks if a custom style was set via `with_style()`.
    /// If found, returns that style. Otherwise, returns `T::Style::default()`.
    fn style_or_default<T: Elicitation + 'static>(&self) -> T::Style
    where
        T::Style: ElicitationStyle,
    {
        self.style_context()
            .get_style::<T, T::Style>()
            .unwrap_or_default()
    }

    /// Get the current style for a type, eliciting if not set.
    ///
    /// This method checks if a custom style was set via `with_style()`.
    /// If found, returns that style. Otherwise, elicits the style from
    /// the user/client.
    ///
    /// This enables "auto-selection": styles are only elicited when needed.
    fn style_or_elicit<T: Elicitation + 'static>(
        &self,
    ) -> impl std::future::Future<Output = ElicitResult<T::Style>> + Send
    where
        T::Style: ElicitationStyle,
    {
        async move {
            if let Some(style) = self.style_context().get_style::<T, T::Style>() {
                Ok(style)
            } else {
                T::Style::elicit(self).await
            }
        }
    }
}

/// Storage for type-specific styles.
///
/// Uses `TypeId` to store different style enums for different types.
/// This allows each type to have its own style selection without interference.
/// Internally uses `Arc<RwLock<_>>` for efficient cloning.
#[derive(Clone, Default)]
pub struct StyleContext {
    styles: Arc<RwLock<HashMap<TypeId, Box<dyn std::any::Any + Send + Sync>>>>,
}

impl StyleContext {
    /// Set a custom style for a specific type.
    ///
    /// Accepts any style type S that implements ElicitationStyle.
    #[tracing::instrument(skip(self, style), level = "debug", fields(type_id = ?TypeId::of::<T>()))]
    pub fn set_style<T: 'static, S: ElicitationStyle>(&mut self, style: S) {
        let type_id = TypeId::of::<T>();
        let mut styles = self.styles.write().expect("Lock poisoned");
        styles.insert(type_id, Box::new(style));
    }

    /// Get the custom style for a specific type, if one was set.
    ///
    /// Returns None if no custom style was provided, allowing
    /// fallback to T::Style::default().
    #[tracing::instrument(skip(self), level = "debug", fields(type_id = ?TypeId::of::<T>()))]
    pub fn get_style<T: 'static, S: ElicitationStyle>(&self) -> Option<S> {
        let type_id = TypeId::of::<T>();
        let styles = self.styles.read().expect("Lock poisoned");
        styles
            .get(&type_id)
            .and_then(|boxed| boxed.downcast_ref::<S>())
            .cloned()
    }
}
