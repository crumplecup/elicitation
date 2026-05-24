//! Elicitation for [`accesskit::ActionRequest`].

use accesskit::{Action, ActionData, ActionRequest, NodeId, TreeId};
use uuid::Uuid;

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata, emit_code::ToCodeLiteral,
};

impl Prompt for ActionRequest {
    fn prompt() -> Option<&'static str> {
        Some("Specify an accessibility action request:")
    }
}

crate::default_style!(ActionRequest => ActionRequestStyle);

impl Elicitation for ActionRequest {
    type Style = ActionRequestStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting accesskit::ActionRequest");
        let action = Action::elicit(communicator).await?;
        let uuid = Uuid::elicit(communicator).await?;
        let target_tree = TreeId(uuid);
        let target_node = NodeId::elicit(communicator).await?;
        let data = <Option<ActionData>>::elicit(communicator).await?;
        Ok(Self {
            action,
            target_tree,
            target_node,
            data,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <Action as Elicitation>::kani_proof();
        ts.extend(<Uuid as Elicitation>::kani_proof());
        ts.extend(<NodeId as Elicitation>::kani_proof());
        ts.extend(<Option<ActionData> as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <Action as Elicitation>::verus_proof();
        ts.extend(<Uuid as Elicitation>::verus_proof());
        ts.extend(<NodeId as Elicitation>::verus_proof());
        ts.extend(<Option<ActionData> as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <Action as Elicitation>::creusot_proof();
        ts.extend(<Uuid as Elicitation>::creusot_proof());
        ts.extend(<NodeId as Elicitation>::creusot_proof());
        ts.extend(<Option<ActionData> as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for ActionRequest {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "accesskit::ActionRequest",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "action",
                        type_name: "accesskit::Action",
                        prompt: Some("Action to perform:"),
                    },
                    FieldInfo {
                        name: "target_tree",
                        type_name: "accesskit::TreeId",
                        prompt: Some("UUID of the target tree:"),
                    },
                    FieldInfo {
                        name: "target_node",
                        type_name: "accesskit::NodeId",
                        prompt: Some("Target node ID:"),
                    },
                    FieldInfo {
                        name: "data",
                        type_name: "Option<accesskit::ActionData>",
                        prompt: Some("Optional action payload:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for ActionRequest {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "accesskit::ActionRequest".to_string(),
            fields: vec![
                ("action".to_string(), Box::new(Action::prompt_tree())),
                ("target_tree".to_string(), Box::new(Uuid::prompt_tree())),
                ("target_node".to_string(), Box::new(NodeId::prompt_tree())),
                (
                    "data".to_string(),
                    Box::new(<Option<ActionData>>::prompt_tree()),
                ),
            ],
        }
    }
}

impl ToCodeLiteral for ActionRequest {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let action_lit = self.action.to_code_literal();
        let uuid_str = self.target_tree.0.to_string();
        let node_lit = self.target_node.to_code_literal();
        quote::quote! {
            accesskit::ActionRequest {
                action: #action_lit,
                target_tree: accesskit::TreeId(uuid::Uuid::parse_str(#uuid_str).unwrap()),
                target_node: #node_lit,
                data: None,
            }
        }
    }
}
