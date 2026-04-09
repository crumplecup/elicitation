//! `AxumExtractPathPlugin` — MCP tools for axum Path extractor.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Types ─────────────────────────────────────────────────────────────────────

/// A single extracted path parameter.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PathParam {
    /// The parameter name as it appears in the route template.
    pub name: String,
    /// The extracted value from the actual URI segment.
    pub value: String,
    /// The Rust type used for this parameter.
    pub param_type: String,
}

/// Describes the result of a path extraction operation.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PathDescriptor {
    /// The extracted parameters.
    pub params: Vec<PathParam>,
    /// Human-readable description of the extraction.
    pub description: String,
}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for extract_path_single.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PathSingleParams {
    /// The route template, e.g. `/users/{id}`.
    pub path_template: String,
    /// The extracted string value.
    pub value: String,
}

/// Parameters for extract_path_tuple.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PathTupleParams {
    /// The route template, e.g. `/orgs/{org}/repos/{repo}`.
    pub path_template: String,
    /// The extracted values in order of appearance.
    pub values: Vec<String>,
}

/// Parameters for extract_path_struct.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PathStructParams {
    /// The route template.
    pub path_template: String,
    /// The Rust struct type name used for deserialization.
    pub struct_type: String,
    /// The JSON-encoded value of the deserialized struct.
    pub json_value: String,
}

/// Parameters for raw_path_params_collect.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RawPathParams {
    /// The route template.
    pub path_template: String,
    /// The actual request URI.
    pub uri: String,
}

/// Parameters for path_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PathDescribeParams {
    /// The route template to describe.
    pub path_template: String,
}

/// Parameters for path_validate_segment.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PathValidateParams {
    /// The template segment to validate, e.g. `{id}`.
    pub segment: String,
}

/// Parameters for path_param_names.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PathTemplateParam {
    /// The route template to extract parameter names from.
    pub path_template: String,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn extract_param_names(template: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut rest = template;
    while let Some(start) = rest.find('{') {
        rest = &rest[start + 1..];
        if let Some(end) = rest.find('}') {
            names.push(rest[..end].to_string());
            rest = &rest[end + 1..];
        } else {
            break;
        }
    }
    names
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_extract_path",
    name = "extract_path_single",
    description = "Describe a Path<String> extraction for a single-segment route template."
)]
#[instrument]
async fn extract_path_single(p: PathSingleParams) -> Result<CallToolResult, ErrorData> {
    let names = extract_param_names(&p.path_template);
    let name = names.into_iter().next().unwrap_or_default();
    let descriptor = PathDescriptor {
        params: vec![PathParam {
            name: name.clone(),
            value: p.value.clone(),
            param_type: "String".to_string(),
        }],
        description: format!(
            "Path<String> extracting '{}' from template '{}'",
            p.value, p.path_template
        ),
    };
    let val = serde_json::to_string(&descriptor).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_extract_path",
    name = "extract_path_tuple",
    description = "Describe a Path<(...)> extraction for a multi-segment route template."
)]
#[instrument]
async fn extract_path_tuple(p: PathTupleParams) -> Result<CallToolResult, ErrorData> {
    let names = extract_param_names(&p.path_template);
    let count = names.len();
    let params: Vec<PathParam> = names
        .into_iter()
        .enumerate()
        .map(|(i, name)| PathParam {
            name,
            value: p.values.get(i).cloned().unwrap_or_default(),
            param_type: "String".to_string(),
        })
        .collect();
    let descriptor = PathDescriptor {
        params,
        description: format!(
            "Path<(...)> extracting {} params from '{}'",
            count, p.path_template
        ),
    };
    let val = serde_json::to_string(&descriptor).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_extract_path",
    name = "extract_path_struct",
    description = "Describe a Path<T> extraction where T is a deserializable struct."
)]
#[instrument]
async fn extract_path_struct(p: PathStructParams) -> Result<CallToolResult, ErrorData> {
    let names = extract_param_names(&p.path_template);
    let params: Vec<PathParam> = names
        .into_iter()
        .map(|name| PathParam {
            name,
            value: String::new(),
            param_type: p.struct_type.clone(),
        })
        .collect();
    let descriptor = PathDescriptor {
        params,
        description: format!(
            "Path<{}> deserializing from template '{}' with value: {}",
            p.struct_type, p.path_template, p.json_value
        ),
    };
    let val = serde_json::to_string(&descriptor).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_extract_path",
    name = "raw_path_params_collect",
    description = "Describe a RawPathParams extraction, matching URI segments to template params."
)]
#[instrument]
async fn raw_path_params_collect(p: RawPathParams) -> Result<CallToolResult, ErrorData> {
    let names = extract_param_names(&p.path_template);
    let template_segments: Vec<&str> = p
        .path_template
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();
    let uri_segments: Vec<&str> = p.uri.split('/').filter(|s| !s.is_empty()).collect();
    let params: Vec<PathParam> = names
        .into_iter()
        .map(|name| {
            let value = template_segments
                .iter()
                .position(|seg| *seg == format!("{{{}}}", name))
                .and_then(|i| uri_segments.get(i).copied())
                .unwrap_or("")
                .to_string();
            PathParam {
                name,
                value,
                param_type: "String".to_string(),
            }
        })
        .collect();
    let descriptor = PathDescriptor {
        params,
        description: format!(
            "RawPathParams from URI '{}' using template '{}'",
            p.uri, p.path_template
        ),
    };
    let val = serde_json::to_string(&descriptor).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_extract_path",
    name = "path_describe",
    description = "Describe the Path extractor for a given route template, listing all param names."
)]
#[instrument]
async fn path_describe(p: PathDescribeParams) -> Result<CallToolResult, ErrorData> {
    let names = extract_param_names(&p.path_template);
    let params: Vec<PathParam> = names
        .into_iter()
        .map(|name| PathParam {
            name,
            value: String::new(),
            param_type: "String".to_string(),
        })
        .collect();
    let descriptor = PathDescriptor {
        params,
        description: format!("Path extractor for template '{}'", p.path_template),
    };
    let val = serde_json::to_string(&descriptor).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_extract_path",
    name = "path_validate_segment",
    description = "Validate a route template segment such as `{id}`. Returns JSON with valid/name/reason."
)]
#[instrument]
async fn path_validate_segment(p: PathValidateParams) -> Result<CallToolResult, ErrorData> {
    let seg = p.segment.trim();
    let (valid, name, reason) = if seg.starts_with('{') && seg.ends_with('}') {
        let inner = &seg[1..seg.len() - 1];
        if inner.is_empty() {
            (false, String::new(), "empty parameter name".to_string())
        } else {
            let first = inner.chars().next().unwrap();
            if first.is_ascii_digit() {
                (
                    false,
                    inner.to_string(),
                    "parameter name must not start with a digit".to_string(),
                )
            } else if inner.chars().all(|c| c.is_alphanumeric() || c == '_') {
                (true, inner.to_string(), "ok".to_string())
            } else {
                (
                    false,
                    inner.to_string(),
                    "parameter name contains invalid characters".to_string(),
                )
            }
        }
    } else {
        (
            false,
            String::new(),
            "segment must start with '{' and end with '}'".to_string(),
        )
    };
    let val = serde_json::json!({
        "valid": valid,
        "name": name,
        "reason": reason,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&val).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_extract_path",
    name = "path_param_names",
    description = "Extract all parameter names from a route template as a JSON array."
)]
#[instrument]
async fn path_param_names(p: PathTemplateParam) -> Result<CallToolResult, ErrorData> {
    let names = extract_param_names(&p.path_template);
    let val = serde_json::to_string(&names).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// Plugin exposing axum Path extractor tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_extract_path")]
pub struct AxumExtractPathPlugin;
