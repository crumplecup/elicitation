//! Trenchcoat wrapper for [`surrealdb_types::Kind`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A SurrealDB geometry type variant for use in `DEFINE FIELD TYPE geometry(…)`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum GeometryKind {
    /// Point geometry.
    Point,
    /// Line geometry.
    Line,
    /// Polygon geometry.
    Polygon,
    /// MultiPoint geometry.
    MultiPoint,
    /// MultiLine geometry.
    MultiLine,
    /// MultiPolygon geometry.
    MultiPolygon,
    /// Collection of geometries.
    Collection,
    /// Any geometry type.
    Feature,
}

/// A SurrealDB type kind for schema field type declarations.
///
/// Used in `DEFINE FIELD … TYPE kind` authoring. Mirrors
/// `surrealdb_types::Kind` without the recursive heap allocations that make
/// the upstream type difficult to use as a JSON parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", content = "params")]
pub enum Kind {
    /// Any value — the most permissive type.
    Any,
    /// No value (option-like absence).
    None,
    /// Null value.
    Null,
    /// Boolean.
    Bool,
    /// Raw bytes.
    Bytes,
    /// Datetime.
    Datetime,
    /// Arbitrary-precision decimal.
    Decimal,
    /// Duration.
    Duration,
    /// 64-bit float.
    Float,
    /// 64-bit signed integer.
    Int,
    /// Any numeric type (int, float, or decimal).
    Number,
    /// JSON object.
    Object,
    /// String.
    String,
    /// UUID.
    Uuid,
    /// Regular expression.
    Regex,
    /// Range.
    Range,
    /// A specific table type (optionally restricted to named tables).
    Table(Vec<String>),
    /// A record reference (optionally restricted to named tables).
    Record(Vec<String>),
    /// A geometry type (optionally restricted to named geometry kinds).
    Geometry(Vec<GeometryKind>),
    /// One of several types.
    Either(Vec<Kind>),
    /// A typed set with an optional maximum size.
    Set(Box<Kind>, Option<u64>),
    /// A typed array with an optional maximum length.
    Array(Box<Kind>, Option<u64>),
    /// File reference (optionally restricted to named buckets).
    File(Vec<String>),
    /// A literal string constant.
    LiteralString(String),
    /// A literal integer constant.
    LiteralInt(i64),
    /// A literal float constant.
    LiteralFloat(f64),
    /// A literal boolean constant.
    LiteralBool(bool),
}

#[cfg(feature = "surreal-types")]
impl From<surrealdb_types::GeometryKind> for GeometryKind {
    fn from(gk: surrealdb_types::GeometryKind) -> Self {
        match gk {
            surrealdb_types::GeometryKind::Point => GeometryKind::Point,
            surrealdb_types::GeometryKind::Line => GeometryKind::Line,
            surrealdb_types::GeometryKind::Polygon => GeometryKind::Polygon,
            surrealdb_types::GeometryKind::MultiPoint => GeometryKind::MultiPoint,
            surrealdb_types::GeometryKind::MultiLine => GeometryKind::MultiLine,
            surrealdb_types::GeometryKind::MultiPolygon => GeometryKind::MultiPolygon,
            surrealdb_types::GeometryKind::Collection => GeometryKind::Collection,
        }
    }
}

#[cfg(feature = "surreal-types")]
impl From<GeometryKind> for surrealdb_types::GeometryKind {
    fn from(gk: GeometryKind) -> Self {
        match gk {
            GeometryKind::Point => surrealdb_types::GeometryKind::Point,
            GeometryKind::Line => surrealdb_types::GeometryKind::Line,
            GeometryKind::Polygon => surrealdb_types::GeometryKind::Polygon,
            GeometryKind::MultiPoint => surrealdb_types::GeometryKind::MultiPoint,
            GeometryKind::MultiLine => surrealdb_types::GeometryKind::MultiLine,
            GeometryKind::MultiPolygon => surrealdb_types::GeometryKind::MultiPolygon,
            GeometryKind::Collection => surrealdb_types::GeometryKind::Collection,
            // Feature variant has no direct upstream equivalent; use Collection as a fallback.
            GeometryKind::Feature => surrealdb_types::GeometryKind::Collection,
        }
    }
}

#[cfg(feature = "surreal-types")]
impl From<surrealdb_types::Kind> for Kind {
    fn from(k: surrealdb_types::Kind) -> Self {
        match k {
            surrealdb_types::Kind::Any => Kind::Any,
            surrealdb_types::Kind::None => Kind::None,
            surrealdb_types::Kind::Null => Kind::Null,
            surrealdb_types::Kind::Bool => Kind::Bool,
            surrealdb_types::Kind::Bytes => Kind::Bytes,
            surrealdb_types::Kind::Datetime => Kind::Datetime,
            surrealdb_types::Kind::Decimal => Kind::Decimal,
            surrealdb_types::Kind::Duration => Kind::Duration,
            surrealdb_types::Kind::Float => Kind::Float,
            surrealdb_types::Kind::Int => Kind::Int,
            surrealdb_types::Kind::Number => Kind::Number,
            surrealdb_types::Kind::Object => Kind::Object,
            surrealdb_types::Kind::String => Kind::String,
            surrealdb_types::Kind::Uuid => Kind::Uuid,
            surrealdb_types::Kind::Regex => Kind::Regex,
            surrealdb_types::Kind::Range => Kind::Range,
            surrealdb_types::Kind::Table(tables) => {
                Kind::Table(tables.into_iter().map(|t| t.into_inner()).collect())
            }
            surrealdb_types::Kind::Record(tables) => {
                Kind::Record(tables.into_iter().map(|t| t.into_inner()).collect())
            }
            surrealdb_types::Kind::Geometry(gks) => {
                Kind::Geometry(gks.into_iter().map(GeometryKind::from).collect())
            }
            surrealdb_types::Kind::Either(kinds) => {
                Kind::Either(kinds.into_iter().map(Kind::from).collect())
            }
            surrealdb_types::Kind::Set(inner, max) => Kind::Set(Box::new(Kind::from(*inner)), max),
            surrealdb_types::Kind::Array(inner, max) => {
                Kind::Array(Box::new(Kind::from(*inner)), max)
            }
            // Function types have no direct equivalent; treat as Any.
            surrealdb_types::Kind::Function(_, _) => Kind::Any,
            surrealdb_types::Kind::File(buckets) => Kind::File(buckets),
            surrealdb_types::Kind::Literal(lit) => match lit {
                surrealdb_types::KindLiteral::String(s) => Kind::LiteralString(s),
                surrealdb_types::KindLiteral::Integer(i) => Kind::LiteralInt(i),
                surrealdb_types::KindLiteral::Float(f) => Kind::LiteralFloat(f),
                surrealdb_types::KindLiteral::Decimal(d) => Kind::LiteralString(d.to_string()),
                surrealdb_types::KindLiteral::Duration(d) => Kind::LiteralString(d.to_string()),
                surrealdb_types::KindLiteral::Bool(b) => Kind::LiteralBool(b),
                // Array/Object literals collapse to their base kinds.
                surrealdb_types::KindLiteral::Array(_) => Kind::Array(Box::new(Kind::Any), None),
                surrealdb_types::KindLiteral::Object(_) => Kind::Object,
            },
        }
    }
}

#[cfg(feature = "surreal-types")]
impl From<Kind> for surrealdb_types::Kind {
    fn from(k: Kind) -> Self {
        match k {
            Kind::Any => surrealdb_types::Kind::Any,
            Kind::None => surrealdb_types::Kind::None,
            Kind::Null => surrealdb_types::Kind::Null,
            Kind::Bool => surrealdb_types::Kind::Bool,
            Kind::Bytes => surrealdb_types::Kind::Bytes,
            Kind::Datetime => surrealdb_types::Kind::Datetime,
            Kind::Decimal => surrealdb_types::Kind::Decimal,
            Kind::Duration => surrealdb_types::Kind::Duration,
            Kind::Float => surrealdb_types::Kind::Float,
            Kind::Int => surrealdb_types::Kind::Int,
            Kind::Number => surrealdb_types::Kind::Number,
            Kind::Object => surrealdb_types::Kind::Object,
            Kind::String => surrealdb_types::Kind::String,
            Kind::Uuid => surrealdb_types::Kind::Uuid,
            Kind::Regex => surrealdb_types::Kind::Regex,
            Kind::Range => surrealdb_types::Kind::Range,
            Kind::Table(names) => surrealdb_types::Kind::Table(
                names.into_iter().map(surrealdb_types::Table::new).collect(),
            ),
            Kind::Record(names) => surrealdb_types::Kind::Record(
                names.into_iter().map(surrealdb_types::Table::new).collect(),
            ),
            Kind::Geometry(gks) => surrealdb_types::Kind::Geometry(
                gks.into_iter()
                    .map(surrealdb_types::GeometryKind::from)
                    .collect(),
            ),
            Kind::Either(kinds) => surrealdb_types::Kind::Either(
                kinds.into_iter().map(surrealdb_types::Kind::from).collect(),
            ),
            Kind::Set(inner, max) => {
                surrealdb_types::Kind::Set(Box::new(surrealdb_types::Kind::from(*inner)), max)
            }
            Kind::Array(inner, max) => {
                surrealdb_types::Kind::Array(Box::new(surrealdb_types::Kind::from(*inner)), max)
            }
            Kind::File(buckets) => surrealdb_types::Kind::File(buckets),
            Kind::LiteralString(s) => {
                surrealdb_types::Kind::Literal(surrealdb_types::KindLiteral::String(s))
            }
            Kind::LiteralInt(i) => {
                surrealdb_types::Kind::Literal(surrealdb_types::KindLiteral::Integer(i))
            }
            Kind::LiteralFloat(f) => {
                surrealdb_types::Kind::Literal(surrealdb_types::KindLiteral::Float(f))
            }
            Kind::LiteralBool(b) => {
                surrealdb_types::Kind::Literal(surrealdb_types::KindLiteral::Bool(b))
            }
        }
    }
}

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};

// ── GeometryKind ────────────────────────────────────────────────────────────

impl Prompt for GeometryKind {
    fn prompt() -> Option<&'static str> {
        Some("Choose the SurrealDB geometry type:")
    }
}

impl Select for GeometryKind {
    fn options() -> Vec<Self> {
        vec![
            GeometryKind::Point,
            GeometryKind::Line,
            GeometryKind::Polygon,
            GeometryKind::MultiPoint,
            GeometryKind::MultiLine,
            GeometryKind::MultiPolygon,
            GeometryKind::Collection,
            GeometryKind::Feature,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "point".to_string(),
            "line".to_string(),
            "polygon".to_string(),
            "multipoint".to_string(),
            "multiline".to_string(),
            "multipolygon".to_string(),
            "collection".to_string(),
            "feature".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "point" => Some(GeometryKind::Point),
            "line" => Some(GeometryKind::Line),
            "polygon" => Some(GeometryKind::Polygon),
            "multipoint" => Some(GeometryKind::MultiPoint),
            "multiline" => Some(GeometryKind::MultiLine),
            "multipolygon" => Some(GeometryKind::MultiPolygon),
            "collection" => Some(GeometryKind::Collection),
            "feature" => Some(GeometryKind::Feature),
            _ => None,
        }
    }
}

crate::default_style!(GeometryKind => GeometryKindStyle);

impl Elicitation for GeometryKind {
    type Style = GeometryKindStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting GeometryKind");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose the geometry type:"),
            &Self::labels(),
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let label = mcp::parse_string(mcp::extract_value(result)?)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid GeometryKind: {}",
                label
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("GeometryKind", "point")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("GeometryKind", "point")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("GeometryKind", "point")
    }
}

impl ElicitIntrospect for GeometryKind {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "GeometryKind",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for GeometryKind {
    fn prompt_tree() -> crate::PromptTree {
        let opts = Self::labels();
        let n = opts.len();
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose the SurrealDB geometry kind:")
                .to_string(),
            type_name: "GeometryKind".to_string(),
            options: opts,
            branches: vec![None; n],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for GeometryKind {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let json = serde_json::to_string(self).expect("GeometryKind should serialize");
        quote::quote! {
            ::serde_json::from_str::<elicitation::GeometryKind>(#json)
                .expect("serialized GeometryKind should deserialize")
        }
    }
}

// ── Kind ────────────────────────────────────────────────────────────────────

/// Labels used in the Kind selector — unit variants first, then data variants.
fn kind_all_labels() -> Vec<&'static str> {
    vec![
        "any",
        "none",
        "null",
        "bool",
        "bytes",
        "datetime",
        "decimal",
        "duration",
        "float",
        "int",
        "number",
        "object",
        "string",
        "uuid",
        "regex",
        "range",
        // data variants
        "table",
        "record",
        "geometry",
        "array",
        "set",
        "file",
        "literal_string",
        "literal_int",
        "literal_float",
        "literal_bool",
        // complex recursive — ask for JSON
        "either",
    ]
}

impl Prompt for Kind {
    fn prompt() -> Option<&'static str> {
        Some("Choose the SurrealDB field type kind:")
    }
}

/// Select covers only the 16 unit variants for simple interactive use.
impl Select for Kind {
    fn options() -> Vec<Self> {
        vec![
            Kind::Any,
            Kind::None,
            Kind::Null,
            Kind::Bool,
            Kind::Bytes,
            Kind::Datetime,
            Kind::Decimal,
            Kind::Duration,
            Kind::Float,
            Kind::Int,
            Kind::Number,
            Kind::Object,
            Kind::String,
            Kind::Uuid,
            Kind::Regex,
            Kind::Range,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "any".to_string(),
            "none".to_string(),
            "null".to_string(),
            "bool".to_string(),
            "bytes".to_string(),
            "datetime".to_string(),
            "decimal".to_string(),
            "duration".to_string(),
            "float".to_string(),
            "int".to_string(),
            "number".to_string(),
            "object".to_string(),
            "string".to_string(),
            "uuid".to_string(),
            "regex".to_string(),
            "range".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "any" => Some(Kind::Any),
            "none" => Some(Kind::None),
            "null" => Some(Kind::Null),
            "bool" => Some(Kind::Bool),
            "bytes" => Some(Kind::Bytes),
            "datetime" => Some(Kind::Datetime),
            "decimal" => Some(Kind::Decimal),
            "duration" => Some(Kind::Duration),
            "float" => Some(Kind::Float),
            "int" => Some(Kind::Int),
            "number" => Some(Kind::Number),
            "object" => Some(Kind::Object),
            "string" => Some(Kind::String),
            "uuid" => Some(Kind::Uuid),
            "regex" => Some(Kind::Regex),
            "range" => Some(Kind::Range),
            _ => None,
        }
    }
}

crate::default_style!(Kind => KindStyle);

impl Elicitation for Kind {
    type Style = KindStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Kind");
        let all_labels: Vec<String> = kind_all_labels().into_iter().map(String::from).collect();
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose the SurrealDB field type kind:"),
            &all_labels,
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let label = mcp::parse_string(mcp::extract_value(result)?)?;
        tracing::debug!(kind = %label, "Selected Kind variant");

        // Unit variants
        if let Some(k) = Kind::from_label(&label) {
            return Ok(k);
        }

        // Data variants — ask follow-up question(s)
        match label.as_str() {
            "table" => {
                let p = mcp::text_params(
                    "Enter comma-separated table names to restrict to (leave blank for any):",
                );
                let r = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                            .with_arguments(p),
                    )
                    .await?;
                let s = mcp::parse_string(mcp::extract_value(r)?)?;
                let names = parse_csv(&s);
                Ok(Kind::Table(names))
            }
            "record" => {
                let p = mcp::text_params(
                    "Enter comma-separated table names to restrict to (leave blank for any):",
                );
                let r = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                            .with_arguments(p),
                    )
                    .await?;
                let s = mcp::parse_string(mcp::extract_value(r)?)?;
                let names = parse_csv(&s);
                Ok(Kind::Record(names))
            }
            "geometry" => {
                let p = mcp::text_params(
                    "Enter comma-separated geometry kinds to restrict to (point, line, polygon, …; leave blank for any):",
                );
                let r = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                            .with_arguments(p),
                    )
                    .await?;
                let s = mcp::parse_string(mcp::extract_value(r)?)?;
                let gks = parse_csv(&s)
                    .into_iter()
                    .filter_map(|g| GeometryKind::from_label(&g))
                    .collect();
                Ok(Kind::Geometry(gks))
            }
            "array" | "set" => {
                let elem_labels = Kind::labels();
                let elem_params =
                    mcp::select_params("Choose the element type (unit types only):", &elem_labels);
                let elem_result = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                            .with_arguments(elem_params),
                    )
                    .await?;
                let elem_label = mcp::parse_string(mcp::extract_value(elem_result)?)?;
                let elem = Kind::from_label(&elem_label).unwrap_or(Kind::Any);

                let max_params =
                    mcp::text_params("Enter the maximum count (leave blank for unlimited):");
                let max_result = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                            .with_arguments(max_params),
                    )
                    .await?;
                let max_str = mcp::parse_string(mcp::extract_value(max_result)?)?;
                let max: Option<u64> = max_str.trim().parse().ok();

                if label == "array" {
                    Ok(Kind::Array(Box::new(elem), max))
                } else {
                    Ok(Kind::Set(Box::new(elem), max))
                }
            }
            "file" => {
                let p = mcp::text_params(
                    "Enter comma-separated bucket names to restrict to (leave blank for any):",
                );
                let r = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                            .with_arguments(p),
                    )
                    .await?;
                let s = mcp::parse_string(mcp::extract_value(r)?)?;
                Ok(Kind::File(parse_csv(&s)))
            }
            "literal_string" => {
                let p = mcp::text_params("Enter the literal string value:");
                let r = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                            .with_arguments(p),
                    )
                    .await?;
                Ok(Kind::LiteralString(mcp::parse_string(mcp::extract_value(
                    r,
                )?)?))
            }
            "literal_int" => {
                let p = mcp::text_params("Enter the literal integer value:");
                let r = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                            .with_arguments(p),
                    )
                    .await?;
                let s = mcp::parse_string(mcp::extract_value(r)?)?;
                let i: i64 = s.trim().parse().map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid integer: {}",
                        e
                    )))
                })?;
                Ok(Kind::LiteralInt(i))
            }
            "literal_float" => {
                let p = mcp::text_params("Enter the literal float value:");
                let r = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                            .with_arguments(p),
                    )
                    .await?;
                let s = mcp::parse_string(mcp::extract_value(r)?)?;
                let f: f64 = s.trim().parse().map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid float: {}", e)))
                })?;
                Ok(Kind::LiteralFloat(f))
            }
            "literal_bool" => {
                let bool_labels = vec!["true".to_string(), "false".to_string()];
                let p = mcp::select_params("Choose the literal boolean value:", &bool_labels);
                let r = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                            .with_arguments(p),
                    )
                    .await?;
                let s = mcp::parse_string(mcp::extract_value(r)?)?;
                Ok(Kind::LiteralBool(s.trim() == "true"))
            }
            // "either" — recursive union type; fall back to Any for interactive use
            _ => Ok(Kind::Any),
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("Kind", "any")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("Kind", "any")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("Kind", "any")
    }
}

impl ElicitIntrospect for Kind {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "SurrealKind",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: kind_all_labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label: label.to_string(),
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for Kind {
    fn prompt_tree() -> crate::PromptTree {
        let opts: Vec<String> = kind_all_labels().into_iter().map(String::from).collect();
        let n = opts.len();
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose the SurrealDB field type kind:")
                .to_string(),
            type_name: "SurrealKind".to_string(),
            options: opts,
            branches: vec![None; n],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for Kind {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let json = serde_json::to_string(self).expect("Kind should serialize");
        quote::quote! {
            ::serde_json::from_str::<elicitation::SurrealKind>(#json)
                .expect("serialized SurrealKind should deserialize")
        }
    }
}

/// Split a comma-separated string into trimmed, non-empty tokens.
fn parse_csv(s: &str) -> Vec<String> {
    s.split(',')
        .map(str::trim)
        .filter(|t| !t.is_empty())
        .map(String::from)
        .collect()
}
