//! `BevyUiPlugin` — UI layout and widget codegen tools.
//!
//! This plugin targets Bevy 0.18's current component-based UI model:
//! `Node`, `Text`, `ImageNode`, `Button`, and CSS-like flex/grid layout fields.

use elicitation::emit_code::{CrateDep, EmitCode, ToCodeLiteral};
use elicitation::{ElicitPlugin, elicit_tool};
use proc_macro2::TokenStream;
use quote::quote;
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// A CSS-grid placement constructor variant.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BevyGridPlacementKind {
    /// `GridPlacement::auto()`
    Auto,
    /// `GridPlacement::span(span)`
    Span,
    /// `GridPlacement::start(start)`
    Start,
    /// `GridPlacement::end(end)`
    End,
    /// `GridPlacement::start_span(start, span)`
    StartSpan,
    /// `GridPlacement::start_end(start, end)`
    StartEnd,
    /// `GridPlacement::end_span(end, span)`
    EndSpan,
}

/// A partial `Node` descriptor expressed as Bevy value expressions.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct BevyUiNodeParams {
    /// Optional `Display` expression.
    #[serde(default)]
    pub display_expr: Option<String>,
    /// Optional `PositionType` expression.
    #[serde(default)]
    pub position_type_expr: Option<String>,
    /// Optional `Overflow` expression.
    #[serde(default)]
    pub overflow_expr: Option<String>,
    /// Optional `left` value.
    #[serde(default)]
    pub left_expr: Option<String>,
    /// Optional `right` value.
    #[serde(default)]
    pub right_expr: Option<String>,
    /// Optional `top` value.
    #[serde(default)]
    pub top_expr: Option<String>,
    /// Optional `bottom` value.
    #[serde(default)]
    pub bottom_expr: Option<String>,
    /// Optional width value.
    #[serde(default)]
    pub width_expr: Option<String>,
    /// Optional height value.
    #[serde(default)]
    pub height_expr: Option<String>,
    /// Optional minimum width.
    #[serde(default)]
    pub min_width_expr: Option<String>,
    /// Optional minimum height.
    #[serde(default)]
    pub min_height_expr: Option<String>,
    /// Optional maximum width.
    #[serde(default)]
    pub max_width_expr: Option<String>,
    /// Optional maximum height.
    #[serde(default)]
    pub max_height_expr: Option<String>,
    /// Optional `aspect_ratio`.
    #[serde(default)]
    pub aspect_ratio: Option<f32>,
    /// Optional `margin`.
    #[serde(default)]
    pub margin_expr: Option<String>,
    /// Optional `padding`.
    #[serde(default)]
    pub padding_expr: Option<String>,
    /// Optional `border`.
    #[serde(default)]
    pub border_expr: Option<String>,
    /// Optional `border_radius`.
    #[serde(default)]
    pub border_radius_expr: Option<String>,
    /// Optional `FlexDirection`.
    #[serde(default)]
    pub flex_direction_expr: Option<String>,
    /// Optional `FlexWrap`.
    #[serde(default)]
    pub flex_wrap_expr: Option<String>,
    /// Optional `AlignItems`.
    #[serde(default)]
    pub align_items_expr: Option<String>,
    /// Optional `JustifyItems`.
    #[serde(default)]
    pub justify_items_expr: Option<String>,
    /// Optional `AlignSelf`.
    #[serde(default)]
    pub align_self_expr: Option<String>,
    /// Optional `JustifySelf`.
    #[serde(default)]
    pub justify_self_expr: Option<String>,
    /// Optional `AlignContent`.
    #[serde(default)]
    pub align_content_expr: Option<String>,
    /// Optional `JustifyContent`.
    #[serde(default)]
    pub justify_content_expr: Option<String>,
    /// Optional `flex_grow`.
    #[serde(default)]
    pub flex_grow: Option<f32>,
    /// Optional `flex_shrink`.
    #[serde(default)]
    pub flex_shrink: Option<f32>,
    /// Optional `flex_basis`.
    #[serde(default)]
    pub flex_basis_expr: Option<String>,
    /// Optional `row_gap`.
    #[serde(default)]
    pub row_gap_expr: Option<String>,
    /// Optional `column_gap`.
    #[serde(default)]
    pub column_gap_expr: Option<String>,
    /// Optional `grid_auto_flow`.
    #[serde(default)]
    pub grid_auto_flow_expr: Option<String>,
    /// Optional `grid_template_rows` expressions.
    #[serde(default)]
    pub grid_template_rows_exprs: Vec<String>,
    /// Optional `grid_template_columns` expressions.
    #[serde(default)]
    pub grid_template_columns_exprs: Vec<String>,
    /// Optional `grid_auto_rows` expressions.
    #[serde(default)]
    pub grid_auto_rows_exprs: Vec<String>,
    /// Optional `grid_auto_columns` expressions.
    #[serde(default)]
    pub grid_auto_columns_exprs: Vec<String>,
    /// Optional `grid_row`.
    #[serde(default)]
    pub grid_row_expr: Option<String>,
    /// Optional `grid_column`.
    #[serde(default)]
    pub grid_column_expr: Option<String>,
}

/// Parameters for `bevy_ui__ui_rect`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::ui::UiRect")]
pub struct BevyUiRectParams {
    /// Left side expression.
    #[serde(default)]
    #[to_code_literal(rename = "left", to_tokens = "ui_rect_side_tokens")]
    pub left_expr: Option<String>,
    /// Right side expression.
    #[serde(default)]
    #[to_code_literal(rename = "right", to_tokens = "ui_rect_side_tokens")]
    pub right_expr: Option<String>,
    /// Top side expression.
    #[serde(default)]
    #[to_code_literal(rename = "top", to_tokens = "ui_rect_side_tokens")]
    pub top_expr: Option<String>,
    /// Bottom side expression.
    #[serde(default)]
    #[to_code_literal(rename = "bottom", to_tokens = "ui_rect_side_tokens")]
    pub bottom_expr: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct BevyGridPlacementFields {
    /// Placement constructor to use.
    pub kind: BevyGridPlacementKind,
    /// Start line for start-based constructors.
    #[serde(default)]
    pub start: Option<i16>,
    /// End line for end-based constructors.
    #[serde(default)]
    pub end: Option<i16>,
    /// Span size for span-based constructors.
    #[serde(default)]
    pub span: Option<u16>,
}

/// Parameters for `bevy_ui__grid_placement`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(transparent, path = "::bevy::ui::GridPlacement")]
pub struct BevyGridPlacementParams {
    /// Flattened placement fields preserving the MCP JSON shape.
    #[serde(flatten)]
    #[to_code_literal(to_tokens = "grid_placement_tokens")]
    fields: BevyGridPlacementFields,
}

/// Parameters for `bevy_ui__node`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BevyUiNodeLiteralParams {
    /// Partial node descriptor.
    pub node: BevyUiNodeParams,
}

/// Parameters for `bevy_ui__text`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BevyUiTextParams {
    /// The displayed string.
    pub value: String,
    /// Optional font handle expression.
    #[serde(default)]
    pub font_handle_expr: Option<String>,
    /// Optional font size in logical pixels.
    #[serde(default)]
    pub font_size: Option<f32>,
    /// Optional text color expression.
    #[serde(default)]
    pub color_expr: Option<String>,
    /// Optional `Justify` expression.
    #[serde(default)]
    pub justify_expr: Option<String>,
    /// Optional `LineBreak` expression.
    #[serde(default)]
    pub linebreak_expr: Option<String>,
}

/// Parameters for `bevy_ui__image`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BevyUiImageParams {
    /// Optional image handle expression.
    #[serde(default)]
    pub image_expr: Option<String>,
    /// Optional color tint expression.
    #[serde(default)]
    pub color_expr: Option<String>,
    /// Whether to flip the image horizontally.
    #[serde(default)]
    pub flip_x: bool,
    /// Whether to flip the image vertically.
    #[serde(default)]
    pub flip_y: bool,
}

/// Parameters for `bevy_ui__button_bundle`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BevyUiButtonBundleParams {
    /// Commands-like receiver expression.
    #[serde(default = "default_commands_var")]
    pub commands_var: String,
    /// Label string displayed in the button.
    pub label: String,
    /// Partial node descriptor for the button.
    #[serde(default)]
    pub node: BevyUiNodeParams,
    /// Optional background color expression.
    #[serde(default)]
    pub background_color_expr: Option<String>,
    /// Optional font handle expression for the label.
    #[serde(default)]
    pub font_handle_expr: Option<String>,
    /// Optional label font size.
    #[serde(default)]
    pub font_size: Option<f32>,
    /// Optional label color expression.
    #[serde(default)]
    pub text_color_expr: Option<String>,
}

/// Parameters for `bevy_ui__flex_container`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BevyUiFlexContainerParams {
    /// Commands-like receiver expression.
    #[serde(default = "default_commands_var")]
    pub commands_var: String,
    /// Partial node descriptor for the flex container.
    #[serde(default)]
    pub node: BevyUiNodeParams,
    /// Child spawn expressions.
    #[serde(default)]
    pub children: Vec<String>,
}

/// Parameters for `bevy_ui__grid_container`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BevyUiGridContainerParams {
    /// Commands-like receiver expression.
    #[serde(default = "default_commands_var")]
    pub commands_var: String,
    /// Partial node descriptor for the grid container.
    #[serde(default)]
    pub node: BevyUiNodeParams,
    /// Child spawn expressions.
    #[serde(default)]
    pub children: Vec<String>,
}

fn default_commands_var() -> String {
    "commands".to_string()
}

fn tool_err(msg: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(msg.to_string(), None)
}

fn ok_source(source: String) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(source)]))
}

fn parse_expr(src: &str, context: &str) -> Result<syn::Expr, ErrorData> {
    syn::parse_str::<syn::Expr>(src)
        .map_err(|error| tool_err(format!("invalid {context} expression `{src}`: {error}")))
}

fn expr_tokens(src: &str, context: &str) -> syn::Expr {
    parse_expr(src, context).expect("validated expressions must parse")
}

fn validate_optional_expr(src: &Option<String>, context: &str) -> Result<(), ErrorData> {
    if let Some(src) = src {
        let _ = parse_expr(src, context)?;
    }
    Ok(())
}

fn validate_expr_list(values: &[String], context: &str) -> Result<(), ErrorData> {
    for value in values {
        let _ = parse_expr(value, context)?;
    }
    Ok(())
}

fn bevy_dep() -> Vec<CrateDep> {
    vec![CrateDep::new("bevy", "0.18")]
}

macro_rules! impl_ui_literal_emit {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl EmitCode for $ty {
                fn emit_code(&self) -> TokenStream {
                    self.to_code_literal()
                }

                fn crate_deps(&self) -> Vec<CrateDep> {
                    bevy_dep()
                }
            }
        )+
    };
}

fn ui_rect_side_tokens(value: &Option<String>) -> TokenStream {
    match value.as_deref() {
        Some(expr) => {
            let expr = expr_tokens(expr, "ui rect side");
            quote! { #expr }
        }
        None => quote! { ::bevy::ui::Val::Auto },
    }
}

fn grid_placement_tokens(value: &BevyGridPlacementFields) -> TokenStream {
    match value.kind {
        BevyGridPlacementKind::Auto => quote! { ::bevy::ui::GridPlacement::auto() },
        BevyGridPlacementKind::Span => {
            let span = value.span.expect("validated span must exist");
            quote! { ::bevy::ui::GridPlacement::span(#span) }
        }
        BevyGridPlacementKind::Start => {
            let start = value.start.expect("validated start must exist");
            quote! { ::bevy::ui::GridPlacement::start(#start) }
        }
        BevyGridPlacementKind::End => {
            let end = value.end.expect("validated end must exist");
            quote! { ::bevy::ui::GridPlacement::end(#end) }
        }
        BevyGridPlacementKind::StartSpan => {
            let start = value.start.expect("validated start must exist");
            let span = value.span.expect("validated span must exist");
            quote! { ::bevy::ui::GridPlacement::start_span(#start, #span) }
        }
        BevyGridPlacementKind::StartEnd => {
            let start = value.start.expect("validated start must exist");
            let end = value.end.expect("validated end must exist");
            quote! { ::bevy::ui::GridPlacement::start_end(#start, #end) }
        }
        BevyGridPlacementKind::EndSpan => {
            let end = value.end.expect("validated end must exist");
            let span = value.span.expect("validated span must exist");
            quote! { ::bevy::ui::GridPlacement::end_span(#end, #span) }
        }
    }
}

fn render_node_fields(node: &BevyUiNodeParams, forced_display: Option<TokenStream>) -> TokenStream {
    let display = forced_display
        .or_else(|| {
            node.display_expr.as_deref().map(|expr| {
                let expr = expr_tokens(expr, "display");
                quote! { display: #expr, }
            })
        })
        .unwrap_or_default();
    let position_type = node
        .position_type_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "position type");
            quote! { position_type: #expr, }
        })
        .unwrap_or_default();
    let overflow = node
        .overflow_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "overflow");
            quote! { overflow: #expr, }
        })
        .unwrap_or_default();
    let left = node
        .left_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "left");
            quote! { left: #expr, }
        })
        .unwrap_or_default();
    let right = node
        .right_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "right");
            quote! { right: #expr, }
        })
        .unwrap_or_default();
    let top = node
        .top_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "top");
            quote! { top: #expr, }
        })
        .unwrap_or_default();
    let bottom = node
        .bottom_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "bottom");
            quote! { bottom: #expr, }
        })
        .unwrap_or_default();
    let width = node
        .width_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "width");
            quote! { width: #expr, }
        })
        .unwrap_or_default();
    let height = node
        .height_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "height");
            quote! { height: #expr, }
        })
        .unwrap_or_default();
    let min_width = node
        .min_width_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "min width");
            quote! { min_width: #expr, }
        })
        .unwrap_or_default();
    let min_height = node
        .min_height_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "min height");
            quote! { min_height: #expr, }
        })
        .unwrap_or_default();
    let max_width = node
        .max_width_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "max width");
            quote! { max_width: #expr, }
        })
        .unwrap_or_default();
    let max_height = node
        .max_height_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "max height");
            quote! { max_height: #expr, }
        })
        .unwrap_or_default();
    let aspect_ratio = node
        .aspect_ratio
        .map(|value| quote! { aspect_ratio: Some(#value), })
        .unwrap_or_default();
    let margin = node
        .margin_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "margin");
            quote! { margin: #expr, }
        })
        .unwrap_or_default();
    let padding = node
        .padding_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "padding");
            quote! { padding: #expr, }
        })
        .unwrap_or_default();
    let border = node
        .border_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "border");
            quote! { border: #expr, }
        })
        .unwrap_or_default();
    let border_radius = node
        .border_radius_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "border radius");
            quote! { border_radius: #expr, }
        })
        .unwrap_or_default();
    let flex_direction = node
        .flex_direction_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "flex direction");
            quote! { flex_direction: #expr, }
        })
        .unwrap_or_default();
    let flex_wrap = node
        .flex_wrap_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "flex wrap");
            quote! { flex_wrap: #expr, }
        })
        .unwrap_or_default();
    let align_items = node
        .align_items_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "align items");
            quote! { align_items: #expr, }
        })
        .unwrap_or_default();
    let justify_items = node
        .justify_items_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "justify items");
            quote! { justify_items: #expr, }
        })
        .unwrap_or_default();
    let align_self = node
        .align_self_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "align self");
            quote! { align_self: #expr, }
        })
        .unwrap_or_default();
    let justify_self = node
        .justify_self_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "justify self");
            quote! { justify_self: #expr, }
        })
        .unwrap_or_default();
    let align_content = node
        .align_content_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "align content");
            quote! { align_content: #expr, }
        })
        .unwrap_or_default();
    let justify_content = node
        .justify_content_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "justify content");
            quote! { justify_content: #expr, }
        })
        .unwrap_or_default();
    let flex_grow = node
        .flex_grow
        .map(|value| quote! { flex_grow: #value, })
        .unwrap_or_default();
    let flex_shrink = node
        .flex_shrink
        .map(|value| quote! { flex_shrink: #value, })
        .unwrap_or_default();
    let flex_basis = node
        .flex_basis_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "flex basis");
            quote! { flex_basis: #expr, }
        })
        .unwrap_or_default();
    let row_gap = node
        .row_gap_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "row gap");
            quote! { row_gap: #expr, }
        })
        .unwrap_or_default();
    let column_gap = node
        .column_gap_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "column gap");
            quote! { column_gap: #expr, }
        })
        .unwrap_or_default();
    let grid_auto_flow = node
        .grid_auto_flow_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "grid auto flow");
            quote! { grid_auto_flow: #expr, }
        })
        .unwrap_or_default();
    let grid_template_rows = if node.grid_template_rows_exprs.is_empty() {
        TokenStream::new()
    } else {
        let values: Vec<syn::Expr> = node
            .grid_template_rows_exprs
            .iter()
            .map(|expr| expr_tokens(expr, "grid template row"))
            .collect();
        quote! { grid_template_rows: vec![#(#values),*], }
    };
    let grid_template_columns = if node.grid_template_columns_exprs.is_empty() {
        TokenStream::new()
    } else {
        let values: Vec<syn::Expr> = node
            .grid_template_columns_exprs
            .iter()
            .map(|expr| expr_tokens(expr, "grid template column"))
            .collect();
        quote! { grid_template_columns: vec![#(#values),*], }
    };
    let grid_auto_rows = if node.grid_auto_rows_exprs.is_empty() {
        TokenStream::new()
    } else {
        let values: Vec<syn::Expr> = node
            .grid_auto_rows_exprs
            .iter()
            .map(|expr| expr_tokens(expr, "grid auto row"))
            .collect();
        quote! { grid_auto_rows: vec![#(#values),*], }
    };
    let grid_auto_columns = if node.grid_auto_columns_exprs.is_empty() {
        TokenStream::new()
    } else {
        let values: Vec<syn::Expr> = node
            .grid_auto_columns_exprs
            .iter()
            .map(|expr| expr_tokens(expr, "grid auto column"))
            .collect();
        quote! { grid_auto_columns: vec![#(#values),*], }
    };
    let grid_row = node
        .grid_row_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "grid row");
            quote! { grid_row: #expr, }
        })
        .unwrap_or_default();
    let grid_column = node
        .grid_column_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "grid column");
            quote! { grid_column: #expr, }
        })
        .unwrap_or_default();

    quote! {
        #display
        #position_type
        #overflow
        #left
        #right
        #top
        #bottom
        #width
        #height
        #min_width
        #min_height
        #max_width
        #max_height
        #aspect_ratio
        #margin
        #padding
        #border
        #border_radius
        #flex_direction
        #flex_wrap
        #align_items
        #justify_items
        #align_self
        #justify_self
        #align_content
        #justify_content
        #flex_grow
        #flex_shrink
        #flex_basis
        #row_gap
        #column_gap
        #grid_auto_flow
        #grid_template_rows
        #grid_template_columns
        #grid_auto_rows
        #grid_auto_columns
        #grid_row
        #grid_column
    }
}

fn node_literal_tokens(
    node: &BevyUiNodeParams,
    forced_display: Option<TokenStream>,
) -> TokenStream {
    let fields = render_node_fields(node, forced_display);
    quote! {
        ::bevy::ui::Node {
            #fields
            ..::std::default::Default::default()
        }
    }
}

fn text_component_tokens(params: &BevyUiTextParams) -> TokenStream {
    let value = &params.value;
    let mut font = quote! { ::bevy::text::TextFont::default() };
    if let Some(handle) = &params.font_handle_expr {
        let handle = expr_tokens(handle, "font handle");
        font = quote! { (#font).with_font(#handle) };
    }
    if let Some(size) = params.font_size {
        font = quote! { (#font).with_font_size(#size) };
    }
    let color = params
        .color_expr
        .as_deref()
        .map(|expr| {
            let expr = expr_tokens(expr, "text color");
            quote! { ::bevy::text::TextColor((#expr).into()) }
        })
        .unwrap_or_else(|| quote! { ::bevy::text::TextColor::default() });
    let mut layout = quote! { ::bevy::text::TextLayout::default() };
    if let Some(justify) = &params.justify_expr {
        let justify = expr_tokens(justify, "justify");
        layout = quote! { (#layout).with_justify(#justify) };
    }
    if let Some(linebreak) = &params.linebreak_expr {
        let linebreak = expr_tokens(linebreak, "linebreak");
        layout = quote! { (#layout).with_linebreak(#linebreak) };
    }
    quote! {
        (
            ::bevy::ui::widget::Text::new(#value),
            #font,
            #color,
            #layout,
        )
    }
}

impl_ui_literal_emit!(BevyUiRectParams, BevyGridPlacementParams);

impl EmitCode for BevyUiNodeLiteralParams {
    fn emit_code(&self) -> TokenStream {
        node_literal_tokens(&self.node, None)
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for BevyUiTextParams {
    fn emit_code(&self) -> TokenStream {
        text_component_tokens(self)
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for BevyUiImageParams {
    fn emit_code(&self) -> TokenStream {
        let base = if let Some(image) = &self.image_expr {
            let image = expr_tokens(image, "image");
            quote! { ::bevy::ui::widget::ImageNode::new(#image) }
        } else if let Some(color) = &self.color_expr {
            let color = expr_tokens(color, "image color");
            quote! { ::bevy::ui::widget::ImageNode::solid_color((#color).into()) }
        } else {
            quote! { ::bevy::ui::widget::ImageNode::default() }
        };
        let tinted = if self.image_expr.is_some() {
            self.color_expr
                .as_deref()
                .map(|expr| {
                    let expr = expr_tokens(expr, "image color");
                    quote! { (#base).with_color((#expr).into()) }
                })
                .unwrap_or(base)
        } else {
            base
        };
        let flip_x = if self.flip_x {
            quote! { (#tinted).with_flip_x() }
        } else {
            tinted
        };
        if self.flip_y {
            quote! { (#flip_x).with_flip_y() }
        } else {
            flip_x
        }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for BevyUiButtonBundleParams {
    fn emit_code(&self) -> TokenStream {
        let commands = expr_tokens(&self.commands_var, "commands receiver");
        let node = node_literal_tokens(&self.node, None);
        let background = self
            .background_color_expr
            .as_deref()
            .map(|expr| {
                let expr = expr_tokens(expr, "background color");
                quote! { ::bevy::ui::BackgroundColor((#expr).into()) }
            })
            .unwrap_or_else(|| {
                quote! {
                    ::bevy::ui::BackgroundColor(::bevy::color::Color::srgb(0.15, 0.15, 0.15))
                }
            });
        let label = BevyUiTextParams {
            value: self.label.clone(),
            font_handle_expr: self.font_handle_expr.clone(),
            font_size: self.font_size,
            color_expr: self.text_color_expr.clone(),
            justify_expr: None,
            linebreak_expr: None,
        };
        let text = label.emit_code();
        quote! {
            #commands
                .spawn((
                    ::bevy::ui::widget::Button,
                    #node,
                    #background,
                ))
                .with_children(|parent| {
                    parent.spawn(#text);
                })
        }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for BevyUiFlexContainerParams {
    fn emit_code(&self) -> TokenStream {
        let commands = expr_tokens(&self.commands_var, "commands receiver");
        let children: Vec<syn::Expr> = self
            .children
            .iter()
            .map(|child| expr_tokens(child, "child"))
            .collect();
        let node = node_literal_tokens(
            &self.node,
            Some(quote! { display: ::bevy::ui::Display::Flex, }),
        );
        quote! {
            #commands.spawn((#node,)).with_children(|parent| {
                #(parent.spawn(#children);)*
            })
        }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for BevyUiGridContainerParams {
    fn emit_code(&self) -> TokenStream {
        let commands = expr_tokens(&self.commands_var, "commands receiver");
        let children: Vec<syn::Expr> = self
            .children
            .iter()
            .map(|child| expr_tokens(child, "child"))
            .collect();
        let node = node_literal_tokens(
            &self.node,
            Some(quote! { display: ::bevy::ui::Display::Grid, }),
        );
        quote! {
            #commands.spawn((#node,)).with_children(|parent| {
                #(parent.spawn(#children);)*
            })
        }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

elicitation::register_emit!("ui_rect", BevyUiRectParams);
elicitation::register_emit!("grid_placement", BevyGridPlacementParams);
elicitation::register_emit!("node", BevyUiNodeLiteralParams);
elicitation::register_emit!("text", BevyUiTextParams);
elicitation::register_emit!("image", BevyUiImageParams);
elicitation::register_emit!("button_bundle", BevyUiButtonBundleParams);
elicitation::register_emit!("flex_container", BevyUiFlexContainerParams);
elicitation::register_emit!("grid_container", BevyUiGridContainerParams);

/// MCP plugin exposing Bevy UI layout and widget fragment tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "bevy_ui")]
pub struct BevyUiPlugin;

impl BevyUiPlugin {
    /// Creates a new Bevy UI fragment plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for BevyUiPlugin {
    fn default() -> Self {
        Self::new()
    }
}

fn validate_node_params(node: &BevyUiNodeParams) -> Result<(), ErrorData> {
    validate_optional_expr(&node.display_expr, "display")?;
    validate_optional_expr(&node.position_type_expr, "position type")?;
    validate_optional_expr(&node.overflow_expr, "overflow")?;
    validate_optional_expr(&node.left_expr, "left")?;
    validate_optional_expr(&node.right_expr, "right")?;
    validate_optional_expr(&node.top_expr, "top")?;
    validate_optional_expr(&node.bottom_expr, "bottom")?;
    validate_optional_expr(&node.width_expr, "width")?;
    validate_optional_expr(&node.height_expr, "height")?;
    validate_optional_expr(&node.min_width_expr, "min width")?;
    validate_optional_expr(&node.min_height_expr, "min height")?;
    validate_optional_expr(&node.max_width_expr, "max width")?;
    validate_optional_expr(&node.max_height_expr, "max height")?;
    validate_optional_expr(&node.margin_expr, "margin")?;
    validate_optional_expr(&node.padding_expr, "padding")?;
    validate_optional_expr(&node.border_expr, "border")?;
    validate_optional_expr(&node.border_radius_expr, "border radius")?;
    validate_optional_expr(&node.flex_direction_expr, "flex direction")?;
    validate_optional_expr(&node.flex_wrap_expr, "flex wrap")?;
    validate_optional_expr(&node.align_items_expr, "align items")?;
    validate_optional_expr(&node.justify_items_expr, "justify items")?;
    validate_optional_expr(&node.align_self_expr, "align self")?;
    validate_optional_expr(&node.justify_self_expr, "justify self")?;
    validate_optional_expr(&node.align_content_expr, "align content")?;
    validate_optional_expr(&node.justify_content_expr, "justify content")?;
    validate_optional_expr(&node.flex_basis_expr, "flex basis")?;
    validate_optional_expr(&node.row_gap_expr, "row gap")?;
    validate_optional_expr(&node.column_gap_expr, "column gap")?;
    validate_optional_expr(&node.grid_auto_flow_expr, "grid auto flow")?;
    validate_expr_list(&node.grid_template_rows_exprs, "grid template row")?;
    validate_expr_list(&node.grid_template_columns_exprs, "grid template column")?;
    validate_expr_list(&node.grid_auto_rows_exprs, "grid auto row")?;
    validate_expr_list(&node.grid_auto_columns_exprs, "grid auto column")?;
    validate_optional_expr(&node.grid_row_expr, "grid row")?;
    validate_optional_expr(&node.grid_column_expr, "grid column")?;
    Ok(())
}

fn validate_ui_rect(params: &BevyUiRectParams) -> Result<(), ErrorData> {
    validate_optional_expr(&params.left_expr, "left")?;
    validate_optional_expr(&params.right_expr, "right")?;
    validate_optional_expr(&params.top_expr, "top")?;
    validate_optional_expr(&params.bottom_expr, "bottom")?;
    Ok(())
}

fn validate_grid_placement(params: &BevyGridPlacementParams) -> Result<(), ErrorData> {
    let start = params.fields.start.unwrap_or_default();
    let end = params.fields.end.unwrap_or_default();
    let span = params.fields.span.unwrap_or_default();
    match params.fields.kind {
        BevyGridPlacementKind::Auto => Ok(()),
        BevyGridPlacementKind::Span => {
            if span == 0 {
                Err(tool_err("span must be greater than zero"))
            } else {
                Ok(())
            }
        }
        BevyGridPlacementKind::Start => {
            if start == 0 {
                Err(tool_err("start must be non-zero"))
            } else {
                Ok(())
            }
        }
        BevyGridPlacementKind::End => {
            if end == 0 {
                Err(tool_err("end must be non-zero"))
            } else {
                Ok(())
            }
        }
        BevyGridPlacementKind::StartSpan => {
            if start == 0 || span == 0 {
                Err(tool_err("start and span must be non-zero"))
            } else {
                Ok(())
            }
        }
        BevyGridPlacementKind::StartEnd => {
            if start == 0 || end == 0 {
                Err(tool_err("start and end must be non-zero"))
            } else {
                Ok(())
            }
        }
        BevyGridPlacementKind::EndSpan => {
            if end == 0 || span == 0 {
                Err(tool_err("end and span must be non-zero"))
            } else {
                Ok(())
            }
        }
    }
}

fn validate_text(params: &BevyUiTextParams) -> Result<(), ErrorData> {
    validate_optional_expr(&params.font_handle_expr, "font handle")?;
    validate_optional_expr(&params.color_expr, "text color")?;
    validate_optional_expr(&params.justify_expr, "justify")?;
    validate_optional_expr(&params.linebreak_expr, "linebreak")?;
    Ok(())
}

fn validate_image(params: &BevyUiImageParams) -> Result<(), ErrorData> {
    validate_optional_expr(&params.image_expr, "image")?;
    validate_optional_expr(&params.color_expr, "image color")?;
    Ok(())
}

fn validate_button_bundle(params: &BevyUiButtonBundleParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.commands_var, "commands receiver")?;
    validate_node_params(&params.node)?;
    validate_optional_expr(&params.background_color_expr, "background color")?;
    validate_optional_expr(&params.font_handle_expr, "font handle")?;
    validate_optional_expr(&params.text_color_expr, "text color")?;
    Ok(())
}

fn validate_flex_container(params: &BevyUiFlexContainerParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.commands_var, "commands receiver")?;
    validate_node_params(&params.node)?;
    validate_expr_list(&params.children, "child")?;
    Ok(())
}

fn validate_grid_container(params: &BevyUiGridContainerParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.commands_var, "commands receiver")?;
    validate_node_params(&params.node)?;
    validate_expr_list(&params.children, "child")?;
    Ok(())
}

#[elicit_tool(
    plugin = "bevy_ui",
    name = "node",
    description = "Emit a `Node` struct literal with CSS-like layout fields.",
    emit = None
)]
#[instrument(skip_all)]
async fn node(p: BevyUiNodeLiteralParams) -> Result<CallToolResult, ErrorData> {
    validate_node_params(&p.node)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ui",
    name = "ui_rect",
    description = "Emit a `UiRect` literal from side expressions.",
    emit = None
)]
#[instrument(skip_all)]
async fn ui_rect(p: BevyUiRectParams) -> Result<CallToolResult, ErrorData> {
    validate_ui_rect(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ui",
    name = "grid_placement",
    description = "Emit a `GridPlacement` constructor such as `start_span` or `start_end`.",
    emit = None
)]
#[instrument(skip_all)]
async fn grid_placement(p: BevyGridPlacementParams) -> Result<CallToolResult, ErrorData> {
    validate_grid_placement(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ui",
    name = "text",
    description = "Emit a tuple of UI text components: `Text`, `TextFont`, `TextColor`, and `TextLayout`.",
    emit = None
)]
#[instrument(skip_all)]
async fn text(p: BevyUiTextParams) -> Result<CallToolResult, ErrorData> {
    validate_text(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ui",
    name = "image",
    description = "Emit an `ImageNode` component expression.",
    emit = None
)]
#[instrument(skip_all)]
async fn image(p: BevyUiImageParams) -> Result<CallToolResult, ErrorData> {
    validate_image(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ui",
    name = "button_bundle",
    description = "Emit a button spawn scaffold using `Button`, `Node`, `BackgroundColor`, and child UI text.",
    emit = None
)]
#[instrument(skip_all)]
async fn button_bundle(p: BevyUiButtonBundleParams) -> Result<CallToolResult, ErrorData> {
    validate_button_bundle(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ui",
    name = "flex_container",
    description = "Emit a flexbox container spawn scaffold with child spawns.",
    emit = None
)]
#[instrument(skip_all)]
async fn flex_container(p: BevyUiFlexContainerParams) -> Result<CallToolResult, ErrorData> {
    validate_flex_container(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ui",
    name = "grid_container",
    description = "Emit a grid container spawn scaffold with child spawns.",
    emit = None
)]
#[instrument(skip_all)]
async fn grid_container(p: BevyUiGridContainerParams) -> Result<CallToolResult, ErrorData> {
    validate_grid_container(&p)?;
    ok_source(p.emit_code().to_string())
}
