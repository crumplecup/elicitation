//! SystemTime implementation for system clock time elicitation.
//!
//! Provides both direct elicitation and generator-based creation of SystemTime.
//!
//! # Generator Pattern
//!
//! The SystemTime generator supports multiple creation strategies useful for testing:
//!
//! ```rust,no_run
//! use elicitation::{SystemTimeGenerationMode, SystemTimeGenerator, Generator};
//! use std::time::{SystemTime, Duration};
//!
//! // Choose generation mode
//! let mode = SystemTimeGenerationMode::Now; // Current system time
//! // or UnixEpoch (1970-01-01), Offset{seconds, nanos}
//!
//! // Create generator
//! let generator = SystemTimeGenerator::new(mode);
//!
//! // Generate multiple timestamps with same strategy
//! let t1 = generator.generate();
//! let t2 = generator.generate();
//! let t3 = generator.generate();
//! ```

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitResult, Elicitation, Generator, Prompt,
    Select, mcp,
};
use std::time::{Duration, SystemTime};

// Generate default-only style enums
crate::default_style!(SystemTime => SystemTimeStyle);
crate::default_style!(SystemTimeGenerationMode => SystemTimeGenerationModeStyle);

// ============================================================================
// SystemTime Generator
// ============================================================================

/// Generation mode for SystemTime.
///
/// This enum allows an agent (or user) to specify how to create a SystemTime:
/// - `Now`: Current system time
/// - `UnixEpoch`: Unix epoch (1970-01-01 00:00:00 UTC)
/// - `Offset`: Time offset from a reference point by seconds and nanoseconds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemTimeGenerationMode {
    /// Use current system time (SystemTime::now()).
    Now,
    /// Use Unix epoch (UNIX_EPOCH constant).
    UnixEpoch,
    /// Offset from reference time.
    ///
    /// Positive seconds = future, negative = past.
    /// Nanoseconds are always added to the absolute value.
    Offset {
        /// Seconds offset (positive = future, negative = past).
        seconds: i64,
        /// Nanoseconds component (0-999,999,999).
        nanos: u32,
    },
}

impl Select for SystemTimeGenerationMode {
    fn options() -> &'static [Self] {
        &[
            SystemTimeGenerationMode::Now,
            SystemTimeGenerationMode::UnixEpoch,
            SystemTimeGenerationMode::Offset {
                seconds: 0,
                nanos: 0,
            },
        ]
    }

    fn labels() -> &'static [&'static str] {
        &[
            "Now (Current time)",
            "Unix Epoch (1970-01-01)",
            "Offset (Custom)",
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Now (Current time)" => Some(SystemTimeGenerationMode::Now),
            "Unix Epoch (1970-01-01)" => Some(SystemTimeGenerationMode::UnixEpoch),
            "Offset (Custom)" => Some(SystemTimeGenerationMode::Offset {
                seconds: 0,
                nanos: 0,
            }),
            _ => None,
        }
    }
}

impl Prompt for SystemTimeGenerationMode {
    fn prompt() -> Option<&'static str> {
        Some("How should system times be generated?")
    }
}

impl Elicitation for SystemTimeGenerationMode {
    type Style = SystemTimeGenerationModeStyle;

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

        let selected = Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(
                "Invalid SystemTime generation mode".to_string(),
            ))
        })?;

        // If Offset was selected, elicit the fields
        match selected {
            SystemTimeGenerationMode::Now => Ok(SystemTimeGenerationMode::Now),
            SystemTimeGenerationMode::UnixEpoch => Ok(SystemTimeGenerationMode::UnixEpoch),
            SystemTimeGenerationMode::Offset { .. } => {
                // Elicit seconds
                let seconds = i64::elicit(communicator).await?;
                // Elicit nanos
                let nanos = u32::elicit(communicator).await?;
                Ok(SystemTimeGenerationMode::Offset { seconds, nanos })
            }
        }
    }
}

/// Generator for creating SystemTime values with a specified strategy.
///
/// Created from a [`SystemTimeGenerationMode`] to enable consistent timestamp
/// generation across multiple calls.
#[derive(Debug, Clone, Copy)]
pub struct SystemTimeGenerator {
    mode: SystemTimeGenerationMode,
    reference: SystemTime,
}

impl SystemTimeGenerator {
    /// Create a new SystemTime generator with the specified mode.
    ///
    /// Uses `SystemTime::now()` as the reference point for offset calculations.
    pub fn new(mode: SystemTimeGenerationMode) -> Self {
        Self {
            mode,
            reference: SystemTime::now(),
        }
    }

    /// Create a generator with a custom reference time.
    ///
    /// Useful for deterministic testing where you want offsets from a known point.
    pub fn with_reference(mode: SystemTimeGenerationMode, reference: SystemTime) -> Self {
        Self { mode, reference }
    }

    /// Get the generation mode.
    pub fn mode(&self) -> SystemTimeGenerationMode {
        self.mode
    }

    /// Get the reference time used for offset calculations.
    pub fn reference(&self) -> SystemTime {
        self.reference
    }
}

impl Generator for SystemTimeGenerator {
    type Target = SystemTime;

    fn generate(&self) -> Self::Target {
        match self.mode {
            SystemTimeGenerationMode::Now => SystemTime::now(),
            SystemTimeGenerationMode::UnixEpoch => SystemTime::UNIX_EPOCH,
            SystemTimeGenerationMode::Offset { seconds, nanos } => {
                let duration = Duration::new(seconds.unsigned_abs(), nanos);
                if seconds >= 0 {
                    self.reference + duration
                } else {
                    self.reference - duration
                }
            }
        }
    }
}

// ============================================================================
// SystemTime Elicitation
// ============================================================================

impl Prompt for SystemTime {
    fn prompt() -> Option<&'static str> {
        Some("Choose how to create the system time:")
    }
}

impl Elicitation for SystemTime {
    type Style = SystemTimeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting SystemTime");

        // Elicit generation mode from agent
        let mode = SystemTimeGenerationMode::elicit(communicator).await?;

        // Create generator and generate time
        let generator = SystemTimeGenerator::new(mode);
        let time = generator.generate();

        tracing::debug!(time = ?time, mode = ?mode, "Generated SystemTime");
        Ok(time)
    }
}
