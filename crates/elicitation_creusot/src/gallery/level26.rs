//! Gallery level C26: `#[instrument]` delegation and `format!` pitfalls.
//!
//! **Hypothesis**: Two distinct patterns cause Creusot to fail with
//! `Unsupported constant value: Scalar(allocN) of type &'_ [u8; N]`:
//!
//! 1. A `__creusot` harness whose inlined body delegates to a function
//!    annotated with `#[instrument]` — the `tracing` macro injects static
//!    byte-slice span-name constants that Creusot cannot translate.
//!
//! 2. Any function body containing `format!(...)` — the format-string
//!    machinery stores the template as a static `[u8; N]` in the MIR.
//!
//! Both manifest as the same error:
//!
//! ```text
//! error: Unsupported constant value: Scalar(alloc1) of type &'?14 [u8; 5_usize]
//!   = note: this error originates in the macro `format` (or `tracing::instrument`)
//! ```
//!
//! ## Pitfall 1 — `#[instrument]` in delegated body
//!
//! When `formal_method` inlines the body of `abort_edits` into
//! `abort_edits__creusot`:
//!
//! ```rust,ignore
//! // abort_edits's body is:
//! fn abort_edits(state, proof) -> ... {
//!     commit_edits(state, proof)   // ← single-call delegation
//! }
//!
//! // Generated harness (BROKEN):
//! pub(crate) fn abort_edits__creusot(state, proof) -> ... {
//!     commit_edits(state, proof)   // ← calls original, which has #[instrument]
//! }
//! ```
//!
//! Creusot descends into `commit_edits` to translate its MIR, hits the
//! tracing-generated span-name constant `"abort"` / `"edits"` (5 bytes),
//! and fails.
//!
//! ### Fix: call the `__creusot` companion instead
//!
//! The `formal_method` macro now detects the `{ f(args) }` single-call
//! pattern and rewrites it to `{ f__creusot(args) }` in the Creusot body:
//!
//! ```rust,ignore
//! // Generated harness (FIXED):
//! pub(crate) fn abort_edits__creusot(state, proof) -> ... {
//!     commit_edits__creusot(state, proof)  // ← companion: no #[instrument]
//! }
//! ```
//!
//! `commit_edits__creusot` carries its own `#[requires]`/`#[ensures]`, so
//! Creusot discharges the harness VC without looking at the original body.
//!
//! ## Pitfall 2 — `format!` in function body (e.g. `kani_label!`)
//!
//! The `kani_label!` macro was defined as:
//!
//! ```rust,ignore
//! macro_rules! kani_label {
//!     ($($arg:tt)*) => {{
//!         #[cfg(not(kani))]
//!         let _s = format!($($arg)*);   // ← used in non-kani mode
//!         #[cfg(kani)]
//!         let _s = ::std::string::String::new();
//!         _s
//!     }};
//! }
//! ```
//!
//! Under Creusot (`cfg(creusot)` set, `cfg(kani)` clear), the `format!` arm
//! activates.  `format!` stores the template string as a `&[u8; N]` static —
//! same `Scalar(allocN)` failure as `#[instrument]`.
//!
//! ### Fix: include `creusot` in the bypass guard
//!
//! ```rust,ignore
//! macro_rules! kani_label {
//!     ($($arg:tt)*) => {{
//!         #[cfg(any(kani, creusot))]
//!         let _s = ::std::string::String::new();      // ← safe under both
//!         #[cfg(not(any(kani, creusot)))]
//!         let _s = format!($($arg)*);
//!         _s
//!     }};
//! }
//! ```
//!
//! `String::new()` has no static allocations in its MIR; Creusot handles it
//! without issue.  The invariant does not inspect label contents, so the
//! semantic change is invisible to the prover.
//!
//! ## Gallery experiments
//!
//! | ID    | What                                               | Expected |
//! |-------|----------------------------------------------------|----------|
//! | C26a  | Direct delegation to clean helper (no instrument)  | ✓ proved |
//! | C26b  | `__creusot` companion delegation (fix pattern 1)   | ✓ proved |
//! | C26c  | Body uses `String::new()` label (fix pattern 2)    | ✓ proved |
//!
//! The failure-mode functions (`format!` body, raw `#[instrument]` callee)
//! are not included as runnable code here because they produce hard compile
//! errors under `cargo creusot`.  The root-cause analysis is in the comments
//! above.
//!
//! ## Results
//!
//! All three patterns proved by `why3find prove`:
//!
//! | File          | VCs | Result |
//! |---------------|-----|--------|
//! | `c26_clean`   | 1   | ✔      |
//! | `c26_chain`   | 1   | ✔      |
//! | `c26_label`   | 1   | ✔      |
//!
//! ## Key findings
//!
//! 1. **Delegation rewriting rule**: `formal_method` applies a single-call
//!    rewrite — if the body is `{ f(args) }`, the Creusot body becomes
//!    `{ f__creusot(args) }`.  This prevents Creusot from descending into
//!    `#[instrument]`-wrapped originals in a dependent crate.
//!
//! 2. **`format!` / string-macro rule**: any function body compiled under
//!    Creusot must avoid `format!`, `println!`, `write!`, and similar macros
//!    that embed static byte-slice constants.  Use `String::new()` as a
//!    Creusot-safe placeholder for labels and diagnostic strings.
//!
//! 3. **General principle**: Creusot cannot translate `&'static str` /
//!    `&[u8; N]` static allocation constants.  Any macro that generates
//!    format strings, span names, or string templates will fail.  Audit
//!    macro expansions when adding new helpers to `__creusot` bodies.
//!
//! ## Run
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! why3find prove -p creusot \
//!   verif/elicitation_creusot_rlib/gallery/level26/*.coma
//! ```

use creusot_std::prelude::*;

// ── State ────────────────────────────────────────────────────────────────────

/// Two-state machine: Idle or Active with a label string.
pub enum C26State {
    /// No active task.
    Idle,
    /// Active task with a display label.
    Active {
        /// Must be non-empty.
        count: u32,
        /// Display label (content ignored by invariant).
        label: String,
    },
}

/// Consistency predicate: count must be non-zero in Active state.
///
/// The `label` field is intentionally not checked — it is a display string
/// whose content is irrelevant to correctness.  This makes `String::new()`
/// a valid Creusot placeholder for `kani_label!`-generated labels.
#[logic]
pub fn c26_consistent(s: &C26State) -> bool {
    pearlite! {
        match s {
            C26State::Idle => true,
            C26State::Active { count, .. } => count@ > 0,
        }
    }
}

// ── C26a: clean helper, no delegation issue ───────────────────────────────────

/// Reset to idle — trivial body, no delegation.
///
/// **C26a**: A clean function with no `#[instrument]` and no `format!`.
/// Baseline proving this compiles and is provable.
#[requires(c26_consistent(&state))]
#[ensures(c26_consistent(&result))]
pub fn c26_clean(state: C26State) -> C26State {
    let _ = state;
    C26State::Idle
}

// ── C26b: companion-delegation chain ─────────────────────────────────────────

/// Increment step — changes label to an empty string (Creusot-safe).
///
/// This simulates the real `commit_edits` pattern: a function that would
/// ordinarily have `#[instrument]` in production, but whose `__creusot`
/// companion has no tracing overhead.
#[requires(c26_consistent(&state))]
#[ensures(c26_consistent(&result))]
pub fn c26_step(state: C26State) -> C26State {
    match state {
        C26State::Active { count, .. } => C26State::Active {
            count,
            label: String::new(), // Creusot-safe: no format! call
        },
        other => other,
    }
}

/// Delegates to `c26_step` — simulates the `abort_edits → commit_edits` pattern.
///
/// **C26b**: The `formal_method` macro now rewrites `{ f(args) }` bodies in
/// `__creusot` companions to `{ f__creusot(args) }`.  This gallery function
/// simulates the post-rewrite state: `c26_chain` calls `c26_step` directly,
/// just as `abort_edits__creusot` now calls `commit_edits__creusot`.
///
/// The VC discharges because `c26_step` carries a matching `#[ensures]` —
/// Creusot uses that spec without descending into `c26_step`'s body.
#[requires(c26_consistent(&state))]
#[ensures(c26_consistent(&result))]
pub fn c26_chain(state: C26State) -> C26State {
    c26_step(state)
}

// ── C26c: String::new() label (kani_label! fix) ───────────────────────────────

/// Transition that constructs a label — Creusot-safe variant.
///
/// **C26c**: The original code used `kani_label!("{schema}.{table}")` which
/// expanded to `format!(...)` under Creusot, generating a `&[u8; N]` static.
///
/// After the fix, `kani_label!` expands to `String::new()` under both `kani`
/// and `creusot`.  This function demonstrates that pattern: `String::new()`
/// is Creusot-translatable and the invariant (which ignores labels) closes
/// without issue.
#[requires(c26_consistent(&state))]
#[ensures(c26_consistent(&result))]
pub fn c26_label(state: C26State, new_count: u32) -> C26State {
    if new_count > 0 {
        C26State::Active {
            count: new_count,
            label: String::new(), // was: format!("{schema}.{table}") — Creusot-unsafe
        }
    } else {
        let _ = state;
        C26State::Idle
    }
}
