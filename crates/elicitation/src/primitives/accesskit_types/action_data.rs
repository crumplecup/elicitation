//! Elicitation for [`accesskit::ActionData`].
//!
//! `ActionData` is a non-unit enum where each variant carries a payload.
//! Elicitation first selects the variant, then elicits the inner value.

use accesskit::{ActionData, Point, ScrollHint, ScrollUnit, TextSelection};

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata,
    emit_code::ToCodeLiteral, mcp,
};

// ── Variant labels ────────────────────────────────────────────────────────────

const LABELS: &[&str] = &[
    "customAction",
    "value",
    "numericValue",
    "scrollUnit",
    "scrollHint",
    "scrollToPoint",
    "setScrollOffset",
    "setTextSelection",
];

// ── Prompt + Select ───────────────────────────────────────────────────────────

impl Prompt for ActionData {
    fn prompt() -> Option<&'static str> {
        Some("Choose the type of action data payload:")
    }
}

impl Select for ActionData {
    fn options() -> Vec<Self> {
        vec![
            ActionData::CustomAction(0),
            ActionData::Value(String::new().into_boxed_str()),
            ActionData::NumericValue(0.0),
            ActionData::ScrollUnit(ScrollUnit::Item),
            ActionData::ScrollHint(ScrollHint::TopLeft),
            ActionData::ScrollToPoint(Point { x: 0.0, y: 0.0 }),
            ActionData::SetScrollOffset(Point { x: 0.0, y: 0.0 }),
            ActionData::SetTextSelection(TextSelection {
                anchor: accesskit::TextPosition {
                    node: accesskit::NodeId(1),
                    character_index: 0,
                },
                focus: accesskit::TextPosition {
                    node: accesskit::NodeId(1),
                    character_index: 0,
                },
            }),
        ]
    }

    fn labels() -> Vec<String> {
        LABELS.iter().map(|s| s.to_string()).collect()
    }

    fn from_label(label: &str) -> Option<Self> {
        // Returns a placeholder; real construction happens in elicit()
        LABELS
            .iter()
            .position(|&l| l == label)
            .map(|i| Self::options()[i].clone())
    }
}

crate::default_style!(ActionData => ActionDataStyle);

impl Elicitation for ActionData {
    type Style = ActionDataStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting accesskit::ActionData variant");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose ActionData variant:"),
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

        match label.as_str() {
            "customAction" => {
                let id = i32::elicit(communicator).await?;
                Ok(ActionData::CustomAction(id))
            }
            "value" => {
                let s = String::elicit(communicator).await?;
                Ok(ActionData::Value(s.into_boxed_str()))
            }
            "numericValue" => {
                let n = f64::elicit(communicator).await?;
                Ok(ActionData::NumericValue(n))
            }
            "scrollUnit" => {
                let unit = ScrollUnit::elicit(communicator).await?;
                Ok(ActionData::ScrollUnit(unit))
            }
            "scrollHint" => {
                let hint = ScrollHint::elicit(communicator).await?;
                Ok(ActionData::ScrollHint(hint))
            }
            "scrollToPoint" => {
                let pt = Point::elicit(communicator).await?;
                Ok(ActionData::ScrollToPoint(pt))
            }
            "setScrollOffset" => {
                let pt = Point::elicit(communicator).await?;
                Ok(ActionData::SetScrollOffset(pt))
            }
            "setTextSelection" => {
                let sel = TextSelection::elicit(communicator).await?;
                Ok(ActionData::SetTextSelection(sel))
            }
            other => Err(ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid ActionData variant: {other}"
            )))),
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "accesskit::ActionData",
            "customAction",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "accesskit::ActionData",
            "customAction",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "accesskit::ActionData",
            "customAction",
        )
    }
}

impl ElicitIntrospect for ActionData {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "accesskit::ActionData",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: vec![
                    VariantMetadata {
                        label: "customAction".to_string(),
                        fields: vec![FieldInfo {
                            name: "0",
                            type_name: "i32",
                            prompt: Some("Custom action ID:"),
                        }],
                    },
                    VariantMetadata {
                        label: "value".to_string(),
                        fields: vec![FieldInfo {
                            name: "0",
                            type_name: "Box<str>",
                            prompt: Some("String value:"),
                        }],
                    },
                    VariantMetadata {
                        label: "numericValue".to_string(),
                        fields: vec![FieldInfo {
                            name: "0",
                            type_name: "f64",
                            prompt: Some("Numeric value:"),
                        }],
                    },
                    VariantMetadata {
                        label: "scrollUnit".to_string(),
                        fields: vec![FieldInfo {
                            name: "0",
                            type_name: "accesskit::ScrollUnit",
                            prompt: Some("Scroll unit:"),
                        }],
                    },
                    VariantMetadata {
                        label: "scrollHint".to_string(),
                        fields: vec![FieldInfo {
                            name: "0",
                            type_name: "accesskit::ScrollHint",
                            prompt: Some("Scroll hint position:"),
                        }],
                    },
                    VariantMetadata {
                        label: "scrollToPoint".to_string(),
                        fields: vec![FieldInfo {
                            name: "0",
                            type_name: "accesskit::geometry::Point",
                            prompt: Some("Target point (x, y):"),
                        }],
                    },
                    VariantMetadata {
                        label: "setScrollOffset".to_string(),
                        fields: vec![FieldInfo {
                            name: "0",
                            type_name: "accesskit::geometry::Point",
                            prompt: Some("Scroll offset (x, y):"),
                        }],
                    },
                    VariantMetadata {
                        label: "setTextSelection".to_string(),
                        fields: vec![FieldInfo {
                            name: "0",
                            type_name: "accesskit::TextSelection",
                            prompt: Some("Text selection (anchor + focus):"),
                        }],
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for ActionData {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose ActionData variant:")
                .to_string(),
            type_name: "accesskit::ActionData".to_string(),
            options: Self::labels(),
            branches: vec![
                Some(Box::new(i32::prompt_tree())),
                Some(Box::new(String::prompt_tree())),
                Some(Box::new(f64::prompt_tree())),
                Some(Box::new(ScrollUnit::prompt_tree())),
                Some(Box::new(ScrollHint::prompt_tree())),
                Some(Box::new(Point::prompt_tree())),
                Some(Box::new(Point::prompt_tree())),
                Some(Box::new(TextSelection::prompt_tree())),
            ],
        }
    }
}

impl ToCodeLiteral for ActionData {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            ActionData::CustomAction(id) => {
                quote::quote! { accesskit::ActionData::CustomAction(#id) }
            }
            ActionData::Value(s) => {
                let s = s.as_ref();
                quote::quote! { accesskit::ActionData::Value(#s.to_string().into_boxed_str()) }
            }
            ActionData::NumericValue(n) => {
                quote::quote! { accesskit::ActionData::NumericValue(#n) }
            }
            ActionData::ScrollUnit(u) => {
                let u_lit = u.to_code_literal();
                quote::quote! { accesskit::ActionData::ScrollUnit(#u_lit) }
            }
            ActionData::ScrollHint(h) => {
                let h_lit = h.to_code_literal();
                quote::quote! { accesskit::ActionData::ScrollHint(#h_lit) }
            }
            ActionData::ScrollToPoint(p) => {
                let p_lit = p.to_code_literal();
                quote::quote! { accesskit::ActionData::ScrollToPoint(#p_lit) }
            }
            ActionData::SetScrollOffset(p) => {
                let p_lit = p.to_code_literal();
                quote::quote! { accesskit::ActionData::SetScrollOffset(#p_lit) }
            }
            ActionData::SetTextSelection(sel) => {
                let sel_lit = sel.to_code_literal();
                quote::quote! { accesskit::ActionData::SetTextSelection(#sel_lit) }
            }
        }
    }
}
