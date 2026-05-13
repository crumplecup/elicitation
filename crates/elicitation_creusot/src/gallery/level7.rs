//! Gallery level C7: named struct fields in enum variants.
//!
//! **Hypothesis**: Pearlite can `match` on enum variants with *named* struct
//! fields and access those fields in `#[logic]` predicates.  This is the
//! direct prerequisite for expressing the `ArchiveConnectionState::Connecting`
//! invariant (`profile_name` must be non-empty, `backend` must be a valid kind).
//!
//! ## Experiment table
//!
//! | ID   | What                                                    | Expected |
//! |------|---------------------------------------------------------|----------|
//! | C7a  | `match` accesses `String` named field                   | ✓        |
//! | C7b  | `match` accesses `i64` named field with arithmetic      | ✓        |
//! | C7c  | `match` combines both predicates (conjunction)          | ✓        |
//! | C7d  | Transition preserving complex named-field invariant     | ✓        |
//!
//! ## Run
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```

use creusot_std::prelude::*;

/// A mini state with a unit variant and a struct variant carrying named fields.
///
/// Mirrors `ArchiveConnectionState::Connecting { profile_name, backend }`.
pub enum TaggedState {
    /// No active connection — trivially consistent.
    Idle,
    /// An in-flight connection with a named profile and a retry count.
    Connecting {
        /// Name of the connection profile (must be non-empty).
        profile_name: String,
        /// Number of connection attempts so far (must be non-negative).
        attempts: i64,
    },
}

/// C7 invariant: Idle is always OK; Connecting requires a non-empty name and
/// a non-negative attempt counter.

#[logic]
pub fn c7_consistent(s: &TaggedState) -> bool {
    pearlite! {
        match s {
            TaggedState::Idle => true,
            TaggedState::Connecting { profile_name, attempts } =>
                profile_name@.len() > 0 && attempts@ >= 0,
        }
    }
}

// ── Per-field helpers ─────────────────────────────────────────────────────────

/// C7a: accessing a `String` named field.

#[logic]
pub fn c7_name_nonempty(s: &TaggedState) -> bool {
    pearlite! {
        match s {
            TaggedState::Idle => true,
            TaggedState::Connecting { profile_name, .. } => profile_name@.len() > 0,
        }
    }
}

/// C7b: accessing an `i64` named field with arithmetic.

#[logic]
pub fn c7_attempts_nonneg(s: &TaggedState) -> bool {
    pearlite! {
        match s {
            TaggedState::Idle => true,
            TaggedState::Connecting { attempts, .. } => attempts@ >= 0,
        }
    }
}

// ── Transitions ───────────────────────────────────────────────────────────────

/// Start a connection — Idle → Connecting.
///
/// Requires non-empty `profile_name`; the attempt counter starts at 0.

#[requires(profile_name@.len() > 0)]
#[ensures(c7_consistent(&result))]
pub fn c7_start(_s: TaggedState, profile_name: String) -> TaggedState {
    TaggedState::Connecting {
        profile_name,
        attempts: 0,
    }
}

/// Retry — increment the attempt counter, staying in `Connecting`.
///
/// C7d: a transition that preserves a two-field invariant.
/// The upper-bound guard prevents `i64` overflow (same lesson as C2).

#[requires(c7_consistent(&s))]
#[requires(match &s {
    TaggedState::Connecting { attempts, .. } => attempts@ < 9223372036854775807i64@,
    _ => true,
})]
#[ensures(c7_consistent(&result))]
pub fn c7_retry(s: TaggedState) -> TaggedState {
    match s {
        TaggedState::Idle => TaggedState::Idle,
        TaggedState::Connecting {
            profile_name,
            attempts,
        } => TaggedState::Connecting {
            profile_name,
            attempts: attempts + 1,
        },
    }
}
