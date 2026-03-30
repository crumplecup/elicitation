//! Ergonomic builder for constructing AccessKit trees.
//!
//! Eliminates manual `NodeId` management, parent-child wiring, and
//! `TreeUpdate` boilerplate. Produces a `Layout<Pending>` ready for
//! WCAG verification.
//!
//! # Example
//!
//! ```rust
//! use elicit_ui::LayoutBuilder;
//!
//! let layout = LayoutBuilder::new()
//!     .button("Submit").size(100, 50)
//!     .checkbox("Accept terms")
//!     .form()
//!         .text_input("Email").placeholder("you@example.com")
//!         .text_input("Password")
//!         .button("Log in").size(120, 44)
//!     .end()
//!     .build();
//! ```

use crate::{Layout, Pending};
use accesskit::{Node, NodeId, Rect, Role, Toggled, Tree, TreeId, TreeUpdate};

/// Stack-based builder for AccessKit trees.
///
/// Auto-generates `NodeId`s, tracks parent-child relationships via
/// an internal container stack, and produces a `Layout<Pending>`.
pub struct LayoutBuilder {
    next_id: u64,
    nodes: Vec<(NodeId, Node)>,
    /// Stack of (container_node_id, children_ids).
    /// The bottom of the stack is always the root window.
    stack: Vec<(NodeId, Vec<NodeId>)>,
    /// ID of the most recently added node (for property setters).
    last_id: Option<NodeId>,
}

impl LayoutBuilder {
    /// Create a new builder with a root `Window` container.
    #[tracing::instrument]
    pub fn new() -> Self {
        let root_id = NodeId::from(0u64);
        let root = Node::new(Role::Window);

        Self {
            next_id: 1,
            nodes: vec![(root_id, root)],
            stack: vec![(root_id, Vec::new())],
            last_id: None,
        }
    }

    /// Finalize and produce a `Layout<Pending>`.
    ///
    /// Closes all open containers and creates the underlying `TreeUpdate`.
    /// After calling `build`, the builder is reset to its initial state.
    ///
    /// # Panics
    ///
    /// Panics if the builder is in an invalid state (should not happen
    /// with normal usage).
    #[tracing::instrument(skip(self))]
    pub fn build(&mut self) -> Layout<Pending> {
        // Close all open containers
        while self.stack.len() > 1 {
            self.close_container();
        }

        // Finalize root
        let (root_id, children) = self.stack.pop().expect("root must exist");
        self.set_children(root_id, children);

        let nodes = std::mem::take(&mut self.nodes);
        let update = TreeUpdate {
            nodes,
            tree: Some(Tree::new(root_id)),
            tree_id: TreeId::ROOT,
            focus: root_id,
        };

        // Reset builder to initial state
        *self = Self::new();

        Layout::from_update(update)
    }

    // ── Leaf widgets ────────────────────────────────────────

    /// Add a button with the given label.
    pub fn button(&mut self, label: &str) -> &mut Self {
        self.add_leaf(Role::Button, label)
    }

    /// Add a text input with the given label.
    pub fn text_input(&mut self, label: &str) -> &mut Self {
        self.add_leaf(Role::TextInput, label)
    }

    /// Add a multiline text input with the given label.
    pub fn multiline_input(&mut self, label: &str) -> &mut Self {
        self.add_leaf(Role::MultilineTextInput, label)
    }

    /// Add a password input with the given label.
    pub fn password_input(&mut self, label: &str) -> &mut Self {
        self.add_leaf(Role::PasswordInput, label)
    }

    /// Add a search input with the given label.
    pub fn search_input(&mut self, label: &str) -> &mut Self {
        self.add_leaf(Role::SearchInput, label)
    }

    /// Add a checkbox with the given label (unchecked by default).
    pub fn checkbox(&mut self, label: &str) -> &mut Self {
        let id = self.alloc_id();
        let mut node = Node::new(Role::CheckBox);
        node.set_label(label);
        node.set_toggled(Toggled::False);
        self.push_node(id, node);
        self
    }

    /// Add a radio button with the given label (unselected by default).
    pub fn radio(&mut self, label: &str) -> &mut Self {
        let id = self.alloc_id();
        let mut node = Node::new(Role::RadioButton);
        node.set_label(label);
        node.set_toggled(Toggled::False);
        self.push_node(id, node);
        self
    }

    /// Add a switch with the given label (off by default).
    pub fn switch(&mut self, label: &str) -> &mut Self {
        let id = self.alloc_id();
        let mut node = Node::new(Role::Switch);
        node.set_label(label);
        node.set_toggled(Toggled::False);
        self.push_node(id, node);
        self
    }

    /// Add a slider with label, value, min, and max.
    pub fn slider(&mut self, label: &str, value: f64, min: f64, max: f64) -> &mut Self {
        let id = self.alloc_id();
        let mut node = Node::new(Role::Slider);
        node.set_label(label);
        node.set_numeric_value(value);
        node.set_min_numeric_value(min);
        node.set_max_numeric_value(max);
        self.push_node(id, node);
        self
    }

    /// Add a progress indicator with label, value, and max.
    pub fn progress(&mut self, label: &str, value: f64, max: f64) -> &mut Self {
        let id = self.alloc_id();
        let mut node = Node::new(Role::ProgressIndicator);
        node.set_label(label);
        node.set_numeric_value(value);
        node.set_max_numeric_value(max);
        self.push_node(id, node);
        self
    }

    /// Add a text label.
    pub fn label(&mut self, text: &str) -> &mut Self {
        let id = self.alloc_id();
        let mut node = Node::new(Role::Label);
        node.set_value(text);
        self.push_node(id, node);
        self
    }

    /// Add a heading with text and level (1-6).
    pub fn heading(&mut self, text: &str, level: usize) -> &mut Self {
        let id = self.alloc_id();
        let mut node = Node::new(Role::Heading);
        node.set_value(text);
        node.set_level(level);
        self.push_node(id, node);
        self
    }

    /// Add a hyperlink with label and URL.
    pub fn link(&mut self, label: &str, url: &str) -> &mut Self {
        let id = self.alloc_id();
        let mut node = Node::new(Role::Link);
        node.set_label(label);
        node.set_url(url);
        self.push_node(id, node);
        self
    }

    /// Add a separator.
    pub fn separator(&mut self) -> &mut Self {
        let id = self.alloc_id();
        let node = Node::new(Role::Splitter);
        self.push_node(id, node);
        self
    }

    /// Add an image placeholder with alt text.
    pub fn image(&mut self, alt: &str) -> &mut Self {
        let id = self.alloc_id();
        let mut node = Node::new(Role::Image);
        node.set_label(alt);
        self.push_node(id, node);
        self
    }

    // ── Container widgets ───────────────────────────────────

    /// Open a `Form` container. Close with `.end()`.
    pub fn form(&mut self) -> &mut Self {
        self.open_container(Role::Form)
    }

    /// Open a `Group` container. Close with `.end()`.
    pub fn group(&mut self) -> &mut Self {
        self.open_container(Role::Group)
    }

    /// Open a `Toolbar` container. Close with `.end()`.
    pub fn toolbar(&mut self) -> &mut Self {
        self.open_container(Role::Toolbar)
    }

    /// Open a `List` container. Close with `.end()`.
    pub fn list(&mut self) -> &mut Self {
        self.open_container(Role::List)
    }

    /// Open a `Navigation` container. Close with `.end()`.
    pub fn navigation(&mut self) -> &mut Self {
        self.open_container(Role::Navigation)
    }

    /// Open a `Section` container. Close with `.end()`.
    pub fn section(&mut self) -> &mut Self {
        self.open_container(Role::Section)
    }

    /// Open a `Dialog` container. Close with `.end()`.
    pub fn dialog(&mut self) -> &mut Self {
        self.open_container(Role::Dialog)
    }

    /// Close the current container and return to its parent.
    ///
    /// # Panics
    ///
    /// Panics if called when only the root container is open.
    pub fn end(&mut self) -> &mut Self {
        assert!(
            self.stack.len() > 1,
            "end() called with no open container to close"
        );
        self.close_container();
        self
    }

    // ── Property setters (apply to last-added node) ─────────

    /// Set bounds on the last-added node.
    pub fn bounds(&mut self, x0: f64, y0: f64, x1: f64, y1: f64) -> &mut Self {
        if let Some(id) = self.last_id {
            self.mutate_node(id, |n| n.set_bounds(Rect { x0, y0, x1, y1 }));
        }
        self
    }

    /// Set size (as bounds from origin) on the last-added node.
    pub fn size(&mut self, width: u32, height: u32) -> &mut Self {
        self.bounds(0.0, 0.0, f64::from(width), f64::from(height))
    }

    /// Set placeholder text on the last-added node.
    pub fn placeholder(&mut self, text: &str) -> &mut Self {
        if let Some(id) = self.last_id {
            self.mutate_node(id, |n| n.set_placeholder(text));
        }
        self
    }

    /// Set the value on the last-added node.
    pub fn value(&mut self, val: &str) -> &mut Self {
        if let Some(id) = self.last_id {
            self.mutate_node(id, |n| n.set_value(val));
        }
        self
    }

    /// Mark the last-added node as disabled.
    pub fn disabled(&mut self) -> &mut Self {
        if let Some(id) = self.last_id {
            self.mutate_node(id, |n| n.set_disabled());
        }
        self
    }

    /// Mark the last-added node as read-only.
    pub fn read_only(&mut self) -> &mut Self {
        if let Some(id) = self.last_id {
            self.mutate_node(id, |n| n.set_read_only());
        }
        self
    }

    /// Set toggled state on the last-added node.
    pub fn checked(&mut self, val: bool) -> &mut Self {
        if let Some(id) = self.last_id {
            self.mutate_node(id, |n| n.set_toggled(Toggled::from(val)));
        }
        self
    }

    /// Set numeric value, min, and max on the last-added node.
    pub fn numeric(&mut self, value: f64, min: f64, max: f64) -> &mut Self {
        if let Some(id) = self.last_id {
            self.mutate_node(id, |n| {
                n.set_numeric_value(value);
                n.set_min_numeric_value(min);
                n.set_max_numeric_value(max);
            });
        }
        self
    }

    // ── Internal helpers ────────────────────────────────────

    fn alloc_id(&mut self) -> NodeId {
        let id = NodeId::from(self.next_id);
        self.next_id += 1;
        id
    }

    fn add_leaf(&mut self, role: Role, label: &str) -> &mut Self {
        let id = self.alloc_id();
        let mut node = Node::new(role);
        node.set_label(label);
        self.push_node(id, node);
        self
    }

    fn push_node(&mut self, id: NodeId, node: Node) {
        self.nodes.push((id, node));
        if let Some((_container_id, children)) = self.stack.last_mut() {
            children.push(id);
        }
        self.last_id = Some(id);
    }

    fn open_container(&mut self, role: Role) -> &mut Self {
        let id = self.alloc_id();
        let node = Node::new(role);
        self.nodes.push((id, node));
        if let Some((_container_id, children)) = self.stack.last_mut() {
            children.push(id);
        }
        self.stack.push((id, Vec::new()));
        self.last_id = Some(id);
        self
    }

    fn close_container(&mut self) {
        let (container_id, children) = self.stack.pop().expect("stack underflow");
        self.set_children(container_id, children);
    }

    fn set_children(&mut self, node_id: NodeId, children: Vec<NodeId>) {
        if let Some((_id, node)) = self.nodes.iter_mut().find(|(id, _)| *id == node_id) {
            node.set_children(children);
        }
    }

    fn mutate_node(&mut self, node_id: NodeId, f: impl FnOnce(&mut Node)) {
        if let Some((_id, node)) = self.nodes.iter_mut().find(|(id, _)| *id == node_id) {
            f(node);
        }
    }
}

impl Default for LayoutBuilder {
    fn default() -> Self {
        Self::new()
    }
}
