//! Gallery level C46: manual label parsing via byte-slice equality.
//!
//! **Hypothesis**: Creusot rejects `match label { "Idle" => .. }` because of
//! the string-pattern shape; comparing bytes may preserve the same parsing
//! semantics while staying within a supported representation.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c46-label-bytes
//! ```

/// Tiny enum used to exercise byte-based label parsing.
#[cfg(feature = "gallery-c46-label-bytes")]
#[derive(Debug, Clone, Copy, Default)]
pub enum C46State {
    /// Initial state.
    #[default]
    Idle,
    /// Finished state.
    Done,
}

/// Manual `from_label` using byte-slice comparisons.
#[cfg(feature = "gallery-c46-label-bytes")]
pub fn c46_from_label(label: &str) -> Option<C46State> {
    if label.as_bytes() == b"Idle" {
        Some(C46State::Idle)
    } else if label.as_bytes() == b"Done" {
        Some(C46State::Done)
    } else {
        None
    }
}
