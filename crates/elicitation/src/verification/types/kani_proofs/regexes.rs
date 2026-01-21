//! Kani proofs for Regex contract types.

#[cfg(feature = "regex")]
use crate::{RegexValid, RegexSetValid, RegexCaseInsensitive, RegexMultiline, RegexSetNonEmpty};

// ============================================================================
// Regex Contract Proofs
// ============================================================================

#[cfg(feature = "regex")]
#[kani::proof]
#[kani::unwind(1)]
fn verify_regex_valid() {
    // Test valid regex patterns
    kani::assert(
        RegexValid::new(r"\d+").is_ok(),
        "Valid digit pattern compiles"
    );
    kani::assert(
        RegexValid::new(r"[a-z]+").is_ok(),
        "Valid character class compiles"
    );
    
    // Test invalid patterns
    kani::assert(
        RegexValid::new(r"[unclosed").is_err(),
        "Unclosed bracket rejected"
    );
}

#[cfg(feature = "regex")]
#[kani::proof]
#[kani::unwind(1)]
fn verify_regex_set_valid() {
    // Test valid regex set
    let set = RegexSetValid::new(&[r"\d+", r"[a-z]+"]);
    kani::assert(set.is_ok(), "Valid patterns compile");
    
    if let Ok(s) = set {
        kani::assert(s.len() == 2, "Set contains 2 patterns");
        kani::assert(!s.is_empty(), "Set is not empty");
    }
}

#[cfg(feature = "regex")]
#[kani::proof]
#[kani::unwind(1)]
fn verify_regex_case_insensitive() {
    let re = RegexCaseInsensitive::new(r"hello");
    kani::assert(re.is_ok(), "Case-insensitive pattern compiles");
    
    if let Ok(regex) = re {
        kani::assert(regex.is_match("hello"), "Matches lowercase");
        kani::assert(regex.is_match("HELLO"), "Matches uppercase");
        kani::assert(regex.is_match("HeLLo"), "Matches mixed case");
    }
}

#[cfg(feature = "regex")]
#[kani::proof]
#[kani::unwind(1)]
fn verify_regex_multiline() {
    let re = RegexMultiline::new(r"^test$");
    kani::assert(re.is_ok(), "Multiline pattern compiles");
    
    if let Ok(regex) = re {
        kani::assert(regex.is_match("test"), "Matches single line");
    }
}

#[cfg(feature = "regex")]
#[kani::proof]
#[kani::unwind(1)]
fn verify_regex_set_non_empty() {
    // Test non-empty set
    kani::assert(
        RegexSetNonEmpty::new(&[r"\d+"]).is_ok(),
        "Single pattern accepted"
    );
    
    // Test empty set rejection
    kani::assert(
        RegexSetNonEmpty::new::<&[&str], _>(&[]).is_err(),
        "Empty set rejected"
    );
}

#[cfg(feature = "regex")]
#[kani::proof]
#[kani::unwind(1)]
fn verify_regex_trenchcoat_pattern() {
    // Prove trenchcoat pattern: pattern → compile → use
    let pattern = r"\d{3}-\d{4}";
    
    if let Ok(wrapped) = RegexValid::new(pattern) {
        let unwrapped = wrapped.into_inner();
        
        // Trenchcoat: Pattern preserved through wrap/unwrap
        kani::assert(
            unwrapped.as_str() == pattern,
            "Pattern preserved through trenchcoat"
        );
        kani::assert(
            unwrapped.is_match("123-4567"),
            "Regex still functions after unwrap"
        );
    }
}

#[cfg(feature = "regex")]
#[kani::proof]
#[kani::unwind(1)]
fn verify_regex_accessor_correctness() {
    let pattern = r"\d+";
    if let Ok(wrapped) = RegexValid::new(pattern) {
        // Accessor preserves pattern
        kani::assert(
            wrapped.get().as_str() == pattern,
            "Accessor returns correct pattern"
        );
        kani::assert(
            wrapped.as_str() == pattern,
            "as_str() returns correct pattern"
        );
    }
}

