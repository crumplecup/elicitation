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
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, Generator, PatternDetails, Prompt, Select, TypeMetadata, mcp,
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
    fn options() -> Vec<Self> {
        vec![
            UuidGenerationMode::V4,
            UuidGenerationMode::Nil,
            UuidGenerationMode::Max,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "V4 (Random)".to_string(),
            "Nil (All zeros)".to_string(),
            "Max (All ones)".to_string(),
        ]
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
            &Self::labels(),
        );

        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
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

impl ElicitIntrospect for UuidGenerationMode {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "UuidGenerationMode",
            description: Self::prompt(),
            details: PatternDetails::Select {
                options: Self::labels(),
            },
        }
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

impl ElicitIntrospect for Uuid {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "uuid::Uuid",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}
