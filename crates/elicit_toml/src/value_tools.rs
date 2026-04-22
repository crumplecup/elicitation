//! Tools for constructing and inspecting `toml_edit::Value` and `toml::Value` instances.

use elicitation::elicit_tool;
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::plugin::{err_text, ok_json, ok_text};

// ── toml_edit::Value construction helpers ─────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValueFromTomlParams {
    /// TOML value fragment to parse and return as JSON.
    pub value_toml: String,
}

/// Parse a TOML value fragment and return it as JSON.
#[elicit_tool(
    plugin = "toml",
    name = "toml__value__from_toml",
    description = "Parse a TOML value fragment (e.g. `42`, `\"hello\"`, `[1,2,3]`) and return it as JSON."
)]
async fn value_from_toml(p: ValueFromTomlParams) -> Result<CallToolResult, ErrorData> {
    let wrapper = format!("__v = {}", p.value_toml);
    let doc: toml_edit::DocumentMut = wrapper
        .parse()
        .map_err(|e| ErrorData::invalid_params(format!("parse error: {}", e), None))?;
    match doc.get("__v") {
        Some(item) => ok_text(item.to_string()),
        None => err_text("could not parse value fragment"),
    }
}

// ── toml::Value JSON→TOML serialization ──────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TomlValueFromJsonParams {
    /// JSON value to interpret as a `toml::Value`.
    /// Must be a JSON object, array, string, number, or boolean (no nulls).
    pub json_value: String,
}

/// Convert a JSON value to a `toml::Value` and return its TOML text representation.
#[elicit_tool(
    plugin = "toml",
    name = "toml__value__toml_from_json",
    description = "Convert a JSON value to a toml::Value and return the TOML text. Useful for validating JSON↔TOML round-trips."
)]
async fn toml_value_from_json(p: TomlValueFromJsonParams) -> Result<CallToolResult, ErrorData> {
    let json: serde_json::Value = serde_json::from_str(&p.json_value)
        .map_err(|e| ErrorData::invalid_params(format!("JSON parse error: {}", e), None))?;
    let tv: toml::Value = serde_json::from_value(json).map_err(|e| {
        ErrorData::invalid_params(format!("toml::Value conversion error: {}", e), None)
    })?;
    match toml::to_string(&tv) {
        Ok(s) => ok_text(s),
        Err(e) => err_text(format!("TOML serialize error: {}", e)),
    }
}

// ── Type name ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValueTypeNameParams {
    /// TOML value fragment to inspect.
    pub value_toml: String,
}

/// Return the type name of a TOML value fragment.
#[elicit_tool(
    plugin = "toml",
    name = "toml__value__type_name",
    description = "Return the type name of a TOML value fragment: string, integer, float, boolean, datetime, array, or inline_table."
)]
async fn value_type_name(p: ValueTypeNameParams) -> Result<CallToolResult, ErrorData> {
    let wrapper = format!("__v = {}", p.value_toml);
    let doc: toml_edit::DocumentMut = wrapper
        .parse()
        .map_err(|e| ErrorData::invalid_params(format!("parse error: {}", e), None))?;
    let name = match doc.get("__v").and_then(|i| i.as_value()) {
        Some(toml_edit::Value::String(_)) => "string",
        Some(toml_edit::Value::Integer(_)) => "integer",
        Some(toml_edit::Value::Float(_)) => "float",
        Some(toml_edit::Value::Boolean(_)) => "boolean",
        Some(toml_edit::Value::Datetime(_)) => "datetime",
        Some(toml_edit::Value::Array(_)) => "array",
        Some(toml_edit::Value::InlineTable(_)) => "inline_table",
        None => "table_or_unknown",
    };
    ok_text(name)
}

// ── toml::Value variant reference ────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValueVariantsParams {}

/// Return a reference table of all `toml::Value` and `toml_edit::Value` variants.
#[elicit_tool(
    plugin = "toml",
    name = "toml__value__variants_reference",
    description = "Return a reference table of all toml::Value and toml_edit::Value variants with their Rust types."
)]
async fn value_variants_reference(_p: ValueVariantsParams) -> Result<CallToolResult, ErrorData> {
    ok_text(
        "toml::Value variants:\n\
         - String(String)\n\
         - Integer(i64)\n\
         - Float(f64)\n\
         - Boolean(bool)\n\
         - Datetime(toml_datetime::Datetime)\n\
         - Array(Vec<Value>)\n\
         - Table(Map<String, Value>)  -- HashMap alias\n\
         \n\
         toml_edit::Value variants:\n\
         - String(Formatted<String>)\n\
         - Integer(Formatted<i64>)\n\
         - Float(Formatted<f64>)\n\
         - Boolean(Formatted<bool>)\n\
         - Datetime(Formatted<Datetime>)\n\
         - Array(Array)             -- preserves whitespace\n\
         - InlineTable(InlineTable) -- note: NOT Table\n\
         \n\
         Formatted<T> wraps the raw value with optional Repr (source text) and Decor (whitespace/comments).",
    )
}

// ── Formatted<T> explanation ─────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FormattedExplainParams {}

/// Explain the `Formatted<T>` / `Repr` / `Decor` / `RawString` structure in `toml_edit`.
#[elicit_tool(
    plugin = "toml",
    name = "toml__value__formatted_explain",
    description = "Return an explanation of Formatted<T>, Repr, Decor, and RawString in toml_edit and how they preserve source formatting."
)]
async fn formatted_explain(_p: FormattedExplainParams) -> Result<CallToolResult, ErrorData> {
    ok_text(
        "toml_edit preserves source formatting through three layers:\n\
         \n\
         Formatted<T> { value: T, repr: Option<Repr>, decor: Decor }\n\
         - value: the actual Rust value (i64, f64, String, bool, Datetime)\n\
         - repr: optional Repr holding the original source text (e.g. `0xFF` for an integer)\n\
         - decor: Decor holding prefix/suffix whitespace and comments\n\
         \n\
         Repr { raw_value: RawString }\n\
         - holds the verbatim source text for the value token\n\
         \n\
         Decor { prefix: RawString, suffix: RawString }\n\
         - prefix: whitespace/comments before the value\n\
         - suffix: whitespace/comments after the value (before the next token)\n\
         \n\
         RawString\n\
         - may be owned (as_str() returns &str) or a span reference into the source\n\
         - when a document is modified, spans are dropped and owned strings are used\n\
         \n\
         To set a custom repr (e.g. hex integer):\n\
         ```rust\n\
         use toml_edit::{Formatted, Repr, Value};\n\
         let v = Value::Integer(Formatted::new(255)\n\
             .with_repr_unchecked(Repr::new_unchecked(\"0xFF\")));\n\
         ```",
    )
}

// ── Construct integer value snippet ──────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ConstructIntegerParams {
    /// The integer value.
    pub value: i64,
    /// Optional custom source representation, e.g. `"0xFF"`.
    #[serde(default)]
    pub repr: Option<String>,
}

/// Return a Rust snippet for constructing a `toml_edit::Value::Integer`.
#[elicit_tool(
    plugin = "toml",
    name = "toml__value__construct_integer",
    description = "Return a Rust snippet for constructing a toml_edit::Value::Integer, optionally with a custom source repr."
)]
async fn construct_integer(p: ConstructIntegerParams) -> Result<CallToolResult, ErrorData> {
    let snippet = match p.repr {
        None => format!(
            "toml_edit::Value::Integer(toml_edit::Formatted::new({}))",
            p.value
        ),
        Some(r) => format!(
            "toml_edit::Value::Integer(toml_edit::Formatted::new({})\n    \
             .with_repr_unchecked(toml_edit::Repr::new_unchecked(\"{}\")))",
            p.value, r
        ),
    };
    ok_text(snippet)
}

// ── Construct string value snippet ────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ConstructStringParams {
    /// The string value.
    pub value: String,
    /// If true, emit a literal string `'''..'''` repr instead of basic `"..."`.
    #[serde(default)]
    pub literal: bool,
}

/// Return a Rust snippet for constructing a `toml_edit::Value::String`.
#[elicit_tool(
    plugin = "toml",
    name = "toml__value__construct_string",
    description = "Return a Rust snippet for constructing a toml_edit::Value::String."
)]
async fn construct_string(p: ConstructStringParams) -> Result<CallToolResult, ErrorData> {
    let snippet = if p.literal {
        format!(
            "toml_edit::Value::String(toml_edit::Formatted::new(\"{}\".to_string())\n    \
             .with_repr_unchecked(toml_edit::Repr::new_unchecked(\"'{}'\")))",
            p.value, p.value
        )
    } else {
        format!(
            "toml_edit::Value::String(toml_edit::Formatted::new(\"{}\".to_string()))",
            p.value
        )
    };
    ok_text(snippet)
}

// ── Parse & round-trip diagnostic ────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RoundTripDiagnosticParams {
    /// TOML text to test for round-trip stability.
    pub toml_text: String,
}

/// Parse TOML and re-serialize it; return whether the round-trip is lossless.
#[elicit_tool(
    plugin = "toml",
    name = "toml__value__round_trip_diagnostic",
    description = "Parse TOML and re-serialize via toml_edit; report whether the round-trip is byte-for-byte identical."
)]
async fn round_trip_diagnostic(p: RoundTripDiagnosticParams) -> Result<CallToolResult, ErrorData> {
    let doc: toml_edit::DocumentMut = p
        .toml_text
        .parse()
        .map_err(|e| ErrorData::invalid_params(format!("parse error: {}", e), None))?;
    let round_tripped = doc.to_string();
    if p.toml_text == round_tripped {
        ok_text("LOSSLESS: round-trip is byte-for-byte identical")
    } else {
        ok_json(&serde_json::json!({
            "lossless": false,
            "original": p.toml_text,
            "round_tripped": round_tripped,
        }))
    }
}
