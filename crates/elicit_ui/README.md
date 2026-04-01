# elicit_ui

Typestate-based verified UI system using AccessKit as universal IR.

## Overview

`elicit_ui` provides a formally verifiable UI construction system that
ensures WCAG compliance through typestate patterns, composable constraints,
and proof-carrying contracts. Layouts are constructed as AccessKit trees,
verified against spec-traceable constraints, and rendered through
pluggable backends (egui, ratatui) from a single IR.

## Architecture

```text
                     ┌──────────────────────────────────────────┐
                     │          Constraint System               │
                     │  WCAG · Spatial · Terminal · Custom      │
                     └────────────────┬─────────────────────────┘
                                      │
AccessKit Tree ──→ Layout<Pending> ──→│verify()──→ Layout<Verified>
                                                        │
                                      ┌─────────────────┤
                                      ▼                 ▼
                               EguiBackend        RatatuiBackend
                              (elicit_egui)      (elicit_ratatui)
                                      │                 │
                                      ▼                 ▼
                                Layout<Rendered>  Layout<Rendered>
```

### Key Components

1. **AccessKit Universal IR** — all UI represented as accessibility trees
2. **Typestate State Machine** — `Pending → Verified → Rendered`
3. **Composable Constraint System** — spec-traceable, tiered (hard/structural/advisory)
4. **Pluggable Backends** — `RenderBackend` trait for frontend-agnostic rendering
5. **Terminal Breakpoints** — multi-size verification across standard terminal sizes
6. **Color Contrast** — WCAG 1.4.3/1.4.6/1.4.11 contrast ratio verification
7. **CSS Units** — type-safe CSS lengths, breakpoints, zoom invariance
8. **Proof-Carrying Contracts** — formal verification via Kani, Verus, Creusot

## State Machine

```text
Layout<Pending>  ──verify_a()──→  Layout<Verified>  ──render()──→  Layout<Rendered>
                 ──verify_aa()─→
                 ──verify_aaa()→
                 ──verify_custom()→
```

- `Layout<Pending>` — constructed from an AccessKit `TreeUpdate`, awaiting verification
- `Layout<Verified>` — all constraints satisfied; exposes `nodes()` and `render()`
- `Layout<Rendered>` — rendered through a `RenderBackend` (egui, ratatui, etc.)

## Constraint System

Constraints implement the `Constraint` trait, each anchored to a recognized standard:

```rust
pub trait Constraint: Send + Sync {
    fn check(&self, node_id: NodeId, ctx: &ConstraintContext<'_>) -> Result<(), Violation>;
    fn spec_ref(&self) -> SpecReference;
}
```

Constraints are composed into a `ConstraintSet` with enforcement tiers:

```rust
let constraints = ConstraintSetBuilder::default()
    .hard(HasLabelConstraint)        // must pass
    .hard(NoOverflowConstraint)      // must pass
    .advisory(GridAlignment { step: 8.0 })  // warning only
    .build();
```

### WCAG Constraints

| Constraint | WCAG | Level | Description |
|-----------|------|-------|-------------|
| `HasLabelConstraint` | 4.1.2 | A | Non-empty accessible label |
| `ValidRoleConstraint` | 4.1.2 | A | Valid ARIA role |
| `KeyboardAccessibleConstraint` | 2.1.1 | A | Keyboard navigable |
| `NoOverflowConstraint` | 1.4.10 | AA | Fits within viewport |
| `MinTouchTargetConstraint` | 2.5.5 | AAA | ≥44×44 touch target |
| `Reflow320` | 1.4.10 | AA | Content reflows at 320px |
| `TextSpacing` | 1.4.12 | AA | Text spacing adjustments |
| `ResizeText200` | 1.4.4 | AA | Text resizable to 200% |

### Spatial Constraints

| Constraint | Standard | Description |
|-----------|----------|-------------|
| `GridAlignment` | Material Design 3 | Position snaps to grid step |
| `MinSpacing` | Design system | Minimum gap between elements |

### Terminal Constraints

| Constraint | Standard | Description |
|-----------|----------|-------------|
| `TerminalNoOverflow` | WCAG 1.4.10 | Fits cell viewport (cols×rows) |
| `MinReadableSize` | ISO 9241-3 | Container ≥10 cols × 3 rows |

### Constraint Profiles

Pre-built profiles for common verification scenarios:

```rust
let verified = layout.verify_with_profile(viewport, ConstraintProfile::WcagAA)?;
```

| Profile | Constraints |
|---------|-------------|
| `WcagA` | HasLabel, ValidRole, KeyboardAccessible |
| `WcagAA` | Level A + NoOverflow |
| `WcagAAA` | Level AA + MinTouchTarget |

### TerminalAccessible

Convenience builder combining WCAG + terminal constraints:

```rust
let constraints = TerminalAccessible::default().to_constraint_set();
// Includes: HasLabel, ValidRole, KeyboardAccessible, TerminalNoOverflow, MinReadableSize
```

## Terminal Breakpoint Verification

Verify a layout across standard terminal sizes in one call:

```rust
let breakpoints = TerminalBreakpointSet::standard();
let constraints = ConstraintSetBuilder::default()
    .hard(TerminalNoOverflow)
    .hard(MinReadableSize::default())
    .build();

let report = breakpoints.verify_all(root_id, &nodes, &constraints);
println!("{}", report.summary());
```

Output:

```text
Terminal Breakpoint Report
─────────────────────────────────────────
micro         40×12  [expected-fail] 📝 expected-fail
tiny          60×20  [advisory]      ⚠️ warning
VT100         80×24  [required]      ✅ pass
small        100×30  [required]      ✅ pass
medium       120×40  [required]      ✅ pass
large        160×50  [required]      ✅ pass
ultrawide    200×60  [required]      ✅ pass
─────────────────────────────────────────
Result: PASS (5 pass, 0 fail, 1 warn, 1 expected-fail)
```

### Breakpoint Tiers

| Tier | Meaning |
|------|---------|
| `Required` | Must pass — blocks validity |
| `Advisory` | Warning only — informational |
| `ExpectedFail` | Documents known limitations |

## Rendering

`elicit_ui` defines the `RenderBackend` trait; implementations live in
companion crates:

```rust
pub trait RenderBackend {
    fn render_tree(&self, nodes: &HashMap<NodeId, Node>, root: NodeId) -> RenderStats;
}
```

| Backend | Crate | Description |
|---------|-------|-------------|
| `EguiBackend` | `elicit_egui` | Renders to egui widgets |
| `RatatuiBackend` | `elicit_ratatui` | Renders to TuiNode trees |

```rust
// After verification, render to any backend:
let (rendered, stats) = verified.render(&backend);
```

## Color Contrast

WCAG contrast ratio verification with `palette`-based sRGB colors:

```rust
use elicit_ui::{SrgbColor, contrast_ratio, ContrastMinimum, ContrastEnhanced};

let fg = SrgbColor::new(0.0, 0.0, 0.0);      // black
let bg = SrgbColor::new(1.0, 1.0, 1.0);      // white
let ratio = contrast_ratio(&fg, &bg);         // 21.0

// Type-level contrast witnesses
let _min: ContrastMinimum = ContrastMinimum::check(&fg, &bg, TextSize::Normal)?;   // 4.5:1
let _enh: ContrastEnhanced = ContrastEnhanced::check(&fg, &bg, TextSize::Normal)?; // 7:1
```

## CSS Units

Type-safe CSS length parsing and zoom invariance checking:

```rust
use elicit_ui::{CssLength, is_zoom_invariant};

let length = CssLength::Em(1.5);
assert!(is_zoom_invariant(&length));   // relative unit

let px = CssLength::Px(16.0);
assert!(!is_zoom_invariant(&px));      // absolute unit
```

## WCAG Propositions (Compile-Time)

Type-level witnesses that carry proof of compliance:

| Proposition | Description |
|------------|-------------|
| `HasLabel<T>` | Non-empty accessible label |
| `ValidRole<T>` | Valid ARIA role |
| `KeyboardAccessible<T>` | Keyboard navigable |
| `NoOverflow<T>` | Fits within viewport |
| `MinTargetSize<T>` | ≥44×44 touch target |
| `AccessibleAA<T>` | Composite Level AA |

## Usage

### Basic Verification

```rust
use elicit_ui::{Layout, Viewport};
use accesskit::{Node, NodeId, Role, Tree, TreeId, TreeUpdate};

let button_id = NodeId::from(1u64);
let root_id = NodeId::from(0u64);

let mut button = Node::new(Role::Button);
button.set_label("Submit");
button.set_bounds(accesskit::Rect {
    x0: 0.0, y0: 0.0, x1: 100.0, y1: 50.0,
});

let mut root = Node::new(Role::Window);
root.set_children(vec![button_id]);

let update = TreeUpdate {
    nodes: vec![(root_id, root), (button_id, button)],
    tree: Some(Tree::new(root_id)),
    tree_id: TreeId::ROOT,
    focus: root_id,
};

let layout = Layout::from_update(update);
let verified = layout.verify_aa(Viewport::new(1920, 1080))?;
assert_eq!(verified.report().error_count(), 0);
# Ok::<(), elicit_ui::VerificationReport>(())
```

### Custom Constraints

```rust
use elicit_ui::{Layout, Viewport, ConstraintSetBuilder, GridAlignment, MinSpacing};

let constraints = ConstraintSetBuilder::default()
    .hard(elicit_ui::HasLabelConstraint)
    .hard(elicit_ui::NoOverflowConstraint)
    .advisory(GridAlignment { step: 8.0 })
    .advisory(MinSpacing { min_gap: 4.0 })
    .build();

let verified = layout.verify_custom(viewport, &constraints)?;
```

## Features

| Feature | Dependencies | Description |
|---------|-------------|-------------|
| `emit` | `proc-macro2`, `quote`, `elicitation/emit` | Code emission for formal verification |
| `geo` | `geo`, `geo-types` | GeoRust spatial type support |
| `css` | `cssparser` | CSS value parsing |
| `color` | `palette` | sRGB color contrast verification |
| `layout-engine` | `taffy` | Flexbox/grid layout computation |

## Formal Verification

When `emit` is enabled, propositions can generate proofs for:

- **Kani** — bounded model checking
- **Verus** — SMT-based verification
- **Creusot** — deductive verification via Why3

## Comparison to Ledger Pattern

| Aspect | Ledger | UI |
|--------|--------|-----|
| Domain | Money transfers | Interactive layouts |
| States | Pending → Validated → Committed | Pending → Verified → Rendered |
| Universal IR | SQL schema | AccessKit tree |
| Propositions | AmountPositive, SufficientFunds | HasLabel, MinTargetSize |
| Backends | SQLite, PostgreSQL, MySQL | egui, ratatui |
| Invariant | Σ debits = Σ credits | All interactive elements accessible |

## License

Licensed under Apache-2.0 OR MIT.
