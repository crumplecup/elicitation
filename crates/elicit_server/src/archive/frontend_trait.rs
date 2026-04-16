//! The [`ArchiveFrontend`] trait — the compiler-enforced key-binding contract.
//!
//! Every archive frontend (ratatui, egui, leptos) must implement this trait.
//! The exhaustive match required inside `dispatch_action` is the guarantee
//! that no declared [`ArchiveAction`] is silently ignored in any rendering
//! target.
//!
//! ## Contract
//!
//! ```text
//! ArchiveKeyMap::default_map()  (IR — key→action mapping)
//!     └─► ArchiveKeyMap::resolve(combo, mode)
//!              └─► ArchiveFrontend::dispatch_action(action)  (trait — exhaustive match)
//!                       └─► frontend-specific state mutation
//! ```
//!
//! Text input (characters typed in a filter bar or SQL editor) is forwarded
//! separately via [`ArchiveFrontend::dispatch_text`] so the key map does not
//! need one variant per printable character.

use crate::archive::actions::ArchiveAction;

/// Implemented by every archive frontend.
///
/// ### Implementing the trait
///
/// `dispatch_action` **must** contain an exhaustive `match action { … }` with
/// a branch for every [`ArchiveAction`] variant.  The compiler enforces this:
/// adding a new variant to `ArchiveAction` immediately produces compile errors
/// in all three frontend impls until they are updated.
///
/// `dispatch_action` returns `true` when the application should quit.
///
/// ### Text input
///
/// Raw printable characters (e.g. typed in a filter bar or SQL editor) that
/// are not consumed by the key map are forwarded via [`dispatch_text`].  This
/// keeps the key map focused on discrete named actions.
///
/// [`dispatch_text`]: ArchiveFrontend::dispatch_text
pub trait ArchiveFrontend {
    /// Execute a named action.
    ///
    /// Returns `true` when the application should exit (the
    /// [`ArchiveAction::Quit`] action or equivalent).
    fn dispatch_action(&mut self, action: ArchiveAction) -> bool;

    /// Forward a printable text chunk to the active text field.
    ///
    /// Called for characters typed in a text-input mode (filter bar,
    /// save-query prompt, SQL editor) that are not intercepted by the key map.
    fn dispatch_text(&mut self, text: &str);
}
