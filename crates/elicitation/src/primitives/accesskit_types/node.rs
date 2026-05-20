//! Elicitation for [`accesskit::Node`].
//!
//! `Node` has a compact private-field representation; only the role is required
//! at construction time via `Node::new(role)`.  All other properties are set
//! via builder-style setters after construction, so elicitation captures the
//! role and returns a minimal node that callers can further configure.

use accesskit::{Node, Role};

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata, emit_code::ToCodeLiteral,
};

impl Prompt for Node {
    fn prompt() -> Option<&'static str> {
        Some("Choose the ARIA role for this accessibility node:")
    }
}

crate::default_style!(Node => NodeStyle);

impl Elicitation for Node {
    type Style = NodeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting accesskit::Node");
        let role = Role::elicit(communicator).await?;
        Ok(Node::new(role))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <Role as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <Role as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <Role as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for Node {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "accesskit::Node",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "role",
                    type_name: "accesskit::Role",
                    prompt: Some("ARIA role (required):"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for Node {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "accesskit::Node".to_string(),
            fields: vec![("role".to_string(), Box::new(Role::prompt_tree()))],
        }
    }
}

impl ToCodeLiteral for Node {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let role_str = format!("{:?}", self.role());
        let role_ident = proc_macro2::Ident::new(&role_str, proc_macro2::Span::call_site());
        quote::quote! { accesskit::Node::new(accesskit::Role::#role_ident) }
    }
}
