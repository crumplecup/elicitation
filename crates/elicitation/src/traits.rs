//! Core traits for elicitation.

use crate::{ElicitClient, ElicitResult};
use rmcp::service::{Peer, RoleClient};

/// Builder for one-off style overrides.
///
/// Enables ergonomic syntax: `Config::with_style(ConfigStyle::Curt).elicit(&peer).await?`
pub struct ElicitBuilder<T: Elicitation> {
    style: T::Style,
}

impl<T: Elicitation + 'static> ElicitBuilder<T> {
    /// Create a new builder with the given style.
    fn new(style: T::Style) -> Self {
        Self { style }
    }

    /// Elicit the value with the pre-set style.
    ///
    /// This is a convenience method that creates an ElicitClient, sets the style,
    /// and elicits the value in one call.
    ///
    /// # Arguments
    ///
    /// * `peer` - The RMCP peer to use for interaction
    ///
    /// # Returns
    ///
    /// Returns the elicited value with the style applied.
    pub async fn elicit(self, peer: &Peer<RoleClient>) -> ElicitResult<T> {
        let client = ElicitClient::new(peer).with_style::<T, T::Style>(self.style);
        T::elicit(&client).await
    }
}

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
pub trait Elicitation: Sized + Prompt + 'static {
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

    /// Create a builder for one-off style override.
    ///
    /// This enables ergonomic syntax for eliciting a value with a specific style
    /// without manually creating a styled client.
    ///
    /// # Arguments
    ///
    /// * `style` - The style to use for this elicitation
    ///
    /// # Returns
    ///
    /// Returns an `ElicitBuilder` that can be used to elicit the value.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use elicitation::Elicitation;
    /// # async fn example(peer: &botticelli::Peer<botticelli_core::RoleClient>) {
    /// // One-off style override - concise syntax
    /// let config = Config::with_style(ConfigStyle::Curt)
    ///     .elicit(&peer)
    ///     .await?;
    /// # }
    /// ```
    fn with_style(style: Self::Style) -> ElicitBuilder<Self> {
        ElicitBuilder::new(style)
    }

    /// Elicit a value with proof it inhabits type Self.
    ///
    /// After successful elicitation, returns both the value and a proof
    /// that the value inhabits type `Self`. This proof can be carried
    /// forward to downstream functions requiring guarantees.
    ///
    /// # Returns
    ///
    /// Returns `Ok((value, proof))` where `proof` is `Established<Is<Self>>`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use elicitation::{Elicitation, contracts::{Established, Is}};
    /// # async fn example(client: &ElicitClient<'_>) -> ElicitResult<()> {
    /// // Elicit with proof
    /// let (email, proof): (String, Established<Is<String>>) = 
    ///     String::elicit_proven(client).await?;
    ///
    /// // Use proof in downstream function
    /// send_email(email, proof).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn elicit_proven(
        client: &ElicitClient<'_>,
    ) -> impl std::future::Future<Output = ElicitResult<(Self, crate::contracts::Established<crate::contracts::Is<Self>>)>>
           + Send {
        async move {
            let value = Self::elicit(client).await?;
            Ok((value, crate::contracts::Established::assert()))
        }
    }
}

/// Trait for generating values of a type.
///
/// Generators encapsulate strategies for creating values without requiring
/// async elicitation. This is useful for:
/// - Test data generation with configurable strategies
/// - Mock value creation for testing
/// - Deterministic value generation (seeded randomness, offsets, etc.)
/// - Agent-driven test fixture creation
///
/// # Design Philosophy
///
/// Generators are **orthogonal to elicitation**. They:
/// - Are synchronous (no async/await)
/// - Don't require MCP client access
/// - Can be configured once and used many times
/// - Encapsulate "how to create this value" as data
///
/// Elicitation implementations can leverage generators when appropriate,
/// but generators exist independently and can be used without elicitation.
///
/// # Example
///
/// ```rust,ignore
/// // Elicit the generation strategy once
/// let mode = InstantGenerationMode::elicit(client).await?;
/// let generator = InstantGenerator::new(mode);
///
/// // Generate many values with the same strategy
/// let t1 = generator.generate();
/// let t2 = generator.generate();
/// let t3 = generator.generate();
/// ```
pub trait Generator {
    /// The type this generator produces.
    type Target;

    /// Generate a value of the target type.
    ///
    /// This is synchronous - all configuration must happen before calling generate().
    fn generate(&self) -> Self::Target;
}
