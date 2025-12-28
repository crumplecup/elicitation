//! Core traits for elicitation.

use crate::ElicitResult;

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
/// # Example
///
/// ```rust,no_run
/// use elicitation::{Elicit, ElicitResult};
/// # async fn example(client: &pmcp::Client) -> ElicitResult<()> {
/// // Elicit an i32 from the user
/// let value: i32 = i32::elicit(client).await?;
/// # Ok(())
/// # }
/// ```
pub trait Elicit: Sized + Prompt {
    /// Elicit a value of this type from the user via MCP.
    ///
    /// # Arguments
    ///
    /// * `client` - The MCP client to use for interaction
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
    async fn elicit<T: pmcp::shared::transport::Transport>(
        client: &pmcp::Client<T>,
    ) -> ElicitResult<Self>;
}
