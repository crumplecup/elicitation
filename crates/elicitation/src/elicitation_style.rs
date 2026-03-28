//! StyleMarker trait and infrastructure.
//!
//! This module defines the marker trait that all style enums must satisfy,
//! enabling users to define custom styles for any type.

/// Marker trait for elicitation style types.
///
/// Style types define how a type should be elicited. Each type has a default
/// style, but users can define custom styles and apply them at runtime.
///
/// This is a marker trait — it enforces bounds but has no methods. For
/// prompt customization, implement [`style::ElicitationStyle`](crate::style::ElicitationStyle)
/// which provides `prompt_for_field()` and related methods.
///
/// # Requirements
///
/// - `Clone`: Styles must be cloneable for context storage
/// - `Send + Sync`: Styles must be thread-safe
/// - `Default`: Provides fallback when no style is specified
/// - `'static`: Required for type-erased storage
///
/// # Example
///
/// ```rust,ignore
/// use elicitation::{StyleMarker, Elicitation, ElicitClient, ElicitResult};
///
/// // Define a custom style for i32
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// pub enum MyI32Style {
///     #[default]
///     Terse,
///     Verbose,
/// }
///
/// impl StyleMarker for MyI32Style {}
///
/// impl Elicitation for MyI32Style {
///     type Style = Self;
///     
///     async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
///         // Implement selection logic
///         Ok(Self::default())
///     }
/// }
///
/// // Use it:
/// let client = base_client.with_style::<i32, _>(MyI32Style::Verbose);
/// let value = i32::elicit(&client).await?;
/// ```
pub trait StyleMarker: Clone + Send + Sync + Default + 'static {
    // Marker trait - no methods required
    // The trait bounds provide everything the system needs
}

/// Blanket implementation for types that satisfy the requirements.
///
/// This automatically implements StyleMarker for any type that meets
/// the trait bounds, making it easy to define custom styles.
impl<T> StyleMarker for T where T: Clone + Send + Sync + Default + 'static {}
