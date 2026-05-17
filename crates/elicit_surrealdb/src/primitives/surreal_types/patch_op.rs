//! JSON Patch operation shadow type.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A JSON Patch operation for `UPDATE … PATCH [PatchOp, …]` in SurrealDB.
///
/// Mirrors the JSON Patch RFC 6902 operations supported by SurrealDB's
/// `PATCH` DML statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "op", rename_all = "lowercase")]
pub enum PatchOp {
    /// Add a value at the given JSON pointer path.
    Add {
        /// JSON pointer path (e.g. `/name`).
        path: String,
        /// The value to add.
        value: serde_json::Value,
    },
    /// Remove the value at the given JSON pointer path.
    Remove {
        /// JSON pointer path.
        path: String,
    },
    /// Replace the value at the given JSON pointer path.
    Replace {
        /// JSON pointer path.
        path: String,
        /// The replacement value.
        value: serde_json::Value,
    },
    /// Change a string at the given path using a diff patch.
    Change {
        /// JSON pointer path.
        path: String,
        /// The diff string.
        value: String,
    },
    /// Copy a value from one path to another.
    Copy {
        /// The source JSON pointer path.
        from: String,
        /// The target JSON pointer path.
        path: String,
    },
    /// Move a value from one path to another.
    Move {
        /// The source JSON pointer path.
        from: String,
        /// The target JSON pointer path.
        path: String,
    },
    /// Test that a value at the given path equals a given value.
    Test {
        /// JSON pointer path.
        path: String,
        /// The expected value.
        value: serde_json::Value,
    },
    /// Increment the numeric value at the given path.
    Increment {
        /// JSON pointer path.
        path: String,
        /// The amount to increment by.
        value: serde_json::Value,
    },
    /// Decrement the numeric value at the given path.
    Decrement {
        /// JSON pointer path.
        path: String,
        /// The amount to decrement by.
        value: serde_json::Value,
    },
}

use elicitation::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};

impl Prompt for PatchOp {
    fn prompt() -> Option<&'static str> {
        Some("Choose the JSON Patch operation:")
    }
}

impl Select for PatchOp {
    fn options() -> Vec<Self> {
        vec![
            PatchOp::Add {
                path: "/field".into(),
                value: serde_json::Value::Null,
            },
            PatchOp::Remove {
                path: "/field".into(),
            },
            PatchOp::Replace {
                path: "/field".into(),
                value: serde_json::Value::Null,
            },
            PatchOp::Change {
                path: "/field".into(),
                value: String::new(),
            },
            PatchOp::Copy {
                from: "/src".into(),
                path: "/dst".into(),
            },
            PatchOp::Move {
                from: "/src".into(),
                path: "/dst".into(),
            },
            PatchOp::Test {
                path: "/field".into(),
                value: serde_json::Value::Null,
            },
            PatchOp::Increment {
                path: "/field".into(),
                value: serde_json::Value::Number(1.into()),
            },
            PatchOp::Decrement {
                path: "/field".into(),
                value: serde_json::Value::Number(1.into()),
            },
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "add".to_string(),
            "remove".to_string(),
            "replace".to_string(),
            "change".to_string(),
            "copy".to_string(),
            "move".to_string(),
            "test".to_string(),
            "increment".to_string(),
            "decrement".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "add" => Some(PatchOp::Add {
                path: "/field".into(),
                value: serde_json::Value::Null,
            }),
            "remove" => Some(PatchOp::Remove {
                path: "/field".into(),
            }),
            "replace" => Some(PatchOp::Replace {
                path: "/field".into(),
                value: serde_json::Value::Null,
            }),
            "change" => Some(PatchOp::Change {
                path: "/field".into(),
                value: String::new(),
            }),
            "copy" => Some(PatchOp::Copy {
                from: "/src".into(),
                path: "/dst".into(),
            }),
            "move" => Some(PatchOp::Move {
                from: "/src".into(),
                path: "/dst".into(),
            }),
            "test" => Some(PatchOp::Test {
                path: "/field".into(),
                value: serde_json::Value::Null,
            }),
            "increment" => Some(PatchOp::Increment {
                path: "/field".into(),
                value: serde_json::Value::Number(1.into()),
            }),
            "decrement" => Some(PatchOp::Decrement {
                path: "/field".into(),
                value: serde_json::Value::Number(1.into()),
            }),
            _ => None,
        }
    }
}

elicitation::default_style!(PatchOp => PatchOpStyle);

impl Elicitation for PatchOp {
    type Style = PatchOpStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting PatchOp");
        let op_params = mcp::select_params(
            Self::prompt().unwrap_or("Choose the JSON Patch operation:"),
            &Self::labels(),
        );
        let op_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(op_params),
            )
            .await?;
        let op = mcp::parse_string(mcp::extract_value(op_result)?)?;
        tracing::debug!(op = %op, "Selected PatchOp");

        let path = {
            let p = mcp::text_params("Enter the JSON pointer path (e.g. /name, /address/city):");
            let r = communicator
                .call_tool(
                    rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                        .with_arguments(p),
                )
                .await?;
            mcp::parse_string(mcp::extract_value(r)?)?
        };

        match op.as_str() {
            "remove" => Ok(PatchOp::Remove { path }),
            "copy" | "move" => {
                let from_p = mcp::text_params("Enter the source JSON pointer path (from):");
                let from_r = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                            .with_arguments(from_p),
                    )
                    .await?;
                let from = mcp::parse_string(mcp::extract_value(from_r)?)?;
                if op == "copy" {
                    Ok(PatchOp::Copy { from, path })
                } else {
                    Ok(PatchOp::Move { from, path })
                }
            }
            "change" => {
                let val_p = mcp::text_params("Enter the diff patch string:");
                let val_r = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                            .with_arguments(val_p),
                    )
                    .await?;
                let value = mcp::parse_string(mcp::extract_value(val_r)?)?;
                Ok(PatchOp::Change { path, value })
            }
            _ => {
                // add, replace, test, increment, decrement — all need a JSON value
                let val_p = mcp::text_params(
                    "Enter the JSON value (e.g. \"hello\", 42, true, {\"x\": 1}):",
                );
                let val_r = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                            .with_arguments(val_p),
                    )
                    .await?;
                let val_str = mcp::parse_string(mcp::extract_value(val_r)?)?;
                let value: serde_json::Value =
                    serde_json::from_str(val_str.trim()).map_err(|e| {
                        ElicitError::new(ElicitErrorKind::ParseError(format!(
                            "Invalid JSON value \"{}\": {}",
                            val_str.trim(),
                            e
                        )))
                    })?;
                match op.as_str() {
                    "add" => Ok(PatchOp::Add { path, value }),
                    "replace" => Ok(PatchOp::Replace { path, value }),
                    "test" => Ok(PatchOp::Test { path, value }),
                    "increment" => Ok(PatchOp::Increment { path, value }),
                    _ => Ok(PatchOp::Decrement { path, value }),
                }
            }
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        elicitation::verification::proof_helpers::kani_select_wrapper("PatchOp", "add")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        elicitation::verification::proof_helpers::verus_select_wrapper("PatchOp", "add")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        elicitation::verification::proof_helpers::creusot_select_wrapper("PatchOp", "add")
    }
}

impl ElicitIntrospect for PatchOp {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "SurrealPatchOp",
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

impl elicitation::ElicitPromptTree for PatchOp {
    fn prompt_tree() -> elicitation::PromptTree {
        let opts = Self::labels();
        let n = opts.len();
        elicitation::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose the JSON Patch operation:")
                .to_string(),
            type_name: "SurrealPatchOp".to_string(),
            options: opts,
            branches: vec![None; n],
        }
    }
}

impl elicitation::emit_code::ToCodeLiteral for PatchOp {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let json = serde_json::to_string(self).expect("PatchOp should serialize");
        quote::quote! {
            ::serde_json::from_str::<elicit_surrealdb::SurrealPatchOp>(#json)
                .expect("serialized SurrealPatchOp should deserialize")
        }
    }
}
