//! Kani proofs for regex validation (layered constraint verification).
//!
//! Each layer proves a specific constraint is validated, narrowing the symbolic space.

#![cfg(kani)]

use crate::verification::types::{
    RegexBytes, BalancedDelimiters, ValidEscapes, ValidQuantifiers, ValidCharClass,
};

// ============================================================================
// Layer 2: Balanced Delimiters Proofs
// ============================================================================

#[kani::proof]
#[kani::unwind(6)]  // "(abc)" = 5 bytes + 1
fn verify_balanced_simple() {
    const MAX_LEN: usize = 16;
    
    let bytes = b"(abc)";
    let result = BalancedDelimiters::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());
}

#[kani::proof]
#[kani::unwind(9)]  // "((a|b)c)" = 8 bytes + 1
fn verify_balanced_nested() {
    const MAX_LEN: usize = 16;
    
    let bytes = b"((a|b)c)";
    let result = BalancedDelimiters::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());
}

#[kani::proof]
#[kani::unwind(5)]  // "(abc" = 4 bytes + 1
fn verify_unbalanced_rejected() {
    const MAX_LEN: usize = 16;
    
    let bytes = b"(abc";
    let result = BalancedDelimiters::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_err());
}

#[kani::proof]
#[kani::unwind(8)]  // "[a-z]+" = 7 bytes + 1
fn verify_balanced_brackets() {
    const MAX_LEN: usize = 16;
    
    let bytes = b"[a-z]+";
    let result = BalancedDelimiters::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());
}

// ============================================================================
// Layer 3: Valid Escapes Proofs
// ============================================================================

#[kani::proof]
#[kani::unwind(4)]  // "\d+" = 3 bytes + 1
fn verify_escape_digit() {
    const MAX_LEN: usize = 16;
    
    let bytes = b"\\d+";
    let result = ValidEscapes::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());
}

#[kani::proof]
#[kani::unwind(4)]  // "\w*" = 3 bytes + 1
fn verify_escape_word() {
    const MAX_LEN: usize = 16;
    
    let bytes = b"\\w*";
    let result = ValidEscapes::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());
}

#[kani::proof]
#[kani::unwind(5)]  // "\." = 2 bytes + margin
fn verify_escape_dot() {
    const MAX_LEN: usize = 16;
    
    let bytes = b"\\.";
    let result = ValidEscapes::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());
}

#[kani::proof]
#[kani::unwind(3)]  // "\x" = 2 bytes + 1
fn verify_invalid_escape() {
    const MAX_LEN: usize = 16;
    
    let bytes = b"\\x";
    let result = ValidEscapes::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_err());
}

// ============================================================================
// Layer 4: Valid Quantifiers Proofs
// ============================================================================

#[kani::proof]
#[kani::unwind(3)]  // "a*" = 2 bytes + 1
fn verify_quantifier_star() {
    const MAX_LEN: usize = 16;
    
    let bytes = b"a*";
    let result = ValidQuantifiers::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());
}

#[kani::proof]
#[kani::unwind(3)]  // "b+" = 2 bytes + 1
fn verify_quantifier_plus() {
    const MAX_LEN: usize = 16;
    
    let bytes = b"b+";
    let result = ValidQuantifiers::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());
}

#[kani::proof]
#[kani::unwind(8)]  // "a{3,5}" = 7 bytes + 1
fn verify_quantifier_range() {
    const MAX_LEN: usize = 16;
    
    let bytes = b"a{3,5}";
    let result = ValidQuantifiers::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());
}

#[kani::proof]
#[kani::unwind(2)]  // "*" = 1 byte + 1
fn verify_quantifier_without_atom() {
    const MAX_LEN: usize = 16;
    
    let bytes = b"*";
    let result = ValidQuantifiers::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_err());
}

#[kani::proof]
#[kani::unwind(8)]  // "a{5,3}" = 7 bytes + 1
fn verify_quantifier_invalid_range() {
    const MAX_LEN: usize = 16;
    
    let bytes = b"a{5,3}";
    let result = ValidQuantifiers::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_err());
}

// ============================================================================
// Layer 5: Valid Character Class Proofs
// ============================================================================

#[kani::proof]
#[kani::unwind(6)]  // "[abc]" = 5 bytes + 1
fn verify_charclass_simple() {
    const MAX_LEN: usize = 16;
    
    let bytes = b"[abc]";
    let result = ValidCharClass::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());
}

#[kani::proof]
#[kani::unwind(6)]  // "[a-z]" = 5 bytes + 1
fn verify_charclass_range() {
    const MAX_LEN: usize = 16;
    
    let bytes = b"[a-z]";
    let result = ValidCharClass::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());
}

#[kani::proof]
#[kani::unwind(7)]  // "[^0-9]" = 6 bytes + 1
fn verify_charclass_negated() {
    const MAX_LEN: usize = 16;
    
    let bytes = b"[^0-9]";
    let result = ValidCharClass::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());
}

#[kani::proof]
#[kani::unwind(6)]  // "[z-a]" = 5 bytes + 1
fn verify_charclass_invalid_range() {
    const MAX_LEN: usize = 16;
    
    let bytes = b"[z-a]";
    let result = ValidCharClass::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_err());
}

// ============================================================================
// Layer 6: Complete Regex Proofs
// ============================================================================

#[kani::proof]
#[kani::unwind(6)]  // "hello" = 5 bytes + 1
fn verify_regex_literal() {
    const MAX_LEN: usize = 16;
    
    let bytes = b"hello";
    let result = RegexBytes::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());
    
    if let Ok(regex) = result {
        assert_eq!(regex.as_str(), "hello");
    }
}

#[kani::proof]
#[kani::unwind(10)]  // "\\d{2,4}" = 9 bytes + 1
fn verify_regex_digit_range() {
    const MAX_LEN: usize = 16;
    
    let bytes = b"\\d{2,4}";
    let result = RegexBytes::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());
}

#[kani::proof]
#[kani::unwind(12)]  // "^[a-z]+$" = 8 bytes + margin
fn verify_regex_anchored() {
    const MAX_LEN: usize = 16;
    
    let bytes = b"^[a-z]+$";
    let result = RegexBytes::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());
}

#[kani::proof]
#[kani::unwind(11)]  // "(\\d+|\\w+)" = 10 bytes + 1
fn verify_regex_alternation() {
    const MAX_LEN: usize = 16;
    
    let bytes = b"(\\d+|\\w+)";
    let result = RegexBytes::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());
}
