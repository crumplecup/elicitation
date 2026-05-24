//! Trenchcoat wrapper for [`redb::TypeName`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A globally unique type identifier used by redb to name key and value types.
///
/// Wraps `redb::TypeName` to add [`JsonSchema`] for MCP boundary crossing.
/// Use `TypeName::new("crate_name::MyType")` — prefix with crate name to
/// avoid collisions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct TypeName {
    /// The fully-qualified type name string (e.g. `"my_crate::MyKey"`).
    pub name: String,
}

impl TypeName {
    /// Create a new type name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[cfg(feature = "redb-types")]
impl From<redb::TypeName> for TypeName {
    fn from(t: redb::TypeName) -> Self {
        Self {
            name: t.name().to_string(),
        }
    }
}

#[cfg(feature = "redb-types")]
impl From<TypeName> for redb::TypeName {
    fn from(t: TypeName) -> Self {
        redb::TypeName::new(&t.name)
    }
}

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

impl Prompt for TypeName {
    fn prompt() -> Option<&'static str> {
        Some("Enter a redb type name (e.g. \"my_crate::MyKey\"):")
    }
}

crate::default_style!(TypeName => TypeNameStyle);

impl Elicitation for TypeName {
    type Style = TypeNameStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting RedbTypeName");
        let name = String::elicit(communicator).await?;
        Ok(Self { name })
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

impl ElicitIntrospect for TypeName {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::RedbTypeName",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "name",
                    type_name: "String",
                    prompt: Some("Fully-qualified type name:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for TypeName {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "RedbTypeName".to_string(),
            fields: vec![("name".to_string(), Box::new(String::prompt_tree()))],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for TypeName {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        quote::quote! {
            elicitation::RedbTypeName::new(#name)
        }
    }
}
