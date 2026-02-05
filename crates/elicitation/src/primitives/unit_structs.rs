//! Unit struct implementations - the simplest possible elicitation!
//!
//! Unit structs have exactly one value, making them trivial to generate.
//! But they're still useful for type-safe agent outputs, especially when
//! they have associated methods.
//!
//! # Why Elicit Unit Structs?
//!
//! Unit structs often carry behavior via associated methods:
//!
//! ```rust,ignore
//! pub struct Validator;
//!
//! impl Validator {
//!     pub fn is_utf8(&self, bytes: &[u8]) -> bool {
//!         std::str::from_utf8(bytes).is_ok()
//!     }
//!     pub fn is_email(&self, s: &str) -> bool {
//!         s.contains('@')
//!     }
//! }
//! ```
//!
//! An agent might need to create a `Validator` instance to use these methods.
//! Even though there's only one possible value, elicitation provides a
//! type-safe way for agents to obtain it.
//!
//! # Generator Pattern
//!
//! ```rust,no_run
//! use elicitation::{Generator, Elicitation};
//!
//! // Unit struct with associated methods
//! #[derive(Debug, Clone, Copy)]
//! pub struct Formatter;
//!
//! impl Formatter {
//!     pub fn format_json(&self, s: &str) -> String { s.to_string() }
//! }
//!
//! // Generator is trivial - only one value exists!
//! // (Would be implemented automatically)
//! ```

use crate::{ElicitClient, ElicitResult, Elicitation, Generator, Prompt};

// ============================================================================
// Example Unit Structs
// ============================================================================

/// Validation helper - unit struct with validation methods.
///
/// This demonstrates a unit struct with associated methods. Even though
/// there's only one value, agents can elicit it to get a type-safe handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Validator;

impl Validator {
    /// Check if bytes are valid UTF-8.
    pub fn is_utf8(&self, bytes: &[u8]) -> bool {
        std::str::from_utf8(bytes).is_ok()
    }

    /// Check if string is non-empty.
    pub fn is_non_empty(&self, s: &str) -> bool {
        !s.is_empty()
    }

    /// Check if value is in range.
    pub fn is_in_range<T: PartialOrd>(&self, value: T, min: T, max: T) -> bool {
        value >= min && value <= max
    }
}

/// Formatting helper - unit struct with formatting methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Formatter;

impl Formatter {
    /// Format as uppercase.
    pub fn uppercase(&self, s: &str) -> String {
        s.to_uppercase()
    }

    /// Format as lowercase.
    pub fn lowercase(&self, s: &str) -> String {
        s.to_lowercase()
    }

    /// Trim whitespace.
    pub fn trim(&self, s: &str) -> String {
        s.trim().to_string()
    }
}

/// Parser helper - unit struct with parsing methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Parser;

impl Parser {
    /// Parse integer.
    pub fn parse_int(&self, s: &str) -> Result<i64, std::num::ParseIntError> {
        s.parse()
    }

    /// Parse float.
    pub fn parse_float(&self, s: &str) -> Result<f64, std::num::ParseFloatError> {
        s.parse()
    }

    /// Parse boolean.
    pub fn parse_bool(&self, s: &str) -> Result<bool, std::str::ParseBoolError> {
        s.parse()
    }
}

// ============================================================================
// Trivial Generator Implementation
// ============================================================================
//
// Unit structs have exactly one value - themselves!
// Generator is trivial: just return Self.

impl Generator for Validator {
    type Target = Self;

    fn generate(&self) -> Self::Target {
        Validator
    }
}

impl Generator for Formatter {
    type Target = Self;

    fn generate(&self) -> Self::Target {
        Formatter
    }
}

impl Generator for Parser {
    type Target = Self;

    fn generate(&self) -> Self::Target {
        Parser
    }
}

// ============================================================================
// Trivial Elicitation Implementation
// ============================================================================
//
// Since there's only one value, we don't need to ask the agent for input.
// We can just return the unit struct directly.
// For consistency, we still show a prompt confirming what's being created.

crate::default_style!(Validator => ValidatorStyle);
crate::default_style!(Formatter => FormatterStyle);
crate::default_style!(Parser => ParserStyle);

impl Prompt for Validator {
    fn prompt() -> Option<&'static str> {
        Some("Create a Validator instance (unit struct - only one value)")
    }
}

impl Elicitation for Validator {
    type Style = ValidatorStyle;

    async fn elicit(_client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Validator (unit struct)");

        // Unit struct - only one possible value!
        // No need to ask the agent for anything.
        Ok(Validator)
    }
}

impl Prompt for Formatter {
    fn prompt() -> Option<&'static str> {
        Some("Create a Formatter instance (unit struct - only one value)")
    }
}

impl Elicitation for Formatter {
    type Style = FormatterStyle;

    async fn elicit(_client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Formatter (unit struct)");
        Ok(Formatter)
    }
}

impl Prompt for Parser {
    fn prompt() -> Option<&'static str> {
        Some("Create a Parser instance (unit struct - only one value)")
    }
}

impl Elicitation for Parser {
    type Style = ParserStyle;

    async fn elicit(_client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Parser (unit struct)");
        Ok(Parser)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_methods() {
        let v = Validator;
        assert!(v.is_utf8(b"hello"));
        assert!(!v.is_utf8(b"\xFF\xFE"));
        assert!(v.is_non_empty("test"));
        assert!(!v.is_non_empty(""));
        assert!(v.is_in_range(5, 0, 10));
        assert!(!v.is_in_range(15, 0, 10));
    }

    #[test]
    fn test_formatter_methods() {
        let f = Formatter;
        assert_eq!(f.uppercase("hello"), "HELLO");
        assert_eq!(f.lowercase("WORLD"), "world");
        assert_eq!(f.trim("  test  "), "test");
    }

    #[test]
    fn test_parser_methods() {
        let p = Parser;
        assert_eq!(p.parse_int("42").unwrap(), 42);
        let result = p.parse_float("2.5").unwrap();
        assert!((result - 2.5_f64).abs() < 0.001);
        assert!(p.parse_bool("true").unwrap());
    }

    #[test]
    fn test_generator_trivial() {
        let v = Validator;
        assert_eq!(v.generate(), Validator);

        let f = Formatter;
        assert_eq!(f.generate(), Formatter);

        let p = Parser;
        assert_eq!(p.generate(), Parser);
    }
}
