//! Trenchcoat wrapper for [`toml::Value`].

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata,
    mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{TomlDate, TomlDatetime};

/// A TOML value covering all seven TOML primitive and compound types.
///
/// Wraps `toml::Value` to add [`JsonSchema`] for MCP boundary crossing.
/// Table entries use `Vec<(String, TomlValue)>` instead of `HashMap` to
/// preserve insertion order and satisfy `JsonSchema`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TomlValue {
    /// A UTF-8 string.
    String(String),
    /// A 64-bit signed integer.
    Integer(i64),
    /// A 64-bit floating-point number.
    Float(f64),
    /// A boolean.
    Boolean(bool),
    /// A datetime (offset, local-datetime, local-date, or local-time).
    Datetime(TomlDatetime),
    /// An array of TOML values.
    Array(Vec<TomlValue>),
    /// An inline table as an ordered list of key–value pairs.
    Table(Vec<(String, TomlValue)>),
}

#[cfg(feature = "toml-types")]
impl From<toml::Value> for TomlValue {
    fn from(v: toml::Value) -> Self {
        match v {
            toml::Value::String(s) => TomlValue::String(s),
            toml::Value::Integer(i) => TomlValue::Integer(i),
            toml::Value::Float(f) => TomlValue::Float(f),
            toml::Value::Boolean(b) => TomlValue::Boolean(b),
            toml::Value::Datetime(dt) => TomlValue::Datetime(TomlDatetime::from(dt)),
            toml::Value::Array(arr) => {
                TomlValue::Array(arr.into_iter().map(TomlValue::from).collect())
            }
            toml::Value::Table(map) => TomlValue::Table(
                map.into_iter()
                    .map(|(k, v)| (k, TomlValue::from(v)))
                    .collect(),
            ),
        }
    }
}

#[cfg(feature = "toml-types")]
impl From<TomlValue> for toml::Value {
    fn from(v: TomlValue) -> Self {
        match v {
            TomlValue::String(s) => toml::Value::String(s),
            TomlValue::Integer(i) => toml::Value::Integer(i),
            TomlValue::Float(f) => toml::Value::Float(f),
            TomlValue::Boolean(b) => toml::Value::Boolean(b),
            TomlValue::Datetime(dt) => toml::Value::Datetime(toml_datetime::Datetime::from(dt)),
            TomlValue::Array(arr) => {
                toml::Value::Array(arr.into_iter().map(toml::Value::from).collect())
            }
            TomlValue::Table(pairs) => toml::Value::Table(
                pairs
                    .into_iter()
                    .map(|(k, v)| (k, toml::Value::from(v)))
                    .collect(),
            ),
        }
    }
}

impl Prompt for TomlValue {
    fn prompt() -> Option<&'static str> {
        Some("Choose a TOML value type:")
    }
}

impl Select for TomlValue {
    fn options() -> Vec<Self> {
        let dummy_date = TomlDate {
            year: 2000,
            month: 1,
            day: 1,
        };
        vec![
            TomlValue::String(String::new()),
            TomlValue::Integer(0),
            TomlValue::Float(0.0),
            TomlValue::Boolean(false),
            TomlValue::Datetime(TomlDatetime::LocalDate { date: dummy_date }),
            TomlValue::Array(vec![]),
            TomlValue::Table(vec![]),
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "String".to_string(),
            "Integer".to_string(),
            "Float".to_string(),
            "Boolean".to_string(),
            "Datetime".to_string(),
            "Array".to_string(),
            "Table".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        let dummy_date = TomlDate {
            year: 2000,
            month: 1,
            day: 1,
        };
        match label {
            "String" => Some(TomlValue::String(String::new())),
            "Integer" => Some(TomlValue::Integer(0)),
            "Float" => Some(TomlValue::Float(0.0)),
            "Boolean" => Some(TomlValue::Boolean(false)),
            "Datetime" => Some(TomlValue::Datetime(TomlDatetime::LocalDate {
                date: dummy_date,
            })),
            "Array" => Some(TomlValue::Array(vec![])),
            "Table" => Some(TomlValue::Table(vec![])),
            _ => None,
        }
    }
}

crate::default_style!(TomlValue => TomlValueStyle);

/// Maximum nesting depth for depth-bounded inductive elicitation.
///
/// See `elicit_toml_value_inner` for the full rationale.
const TOML_VALUE_MAX_DEPTH: usize = 32;

/// Depth-bounded inductive elicitation for the self-referential `TomlValue` type.
///
/// ## Why `impl Future + Send` fails for self-recursive types
///
/// For mutually-recursive types (e.g. `GeoGeometry` ↔ `GeoGeometryCollection`),
/// removing `#[tracing::instrument]` and using `Box::pin` is sufficient: each side
/// returns `impl Future + Send`, and when checking `Send` on side A, the compiler
/// trusts B's *declared* `impl Future + Send` return type without re-entering B's
/// body.
///
/// For directly self-recursive types the same trick does not work.  
/// `TomlValue::elicit()` awaits `Vec::<TomlValue>::elicit()`.  
/// `Vec<T>::elicit()` is an `async fn` — to check whether it is `Send`, the
/// compiler must analyse its body, which awaits `T::elicit()`, i.e.
/// `TomlValue::elicit()` again.  Because every step is an opaque `impl Trait` type
/// that the compiler must peel open, the check never terminates.
///
/// ## Solution: concrete `Pin<Box<dyn Future + Send>>`
///
/// `Pin<Box<dyn Future<Output = _> + Send>>` is a *concrete* type.  Its `Send`
/// bound is structural — `Box<dyn … + Send>` is `Send` by definition, no body
/// analysis required.  Using it as the return type of the recursive helper stops
/// the inference cycle at the boundary of each recursive call site.
///
/// The depth parameter makes the recursion well-founded: if a user somehow builds
/// a TOML structure deeper than `TOML_VALUE_MAX_DEPTH`, elicitation returns an
/// error rather than looping.  The decrement at each recursive call also serves as
/// an inductive measure for formal-methods reasoning (see gallery C24).
///
/// ## What this replaces
///
/// The array and table arms bypass `Vec::<TomlValue>::elicit()` and
/// `Vec::<(String, TomlValue)>::elicit()` entirely, using local loops that call
/// this helper directly.  The standard `Vec<T>` impl (which uses `async fn` with
/// `#[tracing::instrument]`) would re-introduce the inference cycle.
fn elicit_toml_value_inner<C: ElicitCommunicator>(
    comm: C,
    depth: usize,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = ElicitResult<TomlValue>> + Send>> {
    Box::pin(async move {
        let communicator = &comm;
        if depth == 0 {
            return Err(ElicitError::new(ElicitErrorKind::ParseError(
                "TomlValue: maximum nesting depth exceeded".to_string(),
            )));
        }
        tracing::debug!(depth, "Eliciting TomlValue");
        let params = mcp::select_params(
            TomlValue::prompt().unwrap_or("Choose value type:"),
            &TomlValue::labels(),
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        tracing::debug!(variant = %label, depth, "TomlValue variant selected");
        match label.as_str() {
            "String" => Ok(TomlValue::String(String::elicit(communicator).await?)),
            "Integer" => Ok(TomlValue::Integer(i64::elicit(communicator).await?)),
            "Float" => Ok(TomlValue::Float(f64::elicit(communicator).await?)),
            "Boolean" => Ok(TomlValue::Boolean(bool::elicit(communicator).await?)),
            "Datetime" => Ok(TomlValue::Datetime(TomlDatetime::elicit(communicator).await?)),
            "Array" => {
                let items = elicit_toml_value_vec(comm.clone(), depth - 1).await?;
                Ok(TomlValue::Array(items))
            }
            "Table" => {
                let entries = elicit_toml_value_table(comm.clone(), depth - 1).await?;
                Ok(TomlValue::Table(entries))
            }
            _ => Err(ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid TomlValue: {label}"
            )))),
        }
    })
}

/// Collect `Vec<TomlValue>` via depth-bounded inner elicitation.
///
/// Replicates the standard `Vec<T>` bool-gated loop without going through the
/// generic `async fn Vec<T>::elicit()`, which would re-introduce the Send
/// inference cycle for `T = TomlValue`.
fn elicit_toml_value_vec<C: ElicitCommunicator>(
    comm: C,
    depth: usize,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = ElicitResult<Vec<TomlValue>>> + Send>> {
    Box::pin(async move {
        let mut items = Vec::new();
        loop {
            if !bool::elicit(&comm).await? {
                break;
            }
            items.push(elicit_toml_value_inner(comm.clone(), depth).await?);
        }
        Ok(items)
    })
}

/// Collect `Vec<(String, TomlValue)>` via depth-bounded inner elicitation.
fn elicit_toml_value_table<C: ElicitCommunicator>(
    comm: C,
    depth: usize,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = ElicitResult<Vec<(String, TomlValue)>>> + Send>,
> {
    Box::pin(async move {
        let mut entries = Vec::new();
        loop {
            if !bool::elicit(&comm).await? {
                break;
            }
            let key = String::elicit(&comm).await?;
            let value = elicit_toml_value_inner(comm.clone(), depth).await?;
            entries.push((key, value));
        }
        Ok(entries)
    })
}

impl Elicitation for TomlValue {
    type Style = TomlValueStyle;

    fn elicit<C: ElicitCommunicator>(
        communicator: &C,
    ) -> impl std::future::Future<Output = ElicitResult<Self>> + Send {
        elicit_toml_value_inner(communicator.clone(), TOML_VALUE_MAX_DEPTH)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <String as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <String as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <String as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for TomlValue {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "toml::Value",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: vec![
                    VariantMetadata {
                        label: "String".to_string(),
                        fields: vec![FieldInfo {
                            name: "value",
                            type_name: "String",
                            prompt: Some("String value:"),
                        }],
                    },
                    VariantMetadata {
                        label: "Integer".to_string(),
                        fields: vec![FieldInfo {
                            name: "value",
                            type_name: "i64",
                            prompt: Some("Integer value:"),
                        }],
                    },
                    VariantMetadata {
                        label: "Float".to_string(),
                        fields: vec![FieldInfo {
                            name: "value",
                            type_name: "f64",
                            prompt: Some("Float value:"),
                        }],
                    },
                    VariantMetadata {
                        label: "Boolean".to_string(),
                        fields: vec![FieldInfo {
                            name: "value",
                            type_name: "bool",
                            prompt: Some("Boolean value:"),
                        }],
                    },
                    VariantMetadata {
                        label: "Datetime".to_string(),
                        fields: vec![FieldInfo {
                            name: "value",
                            type_name: "TomlDatetime",
                            prompt: Some("Datetime value:"),
                        }],
                    },
                    VariantMetadata {
                        label: "Array".to_string(),
                        fields: vec![FieldInfo {
                            name: "value",
                            type_name: "Vec<TomlValue>",
                            prompt: Some("Array elements:"),
                        }],
                    },
                    VariantMetadata {
                        label: "Table".to_string(),
                        fields: vec![FieldInfo {
                            name: "value",
                            type_name: "Vec<(String, TomlValue)>",
                            prompt: Some("Table key–value pairs:"),
                        }],
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TomlValue {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Choose value type:").to_string(),
            type_name: "TomlValue".to_string(),
            options: Self::labels(),
            branches: vec![
                Some(Box::new(
                    String::prompt_tree().with_prompt(Some("String value:".to_string())),
                )),
                Some(Box::new(
                    i64::prompt_tree().with_prompt(Some("Integer value:".to_string())),
                )),
                Some(Box::new(
                    f64::prompt_tree().with_prompt(Some("Float value:".to_string())),
                )),
                Some(Box::new(
                    bool::prompt_tree().with_prompt(Some("Boolean value:".to_string())),
                )),
                Some(Box::new(
                    TomlDatetime::prompt_tree().with_prompt(Some("Datetime value:".to_string())),
                )),
                // Array and Table are recursive types; use Leaf nodes to break
                // the infinite-recursion cycle in prompt_tree construction.
                Some(Box::new(crate::PromptTree::Leaf {
                    prompt: "JSON array of TOML values (e.g. [1, \"a\", true]):".to_string(),
                    type_name: "Vec<TomlValue>".to_string(),
                })),
                Some(Box::new(crate::PromptTree::Leaf {
                    prompt: "JSON array of [key, value] pairs (e.g. [[\"k\", 1]]):".to_string(),
                    type_name: "Vec<(String, TomlValue)>".to_string(),
                })),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for TomlValue {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            TomlValue::String(s) => {
                let s = <String as crate::emit_code::ToCodeLiteral>::to_code_literal(s);
                quote::quote! { ::elicitation::TomlValue::String(#s) }
            }
            TomlValue::Integer(i) => {
                let i = <i64 as crate::emit_code::ToCodeLiteral>::to_code_literal(i);
                quote::quote! { ::elicitation::TomlValue::Integer(#i) }
            }
            TomlValue::Float(f) => {
                let f = <f64 as crate::emit_code::ToCodeLiteral>::to_code_literal(f);
                quote::quote! { ::elicitation::TomlValue::Float(#f) }
            }
            TomlValue::Boolean(b) => {
                let b = <bool as crate::emit_code::ToCodeLiteral>::to_code_literal(b);
                quote::quote! { ::elicitation::TomlValue::Boolean(#b) }
            }
            TomlValue::Datetime(dt) => {
                let dt = <TomlDatetime as crate::emit_code::ToCodeLiteral>::to_code_literal(dt);
                quote::quote! { ::elicitation::TomlValue::Datetime(#dt) }
            }
            TomlValue::Array(arr) => {
                let arr = <Vec<TomlValue> as crate::emit_code::ToCodeLiteral>::to_code_literal(arr);
                quote::quote! { ::elicitation::TomlValue::Array(#arr) }
            }
            TomlValue::Table(pairs) => {
                let pairs =
                    <Vec<(String, TomlValue)> as crate::emit_code::ToCodeLiteral>::to_code_literal(
                        pairs,
                    );
                quote::quote! { ::elicitation::TomlValue::Table(#pairs) }
            }
        }
    }
}
