//! `elicit_ui` — typestate-based verified UI system.
//!
//! Provides a formally verifiable UI construction system using:
//!
//! 1. **AccessKit universal IR** — all UI represented as accessibility trees
//! 2. **Typestate state machine** — `Pending → Verified → Rendered`
//! 3. **Proof-carrying contracts** — WCAG compliance enforced at compile time
//! 4. **Multiple frontends** — render to egui, leptos, ratatui from single IR
//!
//! # Architecture
//!
//! ```text
//! Domain Types → Typestate → Contracts → AccessKit Tree → Frontend
//! (Button, Label) → (Pending) → (verify_aa) → (tree)  → (egui/leptos/ratatui)
//! ```
//!
//! # State Machine
//!
//! - `Layout<Pending>` — awaiting verification
//! - `Layout<Verified>` — verified against WCAG Level AA constraints
//! - `Layout<Rendered>` — rendered to a specific frontend
//!
//! # Propositions (WCAG Compliance)
//!
//! - `HasLabel<T>` — element has non-empty accessible label
//! - `ValidRole<T>` — element has valid ARIA role
//! - `MinTargetSize<T>` — interactive element ≥44x44 (Level AAA touch targets)
//! - `NoOverflow<T>` — element fits within viewport
//! - `KeyboardAccessible<T>` — element keyboard-navigable
//! - `AccessibleAA<T>` — composite (all Level AA criteria)
//!
//! # Example
//!
//! ```rust,no_run
//! use elicit_ui::{Layout, Viewport};
//! use accesskit::{Node, NodeId, Role, Tree, TreeId, TreeUpdate};
//!
//! // Create a button using AccessKit
//! let button_id = NodeId::from(1u64);
//! let root_id = NodeId::from(0u64);
//!
//! let mut button = Node::new(Role::Button);
//! button.set_label("Submit");
//! button.set_bounds(accesskit::Rect {
//!     x0: 0.0, y0: 0.0, x1: 100.0, y1: 50.0,
//! });
//!
//! let mut root = Node::new(Role::Window);
//! root.set_children(vec![button_id]);
//!
//! let update = TreeUpdate {
//!     nodes: vec![(root_id, root), (button_id, button)],
//!     tree: Some(Tree::new(root_id)),
//!     tree_id: TreeId::ROOT,
//!     focus: root_id,
//! };
//!
//! // Create layout from AccessKit tree
//! let layout = Layout::from_update(update);
//!
//! // Verify WCAG Level AA compliance
//! let verified = layout.verify_aa(Viewport::new(1920, 1080))?;
//!
//! // Render to frontend (requires egui-backend feature)
//! // let rendered = verified.render_egui(&egui_ctx);
//! # Ok::<(), elicit_ui::VerificationReport>(())
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod contracts;
mod errors;
#[cfg(feature = "egui-backend")]
mod renderer;
mod typestate;
mod types;
mod validators;

pub use contracts::{
    AccessibleAA, HasLabel, KeyboardAccessible, MinTargetSize, NoOverflow, ValidRole,
};
pub use errors::{VerificationError, VerificationErrorKind, VerificationReport};
#[cfg(feature = "egui-backend")]
pub use renderer::{bounds_to_size, render_tree, RenderStats};
pub use typestate::{Layout, Pending, Rendered, Verified};
pub use types::{ElementId, Label, Size, Viewport};
