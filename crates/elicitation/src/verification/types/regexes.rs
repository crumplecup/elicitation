//! Regex contract types for formal verification.
//!
//! This module provides contract types for regex validation using the `regex` crate.

#![cfg(feature = "regex")]

use crate::verification::types::ValidationError;
#[cfg(feature = "regex")]
use regex::{Regex, RegexBuilder, RegexSet};

// ============================================================================
// Regex Contract Types
// ============================================================================

/// A valid, compiled regex pattern.
///
/// This contract ensures the regex pattern compiles successfully according
/// to the regex crate's syntax rules.
///
/// # Kani Verification
///
/// In Kani mode, uses PhantomData and symbolic validation. Trusts regex crate's
/// compilation logic, verifies only wrapper invariants.
#[cfg(not(kani))]
#[derive(Debug, Clone)]
pub struct RegexValid(Regex);

#[cfg(kani)]
#[derive(Debug, Clone)]
pub struct RegexValid(std::marker::PhantomData<Regex>);

#[cfg(not(kani))]
impl RegexValid {
    /// Create a new RegexValid from a pattern string.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::RegexInvalid` if the pattern cannot be compiled.
    #[cfg_attr(not(kani), tracing::instrument(err))]
    pub fn new(pattern: &str) -> Result<Self, ValidationError> {
        Regex::new(pattern)
            .map(Self)
            .map_err(|_| ValidationError::RegexInvalid)
    }

    /// Create a new RegexValid from an existing Regex.
    #[cfg_attr(not(kani), tracing::instrument)]
    pub fn from_regex(regex: Regex) -> Self {
        Self(regex)
    }

    /// Get a reference to the wrapped Regex.
    #[cfg_attr(not(kani), tracing::instrument(skip(self), level = "trace", ret))]
    pub fn get(&self) -> &Regex {
        &self.0
    }

    /// Unwrap the Regex.
    #[cfg_attr(not(kani), tracing::instrument(skip(self), level = "trace"))]
    pub fn into_inner(self) -> Regex {
        self.0
    }

    /// Returns true if the regex matches the given text.
    #[cfg_attr(not(kani), tracing::instrument(skip(self)))]
    pub fn is_match(&self, text: &str) -> bool {
        self.0.is_match(text)
    }

    /// Returns the original pattern string.
    #[cfg_attr(not(kani), tracing::instrument(skip(self), level = "trace", ret))]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(kani)]
impl RegexValid {
    /// Create a new RegexValid from a pattern string (Kani mode).
    ///
    /// Uses symbolic boolean to verify wrapper logic without regex compilation.
    pub fn new(_pattern: &str) -> Result<Self, ValidationError> {
        let is_valid: bool = kani::any();
        if is_valid {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::RegexInvalid)
        }
    }

    /// Create a new RegexValid from an existing Regex (Kani mode).
    pub fn from_regex(_regex: Regex) -> Self {
        Self(std::marker::PhantomData)
    }

    /// Get a reference to the wrapped Regex (not available in Kani mode).
    pub fn get(&self) -> &Regex {
        panic!("RegexValid::get() not available in Kani mode - use symbolic validation")
    }

    /// Unwrap the Regex (not available in Kani mode).
    pub fn into_inner(self) -> Regex {
        panic!("RegexValid::into_inner() not available in Kani mode - use symbolic validation")
    }

    /// Returns true if the regex matches the given text (Kani mode).
    pub fn is_match(&self, _text: &str) -> bool {
        kani::any()
    }

    /// Returns the original pattern string (not available in Kani mode).
    pub fn as_str(&self) -> &str {
        panic!("RegexValid::as_str() not available in Kani mode - use symbolic validation")
    }
}

/// A valid, compiled regex pattern set.
///
/// This contract ensures multiple regex patterns compile successfully
/// and can be used for efficient multi-pattern matching.
///
/// # Kani Verification
///
/// In Kani mode, uses PhantomData and symbolic validation. Trusts regex crate's
/// compilation logic, verifies only wrapper invariants.
#[cfg(not(kani))]
#[derive(Debug, Clone)]
pub struct RegexSetValid(RegexSet);

#[cfg(kani)]
#[derive(Debug, Clone)]
pub struct RegexSetValid(std::marker::PhantomData<RegexSet>);

#[cfg(not(kani))]
impl RegexSetValid {
    /// Create a new RegexSetValid from pattern strings.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::RegexInvalid` if any pattern cannot be compiled.
    #[cfg_attr(not(kani), tracing::instrument(skip(patterns), err))]
    pub fn new<I, S>(patterns: I) -> Result<Self, ValidationError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        RegexSet::new(patterns)
            .map(Self)
            .map_err(|_| ValidationError::RegexInvalid)
    }

    /// Create a new RegexSetValid from an existing RegexSet.
    #[cfg_attr(not(kani), tracing::instrument)]
    pub fn from_regex_set(regex_set: RegexSet) -> Self {
        Self(regex_set)
    }

    /// Get a reference to the wrapped RegexSet.
    #[cfg_attr(not(kani), tracing::instrument(skip(self), level = "trace", ret))]
    pub fn get(&self) -> &RegexSet {
        &self.0
    }

    /// Unwrap the RegexSet.
    #[cfg_attr(not(kani), tracing::instrument(skip(self), level = "trace"))]
    pub fn into_inner(self) -> RegexSet {
        self.0
    }

    /// Returns true if any pattern in the set matches the given text.
    #[cfg_attr(not(kani), tracing::instrument(skip(self)))]
    pub fn is_match(&self, text: &str) -> bool {
        self.0.is_match(text)
    }

    /// Returns the number of patterns in the set.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the set contains no patterns.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(kani)]
impl RegexSetValid {
    /// Create a new RegexSetValid from pattern strings (Kani mode).
    ///
    /// Uses symbolic boolean to verify wrapper logic without regex compilation.
    pub fn new<I, S>(_patterns: I) -> Result<Self, ValidationError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let is_valid: bool = kani::any();
        if is_valid {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::RegexInvalid)
        }
    }

    /// Create a new RegexSetValid from an existing RegexSet (Kani mode).
    pub fn from_regex_set(_regex_set: RegexSet) -> Self {
        Self(std::marker::PhantomData)
    }

    /// Get a reference to the wrapped RegexSet (not available in Kani mode).
    pub fn get(&self) -> &RegexSet {
        panic!("RegexSetValid::get() not available in Kani mode - use symbolic validation")
    }

    /// Unwrap the RegexSet (not available in Kani mode).
    pub fn into_inner(self) -> RegexSet {
        panic!("RegexSetValid::into_inner() not available in Kani mode - use symbolic validation")
    }

    /// Returns true if any pattern in the set matches the given text (Kani mode).
    pub fn is_match(&self, _text: &str) -> bool {
        kani::any()
    }

    /// Returns the number of patterns in the set (Kani mode).
    pub fn len(&self) -> usize {
        kani::any()
    }

    /// Returns true if the set contains no patterns (Kani mode).
    pub fn is_empty(&self) -> bool {
        kani::any()
    }
}

/// A case-insensitive regex pattern.
///
/// This contract ensures the regex is compiled with case-insensitive matching.
///
/// # Kani Verification
///
/// In Kani mode, uses PhantomData and symbolic validation. Trusts regex crate's
/// compilation logic, verifies only wrapper invariants.
#[cfg(not(kani))]
#[derive(Debug, Clone)]
pub struct RegexCaseInsensitive(Regex);

#[cfg(kani)]
#[derive(Debug, Clone)]
pub struct RegexCaseInsensitive(std::marker::PhantomData<Regex>);

#[cfg(not(kani))]
impl RegexCaseInsensitive {
    /// Create a new case-insensitive regex from a pattern string.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::RegexInvalid` if the pattern cannot be compiled.
    #[cfg_attr(not(kani), tracing::instrument(err))]
    pub fn new(pattern: &str) -> Result<Self, ValidationError> {
        RegexBuilder::new(pattern)
            .case_insensitive(true)
            .build()
            .map(Self)
            .map_err(|_| ValidationError::RegexInvalid)
    }

    /// Get a reference to the wrapped Regex.
    #[cfg_attr(not(kani), tracing::instrument(skip(self), level = "trace", ret))]
    pub fn get(&self) -> &Regex {
        &self.0
    }

    /// Unwrap the Regex.
    #[cfg_attr(not(kani), tracing::instrument(skip(self), level = "trace"))]
    pub fn into_inner(self) -> Regex {
        self.0
    }

    /// Returns true if the regex matches the given text.
    #[cfg_attr(not(kani), tracing::instrument(skip(self)))]
    pub fn is_match(&self, text: &str) -> bool {
        self.0.is_match(text)
    }

    /// Returns the original pattern string.
    #[cfg_attr(not(kani), tracing::instrument(skip(self), level = "trace", ret))]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(kani)]
impl RegexCaseInsensitive {
    /// Create a new case-insensitive regex from a pattern string (Kani mode).
    ///
    /// Uses symbolic boolean to verify wrapper logic without regex compilation.
    pub fn new(_pattern: &str) -> Result<Self, ValidationError> {
        let is_valid: bool = kani::any();
        if is_valid {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::RegexInvalid)
        }
    }

    /// Get a reference to the wrapped Regex (not available in Kani mode).
    pub fn get(&self) -> &Regex {
        panic!("RegexCaseInsensitive::get() not available in Kani mode - use symbolic validation")
    }

    /// Unwrap the Regex (not available in Kani mode).
    pub fn into_inner(self) -> Regex {
        panic!("RegexCaseInsensitive::into_inner() not available in Kani mode - use symbolic validation")
    }

    /// Returns true if the regex matches the given text (Kani mode).
    pub fn is_match(&self, _text: &str) -> bool {
        kani::any()
    }

    /// Returns the original pattern string (not available in Kani mode).
    pub fn as_str(&self) -> &str {
        panic!("RegexCaseInsensitive::as_str() not available in Kani mode - use symbolic validation")
    }
}

/// A multiline regex pattern.
///
/// This contract ensures the regex is compiled with multiline mode enabled,
/// where ^ and $ match line boundaries.
///
/// # Kani Verification
///
/// In Kani mode, uses PhantomData and symbolic validation. Trusts regex crate's
/// compilation logic, verifies only wrapper invariants.
#[cfg(not(kani))]
#[derive(Debug, Clone)]
pub struct RegexMultiline(Regex);

#[cfg(kani)]
#[derive(Debug, Clone)]
pub struct RegexMultiline(std::marker::PhantomData<Regex>);

#[cfg(not(kani))]
impl RegexMultiline {
    /// Create a new multiline regex from a pattern string.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::RegexInvalid` if the pattern cannot be compiled.
    #[cfg_attr(not(kani), tracing::instrument(err))]
    pub fn new(pattern: &str) -> Result<Self, ValidationError> {
        RegexBuilder::new(pattern)
            .multi_line(true)
            .build()
            .map(Self)
            .map_err(|_| ValidationError::RegexInvalid)
    }

    /// Get a reference to the wrapped Regex.
    #[cfg_attr(not(kani), tracing::instrument(skip(self), level = "trace", ret))]
    pub fn get(&self) -> &Regex {
        &self.0
    }

    /// Unwrap the Regex.
    #[cfg_attr(not(kani), tracing::instrument(skip(self), level = "trace"))]
    pub fn into_inner(self) -> Regex {
        self.0
    }

    /// Returns true if the regex matches the given text.
    #[cfg_attr(not(kani), tracing::instrument(skip(self)))]
    pub fn is_match(&self, text: &str) -> bool {
        self.0.is_match(text)
    }

    /// Returns the original pattern string.
    #[cfg_attr(not(kani), tracing::instrument(skip(self), level = "trace", ret))]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(kani)]
impl RegexMultiline {
    /// Create a new multiline regex from a pattern string (Kani mode).
    ///
    /// Uses symbolic boolean to verify wrapper logic without regex compilation.
    pub fn new(_pattern: &str) -> Result<Self, ValidationError> {
        let is_valid: bool = kani::any();
        if is_valid {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::RegexInvalid)
        }
    }

    /// Get a reference to the wrapped Regex (not available in Kani mode).
    pub fn get(&self) -> &Regex {
        panic!("RegexMultiline::get() not available in Kani mode - use symbolic validation")
    }

    /// Unwrap the Regex (not available in Kani mode).
    pub fn into_inner(self) -> Regex {
        panic!("RegexMultiline::into_inner() not available in Kani mode - use symbolic validation")
    }

    /// Returns true if the regex matches the given text (Kani mode).
    pub fn is_match(&self, _text: &str) -> bool {
        kani::any()
    }

    /// Returns the original pattern string (not available in Kani mode).
    pub fn as_str(&self) -> &str {
        panic!("RegexMultiline::as_str() not available in Kani mode - use symbolic validation")
    }
}

/// A non-empty regex set.
///
/// This contract ensures the regex set contains at least one pattern.
///
/// # Kani Verification
///
/// In Kani mode, uses PhantomData and symbolic validation. Trusts regex crate's
/// compilation logic, verifies only wrapper invariants.
#[cfg(not(kani))]
#[derive(Debug, Clone)]
pub struct RegexSetNonEmpty(RegexSet);

#[cfg(kani)]
#[derive(Debug, Clone)]
pub struct RegexSetNonEmpty(std::marker::PhantomData<RegexSet>);

#[cfg(not(kani))]
impl RegexSetNonEmpty {
    /// Create a new non-empty regex set from pattern strings.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::RegexInvalid` if any pattern cannot be compiled.
    /// Returns `ValidationError::EmptyCollection` if no patterns are provided.
    #[cfg_attr(not(kani), tracing::instrument(skip(patterns), err))]
    pub fn new<I, S>(patterns: I) -> Result<Self, ValidationError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let patterns: Vec<_> = patterns.into_iter().collect();

        if patterns.is_empty() {
            return Err(ValidationError::EmptyCollection);
        }

        RegexSet::new(patterns)
            .map(Self)
            .map_err(|_| ValidationError::RegexInvalid)
    }

    /// Get a reference to the wrapped RegexSet.
    #[cfg_attr(not(kani), tracing::instrument(skip(self), level = "trace", ret))]
    pub fn get(&self) -> &RegexSet {
        &self.0
    }

    /// Unwrap the RegexSet.
    #[cfg_attr(not(kani), tracing::instrument(skip(self), level = "trace"))]
    pub fn into_inner(self) -> RegexSet {
        self.0
    }

    /// Returns true if any pattern in the set matches the given text.
    #[cfg_attr(not(kani), tracing::instrument(skip(self)))]
    pub fn is_match(&self, text: &str) -> bool {
        self.0.is_match(text)
    }

    /// Returns the number of patterns in the set (always > 0).
    #[cfg_attr(not(kani), tracing::instrument(skip(self), level = "trace", ret))]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(kani)]
impl RegexSetNonEmpty {
    /// Create a new non-empty regex set from pattern strings (Kani mode).
    ///
    /// Uses symbolic boolean to verify wrapper logic without regex compilation.
    pub fn new<I, S>(_patterns: I) -> Result<Self, ValidationError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let is_valid: bool = kani::any();
        let is_empty: bool = kani::any();
        
        if is_empty {
            Err(ValidationError::EmptyCollection)
        } else if is_valid {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::RegexInvalid)
        }
    }

    /// Get a reference to the wrapped RegexSet (not available in Kani mode).
    pub fn get(&self) -> &RegexSet {
        panic!("RegexSetNonEmpty::get() not available in Kani mode - use symbolic validation")
    }

    /// Unwrap the RegexSet (not available in Kani mode).
    pub fn into_inner(self) -> RegexSet {
        panic!("RegexSetNonEmpty::into_inner() not available in Kani mode - use symbolic validation")
    }

    /// Returns true if any pattern in the set matches the given text (Kani mode).
    pub fn is_match(&self, _text: &str) -> bool {
        kani::any()
    }

    /// Returns the number of patterns in the set (always > 0, Kani mode).
    pub fn len(&self) -> usize {
        let len: usize = kani::any();
        kani::assume(len > 0);
        len
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_valid() {
        // Valid patterns
        assert!(RegexValid::new(r"\d+").is_ok());
        assert!(RegexValid::new(r"[a-z]+").is_ok());
        assert!(RegexValid::new(r"hello|world").is_ok());

        // Invalid patterns
        assert!(RegexValid::new(r"[unclosed").is_err());
        assert!(RegexValid::new(r"(?P<").is_err());
    }

    #[test]
    fn test_regex_valid_matching() {
        let re = RegexValid::new(r"\d{3}-\d{3}-\d{4}").unwrap();

        assert!(re.is_match("555-123-4567"));
        assert!(!re.is_match("not a phone"));
        assert_eq!(re.as_str(), r"\d{3}-\d{3}-\d{4}");
    }

    #[test]
    fn test_regex_set_valid() {
        let set = RegexSetValid::new(&[r"\d+", r"[a-z]+", r"[A-Z]+"]).unwrap();

        assert_eq!(set.len(), 3);
        assert!(!set.is_empty());
        assert!(set.is_match("123"));
        assert!(set.is_match("abc"));
        assert!(set.is_match("ABC"));
        assert!(!set.is_match("!!!"));
    }

    #[test]
    fn test_regex_set_empty() {
        let set = RegexSetValid::new::<&[&str], _>(&[]).unwrap();
        assert_eq!(set.len(), 0);
        assert!(set.is_empty());
    }

    #[test]
    fn test_regex_case_insensitive() {
        let re = RegexCaseInsensitive::new(r"hello").unwrap();

        assert!(re.is_match("hello"));
        assert!(re.is_match("HELLO"));
        assert!(re.is_match("HeLLo"));
        assert!(!re.is_match("goodbye"));
    }

    #[test]
    fn test_regex_multiline() {
        let re = RegexMultiline::new(r"^test$").unwrap();

        // Multiline mode: ^ and $ match line boundaries
        assert!(re.is_match("test"));
        assert!(re.is_match("test\nmore"));
        assert!(re.is_match("before\ntest"));
    }

    #[test]
    fn test_regex_set_non_empty() {
        assert!(RegexSetNonEmpty::new(&[r"\d+"]).is_ok());
        assert!(RegexSetNonEmpty::new(&[r"\d+", r"[a-z]+"]).is_ok());

        // Empty set rejected
        assert!(RegexSetNonEmpty::new::<&[&str], _>(&[]).is_err());
    }

    #[test]
    fn test_regex_trenchcoat_pattern() {
        let pattern = r"\d{3}-\d{4}";
        let wrapped = RegexValid::new(pattern).unwrap();
        let unwrapped = wrapped.into_inner();

        assert_eq!(unwrapped.as_str(), pattern);
        assert!(unwrapped.is_match("123-4567"));
    }
}
