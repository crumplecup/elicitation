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
//! - `HasLabel` — element has non-empty accessible label
//! - `ValidRole` — element has valid ARIA role
//! - `MinTargetSize` — interactive element ≥44x44 (Level AAA touch targets)
//! - `NoOverflow` — element fits within viewport
//! - `KeyboardAccessible` — element keyboard-navigable
//! - `AccessibleAA` — composite (all Level AA criteria)
//! - `SufficientContrast` — color pair meets 4.5:1 ratio (WCAG 1.4.3)
//! - `FocusVisible` — visible focus indicator (WCAG 2.4.11)
//! - `AltTextProvided` — text alternative for non-text content (WCAG 1.1.1)
//! - `StructuredContent` — structure is programmatically determinable (WCAG 1.3.1)
//! - `RenderComplete` — UI tree successfully rendered
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
//! // Render to frontend via UiRenderer
//! // let backend = elicit_egui::EguiBackend::new(&egui_ctx);
//! // let (rendered, stats) = verified.render(&backend)?;
//! # Ok::<(), elicit_ui::VerificationReport>(())
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod accesskit_backend;
mod builder;
mod color_contrast;
pub mod constraints;
mod contracts;
mod css_units;
mod errors;
mod layout_engine;
mod spatial;
pub mod traits;
mod types;
mod typestate;
mod ui_types;
mod validators;

pub use accesskit_backend::AccessKitUiBackend;
pub use builder::LayoutBuilder;
pub use color_contrast::{
    ContrastEnhanced, ContrastMinimum, NonTextContrast, SrgbColor, TextSize, contrast_ratio,
};
pub use constraints::{
    BreakpointOutcome, BreakpointReport, BreakpointResult, BreakpointTier, Constraint,
    ConstraintContext, ConstraintSet, ConstraintSetBuilder, ConstraintVerification, GridAlignment,
    HasLabelConstraint, KeyboardAccessibleConstraint, MinReadableSize, MinSpacing,
    MinTouchTargetConstraint, NoOverflowConstraint, Reflow320, ResizeText200, SpecReference,
    TerminalAccessible, TerminalBreakpoint, TerminalBreakpointSet, TerminalNoOverflow, TextSpacing,
    ValidRoleConstraint, Violation, WcagLevel,
};
pub use contracts::{
    AccessibleAA, AltTextProvided, FocusVisible, HasLabel, KeyboardAccessible, MinTargetSize,
    NoOverflow, RenderComplete, StructuredContent, SufficientContrast, ValidRole,
};
pub use css_units::{Breakpoint, BreakpointSet, CssLength, CssParseError, is_zoom_invariant};
pub use elicit_accesskit::ColorTheme;
pub use errors::{
    UiError, UiErrorKind, UiResult, VerificationError, VerificationErrorKind, VerificationReport,
};
pub use layout_engine::LayoutEngineError;
#[cfg(feature = "layout-engine")]
pub use layout_engine::{LayoutMode, TaffyBridge};
pub use spatial::{BoundingBox, LayoutContext};
pub use traits::{
    UiAccessibilityAuditor, UiBackend, UiEventDispatcher, UiInspector, UiLayoutManager,
    UiNavigationManager, UiRenderer, UiStyleManager, UiWidgetFactory,
};
pub use types::{ElementId, Label, RenderStats, Size, Viewport};
pub use typestate::{ConstraintProfile, Layout, Pending, Rendered, Verified};
pub use ui_types::{
    ContainerId, ContrastViolation, VerifiedTree, WidgetA11y, WidgetId, WidgetInfo,
};
