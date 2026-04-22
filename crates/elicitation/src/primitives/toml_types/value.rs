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

impl Elicitation for TomlValue {
    type Style = TomlValueStyle;

    // Use explicit fn + Box::pin to break the self-referential cycle between
    // TomlValue and Vec<TomlValue>. Without boxing the compiler cannot resolve
    // the opaque Future type for the recursive async call.
    #[tracing::instrument(skip(communicator))]
    fn elicit<C: ElicitCommunicator>(
        communicator: &C,
    ) -> impl std::future::Future<Output = ElicitResult<Self>> + Send {
        let comm = communicator.clone();
        Box::pin(async move {
            let communicator = &comm;
            tracing::debug!("Eliciting TomlValue");
            let params = mcp::select_params(
                Self::prompt().unwrap_or("Choose value type:"),
                &Self::labels(),
            );
            let result = communicator
                .call_tool(
                    rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                        .with_arguments(params),
                )
                .await?;
            let value = mcp::extract_value(result)?;
            let label = mcp::parse_string(value)?;
            tracing::debug!(variant = %label, "TomlValue variant selected");
            match label.as_str() {
                "String" => Ok(TomlValue::String(String::elicit(communicator).await?)),
                "Integer" => Ok(TomlValue::Integer(i64::elicit(communicator).await?)),
                "Float" => Ok(TomlValue::Float(f64::elicit(communicator).await?)),
                "Boolean" => Ok(TomlValue::Boolean(bool::elicit(communicator).await?)),
                "Datetime" => Ok(TomlValue::Datetime(
                    TomlDatetime::elicit(communicator).await?,
                )),
                "Array" => Ok(TomlValue::Array(
                    Vec::<TomlValue>::elicit(communicator).await?,
                )),
                "Table" => Ok(TomlValue::Table(
                    Vec::<(String, TomlValue)>::elicit(communicator).await?,
                )),
                _ => Err(ElicitError::new(ElicitErrorKind::ParseError(format!(
                    "Invalid TomlValue: {label}"
                )))),
            }
        })
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
