//! Gallery level C43: manual `Vec` construction via `Vec::new()` + `push`.
//!
//! **Hypothesis**: if `vec![..]` is the bad MIR shape, a hand-built `Vec`
//! using ordinary pushes may still be acceptable to Creusot while preserving
//! the same runtime data structure.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c43-vec-push
//! ```

/// Tiny enum used to exercise manual `Vec<Self>` construction.
#[cfg(feature = "gallery-c43-vec-push")]
#[derive(Debug, Clone, Copy, Default)]
pub enum C43State {
    /// Initial state.
    #[default]
    Idle,
    /// Finished state.
    Done,
}

/// Manual `Vec<Self>` construction without `vec![..]`.
#[cfg(feature = "gallery-c43-vec-push")]
pub fn c43_options_push() -> Vec<C43State> {
    let mut options = Vec::new();
    options.push(C43State::Idle);
    options.push(C43State::Done);
    options
}

/// Manual `Vec<String>` construction without `vec![..]`.
#[cfg(feature = "gallery-c43-vec-push")]
pub fn c43_labels_push() -> Vec<String> {
    let mut labels = Vec::new();
    labels.push("Idle".to_string());
    labels.push("Done".to_string());
    labels
}
