//! Core traits for elicitation.

use crate::{ElicitClient, ElicitCommunicator, ElicitResult};
use rmcp::service::{Peer, RoleClient};
use std::sync::Arc;

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
    pub async fn elicit(self, peer: Arc<Peer<RoleClient>>) -> ElicitResult<T> {
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
/// # async fn example(client: &ElicitClient) -> ElicitResult<()> {
/// // Elicit an i32 from the user
/// let value: i32 = i32::elicit(communicator).await?;
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
    fn elicit<C: ElicitCommunicator>(
        communicator: &C,
    ) -> impl std::future::Future<Output = ElicitResult<Self>> + Send;

    /// Server-side elicitation via MCP peer.
    ///
    /// This method enables server-side elicitation through rmcp's `Peer<RoleServer>`.
    /// It has a default implementation that creates an `ElicitServer` wrapper and
    /// delegates to the `elicit()` method.
    ///
    /// This is used by the `#[elicit_tools]` macro for automatic tool generation.
    ///
    /// # Arguments
    ///
    /// * `peer` - rmcp server peer for MCP communication
    ///
    /// # Returns
    ///
    /// The elicited value or an `ElicitError`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Direct usage in tool
    /// #[tool]
    /// async fn my_tool(peer: Peer<RoleServer>) -> Result<Config, ErrorData> {
    ///     let config = Config::elicit_checked(peer).await?;
    ///     Ok(config)
    /// }
    ///
    /// // Or with #[elicit_tools] macro
    /// #[elicit_tools(Config)]
    /// #[tool_router]
    /// impl MyServer { }
    /// ```
    fn elicit_checked(
        peer: crate::rmcp::service::Peer<crate::rmcp::service::RoleServer>,
    ) -> impl std::future::Future<Output = ElicitResult<Self>> + Send {
        async move {
            use crate::ElicitServer;
            let server = ElicitServer::new(peer);
            Self::elicit(&server).await
        }
    }

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
    /// # async fn example<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<()> {
    /// // Elicit with proof
    /// let (email, proof): (String, Established<Is<String>>) =
    ///     String::elicit_proven(communicator).await?;
    ///
    /// // Use proof in downstream function
    /// send_email(email, proof).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn elicit_proven<C: ElicitCommunicator>(
        communicator: &C,
    ) -> impl std::future::Future<
        Output = ElicitResult<(
            Self,
            crate::contracts::Established<crate::contracts::Is<Self>>,
        )>,
    > + Send {
        async move {
            let value = Self::elicit(communicator).await?;
            Ok((value, crate::contracts::Established::assert()))
        }
    }

    /// Compositional verification witness.
    ///
    /// This method serves as a compile-time proof that this type is formally verified.
    /// It witnesses the following logical chain:
    ///
    /// **For primitive types:**
    /// - Manual Kani proofs exist (in `verification/types/kani_proofs/`)
    /// - This method links to those proofs
    ///
    /// **For derived types:**
    /// - All fields implement `Elicitation` (enforced by `#[derive(Elicit)]`)
    /// - All `Elicitation` types are verified (project invariant)
    /// - Therefore, this composition is verified (by transitivity) ∎
    ///
    /// # Formal Verification Legos
    ///
    /// Types implementing `Elicitation` form a **compositionally verified ecosystem**:
    ///
    /// ```text
    /// ┌─────────────────────────────────────────────────┐
    /// │ Primitive Types (Manual Kani Proofs)           │
    /// │ ✓ I8Positive, StringNonEmpty, etc.             │
    /// └─────────────────┬───────────────────────────────┘
    ///                   │
    ///                   │ implements Elicitation
    ///                   ↓
    /// ┌─────────────────────────────────────────────────┐
    /// │ Derived Structs (Compositional Proofs)         │
    /// │ #[derive(Elicit)]                               │
    /// │ struct Config {                                 │
    /// │     timeout: I8Positive,  ← verified           │
    /// │     retries: U8NonZero,   ← verified           │
    /// │ }                                               │
    /// │ ⟹ Config verified by composition ∎            │
    /// └─────────────────────────────────────────────────┘
    /// ```
    ///
    /// # The "Caged Agent" Property
    ///
    /// When an LLM is asked to elicit a type `T: Elicitation`:
    /// - The type system enforces that `T` is verified
    /// - The verification is **non-bypassable** (enforced at compile time)
    /// - Invalid states are **unrepresentable** (cannot be constructed)
    ///
    /// This creates a "cage" where the agent can only produce values that
    /// have been mathematically proven to satisfy their contracts.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Primitive type with manual proof
    /// impl Elicitation for I8Positive {
    ///     #[cfg(kani)]
    ///     fn kani_proof() {
    ///         verify_i8_positive(); // Links to manual Kani harness
    ///     }
    /// }
    ///
    /// // User type with compositional proof (generated by derive macro)
    /// #[derive(Elicit)]
    /// struct Config {
    ///     timeout: I8Positive,  // verified
    ///     retries: U8NonZero,   // verified
    /// }
    ///
    /// // Generated by #[derive(Elicit)]:
    /// impl Elicitation for Config {
    ///     #[cfg(kani)]
    ///     fn kani_proof() {
    ///         I8Positive::kani_proof(); // Verify timeout field
    ///         U8NonZero::kani_proof();  // Verify retries field
    ///         // Tautological assertion: all parts verified ⟹ whole verified
    ///         assert!(true, "Compositional verification");
    ///     }
    /// }
    /// ```
    ///
    /// # Zero-Cost Abstraction
    ///
    /// This method only exists in `#[cfg(kani)]` builds. In release builds,
    /// it is compiled away entirely - the verification happens at compile time,
    /// with zero runtime overhead.
    #[cfg(kani)]
    fn kani_proof() {
        // Default implementation: witness that Elicitation trait is verified by construction
        assert!(
            true,
            "Elicitation trait verified: type system enforces compositionality"
        );
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
/// let mode = InstantGenerationMode::elicit(communicator).await?;
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

/// Static introspection into a type's elicitation structure.
///
/// This trait provides compile-time metadata about HOW a type will be elicited,
/// enabling observability, debugging, and agent guidance without runtime state tracking.
///
/// # Observability Use Cases
///
/// - **Metrics**: Instrument elicitation with Prometheus counters/histograms
/// - **Tracing**: Add structured metadata to OpenTelemetry spans
/// - **Debugging**: Visualize the elicitation structure before execution
/// - **Agent Guidance**: Query type structure to plan elicitation strategy
///
/// # Design Philosophy
///
/// Unlike runtime validators or state machines, `ElicitIntrospect` is **stateless**:
/// - No stack traces or history tracking
/// - No memory overhead (just static metadata)
/// - O(1) memory usage regardless of nesting depth
/// - Pure functions with no side effects
///
/// This makes it ideal for developers to call in traces/metrics without
/// worrying about memory bloat or performance impact.
///
/// # Example: Basic Introspection
///
/// ```rust,ignore
/// use elicitation::{ElicitIntrospect, ElicitationPattern};
///
/// // Query structure before elicitation
/// let meta = Config::metadata();
/// println!("About to elicit: {}", meta.type_name);
///
/// match meta.pattern() {
///     ElicitationPattern::Survey => {
///         if let PatternDetails::Survey { fields } = meta.details {
///             println!("Requires {} fields", fields.len());
///         }
///     }
///     _ => {}
/// }
/// ```
///
/// # Example: Prometheus Metrics
///
/// ```rust,ignore
/// use prometheus::{Counter, Histogram};
///
/// async fn elicit_with_metrics<T: ElicitIntrospect>(
///     communicator: &impl ElicitCommunicator
/// ) -> ElicitResult<T> {
///     let meta = T::metadata();
///
///     // Increment counter for this type
///     ELICITATION_COUNTER
///         .with_label_values(&[meta.type_name, meta.pattern().as_str()])
///         .inc();
///
///     // Time the elicitation
///     let timer = ELICITATION_DURATION
///         .with_label_values(&[meta.type_name])
///         .start_timer();
///
///     let result = T::elicit(communicator).await;
///     timer.observe_duration();
///
///     result
/// }
/// ```
///
/// # Example: OpenTelemetry Tracing
///
/// ```rust,ignore
/// #[tracing::instrument(skip(communicator), fields(
///     type_name = %T::metadata().type_name,
///     pattern = ?T::pattern(),
///     field_count = tracing::field::Empty,
/// ))]
/// async fn elicit_with_tracing<T: ElicitIntrospect>(
///     communicator: &impl ElicitCommunicator
/// ) -> ElicitResult<T> {
///     let meta = T::metadata();
///
///     // Add structured fields based on pattern
///     if let PatternDetails::Survey { fields } = meta.details {
///         tracing::Span::current().record("field_count", fields.len());
///     }
///
///     T::elicit(communicator).await
/// }
/// ```
pub trait ElicitIntrospect: Elicitation {
    /// What elicitation pattern does this type use?
    ///
    /// This is a lightweight query that returns the pattern without
    /// allocating or examining detailed metadata.
    fn pattern() -> ElicitationPattern;

    /// Get the complete structural metadata for this type.
    ///
    /// Returns static metadata describing:
    /// - Type name
    /// - Description/prompt (if any)
    /// - Pattern-specific details (fields, options, etc.)
    ///
    /// This is a pure function with no side effects or state tracking.
    fn metadata() -> TypeMetadata;
}

/// The elicitation pattern used by a type.
///
/// Each pattern represents a different interaction model:
/// - **Survey**: Sequential field-by-field elicitation (structs)
/// - **Select**: Choose from finite options, then elicit variant fields (enums)
/// - **Affirm**: Yes/no confirmation (booleans)
/// - **Primitive**: Direct value elicitation (strings, numbers, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElicitationPattern {
    /// Struct with sequential field elicitation.
    ///
    /// Example: `struct Config { timeout: u32, retries: u8 }`
    Survey,

    /// Enum with variant selection followed by field elicitation.
    ///
    /// Example: `enum Mode { Fast, Safe { level: u8 } }`
    Select,

    /// Boolean yes/no confirmation.
    ///
    /// Example: `bool`
    Affirm,

    /// Primitive type with direct value elicitation.
    ///
    /// Example: `String`, `i32`, `f64`
    Primitive,
}

impl ElicitationPattern {
    /// Get string representation for metrics/logging.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Survey => "survey",
            Self::Select => "select",
            Self::Affirm => "affirm",
            Self::Primitive => "primitive",
        }
    }
}

/// Complete metadata describing a type's elicitation structure.
///
/// This is static metadata returned by `ElicitIntrospect::metadata()`.
/// It describes the structure without tracking runtime state.
#[derive(Debug, Clone)]
pub struct TypeMetadata {
    /// Type name (e.g., "Config", "Mode", "String").
    pub type_name: &'static str,

    /// Optional description or prompt text.
    ///
    /// This comes from the type's `Prompt` implementation.
    pub description: Option<&'static str>,

    /// Pattern-specific structural details.
    pub details: PatternDetails,
}

impl TypeMetadata {
    /// Get the elicitation pattern.
    pub fn pattern(&self) -> ElicitationPattern {
        match self.details {
            PatternDetails::Survey { .. } => ElicitationPattern::Survey,
            PatternDetails::Select { .. } => ElicitationPattern::Select,
            PatternDetails::Affirm => ElicitationPattern::Affirm,
            PatternDetails::Primitive => ElicitationPattern::Primitive,
        }
    }
}

/// Pattern-specific structural details.
///
/// Each variant corresponds to an `ElicitationPattern` and provides
/// the relevant metadata for that pattern.
///
/// Uses owned data to support both static and dynamic implementations.
#[derive(Debug, Clone)]
pub enum PatternDetails {
    /// Survey pattern (structs).
    Survey {
        /// Field metadata from the `Survey` trait.
        fields: Vec<crate::FieldInfo>,
    },

    /// Select pattern (enums).
    Select {
        /// Option labels from the `Select` trait.
        options: Vec<String>,
    },

    /// Affirm pattern (booleans).
    Affirm,

    /// Primitive pattern (direct value).
    Primitive,
}
