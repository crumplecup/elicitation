//! Gallery level C4: String length invariant.
//!
//! **Hypothesis**: Pearlite can reason about `String` length via the `@` model
//! operator (which maps `String` to a `Seq<char>` in Why3).  Specifically:
//! - `s@.len() > 0` is expressible in a `#[logic]` predicate
//! - Functions that create non-empty strings satisfy a non-empty postcondition
//! - Functions that receive strings can require non-emptiness
//!
//! **Why this matters**: The archive VSM invariants need to express that error
//! messages are non-empty (`ConnectionError` variant), that panel titles are
//! non-empty, etc.  If `String` length is not modelable in Pearlite, the
//! invariant predicates must avoid String fields entirely.
//!
//! ## Experiment table
//!
//! | ID   | What                                   | Expected |
//! |------|----------------------------------------|----------|
//! | C4a  | `s@.len() > 0` in `#[logic]`          | ✓        |
//! | C4b  | Literal `"hello"` satisfies nonempty  | ✓        |
//! | C4c  | `String::from` preserves nonempty     | ✓ or ✗   |
//!
//! ## Run
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```

use creusot_std::prelude::*;

/// C4a: a string is "non-empty" if its logical sequence has at least one element.
///
/// The `@` operator maps `String` → `creusot_std::logic::Seq<char>` in Why3.

#[logic]
pub fn c4_nonempty(s: &String) -> bool {
    pearlite! { s@.len() > 0 }
}

/// C4b: the literal `"hello"` satisfies `c4_nonempty`.
///
/// Verifies: a concrete, known-non-empty string has `len > 0` in the model.

#[requires(true)]
#[ensures(c4_nonempty(&result))]
pub fn c4_hello() -> String {
    String::from("hello")
}

/// C4c: a function that accepts a non-empty string and returns it unchanged.
///
/// Verifies: the precondition on `String` length compiles and is usable.

#[requires(c4_nonempty(&s))]
#[ensures(c4_nonempty(&result))]
pub fn c4_pass_nonempty(s: String) -> String {
    s
}

/// A struct with a String field — does the `@` model reach through the field?

pub struct ErrorState {
    pub message: String,
}

/// C4d: invariant over a struct field of type `String`.

#[logic]
pub fn c4_error_has_message(e: &ErrorState) -> bool {
    pearlite! { e.message@.len() > 0 }
}

/// Construct an `ErrorState` from a non-empty message.

#[requires(c4_nonempty(&message))]
#[ensures(c4_error_has_message(&result))]
pub fn c4_make_error(message: String) -> ErrorState {
    ErrorState { message }
}
