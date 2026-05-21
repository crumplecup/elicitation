//! Runtime context management for headless and windowed egui.
//!
//! Provides [`EguiRuntimePlugin`] (a [`StatefulPlugin`]-style plugin) that
//! manages egui [`Context`] instances and renders [`UiNode`] trees.
//!
//! Gated behind the `runtime` feature (requires `uuid`).
//!
//! [`StatefulPlugin`]: elicitation::StatefulPlugin
//! [`Context`]: egui::Context
//! [`UiNode`]: crate::UiNode

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use egui::Color32;
use elicitation::PluginContext;
use elicitation::elicit_tool;
use elicitation_derive::ElicitPlugin;
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::serde_types::{
    ColorJson, ContainerJson, CornerRadiusJson, LayoutAlign, LayoutJson, MarginJson, RectJson,
    StrokeJson, UiNode, Vec2Json, WidgetJson,
};
use crate::style_tools::StyleJson;

// ===========================================================================
// Context type
// ===========================================================================

/// Runtime state shared across all egui context tools.
///
/// Each session has an independent [`egui::Context`] identified by a [`Uuid`].
#[derive(Debug)]
pub struct EguiRuntimeContext {
    sessions: Mutex<BTreeMap<Uuid, egui::Context>>,
}

impl PluginContext for EguiRuntimeContext {}

impl EguiRuntimeContext {
    /// Create a new runtime context with no active sessions.
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            sessions: Mutex::new(BTreeMap::new()),
        })
    }
}

impl Default for EguiRuntimeContext {
    fn default() -> Self {
        Self {
            sessions: Mutex::new(BTreeMap::new()),
        }
    }
}

// ===========================================================================
// Plugin
// ===========================================================================

/// Runtime plugin for egui context management.
///
/// Register with [`PluginRegistry`] to expose context creation, frame
/// rendering, and style application tools via MCP.
///
/// [`PluginRegistry`]: elicitation::PluginRegistry
#[derive(ElicitPlugin)]
#[plugin(name = "egui_runtime")]
pub struct EguiRuntimePlugin(pub Arc<EguiRuntimeContext>);

impl EguiRuntimePlugin {
    /// Create a plugin with a fresh runtime context.
    pub fn new() -> Self {
        Self(EguiRuntimeContext::new())
    }
}

impl Default for EguiRuntimePlugin {
    fn default() -> Self {
        Self::new()
    }
}

// ===========================================================================
// Frame output
// ===========================================================================

/// Simplified output from running a single egui frame.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FrameOutputJson {
    /// Whether the frame requested a repaint.
    pub needs_repaint: bool,
    /// URLs that should be opened (from hyperlinks etc.).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub open_urls: Vec<String>,
    /// Text copied to clipboard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copied_text: Option<String>,
    /// Number of widgets rendered.
    pub widget_count: usize,
}

// ===========================================================================
// Session info
// ===========================================================================

/// Information about an active egui session.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SessionInfo {
    /// Session identifier.
    pub id: String,
}

// ===========================================================================
// Tools
// ===========================================================================

/// Empty params.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EmptyRuntimeParams {}

// ---------------------------------------------------------------------------
// context_create
// ---------------------------------------------------------------------------

/// Create a new egui context session.
#[elicit_tool(
    plugin = "egui_runtime",
    name = "context_create",
    description = "Create a new egui context session. Returns a UUID for the session."
)]
#[instrument(skip_all)]
async fn context_create(
    ctx: Arc<EguiRuntimeContext>,
    _p: EmptyRuntimeParams,
) -> Result<CallToolResult, ErrorData> {
    let session_id = Uuid::new_v4();
    let egui_ctx = egui::Context::default();

    ctx.sessions
        .lock()
        .map_err(|e| ErrorData::internal_error(format!("lock: {e}"), None))?
        .insert(session_id, egui_ctx);

    let result = serde_json::json!({ "session_id": session_id.to_string() });
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

// ---------------------------------------------------------------------------
// context_destroy
// ---------------------------------------------------------------------------

/// Parameters for `context_destroy`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SessionIdParams {
    /// Session UUID string.
    pub session_id: String,
}

/// Destroy an egui context session.
#[elicit_tool(
    plugin = "egui_runtime",
    name = "context_destroy",
    description = "Destroy an egui context session by UUID."
)]
#[instrument(skip_all, fields(session_id = %p.session_id))]
async fn context_destroy(
    ctx: Arc<EguiRuntimeContext>,
    p: SessionIdParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.session_id)?;

    let removed = ctx
        .sessions
        .lock()
        .map_err(|e| ErrorData::internal_error(format!("lock: {e}"), None))?
        .remove(&id)
        .is_some();

    if removed {
        Ok(CallToolResult::success(vec![Content::text(
            r#"{"destroyed":true}"#,
        )]))
    } else {
        Err(ErrorData::invalid_params(
            format!("session not found: {id}"),
            None,
        ))
    }
}

// ---------------------------------------------------------------------------
// context_list
// ---------------------------------------------------------------------------

/// List all active egui sessions.
#[elicit_tool(
    plugin = "egui_runtime",
    name = "context_list",
    description = "List all active egui context sessions.",
    emit = None
)]
#[instrument(skip_all)]
async fn context_list(
    ctx: Arc<EguiRuntimeContext>,
    _p: EmptyRuntimeParams,
) -> Result<CallToolResult, ErrorData> {
    let sessions: Vec<SessionInfo> = ctx
        .sessions
        .lock()
        .map_err(|e| ErrorData::internal_error(format!("lock: {e}"), None))?
        .keys()
        .map(|id| SessionInfo { id: id.to_string() })
        .collect();

    let json = serde_json::to_string(&sessions)
        .map_err(|e| ErrorData::internal_error(format!("serialize: {e}"), None))?;

    Ok(CallToolResult::success(vec![Content::text(json)]))
}

// ---------------------------------------------------------------------------
// context_run_frame
// ---------------------------------------------------------------------------

/// Parameters for `context_run_frame`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RunFrameParams {
    /// Session UUID string.
    pub session_id: String,
    /// UI tree to render.
    pub ui_tree: Vec<UiNode>,
}

/// Run one egui frame, rendering a UI tree into the context.
#[elicit_tool(
    plugin = "egui_runtime",
    name = "context_run_frame",
    description = "Run one egui frame with a UI tree. Returns frame output info."
)]
#[instrument(skip_all, fields(session_id = %p.session_id, nodes = p.ui_tree.len()))]
async fn context_run_frame(
    ctx: Arc<EguiRuntimeContext>,
    p: RunFrameParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.session_id)?;

    let egui_ctx = {
        let sessions = ctx
            .sessions
            .lock()
            .map_err(|e| ErrorData::internal_error(format!("lock: {e}"), None))?;
        sessions
            .get(&id)
            .cloned()
            .ok_or_else(|| ErrorData::invalid_params(format!("session not found: {id}"), None))?
    };

    let ui_tree = p.ui_tree;
    let mut widget_count = 0usize;

    let full_output = egui_ctx.run_ui(egui::RawInput::default(), |ui| {
        for node in &ui_tree {
            widget_count += render_node(ui, node);
        }
    });

    let open_urls: Vec<String> = full_output
        .platform_output
        .commands
        .iter()
        .filter_map(|cmd| {
            if let egui::OutputCommand::OpenUrl(u) = cmd {
                Some(u.url.clone())
            } else {
                None
            }
        })
        .collect();

    let copied_text = full_output.platform_output.commands.iter().find_map(|cmd| {
        if let egui::OutputCommand::CopyText(t) = cmd {
            Some(t.clone())
        } else {
            None
        }
    });

    let needs_repaint = !full_output.viewport_output.is_empty();

    let output = FrameOutputJson {
        needs_repaint,
        open_urls,
        copied_text,
        widget_count,
    };

    let json = serde_json::to_string(&output)
        .map_err(|e| ErrorData::internal_error(format!("serialize: {e}"), None))?;

    Ok(CallToolResult::success(vec![Content::text(json)]))
}

// ---------------------------------------------------------------------------
// context_apply_style
// ---------------------------------------------------------------------------

/// Parameters for `context_apply_style`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ApplyStyleParams {
    /// Session UUID string.
    pub session_id: String,
    /// Style configuration to apply.
    pub style: StyleJson,
}

/// Apply a style configuration to an egui context.
#[elicit_tool(
    plugin = "egui_runtime",
    name = "context_apply_style",
    description = "Apply a StyleJson configuration to an egui context session."
)]
#[instrument(skip_all, fields(session_id = %p.session_id))]
async fn context_apply_style(
    ctx: Arc<EguiRuntimeContext>,
    p: ApplyStyleParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&p.session_id)?;

    let egui_ctx = {
        let sessions = ctx
            .sessions
            .lock()
            .map_err(|e| ErrorData::internal_error(format!("lock: {e}"), None))?;
        sessions
            .get(&id)
            .cloned()
            .ok_or_else(|| ErrorData::invalid_params(format!("session not found: {id}"), None))?
    };

    apply_style(&egui_ctx, &p.style);

    Ok(CallToolResult::success(vec![Content::text(
        r#"{"applied":true}"#,
    )]))
}

// ===========================================================================
// Rendering engine
// ===========================================================================

/// Render a [`UiNode`] tree into an egui [`Ui`]. Returns widget count.
pub fn render_node(ui: &mut egui::Ui, node: &UiNode) -> usize {
    match node {
        UiNode::Widget { widget } => {
            render_widget(ui, widget);
            1
        }
        UiNode::Container {
            container,
            children,
        } => render_container(ui, container, children),
        UiNode::Layout { layout, children } => render_layout(ui, layout, children),
    }
}

/// Render a single widget into the UI.
pub fn render_widget(ui: &mut egui::Ui, widget: &WidgetJson) {
    match widget {
        WidgetJson::Label { text, wrap, color } => {
            let mut rt = egui::RichText::new(text);
            if let Some(c) = color {
                rt = rt.color(color_json_to_egui(c));
            }
            let label = egui::Label::new(rt).wrap_mode(if *wrap {
                egui::TextWrapMode::Wrap
            } else {
                egui::TextWrapMode::Extend
            });
            ui.add(label);
        }
        WidgetJson::Button {
            text,
            fill,
            selected,
            ..
        } => {
            let mut btn = egui::Button::new(text);
            if let Some(f) = fill {
                btn = btn.fill(color_json_to_egui(f));
            }
            if *selected {
                btn = btn.selected(true);
            }
            ui.add(btn);
        }
        WidgetJson::SmallButton { text } => {
            let _ = ui.small_button(text);
        }
        WidgetJson::Checkbox { text, checked } => {
            let mut val = *checked;
            ui.checkbox(&mut val, text);
        }
        WidgetJson::RadioValue { text, selected } => {
            let mut val = *selected;
            ui.radio_value(&mut val, true, text);
        }
        WidgetJson::SelectableLabel { text, selected } => {
            let _ = ui.selectable_label(*selected, text);
        }
        WidgetJson::Hyperlink { text, url } => {
            ui.hyperlink_to(text, url);
        }
        WidgetJson::Heading { text } => {
            ui.heading(text);
        }
        WidgetJson::Monospace { text } => {
            ui.monospace(text);
        }
        WidgetJson::Code { text } => {
            ui.code(text);
        }
        WidgetJson::Small { text } => {
            ui.small(text);
        }
        WidgetJson::Strong { text } => {
            ui.strong(text);
        }
        WidgetJson::Weak { text } => {
            ui.weak(text);
        }
        WidgetJson::ColoredLabel { text, color } => {
            ui.colored_label(color_json_to_egui(color), text);
        }
        WidgetJson::Separator => {
            ui.separator();
        }
        WidgetJson::Spinner => {
            ui.spinner();
        }
        WidgetJson::TextEditSingleline { text, hint, .. } => {
            let mut buf = text.clone();
            let mut te = egui::TextEdit::singleline(&mut buf);
            if let Some(h) = hint {
                te = te.hint_text(h);
            }
            ui.add(te);
        }
        WidgetJson::TextEditMultiline { text, hint, .. } => {
            let mut buf = text.clone();
            let mut te = egui::TextEdit::multiline(&mut buf);
            if let Some(h) = hint {
                te = te.hint_text(h);
            }
            ui.add(te);
        }
        WidgetJson::CodeEditor { text, .. } => {
            let mut buf = text.clone();
            let te = egui::TextEdit::multiline(&mut buf).code_editor();
            ui.add(te);
        }
        WidgetJson::Slider {
            value,
            range,
            text,
            suffix,
            logarithmic,
            show_value,
            ..
        } => {
            let mut val = *value;
            let mut slider = egui::Slider::new(&mut val, range.min..=range.max);
            if let Some(t) = text {
                slider = slider.text(t);
            }
            if let Some(s) = suffix {
                slider = slider.suffix(s);
            }
            if *logarithmic {
                slider = slider.logarithmic(true);
            }
            if !show_value {
                slider = slider.show_value(false);
            }
            ui.add(slider);
        }
        WidgetJson::DragValue {
            value,
            range,
            speed,
            prefix,
            suffix,
            ..
        } => {
            let mut val = *value;
            let mut dv = egui::DragValue::new(&mut val);
            if let Some(r) = range {
                dv = dv.range(r.min..=r.max);
            }
            if let Some(s) = speed {
                dv = dv.speed(*s);
            }
            if let Some(p) = prefix {
                dv = dv.prefix(p.as_str());
            }
            if let Some(s) = suffix {
                dv = dv.suffix(s.as_str());
            }
            ui.add(dv);
        }
        WidgetJson::ProgressBar { progress, text, .. } => {
            let mut pb = egui::ProgressBar::new(*progress);
            if let Some(t) = text {
                pb = pb.text(t.as_str());
            }
            ui.add(pb);
        }
        WidgetJson::Image { uri, size, .. } => {
            let mut img = egui::Image::new(uri.as_str());
            if let Some(s) = size {
                img = img.fit_to_exact_size(egui::vec2(s.x, s.y));
            }
            ui.add(img);
        }
        WidgetJson::Link { text } => {
            let _ = ui.link(text);
        }
        WidgetJson::ToggleValue { text, selected } => {
            let mut val = *selected;
            ui.toggle_value(&mut val, text);
        }
        WidgetJson::Radio { text, selected } => {
            let _ = ui.radio(*selected, text);
        }
        WidgetJson::DragAngle { radians } => {
            let mut val = *radians as f32;
            ui.drag_angle(&mut val);
        }
        WidgetJson::DragAngleTau { radians } => {
            let mut val = *radians as f32;
            ui.drag_angle_tau(&mut val);
        }
        WidgetJson::ColorEditButtonSrgba { color, .. } => {
            let mut c = color_json_to_egui(color);
            ui.color_edit_button_srgba(&mut c);
        }
        WidgetJson::ColorEditButtonHsva { color, .. } => {
            let mut c = egui::epaint::Hsva::from(color_json_to_egui(color));
            ui.color_edit_button_hsva(&mut c);
        }
        WidgetJson::SliderVertical {
            value, range, text, ..
        } => {
            let mut val = *value;
            let mut slider = egui::Slider::new(&mut val, range.min..=range.max).vertical();
            if let Some(t) = text {
                slider = slider.text(t);
            }
            ui.add(slider);
        }
    }
}

/// Render a container with its children. Returns total widget count.
fn render_container(ui: &mut egui::Ui, container: &ContainerJson, children: &[UiNode]) -> usize {
    match container {
        ContainerJson::CentralPanel | ContainerJson::Group | ContainerJson::MenuBar => {
            // These container types just wrap children directly
            let mut count = 0;
            ui.group(|ui| {
                for child in children {
                    count += render_node(ui, child);
                }
            });
            count
        }

        ContainerJson::ScrollArea {
            vertical,
            horizontal,
            max_height,
            ..
        } => {
            let mut sa = egui::ScrollArea::new([*horizontal, *vertical]);
            if let Some(mh) = max_height {
                sa = sa.max_height(*mh);
            }
            let mut count = 0;
            sa.show(ui, |ui| {
                for child in children {
                    count += render_node(ui, child);
                }
            });
            count
        }

        ContainerJson::CollapsingHeader { text, default_open } => {
            let mut count = 0;
            egui::CollapsingHeader::new(text)
                .default_open(*default_open)
                .show(ui, |ui| {
                    for child in children {
                        count += render_node(ui, child);
                    }
                });
            count
        }

        ContainerJson::Frame {
            fill,
            stroke,
            corner_radius,
            inner_margin,
            outer_margin,
        } => {
            let mut frame = egui::Frame::default();
            if let Some(f) = fill {
                frame = frame.fill(color_json_to_egui(f));
            }
            if let Some(s) = stroke {
                frame = frame.stroke(stroke_json_to_egui(s));
            }
            if let Some(cr) = corner_radius {
                frame = frame.corner_radius(corner_radius_json_to_egui(cr));
            }
            if let Some(im) = inner_margin {
                frame = frame.inner_margin(margin_json_to_egui(im));
            }
            if let Some(om) = outer_margin {
                frame = frame.outer_margin(margin_json_to_egui(om));
            }
            let mut count = 0;
            frame.show(ui, |ui| {
                for child in children {
                    count += render_node(ui, child);
                }
            });
            count
        }

        ContainerJson::Menu { title } => {
            let mut count = 0;
            ui.menu_button(title, |ui| {
                for child in children {
                    count += render_node(ui, child);
                }
            });
            count
        }

        ContainerJson::Tooltip { text } => {
            ui.label(text).on_hover_ui(|ui| {
                let mut count = 0;
                for child in children {
                    count += render_node(ui, child);
                }
                let _ = count;
            });
            children.len()
        }

        // Panels and windows require egui::Context, not &mut Ui.
        // Inside a render pass they degrade to groups.
        ContainerJson::Window { title, .. } => {
            let mut count = 0;
            ui.group(|ui| {
                ui.heading(title);
                for child in children {
                    count += render_node(ui, child);
                }
            });
            count
        }

        ContainerJson::LeftPanel { .. }
        | ContainerJson::RightPanel { .. }
        | ContainerJson::TopPanel { .. }
        | ContainerJson::BottomPanel { .. } => {
            let mut count = 0;
            ui.group(|ui| {
                for child in children {
                    count += render_node(ui, child);
                }
            });
            count
        }

        ContainerJson::Popup { .. } => {
            // Popups need an Id and click state; degrade to group
            let mut count = 0;
            ui.group(|ui| {
                for child in children {
                    count += render_node(ui, child);
                }
            });
            count
        }
    }
}

/// Render a layout with its children. Returns total widget count.
fn render_layout(ui: &mut egui::Ui, layout: &LayoutJson, children: &[UiNode]) -> usize {
    match layout {
        LayoutJson::Horizontal { align } => {
            let layout = egui::Layout::left_to_right(align_to_egui(align));
            let mut count = 0;
            ui.with_layout(layout, |ui| {
                for child in children {
                    count += render_node(ui, child);
                }
            });
            count
        }

        LayoutJson::Vertical { align } => {
            let layout = egui::Layout::top_down(align_to_egui(align));
            let mut count = 0;
            ui.with_layout(layout, |ui| {
                for child in children {
                    count += render_node(ui, child);
                }
            });
            count
        }

        LayoutJson::HorizontalCentered => {
            let mut count = 0;
            ui.horizontal_centered(|ui| {
                for child in children {
                    count += render_node(ui, child);
                }
            });
            count
        }

        LayoutJson::VerticalCentered => {
            let mut count = 0;
            ui.vertical_centered(|ui| {
                for child in children {
                    count += render_node(ui, child);
                }
            });
            count
        }

        LayoutJson::HorizontalJustified => {
            let layout = egui::Layout::left_to_right(egui::Align::Center).with_cross_justify(true);
            let mut count = 0;
            ui.with_layout(layout, |ui| {
                for child in children {
                    count += render_node(ui, child);
                }
            });
            count
        }

        LayoutJson::VerticalJustified => {
            let mut count = 0;
            ui.vertical_centered_justified(|ui| {
                for child in children {
                    count += render_node(ui, child);
                }
            });
            count
        }

        LayoutJson::HorizontalWrapped => {
            let mut count = 0;
            ui.horizontal_wrapped(|ui| {
                for child in children {
                    count += render_node(ui, child);
                }
            });
            count
        }

        LayoutJson::Columns { count: col_count } => {
            let mut widget_count = 0;
            ui.columns(*col_count, |cols| {
                for (i, child) in children.iter().enumerate() {
                    if let Some(col) = cols.get_mut(i % *col_count) {
                        widget_count += render_node(col, child);
                    }
                }
            });
            widget_count
        }

        LayoutJson::Grid {
            id,
            num_columns,
            striped,
            ..
        } => {
            let mut grid = egui::Grid::new(id).striped(*striped);
            if let Some(nc) = num_columns {
                grid = grid.num_columns(*nc);
            }
            let mut count = 0;
            grid.show(ui, |ui| {
                for child in children {
                    count += render_node(ui, child);
                }
            });
            count
        }

        LayoutJson::Indent { indent } => {
            let mut count = 0;
            ui.indent("indent", |ui| {
                if let Some(px) = indent {
                    ui.add_space(*px);
                }
                for child in children {
                    count += render_node(ui, child);
                }
            });
            count
        }

        LayoutJson::AddSpace { amount } => {
            ui.add_space(*amount);
            let mut count = 0;
            for child in children {
                count += render_node(ui, child);
            }
            count
        }
    }
}

// ===========================================================================
// Style application
// ===========================================================================

/// Apply a [`StyleJson`] to an [`egui::Context`].
pub fn apply_style(ctx: &egui::Context, style: &StyleJson) {
    match style {
        StyleJson::DarkMode => {
            ctx.set_visuals(egui::Visuals::dark());
        }
        StyleJson::LightMode => {
            ctx.set_visuals(egui::Visuals::light());
        }
        StyleJson::Spacing {
            item_spacing,
            window_margin,
            button_padding,
            indent,
        } => {
            let mut s = (*ctx.global_style()).clone();
            if let Some(is) = item_spacing {
                s.spacing.item_spacing = egui::vec2(is.x, is.y);
            }
            if let Some(wm) = window_margin {
                s.spacing.window_margin = egui::Margin::symmetric(wm.x as i8, wm.y as i8);
            }
            if let Some(bp) = button_padding {
                s.spacing.button_padding = egui::vec2(bp.x, bp.y);
            }
            if let Some(i) = indent {
                s.spacing.indent = *i;
            }
            ctx.set_global_style(s);
        }
        StyleJson::Visual { property, color } => {
            let mut v = ctx.global_style().visuals.clone();
            let c = color_json_to_egui(color);
            use crate::style_tools::VisualProperty;
            match property {
                VisualProperty::HyperlinkColor => v.hyperlink_color = c,
                VisualProperty::FaintBgColor => v.faint_bg_color = c,
                VisualProperty::ExtremeBgColor => v.extreme_bg_color = c,
                VisualProperty::CodeBgColor => v.code_bg_color = c,
                VisualProperty::WarnFgColor => v.warn_fg_color = c,
                VisualProperty::ErrorFgColor => v.error_fg_color = c,
                VisualProperty::WindowFill => v.window_fill = c,
                VisualProperty::PanelFill => v.panel_fill = c,
            }
            ctx.set_visuals(v);
        }
        StyleJson::WindowRounding { corner_radius } => {
            let mut v = ctx.global_style().visuals.clone();
            v.window_corner_radius = corner_radius_json_to_egui(corner_radius);
            ctx.set_visuals(v);
        }
        StyleJson::WindowShadow {
            offset_x,
            offset_y,
            blur,
            color,
        } => {
            let mut v = ctx.global_style().visuals.clone();
            v.window_shadow = egui::epaint::Shadow {
                offset: [*offset_x as i8, *offset_y as i8],
                blur: *blur as u8,
                spread: 0,
                color: color_json_to_egui(color),
            };
            ctx.set_visuals(v);
        }
        StyleJson::WidgetVisuals {
            state,
            bg_fill,
            weak_bg_fill,
            bg_stroke,
            corner_radius,
            fg_stroke,
        } => {
            let mut v = ctx.global_style().visuals.clone();
            let wv = match state {
                crate::style_tools::WidgetState::Noninteractive => &mut v.widgets.noninteractive,
                crate::style_tools::WidgetState::Inactive => &mut v.widgets.inactive,
                crate::style_tools::WidgetState::Hovered => &mut v.widgets.hovered,
                crate::style_tools::WidgetState::Active => &mut v.widgets.active,
                crate::style_tools::WidgetState::Open => &mut v.widgets.open,
            };
            if let Some(f) = bg_fill {
                wv.bg_fill = color_json_to_egui(f);
            }
            if let Some(f) = weak_bg_fill {
                wv.weak_bg_fill = color_json_to_egui(f);
            }
            if let Some(s) = bg_stroke {
                wv.bg_stroke = stroke_json_to_egui(s);
            }
            if let Some(cr) = corner_radius {
                wv.corner_radius = corner_radius_json_to_egui(cr);
            }
            if let Some(s) = fg_stroke {
                wv.fg_stroke = stroke_json_to_egui(s);
            }
            ctx.set_visuals(v);
        }
        StyleJson::SelectionColor { bg_fill, stroke } => {
            let mut v = ctx.global_style().visuals.clone();
            v.selection.bg_fill = color_json_to_egui(bg_fill);
            v.selection.stroke = stroke_json_to_egui(stroke);
            ctx.set_visuals(v);
        }
        StyleJson::TextCursor { color, width } => {
            let mut v = ctx.global_style().visuals.clone();
            if let Some(c) = color {
                v.text_cursor.stroke.color = color_json_to_egui(c);
            }
            if let Some(w) = width {
                v.text_cursor.stroke.width = *w;
            }
            ctx.set_visuals(v);
        }
        StyleJson::SetFonts { family, font_names } => {
            let mut fonts = egui::FontDefinitions::default();
            let egui_family = match family {
                crate::style_tools::FontFamily::Proportional => egui::FontFamily::Proportional,
                crate::style_tools::FontFamily::Monospace => egui::FontFamily::Monospace,
            };
            fonts
                .families
                .entry(egui_family)
                .or_default()
                .clone_from(font_names);
            ctx.set_fonts(fonts);
        }
        StyleJson::OverrideTextStyle {
            style: ts,
            family,
            size,
        } => {
            let egui_family = match family {
                crate::style_tools::FontFamily::Proportional => egui::FontFamily::Proportional,
                crate::style_tools::FontFamily::Monospace => egui::FontFamily::Monospace,
            };
            let egui_ts = match ts {
                crate::style_tools::TextStyleName::Heading => egui::TextStyle::Heading,
                crate::style_tools::TextStyleName::Body => egui::TextStyle::Body,
                crate::style_tools::TextStyleName::Monospace => egui::TextStyle::Monospace,
                crate::style_tools::TextStyleName::Button => egui::TextStyle::Button,
                crate::style_tools::TextStyleName::Small => egui::TextStyle::Small,
            };
            let mut s = (*ctx.global_style()).clone();
            s.text_styles
                .insert(egui_ts, egui::FontId::new(*size, egui_family));
            ctx.set_global_style(s);
        }
        StyleJson::SetTextValign { valign } => {
            let mut s = (*ctx.global_style()).clone();
            s.explanation_tooltips = matches!(valign, crate::style_tools::TextValign::Bottom);
            // Note: egui doesn't have a direct text valign setting; this is a best-effort mapping
            ctx.set_global_style(s);
        }
        StyleJson::Interaction {
            tooltip_delay,
            show_tooltips_only_when_still,
            ..
        } => {
            let mut s = (*ctx.global_style()).clone();
            if let Some(t) = tooltip_delay {
                s.interaction.tooltip_delay = *t;
            }
            if let Some(b) = show_tooltips_only_when_still {
                s.interaction.show_tooltips_only_when_still = *b;
            }
            ctx.set_global_style(s);
        }
        StyleJson::AnimationTime { duration } => {
            let mut s = (*ctx.global_style()).clone();
            s.animation_time = *duration;
            ctx.set_global_style(s);
        }
        StyleJson::DebugOptions {
            show_widget_hits,
            debug_on_hover,
            show_resize,
            show_interactive_widgets,
            ..
        } => {
            let mut s = (*ctx.global_style()).clone();
            if let Some(v) = show_widget_hits {
                s.debug.show_widget_hits = *v;
            }
            #[cfg(debug_assertions)]
            if let Some(v) = debug_on_hover {
                s.debug.debug_on_hover = *v;
            }
            #[cfg(not(debug_assertions))]
            let _ = debug_on_hover;
            if let Some(v) = show_resize {
                s.debug.show_resize = *v;
            }
            if let Some(v) = show_interactive_widgets {
                s.debug.show_interactive_widgets = *v;
            }
            ctx.set_global_style(s);
        }
        StyleJson::WindowStroke { stroke } => {
            let mut v = ctx.global_style().visuals.clone();
            v.window_stroke = stroke_json_to_egui(stroke);
            ctx.set_visuals(v);
        }
        StyleJson::MenuMargin { margin } => {
            let mut s = (*ctx.global_style()).clone();
            s.spacing.menu_margin = margin_json_to_egui(margin);
            ctx.set_global_style(s);
        }
        StyleJson::ScrollBar {
            bar_width,
            handle_min_length,
            bar_inner_margin,
            bar_outer_margin,
            floating,
        } => {
            let mut s = (*ctx.global_style()).clone();
            if let Some(w) = bar_width {
                s.spacing.scroll.bar_width = *w;
            }
            if let Some(l) = handle_min_length {
                s.spacing.scroll.handle_min_length = *l;
            }
            if let Some(m) = bar_inner_margin {
                s.spacing.scroll.bar_inner_margin = *m;
            }
            if let Some(m) = bar_outer_margin {
                s.spacing.scroll.bar_outer_margin = *m;
            }
            if let Some(f) = floating {
                s.spacing.scroll.floating = *f;
            }
            ctx.set_global_style(s);
        }
        StyleJson::ResizeGripSize { size } => {
            let mut v = ctx.global_style().visuals.clone();
            v.resize_corner_size = *size;
            ctx.set_visuals(v);
        }
        StyleJson::TextCursorBlink {
            width,
            blink_on,
            blink_off,
            preview,
        } => {
            let mut v = ctx.global_style().visuals.clone();
            if let Some(w) = width {
                v.text_cursor.stroke.width = *w;
            }
            if let Some(on) = blink_on {
                v.text_cursor.on_duration = *on;
            }
            if let Some(off) = blink_off {
                v.text_cursor.off_duration = *off;
            }
            if let Some(p) = preview {
                v.text_cursor.preview = *p;
            }
            ctx.set_visuals(v);
        }
    }
}

// ===========================================================================
// Conversion helpers
// ===========================================================================

fn parse_uuid(s: &str) -> Result<Uuid, ErrorData> {
    s.parse::<Uuid>()
        .map_err(|e| ErrorData::invalid_params(format!("invalid UUID: {e}"), None))
}

/// Convert [`ColorJson`] to [`egui::Color32`].
pub fn color_json_to_egui(c: &ColorJson) -> Color32 {
    Color32::from_rgba_premultiplied(c.r, c.g, c.b, c.a)
}

/// Convert [`StrokeJson`] to [`egui::Stroke`].
pub fn stroke_json_to_egui(s: &StrokeJson) -> egui::Stroke {
    egui::Stroke::new(s.width, color_json_to_egui(&s.color))
}

/// Convert [`CornerRadiusJson`] to [`egui::CornerRadius`].
pub fn corner_radius_json_to_egui(cr: &CornerRadiusJson) -> egui::CornerRadius {
    egui::CornerRadius {
        nw: cr.nw,
        ne: cr.ne,
        sw: cr.sw,
        se: cr.se,
    }
}

/// Convert [`MarginJson`] to [`egui::Margin`].
pub fn margin_json_to_egui(m: &MarginJson) -> egui::Margin {
    egui::Margin {
        left: m.left as i8,
        right: m.right as i8,
        top: m.top as i8,
        bottom: m.bottom as i8,
    }
}

/// Convert [`RectJson`] to [`egui::Rect`].
pub fn rect_json_to_egui(r: &RectJson) -> egui::Rect {
    egui::Rect::from_min_max(egui::pos2(r.min_x, r.min_y), egui::pos2(r.max_x, r.max_y))
}

/// Convert [`Vec2Json`] to [`egui::Vec2`].
pub fn vec2_json_to_egui(v: &Vec2Json) -> egui::Vec2 {
    egui::vec2(v.x, v.y)
}

fn align_to_egui(align: &Option<LayoutAlign>) -> egui::Align {
    match align {
        Some(LayoutAlign::Min) => egui::Align::Min,
        Some(LayoutAlign::Center) | None => egui::Align::Center,
        Some(LayoutAlign::Max) => egui::Align::Max,
    }
}
