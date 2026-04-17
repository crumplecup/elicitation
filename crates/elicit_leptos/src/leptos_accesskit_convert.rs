//! Bidirectional bridge: AccessKit trees ↔ Leptos output.
//!
//! Converts a verified AccessKit node tree into either:
//!
//! 1. **Semantic HTML5 string** — for SSR delivery via axum/tower.
//! 2. **Leptos `view!` macro source** — for CSR/WASM compilation or
//!    code-generation pipelines.
//!
//! # Role mapping
//!
//! Every [`accesskit::Role`] variant maps to the closest semantic HTML5
//! element.  When a single correct mapping exists it is used unconditionally.
//! When multiple layouts are reasonable (e.g. horizontal vs. vertical list),
//! the AccessKit [`Orientation`] attribute on the node determines the output
//! and no extra user-facing enum is required.
//!
//! # Usage
//!
//! ```rust
//! use accesskit::{Node, NodeId, Role, Tree, TreeId, TreeUpdate};
//! use std::collections::HashMap;
//! use elicit_leptos::leptos_accesskit_convert::{render_tree, LeptosRenderMode};
//!
//! let root_id = NodeId::from(0u64);
//! let mut root = Node::new(Role::Main);
//! root.set_label("My App");
//!
//! let mut nodes = HashMap::new();
//! nodes.insert(root_id, root);
//!
//! let html = render_tree(&nodes, root_id, LeptosRenderMode::Html);
//! assert!(html.contains("<main"));
//!
//! let code = render_tree(&nodes, root_id, LeptosRenderMode::ViewMacro);
//! assert!(code.contains("<main"));
//! ```

use accesskit::{Node, NodeId, Orientation, Role, Toggled};
use elicit_ui::{ColorTheme, RenderStats};
use std::collections::HashMap;

// ── Render mode ───────────────────────────────────────────────────────────────

/// Output format for Leptos rendering.
///
/// Both modes walk the same AccessKit tree; they differ only in how strings
/// are quoted and whether self-closing tags are `<br />` or `<br>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LeptosRenderMode {
    /// Produce a semantic HTML5 string suitable for SSR via axum/tower.
    #[default]
    Html,
    /// Produce Leptos `view!` macro source code for CSR/WASM or codegen.
    ///
    /// Text content is wrapped in `"..."` and the output is ready to paste
    /// inside a `view! { … }` block.
    ViewMacro,
}

// ── Public entry points ───────────────────────────────────────────────────────

/// Render an AccessKit node tree to a Leptos output string.
///
/// Returns the rendered string.  Collect [`RenderStats`] by passing a
/// mutable reference; use `RenderStats::default()` if you don't need them.
pub fn render_tree(nodes: &HashMap<NodeId, Node>, root: NodeId, mode: LeptosRenderMode) -> String {
    let mut stats = RenderStats::default();
    render_node(nodes, root, mode, 0, &mut stats)
}

/// Render with stats collection.
pub fn render_tree_with_stats(
    nodes: &HashMap<NodeId, Node>,
    root: NodeId,
    mode: LeptosRenderMode,
) -> (String, RenderStats) {
    let mut stats = RenderStats::default();
    let output = render_node(nodes, root, mode, 0, &mut stats);
    (output, stats)
}

// ── Recursive renderer ────────────────────────────────────────────────────────

fn render_node(
    nodes: &HashMap<NodeId, Node>,
    node_id: NodeId,
    mode: LeptosRenderMode,
    depth: usize,
    stats: &mut RenderStats,
) -> String {
    let Some(node) = nodes.get(&node_id) else {
        stats.nodes_skipped += 1;
        return String::new();
    };
    if node.is_hidden() {
        stats.nodes_skipped += 1;
        return String::new();
    }
    stats.nodes_visited += 1;

    let children = node.children();
    let has_children = !children.is_empty();

    let rendered = match node.role() {
        // ── Document-level containers ─────────────────────────────────────────
        Role::Window => {
            // Transparent: render children at the same depth without a wrapper.
            render_children(nodes, children, mode, depth, stats)
        }
        Role::Pane | Role::GenericContainer => {
            stats.containers_rendered += 1;
            let desc_attrs = desc_attrs_str(node);
            if !desc_attrs.is_empty() {
                let aria = aria_label_attr(node);
                let inner = render_children(nodes, children, mode, depth + 1, stats);
                let pad = indent(depth);
                if inner.is_empty() {
                    format!("{pad}<div{desc_attrs}{aria}></div>\n")
                } else {
                    format!("{pad}<div{desc_attrs}{aria}>\n{inner}{pad}</div>\n")
                }
            } else {
                let orient = orientation_class(node);
                wrap_element(
                    "div",
                    orient.as_deref(),
                    node,
                    nodes,
                    children,
                    mode,
                    depth,
                    stats,
                )
            }
        }
        Role::Document => {
            stats.containers_rendered += 1;
            wrap_element("article", None, node, nodes, children, mode, depth, stats)
        }
        Role::Main => {
            stats.containers_rendered += 1;
            wrap_element("main", None, node, nodes, children, mode, depth, stats)
        }
        Role::Banner => {
            stats.containers_rendered += 1;
            wrap_element("header", None, node, nodes, children, mode, depth, stats)
        }
        Role::ContentInfo => {
            stats.containers_rendered += 1;
            wrap_element("footer", None, node, nodes, children, mode, depth, stats)
        }
        Role::Navigation => {
            stats.containers_rendered += 1;
            let desc_attrs = desc_attrs_str(node);
            let aria = aria_label_attr(node);
            let inner = render_children(nodes, children, mode, depth + 1, stats);
            let pad = indent(depth);
            if inner.is_empty() {
                format!("{pad}<nav{desc_attrs}{aria}></nav>\n")
            } else {
                format!("{pad}<nav{desc_attrs}{aria}>\n{inner}{pad}</nav>\n")
            }
        }
        Role::Complementary => {
            stats.containers_rendered += 1;
            wrap_element("aside", None, node, nodes, children, mode, depth, stats)
        }
        Role::Section | Role::Region => {
            stats.containers_rendered += 1;
            wrap_element("section", None, node, nodes, children, mode, depth, stats)
        }
        Role::Article => {
            stats.containers_rendered += 1;
            wrap_element("article", None, node, nodes, children, mode, depth, stats)
        }
        Role::Form => {
            stats.containers_rendered += 1;
            wrap_element("form", None, node, nodes, children, mode, depth, stats)
        }
        Role::Search => {
            stats.containers_rendered += 1;
            wrap_with_role("div", "search", node, nodes, children, mode, depth, stats)
        }
        Role::Group => {
            stats.containers_rendered += 1;
            let desc_attrs = desc_attrs_str(node);
            if !desc_attrs.is_empty() {
                // desc_attrs encoding takes priority: render as <div> with those attrs.
                let aria = aria_label_attr(node);
                let inner = render_children(nodes, children, mode, depth + 1, stats);
                let pad = indent(depth);
                if inner.is_empty() {
                    format!("{pad}<div{desc_attrs}{aria}></div>\n")
                } else {
                    format!("{pad}<div{desc_attrs}{aria}>\n{inner}{pad}</div>\n")
                }
            } else if node.label().is_some() {
                let label = node.label().unwrap_or("").to_string();
                let inner = render_children(nodes, children, mode, depth + 1, stats);
                format!(
                    "{pad}<fieldset>\n{pad}  <legend>{}</legend>\n{inner}{pad}</fieldset>\n",
                    text_content(&label, mode),
                    pad = indent(depth),
                )
            } else {
                wrap_with_role("div", "group", node, nodes, children, mode, depth, stats)
            }
        }
        Role::Dialog => {
            stats.containers_rendered += 1;
            wrap_element("dialog", None, node, nodes, children, mode, depth, stats)
        }
        Role::AlertDialog => {
            stats.containers_rendered += 1;
            wrap_with_role(
                "dialog",
                "alertdialog",
                node,
                nodes,
                children,
                mode,
                depth,
                stats,
            )
        }
        Role::ScrollView => {
            stats.containers_rendered += 1;
            let class = Some("ak-scroll");
            wrap_element_class("div", class, node, nodes, children, mode, depth, stats)
        }
        Role::SectionHeader | Role::Header => {
            stats.containers_rendered += 1;
            wrap_element("header", None, node, nodes, children, mode, depth, stats)
        }
        Role::SectionFooter | Role::Footer => {
            stats.containers_rendered += 1;
            wrap_element("footer", None, node, nodes, children, mode, depth, stats)
        }

        // ── Toolbar ───────────────────────────────────────────────────────────
        Role::Toolbar => {
            stats.containers_rendered += 1;
            let desc_attrs = desc_attrs_str(node);
            let aria = aria_label_attr(node);
            let inner = render_children(nodes, children, mode, depth + 1, stats);
            let pad = indent(depth);
            if inner.is_empty() {
                format!("{pad}<header class=\"toolbar\"{desc_attrs}{aria}></header>\n")
            } else {
                format!(
                    "{pad}<header class=\"toolbar\"{desc_attrs}{aria}>\n{inner}{pad}</header>\n"
                )
            }
        }

        // ── Lists ─────────────────────────────────────────────────────────────
        Role::List | Role::Feed => {
            stats.containers_rendered += 1;
            // Horizontal orientation → flex row list
            if is_horizontal(node) {
                wrap_element_class(
                    "ul",
                    Some("ak-hlist"),
                    node,
                    nodes,
                    children,
                    mode,
                    depth,
                    stats,
                )
            } else {
                wrap_element("ul", None, node, nodes, children, mode, depth, stats)
            }
        }
        Role::ListBox => {
            stats.containers_rendered += 1;
            wrap_with_role("ul", "listbox", node, nodes, children, mode, depth, stats)
        }
        Role::DescriptionList => {
            stats.containers_rendered += 1;
            wrap_element("dl", None, node, nodes, children, mode, depth, stats)
        }
        Role::ListItem | Role::ListBoxOption => {
            if has_children {
                stats.containers_rendered += 1;
                wrap_element("li", None, node, nodes, children, mode, depth, stats)
            } else {
                stats.widgets_rendered += 1;
                let text = node_text(node);
                format!("{}<li>{}</li>\n", indent(depth), text_content(&text, mode))
            }
        }
        Role::Term => {
            stats.widgets_rendered += 1;
            let text = node_text(node);
            format!("{}<dt>{}</dt>\n", indent(depth), text_content(&text, mode))
        }
        Role::Definition => {
            stats.widgets_rendered += 1;
            let text = node_text(node);
            format!("{}<dd>{}</dd>\n", indent(depth), text_content(&text, mode))
        }

        // ── Tables ────────────────────────────────────────────────────────────
        Role::Table => {
            stats.containers_rendered += 1;
            render_table(nodes, node, children, "table", None, mode, depth, stats)
        }
        Role::Grid | Role::TreeGrid | Role::ListGrid => {
            stats.containers_rendered += 1;
            render_table(
                nodes,
                node,
                children,
                "table",
                Some("grid"),
                mode,
                depth,
                stats,
            )
        }
        Role::RowGroup => {
            stats.containers_rendered += 1;
            wrap_element("tbody", None, node, nodes, children, mode, depth, stats)
        }
        Role::Row => {
            stats.containers_rendered += 1;
            wrap_element("tr", None, node, nodes, children, mode, depth, stats)
        }
        Role::Cell | Role::GridCell => {
            if has_children {
                stats.containers_rendered += 1;
                wrap_element("td", None, node, nodes, children, mode, depth, stats)
            } else {
                stats.widgets_rendered += 1;
                let text = node_text(node);
                format!("{}<td>{}</td>\n", indent(depth), text_content(&text, mode))
            }
        }
        Role::ColumnHeader => {
            if has_children {
                stats.containers_rendered += 1;
                wrap_element(
                    "th",
                    Some("scope=\"col\""),
                    node,
                    nodes,
                    children,
                    mode,
                    depth,
                    stats,
                )
            } else {
                stats.widgets_rendered += 1;
                let text = node_text(node);
                format!(
                    r#"{}<th scope="col">{}</th>{}"#,
                    indent(depth),
                    text_content(&text, mode),
                    "\n"
                )
            }
        }
        Role::RowHeader => {
            if has_children {
                stats.containers_rendered += 1;
                wrap_element(
                    "th",
                    Some("scope=\"row\""),
                    node,
                    nodes,
                    children,
                    mode,
                    depth,
                    stats,
                )
            } else {
                stats.widgets_rendered += 1;
                let text = node_text(node);
                format!(
                    r#"{}<th scope="row">{}</th>{}"#,
                    indent(depth),
                    text_content(&text, mode),
                    "\n"
                )
            }
        }

        // ── Tabs ──────────────────────────────────────────────────────────────
        Role::TabList => {
            stats.containers_rendered += 1;
            wrap_with_role("div", "tablist", node, nodes, children, mode, depth, stats)
        }
        Role::Tab => {
            stats.widgets_rendered += 1;
            let text = node_text(node);
            let selected = matches!(node.toggled(), Some(Toggled::True));
            format!(
                r#"{}<button role="tab" aria-selected="{}">{}</button>{}"#,
                indent(depth),
                selected,
                text_content(&text, mode),
                "\n"
            )
        }
        Role::TabPanel => {
            stats.containers_rendered += 1;
            wrap_with_role("div", "tabpanel", node, nodes, children, mode, depth, stats)
        }

        // ── Menus ─────────────────────────────────────────────────────────────
        Role::MenuBar => {
            stats.containers_rendered += 1;
            wrap_with_role("nav", "menubar", node, nodes, children, mode, depth, stats)
        }
        Role::Menu | Role::MenuListPopup => {
            stats.containers_rendered += 1;
            wrap_with_role("ul", "menu", node, nodes, children, mode, depth, stats)
        }
        Role::MenuItem => {
            if has_children {
                stats.containers_rendered += 1;
                wrap_with_role("li", "menuitem", node, nodes, children, mode, depth, stats)
            } else {
                stats.widgets_rendered += 1;
                let text = node_text(node);
                format!(
                    r#"{}<li role="menuitem">{}</li>{}"#,
                    indent(depth),
                    text_content(&text, mode),
                    "\n"
                )
            }
        }
        Role::MenuItemCheckBox => {
            stats.widgets_rendered += 1;
            let text = node_text(node);
            let checked = matches!(node.toggled(), Some(Toggled::True));
            format!(
                r#"{}<li role="menuitemcheckbox" aria-checked="{}">{}</li>{}"#,
                indent(depth),
                checked,
                text_content(&text, mode),
                "\n"
            )
        }
        Role::MenuItemRadio => {
            stats.widgets_rendered += 1;
            let text = node_text(node);
            let checked = matches!(node.toggled(), Some(Toggled::True));
            format!(
                r#"{}<li role="menuitemradio" aria-checked="{}">{}</li>{}"#,
                indent(depth),
                checked,
                text_content(&text, mode),
                "\n"
            )
        }
        Role::MenuListOption => {
            stats.widgets_rendered += 1;
            let text = node_text(node);
            let selected = matches!(node.toggled(), Some(Toggled::True));
            format!(
                r#"{}<option value="{}" aria-selected="{}">{}</option>{}"#,
                indent(depth),
                html_escape(&text),
                selected,
                text_content(&text, mode),
                "\n"
            )
        }

        // ── Interactive controls ───────────────────────────────────────────────
        Role::Button | Role::DefaultButton => {
            stats.widgets_rendered += 1;
            let text = node_text(node);
            let disabled = if node.is_disabled() { " disabled" } else { "" };
            let desc_attrs = desc_attrs_str(node);
            format!(
                "{}<button{}{}>{}</button>\n",
                indent(depth),
                desc_attrs,
                disabled,
                text_content(&text, mode)
            )
        }
        Role::CheckBox => {
            stats.widgets_rendered += 1;
            checkbox_html(node, "checkbox", depth, mode)
        }
        Role::RadioButton => {
            stats.widgets_rendered += 1;
            checkbox_html(node, "radio", depth, mode)
        }
        Role::Switch => {
            stats.widgets_rendered += 1;
            let text = node_text(node);
            let checked = matches!(node.toggled(), Some(Toggled::True));
            let disabled = if node.is_disabled() { " disabled" } else { "" };
            format!(
                r#"{}<button role="switch" aria-checked="{}"{}>{}</button>{}"#,
                indent(depth),
                checked,
                disabled,
                text_content(&text, mode),
                "\n"
            )
        }
        Role::TextInput => {
            stats.widgets_rendered += 1;
            text_input_html(node, "text", depth, mode)
        }
        Role::SearchInput => {
            stats.widgets_rendered += 1;
            text_input_html(node, "search", depth, mode)
        }
        Role::EmailInput => {
            stats.widgets_rendered += 1;
            text_input_html(node, "email", depth, mode)
        }
        Role::UrlInput => {
            stats.widgets_rendered += 1;
            text_input_html(node, "url", depth, mode)
        }
        Role::PhoneNumberInput => {
            stats.widgets_rendered += 1;
            text_input_html(node, "tel", depth, mode)
        }
        Role::PasswordInput => {
            stats.widgets_rendered += 1;
            text_input_html(node, "password", depth, mode)
        }
        Role::NumberInput | Role::SpinButton => {
            stats.widgets_rendered += 1;
            numeric_input_html(node, depth, mode)
        }
        Role::MultilineTextInput => {
            stats.widgets_rendered += 1;
            textarea_html(node, depth, mode)
        }
        Role::Slider => {
            stats.widgets_rendered += 1;
            range_input_html(node, depth, mode)
        }
        Role::ProgressIndicator => {
            stats.widgets_rendered += 1;
            progress_html(node, depth, mode)
        }
        Role::Meter => {
            stats.widgets_rendered += 1;
            meter_html(node, depth, mode)
        }
        Role::ComboBox | Role::EditableComboBox => {
            stats.containers_rendered += 1;
            let label = node_text(node);
            let inner = render_children(nodes, children, mode, depth + 1, stats);
            format!(
                "{pad}<select aria-label=\"{}\">\n{inner}{pad}</select>\n",
                html_escape(&label),
                pad = indent(depth),
            )
        }
        Role::ColorWell => {
            stats.widgets_rendered += 1;
            let label = node_text(node);
            let val = node.value().unwrap_or("#000000");
            format!(
                r#"{}<input type="color" aria-label="{}" value="{}"/>"#,
                indent(depth),
                html_escape(&label),
                html_escape(val)
            ) + "\n"
        }
        Role::Link => {
            stats.widgets_rendered += 1;
            let text = node_text(node);
            let href = node.url().unwrap_or("#");
            let desc_attrs = desc_attrs_str(node);
            format!(
                r#"{}<a href="{}"{}>{}</a>{}"#,
                indent(depth),
                html_escape(href),
                desc_attrs,
                text_content(&text, mode),
                "\n"
            )
        }
        Role::ScrollBar => {
            stats.widgets_rendered += 1;
            range_input_html(node, depth, mode)
        }

        // ── Text and semantic content ──────────────────────────────────────────
        Role::Label | Role::Legend => {
            stats.widgets_rendered += 1;
            let text = node_text(node);
            let tag = if node.role() == Role::Legend {
                "legend"
            } else {
                "label"
            };
            format!(
                "{}<{}>{}</{}>\n",
                indent(depth),
                tag,
                text_content(&text, mode),
                tag
            )
        }
        Role::Paragraph => {
            if has_children {
                stats.containers_rendered += 1;
                wrap_element("p", None, node, nodes, children, mode, depth, stats)
            } else {
                stats.widgets_rendered += 1;
                let text = node_text(node);
                format!("{}<p>{}</p>\n", indent(depth), text_content(&text, mode))
            }
        }
        Role::TextRun => {
            stats.widgets_rendered += 1;
            let text = node_text(node);
            format!(
                "{}<span>{}</span>\n",
                indent(depth),
                text_content(&text, mode)
            )
        }
        Role::Heading => {
            stats.widgets_rendered += 1;
            let text = node_text(node);
            let level = node.level().unwrap_or(2).clamp(1, 6);
            format!(
                "{}<h{}>{}</h{}>\n",
                indent(depth),
                level,
                text_content(&text, mode),
                level
            )
        }
        Role::Caption => {
            stats.widgets_rendered += 1;
            let text = node_text(node);
            format!(
                "{}<caption>{}</caption>\n",
                indent(depth),
                text_content(&text, mode)
            )
        }
        Role::Blockquote => {
            if has_children {
                stats.containers_rendered += 1;
                wrap_element(
                    "blockquote",
                    None,
                    node,
                    nodes,
                    children,
                    mode,
                    depth,
                    stats,
                )
            } else {
                stats.widgets_rendered += 1;
                let text = node_text(node);
                format!(
                    "{}<blockquote>{}</blockquote>\n",
                    indent(depth),
                    text_content(&text, mode)
                )
            }
        }
        Role::Code => {
            stats.widgets_rendered += 1;
            let text = node_text(node);
            format!(
                "{}<code>{}</code>\n",
                indent(depth),
                text_content(&text, mode)
            )
        }
        Role::Strong => {
            stats.widgets_rendered += 1;
            let text = node_text(node);
            format!(
                "{}<strong>{}</strong>\n",
                indent(depth),
                text_content(&text, mode)
            )
        }
        Role::Emphasis => {
            stats.widgets_rendered += 1;
            let text = node_text(node);
            format!("{}<em>{}</em>\n", indent(depth), text_content(&text, mode))
        }
        Role::Mark => {
            stats.widgets_rendered += 1;
            let text = node_text(node);
            format!(
                "{}<mark>{}</mark>\n",
                indent(depth),
                text_content(&text, mode)
            )
        }
        Role::Abbr => {
            stats.widgets_rendered += 1;
            let text = node_text(node);
            format!(
                "{}<abbr>{}</abbr>\n",
                indent(depth),
                text_content(&text, mode)
            )
        }
        Role::Note => {
            if has_children {
                stats.containers_rendered += 1;
                wrap_with_role("aside", "note", node, nodes, children, mode, depth, stats)
            } else {
                stats.widgets_rendered += 1;
                let text = node_text(node);
                format!(
                    r#"{}<aside role="note">{}</aside>{}"#,
                    indent(depth),
                    text_content(&text, mode),
                    "\n"
                )
            }
        }
        Role::Status => {
            stats.containers_rendered += 1;
            // Render as a keybinding status bar if children are Group chips,
            // otherwise fall back to a plain <output> for backwards compat.
            if has_children {
                let theme = node
                    .class_name()
                    .and_then(|cn| cn.parse::<ColorTheme>().ok())
                    .unwrap_or_default();
                let css_class = theme.css_class();
                let mut inner = String::new();
                for cid in children {
                    let Some(chip) = nodes.get(&cid) else {
                        continue;
                    };
                    let key = html_escape(chip.label().unwrap_or(""));
                    let action = html_escape(chip.description().unwrap_or(""));
                    inner += &format!(
                        "{}<span class=\"keybind\"><kbd>{key}</kbd><span class=\"action\">{action}</span></span>\n",
                        indent(depth + 1)
                    );
                }
                format!(
                    "{}<footer role=\"status\" class=\"status-bar {css_class}\">\n{}{}</footer>\n",
                    indent(depth),
                    inner,
                    indent(depth)
                )
            } else {
                stats.widgets_rendered += 1;
                let text = node_text(node);
                format!(
                    "{}<output>{}</output>\n",
                    indent(depth),
                    text_content(&text, mode)
                )
            }
        }
        Role::Alert => {
            if has_children {
                stats.containers_rendered += 1;
                wrap_with_role("div", "alert", node, nodes, children, mode, depth, stats)
            } else {
                stats.widgets_rendered += 1;
                let text = node_text(node);
                format!(
                    r#"{}<div role="alert">{}</div>{}"#,
                    indent(depth),
                    text_content(&text, mode),
                    "\n"
                )
            }
        }
        Role::Log => {
            stats.containers_rendered += 1;
            wrap_with_role("div", "log", node, nodes, children, mode, depth, stats)
        }
        Role::Time | Role::Timer => {
            stats.widgets_rendered += 1;
            let text = node_text(node);
            format!(
                "{}<time>{}</time>\n",
                indent(depth),
                text_content(&text, mode)
            )
        }
        Role::Image => {
            stats.widgets_rendered += 1;
            let alt = node_text(node);
            let src = node.url().unwrap_or("");
            format!(
                r#"{}<img src="{}" alt="{}"/>{}"#,
                indent(depth),
                html_escape(src),
                html_escape(&alt),
                "\n"
            )
        }
        // ── Figure / ERD diagram ───────────────────────────────────────────────
        Role::Figure => {
            let desc = node.description().unwrap_or("");
            if desc.contains("w=") && desc.contains("h=") && has_children {
                // Spatial ERD figure: render as inline SVG.
                stats.containers_rendered += 1;
                let coords = parse_kv_coords(desc);
                let cw = coords.get("w").copied().unwrap_or(800.0);
                let ch = coords.get("h").copied().unwrap_or(600.0);
                let label = html_escape(node.label().unwrap_or("ERD diagram"));
                let pad = indent(depth);

                let mut svg_body = String::new();
                // Arrow marker definition.
                svg_body.push_str(
                    "<defs><marker id=\"erd-arrow\" markerWidth=\"8\" markerHeight=\"6\" \
                     refX=\"8\" refY=\"3\" orient=\"auto\">\
                     <polygon points=\"0 0, 8 3, 0 6\" fill=\"#6c7086\"/></marker></defs>\n",
                );

                // Edges first (rendered behind boxes).
                for &child_id in children.iter() {
                    let Some(child) = nodes.get(&child_id) else {
                        continue;
                    };
                    let child_desc = child.description().unwrap_or("");
                    if !child_desc.contains("x1=") {
                        continue;
                    }
                    let c = parse_kv_coords(child_desc);
                    let x1 = c.get("x1").copied().unwrap_or(0.0);
                    let y1 = c.get("y1").copied().unwrap_or(0.0);
                    let x2 = c.get("x2").copied().unwrap_or(0.0);
                    let y2 = c.get("y2").copied().unwrap_or(0.0);
                    let edge_label = html_escape(child.label().unwrap_or(""));
                    svg_body.push_str(&format!(
                        "<line x1=\"{x1:.1}\" y1=\"{y1:.1}\" x2=\"{x2:.1}\" y2=\"{y2:.1}\" \
                         class=\"erd-edge\"><title>{edge_label}</title></line>\n"
                    ));
                }

                // Table boxes on top.
                for &child_id in children.iter() {
                    let Some(child) = nodes.get(&child_id) else {
                        continue;
                    };
                    let child_desc = child.description().unwrap_or("");
                    if !child_desc.contains("x=") {
                        continue;
                    }
                    let c = parse_kv_coords(child_desc);
                    let bx = c.get("x").copied().unwrap_or(0.0);
                    let by = c.get("y").copied().unwrap_or(0.0);
                    let bw = c.get("w").copied().unwrap_or(200.0);
                    let bh = c.get("h").copied().unwrap_or(80.0);
                    let table_name = html_escape(child.label().unwrap_or(""));

                    // Outer box.
                    svg_body.push_str(&format!(
                        "<rect x=\"{bx:.1}\" y=\"{by:.1}\" width=\"{bw:.1}\" height=\"{bh:.1}\" \
                         class=\"erd-box\" rx=\"3\"/>\n"
                    ));
                    // Header band.
                    svg_body.push_str(&format!(
                        "<rect x=\"{bx:.1}\" y=\"{by:.1}\" width=\"{bw:.1}\" height=\"24\" \
                         class=\"erd-header\" rx=\"3\"/>\n"
                    ));
                    // Table name label.
                    svg_body.push_str(&format!(
                        "<text x=\"{:.1}\" y=\"{:.1}\" class=\"erd-title\">{table_name}</text>\n",
                        bx + bw / 2.0,
                        by + 14.0,
                    ));

                    // Column rows.
                    for (i, &col_id) in child.children().iter().enumerate() {
                        let Some(col_node) = nodes.get(&col_id) else {
                            continue;
                        };
                        let col_text = html_escape(col_node.label().unwrap_or(""));
                        let ty = by + 24.0 + (i as f64 + 0.5) * 20.0 + 4.0;
                        svg_body.push_str(&format!(
                            "<text x=\"{:.1}\" y=\"{ty:.1}\" class=\"erd-col\">{col_text}</text>\n",
                            bx + 6.0,
                        ));
                    }
                }

                format!(
                    "{pad}<svg viewBox=\"0 0 {cw:.1} {ch:.1}\" \
                     class=\"erd-diagram\" role=\"img\" \
                     aria-label=\"{label}\">\n{svg_body}{pad}</svg>\n"
                )
            } else {
                // Generic figure fallback.
                stats.containers_rendered += 1;
                let inner = render_children(nodes, children, mode, depth + 1, stats);
                let pad = indent(depth);
                format!("{pad}<figure>\n{inner}{pad}</figure>\n")
            }
        }

        // ── Structural primitives ──────────────────────────────────────────────
        Role::Splitter => {
            stats.widgets_rendered += 1;
            format!("{}<hr/>\n", indent(depth))
        }
        Role::LineBreak => {
            stats.widgets_rendered += 1;
            format!("{}<br/>\n", indent(depth))
        }

        // ── Tree items (rendered as collapsible details/summary) ──────────────
        Role::Tree => {
            stats.containers_rendered += 1;
            let label = html_escape(node.label().unwrap_or("tree"));
            let desc_attrs = desc_attrs_str(node);
            let inner = render_children(nodes, children, mode, depth + 1, stats);
            format!(
                "{}<ul role=\"tree\" class=\"nav-tree\"{desc_attrs} aria-label=\"{label}\">\n{}{}</ul>\n",
                indent(depth),
                inner,
                indent(depth),
            )
        }
        Role::TreeItem => {
            if has_children {
                stats.containers_rendered += 1;
                let label = html_escape(&node_text(node));
                let selected = if node.is_selected() == Some(true) {
                    r#" aria-selected="true" class="selected""#
                } else {
                    ""
                };
                let open = if matches!(node.toggled(), Some(Toggled::True)) {
                    " open"
                } else {
                    ""
                };
                let inner = render_children(nodes, children, mode, depth + 3, stats);
                format!(
                    "{}<li role=\"none\">\n\
                     {}<details class=\"schema-group\"{open}>\n\
                     {}<summary role=\"treeitem\" tabindex=\"0\"{selected}>{label}</summary>\n\
                     {}<ul role=\"group\">\n\
                     {}\
                     {}</ul>\n\
                     {}</details>\n\
                     {}</li>\n",
                    indent(depth),
                    indent(depth + 1),
                    indent(depth + 2),
                    indent(depth + 2),
                    inner,
                    indent(depth + 2),
                    indent(depth + 1),
                    indent(depth),
                )
            } else {
                stats.widgets_rendered += 1;
                let text = html_escape(&node_text(node));
                let selected = if node.is_selected() == Some(true) {
                    r#" aria-selected="true" class="selected""#
                } else {
                    ""
                };
                // Emit HTMX interaction attrs when description carries schema/table metadata.
                let htmx_attrs = if let Some(desc) = node.description() {
                    if desc.contains(",table:") {
                        // Table item: clicking previews the table in the content panel.
                        // description format: "schema:S,table:T"
                        let parts: std::collections::HashMap<_, _> = desc
                            .split(',')
                            .filter_map(|kv| {
                                let mut it = kv.splitn(2, ':');
                                Some((it.next()?, it.next()?))
                            })
                            .collect();
                        if let (Some(s), Some(t)) = (parts.get("schema"), parts.get("table")) {
                            let s = html_escape(s);
                            let t = html_escape(t);
                            format!(
                                r##" data-meta="{}" hx-get="/api/preview?schema={}&amp;table={}" hx-target="#content" hx-swap="outerHTML""##,
                                html_escape(desc),
                                s,
                                t,
                            )
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };
                format!(
                    "{}<li role=\"treeitem\" tabindex=\"-1\"{selected}{htmx_attrs}>{text}</li>\n",
                    indent(depth),
                )
            }
        }

        // ── Fallback ───────────────────────────────────────────────────────────
        _ => {
            if has_children {
                stats.containers_rendered += 1;
                let desc_attrs = desc_attrs_str(node);
                if !desc_attrs.is_empty() {
                    let aria = aria_label_attr(node);
                    let inner = render_children(nodes, children, mode, depth + 1, stats);
                    let pad = indent(depth);
                    if inner.is_empty() {
                        format!("{pad}<div{desc_attrs}{aria}></div>\n")
                    } else {
                        format!("{pad}<div{desc_attrs}{aria}>\n{inner}{pad}</div>\n")
                    }
                } else {
                    wrap_element("div", None, node, nodes, children, mode, depth, stats)
                }
            } else {
                let text = node_text(node);
                if text.is_empty() {
                    stats.nodes_skipped += 1;
                    String::new()
                } else {
                    stats.widgets_rendered += 1;
                    let desc_attrs = desc_attrs_str(node);
                    format!(
                        "{}<span{}>{}</span>\n",
                        indent(depth),
                        desc_attrs,
                        text_content(&text, mode)
                    )
                }
            }
        }
    };

    rendered
}

// ── Element builders ──────────────────────────────────────────────────────────

fn wrap_element(
    tag: &str,
    extra_attr: Option<&str>,
    node: &Node,
    nodes: &HashMap<NodeId, Node>,
    children: &[NodeId],
    mode: LeptosRenderMode,
    depth: usize,
    stats: &mut RenderStats,
) -> String {
    let aria = aria_label_attr(node);
    let extra = extra_attr.map(|a| format!(" {a}")).unwrap_or_default();
    let inner = render_children(nodes, children, mode, depth + 1, stats);
    if inner.is_empty() {
        format!("{pad}<{tag}{extra}{aria}></{tag}>\n", pad = indent(depth))
    } else {
        format!(
            "{pad}<{tag}{extra}{aria}>\n{inner}{pad}</{tag}>\n",
            pad = indent(depth)
        )
    }
}

fn wrap_element_class(
    tag: &str,
    class: Option<&str>,
    node: &Node,
    nodes: &HashMap<NodeId, Node>,
    children: &[NodeId],
    mode: LeptosRenderMode,
    depth: usize,
    stats: &mut RenderStats,
) -> String {
    let aria = aria_label_attr(node);
    let cls = class.map(|c| format!(" class=\"{c}\"")).unwrap_or_default();
    let inner = render_children(nodes, children, mode, depth + 1, stats);
    if inner.is_empty() {
        format!("{pad}<{tag}{cls}{aria}></{tag}>\n", pad = indent(depth))
    } else {
        format!(
            "{pad}<{tag}{cls}{aria}>\n{inner}{pad}</{tag}>\n",
            pad = indent(depth)
        )
    }
}

fn wrap_with_role(
    tag: &str,
    role: &str,
    node: &Node,
    nodes: &HashMap<NodeId, Node>,
    children: &[NodeId],
    mode: LeptosRenderMode,
    depth: usize,
    stats: &mut RenderStats,
) -> String {
    let aria = aria_label_attr(node);
    let inner = render_children(nodes, children, mode, depth + 1, stats);
    if inner.is_empty() {
        format!(
            r#"{pad}<{tag} role="{role}"{aria}></{tag}>{nl}"#,
            pad = indent(depth),
            nl = "\n"
        )
    } else {
        format!(
            "{pad}<{tag} role=\"{role}\"{aria}>\n{inner}{pad}</{tag}>\n",
            pad = indent(depth)
        )
    }
}

fn render_children(
    nodes: &HashMap<NodeId, Node>,
    children: &[NodeId],
    mode: LeptosRenderMode,
    depth: usize,
    stats: &mut RenderStats,
) -> String {
    children
        .iter()
        .map(|id| render_node(nodes, *id, mode, depth, stats))
        .collect()
}

// ── Table helpers ─────────────────────────────────────────────────────────────

/// Returns `true` when every child of `row_id` is a [`Role::ColumnHeader`].
fn is_header_row(nodes: &HashMap<NodeId, Node>, row_id: NodeId) -> bool {
    let Some(row) = nodes.get(&row_id) else {
        return false;
    };
    if row.role() != Role::Row {
        return false;
    }
    let children = row.children();
    !children.is_empty()
        && children.iter().all(|id| {
            nodes
                .get(id)
                .map(|n| n.role() == Role::ColumnHeader)
                .unwrap_or(false)
        })
}

/// Render a `Table` or `Grid` node with automatic `<thead>`/`<tbody>` split.
///
/// If the children already contain [`Role::RowGroup`] nodes (explicit
/// `<tbody>`/`<thead>` structure), falls back to a plain wrapper and lets
/// those groups render themselves.  Otherwise, rows whose children are all
/// [`Role::ColumnHeader`] are lifted into `<thead>`; the rest go into
/// `<tbody>`.
fn render_table(
    nodes: &HashMap<NodeId, Node>,
    node: &Node,
    children: &[NodeId],
    tag: &str,
    role: Option<&str>,
    mode: LeptosRenderMode,
    depth: usize,
    stats: &mut RenderStats,
) -> String {
    let aria = aria_label_attr(node);
    let role_attr = role.map(|r| format!(" role=\"{r}\"")).unwrap_or_default();

    // If caller already wrapped rows in RowGroup, render flat and let those
    // groups produce <thead>/<tbody> themselves.
    let has_row_groups = children.iter().any(|id| {
        nodes
            .get(id)
            .map(|n| n.role() == Role::RowGroup)
            .unwrap_or(false)
    });
    if has_row_groups {
        let inner = render_children(nodes, children, mode, depth + 1, stats);
        return if inner.is_empty() {
            format!(
                "{pad}<{tag}{role_attr}{aria}></{tag}>\n",
                pad = indent(depth)
            )
        } else {
            format!(
                "{pad}<{tag}{role_attr}{aria}>\n{inner}{pad}</{tag}>\n",
                pad = indent(depth)
            )
        };
    }

    // Split rows: header rows (all ColumnHeader children) vs body rows.
    let (header_ids, body_ids): (Vec<&NodeId>, Vec<&NodeId>) =
        children.iter().partition(|id| is_header_row(nodes, **id));

    let mut out = format!("{pad}<{tag}{role_attr}{aria}>\n", pad = indent(depth));

    if !header_ids.is_empty() {
        out.push_str(&format!("{}<thead>\n", indent(depth + 1)));
        for id in &header_ids {
            out.push_str(&render_node(nodes, **id, mode, depth + 2, stats));
        }
        out.push_str(&format!("{}</thead>\n", indent(depth + 1)));
    }

    if !body_ids.is_empty() {
        out.push_str(&format!("{}<tbody>\n", indent(depth + 1)));
        for id in &body_ids {
            out.push_str(&render_node(nodes, **id, mode, depth + 2, stats));
        }
        out.push_str(&format!("{}</tbody>\n", indent(depth + 1)));
    }

    out.push_str(&format!("{pad}</{tag}>\n", pad = indent(depth)));
    out
}

// ── Widget builders ───────────────────────────────────────────────────────────

fn checkbox_html(node: &Node, ty: &str, depth: usize, mode: LeptosRenderMode) -> String {
    let label = node_text(node);
    let checked = if matches!(node.toggled(), Some(Toggled::True)) {
        " checked"
    } else {
        ""
    };
    let disabled = if node.is_disabled() { " disabled" } else { "" };
    format!(
        "{}<label><input type=\"{}\"{}{}/> {}</label>\n",
        indent(depth),
        ty,
        checked,
        disabled,
        text_content(&label, mode)
    )
}

fn text_input_html(node: &Node, ty: &str, depth: usize, mode: LeptosRenderMode) -> String {
    let label = node_label_text(node);
    let value = node.value().unwrap_or("");
    let placeholder = node.placeholder().unwrap_or("");
    let disabled = if node.is_disabled() { " disabled" } else { "" };
    let readonly = if node.is_read_only() { " readonly" } else { "" };
    let aria = if !label.is_empty() {
        format!(" aria-label=\"{}\"", html_escape(&label))
    } else {
        String::new()
    };
    let ph = if !placeholder.is_empty() {
        format!(" placeholder=\"{}\"", html_escape(placeholder))
    } else {
        String::new()
    };
    let desc_attrs = desc_attrs_str(node);
    let _ = mode; // value display is always plain HTML
    format!(
        "{}<input type=\"{}\" value=\"{}\"{}{}{}{}{}/>\n",
        indent(depth),
        ty,
        html_escape(value),
        ph,
        desc_attrs,
        aria,
        disabled,
        readonly,
    )
}

fn textarea_html(node: &Node, depth: usize, mode: LeptosRenderMode) -> String {
    let label = node_label_text(node);
    let value = node.value().unwrap_or("");
    let placeholder = node.placeholder().unwrap_or("");
    let disabled = if node.is_disabled() { " disabled" } else { "" };
    let readonly = if node.is_read_only() { " readonly" } else { "" };
    let aria = if !label.is_empty() {
        format!(" aria-label=\"{}\"", html_escape(&label))
    } else {
        String::new()
    };
    let ph = if !placeholder.is_empty() {
        format!(" placeholder=\"{}\"", html_escape(placeholder))
    } else {
        String::new()
    };
    let _ = mode;

    // SQL editor: overlay pattern — transparent textarea over highlighted <pre>.
    if label.eq_ignore_ascii_case("sql editor") {
        return format!(
            "{ind}<div class=\"code-wrap\">\n\
             {ind1}<pre class=\"code-output\" aria-hidden=\"true\"></pre>\n\
             {ind1}<textarea class=\"sql-ta\"{ph}{aria}{disabled}{readonly}>{value}</textarea>\n\
             {ind}</div>\n",
            ind = indent(depth),
            ind1 = indent(depth + 1),
            ph = ph,
            aria = aria,
            disabled = disabled,
            readonly = readonly,
            value = html_escape(value),
        );
    }

    format!(
        "{}<textarea{}{}{}{}>\n{}</textarea>\n",
        indent(depth),
        ph,
        aria,
        disabled,
        readonly,
        html_escape(value),
    )
}

fn numeric_input_html(node: &Node, depth: usize, mode: LeptosRenderMode) -> String {
    let label = node_label_text(node);
    let value = node.numeric_value().unwrap_or(0.0).to_string();
    let min = node
        .min_numeric_value()
        .map(|v| format!(" min=\"{v}\""))
        .unwrap_or_default();
    let max = node
        .max_numeric_value()
        .map(|v| format!(" max=\"{v}\""))
        .unwrap_or_default();
    let step = node
        .numeric_value_step()
        .map(|v| format!(" step=\"{v}\""))
        .unwrap_or_default();
    let disabled = if node.is_disabled() { " disabled" } else { "" };
    let aria = if !label.is_empty() {
        format!(" aria-label=\"{}\"", html_escape(&label))
    } else {
        String::new()
    };
    let _ = mode;
    format!(
        "{}<input type=\"number\" value=\"{}\"{}{}{}{}{}/>\n",
        indent(depth),
        html_escape(&value),
        min,
        max,
        step,
        aria,
        disabled,
    )
}

fn range_input_html(node: &Node, depth: usize, mode: LeptosRenderMode) -> String {
    let label = node_label_text(node);
    let value = node.numeric_value().unwrap_or(0.0).to_string();
    let min = node
        .min_numeric_value()
        .map(|v| format!(" min=\"{v}\""))
        .unwrap_or_default();
    let max = node
        .max_numeric_value()
        .map(|v| format!(" max=\"{v}\""))
        .unwrap_or_default();
    let step = node
        .numeric_value_step()
        .map(|v| format!(" step=\"{v}\""))
        .unwrap_or_default();
    let disabled = if node.is_disabled() { " disabled" } else { "" };
    let aria = if !label.is_empty() {
        format!(" aria-label=\"{}\"", html_escape(&label))
    } else {
        String::new()
    };
    let _ = mode;
    format!(
        "{}<input type=\"range\" value=\"{}\"{}{}{}{}{}/>\n",
        indent(depth),
        html_escape(&value),
        min,
        max,
        step,
        aria,
        disabled,
    )
}

fn progress_html(node: &Node, depth: usize, _mode: LeptosRenderMode) -> String {
    let val = node.numeric_value().unwrap_or(0.0);
    let max = node.max_numeric_value().unwrap_or(100.0);
    let label = node_label_text(node);
    let aria = if !label.is_empty() {
        format!(" aria-label=\"{}\"", html_escape(&label))
    } else {
        String::new()
    };
    format!(
        "{}<progress value=\"{}\" max=\"{}\"{}/>\n",
        indent(depth),
        val,
        max,
        aria,
    )
}

fn meter_html(node: &Node, depth: usize, _mode: LeptosRenderMode) -> String {
    let val = node.numeric_value().unwrap_or(0.0);
    let min = node.min_numeric_value().unwrap_or(0.0);
    let max = node.max_numeric_value().unwrap_or(100.0);
    let label = node_label_text(node);
    let aria = if !label.is_empty() {
        format!(" aria-label=\"{}\"", html_escape(&label))
    } else {
        String::new()
    };
    format!(
        "{}<meter min=\"{}\" max=\"{}\" value=\"{}\"{}/>\n",
        indent(depth),
        min,
        max,
        val,
        aria,
    )
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Parses a semicolon-separated `key=val;key=val` encoding from the node's
/// description and returns a string of HTML attributes ready to splice into a
/// tag (e.g., `" id=\"nav-tree\" hx-get=\"/api/nav\"`).
///
/// Returns an empty string when the description is absent, or when it contains
/// no `=` character (legacy `schema:S,table:T` tree-item descriptions are left
/// to the dedicated tree-item handler).
fn desc_attrs_str(node: &Node) -> String {
    const ALLOWED: &[&str] = &[
        "id",
        "class",
        "name",
        "autocomplete",
        "placeholder",
        "download",
        "hx-get",
        "hx-post",
        "hx-put",
        "hx-delete",
        "hx-patch",
        "hx-target",
        "hx-swap",
        "hx-trigger",
        "hx-push-url",
        "hx-indicator",
        "hx-confirm",
        "data-action",
        "data-panel",
    ];
    let Some(desc) = node.description() else {
        return String::new();
    };
    if !desc.contains('=') {
        return String::new();
    }
    desc.split(';')
        .filter_map(|part| {
            let (k, v) = part.split_once('=')?;
            let k = k.trim();
            if ALLOWED.contains(&k) {
                Some(format!(" {}=\"{}\"", k, html_escape(v)))
            } else {
                None
            }
        })
        .collect()
}

fn indent(depth: usize) -> String {
    "  ".repeat(depth)
}

fn node_text(node: &Node) -> String {
    node.value().or(node.label()).unwrap_or("").to_string()
}

fn node_label_text(node: &Node) -> String {
    elicit_accesskit::node_label(node).to_string()
}

/// In `ViewMacro` mode, text must be a quoted string literal.
fn text_content(text: &str, mode: LeptosRenderMode) -> String {
    match mode {
        LeptosRenderMode::Html => html_escape(text),
        LeptosRenderMode::ViewMacro => {
            if text.is_empty() {
                String::new()
            } else {
                format!("\"{}\"", text.replace('"', "\\\""))
            }
        }
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn aria_label_attr(node: &Node) -> String {
    node.label()
        .filter(|s| !s.is_empty())
        .map(|s| format!(" aria-label=\"{}\"", html_escape(s)))
        .unwrap_or_default()
}

fn is_horizontal(node: &Node) -> bool {
    matches!(node.orientation(), Some(Orientation::Horizontal))
}

fn orientation_class(node: &Node) -> Option<String> {
    match node.orientation() {
        Some(Orientation::Horizontal) => Some("ak-hbox".to_string()),
        Some(Orientation::Vertical) => Some("ak-vbox".to_string()),
        None => None,
    }
}

/// Parse a comma-separated `key=value` coordinate string (e.g. `"x=10,y=20,w=200,h=80"`)
/// into a map of string keys to f64 values.
///
/// Used to decode spatial metadata from [`accesskit::Role::Figure`] and its
/// child [`accesskit::Role::Group`] nodes in the ERD diagram IR.
fn parse_kv_coords(desc: &str) -> std::collections::HashMap<&str, f64> {
    desc.split(',')
        .filter_map(|part| {
            let (k, v) = part.split_once('=')?;
            let v = v.trim().parse::<f64>().ok()?;
            Some((k.trim(), v))
        })
        .collect()
}
