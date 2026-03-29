# elicit_ui

Typestate-based verified UI system using AccessKit as universal IR.

## Overview

`elicit_ui` provides a formally verifiable UI construction system that ensures WCAG compliance at compile time using typestate patterns and proof-carrying contracts.

## Architecture

```text
Domain Types → Typestate → Contracts → AccessKit Tree → Frontend
(Button, Label) → (Pending) → (verify_aa) → (tree)  → (egui/leptos/ratatui)
```

### Key Components

1. **AccessKit Universal IR** — All UI represented as accessibility trees
2. **Typestate State Machine** — `Pending → Verified → Rendered`
3. **Proof-Carrying Contracts** — WCAG compliance enforced at compile time
4. **Multiple Frontends** — Render to egui, leptos, ratatui from single IR

## State Machine

- `Layout<Pending>` — awaiting verification
- `Layout<Verified>` — verified against WCAG constraints
- `Layout<Rendered>` — rendered to a specific frontend

## WCAG Propositions

### Level A
- `HasLabel<T>` — element has non-empty accessible label (WCAG 2.4.6, 4.1.2)
- `ValidRole<T>` — element has valid ARIA role (WCAG 4.1.2)
- `KeyboardAccessible<T>` — element keyboard-navigable (WCAG 2.1.1)

### Level AA
- `NoOverflow<T>` — element fits within viewport (WCAG 1.4.10)
- `AccessibleAA<T>` — composite (all Level AA criteria)

### Level AAA
- `MinTargetSize<T>` — interactive element ≥44x44 (WCAG 2.5.5)

## Usage

### Basic Verification

```rust
use elicit_ui::{Layout, Viewport};
use accesskit::{Node, NodeId, Role, Tree, TreeId, TreeUpdate};

// Create a button using AccessKit
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

// Create layout from AccessKit tree
let layout = Layout::from_update(update);

// Verify WCAG Level AA compliance
let verified = layout.verify_aa(Viewport::new(1920, 1080))?;

// Access verification results
assert_eq!(verified.report().error_count(), 0);
```

### Verification Levels

```rust
// Level A: Basic accessibility
let verified_a = layout.verify_a(viewport)?;

// Level AA: Enhanced accessibility (includes NoOverflow)
let verified_aa = layout.verify_aa(viewport)?;

// Level AAA: Maximum accessibility (includes MinTargetSize)
let verified_aaa = layout.verify_aaa(viewport)?;
```

### Error Handling

```rust
match layout.verify_aa(viewport) {
    Ok(verified) => {
        println!("✓ UI verified against WCAG Level AA");
        // Continue with rendering...
    }
    Err(report) => {
        eprintln!("✗ Verification failed with {} errors:", report.error_count());
        for error in &report.errors {
            eprintln!("  - {}", error);
        }
    }
}
```

## Features

- `emit` — Code emission for proof-carrying contracts (Kani/Verus/Creusot)
- `egui-backend` — egui frontend renderer

## Verification

When the `emit` feature is enabled, each proposition can generate formal verification proofs:

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
| Backends | SQLite, PostgreSQL, MySQL | egui, leptos, ratatui |
| Invariant | Σ debits = Σ credits | All interactive elements accessible |

## Implementation Status

### Phase 1: Core Typestate ✓
- [x] Basic types (Label, Size, ElementId, Viewport)
- [x] Error types and verification reports
- [x] Typestate markers (Pending, Verified, Rendered)
- [x] Layout state machine
- [x] Basic propositions (HasLabel, ValidRole, KeyboardAccessible)
- [x] WCAG Level A/AA/AAA verification
- [x] Tests demonstrating button verification

### Phase 2: Additional Constraints (TODO)
- [ ] MinTargetSize verification refinement
- [ ] NoOverflow viewport clipping detection
- [ ] ColorContrast proposition (WCAG 1.4.3)
- [ ] FocusVisible proposition (WCAG 2.4.7)

### Phase 3: Multiple Frontends (TODO)
- [ ] egui renderer implementation
- [ ] leptos renderer stub
- [ ] ratatui renderer stub

### Phase 4: Builder DSL (Optional)
- [ ] Ergonomic `ui!` macro
- [ ] Domain-specific constructors (Button, Label, etc.)

## License

Licensed under Apache-2.0 OR MIT.
