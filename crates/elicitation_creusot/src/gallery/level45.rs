//! Gallery level C45: manual string-label matching.
//!
//! **Hypothesis**: once `Vec` construction is no longer using `vec![..]`,
//! the next failing shape may be `match label { "Idle" => .. }` over `&str`.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c45-label-match
//! ```

/// Tiny enum used to exercise label parsing.
#[cfg(feature = "gallery-c45-label-match")]
#[derive(Debug, Clone, Copy, Default)]
pub enum C45State {
    /// Initial state.
    #[default]
    Idle,
    /// Finished state.
    Done,
}

/// Manual `from_label` shape matching the enum derive.
#[cfg(feature = "gallery-c45-label-match")]
pub fn c45_from_label(label: &str) -> Option<C45State> {
    match label {
        "Idle" => Some(C45State::Idle),
        "Done" => Some(C45State::Done),
        _ => None,
    }
}
