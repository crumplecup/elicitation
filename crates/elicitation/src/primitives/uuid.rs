//! UUID implementation for universally unique identifier elicitation.
//!
//! Provides both direct elicitation and generator-based creation of UUIDs.
//!
//! # Generator Pattern
//!
//! The UUID generator supports multiple creation strategies useful for testing:
//!
//! ```rust,no_run
//! use elicitation::{UuidGenerationMode, UuidGenerator, Generator};
//!
//! // Choose generation mode
//! let mode = UuidGenerationMode::V4; // Random v4
//! // or V7 (timestamp-based), Nil (all zeros), Max (all ones)
//!
//! // Create generator
//! let generator = UuidGenerator::new(mode);
//!
//! // Generate multiple UUIDs with same strategy
//! let id1 = generator.generate();
//! let id2 = generator.generate();
//! let id3 = generator.generate();
//! ```

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitResult, Elicitation, Generator, Prompt,
    Select, mcp,
};
use uuid::Uuid;

// Generate default-only style enum
crate::default_style!(Uuid => UuidStyle);
crate::default_style!(UuidGenerationMode => UuidGenerationModeStyle);

// ============================================================================
// UUID Generator
// ============================================================================

/// Generation mode for UUID.
///
/// This enum allows an agent (or user) to specify how to create a UUID:
/// - `V4`: Random UUID (version 4)
/// - `Nil`: All zeros (00000000-0000-0000-0000-000000000000)
/// - `Max`: All ones (ffffffff-ffff-ffff-ffff-ffffffffffff)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UuidGenerationMode {
    /// Generate a random v4 UUID (most common).
    V4,
    /// Use nil UUID (all zeros) - useful for defaults.
    Nil,
    /// Use max UUID (all ones) - useful for sentinels.
    Max,
}

impl Select for UuidGenerationMode {
    fn options() -> &'static [Self] {
        &[
            UuidGenerationMode::V4,
            UuidGenerationMode::Nil,
            UuidGenerationMode::Max,
        ]
    }

    fn labels() -> &'static [&'static str] {
        &["V4 (Random)", "Nil (All zeros)", "Max (All ones)"]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "V4 (Random)" => Some(UuidGenerationMode::V4),
            "Nil (All zeros)" => Some(UuidGenerationMode::Nil),
            "Max (All ones)" => Some(UuidGenerationMode::Max),
            _ => None,
        }
    }
}

impl Prompt for UuidGenerationMode {
    fn prompt() -> Option<&'static str> {
        Some("How should UUIDs be generated?")
    }
}

impl Elicitation for UuidGenerationMode {
    type Style = UuidGenerationModeStyle;

    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        // Use standard Select elicit pattern
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Select an option:"),
            Self::labels(),
        );

        let result = communicator
            .call_tool(rmcp::model::CallToolRequestParams {
                meta: None,
                name: mcp::tool_names::elicit_select().into(),
                arguments: Some(params),
                task: None,
            })
            .await?;

        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;

        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(
                "Invalid UUID generation mode".to_string(),
            ))
        })
    }
}

/// Generator for creating UUIDs with a specified strategy.
///
/// Created from a [`UuidGenerationMode`] to enable consistent UUID generation
/// across multiple calls.
#[derive(Debug, Clone, Copy)]
pub struct UuidGenerator {
    mode: UuidGenerationMode,
}

impl UuidGenerator {
    /// Create a new UUID generator with the specified mode.
    pub fn new(mode: UuidGenerationMode) -> Self {
        Self { mode }
    }

    /// Get the generation mode.
    pub fn mode(&self) -> UuidGenerationMode {
        self.mode
    }
}

impl Generator for UuidGenerator {
    type Target = Uuid;

    fn generate(&self) -> Self::Target {
        match self.mode {
            UuidGenerationMode::V4 => Uuid::new_v4(),
            UuidGenerationMode::Nil => Uuid::nil(),
            UuidGenerationMode::Max => Uuid::max(),
        }
    }
}

// ============================================================================
// UUID Elicitation
// ============================================================================

impl Prompt for Uuid {
    fn prompt() -> Option<&'static str> {
        Some(
            "Please provide a UUID (hyphenated format), type 'generate' for a new random UUID, or choose a generation strategy:",
        )
    }
}

impl Elicitation for Uuid {
    type Style = UuidStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting UUID");

        // Elicit generation mode from agent
        let mode = UuidGenerationMode::elicit(communicator).await?;

        // Create generator and generate UUID
        let generator = UuidGenerator::new(mode);
        let uuid = generator.generate();

        tracing::debug!(uuid = %uuid, mode = ?mode, "Generated UUID");
        Ok(uuid)
    }
}
