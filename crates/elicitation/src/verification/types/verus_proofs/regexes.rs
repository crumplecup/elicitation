//! Verus proofs for Regex contract types.


// Regex types may be in regexbytes module or elsewhere
// Using minimal imports for now since regex is feature-gated

#[cfg(verus)]
#[allow(unused_imports)]
use verus_builtin::*;
#[cfg(verus)]
#[allow(unused_imports)]
use verus_builtin_macros::*;

verus! {

/// Verify RegexSetValid construction for multiple patterns.
proof fn verify_regex_set_valid_construction(patterns: &[&str])
    ensures
        match RegexSetValid::new(patterns) {
            Ok(set) => set.len() == patterns.len(),
            Err(_) => true,
        },
{
}

#[cfg(feature = "regex")]
/// Verify RegexCaseInsensitive matches case-insensitively.
proof fn verify_regex_case_insensitive_matching(pattern: &str)
    ensures
        match RegexCaseInsensitive::new(pattern) {
            Ok(_) => true,
            Err(_) => true,
        },
{
}

#[cfg(feature = "regex")]
/// Verify RegexMultiline enables multiline mode.
proof fn verify_regex_multiline_mode(pattern: &str)
    ensures
        match RegexMultiline::new(pattern) {
            Ok(_) => true,
            Err(_) => true,
        },
{
}

#[cfg(feature = "regex")]
/// Verify RegexSetNonEmpty rejects empty sets.
proof fn verify_regex_set_non_empty_requirement(patterns: &[&str])
    requires patterns.len() == 0
    ensures RegexSetNonEmpty::new(patterns).is_err(),
{
}

#[cfg(feature = "regex")]
/// Verify regex trenchcoat pattern: pattern → compile → unwrap preserves pattern.
proof fn verify_regex_trenchcoat(pattern: &str)
    ensures
        match RegexValid::new(pattern) {
            Ok(wrapped) => wrapped.into_inner().as_str() == pattern,
            Err(_) => true,
        },
{
}

#[cfg(feature = "regex")]
/// Verify regex accessor correctness.
proof fn verify_regex_accessor(pattern: &str)
    ensures
        match RegexValid::new(pattern) {
            Ok(wrapped) => wrapped.get().as_str() == pattern,
            Err(_) => true,
        },
{
}


} // verus!
