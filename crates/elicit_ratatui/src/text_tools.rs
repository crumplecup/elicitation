//! Dual-mode text composition tools for ratatui.
//!
//! Each tool returns a [`TextJson`], [`SpanJson`], or [`LineJson`] description
//! that can be rendered by a ratatui backend or emitted as Rust source code.

use crate::serde_types::{AlignmentJson, LineJson, SpanJson, StyleJson, TextJson};
use rmcp::model::{CallToolResult, Content};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use elicitation::elicit_tool;

/// Serialise a text to a JSON `CallToolResult`.
fn text_result(text: &TextJson) -> CallToolResult {
    match serde_json::to_string(text) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

/// Serialise a span to a JSON `CallToolResult`.
fn span_result(span: &SpanJson) -> CallToolResult {
    match serde_json::to_string(span) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

/// Serialise a line to a JSON `CallToolResult`.
fn line_result(line: &LineJson) -> CallToolResult {
    match serde_json::to_string(line) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

// ---------------------------------------------------------------------------
// Text — raw
// ---------------------------------------------------------------------------

/// Parameters for [`text_raw`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TextRawParams {
    /// Plain text content.
    pub content: String,
}

/// Create plain unstyled text.
#[elicit_tool(
    plugin = "ratatui_text",
    name = "text_raw",
    description = "Create plain unstyled text. Returns TextJson with a single unstyled span."
)]
#[instrument(skip_all)]
async fn text_raw(p: TextRawParams) -> Result<CallToolResult, ErrorData> {
    let t = TextJson {
        lines: vec![LineJson {
            spans: vec![SpanJson {
                content: p.content,
                style: None,
            }],
            style: None,
            alignment: None,
        }],
        style: None,
        alignment: None,
    };
    Ok(text_result(&t))
}

// ---------------------------------------------------------------------------
// Text — styled
// ---------------------------------------------------------------------------

/// Parameters for [`text_styled`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TextStyledParams {
    /// Text content.
    pub content: String,
    /// Text style.
    pub style: StyleJson,
}

/// Create styled text with a single span.
#[elicit_tool(
    plugin = "ratatui_text",
    name = "text_styled",
    description = "Create styled text with a single span. Returns TextJson with one styled span."
)]
#[instrument(skip_all)]
async fn text_styled(p: TextStyledParams) -> Result<CallToolResult, ErrorData> {
    let t = TextJson {
        lines: vec![LineJson {
            spans: vec![SpanJson {
                content: p.content,
                style: Some(p.style),
            }],
            style: None,
            alignment: None,
        }],
        style: None,
        alignment: None,
    };
    Ok(text_result(&t))
}

// ---------------------------------------------------------------------------
// Span — raw
// ---------------------------------------------------------------------------

/// Parameters for [`span_raw`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SpanRawParams {
    /// Span text content.
    pub content: String,
}

/// Create a plain unstyled span.
#[elicit_tool(
    plugin = "ratatui_text",
    name = "span_raw",
    description = "Create a plain unstyled span. Returns SpanJson with no style."
)]
#[instrument(skip_all)]
async fn span_raw(p: SpanRawParams) -> Result<CallToolResult, ErrorData> {
    let s = SpanJson {
        content: p.content,
        style: None,
    };
    Ok(span_result(&s))
}

// ---------------------------------------------------------------------------
// Span — styled
// ---------------------------------------------------------------------------

/// Parameters for [`span_styled`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SpanStyledParams {
    /// Span text content.
    pub content: String,
    /// Span style.
    pub style: StyleJson,
}

/// Create a styled span.
#[elicit_tool(
    plugin = "ratatui_text",
    name = "span_styled",
    description = "Create a styled span with foreground/background/modifiers. Returns SpanJson."
)]
#[instrument(skip_all)]
async fn span_styled(p: SpanStyledParams) -> Result<CallToolResult, ErrorData> {
    let s = SpanJson {
        content: p.content,
        style: Some(p.style),
    };
    Ok(span_result(&s))
}

// ---------------------------------------------------------------------------
// Line from spans
// ---------------------------------------------------------------------------

/// Parameters for [`line_from_spans`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LineFromSpansParams {
    /// Spans composing the line.
    pub spans: Vec<SpanJson>,
    /// Line style (applied to entire line).
    #[serde(default)]
    pub style: Option<StyleJson>,
    /// Line alignment.
    #[serde(default)]
    pub alignment: Option<AlignmentJson>,
}

/// Create a line from a list of spans.
#[elicit_tool(
    plugin = "ratatui_text",
    name = "line_from_spans",
    description = "Create a line from a list of spans with optional style and alignment. Returns LineJson."
)]
#[instrument(skip_all)]
async fn line_from_spans(p: LineFromSpansParams) -> Result<CallToolResult, ErrorData> {
    let l = LineJson {
        spans: p.spans,
        style: p.style,
        alignment: p.alignment,
    };
    Ok(line_result(&l))
}

// ---------------------------------------------------------------------------
// Text from lines
// ---------------------------------------------------------------------------

/// Parameters for [`text_from_lines`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TextFromLinesParams {
    /// Lines composing the text.
    pub lines: Vec<LineJson>,
    /// Text style.
    #[serde(default)]
    pub style: Option<StyleJson>,
    /// Text alignment.
    #[serde(default)]
    pub alignment: Option<AlignmentJson>,
}

/// Create multi-line text from a list of lines.
#[elicit_tool(
    plugin = "ratatui_text",
    name = "text_from_lines",
    description = "Create multi-line text from a list of lines with optional style and alignment. Returns TextJson."
)]
#[instrument(skip_all)]
async fn text_from_lines(p: TextFromLinesParams) -> Result<CallToolResult, ErrorData> {
    let t = TextJson {
        lines: p.lines,
        style: p.style,
        alignment: p.alignment,
    };
    Ok(text_result(&t))
}
