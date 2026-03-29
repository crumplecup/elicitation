# Typestate UI Design

> **Goal:** Design a formally verified UI system using elicitation framework's typestate state machines with proof-carrying contracts, building on the proven patterns from the ledger implementation.

---

## Design Principles (from Ledger)

**Pattern observed in ledger:**
1. **Typestate phases**: Each phase is a distinct type (Pending → Validated → Committed)
2. **Propositions**: Type-level statements (AmountPositive, SufficientFunds)
3. **Composite props**: And<P, Q> for combining preconditions
4. **Validation functions**: Return `Established<Prop>` on success, error otherwise
5. **Proof-carrying execution**: Functions require `Established<Prop>` parameter (zero-cost)
6. **Transitions consume self**: `pending.validate()` consumes and returns new state
7. **Universal IR**: SQL schema as backend-agnostic representation
8. **Multiple backends**: Same IR → different implementations (SQLite, PostgreSQL, MySQL)

**Applied to UI:**
1. **Typestate phases**: Layout<Pending> → Layout<Verified> → Layout<Rendered>
2. **Propositions**: HasLabel, MinTargetSize, NoOverflow, ValidRole
3. **Composite props**: Accessible<T> = And<HasLabel, And<MinTargetSize, ...>>
4. **Validation functions**: Walk AccessKit tree, establish proofs
5. **Proof-carrying rendering**: render() requires `Established<Accessible<T>>`
6. **Transitions consume self**: `pending.verify()` consumes and returns verified
7. **Universal IR**: AccessKit tree as frontend-agnostic representation
8. **Multiple frontends**: Same AccessKit tree → different renderers (egui, leptos, ratatui)

---

## Why AccessKit as Universal IR?

### Ledger Analogy

**Ledger:**
- **Domain problem**: Money transfers between accounts
- **Universal IR**: SQL schema (ledger_entries table)
- **Why SQL?**: Every database understands tables, rows, columns
- **Invariant**: Double-entry bookkeeping (sum of debits = sum of credits)
- **Backends**: SQLite, PostgreSQL, MySQL (all implement same schema)

**UI:**
- **Domain problem**: Interactive elements with labels, sizes, roles
- **Universal IR**: AccessKit tree (nodes with properties)
- **Why AccessKit?**: Every frontend can map to tree structure, designed for accessibility
- **Invariant**: WCAG compliance (all interactive elements labeled, sized, navigable)
- **Frontends**: egui, leptos, ratatui, slint (all render from same tree)

### AccessKit Tree Structure

```rust
pub struct Tree {
    root: NodeId,
    nodes: HashMap<NodeId, Node>,
}

pub struct Node {
    role: Role,              // Button, TextField, etc.
    name: Option<String>,    // Label text
    bounds: Option<Rect>,    // Size and position
    children: Vec<NodeId>,   // Tree structure
    actions: ActionSet,      // Click, Focus, etc.
    // ... other accessibility properties
}
```

**This is our "ledger schema" for UIs.**

---

## UI State Machine

### States (Typestate Phases)

```rust
/// Layout awaiting verification
struct Layout<Pending> {
    tree: accesskit::Tree,
    root: NodeId,
    _state: PhantomData<Pending>,
}

/// Layout verified against constraints
struct Layout<Verified> {
    tree: accesskit::Tree,
    root: NodeId,
    viewport: (u32, u32),           // Captured during verification
    constraint_report: ConstraintReport,  // Proof artifacts
    _state: PhantomData<Verified>,
}

/// Layout rendered to frontend
struct Layout<Rendered> {
    tree: accesskit::Tree,
    root: NodeId,
    frontend_handle: Box<dyn Any>,  // egui::Response, leptos::View, etc.
    _state: PhantomData<Rendered>,
}
```

### Transitions

```
Pending ──verify()──> Verified ──render_*()──> Rendered
```

**No rejection state** (unlike ledger) - verification either succeeds or returns error with details.

---

## Propositions (Contract Types)

### Basic Propositions

```rust
/// Proposition: Element has a non-empty label.
pub struct HasLabel<T>(PhantomData<T>);

impl<T> Prop for HasLabel<T> {
    fn kani_proof() -> TokenStream {
        quote! {
            #[kani::proof]
            fn verify_has_label() {
                let label: &str = kani::any();
                kani::assume(!label.is_empty());
                assert!(!label.is_empty(), "Label must be non-empty");
            }
        }
    }

    fn verus_proof() -> TokenStream {
        quote! {
            verus! {
                pub fn verify_has_label(label: &str) -> (result: bool)
                    requires !label.is_empty(),
                    ensures result == true,
                {
                    true
                }
            }
        }
    }

    fn creusot_proof() -> TokenStream {
        quote! {
            #[requires(!label.is_empty())]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_has_label(label: &str) -> bool {
                true
            }
        }
    }
}

/// Proposition: Element meets minimum touch target size (44x44).
pub struct MinTargetSize<T>(PhantomData<T>);

impl<T> Prop for MinTargetSize<T> {
    fn kani_proof() -> TokenStream {
        quote! {
            #[kani::proof]
            fn verify_min_target_size() {
                let width: u32 = kani::any();
                let height: u32 = kani::any();
                kani::assume(width >= 44 && height >= 44);
                assert!(width >= 44 && height >= 44, "Touch target too small");
            }
        }
    }
    // verus_proof, creusot_proof...
}

/// Proposition: Element doesn't overflow viewport.
pub struct NoOverflow<T>(PhantomData<T>);

impl<T> Prop for NoOverflow<T> {
    fn kani_proof() -> TokenStream {
        quote! {
            #[kani::proof]
            fn verify_no_overflow() {
                let elem_right: u32 = kani::any();
                let viewport_width: u32 = kani::any();
                kani::assume(elem_right <= viewport_width);
                assert!(elem_right <= viewport_width, "Element overflows viewport");
            }
        }
    }
}

/// Proposition: Element has a valid ARIA role.
pub struct ValidRole<T>(PhantomData<T>);

impl<T> Prop for ValidRole<T> {
    fn kani_proof() -> TokenStream {
        quote! {
            #[kani::proof]
            fn verify_valid_role() {
                let role: u8 = kani::any(); // Simplified
                kani::assume(role > 0); // Not Role::Unknown
                assert!(role > 0, "Role must be valid");
            }
        }
    }
}

/// Proposition: Element is keyboard navigable.
pub struct KeyboardAccessible<T>(PhantomData<T>);

impl<T> Prop for KeyboardAccessible<T> {
    fn kani_proof() -> TokenStream {
        quote! {
            #[kani::proof]
            fn verify_keyboard_accessible() {
                let has_focus_action: bool = kani::any();
                kani::assume(has_focus_action);
                assert!(has_focus_action, "Must support keyboard focus");
            }
        }
    }
}
```

### Composite Propositions

```rust
/// Composite: Element is accessible (WCAG Level A).
pub type AccessibleA<T> = And<
    HasLabel<T>,
    And<ValidRole<T>, KeyboardAccessible<T>>
>;

/// Composite: Element is accessible (WCAG Level AA).
pub type AccessibleAA<T> = And<
    AccessibleA<T>,
    And<MinTargetSize<T>, NoOverflow<T>>
>;
```

---

## Domain Types

### Basic Types

```rust
/// Label text (must be non-empty).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Label(pub String);

impl Label {
    pub fn new(text: impl Into<String>) -> Result<Self, InvalidLabel> {
        let text = text.into();
        if text.is_empty() {
            Err(InvalidLabel)
        } else {
            Ok(Self(text))
        }
    }
}

/// Size constraint (must be positive).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Result<Self, InvalidSize> {
        if width == 0 || height == 0 {
            Err(InvalidSize { width, height })
        } else {
            Ok(Self { width, height })
        }
    }
}

/// UI element identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ElementId(pub String);

impl ElementId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// Viewport bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}
```

---

## Error Types

```rust
/// Validation error when checking UI constraints.
#[derive(Debug, Clone, Display, Error)]
pub enum VerificationError {
    /// Element missing required label.
    #[display("Element {} missing label", element_id)]
    MissingLabel {
        element_id: NodeId,
        role: Role,
    },

    /// Element too small for touch target.
    #[display("Element {} too small: {}x{}, minimum 44x44", element_id, width, height)]
    TargetTooSmall {
        element_id: NodeId,
        width: u32,
        height: u32,
    },

    /// Element overflows viewport.
    #[display("Element {} overflows viewport: right={}, viewport_width={}", element_id, right, viewport_width)]
    Overflow {
        element_id: NodeId,
        right: u32,
        viewport_width: u32,
    },

    /// Invalid ARIA role.
    #[display("Element {} has invalid role: {:?}", element_id, role)]
    InvalidRole {
        element_id: NodeId,
        role: Role,
    },

    /// Element not keyboard accessible.
    #[display("Element {} not keyboard accessible", element_id)]
    NotKeyboardAccessible {
        element_id: NodeId,
    },

    /// AccessKit tree error.
    #[display("Tree error: {}", _0)]
    TreeError(String),
}

/// Report of all verification issues found.
#[derive(Debug, Clone)]
pub struct VerificationReport {
    pub errors: Vec<VerificationError>,
    pub warnings: Vec<String>,
}

impl VerificationReport {
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}
```

---

## Validation Functions (Establish Proofs)

### HasLabel Validation

```rust
fn validate_has_label(
    tree: &accesskit::Tree,
    node_id: NodeId,
) -> Result<Established<HasLabel<Node>>, VerificationError> {
    let node = tree.get_node(node_id)
        .ok_or_else(|| VerificationError::TreeError(format!("Node {} not found", node_id)))?;

    // Interactive elements must have labels
    if requires_label(node.role) {
        if let Some(name) = &node.name {
            if !name.is_empty() {
                Ok(Established::assert())
            } else {
                Err(VerificationError::MissingLabel {
                    element_id: node_id,
                    role: node.role,
                })
            }
        } else {
            Err(VerificationError::MissingLabel {
                element_id: node_id,
                role: node.role,
            })
        }
    } else {
        // Non-interactive elements don't need labels
        Ok(Established::assert())
    }
}

fn requires_label(role: Role) -> bool {
    matches!(role,
        Role::Button |
        Role::TextField |
        Role::CheckBox |
        Role::RadioButton |
        Role::Link |
        Role::MenuItem
    )
}
```

### MinTargetSize Validation

```rust
fn validate_min_target_size(
    tree: &accesskit::Tree,
    node_id: NodeId,
    touch_device: bool,
) -> Result<Established<MinTargetSize<Node>>, VerificationError> {
    let node = tree.get_node(node_id)
        .ok_or_else(|| VerificationError::TreeError(format!("Node {} not found", node_id)))?;

    // Only check touch targets on touch devices
    if !touch_device {
        return Ok(Established::assert());
    }

    // Interactive elements must meet minimum size
    if requires_touch_target(node.role) {
        if let Some(bounds) = &node.bounds {
            let width = bounds.width();
            let height = bounds.height();

            if width >= 44.0 && height >= 44.0 {
                Ok(Established::assert())
            } else {
                Err(VerificationError::TargetTooSmall {
                    element_id: node_id,
                    width: width as u32,
                    height: height as u32,
                })
            }
        } else {
            Err(VerificationError::TreeError(format!("Node {} missing bounds", node_id)))
        }
    } else {
        Ok(Established::assert())
    }
}

fn requires_touch_target(role: Role) -> bool {
    matches!(role,
        Role::Button |
        Role::CheckBox |
        Role::RadioButton |
        Role::Link |
        Role::MenuItem
    )
}
```

### NoOverflow Validation

```rust
fn validate_no_overflow(
    tree: &accesskit::Tree,
    node_id: NodeId,
    viewport: Viewport,
) -> Result<Established<NoOverflow<Node>>, VerificationError> {
    let node = tree.get_node(node_id)
        .ok_or_else(|| VerificationError::TreeError(format!("Node {} not found", node_id)))?;

    if let Some(bounds) = &node.bounds {
        let right = bounds.x1;
        let bottom = bounds.y1;

        if right <= viewport.width as f64 && bottom <= viewport.height as f64 {
            Ok(Established::assert())
        } else if right > viewport.width as f64 {
            Err(VerificationError::Overflow {
                element_id: node_id,
                right: right as u32,
                viewport_width: viewport.width,
            })
        } else {
            Err(VerificationError::Overflow {
                element_id: node_id,
                right: bottom as u32,
                viewport_width: viewport.height,
            })
        }
    } else {
        // No bounds means element is not rendered (e.g., invisible)
        Ok(Established::assert())
    }
}
```

### ValidRole Validation

```rust
fn validate_valid_role(
    tree: &accesskit::Tree,
    node_id: NodeId,
) -> Result<Established<ValidRole<Node>>, VerificationError> {
    let node = tree.get_node(node_id)
        .ok_or_else(|| VerificationError::TreeError(format!("Node {} not found", node_id)))?;

    if node.role == Role::Unknown {
        Err(VerificationError::InvalidRole {
            element_id: node_id,
            role: node.role,
        })
    } else {
        Ok(Established::assert())
    }
}
```

### KeyboardAccessible Validation

```rust
fn validate_keyboard_accessible(
    tree: &accesskit::Tree,
    node_id: NodeId,
) -> Result<Established<KeyboardAccessible<Node>>, VerificationError> {
    let node = tree.get_node(node_id)
        .ok_or_else(|| VerificationError::TreeError(format!("Node {} not found", node_id)))?;

    // Interactive elements must support focus
    if requires_keyboard_access(node.role) {
        if node.actions.contains(Action::Focus) {
            Ok(Established::assert())
        } else {
            Err(VerificationError::NotKeyboardAccessible {
                element_id: node_id,
            })
        }
    } else {
        Ok(Established::assert())
    }
}

fn requires_keyboard_access(role: Role) -> bool {
    matches!(role,
        Role::Button |
        Role::TextField |
        Role::CheckBox |
        Role::RadioButton |
        Role::Link |
        Role::MenuItem
    )
}
```

---

## State Transitions

### Layout<Pending> - Initial State

```rust
impl Layout<Pending> {
    /// Creates a new pending layout from AccessKit tree.
    pub fn new(tree: accesskit::Tree, root: NodeId) -> Self {
        Self {
            tree,
            root,
            state_data: StateData::Pending(PendingData),
            _state: PhantomData,
        }
    }

    /// Verifies the layout against WCAG Level A constraints.
    pub fn verify_a(
        self,
        viewport: Viewport,
    ) -> Result<Layout<Verified>, VerificationReport> {
        let mut errors = Vec::new();

        // Walk tree and validate each node
        for node_id in walk_tree(&self.tree, self.root) {
            // Validate HasLabel
            if let Err(e) = validate_has_label(&self.tree, node_id) {
                errors.push(e);
            }

            // Validate ValidRole
            if let Err(e) = validate_valid_role(&self.tree, node_id) {
                errors.push(e);
            }

            // Validate KeyboardAccessible
            if let Err(e) = validate_keyboard_accessible(&self.tree, node_id) {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(Layout {
                tree: self.tree,
                root: self.root,
                state_data: StateData::Verified(VerifiedData {
                    viewport,
                    constraint_report: ConstraintReport::satisfied(),
                    wcag_level: WCAGLevel::A,
                }),
                _state: PhantomData,
            })
        } else {
            Err(VerificationReport {
                errors,
                warnings: Vec::new(),
            })
        }
    }

    /// Verifies the layout against WCAG Level AA constraints.
    pub fn verify_aa(
        self,
        viewport: Viewport,
        touch_device: bool,
    ) -> Result<Layout<Verified>, VerificationReport> {
        let mut errors = Vec::new();

        // Walk tree and validate each node
        for node_id in walk_tree(&self.tree, self.root) {
            // Level A checks
            if let Err(e) = validate_has_label(&self.tree, node_id) {
                errors.push(e);
            }
            if let Err(e) = validate_valid_role(&self.tree, node_id) {
                errors.push(e);
            }
            if let Err(e) = validate_keyboard_accessible(&self.tree, node_id) {
                errors.push(e);
            }

            // Level AA checks
            if let Err(e) = validate_min_target_size(&self.tree, node_id, touch_device) {
                errors.push(e);
            }
            if let Err(e) = validate_no_overflow(&self.tree, node_id, viewport) {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(Layout {
                tree: self.tree,
                root: self.root,
                state_data: StateData::Verified(VerifiedData {
                    viewport,
                    constraint_report: ConstraintReport::satisfied(),
                    wcag_level: WCAGLevel::AA,
                }),
                _state: PhantomData,
            })
        } else {
            Err(VerificationReport {
                errors,
                warnings: Vec::new(),
            })
        }
    }
}
```

### Layout<Verified> - Ready to Render

```rust
impl Layout<Verified> {
    /// Returns the viewport used during verification.
    pub fn viewport(&self) -> Viewport {
        match &self.state_data {
            StateData::Verified(data) => data.viewport,
            _ => unreachable!("Verified layout must have VerifiedData"),
        }
    }

    /// Returns the WCAG level satisfied.
    pub fn wcag_level(&self) -> WCAGLevel {
        match &self.state_data {
            StateData::Verified(data) => data.wcag_level,
            _ => unreachable!("Verified layout must have VerifiedData"),
        }
    }

    /// Renders the layout to egui.
    #[cfg(feature = "egui")]
    pub fn render_egui(self, ctx: &egui::Context) -> Layout<Rendered> {
        let handle = render_to_egui(&self.tree, self.root, ctx);

        Layout {
            tree: self.tree,
            root: self.root,
            state_data: StateData::Rendered(RenderedData {
                frontend_handle: Box::new(handle),
                frontend: Frontend::Egui,
            }),
            _state: PhantomData,
        }
    }

    /// Renders the layout to leptos.
    #[cfg(feature = "leptos")]
    pub fn render_leptos(self) -> Layout<Rendered> {
        let handle = render_to_leptos(&self.tree, self.root);

        Layout {
            tree: self.tree,
            root: self.root,
            state_data: StateData::Rendered(RenderedData {
                frontend_handle: Box::new(handle),
                frontend: Frontend::Leptos,
            }),
            _state: PhantomData,
        }
    }

    /// Renders the layout to ratatui.
    #[cfg(feature = "ratatui")]
    pub fn render_ratatui(self, terminal: &mut Terminal<impl Backend>) -> Layout<Rendered> {
        let handle = render_to_ratatui(&self.tree, self.root, terminal);

        Layout {
            tree: self.tree,
            root: self.root,
            state_data: StateData::Rendered(RenderedData {
                frontend_handle: Box::new(handle),
                frontend: Frontend::Ratatui,
            }),
            _state: PhantomData,
        }
    }
}
```

### Layout<Rendered> - Terminal State

```rust
impl Layout<Rendered> {
    /// Returns the frontend used for rendering.
    pub fn frontend(&self) -> Frontend {
        match &self.state_data {
            StateData::Rendered(data) => data.frontend,
            _ => unreachable!("Rendered layout must have RenderedData"),
        }
    }

    /// Returns a reference to the frontend-specific handle.
    pub fn handle(&self) -> &dyn Any {
        match &self.state_data {
            StateData::Rendered(data) => &*data.frontend_handle,
            _ => unreachable!("Rendered layout must have RenderedData"),
        }
    }
}
```

---

## Frontend Rendering

### Egui Renderer

```rust
// elicit_ui/src/frontends/egui.rs

pub fn render_to_egui(
    tree: &accesskit::Tree,
    root: NodeId,
    ctx: &egui::Context,
) -> egui::Response {
    render_node(tree, root, ctx)
}

fn render_node(
    tree: &accesskit::Tree,
    node_id: NodeId,
    ui: &mut egui::Ui,
) -> egui::Response {
    let node = tree.get_node(node_id).expect("Node must exist in verified tree");

    match node.role {
        Role::Button => {
            let label = node.name.as_deref().unwrap_or("Button");
            ui.button(label)
        }
        Role::TextField => {
            let label = node.name.as_deref().unwrap_or("Text");
            let mut text = String::new(); // Would need state management
            ui.text_edit_singleline(&mut text)
        }
        Role::GenericContainer => {
            // Render children in vertical layout
            ui.vertical(|ui| {
                for child_id in &node.children {
                    render_node(tree, *child_id, ui);
                }
            }).response
        }
        _ => {
            ui.label(node.name.as_deref().unwrap_or(""))
        }
    }
}
```

### Leptos Renderer

```rust
// elicit_ui/src/frontends/leptos.rs

pub fn render_to_leptos(
    tree: &accesskit::Tree,
    root: NodeId,
) -> leptos::View {
    render_node(tree, root)
}

fn render_node(
    tree: &accesskit::Tree,
    node_id: NodeId,
) -> leptos::View {
    let node = tree.get_node(node_id).expect("Node must exist in verified tree");

    match node.role {
        Role::Button => {
            let label = node.name.clone().unwrap_or_else(|| "Button".to_string());
            view! {
                <button>{label}</button>
            }
        }
        Role::TextField => {
            let label = node.name.clone().unwrap_or_else(|| "Text".to_string());
            view! {
                <input type="text" placeholder={label} />
            }
        }
        Role::GenericContainer => {
            let children: Vec<_> = node.children.iter()
                .map(|child_id| render_node(tree, *child_id))
                .collect();
            view! {
                <div>{children}</div>
            }
        }
        _ => {
            let text = node.name.clone().unwrap_or_default();
            view! {
                <span>{text}</span>
            }
        }
    }
}
```

### Ratatui Renderer

```rust
// elicit_ui/src/frontends/ratatui.rs

pub fn render_to_ratatui(
    tree: &accesskit::Tree,
    root: NodeId,
    terminal: &mut Terminal<impl Backend>,
) -> std::io::Result<()> {
    terminal.draw(|f| {
        render_node(tree, root, f, f.size())
    })?;
    Ok(())
}

fn render_node(
    tree: &accesskit::Tree,
    node_id: NodeId,
    frame: &mut Frame,
    area: Rect,
) {
    let node = tree.get_node(node_id).expect("Node must exist in verified tree");

    match node.role {
        Role::Button => {
            let label = node.name.as_deref().unwrap_or("Button");
            let block = Block::default()
                .borders(Borders::ALL)
                .title(label);
            frame.render_widget(block, area);
        }
        Role::TextField => {
            let label = node.name.as_deref().unwrap_or("Text");
            let paragraph = Paragraph::new(label)
                .block(Block::default().borders(Borders::ALL));
            frame.render_widget(paragraph, area);
        }
        Role::GenericContainer => {
            // Split area for children
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(100 / node.children.len() as u16); node.children.len()])
                .split(area);

            for (i, child_id) in node.children.iter().enumerate() {
                render_node(tree, *child_id, frame, layout[i]);
            }
        }
        _ => {
            let text = node.name.as_deref().unwrap_or("");
            let paragraph = Paragraph::new(text);
            frame.render_widget(paragraph, area);
        }
    }
}
```

---

## Implementation Plan

### Phase 1: Core Typestate (1 week)

**Goal:** Prove the pattern works with minimal implementation

**Files:**
- `crates/elicit_ui/src/lib.rs` - Module root
- `crates/elicit_ui/src/types.rs` - Label, Size, ElementId, Viewport
- `crates/elicit_ui/src/errors.rs` - VerificationError, VerificationReport
- `crates/elicit_ui/src/contracts.rs` - HasLabel, ValidRole, KeyboardAccessible
- `crates/elicit_ui/src/typestate.rs` - Layout<S>, verify_a()
- `crates/elicit_ui/src/frontends/egui.rs` - render_to_egui()

**Test:**
- `crates/elicit_ui/tests/button_test.rs`
  - Create button in AccessKit tree
  - Verify with `verify_a()`
  - Render to egui
  - Verify proof composition works

**Success Criteria:**
```rust
let tree = create_button_tree("Submit");
let layout = Layout::new(tree, button_id);
let verified = layout.verify_a(Viewport { width: 1024, height: 768 })?;
let rendered = verified.render_egui(&ctx);
```

### Phase 2: Constraint Verification (1 week)

**Goal:** Add size and overflow constraints

**Files:**
- `crates/elicit_ui/src/constraints.rs` - MinTargetSize, NoOverflow propositions
- `crates/elicit_ui/src/typestate.rs` - Add `verify_aa()`

**Test:**
- `crates/elicit_ui/tests/layout_test.rs`
  - Create complex layout (column with buttons)
  - Verify viewport overflow detection
  - Verify touch target size validation
  - Test failure cases

**Success Criteria:**
```rust
// Should succeed
let layout = create_large_button_layout();
let verified = layout.verify_aa(Viewport { width: 1024, height: 768 }, true)?;

// Should fail: button too small
let tiny = create_small_button();
let result = layout.verify_aa(Viewport { width: 1024, height: 768 }, true);
assert!(result.is_err());
```

### Phase 3: Multiple Frontends (1 week)

**Goal:** Prove frontend-agnostic design

**Files:**
- `crates/elicit_ui/src/frontends/leptos.rs` - Leptos renderer
- `crates/elicit_ui/src/frontends/ratatui.rs` - Ratatui renderer

**Test:**
- `crates/elicit_ui/tests/frontend_test.rs`
  - Verify same AccessKit tree
  - Render to egui
  - Render to leptos
  - Render to ratatui
  - Verify all produce correct output

**Success Criteria:**
```rust
let tree = create_button_tree("Submit");
let layout = Layout::new(tree, button_id);
let verified = layout.verify_aa(viewport, true)?;

// Same verified tree → different frontends
let egui_rendered = verified.clone().render_egui(&ctx);
let leptos_rendered = verified.clone().render_leptos();
let ratatui_rendered = verified.render_ratatui(&mut terminal);
```

### Phase 4: Builder DSL (Optional)

**Goal:** Ergonomic UI construction

**Files:**
- `crates/elicit_ui_macros/src/lib.rs` - `ui!` macro

**Test:**
- `crates/elicit_ui/tests/dsl_test.rs`

**Example:**
```rust
let layout = ui! {
    column {
        button("Submit").on_click(submit_handler),
        input("Email"),
        row {
            button("Cancel"),
            button("OK"),
        }
    }
}
.verify_aa(viewport, touch_device)?
.render_egui(&ctx);
```

---

## WCAG Compliance Mapping

### Level A (Basic)

| WCAG Criterion | Proposition | Validation |
|----------------|-------------|------------|
| 1.1.1 Non-text Content | `HasLabel<T>` | Interactive elements have text alternative |
| 2.1.1 Keyboard | `KeyboardAccessible<T>` | All functionality available via keyboard |
| 4.1.2 Name, Role, Value | `ValidRole<T>` | All elements have valid ARIA roles |

### Level AA (Enhanced)

| WCAG Criterion | Proposition | Validation |
|----------------|-------------|------------|
| 2.5.5 Target Size | `MinTargetSize<T>` | Touch targets ≥ 44x44 pixels |
| 1.4.10 Reflow | `NoOverflow<T>` | Content fits viewport at 320px width |

### Not Covered (Require Runtime)

- **Contrast ratios** (1.4.3) - Need actual colors, not structure
- **Timing** (2.2.*) - Need runtime behavior
- **Seizures** (2.3.*) - Need animation analysis
- **Navigation order** (2.4.3) - Need actual tab order

These can be added as **deferred obligations** in verification report.

---

## Benefits

### Compared to Traditional UI

**Traditional approach:**
```rust
struct UI {
    elements: Vec<Element>,
    verified: bool,  // Runtime flag
}

// ❌ Can render without verification
ui.verified = false;
render(ui);
```

**Typestate approach:**
```rust
struct Layout<Pending> { ... }
struct Layout<Verified> { ... }
struct Layout<Rendered> { ... }

// ✅ Compiler enforces verification before render
let pending: Layout<Pending> = create_layout();
pending.render_egui(&ctx); // ERROR: no method `render_egui`

let verified = pending.verify_aa(viewport, true)?;
verified.render_egui(&ctx); // OK
```

### Universal IR Benefits

**Same verification, multiple frontends:**
```rust
// Verify once
let verified = layout.verify_aa(viewport, true)?;

// Render to any frontend
match target {
    Target::Desktop => verified.render_egui(&ctx),
    Target::Web => verified.render_leptos(),
    Target::Terminal => verified.render_ratatui(&mut term),
}
```

**Frontend changes don't affect verification:**
- Add new frontend: implement renderer, verification unchanged
- Update egui: change renderer, verification unchanged
- AccessKit tree is stable contract

---

## Comparison to Ledger

| Aspect | Ledger | UI |
|--------|--------|-----|
| **Domain** | Money transfers | Interactive layouts |
| **States** | Pending → Validated → Committed | Pending → Verified → Rendered |
| **Universal IR** | SQL schema | AccessKit tree |
| **Propositions** | AmountPositive, SufficientFunds | HasLabel, MinTargetSize |
| **Composite** | ValidTransfer | AccessibleAA |
| **Validation** | Query balance, check constraints | Walk tree, check WCAG |
| **Backends** | SQLite, PostgreSQL, MySQL | egui, leptos, ratatui |
| **Invariant** | Σ debits = Σ credits | All interactive elements accessible |
| **Proof cost** | Zero (PhantomData) | Zero (PhantomData) |
| **Verification** | Kani/Verus/Creusot | Kani/Verus/Creusot |

**Same pattern, different domain.**

---

## Related Documentation

- [TYPESTATE_LEDGER_DESIGN.md](TYPESTATE_LEDGER_DESIGN.md) - Reference implementation
- [elicitation contracts](crates/elicitation/src/contracts.rs) - Proof framework
- [AccessKit documentation](https://docs.rs/accesskit/) - Universal IR
- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/) - Accessibility criteria

---

## Next Steps

1. Implement Phase 1 (core typestate with egui)
2. Add Kani harnesses for proposition proofs
3. Integrate with Phase 2 (constraint verification)
4. Document typestate guarantees
5. Add Creusot/Verus proofs (if time permits)
6. Extend to leptos and ratatui (Phase 3)

---

## Open Questions

1. **State management**: How do we handle stateful widgets (text input values)?
   - **Answer**: Layout is pure structure, state is external (like React)

2. **Event handlers**: How do we attach callbacks?
   - **Answer**: AccessKit actions define capabilities, handlers are frontend-specific

3. **Styling**: How do we handle colors, fonts?
   - **Answer**: Separate from verification (structure first, style second)

4. **Dynamic layouts**: How do we handle conditional rendering?
   - **Answer**: Build new AccessKit tree, re-verify (like database transactions)

5. **Performance**: Is tree walking too slow?
   - **Answer**: Verification is one-time (like ledger validation), rendering is separate

---

## License

Same as parent crate (elicitation).
