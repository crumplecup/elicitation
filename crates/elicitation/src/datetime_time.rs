//! time datetime library elicitation implementations.
//!
//! Available with the `time` feature.
//!
//! This module provides `Elicitation` implementations for the modern, high
//! performance `time` crate. Supports both ISO 8601 string parsing and manual
//! component entry.
//!
//! # Supported Types
//!
//! - [`OffsetDateTime`] - Datetime with timezone offset
//! - [`PrimitiveDateTime`] - Datetime without timezone
//!
//! # Example
//!
//! ```rust,ignore
//! use time::OffsetDateTime;
//! use elicitation::Elicitation;
//! use rmcp::service::{Peer, RoleClient};
//!
//! async fn example(communicator: &Peer<RoleClient>) {
//!     // Elicit a datetime with offset
//!     let timestamp: OffsetDateTime = OffsetDateTime::elicit(communicator).await?;
//!     
//!     // User can choose:
//!     // 1. ISO 8601 string: "2024-07-11T15:30:00+05:00"
//!     // 2. Manual components: year, month, day, hour, minute, second, offset
//! }
//! ```
//!
//! # Elicitation Flow
//!
//! 1. **Input Method Selection** - User chooses ISO 8601 or manual components
//! 2. **Data Entry** - Based on selection:
//!    - ISO: Single string prompt with format validation
//!    - Manual: Six prompts for datetime + offset (for OffsetDateTime)
//! 3. **Validation** - time crate validates datetime construction
//! 4. **Result** - Returns validated datetime or error

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitResult, Elicitation, Generator, Prompt,
    Select,
    datetime_common::{DateTimeComponents, DateTimeInputMethod},
};
use std::time::{Duration, Instant};
use time::{OffsetDateTime, PrimitiveDateTime, UtcOffset};

// Style enums for time types
crate::default_style!(OffsetDateTime => OffsetDateTimeStyle);
crate::default_style!(PrimitiveDateTime => PrimitiveDateTimeStyle);
crate::default_style!(Instant => InstantStyle);
crate::default_style!(Time => TimeStyle);
crate::default_style!(OffsetDateTimeGenerationMode => OffsetDateTimeGenerationModeStyle);
crate::default_style!(PrimitiveDateTimeGenerationMode => PrimitiveDateTimeGenerationModeStyle);

// ============================================================================
// Instant Generator
// ============================================================================

/// Generation mode for time::Instant.
///
/// This enum allows an agent (or user) to specify how to create an Instant:
/// - `Now`: Use the actual current instant
/// - `Offset`: Create a mock instant by offsetting from a reference point
///
/// This is particularly useful for test data generation where deterministic
/// or specific timing is needed.
#[derive(Debug, Clone, Copy)]
pub enum InstantGenerationMode {
    /// Use the actual current instant (Instant::now())
    Now,

    /// Create an instant offset from a reference point.
    ///
    /// The offset can be positive (future) or negative (past).
    Offset {
        /// Seconds offset from reference (negative = past, positive = future)
        seconds: i64,
        /// Additional nanoseconds (0-999,999,999)
        nanos: u32,
    },
}

// Manual implementation of Select pattern for InstantGenerationMode
crate::default_style!(InstantGenerationMode => InstantGenerationModeStyle);

impl Prompt for InstantGenerationMode {
    fn prompt() -> Option<&'static str> {
        Some("Choose how to generate the instant:")
    }
}

impl crate::Select for InstantGenerationMode {
    fn options() -> &'static [Self] {
        &[
            InstantGenerationMode::Now,
            InstantGenerationMode::Offset {
                seconds: 0,
                nanos: 0,
            },
        ]
    }

    fn labels() -> &'static [&'static str] {
        &["Now (current time)", "Offset (from reference)"]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Now (current time)" => Some(InstantGenerationMode::Now),
            "Offset (from reference)" => Some(InstantGenerationMode::Offset {
                seconds: 0,
                nanos: 0,
            }),
            _ => None,
        }
    }
}

impl Elicitation for InstantGenerationMode {
    type Style = InstantGenerationModeStyle;

    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let prompt = Self::prompt().unwrap_or("Select an option:");
        
        let options_text = Self::labels()
            .iter()
            .enumerate()
            .map(|(i, label)| format!("{}. {}", i + 1, label))
            .collect::<Vec<_>>()
            .join("\n");
        let full_prompt = format!("{}\n\nOptions:\n{}", prompt, options_text);
        
        let response = communicator.send_prompt(&full_prompt).await?;
        let trimmed = response.trim();
        
        // Try to parse as number first
        let selected = if let Ok(choice) = trimmed.parse::<usize>() {
            if choice >= 1 && choice <= Self::labels().len() {
                let label = Self::labels()[choice - 1];
                Self::from_label(label)
            } else {
                None
            }
        } else {
            Self::from_label(trimmed)
        };
        
        let selected = selected.ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(
                "Invalid variant selection".to_string(),
            ))
        })?;

        // If Offset was selected, elicit the fields
        match selected {
            InstantGenerationMode::Now => Ok(InstantGenerationMode::Now),
            InstantGenerationMode::Offset { .. } => {
                // Elicit seconds
                let seconds = i64::elicit(communicator).await?;
                // Elicit nanos
                let nanos = u32::elicit(communicator).await?;
                Ok(InstantGenerationMode::Offset { seconds, nanos })
            }
        }
    }
}

/// Generator for time::Instant.
///
/// Encapsulates a strategy for creating Instant values. Can be configured
/// once via elicitation and then used to generate multiple instants with
/// the same strategy.
///
/// # Example
///
/// ```rust,ignore
/// // Elicit the strategy
/// let mode = InstantGenerationMode::elicit(communicator).await?;
/// let generator = InstantGenerator::new(mode);
///
/// // Generate multiple instants with same strategy
/// let event1_time = generator.generate();
/// let event2_time = generator.generate();
/// ```
#[derive(Debug, Clone)]
pub struct InstantGenerator {
    mode: InstantGenerationMode,
    reference: Instant,
}

impl InstantGenerator {
    /// Create a new generator with the given mode.
    ///
    /// The reference instant is captured at creation time.
    pub fn new(mode: InstantGenerationMode) -> Self {
        Self {
            mode,
            reference: Instant::now(),
        }
    }

    /// Create a generator with a specific reference instant.
    ///
    /// Useful for tests where you want deterministic offsets from a known point.
    pub fn with_reference(mode: InstantGenerationMode, reference: Instant) -> Self {
        Self { mode, reference }
    }
}

impl Generator for InstantGenerator {
    type Target = Instant;

    fn generate(&self) -> Instant {
        match &self.mode {
            InstantGenerationMode::Now => Instant::now(),
            InstantGenerationMode::Offset { seconds, nanos } => {
                let duration = Duration::new(*seconds as u64, *nanos);

                // For offset mode, we use the reference instant
                if *seconds >= 0 {
                    self.reference + duration
                } else {
                    // Negative offset - subtract duration
                    self.reference - Duration::new((-*seconds) as u64, *nanos)
                }
            }
        }
    }
}

// ============================================================================
// Instant Elicitation
// ============================================================================

#[cfg_attr(not(kani), elicitation_macros::instrumented_impl)]
impl Prompt for Instant {
    fn prompt() -> Option<&'static str> {
        Some("Specify how to create an instant (now vs offset):")
    }
}

#[cfg_attr(not(kani), elicitation_macros::instrumented_impl)]
impl Elicitation for Instant {
    type Style = InstantStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "Instant"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting time::Instant");

        // Elicit the generation mode
        let mode = InstantGenerationMode::elicit(communicator).await?;

        // Create generator and generate immediately
        let generator = InstantGenerator::new(mode);
        Ok(generator.generate())
    }
}

// ============================================================================
// OffsetDateTime Generator
// ============================================================================

/// Generation mode for time::OffsetDateTime.
///
/// This enum allows an agent (or user) to specify how to create an OffsetDateTime:
/// - `Now`: Current UTC time
/// - `UnixEpoch`: Unix epoch (1970-01-01 00:00:00 UTC)
/// - `Offset`: Time offset from a reference point
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OffsetDateTimeGenerationMode {
    /// Use current UTC time.
    Now,
    /// Use Unix epoch (1970-01-01 00:00:00 UTC).
    UnixEpoch,
    /// Offset from reference time.
    Offset {
        /// Seconds offset (positive = future, negative = past).
        seconds: i64,
        /// Nanoseconds component (0-999,999,999).
        nanos: i32,
    },
}

impl Select for OffsetDateTimeGenerationMode {
    fn options() -> &'static [Self] {
        &[
            OffsetDateTimeGenerationMode::Now,
            OffsetDateTimeGenerationMode::UnixEpoch,
            OffsetDateTimeGenerationMode::Offset {
                seconds: 0,
                nanos: 0,
            },
        ]
    }

    fn labels() -> &'static [&'static str] {
        &[
            "Now (Current UTC)",
            "Unix Epoch (1970-01-01)",
            "Offset (Custom)",
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Now (Current UTC)" => Some(OffsetDateTimeGenerationMode::Now),
            "Unix Epoch (1970-01-01)" => Some(OffsetDateTimeGenerationMode::UnixEpoch),
            "Offset (Custom)" => Some(OffsetDateTimeGenerationMode::Offset {
                seconds: 0,
                nanos: 0,
            }),
            _ => None,
        }
    }
}

impl Prompt for OffsetDateTimeGenerationMode {
    fn prompt() -> Option<&'static str> {
        Some("How should OffsetDateTime values be generated?")
    }
}

impl Elicitation for OffsetDateTimeGenerationMode {
    type Style = OffsetDateTimeGenerationModeStyle;

    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let prompt = Self::prompt().unwrap_or("Select an option:");
        
        let options_text = Self::labels()
            .iter()
            .enumerate()
            .map(|(i, label)| format!("{}. {}", i + 1, label))
            .collect::<Vec<_>>()
            .join("\n");
        let full_prompt = format!("{}\n\nOptions:\n{}", prompt, options_text);
        
        let response = communicator.send_prompt(&full_prompt).await?;
        let trimmed = response.trim();
        
        // Try to parse as number first
        let selected = if let Ok(choice) = trimmed.parse::<usize>() {
            if choice >= 1 && choice <= Self::labels().len() {
                let label = Self::labels()[choice - 1];
                Self::from_label(label)
            } else {
                None
            }
        } else {
            Self::from_label(trimmed)
        };
        
        let selected = selected.ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(
                "Invalid OffsetDateTime generation mode".to_string(),
            ))
        })?;

        match selected {
            OffsetDateTimeGenerationMode::Now => Ok(OffsetDateTimeGenerationMode::Now),
            OffsetDateTimeGenerationMode::UnixEpoch => Ok(OffsetDateTimeGenerationMode::UnixEpoch),
            OffsetDateTimeGenerationMode::Offset { .. } => {
                let seconds = i64::elicit(communicator).await?;
                let nanos = i32::elicit(communicator).await?;
                Ok(OffsetDateTimeGenerationMode::Offset { seconds, nanos })
            }
        }
    }
}

/// Generator for creating OffsetDateTime values with a specified strategy.
#[derive(Debug, Clone, Copy)]
pub struct OffsetDateTimeGenerator {
    mode: OffsetDateTimeGenerationMode,
    reference: OffsetDateTime,
}

impl OffsetDateTimeGenerator {
    /// Create a new OffsetDateTime generator with the specified mode.
    pub fn new(mode: OffsetDateTimeGenerationMode) -> Self {
        Self {
            mode,
            reference: OffsetDateTime::now_utc(),
        }
    }

    /// Create a generator with a custom reference time.
    pub fn with_reference(mode: OffsetDateTimeGenerationMode, reference: OffsetDateTime) -> Self {
        Self { mode, reference }
    }

    /// Get the generation mode.
    pub fn mode(&self) -> OffsetDateTimeGenerationMode {
        self.mode
    }

    /// Get the reference time.
    pub fn reference(&self) -> OffsetDateTime {
        self.reference
    }
}

impl Generator for OffsetDateTimeGenerator {
    type Target = OffsetDateTime;

    fn generate(&self) -> Self::Target {
        match self.mode {
            OffsetDateTimeGenerationMode::Now => OffsetDateTime::now_utc(),
            OffsetDateTimeGenerationMode::UnixEpoch => OffsetDateTime::UNIX_EPOCH,
            OffsetDateTimeGenerationMode::Offset { seconds, nanos } => {
                if seconds >= 0 {
                    self.reference + Duration::new(seconds as u64, nanos as u32)
                } else {
                    self.reference - Duration::new((-seconds) as u64, nanos.unsigned_abs())
                }
            }
        }
    }
}

// ============================================================================
// OffsetDateTime Elicitation
// ============================================================================

// OffsetDateTime implementation
impl Prompt for OffsetDateTime {
    fn prompt() -> Option<&'static str> {
        Some("Enter datetime with timezone offset:")
    }
}

impl Elicitation for OffsetDateTime {
    type Style = OffsetDateTimeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting OffsetDateTime");

        // Step 1: Choose input method
        let method = DateTimeInputMethod::elicit(communicator).await?;
        tracing::debug!(?method, "Input method selected");

        match method {
            DateTimeInputMethod::Iso8601String => {
                // Elicit ISO 8601 string
                let prompt =
                    "Enter ISO 8601 datetime with offset (e.g., \"2024-07-11T15:30:00+05:00\"):";
                let response = communicator.send_prompt(prompt).await?;
                let iso_string = response.trim();

                // Parse ISO 8601
                OffsetDateTime::parse(iso_string, &time::format_description::well_known::Rfc3339)
                    .map_err(|e| {
                        ElicitError::new(ElicitErrorKind::ParseError(format!(
                            "Invalid ISO 8601 datetime: {}",
                            e
                        )))
                    })
            }
            DateTimeInputMethod::ManualComponents => {
                // Elicit components
                let components = DateTimeComponents::elicit(communicator).await?;

                // Elicit offset
                let offset_prompt = "Enter timezone offset in hours (e.g., +5 or -8):";
                let offset_response = communicator.send_prompt(offset_prompt).await?;
                let offset_hours = offset_response.trim().parse::<i32>().map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(
                        format!("Invalid offset: '{}' ({})", offset_response.trim(), e)
                    ))
                })?;
                if !(-12..=14).contains(&offset_hours) {
                    return Err(ElicitError::new(ElicitErrorKind::ParseError(
                        format!("Offset must be between -12 and 14, got {}", offset_hours)
                    )));
                }

                let offset = UtcOffset::from_hms(offset_hours as i8, 0, 0).map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid timezone offset: {}",
                        e
                    )))
                })?;

                // Construct PrimitiveDateTime first
                let date = time::Date::from_calendar_date(
                    components.year,
                    time::Month::try_from(components.month).map_err(|e| {
                        ElicitError::new(ElicitErrorKind::ParseError(format!(
                            "Invalid month: {}",
                            e
                        )))
                    })?,
                    components.day,
                )
                .map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid date: {}", e)))
                })?;

                let time =
                    time::Time::from_hms(components.hour, components.minute, components.second)
                        .map_err(|e| {
                            ElicitError::new(ElicitErrorKind::ParseError(format!(
                                "Invalid time: {}",
                                e
                            )))
                        })?;

                Ok(PrimitiveDateTime::new(date, time).assume_offset(offset))
            }
        }
    }
}

// PrimitiveDateTime implementation
impl Prompt for PrimitiveDateTime {
    fn prompt() -> Option<&'static str> {
        Some("Enter datetime (no timezone):")
    }
}

impl Elicitation for PrimitiveDateTime {
    type Style = PrimitiveDateTimeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting PrimitiveDateTime");

        // Step 1: Choose input method
        let method = DateTimeInputMethod::elicit(communicator).await?;
        tracing::debug!(?method, "Input method selected");

        match method {
            DateTimeInputMethod::Iso8601String => {
                // Elicit ISO 8601 string (no timezone)
                let prompt = "Enter datetime (e.g., \"2024-07-11T15:30:00\"):";
                let response = communicator.send_prompt(prompt).await?;
                let iso_string = response.trim();

                // Parse ISO 8601 (primitive)
                PrimitiveDateTime::parse(
                    iso_string,
                    &time::format_description::well_known::Rfc3339,
                )
                .map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid datetime: {}",
                        e
                    )))
                })
            }
            DateTimeInputMethod::ManualComponents => {
                // Elicit components
                let components = DateTimeComponents::elicit(communicator).await?;

                // Construct PrimitiveDateTime
                let date = time::Date::from_calendar_date(
                    components.year,
                    time::Month::try_from(components.month).map_err(|e| {
                        ElicitError::new(ElicitErrorKind::ParseError(format!(
                            "Invalid month: {}",
                            e
                        )))
                    })?,
                    components.day,
                )
                .map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid date: {}", e)))
                })?;

                let time =
                    time::Time::from_hms(components.hour, components.minute, components.second)
                        .map_err(|e| {
                            ElicitError::new(ElicitErrorKind::ParseError(format!(
                                "Invalid time: {}",
                                e
                            )))
                        })?;

                Ok(PrimitiveDateTime::new(date, time))
            }
        }
    }
}
