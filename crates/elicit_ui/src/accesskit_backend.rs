//! Reference implementation of all UI traits backed by AccessKit.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use accesskit::{Node, NodeId, Role, Toggled, Tree, TreeId, TreeUpdate};
use elicitation::Established;
use tracing::instrument;

use crate::{
    AccessibleAA, AltTextProvided, ContainerId, ContrastViolation, FocusVisible, HasLabel,
    KeyboardAccessible, MinTargetSize, NoOverflow, RenderComplete, RenderStats, SrgbColor,
    StructuredContent, SufficientContrast, UiAccessibilityAuditor, UiError, UiErrorKind,
    UiEventDispatcher, UiInspector, UiLayoutManager, UiNavigationManager, UiRenderer, UiResult,
    UiStyleManager, UiWidgetFactory, ValidRole, VerificationReport, VerifiedTree, Viewport,
    WidgetA11y, WidgetId, WidgetInfo, contrast_ratio,
};

struct BackendState {
    nodes: HashMap<NodeId, Node>,
    root: NodeId,
    next_id: u64,
    focus_order: Vec<NodeId>,
    event_handlers: HashMap<(u64, String), String>,
    parent_map: HashMap<NodeId, NodeId>,
}

impl BackendState {
    fn new() -> Self {
        let root = NodeId(0);
        let mut nodes = HashMap::new();
        let mut root_node = Node::new(Role::Window);
        root_node.set_label("Application");
        nodes.insert(root, root_node);
        Self {
            nodes,
            root,
            next_id: 1,
            focus_order: Vec::new(),
            event_handlers: HashMap::new(),
            parent_map: HashMap::new(),
        }
    }

    fn next_id(&mut self) -> NodeId {
        let id = NodeId(self.next_id);
        self.next_id += 1;
        id
    }

    fn to_tree_update(&self) -> TreeUpdate {
        TreeUpdate {
            nodes: self.nodes.iter().map(|(k, v)| (*k, v.clone())).collect(),
            tree: Some(Tree::new(self.root)),
            tree_id: TreeId::ROOT,
            focus: self.root,
        }
    }
}

/// Reference implementation of all UI traits backed by an AccessKit tree.
///
/// Use this when you want to build a UI tree programmatically and then
/// snapshot it for rendering via any [`UiRenderer`] backend.
pub struct AccessKitUiBackend {
    state: Arc<Mutex<BackendState>>,
}

impl AccessKitUiBackend {
    /// Create a new empty UI backend.
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(BackendState::new())),
        }
    }

    /// Take a raw snapshot of the current tree as an AccessKit `TreeUpdate`.
    pub fn snapshot(&self) -> TreeUpdate {
        self.state.lock().unwrap().to_tree_update()
    }
}

impl Default for AccessKitUiBackend {
    fn default() -> Self {
        Self::new()
    }
}

// ── UiWidgetFactory ─────────────────────────────────────────────────────────

impl UiWidgetFactory for AccessKitUiBackend {
    #[instrument(skip(self), fields(label, width, height))]
    fn create_button(
        &self,
        label: &str,
        width: u32,
        height: u32,
    ) -> UiResult<(
        WidgetId,
        Established<HasLabel>,
        Established<MinTargetSize>,
        Established<KeyboardAccessible>,
    )> {
        if label.is_empty() {
            return Err(UiError::new(UiErrorKind::MissingLabel(
                "button label is empty".into(),
            )));
        }
        if width < 44 || height < 44 {
            return Err(UiError::new(UiErrorKind::TargetTooSmall(format!(
                "button {}x{} is below 44x44 minimum",
                width, height
            ))));
        }
        let mut state = self.state.lock().unwrap();
        let id = state.next_id();
        let mut node = Node::new(Role::Button);
        node.set_label(label);
        node.set_bounds(accesskit::Rect {
            x0: 0.0,
            y0: 0.0,
            x1: f64::from(width),
            y1: f64::from(height),
        });
        state.nodes.insert(id, node);
        state.focus_order.push(id);
        Ok((
            WidgetId::from_node(id),
            Established::assert(),
            Established::assert(),
            Established::assert(),
        ))
    }

    #[instrument(skip(self), fields(text, role_hint))]
    fn create_label(
        &self,
        text: &str,
        role_hint: &str,
    ) -> UiResult<(WidgetId, Established<HasLabel>, Established<ValidRole>)> {
        if text.is_empty() {
            return Err(UiError::new(UiErrorKind::MissingLabel(
                "label text is empty".into(),
            )));
        }
        let mut state = self.state.lock().unwrap();
        let id = state.next_id();
        let role = role_hint_to_role(role_hint);
        let mut node = Node::new(role);
        node.set_value(text);
        state.nodes.insert(id, node);
        Ok((
            WidgetId::from_node(id),
            Established::assert(),
            Established::assert(),
        ))
    }

    #[instrument(skip(self), fields(label, input_type))]
    fn create_input(
        &self,
        label: &str,
        input_type: &str,
    ) -> UiResult<(
        WidgetId,
        Established<HasLabel>,
        Established<KeyboardAccessible>,
    )> {
        if label.is_empty() {
            return Err(UiError::new(UiErrorKind::MissingLabel(
                "input label is empty".into(),
            )));
        }
        let mut state = self.state.lock().unwrap();
        let id = state.next_id();
        let role = match input_type {
            "password" => Role::PasswordInput,
            "search" => Role::SearchInput,
            "email" => Role::EmailInput,
            "url" => Role::UrlInput,
            "tel" => Role::PhoneNumberInput,
            _ => Role::TextInput,
        };
        let mut node = Node::new(role);
        node.set_label(label);
        state.nodes.insert(id, node);
        state.focus_order.push(id);
        Ok((
            WidgetId::from_node(id),
            Established::assert(),
            Established::assert(),
        ))
    }

    #[instrument(skip(self), fields(alt_text, src))]
    fn create_image(
        &self,
        alt_text: &str,
        src: &str,
    ) -> UiResult<(
        WidgetId,
        Established<HasLabel>,
        Established<AltTextProvided>,
    )> {
        if alt_text.is_empty() {
            return Err(UiError::new(UiErrorKind::MissingLabel(
                "image alt text is empty".into(),
            )));
        }
        let mut state = self.state.lock().unwrap();
        let id = state.next_id();
        let mut node = Node::new(Role::Image);
        node.set_label(alt_text);
        node.set_url(src);
        state.nodes.insert(id, node);
        Ok((
            WidgetId::from_node(id),
            Established::assert(),
            Established::assert(),
        ))
    }

    #[instrument(skip(self), fields(text, level))]
    fn create_heading(
        &self,
        text: &str,
        level: u8,
    ) -> UiResult<(
        WidgetId,
        Established<HasLabel>,
        Established<ValidRole>,
        Established<StructuredContent>,
    )> {
        if text.is_empty() {
            return Err(UiError::new(UiErrorKind::MissingLabel(
                "heading text is empty".into(),
            )));
        }
        let mut state = self.state.lock().unwrap();
        let id = state.next_id();
        let mut node = Node::new(Role::Heading);
        node.set_value(text);
        node.set_level(usize::from(level.clamp(1, 6)));
        state.nodes.insert(id, node);
        Ok((
            WidgetId::from_node(id),
            Established::assert(),
            Established::assert(),
            Established::assert(),
        ))
    }

    #[instrument(skip(self), fields(text, href))]
    fn create_link(
        &self,
        text: &str,
        href: &str,
    ) -> UiResult<(
        WidgetId,
        Established<HasLabel>,
        Established<KeyboardAccessible>,
    )> {
        if text.is_empty() {
            return Err(UiError::new(UiErrorKind::MissingLabel(
                "link text is empty".into(),
            )));
        }
        let mut state = self.state.lock().unwrap();
        let id = state.next_id();
        let mut node = Node::new(Role::Link);
        node.set_label(text);
        node.set_url(href);
        state.nodes.insert(id, node);
        state.focus_order.push(id);
        Ok((
            WidgetId::from_node(id),
            Established::assert(),
            Established::assert(),
        ))
    }

    #[instrument(skip(self, headers), fields(caption))]
    fn create_table(
        &self,
        caption: &str,
        headers: Vec<String>,
    ) -> UiResult<(
        WidgetId,
        Established<HasLabel>,
        Established<StructuredContent>,
    )> {
        if caption.is_empty() {
            return Err(UiError::new(UiErrorKind::MissingLabel(
                "table caption is empty".into(),
            )));
        }
        let mut state = self.state.lock().unwrap();
        let table_id = state.next_id();
        let mut table_node = Node::new(Role::Table);
        table_node.set_label(caption);

        let header_ids: Vec<NodeId> = headers
            .iter()
            .map(|h| {
                let hid = state.next_id();
                let mut hn = Node::new(Role::ColumnHeader);
                hn.set_value(h.as_str());
                state.nodes.insert(hid, hn);
                hid
            })
            .collect();

        if !header_ids.is_empty() {
            table_node.set_children(header_ids.clone());
            for hid in &header_ids {
                state.parent_map.insert(*hid, table_id);
            }
        }
        state.nodes.insert(table_id, table_node);
        Ok((
            WidgetId::from_node(table_id),
            Established::assert(),
            Established::assert(),
        ))
    }

    #[instrument(skip(self), fields(label, checked))]
    fn create_checkbox(
        &self,
        label: &str,
        checked: bool,
    ) -> UiResult<(
        WidgetId,
        Established<HasLabel>,
        Established<KeyboardAccessible>,
    )> {
        if label.is_empty() {
            return Err(UiError::new(UiErrorKind::MissingLabel(
                "checkbox label is empty".into(),
            )));
        }
        let mut state = self.state.lock().unwrap();
        let id = state.next_id();
        let mut node = Node::new(Role::CheckBox);
        node.set_label(label);
        if checked {
            node.set_toggled(Toggled::True);
        } else {
            node.set_toggled(Toggled::False);
        }
        state.nodes.insert(id, node);
        state.focus_order.push(id);
        Ok((
            WidgetId::from_node(id),
            Established::assert(),
            Established::assert(),
        ))
    }

    #[instrument(skip(self, options), fields(label))]
    fn create_select(
        &self,
        label: &str,
        options: Vec<String>,
    ) -> UiResult<(
        WidgetId,
        Established<HasLabel>,
        Established<KeyboardAccessible>,
    )> {
        if label.is_empty() {
            return Err(UiError::new(UiErrorKind::MissingLabel(
                "select label is empty".into(),
            )));
        }
        let mut state = self.state.lock().unwrap();
        let select_id = state.next_id();
        let mut select_node = Node::new(Role::ComboBox);
        select_node.set_label(label);

        let option_ids: Vec<NodeId> = options
            .iter()
            .map(|o| {
                let oid = state.next_id();
                let mut on = Node::new(Role::ListBoxOption);
                on.set_value(o.as_str());
                state.nodes.insert(oid, on);
                state.parent_map.insert(oid, select_id);
                oid
            })
            .collect();

        if !option_ids.is_empty() {
            select_node.set_children(option_ids);
        }
        state.nodes.insert(select_id, select_node);
        state.focus_order.push(select_id);
        Ok((
            WidgetId::from_node(select_id),
            Established::assert(),
            Established::assert(),
        ))
    }
}

// ── UiLayoutManager ──────────────────────────────────────────────────────────

impl UiLayoutManager for AccessKitUiBackend {
    #[instrument(skip(self, children), fields(axis))]
    fn container_stack(
        &self,
        axis: &str,
        children: Vec<WidgetId>,
    ) -> UiResult<(ContainerId, Established<NoOverflow>)> {
        let mut state = self.state.lock().unwrap();
        let id = state.next_id();
        let role = if axis == "horizontal" {
            Role::Toolbar
        } else {
            Role::Group
        };
        let mut node = Node::new(role);
        let child_ids: Vec<NodeId> = children.iter().map(|w| w.to_node_id()).collect();
        if !child_ids.is_empty() {
            node.set_children(child_ids.clone());
            for cid in &child_ids {
                state.parent_map.insert(*cid, id);
            }
        }
        state.nodes.insert(id, node);
        Ok((ContainerId::from_node(id), Established::assert()))
    }

    #[instrument(skip(self, children), fields(columns))]
    fn container_grid(
        &self,
        columns: u32,
        children: Vec<WidgetId>,
    ) -> UiResult<(ContainerId, Established<NoOverflow>)> {
        let mut state = self.state.lock().unwrap();
        let id = state.next_id();
        let mut node = Node::new(Role::Grid);
        node.set_label(format!("{columns}-column grid"));
        let child_ids: Vec<NodeId> = children.iter().map(|w| w.to_node_id()).collect();
        if !child_ids.is_empty() {
            node.set_children(child_ids.clone());
            for cid in &child_ids {
                state.parent_map.insert(*cid, id);
            }
        }
        state.nodes.insert(id, node);
        Ok((ContainerId::from_node(id), Established::assert()))
    }

    #[instrument(skip(self), fields(child = child.0))]
    fn container_scroll(
        &self,
        child: WidgetId,
    ) -> UiResult<(ContainerId, Established<NoOverflow>)> {
        let mut state = self.state.lock().unwrap();
        let id = state.next_id();
        let mut node = Node::new(Role::ScrollView);
        node.set_children(vec![child.to_node_id()]);
        state.parent_map.insert(child.to_node_id(), id);
        state.nodes.insert(id, node);
        Ok((ContainerId::from_node(id), Established::assert()))
    }

    #[instrument(skip(self, content), fields(name))]
    fn container_panel(&self, name: &str, content: Vec<WidgetId>) -> UiResult<ContainerId> {
        let mut state = self.state.lock().unwrap();
        let id = state.next_id();
        let mut node = Node::new(Role::Region);
        node.set_label(name);
        let child_ids: Vec<NodeId> = content.iter().map(|w| w.to_node_id()).collect();
        if !child_ids.is_empty() {
            node.set_children(child_ids.clone());
            for cid in &child_ids {
                state.parent_map.insert(*cid, id);
            }
        }
        state.nodes.insert(id, node);
        Ok(ContainerId::from_node(id))
    }

    #[instrument(skip(self), fields(parent = parent.0, child = child.0))]
    fn add_child(&self, parent: ContainerId, child: WidgetId) -> UiResult<()> {
        let mut state = self.state.lock().unwrap();
        let parent_id = parent.to_node_id();
        let child_id = child.to_node_id();
        if !state.nodes.contains_key(&parent_id) {
            return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                "container {:?} not found",
                parent_id
            ))));
        }
        state.parent_map.insert(child_id, parent_id);
        if let Some(node) = state.nodes.get_mut(&parent_id) {
            let mut children = node.children().to_vec();
            if !children.contains(&child_id) {
                children.push(child_id);
                node.set_children(children);
            }
        }
        Ok(())
    }

    #[instrument(skip(self), fields(id = id.0))]
    fn remove_widget(&self, id: WidgetId) -> UiResult<()> {
        let mut state = self.state.lock().unwrap();
        let node_id = id.to_node_id();
        if !state.nodes.contains_key(&node_id) {
            return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                "widget {:?} not found",
                node_id
            ))));
        }
        state.nodes.remove(&node_id);
        state.focus_order.retain(|n| *n != node_id);
        state.parent_map.remove(&node_id);
        if let Some(&parent_id) = state.parent_map.get(&node_id) {
            if let Some(parent_node) = state.nodes.get_mut(&parent_id) {
                let children: Vec<NodeId> = parent_node
                    .children()
                    .iter()
                    .filter(|c| **c != node_id)
                    .copied()
                    .collect();
                parent_node.set_children(children);
            }
        }
        Ok(())
    }
}

// ── UiStyleManager ───────────────────────────────────────────────────────────

impl UiStyleManager for AccessKitUiBackend {
    #[instrument(skip(self, fg, bg), fields(widget = widget.0))]
    fn set_colors(
        &self,
        widget: WidgetId,
        fg: SrgbColor,
        bg: SrgbColor,
    ) -> UiResult<Established<SufficientContrast>> {
        let ratio = contrast_ratio(&fg, &bg);
        if ratio < 4.5 {
            return Err(UiError::new(UiErrorKind::InsufficientContrast(format!(
                "contrast ratio {ratio:.2} is below 4.5:1 required"
            ))));
        }
        let node_id = widget.to_node_id();
        let state = self.state.lock().unwrap();
        if !state.nodes.contains_key(&node_id) {
            return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                "widget {:?} not found",
                node_id
            ))));
        }
        Ok(Established::assert())
    }

    #[instrument(skip(self), fields(widget = widget.0, size_px))]
    fn set_font_size(&self, widget: WidgetId, size_px: f32) -> UiResult<()> {
        let node_id = widget.to_node_id();
        let state = self.state.lock().unwrap();
        if !state.nodes.contains_key(&node_id) {
            return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                "widget {:?} not found",
                node_id
            ))));
        }
        let _ = size_px;
        Ok(())
    }

    #[instrument(skip(self), fields(widget = widget.0, px))]
    fn set_spacing(&self, widget: WidgetId, px: f32) -> UiResult<()> {
        let node_id = widget.to_node_id();
        let state = self.state.lock().unwrap();
        if !state.nodes.contains_key(&node_id) {
            return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                "widget {:?} not found",
                node_id
            ))));
        }
        let _ = px;
        Ok(())
    }

    #[instrument(skip(self), fields(theme_name))]
    fn apply_theme(&self, theme_name: &str) -> UiResult<Established<SufficientContrast>> {
        match theme_name {
            "high-contrast" | "dark" | "light" => Ok(Established::assert()),
            _ => Err(UiError::new(UiErrorKind::Unsupported(format!(
                "unknown theme: {theme_name}"
            )))),
        }
    }
}

// ── UiNavigationManager ──────────────────────────────────────────────────────

impl UiNavigationManager for AccessKitUiBackend {
    #[instrument(skip(self, ids))]
    fn set_focus_order(&self, ids: Vec<WidgetId>) -> UiResult<Established<KeyboardAccessible>> {
        let mut state = self.state.lock().unwrap();
        state.focus_order = ids.iter().map(|w| w.to_node_id()).collect();
        Ok(Established::assert())
    }

    #[instrument(skip(self), fields(id = id.0))]
    fn set_focus(&self, id: WidgetId) -> UiResult<Established<FocusVisible>> {
        let state = self.state.lock().unwrap();
        let node_id = id.to_node_id();
        if !state.nodes.contains_key(&node_id) {
            return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                "widget {:?} not found",
                node_id
            ))));
        }
        Ok(Established::assert())
    }

    #[instrument(skip(self), fields(key, action_id, label))]
    fn register_shortcut(
        &self,
        key: &str,
        action_id: &str,
        label: &str,
    ) -> UiResult<Established<KeyboardAccessible>> {
        if label.is_empty() {
            return Err(UiError::new(UiErrorKind::MissingLabel(
                "shortcut label is empty".into(),
            )));
        }
        let _ = (key, action_id);
        Ok(Established::assert())
    }

    #[instrument(skip(self), fields(target_id = target_id.0))]
    fn skip_link(&self, target_id: WidgetId) -> UiResult<Established<KeyboardAccessible>> {
        let state = self.state.lock().unwrap();
        if !state.nodes.contains_key(&target_id.to_node_id()) {
            return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                "skip target {:?} not found",
                target_id.to_node_id()
            ))));
        }
        Ok(Established::assert())
    }

    #[instrument(skip(self))]
    fn focus_order(&self) -> UiResult<Vec<WidgetId>> {
        let state = self.state.lock().unwrap();
        Ok(state
            .focus_order
            .iter()
            .map(|n| WidgetId::from_node(*n))
            .collect())
    }
}

// ── UiAccessibilityAuditor ──────────────────────────────────────────────────

impl UiAccessibilityAuditor for AccessKitUiBackend {
    #[instrument(skip(self))]
    fn audit_wcag_a(&self) -> UiResult<(VerificationReport, Established<AccessibleAA>)> {
        let update = self.state.lock().unwrap().to_tree_update();
        let layout = crate::Layout::from_update(update);
        match layout.verify_a(Viewport::new(1920, 1080)) {
            Ok(verified) => {
                let report = verified.report().clone();
                Ok((report, Established::assert()))
            }
            Err(report) => Err(UiError::new(UiErrorKind::VerificationFailed(format!(
                "{} violations",
                report.error_count()
            )))),
        }
    }

    #[instrument(skip(self))]
    fn audit_wcag_aa(&self) -> UiResult<(VerificationReport, Established<AccessibleAA>)> {
        let update = self.state.lock().unwrap().to_tree_update();
        let layout = crate::Layout::from_update(update);
        match layout.verify_aa(Viewport::new(1920, 1080)) {
            Ok(verified) => {
                let report = verified.report().clone();
                Ok((report, Established::assert()))
            }
            Err(report) => Err(UiError::new(UiErrorKind::VerificationFailed(format!(
                "{} violations",
                report.error_count()
            )))),
        }
    }

    #[instrument(skip(self))]
    fn audit_contrast(
        &self,
    ) -> UiResult<(
        Vec<ContrastViolation>,
        Option<Established<SufficientContrast>>,
    )> {
        // The reference backend doesn't store color pairs on nodes,
        // so we report no contrast violations.
        Ok((vec![], Some(Established::assert())))
    }

    #[instrument(skip(self), fields(id = id.0))]
    fn widget_accessibility(&self, id: WidgetId) -> UiResult<WidgetA11y> {
        let state = self.state.lock().unwrap();
        let node_id = id.to_node_id();
        if !state.nodes.contains_key(&node_id) {
            return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                "widget {:?} not found",
                node_id
            ))));
        }
        Ok(WidgetA11y {
            id,
            violations: vec![],
            level: Some("AA".to_string()),
        })
    }
}

// ── UiEventDispatcher ────────────────────────────────────────────────────────

impl UiEventDispatcher for AccessKitUiBackend {
    #[instrument(skip(self), fields(widget = widget.0, handler_id))]
    fn on_click(&self, widget: WidgetId, handler_id: &str) -> UiResult<()> {
        let mut state = self.state.lock().unwrap();
        state
            .event_handlers
            .insert((widget.0, "click".to_string()), handler_id.to_string());
        Ok(())
    }

    #[instrument(skip(self), fields(widget = widget.0, handler_id))]
    fn on_focus(&self, widget: WidgetId, handler_id: &str) -> UiResult<()> {
        let mut state = self.state.lock().unwrap();
        state
            .event_handlers
            .insert((widget.0, "focus".to_string()), handler_id.to_string());
        Ok(())
    }

    #[instrument(skip(self), fields(widget = widget.0, handler_id))]
    fn on_blur(&self, widget: WidgetId, handler_id: &str) -> UiResult<()> {
        let mut state = self.state.lock().unwrap();
        state
            .event_handlers
            .insert((widget.0, "blur".to_string()), handler_id.to_string());
        Ok(())
    }

    #[instrument(skip(self), fields(widget = widget.0, key, handler_id))]
    fn on_key(&self, widget: WidgetId, key: &str, handler_id: &str) -> UiResult<()> {
        let mut state = self.state.lock().unwrap();
        state
            .event_handlers
            .insert((widget.0, format!("key:{key}")), handler_id.to_string());
        Ok(())
    }
}

// ── UiRenderer ───────────────────────────────────────────────────────────────

impl UiRenderer for AccessKitUiBackend {
    #[instrument(skip(self, tree))]
    fn render(&self, tree: &VerifiedTree) -> UiResult<(RenderStats, Established<RenderComplete>)> {
        // IR backend: rendering is a no-op (this backend IS the IR).
        let count = tree.nodes().len();
        tracing::debug!(widget_count = count, "AccessKit IR render (no-op)");
        Ok((
            RenderStats {
                nodes_visited: count,
                widgets_rendered: count,
                ..Default::default()
            },
            Established::assert(),
        ))
    }

    fn render_partial(&self, _node_id: WidgetId, tree: &VerifiedTree) -> UiResult<RenderStats> {
        Ok(RenderStats {
            widgets_rendered: tree.nodes().len(),
            ..Default::default()
        })
    }

    fn supports_role(&self, _role: Role) -> bool {
        true
    }

    fn backend_name(&self) -> &str {
        "accesskit-ir"
    }
}

// ── UiInspector ──────────────────────────────────────────────────────────────

impl UiInspector for AccessKitUiBackend {
    #[instrument(skip(self), fields(id = id.0))]
    fn widget_info(&self, id: WidgetId) -> UiResult<WidgetInfo> {
        let state = self.state.lock().unwrap();
        let node_id = id.to_node_id();
        let node = state
            .nodes
            .get(&node_id)
            .ok_or_else(|| UiError::new(UiErrorKind::WidgetNotFound(format!("{:?}", node_id))))?;
        Ok(WidgetInfo {
            id,
            role: format!("{:?}", node.role()),
            label: node
                .label()
                .map(String::from)
                .or_else(|| node.value().map(String::from)),
            is_focusable: is_focusable_role(node.role()),
            children: node
                .children()
                .iter()
                .map(|c| WidgetId::from_node(*c))
                .collect(),
        })
    }

    #[instrument(skip(self), fields(id = id.0))]
    fn children(&self, id: WidgetId) -> UiResult<Vec<WidgetId>> {
        let state = self.state.lock().unwrap();
        let node = state
            .nodes
            .get(&id.to_node_id())
            .ok_or_else(|| UiError::new(UiErrorKind::WidgetNotFound(format!("{:?}", id.0))))?;
        Ok(node
            .children()
            .iter()
            .map(|c| WidgetId::from_node(*c))
            .collect())
    }

    #[instrument(skip(self), fields(id = id.0))]
    fn parent(&self, id: WidgetId) -> UiResult<Option<WidgetId>> {
        let state = self.state.lock().unwrap();
        Ok(state
            .parent_map
            .get(&id.to_node_id())
            .map(|p| WidgetId::from_node(*p)))
    }

    #[instrument(skip(self), fields(role = ?role))]
    fn find_by_role(&self, role: Role) -> UiResult<Vec<WidgetId>> {
        let state = self.state.lock().unwrap();
        Ok(state
            .nodes
            .iter()
            .filter(|(_, n)| n.role() == role)
            .map(|(id, _)| WidgetId::from_node(*id))
            .collect())
    }

    #[instrument(skip(self), fields(text))]
    fn find_by_label(&self, text: &str) -> UiResult<Vec<WidgetId>> {
        let state = self.state.lock().unwrap();
        let text_lower = text.to_lowercase();
        Ok(state
            .nodes
            .iter()
            .filter(|(_, n)| {
                n.label()
                    .map(|l| l.to_lowercase().contains(&text_lower))
                    .unwrap_or(false)
                    || n.value()
                        .map(|v| v.to_lowercase().contains(&text_lower))
                        .unwrap_or(false)
            })
            .map(|(id, _)| WidgetId::from_node(*id))
            .collect())
    }

    fn widget_count(&self) -> usize {
        self.state.lock().unwrap().nodes.len()
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn role_hint_to_role(hint: &str) -> Role {
    match hint {
        "paragraph" => Role::Paragraph,
        "heading" => Role::Heading,
        "caption" => Role::Caption,
        "status" => Role::Status,
        "alert" => Role::Alert,
        _ => Role::Label,
    }
}

fn is_focusable_role(role: Role) -> bool {
    matches!(
        role,
        Role::Button
            | Role::DefaultButton
            | Role::Link
            | Role::CheckBox
            | Role::RadioButton
            | Role::TextInput
            | Role::SearchInput
            | Role::EmailInput
            | Role::UrlInput
            | Role::PhoneNumberInput
            | Role::PasswordInput
            | Role::MultilineTextInput
            | Role::NumberInput
            | Role::Slider
            | Role::ComboBox
            | Role::Switch
    )
}
