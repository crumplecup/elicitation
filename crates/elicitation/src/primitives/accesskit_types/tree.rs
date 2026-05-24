//! Elicitation for [`accesskit::Tree`].

use accesskit::{NodeId, Tree};

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata, emit_code::ToCodeLiteral,
};

impl Prompt for Tree {
    fn prompt() -> Option<&'static str> {
        Some("Specify an accessibility tree (root node ID and optional toolkit info):")
    }
}

crate::default_style!(Tree => TreeElicitStyle);

impl Elicitation for Tree {
    type Style = TreeElicitStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting accesskit::Tree");
        Ok(Self {
            root: NodeId::elicit(communicator).await?,
            toolkit_name: <Option<String>>::elicit(communicator).await?,
            toolkit_version: <Option<String>>::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <NodeId as Elicitation>::kani_proof();
        ts.extend(<Option<String> as Elicitation>::kani_proof());
        ts.extend(<Option<String> as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <NodeId as Elicitation>::verus_proof();
        ts.extend(<Option<String> as Elicitation>::verus_proof());
        ts.extend(<Option<String> as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <NodeId as Elicitation>::creusot_proof();
        ts.extend(<Option<String> as Elicitation>::creusot_proof());
        ts.extend(<Option<String> as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for Tree {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "accesskit::Tree",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "root",
                        type_name: "accesskit::NodeId",
                        prompt: Some("Root node ID:"),
                    },
                    FieldInfo {
                        name: "toolkit_name",
                        type_name: "Option<String>",
                        prompt: Some("UI toolkit name (e.g. \"egui\"):"),
                    },
                    FieldInfo {
                        name: "toolkit_version",
                        type_name: "Option<String>",
                        prompt: Some("UI toolkit version (e.g. \"0.29.0\"):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for Tree {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "accesskit::Tree".to_string(),
            fields: vec![
                ("root".to_string(), Box::new(NodeId::prompt_tree())),
                (
                    "toolkit_name".to_string(),
                    Box::new(<Option<String>>::prompt_tree()),
                ),
                (
                    "toolkit_version".to_string(),
                    Box::new(<Option<String>>::prompt_tree()),
                ),
            ],
        }
    }
}

impl ToCodeLiteral for Tree {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let root_lit = self.root.to_code_literal();
        match (&self.toolkit_name, &self.toolkit_version) {
            (Some(name), Some(version)) => quote::quote! {
                {
                    let mut _t = accesskit::Tree::new(#root_lit);
                    _t.toolkit_name = Some(#name.to_string());
                    _t.toolkit_version = Some(#version.to_string());
                    _t
                }
            },
            (Some(name), None) => quote::quote! {
                {
                    let mut _t = accesskit::Tree::new(#root_lit);
                    _t.toolkit_name = Some(#name.to_string());
                    _t
                }
            },
            (None, Some(version)) => quote::quote! {
                {
                    let mut _t = accesskit::Tree::new(#root_lit);
                    _t.toolkit_version = Some(#version.to_string());
                    _t
                }
            },
            (None, None) => quote::quote! { accesskit::Tree::new(#root_lit) },
        }
    }
}
