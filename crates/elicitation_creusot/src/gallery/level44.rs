//! Gallery level C44: `Vec` construction from a fixed array.
//!
//! **Hypothesis**: if the problem is specifically the `vec![..]` macro
//! expansion, converting a fixed array into a `Vec` may preserve the same
//! API shape while producing different MIR.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c44-vec-from-array
//! ```

/// Tiny enum used to exercise array-to-`Vec<Self>` conversion.
#[cfg(feature = "gallery-c44-vec-from-array")]
#[derive(Debug, Clone, Copy, Default)]
pub enum C44State {
    /// Initial state.
    #[default]
    Idle,
    /// Finished state.
    Done,
}

/// Build `Vec<Self>` from a fixed array.
#[cfg(feature = "gallery-c44-vec-from-array")]
pub fn c44_options_from_array() -> Vec<C44State> {
    Vec::from([C44State::Idle, C44State::Done])
}

/// Build `Vec<String>` from a fixed array.
#[cfg(feature = "gallery-c44-vec-from-array")]
pub fn c44_labels_from_array() -> Vec<String> {
    Vec::from(["Idle".to_string(), "Done".to_string()])
}
