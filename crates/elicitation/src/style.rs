//! Elicitation style system - separates UX from behavior.
//!
//! This module provides a trait-based system for customizing how elicitation
//! prompts are presented to users, without changing the underlying questions
//! being asked.
//!
//! # Core Concept
//!
//! **Elicitation behavior** (what questions to ask) is separate from
//! **elicitation style** (how to present those questions).
//!
//! # Example
//!
//! ```rust,ignore
//! use elicitation::{Elicitation, style::{DefaultStyle, CompactStyle}};
//!
//! // Same type, different UX
//! #[derive(Elicit)]
//! #[elicit(style = DefaultStyle)]
//! struct Config {
//!     host: String,  // "Enter host:"
//!     port: u16,     // "Enter port:"
//! }
//!
//! #[derive(Elicit)]
//! #[elicit(style = CompactStyle)]
//! struct CompactConfig {
//!     host: String,  // "host:"
//!     port: u16,     // "port:"
//! }
//! ```
//!
//! # Built-in Styles
//!
//! - [`DefaultStyle`] - Balanced, standard prompts
//! - [`CompactStyle`] - Minimal, terse output
//! - [`VerboseStyle`] - Detailed help and guidance
//! - [`WizardStyle`] - Step-by-step with progress indicators

use std::fmt;

/// Context information for prompt generation.
///
/// Provides metadata about the current field being elicited to help
/// styles make informed decisions about formatting.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PromptContext {
    /// Current field index (0-based)
    pub field_index: usize,
    /// Total number of fields in the survey
    pub total_fields: usize,
    /// Parent type name (for nested elicitation)
    pub parent_type: Option<String>,
    /// Nesting depth (0 = top-level)
    pub depth: usize,
}

impl PromptContext {
    /// Create a new prompt context.
    #[tracing::instrument(level = "trace")]
    pub fn new(field_index: usize, total_fields: usize) -> Self {
        Self {
            field_index,
            total_fields,
            parent_type: None,
            depth: 0,
        }
    }

    /// Create context with parent information.
    #[tracing::instrument(level = "trace")]
    pub fn with_parent(
        field_index: usize,
        total_fields: usize,
        parent_type: String,
        depth: usize,
    ) -> Self {
        Self {
            field_index,
            total_fields,
            parent_type: Some(parent_type),
            depth,
        }
    }
}

/// Controls how elicitation prompts are presented.
///
/// Implement this trait to create custom elicitation styles. The library
/// provides several built-in implementations covering common use cases.
pub trait ElicitationStyle: Send + Sync {
    /// Generate prompt text for a field.
    ///
    /// # Arguments
    ///
    /// * `field_name` - Field identifier (e.g., "host")
    /// * `field_type` - Type name (e.g., "String", "u16")
    /// * `context` - Additional metadata about the field
    ///
    /// # Returns
    ///
    /// The formatted prompt string to display to the user.
    fn prompt_for_field(
        &self,
        field_name: &str,
        field_type: &str,
        context: &PromptContext,
    ) -> String;

    /// Generate optional help text for a field.
    ///
    /// # Arguments
    ///
    /// * `field_name` - Field identifier
    /// * `field_type` - Type name
    ///
    /// # Returns
    ///
    /// `Some(help)` to display help text, `None` for no help.
    fn help_text(&self, field_name: &str, field_type: &str) -> Option<String> {
        let _ = (field_name, field_type);
        None
    }

    /// Format validation error messages.
    ///
    /// # Arguments
    ///
    /// * `field_name` - Field that failed validation
    /// * `error` - Error message from validator
    ///
    /// # Returns
    ///
    /// User-friendly error message.
    fn validation_error(&self, field_name: &str, error: &str) -> String {
        format!("Invalid value for {}: {}", field_name, error)
    }

    /// Whether to show type hints in prompts.
    fn show_type_hints(&self) -> bool {
        true
    }

    /// Style for select/dropdown interactions.
    fn select_style(&self) -> SelectStyle {
        SelectStyle::Menu
    }

    /// Whether to use decorative elements (borders, icons, etc.).
    fn use_decorations(&self) -> bool {
        false
    }

    /// Prefix for prompts (e.g., "? ", "> ").
    fn prompt_prefix(&self) -> &str {
        ""
    }
}

/// Rendering style for select/dropdown interactions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectStyle {
    /// Full menu with numbered options
    Menu,
    /// Inline options in prompt text
    Inline,
    /// Searchable/filterable list
    Search,
}

/// Default balanced style - standard prompts with type hints.
#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultStyle;

impl ElicitationStyle for DefaultStyle {
    fn prompt_for_field(
        &self,
        field_name: &str,
        field_type: &str,
        _context: &PromptContext,
    ) -> String {
        format!("Enter {} ({}):", field_name, field_type)
    }

    fn show_type_hints(&self) -> bool {
        true
    }

    fn select_style(&self) -> SelectStyle {
        SelectStyle::Menu
    }
}

impl fmt::Display for DefaultStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "default")
    }
}

/// Compact terse style - minimal prompts, no type hints.
#[derive(Debug, Clone, Copy, Default)]
pub struct CompactStyle;

impl ElicitationStyle for CompactStyle {
    fn prompt_for_field(
        &self,
        field_name: &str,
        _field_type: &str,
        _context: &PromptContext,
    ) -> String {
        format!("{}:", field_name)
    }

    fn show_type_hints(&self) -> bool {
        false
    }

    fn validation_error(&self, field_name: &str, error: &str) -> String {
        format!("{}: {}", field_name, error)
    }

    fn prompt_prefix(&self) -> &str {
        "> "
    }
}

impl fmt::Display for CompactStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "compact")
    }
}

/// Verbose helpful style - detailed prompts with guidance.
#[derive(Debug, Clone, Copy, Default)]
pub struct VerboseStyle;

impl ElicitationStyle for VerboseStyle {
    fn prompt_for_field(
        &self,
        field_name: &str,
        field_type: &str,
        context: &PromptContext,
    ) -> String {
        format!(
            "Please enter {} (type: {}, field {}/{})",
            field_name,
            field_type,
            context.field_index + 1,
            context.total_fields
        )
    }

    fn help_text(&self, _field_name: &str, field_type: &str) -> Option<String> {
        Some(match field_type {
            "String" => "Enter any text value".to_string(),
            "u16" | "u32" | "u64" => "Enter a positive number".to_string(),
            "i16" | "i32" | "i64" => "Enter any integer".to_string(),
            "f32" | "f64" => "Enter a decimal number".to_string(),
            "bool" => "Enter yes or no".to_string(),
            _ => format!("Enter a valid {}", field_type),
        })
    }

    fn show_type_hints(&self) -> bool {
        true
    }

    fn validation_error(&self, field_name: &str, error: &str) -> String {
        format!(
            "The value you entered for '{}' is invalid: {}. Please try again.",
            field_name, error
        )
    }

    fn prompt_prefix(&self) -> &str {
        "? "
    }
}

impl fmt::Display for VerboseStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "verbose")
    }
}

/// Wizard step-by-step style - progress indicators and guidance.
#[derive(Debug, Clone, Copy, Default)]
pub struct WizardStyle;

impl ElicitationStyle for WizardStyle {
    fn prompt_for_field(
        &self,
        field_name: &str,
        field_type: &str,
        context: &PromptContext,
    ) -> String {
        format!(
            "Step {} of {}: Enter {} ({})",
            context.field_index + 1,
            context.total_fields,
            field_name,
            field_type
        )
    }

    fn help_text(&self, _field_name: &str, field_type: &str) -> Option<String> {
        Some(match field_type {
            "String" => "Type your answer and press Enter".to_string(),
            ty if ty.starts_with('u') || ty.starts_with('i') || ty.starts_with('f') => {
                "Enter a numeric value".to_string()
            }
            "bool" => "Answer yes or no".to_string(),
            _ => "Enter your response".to_string(),
        })
    }

    fn show_type_hints(&self) -> bool {
        true
    }

    fn use_decorations(&self) -> bool {
        true
    }

    fn validation_error(&self, field_name: &str, error: &str) -> String {
        format!("❌ Invalid {}: {}. Let's try again.", field_name, error)
    }

    fn prompt_prefix(&self) -> &str {
        "➤ "
    }
}

impl fmt::Display for WizardStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "wizard")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_style() {
        let style = DefaultStyle;
        let ctx = PromptContext::new(0, 3);
        assert_eq!(
            style.prompt_for_field("host", "String", &ctx),
            "Enter host (String):"
        );
        assert!(style.show_type_hints());
        assert_eq!(style.select_style(), SelectStyle::Menu);
    }

    #[test]
    fn test_compact_style() {
        let style = CompactStyle;
        let ctx = PromptContext::new(0, 3);
        assert_eq!(style.prompt_for_field("host", "String", &ctx), "host:");
        assert!(!style.show_type_hints());
        assert_eq!(style.prompt_prefix(), "> ");
    }

    #[test]
    fn test_verbose_style() {
        let style = VerboseStyle;
        let ctx = PromptContext::new(1, 3);
        assert_eq!(
            style.prompt_for_field("port", "u16", &ctx),
            "Please enter port (type: u16, field 2/3)"
        );
        assert!(style.help_text("port", "u16").is_some());
        assert_eq!(style.prompt_prefix(), "? ");
    }

    #[test]
    fn test_wizard_style() {
        let style = WizardStyle;
        let ctx = PromptContext::new(0, 3);
        assert_eq!(
            style.prompt_for_field("host", "String", &ctx),
            "Step 1 of 3: Enter host (String)"
        );
        assert!(style.use_decorations());
        assert_eq!(style.prompt_prefix(), "➤ ");
    }

    #[test]
    fn test_prompt_context() {
        let ctx = PromptContext::new(2, 5);
        assert_eq!(ctx.field_index, 2);
        assert_eq!(ctx.total_fields, 5);
        assert_eq!(ctx.depth, 0);

        let nested = PromptContext::with_parent(1, 3, "Config".to_string(), 1);
        assert_eq!(nested.parent_type, Some("Config".to_string()));
        assert_eq!(nested.depth, 1);
    }
}
