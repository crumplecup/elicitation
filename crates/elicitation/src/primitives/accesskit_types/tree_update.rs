//! Elicitation for [`accesskit::TreeUpdate`].
//!
//! `TreeUpdate` is the main message type for atomic accessibility tree updates.
//! The `tree_id` field wraps a [`uuid::Uuid`] via [`accesskit::TreeId`].

use accesskit::{Node, NodeId, Tree, TreeId, TreeUpdate};
use uuid::Uuid;

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata, emit_code::ToCodeLiteral,
};

impl Prompt for TreeUpdate {
    fn prompt() -> Option<&'static str> {
        Some("Specify an accessibility tree update (nodes, optional tree metadata, tree ID, and focus):")
    }
}

crate::default_style!(TreeUpdate => TreeUpdateStyle);

impl Elicitation for TreeUpdate {
    type Style = TreeUpdateStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting accesskit::TreeUpdate");
        let nodes = <Vec<(NodeId, Node)>>::elicit(communicator).await?;
        let tree = <Option<Tree>>::elicit(communicator).await?;
        let uuid = Uuid::elicit(communicator).await?;
        let tree_id = TreeId(uuid);
        let focus = NodeId::elicit(communicator).await?;
        Ok(Self { nodes, tree, tree_id, focus })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <Vec<(NodeId, Node)> as Elicitation>::kani_proof();
        ts.extend(<Option<Tree> as Elicitation>::kani_proof());
        ts.extend(<Uuid as Elicitation>::kani_proof());
        ts.extend(<NodeId as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <Vec<(NodeId, Node)> as Elicitation>::verus_proof();
        ts.extend(<Option<Tree> as Elicitation>::verus_proof());
        ts.extend(<Uuid as Elicitation>::verus_proof());
        ts.extend(<NodeId as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <Vec<(NodeId, Node)> as Elicitation>::creusot_proof();
        ts.extend(<Option<Tree> as Elicitation>::creusot_proof());
        ts.extend(<Uuid as Elicitation>::creusot_proof());
        ts.extend(<NodeId as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for TreeUpdate {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "accesskit::TreeUpdate",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "nodes",
                        type_name: "Vec<(NodeId, Node)>",
                        prompt: Some("Nodes to add or update (ID + node pairs):"),
                    },
                    FieldInfo {
                        name: "tree",
                        type_name: "Option<accesskit::Tree>",
                        prompt: Some("Optional tree-level metadata (required on first update):"),
                    },
                    FieldInfo {
                        name: "tree_id",
                        type_name: "accesskit::TreeId",
                        prompt: Some("Tree UUID (use TreeId::ROOT for the main tree):"),
                    },
                    FieldInfo {
                        name: "focus",
                        type_name: "accesskit::NodeId",
                        prompt: Some("Focused node ID (must be set even if focus did not change):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TreeUpdate {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "accesskit::TreeUpdate".to_string(),
            fields: vec![
                ("nodes".to_string(), Box::new(<Vec<(NodeId, Node)>>::prompt_tree())),
                ("tree".to_string(), Box::new(<Option<Tree>>::prompt_tree())),
                ("tree_id".to_string(), Box::new(Uuid::prompt_tree())),
                ("focus".to_string(), Box::new(NodeId::prompt_tree())),
            ],
        }
    }
}

impl ToCodeLiteral for TreeUpdate {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let uuid_str = self.tree_id.0.to_string();
        let focus_lit = self.focus.to_code_literal();
        quote::quote! {
            accesskit::TreeUpdate {
                nodes: vec![],
                tree: None,
                tree_id: accesskit::TreeId(uuid::Uuid::parse_str(#uuid_str).unwrap()),
                focus: #focus_lit,
            }
        }
    }
}
