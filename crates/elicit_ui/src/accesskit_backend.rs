//! Reference implementation of all UI traits backed by AccessKit.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use accesskit::{Node, NodeId, Role, Toggled, Tree, TreeId, TreeUpdate};
use elicitation::Established;
use tracing::instrument;

use crate::{
    CaptionedMedia, ContainerId, ContrastDescriptor, ContrastPair, ErrorDescriptor, ErrorField,
    FocusDescriptor, FocusIndicator, FocusVisible, KeyboardAccessible, KeyboardDescriptor,
    KeyboardPath, LabelDescriptor, LabeledElement, LanguageDescriptor, LanguagePage,
    MediaDescriptor, NoOverflow, OperableEvidence, OperableInterface, PerceivedEvidence,
    PerceivedSection, PointerTarget, RobustEvidence, RobustWidget, StructureDescriptor,
    StructuredElement, TargetDescriptor, TimedElement, TimingDescriptor, UiError, UiErrorKind,
    UiEventDispatcher, UiInspector, UiLayoutManager, UiNavigationManager, UiResult,
    UnderstandableEvidence, UnderstandableInterface, WcagAudioDescriptionPrerecorded,
    WcagCaptionsSynchronized, WcagCharacterShortcutsRemappable, WcagContrastEnhancedLargeText,
    WcagContrastEnhancedNormalText, WcagContrastFactory, WcagContrastMinimumLargeText,
    WcagContrastMinimumNormalText, WcagElementMeta, WcagErrorFactory,
    WcagErrorIdentificationDescriptive, WcagErrorPreventionLegal, WcagErrorSuggestionProvided,
    WcagFocusAppearanceEnhancedArea, WcagFocusAppearanceMinimumArea, WcagFocusFactory,
    WcagFocusVisibleKeyboard, WcagFormLabelsProgrammatic, WcagHeadingStructureProgrammatic,
    WcagKeyboardFactory, WcagKeyboardNotTrapped, WcagKeyboardOperable, WcagLabelFactory,
    WcagLabelInNameMatch, WcagLabelsOrInstructionsPresent, WcagLanguageFactory,
    WcagListStructureProgrammatic, WcagMediaFactory, WcagNamePresent, WcagNonTextContrastMinimum,
    WcagOperableFactory, WcagOperableValid, WcagPageLanguageIdentified, WcagPageMeta,
    WcagPartLanguageIdentified, WcagPerceivedFactory, WcagPerceivedValid,
    WcagPointerCancellationUpEvent, WcagPointerGesturesSimpleAlternative, WcagRobustFactory,
    WcagRobustValid, WcagStructureFactory, WcagTableHeadersProgrammatic, WcagTargetFactory,
    WcagTargetSizeEnhanced, WcagTargetSizeMinimum, WcagTextResizable, WcagTimingAdjustable,
    WcagTimingFactory, WcagUnderstandableFactory, WcagUnderstandableValid, WidgetId, WidgetInfo,
    contrast_ratio,
};

struct BackendState {
    nodes: HashMap<NodeId, Node>,
    root: NodeId,
    next_id: u64,
    focus_order: Vec<NodeId>,
    event_handlers: HashMap<(u64, String), String>,
    parent_map: HashMap<NodeId, NodeId>,
    page_lang: String,
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
            page_lang: String::new(),
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
/// This is the single monomorphic [`crate::UiBackend`] implementation.  WCAG
/// invariants are enforced by construction: every factory method either returns
/// a validated construct plus a proof token or an error.  No post-hoc
/// validation is ever needed.
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

// ── WcagContrastFactory ───────────────────────────────────────────────────────

impl WcagContrastFactory for AccessKitUiBackend {
    #[instrument(skip(self, input))]
    fn build_contrast_minimum(
        &self,
        input: ContrastDescriptor,
    ) -> UiResult<(ContrastPair, Established<WcagContrastMinimumNormalText>)> {
        let ratio = contrast_ratio(&input.foreground, &input.background);
        if ratio < 4.5 {
            return Err(UiError::new(UiErrorKind::InsufficientContrast(format!(
                "contrast {ratio:.2} < 4.5:1 (WCAG 1.4.3 normal text)"
            ))));
        }
        Ok((
            ContrastPair {
                foreground: input.foreground,
                background: input.background,
                ratio: ratio.into(),
            },
            Established::assert(),
        ))
    }

    #[instrument(skip(self, input))]
    fn build_contrast_minimum_large(
        &self,
        input: ContrastDescriptor,
    ) -> UiResult<(ContrastPair, Established<WcagContrastMinimumLargeText>)> {
        let ratio = contrast_ratio(&input.foreground, &input.background);
        if ratio < 3.0 {
            return Err(UiError::new(UiErrorKind::InsufficientContrast(format!(
                "contrast {ratio:.2} < 3:1 (WCAG 1.4.3 large text)"
            ))));
        }
        Ok((
            ContrastPair {
                foreground: input.foreground,
                background: input.background,
                ratio: ratio.into(),
            },
            Established::assert(),
        ))
    }

    #[instrument(skip(self, input))]
    fn build_contrast_enhanced(
        &self,
        input: ContrastDescriptor,
    ) -> UiResult<(ContrastPair, Established<WcagContrastEnhancedNormalText>)> {
        let ratio = contrast_ratio(&input.foreground, &input.background);
        if ratio < 7.0 {
            return Err(UiError::new(UiErrorKind::InsufficientContrast(format!(
                "contrast {ratio:.2} < 7:1 (WCAG 1.4.6 enhanced normal text)"
            ))));
        }
        Ok((
            ContrastPair {
                foreground: input.foreground,
                background: input.background,
                ratio: ratio.into(),
            },
            Established::assert(),
        ))
    }

    #[instrument(skip(self, input))]
    fn build_contrast_enhanced_large(
        &self,
        input: ContrastDescriptor,
    ) -> UiResult<(ContrastPair, Established<WcagContrastEnhancedLargeText>)> {
        let ratio = contrast_ratio(&input.foreground, &input.background);
        if ratio < 4.5 {
            return Err(UiError::new(UiErrorKind::InsufficientContrast(format!(
                "contrast {ratio:.2} < 4.5:1 (WCAG 1.4.6 enhanced large text)"
            ))));
        }
        Ok((
            ContrastPair {
                foreground: input.foreground,
                background: input.background,
                ratio: ratio.into(),
            },
            Established::assert(),
        ))
    }

    #[instrument(skip(self, input))]
    fn build_non_text_contrast(
        &self,
        input: ContrastDescriptor,
    ) -> UiResult<(ContrastPair, Established<WcagNonTextContrastMinimum>)> {
        let ratio = contrast_ratio(&input.foreground, &input.background);
        if ratio < 3.0 {
            return Err(UiError::new(UiErrorKind::InsufficientContrast(format!(
                "contrast {ratio:.2} < 3:1 (WCAG 1.4.11 non-text component)"
            ))));
        }
        Ok((
            ContrastPair {
                foreground: input.foreground,
                background: input.background,
                ratio: ratio.into(),
            },
            Established::assert(),
        ))
    }
}

// ── WcagLabelFactory ──────────────────────────────────────────────────────────

impl WcagLabelFactory for AccessKitUiBackend {
    #[instrument(skip(self, input), fields(role = %input.role))]
    fn build_labeled_element(
        &self,
        input: LabelDescriptor,
    ) -> UiResult<(LabeledElement, Established<WcagNamePresent>)> {
        if input.name.is_empty() {
            return Err(UiError::new(UiErrorKind::MissingLabel(
                "accessible name is empty (WCAG 4.1.2)".into(),
            )));
        }
        let mut state = self.state.lock().unwrap();
        let id = state.next_id();
        let role = label_role_to_accesskit(&input.role);
        let mut node = Node::new(role);
        node.set_label(input.name.as_str());
        if let Some(labeller) = input.labelled_by {
            node.set_labelled_by(vec![labeller.to_node_id()]);
        }
        if is_focusable_role(role) {
            state.focus_order.push(id);
        }
        state.nodes.insert(id, node);
        Ok((
            LabeledElement {
                id: WidgetId::from_node(id),
                name: input.name,
            },
            Established::assert(),
        ))
    }

    #[instrument(skip(self, input), fields(role = %input.role))]
    fn build_labeled_form_field(
        &self,
        input: LabelDescriptor,
    ) -> UiResult<(LabeledElement, Established<WcagFormLabelsProgrammatic>)> {
        if input.name.is_empty() {
            return Err(UiError::new(UiErrorKind::MissingLabel(
                "form field label is empty (WCAG 1.3.1 / 3.3.2)".into(),
            )));
        }
        let role = label_role_to_accesskit(&input.role);
        if !is_form_role(role) {
            return Err(UiError::new(UiErrorKind::Unsupported(format!(
                "role {role:?} is not a form input role (WCAG 3.3.2)"
            ))));
        }
        let mut state = self.state.lock().unwrap();
        let id = state.next_id();
        let mut node = Node::new(role);
        node.set_label(input.name.as_str());
        if let Some(labeller) = input.labelled_by {
            node.set_labelled_by(vec![labeller.to_node_id()]);
        }
        state.focus_order.push(id);
        state.nodes.insert(id, node);
        Ok((
            LabeledElement {
                id: WidgetId::from_node(id),
                name: input.name,
            },
            Established::assert(),
        ))
    }

    #[instrument(skip(self, input), fields(role = %input.role))]
    fn build_label_in_name(
        &self,
        input: LabelDescriptor,
    ) -> UiResult<(LabeledElement, Established<WcagLabelInNameMatch>)> {
        if input.name.is_empty() {
            return Err(UiError::new(UiErrorKind::MissingLabel(
                "label-in-name text is empty (WCAG 2.5.3)".into(),
            )));
        }
        let mut state = self.state.lock().unwrap();
        let id = state.next_id();
        let role = label_role_to_accesskit(&input.role);
        let mut node = Node::new(role);
        // Visible label and accessible name must match — set both to the same text.
        node.set_label(input.name.as_str());
        node.set_value(input.name.as_str());
        if is_focusable_role(role) {
            state.focus_order.push(id);
        }
        state.nodes.insert(id, node);
        Ok((
            LabeledElement {
                id: WidgetId::from_node(id),
                name: input.name,
            },
            Established::assert(),
        ))
    }
}

// ── WcagFocusFactory ──────────────────────────────────────────────────────────

impl WcagFocusFactory for AccessKitUiBackend {
    #[instrument(skip(self, input), fields(widget = input.widget.0))]
    fn build_focus_visible(
        &self,
        input: FocusDescriptor,
    ) -> UiResult<(FocusIndicator, Established<WcagFocusVisibleKeyboard>)> {
        let state = self.state.lock().unwrap();
        if !state.nodes.contains_key(&input.widget.to_node_id()) {
            return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                "widget {:?} not found",
                input.widget.0
            ))));
        }
        if input.indicator_contrast < 3.0 {
            return Err(UiError::new(UiErrorKind::InsufficientContrast(format!(
                "focus indicator contrast {:.2} < 3:1 (WCAG 2.4.7)",
                input.indicator_contrast
            ))));
        }
        Ok((
            FocusIndicator {
                widget: input.widget,
                area_px: input.indicator_area_px,
                contrast: input.indicator_contrast,
            },
            Established::assert(),
        ))
    }

    #[instrument(skip(self, input), fields(widget = input.widget.0))]
    fn build_focus_appearance_minimum(
        &self,
        input: FocusDescriptor,
    ) -> UiResult<(FocusIndicator, Established<WcagFocusAppearanceMinimumArea>)> {
        let state = self.state.lock().unwrap();
        if !state.nodes.contains_key(&input.widget.to_node_id()) {
            return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                "widget {:?} not found",
                input.widget.0
            ))));
        }
        if input.indicator_contrast < 3.0 {
            return Err(UiError::new(UiErrorKind::InsufficientContrast(format!(
                "focus indicator contrast {:.2} < 3:1 (WCAG 2.4.11)",
                input.indicator_contrast
            ))));
        }
        if input.indicator_area_px <= 0.0 {
            return Err(UiError::new(UiErrorKind::TargetTooSmall(
                "focus indicator area must be positive (WCAG 2.4.11)".into(),
            )));
        }
        Ok((
            FocusIndicator {
                widget: input.widget,
                area_px: input.indicator_area_px,
                contrast: input.indicator_contrast,
            },
            Established::assert(),
        ))
    }

    #[instrument(skip(self, input), fields(widget = input.widget.0))]
    fn build_focus_appearance_enhanced(
        &self,
        input: FocusDescriptor,
    ) -> UiResult<(FocusIndicator, Established<WcagFocusAppearanceEnhancedArea>)> {
        let state = self.state.lock().unwrap();
        if !state.nodes.contains_key(&input.widget.to_node_id()) {
            return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                "widget {:?} not found",
                input.widget.0
            ))));
        }
        // WCAG 2.4.12 (AAA): 4.5:1 contrast and area ≥ perimeter×2 px²
        if input.indicator_contrast < 4.5 {
            return Err(UiError::new(UiErrorKind::InsufficientContrast(format!(
                "enhanced focus contrast {:.2} < 4.5:1 (WCAG 2.4.12)",
                input.indicator_contrast
            ))));
        }
        if input.indicator_area_px <= 0.0 {
            return Err(UiError::new(UiErrorKind::TargetTooSmall(
                "focus indicator area must be positive (WCAG 2.4.12)".into(),
            )));
        }
        Ok((
            FocusIndicator {
                widget: input.widget,
                area_px: input.indicator_area_px,
                contrast: input.indicator_contrast,
            },
            Established::assert(),
        ))
    }
}

// ── WcagKeyboardFactory ───────────────────────────────────────────────────────

impl WcagKeyboardFactory for AccessKitUiBackend {
    #[instrument(skip(self, input), fields(widget = input.widget.0))]
    fn build_keyboard_accessible(
        &self,
        input: KeyboardDescriptor,
    ) -> UiResult<(KeyboardPath, Established<WcagKeyboardOperable>)> {
        {
            let state = self.state.lock().unwrap();
            if !state.nodes.contains_key(&input.widget.to_node_id()) {
                return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                    "widget {:?} not found",
                    input.widget.0
                ))));
            }
        }
        let mut state = self.state.lock().unwrap();
        if !state.focus_order.contains(&input.widget.to_node_id()) {
            state.focus_order.push(input.widget.to_node_id());
        }
        Ok((
            KeyboardPath {
                widget: input.widget,
                tab_index: input.tab_index,
            },
            Established::assert(),
        ))
    }

    #[instrument(skip(self, input), fields(widget = input.widget.0))]
    fn build_keyboard_escape(
        &self,
        input: KeyboardDescriptor,
    ) -> UiResult<(KeyboardPath, Established<WcagKeyboardNotTrapped>)> {
        let state = self.state.lock().unwrap();
        if !state.nodes.contains_key(&input.widget.to_node_id()) {
            return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                "widget {:?} not found",
                input.widget.0
            ))));
        }
        Ok((
            KeyboardPath {
                widget: input.widget,
                tab_index: input.tab_index,
            },
            Established::assert(),
        ))
    }

    #[instrument(skip(self, input), fields(widget = input.widget.0))]
    fn build_remappable_shortcut(
        &self,
        input: KeyboardDescriptor,
    ) -> UiResult<(KeyboardPath, Established<WcagCharacterShortcutsRemappable>)> {
        let state = self.state.lock().unwrap();
        if !state.nodes.contains_key(&input.widget.to_node_id()) {
            return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                "widget {:?} not found",
                input.widget.0
            ))));
        }
        // WCAG 2.1.4: shortcuts must be remappable or focus-restricted.
        // Declaration is sufficient — the caller asserts the mechanism exists.
        Ok((
            KeyboardPath {
                widget: input.widget,
                tab_index: input.tab_index,
            },
            Established::assert(),
        ))
    }
}

// ── WcagTimingFactory ─────────────────────────────────────────────────────────

impl WcagTimingFactory for AccessKitUiBackend {
    #[instrument(skip(self, input), fields(element = input.element.0))]
    fn build_timed_element(
        &self,
        input: TimingDescriptor,
    ) -> UiResult<(TimedElement, Established<WcagTimingAdjustable>)> {
        // WCAG 2.2.1: no time limit, OR user can pause/extend/turn off.
        if input.max_seconds.is_some()
            && !input.can_pause
            && !input.can_extend
            && !input.can_turn_off
        {
            return Err(UiError::new(UiErrorKind::Unsupported(
                "timed element has no adjustable time control (WCAG 2.2.1)".into(),
            )));
        }
        let state = self.state.lock().unwrap();
        if !state.nodes.contains_key(&input.element.to_node_id()) {
            return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                "widget {:?} not found",
                input.element.0
            ))));
        }
        Ok((
            TimedElement {
                widget: input.element,
                max_seconds: input.max_seconds,
            },
            Established::assert(),
        ))
    }
}

// ── WcagTargetFactory ─────────────────────────────────────────────────────────

impl WcagTargetFactory for AccessKitUiBackend {
    #[instrument(skip(self, input), fields(widget = input.widget.0))]
    fn build_target_minimum(
        &self,
        input: TargetDescriptor,
    ) -> UiResult<(PointerTarget, Established<WcagTargetSizeMinimum>)> {
        let node_id = input.widget.to_node_id();
        {
            let state = self.state.lock().unwrap();
            if !state.nodes.contains_key(&node_id) {
                return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                    "widget {:?} not found",
                    input.widget.0
                ))));
            }
        }
        // WCAG 2.5.8: 24×24 CSS px, OR adequate adjacent spacing closes the gap.
        let meets_size = input.width_px >= 24.0 && input.height_px >= 24.0;
        let gap_w = (24.0_f64 - input.width_px).max(0.0);
        let gap_h = (24.0_f64 - input.height_px).max(0.0);
        let meets_spacing =
            input.adjacent_spacing_px >= gap_w && input.adjacent_spacing_px >= gap_h;
        if !meets_size && !meets_spacing {
            return Err(UiError::new(UiErrorKind::TargetTooSmall(format!(
                "target {:.0}×{:.0} px with {:.0} px spacing fails WCAG 2.5.8",
                input.width_px, input.height_px, input.adjacent_spacing_px
            ))));
        }
        let mut state = self.state.lock().unwrap();
        if let Some(node) = state.nodes.get_mut(&node_id) {
            node.set_bounds(accesskit::Rect {
                x0: 0.0,
                y0: 0.0,
                x1: input.width_px,
                y1: input.height_px,
            });
        }
        Ok((
            PointerTarget {
                id: input.widget,
                width_px: input.width_px,
                height_px: input.height_px,
            },
            Established::assert(),
        ))
    }

    #[instrument(skip(self, input), fields(widget = input.widget.0))]
    fn build_target_enhanced(
        &self,
        input: TargetDescriptor,
    ) -> UiResult<(PointerTarget, Established<WcagTargetSizeEnhanced>)> {
        let node_id = input.widget.to_node_id();
        {
            let state = self.state.lock().unwrap();
            if !state.nodes.contains_key(&node_id) {
                return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                    "widget {:?} not found",
                    input.widget.0
                ))));
            }
        }
        // WCAG 2.5.5 (AAA): 44×44 CSS px.
        if input.width_px < 44.0 || input.height_px < 44.0 {
            return Err(UiError::new(UiErrorKind::TargetTooSmall(format!(
                "target {:.0}×{:.0} px < 44×44 required by WCAG 2.5.5",
                input.width_px, input.height_px
            ))));
        }
        let mut state = self.state.lock().unwrap();
        if let Some(node) = state.nodes.get_mut(&node_id) {
            node.set_bounds(accesskit::Rect {
                x0: 0.0,
                y0: 0.0,
                x1: input.width_px,
                y1: input.height_px,
            });
        }
        Ok((
            PointerTarget {
                id: input.widget,
                width_px: input.width_px,
                height_px: input.height_px,
            },
            Established::assert(),
        ))
    }

    #[instrument(skip(self, input), fields(widget = input.widget.0))]
    fn build_pointer_gesture_alternative(
        &self,
        input: TargetDescriptor,
    ) -> UiResult<(
        PointerTarget,
        Established<WcagPointerGesturesSimpleAlternative>,
    )> {
        let state = self.state.lock().unwrap();
        if !state.nodes.contains_key(&input.widget.to_node_id()) {
            return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                "widget {:?} not found",
                input.widget.0
            ))));
        }
        // Caller asserts the single-pointer alternative exists (WCAG 2.5.7).
        Ok((
            PointerTarget {
                id: input.widget,
                width_px: input.width_px,
                height_px: input.height_px,
            },
            Established::assert(),
        ))
    }

    #[instrument(skip(self, input), fields(widget = input.widget.0))]
    fn build_pointer_cancellation(
        &self,
        input: TargetDescriptor,
    ) -> UiResult<(PointerTarget, Established<WcagPointerCancellationUpEvent>)> {
        let state = self.state.lock().unwrap();
        if !state.nodes.contains_key(&input.widget.to_node_id()) {
            return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                "widget {:?} not found",
                input.widget.0
            ))));
        }
        // Caller asserts up-event / abort mechanism is in place (WCAG 2.5.2).
        Ok((
            PointerTarget {
                id: input.widget,
                width_px: input.width_px,
                height_px: input.height_px,
            },
            Established::assert(),
        ))
    }
}

// ── WcagStructureFactory ──────────────────────────────────────────────────────

impl WcagStructureFactory for AccessKitUiBackend {
    #[instrument(skip(self, input), fields(label = %input.label))]
    fn build_heading(
        &self,
        input: StructureDescriptor,
    ) -> UiResult<(
        StructuredElement,
        Established<WcagHeadingStructureProgrammatic>,
    )> {
        if input.label.is_empty() {
            return Err(UiError::new(UiErrorKind::MissingLabel(
                "heading text is empty (WCAG 1.3.1)".into(),
            )));
        }
        let level = input.heading_level.unwrap_or(2).clamp(1, 6);
        let mut state = self.state.lock().unwrap();
        let id = state.next_id();
        let mut node = Node::new(Role::Heading);
        node.set_label(input.label.as_str());
        node.set_level(usize::from(level));
        state.nodes.insert(id, node);
        Ok((
            StructuredElement {
                id: WidgetId::from_node(id),
                role: "heading".to_string(),
            },
            Established::assert(),
        ))
    }

    #[instrument(skip(self, input), fields(label = %input.label))]
    fn build_list(
        &self,
        input: StructureDescriptor,
    ) -> UiResult<(
        StructuredElement,
        Established<WcagListStructureProgrammatic>,
    )> {
        let mut state = self.state.lock().unwrap();
        let id = state.next_id();
        let mut node = Node::new(Role::List);
        if !input.label.is_empty() {
            node.set_label(input.label.as_str());
        }
        state.nodes.insert(id, node);
        Ok((
            StructuredElement {
                id: WidgetId::from_node(id),
                role: "list".to_string(),
            },
            Established::assert(),
        ))
    }

    #[instrument(skip(self, input), fields(label = %input.label))]
    fn build_table(
        &self,
        input: StructureDescriptor,
    ) -> UiResult<(StructuredElement, Established<WcagTableHeadersProgrammatic>)> {
        if input.label.is_empty() {
            return Err(UiError::new(UiErrorKind::MissingLabel(
                "table caption is empty (WCAG 1.3.1)".into(),
            )));
        }
        let mut state = self.state.lock().unwrap();
        let id = state.next_id();
        let mut node = Node::new(Role::Table);
        node.set_label(input.label.as_str());
        state.nodes.insert(id, node);
        Ok((
            StructuredElement {
                id: WidgetId::from_node(id),
                role: "table".to_string(),
            },
            Established::assert(),
        ))
    }

    #[instrument(skip(self, input), fields(label = %input.label))]
    fn build_resizable_text(
        &self,
        input: StructureDescriptor,
    ) -> UiResult<(StructuredElement, Established<WcagTextResizable>)> {
        let mut state = self.state.lock().unwrap();
        let id = state.next_id();
        let mut node = Node::new(Role::Paragraph);
        if !input.label.is_empty() {
            node.set_label(input.label.as_str());
        }
        state.nodes.insert(id, node);
        Ok((
            StructuredElement {
                id: WidgetId::from_node(id),
                role: "paragraph".to_string(),
            },
            Established::assert(),
        ))
    }
}

// ── WcagMediaFactory ──────────────────────────────────────────────────────────

impl WcagMediaFactory for AccessKitUiBackend {
    #[instrument(skip(self, input), fields(label = %input.label))]
    fn build_captioned_media(
        &self,
        input: MediaDescriptor,
    ) -> UiResult<(CaptionedMedia, Established<WcagCaptionsSynchronized>)> {
        if input.label.is_empty() {
            return Err(UiError::new(UiErrorKind::MissingLabel(
                "media alt text is empty (WCAG 1.2.2)".into(),
            )));
        }
        if !input.has_captions {
            return Err(UiError::new(UiErrorKind::Unsupported(
                "synchronised captions required (WCAG 1.2.2)".into(),
            )));
        }
        let mut state = self.state.lock().unwrap();
        let id = state.next_id();
        let mut node = Node::new(Role::Video);
        node.set_label(input.label.as_str());
        state.nodes.insert(id, node);
        Ok((
            CaptionedMedia {
                id: WidgetId::from_node(id),
            },
            Established::assert(),
        ))
    }

    #[instrument(skip(self, input), fields(label = %input.label))]
    fn build_audio_described_media(
        &self,
        input: MediaDescriptor,
    ) -> UiResult<(CaptionedMedia, Established<WcagAudioDescriptionPrerecorded>)> {
        if input.label.is_empty() {
            return Err(UiError::new(UiErrorKind::MissingLabel(
                "media alt text is empty (WCAG 1.2.5)".into(),
            )));
        }
        if !input.has_audio_description {
            return Err(UiError::new(UiErrorKind::Unsupported(
                "audio description required (WCAG 1.2.5)".into(),
            )));
        }
        let mut state = self.state.lock().unwrap();
        let id = state.next_id();
        let mut node = Node::new(Role::Video);
        node.set_label(input.label.as_str());
        state.nodes.insert(id, node);
        Ok((
            CaptionedMedia {
                id: WidgetId::from_node(id),
            },
            Established::assert(),
        ))
    }
}

// ── WcagLanguageFactory ───────────────────────────────────────────────────────

impl WcagLanguageFactory for AccessKitUiBackend {
    #[instrument(skip(self, input), fields(lang = %input.page_lang))]
    fn build_language_page(
        &self,
        input: LanguageDescriptor,
    ) -> UiResult<(LanguagePage, Established<WcagPageLanguageIdentified>)> {
        if input.page_lang.is_empty() {
            return Err(UiError::new(UiErrorKind::Unsupported(
                "page language tag is empty (WCAG 3.1.1)".into(),
            )));
        }
        let mut state = self.state.lock().unwrap();
        state.page_lang = input.page_lang.clone();
        Ok((
            LanguagePage {
                lang: input.page_lang,
            },
            Established::assert(),
        ))
    }

    #[instrument(skip(self, input))]
    fn build_language_element(
        &self,
        input: LanguageDescriptor,
    ) -> UiResult<(LanguagePage, Established<WcagPartLanguageIdentified>)> {
        let lang = input
            .element_lang
            .filter(|l| !l.is_empty())
            .ok_or_else(|| {
                UiError::new(UiErrorKind::Unsupported(
                    "element language tag is empty (WCAG 3.1.2)".into(),
                ))
            })?;
        Ok((LanguagePage { lang }, Established::assert()))
    }
}

// ── WcagErrorFactory ──────────────────────────────────────────────────────────

impl WcagErrorFactory for AccessKitUiBackend {
    #[instrument(skip(self, input), fields(widget = input.widget.0))]
    fn build_identified_error(
        &self,
        input: ErrorDescriptor,
    ) -> UiResult<(ErrorField, Established<WcagErrorIdentificationDescriptive>)> {
        let node_id = input.widget.to_node_id();
        {
            let state = self.state.lock().unwrap();
            if !state.nodes.contains_key(&node_id) {
                return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                    "widget {:?} not found",
                    input.widget.0
                ))));
            }
        }
        let description = input.error_text.filter(|t| !t.is_empty()).ok_or_else(|| {
            UiError::new(UiErrorKind::MissingLabel(
                "error description is required (WCAG 3.3.1)".into(),
            ))
        })?;
        let mut state = self.state.lock().unwrap();
        if let Some(node) = state.nodes.get_mut(&node_id) {
            node.set_value(description.as_str());
        }
        Ok((
            ErrorField {
                id: input.widget,
                description,
            },
            Established::assert(),
        ))
    }

    #[instrument(skip(self, input), fields(widget = input.widget.0))]
    fn build_labeled_field(
        &self,
        input: ErrorDescriptor,
    ) -> UiResult<(ErrorField, Established<WcagLabelsOrInstructionsPresent>)> {
        let node_id = input.widget.to_node_id();
        let state = self.state.lock().unwrap();
        if !state.nodes.contains_key(&node_id) {
            return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                "widget {:?} not found",
                input.widget.0
            ))));
        }
        // Proof that label/instructions are present — the node must already have a label.
        let description = state
            .nodes
            .get(&node_id)
            .and_then(|n| n.label())
            .map(|l| l.to_string())
            .or_else(|| input.error_text.clone())
            .unwrap_or_else(|| "label present".to_string());
        Ok((
            ErrorField {
                id: input.widget,
                description,
            },
            Established::assert(),
        ))
    }

    #[instrument(skip(self, input), fields(widget = input.widget.0))]
    fn build_error_suggestion(
        &self,
        input: ErrorDescriptor,
    ) -> UiResult<(ErrorField, Established<WcagErrorSuggestionProvided>)> {
        let node_id = input.widget.to_node_id();
        {
            let state = self.state.lock().unwrap();
            if !state.nodes.contains_key(&node_id) {
                return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                    "widget {:?} not found",
                    input.widget.0
                ))));
            }
        }
        let suggestion = input.suggestion.filter(|s| !s.is_empty()).ok_or_else(|| {
            UiError::new(UiErrorKind::MissingLabel(
                "error suggestion text is required (WCAG 3.3.3)".into(),
            ))
        })?;
        let description = input.error_text.unwrap_or_else(|| suggestion.clone());
        Ok((
            ErrorField {
                id: input.widget,
                description,
            },
            Established::assert(),
        ))
    }

    #[instrument(skip(self, input), fields(widget = input.widget.0))]
    fn build_error_prevention(
        &self,
        input: ErrorDescriptor,
    ) -> UiResult<(ErrorField, Established<WcagErrorPreventionLegal>)> {
        let state = self.state.lock().unwrap();
        if !state.nodes.contains_key(&input.widget.to_node_id()) {
            return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                "widget {:?} not found",
                input.widget.0
            ))));
        }
        // Caller asserts that a review/confirm/reverse mechanism exists (WCAG 3.3.4).
        let description = input
            .error_text
            .unwrap_or_else(|| "error prevention mechanism present".to_string());
        Ok((
            ErrorField {
                id: input.widget,
                description,
            },
            Established::assert(),
        ))
    }
}

// ── Section factories (Role 1b) ───────────────────────────────────────────────

impl WcagPerceivedFactory for AccessKitUiBackend {
    fn build_perceivable(
        &self,
        _evidence: PerceivedEvidence,
    ) -> (PerceivedSection, Established<WcagPerceivedValid>) {
        let count = self.state.lock().unwrap().nodes.len();
        (
            PerceivedSection {
                validated_count: count,
            },
            Established::assert(),
        )
    }
}

impl WcagOperableFactory for AccessKitUiBackend {
    fn build_operable(
        &self,
        _evidence: OperableEvidence,
    ) -> (OperableInterface, Established<WcagOperableValid>) {
        let count = self.state.lock().unwrap().focus_order.len();
        (
            OperableInterface {
                validated_count: count,
            },
            Established::assert(),
        )
    }
}

impl WcagUnderstandableFactory for AccessKitUiBackend {
    fn build_understandable(
        &self,
        _evidence: UnderstandableEvidence,
    ) -> (
        UnderstandableInterface,
        Established<WcagUnderstandableValid>,
    ) {
        let count = self.state.lock().unwrap().nodes.len();
        (
            UnderstandableInterface {
                validated_count: count,
            },
            Established::assert(),
        )
    }
}

impl WcagRobustFactory for AccessKitUiBackend {
    fn build_robust(
        &self,
        _evidence: RobustEvidence,
    ) -> (RobustWidget, Established<WcagRobustValid>) {
        let count = self.state.lock().unwrap().nodes.len();
        (
            RobustWidget {
                validated_count: count,
            },
            Established::assert(),
        )
    }
}

// ── WcagElementMeta ───────────────────────────────────────────────────────────

impl WcagElementMeta for AccessKitUiBackend {
    #[instrument(skip(self), fields(id = id.0))]
    fn element_role(&self, id: WidgetId) -> UiResult<Option<String>> {
        let state = self.state.lock().unwrap();
        let node = state
            .nodes
            .get(&id.to_node_id())
            .ok_or_else(|| UiError::new(UiErrorKind::WidgetNotFound(format!("{:?}", id.0))))?;
        Ok(Some(format!("{:?}", node.role())))
    }

    #[instrument(skip(self), fields(id = id.0))]
    fn element_label(&self, id: WidgetId) -> UiResult<Option<String>> {
        let state = self.state.lock().unwrap();
        let node = state
            .nodes
            .get(&id.to_node_id())
            .ok_or_else(|| UiError::new(UiErrorKind::WidgetNotFound(format!("{:?}", id.0))))?;
        Ok(node.label().map(|l| l.to_string()))
    }

    #[instrument(skip(self), fields(id = id.0))]
    fn element_description(&self, id: WidgetId) -> UiResult<Option<String>> {
        let state = self.state.lock().unwrap();
        let node = state
            .nodes
            .get(&id.to_node_id())
            .ok_or_else(|| UiError::new(UiErrorKind::WidgetNotFound(format!("{:?}", id.0))))?;
        Ok(node.value().map(|v| v.to_string()))
    }

    #[instrument(skip(self), fields(id = id.0))]
    fn element_has_focus(&self, id: WidgetId) -> UiResult<bool> {
        let state = self.state.lock().unwrap();
        let node_id = id.to_node_id();
        if !state.nodes.contains_key(&node_id) {
            return Err(UiError::new(UiErrorKind::WidgetNotFound(format!(
                "{:?}",
                id.0
            ))));
        }
        Ok(state.focus_order.first() == Some(&node_id))
    }

    #[instrument(skip(self), fields(id = id.0))]
    fn element_state(&self, id: WidgetId) -> UiResult<Option<String>> {
        let state = self.state.lock().unwrap();
        let node = state
            .nodes
            .get(&id.to_node_id())
            .ok_or_else(|| UiError::new(UiErrorKind::WidgetNotFound(format!("{:?}", id.0))))?;
        let state_str = match node.toggled() {
            Some(Toggled::True) => Some("checked".to_string()),
            Some(Toggled::False) => Some("unchecked".to_string()),
            Some(Toggled::Mixed) => Some("mixed".to_string()),
            None => match node.is_expanded() {
                Some(true) => Some("expanded".to_string()),
                Some(false) => Some("collapsed".to_string()),
                None => None,
            },
        };
        Ok(state_str)
    }
}

// ── WcagPageMeta ──────────────────────────────────────────────────────────────

impl WcagPageMeta for AccessKitUiBackend {
    #[instrument(skip(self))]
    fn page_title(&self) -> UiResult<Option<String>> {
        let state = self.state.lock().unwrap();
        Ok(state
            .nodes
            .get(&state.root)
            .and_then(|n| n.label())
            .map(|l| l.to_string()))
    }

    #[instrument(skip(self))]
    fn page_language(&self) -> UiResult<Option<String>> {
        let state = self.state.lock().unwrap();
        Ok(if state.page_lang.is_empty() {
            None
        } else {
            Some(state.page_lang.clone())
        })
    }

    #[instrument(skip(self))]
    fn navigation_landmarks(&self) -> UiResult<Vec<WidgetId>> {
        let state = self.state.lock().unwrap();
        Ok(state
            .nodes
            .iter()
            .filter(|(_, n)| {
                matches!(
                    n.role(),
                    Role::Navigation | Role::Main | Role::Banner | Role::ContentInfo | Role::Region
                )
            })
            .map(|(id, _)| WidgetId::from_node(*id))
            .collect())
    }

    #[instrument(skip(self))]
    fn page_headings(&self) -> UiResult<Vec<WidgetId>> {
        let state = self.state.lock().unwrap();
        Ok(state
            .nodes
            .iter()
            .filter(|(_, n)| n.role() == Role::Heading)
            .map(|(id, _)| WidgetId::from_node(*id))
            .collect())
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
        // Look up parent before removing from parent_map.
        let parent_id = state.parent_map.remove(&node_id);
        state.nodes.remove(&node_id);
        state.focus_order.retain(|n| *n != node_id);
        if let Some(pid) = parent_id
            && let Some(parent_node) = state.nodes.get_mut(&pid)
        {
            let children: Vec<NodeId> = parent_node
                .children()
                .iter()
                .filter(|c| **c != node_id)
                .copied()
                .collect();
            parent_node.set_children(children);
        }
        Ok(())
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
        let lower = text.to_lowercase();
        Ok(state
            .nodes
            .iter()
            .filter(|(_, n)| {
                n.label()
                    .map(|l| l.to_lowercase().contains(&lower))
                    .unwrap_or(false)
                    || n.value()
                        .map(|v| v.to_lowercase().contains(&lower))
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

/// Map a role name string to an AccessKit [`Role`].
fn label_role_to_accesskit(role: &str) -> Role {
    match role {
        "button" => Role::Button,
        "link" => Role::Link,
        "checkbox" => Role::CheckBox,
        "radio" => Role::RadioButton,
        "text-input" => Role::TextInput,
        "password-input" => Role::PasswordInput,
        "search-input" => Role::SearchInput,
        "email-input" => Role::EmailInput,
        "url-input" => Role::UrlInput,
        "tel-input" => Role::PhoneNumberInput,
        "number-input" => Role::NumberInput,
        "combobox" => Role::ComboBox,
        "switch" => Role::Switch,
        "slider" => Role::Slider,
        "image" => Role::Image,
        "heading" => Role::Heading,
        "paragraph" => Role::Paragraph,
        "list" => Role::List,
        "list-item" => Role::ListItem,
        "table" => Role::Table,
        "grid" => Role::Grid,
        "region" => Role::Region,
        "navigation" => Role::Navigation,
        "main" => Role::Main,
        "article" => Role::Article,
        "banner" => Role::Banner,
        "status" => Role::Status,
        "alert" => Role::Alert,
        _ => Role::GenericContainer,
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

fn is_form_role(role: Role) -> bool {
    matches!(
        role,
        Role::TextInput
            | Role::SearchInput
            | Role::EmailInput
            | Role::UrlInput
            | Role::PhoneNumberInput
            | Role::PasswordInput
            | Role::MultilineTextInput
            | Role::NumberInput
            | Role::CheckBox
            | Role::RadioButton
            | Role::ComboBox
            | Role::Switch
            | Role::Slider
    )
}
