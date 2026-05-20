//! UOM workflow descriptor types.
//!
//! Available with the `uom-types` feature.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::UomQuantityKind;

/// A single step in a unit-of-measurement computation workflow.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct UomStep {
    /// Human-readable description of the step.
    pub description: String,
    /// The quantity kind produced by this step.
    pub kind: UomQuantityKind,
    /// Rust code snippet that implements this step.
    pub code_snippet: String,
}

/// Descriptor for a physics formula involving quantities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct UomFormula {
    /// Formula name (e.g. `"KineticEnergy"`).
    pub name: String,
    /// Symbolic formula string (e.g. `"E = ½mv²"`).
    pub formula: String,
    /// Description of what the formula computes.
    pub description: String,
    /// Ordered list of parameter names and their quantity kinds.
    pub params: Vec<(String, UomQuantityKind)>,
    /// The quantity kind of the result.
    pub result_kind: UomQuantityKind,
}

// ============================================================================
// Elicitation traits
// ============================================================================

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    FieldInfo, PatternDetails, Prompt, TypeMetadata,
};

// --- UomStep -----------------------------------------------------------------

impl Prompt for UomStep {
    fn prompt() -> Option<&'static str> {
        Some("Describe a single step in a unit-of-measurement computation:")
    }
}

crate::default_style!(UomStep => UomStepStyle);

impl Elicitation for UomStep {
    type Style = UomStepStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting UomStep");
        let description = String::elicit(communicator).await?;
        let kind = UomQuantityKind::elicit(communicator).await?;
        let code_snippet = String::elicit(communicator).await?;
        Ok(Self { description, kind, code_snippet })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <String as crate::Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <String as crate::Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <String as crate::Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for UomStep {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Survey }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::UomStep",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo { name: "description", type_name: "String", prompt: Some("Step description:") },
                    FieldInfo { name: "kind", type_name: "UomQuantityKind", prompt: Some("Quantity kind produced:") },
                    FieldInfo { name: "code_snippet", type_name: "String", prompt: Some("Rust code snippet:") },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for UomStep {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "UomStep".to_string(),
            fields: vec![
                ("description".to_string(), Box::new(String::prompt_tree())),
                ("kind".to_string(), Box::new(UomQuantityKind::prompt_tree())),
                ("code_snippet".to_string(), Box::new(String::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for UomStep {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let description = &self.description;
        let kind = self.kind.to_code_literal();
        let code_snippet = &self.code_snippet;
        quote::quote! {
            elicitation::UomStep {
                description: #description.to_string(),
                kind: #kind,
                code_snippet: #code_snippet.to_string(),
            }
        }
    }
}

// --- UomFormula --------------------------------------------------------------

impl Prompt for UomFormula {
    fn prompt() -> Option<&'static str> {
        Some("Describe a physics formula involving unit quantities:")
    }
}

crate::default_style!(UomFormula => UomFormulaStyle);

impl Elicitation for UomFormula {
    type Style = UomFormulaStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting UomFormula");
        let name = String::elicit(communicator).await?;
        let formula = String::elicit(communicator).await?;
        let description = String::elicit(communicator).await?;
        let params = Vec::<(String, UomQuantityKind)>::elicit(communicator).await?;
        let result_kind = UomQuantityKind::elicit(communicator).await?;
        Ok(Self { name, formula, description, params, result_kind })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <String as crate::Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <String as crate::Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <String as crate::Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for UomFormula {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Survey }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::UomFormula",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo { name: "name", type_name: "String", prompt: Some("Formula name (e.g. \"KineticEnergy\"):") },
                    FieldInfo { name: "formula", type_name: "String", prompt: Some("Symbolic formula (e.g. \"E = ½mv²\"):") },
                    FieldInfo { name: "description", type_name: "String", prompt: Some("What the formula computes:") },
                    FieldInfo { name: "params", type_name: "Vec<(String, UomQuantityKind)>", prompt: Some("Parameter names and their quantity kinds:") },
                    FieldInfo { name: "result_kind", type_name: "UomQuantityKind", prompt: Some("Quantity kind of the result:") },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for UomFormula {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "UomFormula".to_string(),
            fields: vec![
                ("name".to_string(), Box::new(String::prompt_tree())),
                ("formula".to_string(), Box::new(String::prompt_tree())),
                ("description".to_string(), Box::new(String::prompt_tree())),
                ("params".to_string(), Box::new(String::prompt_tree())),
                ("result_kind".to_string(), Box::new(UomQuantityKind::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for UomFormula {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        let formula = &self.formula;
        let description = &self.description;
        let params: Vec<_> = self.params.iter().map(|(s, k)| {
            let kind = k.to_code_literal();
            quote::quote! { (#s.to_string(), #kind) }
        }).collect();
        let result_kind = self.result_kind.to_code_literal();
        quote::quote! {
            elicitation::UomFormula {
                name: #name.to_string(),
                formula: #formula.to_string(),
                description: #description.to_string(),
                params: vec![#(#params),*],
                result_kind: #result_kind,
            }
        }
    }
}
