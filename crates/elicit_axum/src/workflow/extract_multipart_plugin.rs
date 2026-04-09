//! `AxumExtractMultipartPlugin` — MCP tools for axum Multipart extractor.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Types ─────────────────────────────────────────────────────────────────────

/// Describes a single field in a multipart stream.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultipartFieldDescriptor {
    /// The `name` parameter from the Content-Disposition header.
    pub name: Option<String>,
    /// The Content-Type of this field, if present.
    pub content_type: Option<String>,
    /// The `filename` parameter from the Content-Disposition header, if present.
    pub filename: Option<String>,
    /// All raw header strings for this field.
    pub headers: Vec<String>,
    /// Optional size hint in bytes.
    pub size_hint: Option<u64>,
}

impl elicitation::emit_code::ToCodeLiteral for MultipartFieldDescriptor {
    fn to_code_literal(&self) -> elicitation::proc_macro2::TokenStream {
        let name = elicitation::emit_code::ToCodeLiteral::to_code_literal(&self.name);
        let content_type =
            elicitation::emit_code::ToCodeLiteral::to_code_literal(&self.content_type);
        let filename = elicitation::emit_code::ToCodeLiteral::to_code_literal(&self.filename);
        let headers = elicitation::emit_code::ToCodeLiteral::to_code_literal(&self.headers);
        let size_hint = elicitation::emit_code::ToCodeLiteral::to_code_literal(&self.size_hint);
        elicitation::quote::quote! {
            MultipartFieldDescriptor {
                name: #name,
                content_type: #content_type,
                filename: #filename,
                headers: #headers,
                size_hint: #size_hint,
            }
        }
    }
}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for multipart_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultipartDescribeParams {
    /// The multipart boundary string.
    pub boundary: String,
}

/// Parameters for multipart_field_name.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultipartFieldDescriptorInput {
    /// The multipart field to inspect.
    pub field: MultipartFieldDescriptor,
}

/// Parameters for multipart_field_content_type.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultipartFieldContentTypeInput {
    /// The multipart field to inspect.
    pub field: MultipartFieldDescriptor,
}

/// Parameters for multipart_field_headers.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultipartFieldHeadersInput {
    /// The multipart field whose headers to list.
    pub field: MultipartFieldDescriptor,
}

/// Parameters for multipart_field_bytes.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultipartFieldBytesParams {
    /// The multipart field being read.
    pub field: MultipartFieldDescriptor,
    /// The number of bytes collected from the field.
    pub size: u64,
}

/// Parameters for multipart_field_text.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultipartFieldTextParams {
    /// The multipart field being read.
    pub field: MultipartFieldDescriptor,
    /// The UTF-8 text content of the field.
    pub text: String,
}

/// Parameters for multipart_next_field_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultipartNextParams {
    /// The number of fields still remaining in the multipart stream.
    pub fields_remaining: u32,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_extract_multipart",
    name = "multipart_describe",
    description = "Describe a Multipart stream with a given MIME boundary."
)]
#[instrument]
async fn multipart_describe(p: MultipartDescribeParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "Multipart stream with boundary '{}'. \
        Use next_field() to iterate over each part. \
        Each field exposes name(), content_type(), filename(), headers(), and chunk() / text() / bytes().",
        p.boundary
    ))]))
}

#[elicit_tool(
    plugin = "axum_extract_multipart",
    name = "multipart_field_name",
    description = "Return the name of a multipart field from its Content-Disposition header."
)]
#[instrument]
async fn multipart_field_name(
    p: MultipartFieldDescriptorInput,
) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        p.field.name.as_deref().unwrap_or("unnamed").to_string(),
    )]))
}

#[elicit_tool(
    plugin = "axum_extract_multipart",
    name = "multipart_field_content_type",
    description = "Return the Content-Type of a multipart field, defaulting to application/octet-stream."
)]
#[instrument]
async fn multipart_field_content_type(
    p: MultipartFieldContentTypeInput,
) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        p.field
            .content_type
            .as_deref()
            .unwrap_or("application/octet-stream")
            .to_string(),
    )]))
}

#[elicit_tool(
    plugin = "axum_extract_multipart",
    name = "multipart_field_headers",
    description = "Return a JSON array of all header strings for a multipart field."
)]
#[instrument]
async fn multipart_field_headers(
    p: MultipartFieldHeadersInput,
) -> Result<CallToolResult, ErrorData> {
    let val = serde_json::to_string(&p.field.headers).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_extract_multipart",
    name = "multipart_field_bytes",
    description = "Describe the byte collection result for a multipart field."
)]
#[instrument]
async fn multipart_field_bytes(p: MultipartFieldBytesParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "Field '{}' bytes: {} bytes collected",
        p.field.name.as_deref().unwrap_or("unnamed"),
        p.size
    ))]))
}

#[elicit_tool(
    plugin = "axum_extract_multipart",
    name = "multipart_field_text",
    description = "Describe the text content of a multipart field."
)]
#[instrument]
async fn multipart_field_text(p: MultipartFieldTextParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "Field '{}' text: {}",
        p.field.name.as_deref().unwrap_or("unnamed"),
        p.text
    ))]))
}

#[elicit_tool(
    plugin = "axum_extract_multipart",
    name = "multipart_next_field_describe",
    description = "Describe the result of calling next_field() on a Multipart stream."
)]
#[instrument]
async fn multipart_next_field_describe(
    p: MultipartNextParams,
) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "next_field() called. {} field(s) remain in the multipart stream.",
        p.fields_remaining
    ))]))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// Plugin exposing axum Multipart extractor tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_extract_multipart")]
pub struct AxumExtractMultipartPlugin;
