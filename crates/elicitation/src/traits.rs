//! Core traits for elicitation.

use crate::{ElicitClient, ElicitResult};

/// Shared metadata for prompts across all elicitation patterns.
///
/// This trait provides optional prompt text to guide user interaction.
/// Types can override this to provide custom prompts, or accept the
/// default (None).
pub trait Prompt {
    /// Optional prompt to guide user interaction.
    ///
    /// Returns `None` by default. Implement this to provide a custom prompt
    /// for a type.
    fn prompt() -> Option<&'static str> {
        None
    }
}

/// Main elicitation trait - entry point for value elicitation.
///
/// This trait defines how to elicit a value of a given type from the user
/// via MCP (Model Context Protocol). All types that can be elicited implement
/// this trait.
///
/// # Associated Types
///
/// * `Style` - The style enum for this type. Each type has its own style
///   enum that controls how prompts are presented. The style enum itself
///   implements `Elicitation`, allowing automatic style selection.
///
/// # Example
///
/// ```rust,ignore
/// use elicitation::{Elicitation, ElicitClient, ElicitResult};
/// # async fn example(client: &ElicitClient<'_>) -> ElicitResult<()> {
/// // Elicit an i32 from the user
/// let value: i32 = i32::elicit(client).await?;
/// # Ok(())
/// # }
/// ```
pub trait Elicitation: Sized + Prompt {
    /// The style enum for this type.
    ///
    /// Controls how prompts are presented. For types with multiple styles,
    /// this enum has variants for each style. For types with no custom styles,
    /// this enum has only a `Default` variant.
    ///
    /// The style enum itself implements `Elicitation` (using the Select pattern),
    /// enabling automatic style selection when no style is pre-set.
    type Style: Elicitation + Default + Clone + Send + Sync + 'static;

    /// Elicit a value of this type from the user via style-aware client.
    ///
    /// # Arguments
    ///
    /// * `client` - The style-aware client wrapper to use for interaction
    ///
    /// # Returns
    ///
    /// Returns `Ok(Self)` if elicitation succeeds, or `Err(ElicitError)` if:
    /// - The user provides invalid input
    /// - The MCP tool call fails
    /// - The user cancels the operation
    ///
    /// # Errors
    ///
    /// See [`ElicitError`](crate::ElicitError) for details on error conditions.
    fn elicit(
        client: &ElicitClient<'_>,
    ) -> impl std::future::Future<Output = ElicitResult<Self>> + Send;
}
