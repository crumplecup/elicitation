//! Communication abstraction for elicitation.
//!
//! This module provides the `ElicitCommunicator` trait which abstracts over
//! client-side and server-side elicitation contexts. Both `ElicitClient` and
//! `ElicitServer` implement this trait, allowing the `Elicitation` trait to
//! work with either context seamlessly.

use crate::{ElicitError, ElicitErrorKind, ElicitResult, Elicitation, StyleMarker, TypeMetadata};
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
pub trait ElicitCommunicator: Clone + Send + Sync + 'static {
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
    /// * `S` - The style type (must implement [`StyleMarker`] and
    ///   [`style::ElicitationStyle`](crate::style::ElicitationStyle))
    fn with_style<T: 'static, S: StyleMarker + crate::style::ElicitationStyle + 'static>(
        &self,
        style: S,
    ) -> Self;

    /// Get the current style for a type, or use default if not set.
    ///
    /// This method checks if a custom style was set via `with_style()`.
    /// If found, returns that style. Otherwise, returns `T::Style::default()`.
    ///
    /// # Errors
    ///
    /// Returns an error if the style context lock is poisoned.
    fn style_or_default<T: Elicitation + 'static>(&self) -> ElicitResult<T::Style>
    where
        T::Style: StyleMarker,
    {
        Ok(self
            .style_context()
            .get_style::<T, T::Style>()?
            .unwrap_or_default())
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
        T::Style: StyleMarker,
    {
        async move {
            if let Some(style) = self.style_context().get_style::<T, T::Style>()? {
                Ok(style)
            } else {
                T::Style::elicit(self).await
            }
        }
    }

    /// Get the elicitation context for introspection.
    ///
    /// The elicitation context tracks the current chain of nested elicitations,
    /// enabling observability without storing full history.
    fn elicitation_context(&self) -> &ElicitationContext;

    /// Get the metadata for the currently elicited type.
    ///
    /// Returns `None` if no elicitation is in progress (e.g., at the top level
    /// before any elicitation starts).
    ///
    /// # Errors
    ///
    /// Returns an error if the elicitation context lock is poisoned.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // In a traced function
    /// if let Some(meta) = communicator.current_type()? {
    ///     tracing::info!(
    ///         type_name = %meta.type_name,
    ///         pattern = ?meta.pattern(),
    ///         "Eliciting type"
    ///     );
    /// }
    /// ```
    fn current_type(&self) -> ElicitResult<Option<TypeMetadata>> {
        self.elicitation_context().current()
    }

    /// Get the current elicitation depth.
    ///
    /// Returns:
    /// - `0` if at the top level (before any elicitation)
    /// - `1` if eliciting a top-level type
    /// - `2` if eliciting a field of a struct, etc.
    ///
    /// # Errors
    ///
    /// Returns an error if the elicitation context lock is poisoned.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let depth = communicator.current_depth()?;
    /// tracing::debug!(depth, "Elicitation depth");
    /// ```
    fn current_depth(&self) -> ElicitResult<usize> {
        self.elicitation_context().depth()
    }

    /// Get a snapshot of the full elicitation stack.
    ///
    /// Returns the complete chain from root to current type.
    /// Useful for detailed logging or debugging.
    ///
    /// # Errors
    ///
    /// Returns an error if the elicitation context lock is poisoned.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// for meta in communicator.elicitation_stack()? {
    ///     println!("  {}", meta.type_name);
    ///     }
    /// ```
    fn elicitation_stack(&self) -> ElicitResult<Vec<TypeMetadata>> {
        self.elicitation_context().stack()
    }
}

/// Internal trait for type-erased style storage.
///
/// Combines `Any` (for concrete type downcasting) with
/// [`style::ElicitationStyle`](crate::style::ElicitationStyle)
/// (for dynamic prompt generation). This allows stored styles to be both
/// retrieved as concrete types via `get_style` and used dynamically via
/// `prompt_for_type` without knowing the concrete type.
trait StyleEntry: Send + Sync {
    /// Downcast support for concrete type recovery.
    fn as_any(&self) -> &dyn std::any::Any;

    /// Dynamic access to the style's prompt generation.
    fn as_style(&self) -> &dyn crate::style::ElicitationStyle;
}

impl<T: crate::style::ElicitationStyle + 'static> StyleEntry for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_style(&self) -> &dyn crate::style::ElicitationStyle {
        self
    }
}

/// Storage for type-specific styles.
///
/// Uses `TypeId` to store different style enums for different types.
/// This allows each type to have its own style selection without interference.
/// Internally uses `Arc<RwLock<_>>` for efficient cloning.
#[derive(Clone, Default)]
pub struct StyleContext {
    styles: Arc<RwLock<HashMap<TypeId, Box<dyn StyleEntry>>>>,
    /// One-shot prompt override set by struct field elicitation before calling
    /// the field type's `elicit`. Consumed by the first [`prompt_for_type`] call
    /// or by [`take_field_prompt`], so nested elicitations see their own prompts.
    ///
    /// [`prompt_for_type`]: StyleContext::prompt_for_type
    /// [`take_field_prompt`]: StyleContext::take_field_prompt
    field_prompt: Arc<RwLock<Option<String>>>,
}

impl StyleContext {
    /// Set a custom style for a specific type.
    ///
    /// Accepts any style type S that implements [`StyleMarker`] and
    /// [`style::ElicitationStyle`](crate::style::ElicitationStyle).
    ///
    /// # Errors
    ///
    /// Returns an error if the lock is poisoned.
    #[tracing::instrument(skip(self, style), level = "debug", fields(type_id = ?TypeId::of::<T>()))]
    pub fn set_style<T: 'static, S: StyleMarker + crate::style::ElicitationStyle + 'static>(
        &mut self,
        style: S,
    ) -> ElicitResult<()> {
        let type_id = TypeId::of::<T>();
        let mut styles = self.styles.write().map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "StyleContext lock poisoned: {}",
                e
            )))
        })?;
        styles.insert(type_id, Box::new(style));
        Ok(())
    }

    /// Get the custom style for a specific type, if one was set.
    ///
    /// Returns None if no custom style was provided, allowing
    /// fallback to T::Style::default().
    ///
    /// # Errors
    ///
    /// Returns an error if the lock is poisoned.
    #[tracing::instrument(skip(self), level = "debug", fields(type_id = ?TypeId::of::<T>()))]
    pub fn get_style<T: 'static, S: StyleMarker>(&self) -> ElicitResult<Option<S>> {
        let type_id = TypeId::of::<T>();
        let styles = self.styles.read().map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "StyleContext lock poisoned: {}",
                e
            )))
        })?;
        Ok(styles
            .get(&type_id)
            .and_then(|entry| entry.as_any().downcast_ref::<S>())
            .cloned())
    }

    /// Set a one-shot field-level prompt override.
    ///
    /// Called by the generated struct `elicit` before delegating to a field
    /// type's `elicit`. The override is consumed by the first
    /// [`prompt_for_type`] or [`take_field_prompt`] call, so it does not
    /// leak into nested elicitations.
    ///
    /// [`prompt_for_type`]: StyleContext::prompt_for_type
    /// [`take_field_prompt`]: StyleContext::take_field_prompt
    ///
    /// # Errors
    ///
    /// Returns an error if the lock is poisoned.
    pub fn set_field_prompt(&self, prompt: String) -> ElicitResult<()> {
        let mut guard = self.field_prompt.write().map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "StyleContext field_prompt lock poisoned: {}",
                e
            )))
        })?;
        *guard = Some(prompt);
        Ok(())
    }

    /// Consume and return the current field-level prompt override, if any.
    ///
    /// Returns `None` when no override is set, allowing types to fall back
    /// to their own [`Prompt::prompt`](crate::Prompt::prompt) default.
    ///
    /// # Errors
    ///
    /// Returns an error if the lock is poisoned.
    pub fn take_field_prompt(&self) -> ElicitResult<Option<String>> {
        let mut guard = self.field_prompt.write().map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "StyleContext field_prompt lock poisoned: {}",
                e
            )))
        })?;
        Ok(guard.take())
    }

    /// Generate a prompt for a type using its stored custom style.
    ///
    /// Checks the one-shot field-level prompt override first (set by the
    /// parent struct's generated `elicit`). Falls back to the style-derived
    /// prompt, then returns `None` so callers can fall back to the type's own
    /// [`Prompt::prompt`](crate::Prompt::prompt).
    ///
    /// # Errors
    ///
    /// Returns an error if the lock is poisoned.
    #[tracing::instrument(skip(self), level = "debug", fields(type_id = ?TypeId::of::<T>()))]
    pub fn prompt_for_type<T: 'static>(
        &self,
        field_name: &str,
        field_type: &str,
        context: &crate::style::PromptContext,
    ) -> ElicitResult<Option<String>> {
        // Field-level override wins; consumed here so nested calls see their own prompts.
        if let Some(prompt) = self.take_field_prompt()? {
            return Ok(Some(prompt));
        }
        let type_id = TypeId::of::<T>();
        let styles = self.styles.read().map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "StyleContext lock poisoned: {}",
                e
            )))
        })?;
        Ok(styles.get(&type_id).map(|entry| {
            entry
                .as_style()
                .prompt_for_field(field_name, field_type, context)
        }))
    }
}

/// Storage for current elicitation context (for observability).
///
/// Tracks the current "stack" of types being elicited, allowing introspection
/// of the elicitation state without storing full history. The stack only contains
/// the current chain of nested elicitations, providing O(1) memory per nesting level.
///
/// # Use Cases
///
/// - **Tracing**: Add type context to OpenTelemetry spans
/// - **Metrics**: Label Prometheus metrics with current type
/// - **Debugging**: Understand elicitation depth and current type
///
/// # Memory Efficiency
///
/// - **O(depth) memory**: Only stores current chain, not history
/// - **No accumulation**: Stack shrinks as elicitations complete
/// - **Stateless metadata**: TypeMetadata contains only static strings
#[derive(Clone, Default)]
pub struct ElicitationContext {
    stack: Arc<RwLock<Vec<TypeMetadata>>>,
}

impl ElicitationContext {
    /// Push a new type onto the elicitation stack.
    ///
    /// Call this when entering a new elicitation. Pair with `pop()` when done.
    ///
    /// # Errors
    ///
    /// Returns an error if the lock is poisoned.
    pub fn push(&self, metadata: TypeMetadata) -> ElicitResult<()> {
        let mut stack = self.stack.write().map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "ElicitationContext lock poisoned: {}",
                e
            )))
        })?;
        stack.push(metadata.clone());
        tracing::debug!(
            type_name = metadata.type_name,
            depth = stack.len(),
            "Entering elicitation"
        );
        Ok(())
    }

    /// Pop the current type from the elicitation stack.
    ///
    /// Call this when exiting an elicitation.
    ///
    /// # Errors
    ///
    /// Returns an error if the lock is poisoned.
    pub fn pop(&self) -> ElicitResult<()> {
        let mut stack = self.stack.write().map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "ElicitationContext lock poisoned: {}",
                e
            )))
        })?;
        if let Some(metadata) = stack.pop() {
            tracing::debug!(
                type_name = metadata.type_name,
                depth = stack.len(),
                "Exiting elicitation"
            );
        }
        Ok(())
    }

    /// Get the metadata for the currently elicited type.
    ///
    /// Returns `None` if no elicitation is currently in progress.
    ///
    /// # Errors
    ///
    /// Returns an error if the lock is poisoned.
    pub fn current(&self) -> ElicitResult<Option<TypeMetadata>> {
        let stack = self.stack.read().map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "ElicitationContext lock poisoned: {}",
                e
            )))
        })?;
        Ok(stack.last().cloned())
    }

    /// Get the current elicitation depth.
    ///
    /// Returns 0 if at the top level, 1 if eliciting a field of a struct, etc.
    ///
    /// # Errors
    ///
    /// Returns an error if the lock is poisoned.
    pub fn depth(&self) -> ElicitResult<usize> {
        let stack = self.stack.read().map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "ElicitationContext lock poisoned: {}",
                e
            )))
        })?;
        Ok(stack.len())
    }

    /// Get a snapshot of the full elicitation stack.
    ///
    /// Returns a vector of all types in the current chain, from root to current.
    /// Useful for debugging or detailed logging.
    ///
    /// # Errors
    ///
    /// Returns an error if the lock is poisoned.
    pub fn stack(&self) -> ElicitResult<Vec<TypeMetadata>> {
        let stack = self.stack.read().map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "ElicitationContext lock poisoned: {}",
                e
            )))
        })?;
        Ok(stack.clone())
    }
}
