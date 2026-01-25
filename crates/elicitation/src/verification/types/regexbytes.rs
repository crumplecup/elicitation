//! Regex validation with recursive trait bounds for tractable verification.
//!
//! Architecture:
//! ```text
//! Layer 1: Utf8Bytes           → Valid UTF-8 encoding
//! Layer 2: BalancedDelimiters  → Parentheses/brackets/braces balanced
//! Layer 3: ValidEscapes        → \n, \t, \d, \w, etc. valid
//! Layer 4: ValidQuantifiers    → *, +, ?, {n,m} follow atoms
//! Layer 5: ValidCharClass      → [...] contents correct
//! Layer 6: RegexBytes          → Complete regex
//! ```

use crate::verification::types::{Utf8Bytes, ValidationError};

// ============================================================================
// Layer 1: UTF-8 Foundation (Already proven)
// ============================================================================

// Reuse Utf8Bytes<MAX_LEN>

// ============================================================================
// Layer 2: Balanced Delimiters
// ============================================================================

/// Validates that delimiters are balanced.
///
/// Checks: ( == ), [ == ], { == }
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BalancedDelimiters<const MAX_LEN: usize> {
    utf8: Utf8Bytes<MAX_LEN>,
}

impl<const MAX_LEN: usize> BalancedDelimiters<MAX_LEN> {
    /// Validate balanced delimiters.
    pub fn from_slice(bytes: &[u8]) -> Result<Self, ValidationError> {
        let len = bytes.len();
        
        if len > MAX_LEN {
            return Err(ValidationError::TooLong {
                max: MAX_LEN,
                actual: len,
            });
        }
        
        // For Kani: bound the length
        #[cfg(kani)]
        kani::assume(len <= MAX_LEN);
        
        // Count delimiters
        let mut paren_count = 0i32;
        let mut bracket_count = 0i32;
        let mut brace_count = 0i32;
        
        let mut i = 0;
        while i < len {
            let b = bytes[i];
            
            match b {
                b'(' => paren_count += 1,
                b')' => paren_count -= 1,
                b'[' => bracket_count += 1,
                b']' => bracket_count -= 1,
                b'{' if i + 1 < len && bytes[i + 1].is_ascii_digit() => {
                    // Part of quantifier, not a group
                }
                b'{' => brace_count += 1,
                b'}' if !is_quantifier_end(bytes, i) => brace_count -= 1,
                b'\\' if i + 1 < len => {
                    i += 1; // Skip escaped character
                }
                _ => {}
            }
            
            // Negative counts = unbalanced
            if paren_count < 0 || bracket_count < 0 || brace_count < 0 {
                return Err(ValidationError::InvalidRegexSyntax);
            }
            
            i += 1;
        }
        
        // Must end balanced
        if paren_count != 0 || bracket_count != 0 || brace_count != 0 {
            return Err(ValidationError::InvalidRegexSyntax);
        }
        
        // Copy to fixed array and validate UTF-8
        let mut fixed = [0u8; MAX_LEN];
        fixed[..len].copy_from_slice(bytes);
        let utf8 = Utf8Bytes::new(fixed, len)?;
        
        Ok(Self { utf8 })
    }
    
    /// Get as string slice.
    pub fn as_str(&self) -> &str {
        self.utf8.as_str()
    }
}

/// Check if '}' is part of quantifier like {3,5}
fn is_quantifier_end(bytes: &[u8], pos: usize) -> bool {
    if pos == 0 {
        return false;
    }
    
    // Look backward for '{'
    let mut i = pos;
    while i > 0 {
        i -= 1;
        let b = bytes[i];
        
        if b == b'{' {
            // Check if everything between { and } is digits/comma
            let mut j = i + 1;
            while j < pos {
                if !bytes[j].is_ascii_digit() && bytes[j] != b',' {
                    return false;
                }
                j += 1;
            }
            return true;
        }
        
        if !b.is_ascii_digit() && b != b',' {
            return false;
        }
    }
    
    false
}

// ============================================================================
// Layer 3: Valid Escapes
// ============================================================================

/// Validates escape sequences.
///
/// Valid: \n, \t, \r, \d, \w, \s, \., \*, \\, etc.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValidEscapes<const MAX_LEN: usize> {
    balanced: BalancedDelimiters<MAX_LEN>,
}

impl<const MAX_LEN: usize> ValidEscapes<MAX_LEN> {
    /// Validate escape sequences.
    pub fn from_slice(bytes: &[u8]) -> Result<Self, ValidationError> {
        let len = bytes.len();
        
        if len > MAX_LEN {
            return Err(ValidationError::TooLong {
                max: MAX_LEN,
                actual: len,
            });
        }
        
        #[cfg(kani)]
        kani::assume(len <= MAX_LEN);
        
        // Check each escape
        let mut i = 0;
        while i < len {
            if bytes[i] == b'\\' {
                if i + 1 >= len {
                    return Err(ValidationError::InvalidRegexSyntax);
                }
                
                let escaped = bytes[i + 1];
                if !is_valid_escape(escaped) {
                    return Err(ValidationError::InvalidRegexSyntax);
                }
                
                i += 2; // Skip escape sequence
            } else {
                i += 1;
            }
        }
        
        // Validate balanced delimiters
        let balanced = BalancedDelimiters::from_slice(bytes)?;
        
        Ok(Self { balanced })
    }
    
    /// Get as string slice.
    pub fn as_str(&self) -> &str {
        self.balanced.as_str()
    }
}

/// Check if character is valid after backslash.
fn is_valid_escape(b: u8) -> bool {
    matches!(
        b,
        b'n' | b't' | b'r' | b'd' | b'D' | b'w' | b'W' | b's' | b'S' |
        b'.' | b'*' | b'+' | b'?' | b'(' | b')' | b'[' | b']' | b'{' | b'}' |
        b'^' | b'$' | b'|' | b'\\' | b'0'..=b'9'
    )
}

// ============================================================================
// Layer 4: Valid Quantifiers
// ============================================================================

/// Validates quantifiers follow atoms.
///
/// Valid: a*, (ab)+, [xyz]?, x{3,5}
/// Invalid: **, +?, {3,2}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValidQuantifiers<const MAX_LEN: usize> {
    escapes: ValidEscapes<MAX_LEN>,
}

impl<const MAX_LEN: usize> ValidQuantifiers<MAX_LEN> {
    /// Validate quantifiers.
    pub fn from_slice(bytes: &[u8]) -> Result<Self, ValidationError> {
        let len = bytes.len();
        
        if len > MAX_LEN {
            return Err(ValidationError::TooLong {
                max: MAX_LEN,
                actual: len,
            });
        }
        
        #[cfg(kani)]
        kani::assume(len <= MAX_LEN);
        
        let mut i = 0;
        let mut has_atom = false;
        
        while i < len {
            let b = bytes[i];
            
            match b {
                b'*' | b'+' | b'?' => {
                    if !has_atom {
                        return Err(ValidationError::InvalidRegexSyntax);
                    }
                    has_atom = false; // Quantifier consumes atom
                }
                b'{' if i + 1 < len && bytes[i + 1].is_ascii_digit() => {
                    if !has_atom {
                        return Err(ValidationError::InvalidRegexSyntax);
                    }
                    
                    // Validate {n,m} format
                    let start = i;
                    i += 1;
                    while i < len && (bytes[i].is_ascii_digit() || bytes[i] == b',') {
                        i += 1;
                    }
                    
                    if i >= len || bytes[i] != b'}' {
                        return Err(ValidationError::InvalidRegexSyntax);
                    }
                    
                    // Check n <= m if comma present
                    if !validate_quantifier_range(bytes, start, i) {
                        return Err(ValidationError::InvalidRegexSyntax);
                    }
                    
                    has_atom = false;
                }
                b'\\' if i + 1 < len => {
                    i += 1; // Skip escape
                    has_atom = true;
                }
                b'(' | b'[' => {
                    // Groups are atoms
                    has_atom = true;
                }
                b')' | b']' => {
                    // Closing groups
                    has_atom = true;
                }
                b'^' | b'$' | b'|' => {
                    // Anchors/alternation reset atom state
                    has_atom = false;
                }
                _ => {
                    has_atom = true;
                }
            }
            
            i += 1;
        }
        
        // Validate escapes
        let escapes = ValidEscapes::from_slice(bytes)?;
        
        Ok(Self { escapes })
    }
    
    /// Get as string slice.
    pub fn as_str(&self) -> &str {
        self.escapes.as_str()
    }
}

/// Validate {n,m} has n <= m
fn validate_quantifier_range(bytes: &[u8], start: usize, end: usize) -> bool {
    let mut i = start + 1;
    let mut n = 0u32;
    
    // Parse n
    while i < end && bytes[i].is_ascii_digit() {
        n = n * 10 + (bytes[i] - b'0') as u32;
        i += 1;
    }
    
    // No comma = {n} exact match, always valid
    if i >= end || bytes[i] != b',' {
        return true;
    }
    
    i += 1; // Skip comma
    
    // No digits after comma = {n,} unbounded, always valid
    if i >= end || !bytes[i].is_ascii_digit() {
        return true;
    }
    
    // Parse m
    let mut m = 0u32;
    while i < end && bytes[i].is_ascii_digit() {
        m = m * 10 + (bytes[i] - b'0') as u32;
        i += 1;
    }
    
    n <= m
}

// ============================================================================
// Layer 5: Valid Character Classes
// ============================================================================

/// Validates character class contents.
///
/// Valid: [abc], [a-z], [^0-9], [\d\w]
/// Invalid: [z-a], [[]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValidCharClass<const MAX_LEN: usize> {
    quantifiers: ValidQuantifiers<MAX_LEN>,
}

impl<const MAX_LEN: usize> ValidCharClass<MAX_LEN> {
    /// Validate character classes.
    pub fn from_slice(bytes: &[u8]) -> Result<Self, ValidationError> {
        let len = bytes.len();
        
        if len > MAX_LEN {
            return Err(ValidationError::TooLong {
                max: MAX_LEN,
                actual: len,
            });
        }
        
        #[cfg(kani)]
        kani::assume(len <= MAX_LEN);
        
        let mut i = 0;
        while i < len {
            if bytes[i] == b'[' {
                i += 1;
                
                // Skip optional negation
                if i < len && bytes[i] == b'^' {
                    i += 1;
                }
                
                // Must have at least one character
                if i >= len || bytes[i] == b']' {
                    return Err(ValidationError::InvalidRegexSyntax);
                }
                
                // Validate contents
                while i < len && bytes[i] != b']' {
                    if bytes[i] == b'\\' {
                        i += 2; // Skip escape
                    } else if i + 2 < len && bytes[i + 1] == b'-' && bytes[i + 2] != b']' {
                        // Range: a-z
                        let start = bytes[i];
                        let end = bytes[i + 2];
                        
                        if start > end {
                            return Err(ValidationError::InvalidRegexSyntax);
                        }
                        
                        i += 3;
                    } else {
                        i += 1;
                    }
                }
                
                // Must have closing bracket
                if i >= len {
                    return Err(ValidationError::InvalidRegexSyntax);
                }
            }
            
            i += 1;
        }
        
        // Validate quantifiers
        let quantifiers = ValidQuantifiers::from_slice(bytes)?;
        
        Ok(Self { quantifiers })
    }
    
    /// Get as string slice.
    pub fn as_str(&self) -> &str {
        self.quantifiers.as_str()
    }
}

// ============================================================================
// Layer 6: Complete Regex
// ============================================================================

/// Validated regex bytes (all constraints satisfied).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegexBytes<const MAX_LEN: usize> {
    charclass: ValidCharClass<MAX_LEN>,
}

impl<const MAX_LEN: usize> RegexBytes<MAX_LEN> {
    /// Create validated regex from slice.
    ///
    /// Validates all regex syntax constraints through layered validation.
    pub fn from_slice(bytes: &[u8]) -> Result<Self, ValidationError> {
        let charclass = ValidCharClass::from_slice(bytes)?;
        Ok(Self { charclass })
    }
    
    /// Create from Vec (user-facing API).
    pub fn new(bytes: Vec<u8>) -> Result<Self, ValidationError> {
        Self::from_slice(&bytes)
    }
    
    /// Get regex as string slice.
    pub fn as_str(&self) -> &str {
        self.charclass.as_str()
    }
    
    /// Get regex as bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.charclass.as_str().as_bytes()
    }
    
    /// Get length in bytes.
    pub fn len(&self) -> usize {
        self.charclass.as_str().len()
    }
    
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_balanced_delimiters_valid() {
        let result = BalancedDelimiters::<32>::from_slice(b"(abc)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_balanced_delimiters_nested() {
        let result = BalancedDelimiters::<32>::from_slice(b"((a|b)c)");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_balanced_delimiters_unbalanced() {
        let result = BalancedDelimiters::<32>::from_slice(b"(abc");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_valid_escapes() {
        let result = ValidEscapes::<32>::from_slice(b"\\d+");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_invalid_escape() {
        let result = ValidEscapes::<32>::from_slice(b"\\x");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_quantifier_after_atom() {
        let result = ValidQuantifiers::<32>::from_slice(b"a*");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_quantifier_without_atom() {
        let result = ValidQuantifiers::<32>::from_slice(b"*");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_quantifier_range_valid() {
        let result = ValidQuantifiers::<32>::from_slice(b"a{3,5}");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_quantifier_range_invalid() {
        let result = ValidQuantifiers::<32>::from_slice(b"a{5,3}");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_charclass_simple() {
        let result = ValidCharClass::<32>::from_slice(b"[abc]");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_charclass_range() {
        let result = ValidCharClass::<32>::from_slice(b"[a-z]");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_charclass_invalid_range() {
        let result = ValidCharClass::<32>::from_slice(b"[z-a]");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_regex_simple() {
        let result = RegexBytes::<32>::from_slice(b"hello");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "hello");
    }
    
    #[test]
    fn test_regex_complex() {
        let result = RegexBytes::<64>::from_slice(b"^[a-zA-Z0-9]+@[a-z]+\\.[a-z]{2,4}$");
        assert!(result.is_ok());
    }
}
