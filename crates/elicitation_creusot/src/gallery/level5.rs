//! Gallery level C5: data-carrying enum, invariant over payload.
//!
//! **Hypothesis**: Pearlite can express invariants over enum variants that carry
//! data (specifically `String` payloads), matching the shape of the real
//! `ArchivePanelState` / `ArchiveConnectionState` enums.
//!
//! The key question: can `#[logic]` functions `match` on an enum, access a
//! `String` field from a data variant, and apply a length predicate?
//!
//! ## Experiment table
//!
//! | ID   | What                                              | Expected |
//! |------|---------------------------------------------------|----------|
//! | C5a  | Unit variant always satisfies invariant           | ✓        |
//! | C5b  | Data variant satisfies invariant iff msg nonempty | ✓ or ✗   |
//! | C5c  | Transition from unit → data variant               | ✓ or ✗   |
//!
//! ## Run
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```


use creusot_std::prelude::*;

/// A mini connection state machine: Connected or Error(message).
///
/// Mirrors the shape of `ArchiveConnectionState::ConnectionError(String)`.

pub enum ConnState {
    /// No active error — trivially consistent.
    Connected,
    /// An error occurred; the message must be non-empty.
    Error(String),
}

/// C5 invariant: Connected is always consistent; Error only if message non-empty.
///
/// This mirrors what `archive_connection_consistent` should eventually express.

#[logic]
pub fn c5_consistent(s: &ConnState) -> bool {
    pearlite! {
        match s {
            ConnState::Connected => true,
            ConnState::Error(msg) => msg@.len() > 0,
        }
    }
}

/// C5a: Connected satisfies the invariant.

#[requires(true)]
#[ensures(c5_consistent(&result))]
pub fn c5_connect() -> ConnState {
    ConnState::Connected
}

/// C5b: entering the Error state requires a non-empty message.
///
/// Verifies: data-variant construction with payload precondition.

#[requires(msg@.len() > 0)]
#[ensures(c5_consistent(&result))]
pub fn c5_enter_error(msg: String) -> ConnState {
    ConnState::Error(msg)
}

/// C5c: identity on a consistent state preserves the invariant.

#[requires(c5_consistent(&s))]
#[ensures(c5_consistent(&result))]
pub fn c5_identity(s: ConnState) -> ConnState {
    s
}
