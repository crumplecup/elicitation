//! Leptos component, view, route, and app descriptor types.
//!
//! Available with the `leptos-types` feature.

use elicitation_derive::ToCodeLiteral;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::LeptosMode;

/// Describes a single prop on a Leptos component.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct LeptosPropDescriptor {
    /// Prop name (Rust identifier).
    pub name: String,
    /// Prop type as a Rust type string.
    pub ty: String,
    /// Whether the prop is optional (`Option<T>`).
    pub optional: bool,
    /// Default value expression, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// Whether to apply `#[prop(into)]`.
    pub into: bool,
}

/// Describes a Leptos `#[component]` function.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct LeptosComponentDescriptor {
    /// Component name (PascalCase).
    pub name: String,
    /// Props list.
    pub props: Vec<LeptosPropDescriptor>,
    /// Whether this component accepts children.
    pub has_children: bool,
    /// Whether this is a `#[island]` component.
    pub island: bool,
    /// The body of the component function (view! macro body or raw code).
    pub body: String,
}

impl LeptosComponentDescriptor {
    /// Create a new component descriptor with the given name and defaults.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            props: vec![],
            has_children: false,
            island: false,
            body: String::new(),
        }
    }
}

/// A node in a Leptos view tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LeptosViewNode {
    /// Element tag or "text" for text nodes.
    pub tag: String,
    /// Static HTML attributes as (name, value) pairs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attrs: Vec<(String, String)>,
    /// Event handlers as (event_name, handler_body) pairs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub on_events: Vec<(String, String)>,
    /// Child nodes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<LeptosViewNode>,
    /// Text content (for text nodes).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Reactive expression (e.g. `{move || count.get()}`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactive_expr: Option<String>,
}

impl LeptosViewNode {
    /// Create an element node with the given tag.
    pub fn element(tag: impl Into<String>) -> Self {
        Self {
            tag: tag.into(),
            attrs: vec![],
            on_events: vec![],
            children: vec![],
            text: None,
            reactive_expr: None,
        }
    }

    /// Create a static text node.
    pub fn text_node(text: impl Into<String>) -> Self {
        Self {
            tag: "text".to_string(),
            attrs: vec![],
            on_events: vec![],
            children: vec![],
            text: Some(text.into()),
            reactive_expr: None,
        }
    }
}

/// Describes a Leptos route.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LeptosRouteDescriptor {
    /// URL path pattern (e.g. `"/about"` or `"/post/:id"`).
    pub path: String,
    /// View component name rendered at this route.
    pub view: String,
    /// Nested routes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nested: Vec<LeptosRouteDescriptor>,
}

/// Top-level application descriptor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct LeptosAppDescriptor {
    /// Cargo package name (snake_case).
    pub package_name: String,
    /// Rendering mode.
    pub mode: LeptosMode,
    /// Components in this application.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<LeptosComponentDescriptor>,
    /// Top-level routes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub routes: Vec<LeptosRouteDescriptor>,
}

impl LeptosAppDescriptor {
    /// Create a new app descriptor with the given package name and rendering mode.
    pub fn new(package_name: impl Into<String>, mode: LeptosMode) -> Self {
        Self {
            package_name: package_name.into(),
            mode,
            components: vec![],
            routes: vec![],
        }
    }
}

// ============================================================================
// Elicitation traits
// ============================================================================

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    FieldInfo, PatternDetails, Prompt, TypeMetadata,
};

// --- LeptosPropDescriptor ---------------------------------------------------

impl Prompt for LeptosPropDescriptor {
    fn prompt() -> Option<&'static str> { Some("Describe a Leptos component prop:") }
}
crate::default_style!(LeptosPropDescriptor => LeptosPropDescriptorStyle);
impl Elicitation for LeptosPropDescriptor {
    type Style = LeptosPropDescriptorStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting LeptosPropDescriptor");
        let name = String::elicit(communicator).await?;
        let ty = String::elicit(communicator).await?;
        let optional = bool::elicit(communicator).await?;
        let default_value = Option::<String>::elicit(communicator).await?;
        let into = bool::elicit(communicator).await?;
        Ok(Self { name, ty, optional, default_value, into })
    }
    fn kani_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::kani_proof() }
    fn verus_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::verus_proof() }
    fn creusot_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::creusot_proof() }
}
impl ElicitIntrospect for LeptosPropDescriptor {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Survey }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::LeptosPropDescriptor",
            description: Self::prompt(),
            details: PatternDetails::Survey { fields: vec![
                FieldInfo { name: "name", type_name: "String", prompt: None },
                FieldInfo { name: "ty", type_name: "String", prompt: None },
                FieldInfo { name: "optional", type_name: "bool", prompt: None },
                FieldInfo { name: "default_value", type_name: "Option<String>", prompt: None },
                FieldInfo { name: "into", type_name: "bool", prompt: None },
            ]},
        }
    }
}
impl crate::ElicitPromptTree for LeptosPropDescriptor {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "LeptosPropDescriptor".to_string(),
            fields: vec![
                ("name".to_string(), Box::new(String::prompt_tree())),
                ("ty".to_string(), Box::new(String::prompt_tree())),
                ("optional".to_string(), Box::new(bool::prompt_tree())),
                ("default_value".to_string(), Box::new(String::prompt_tree())),
                ("into".to_string(), Box::new(bool::prompt_tree())),
            ],
        }
    }
}

// --- LeptosComponentDescriptor ----------------------------------------------

impl Prompt for LeptosComponentDescriptor {
    fn prompt() -> Option<&'static str> { Some("Describe a Leptos component:") }
}
crate::default_style!(LeptosComponentDescriptor => LeptosComponentDescriptorStyle);
impl Elicitation for LeptosComponentDescriptor {
    type Style = LeptosComponentDescriptorStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting LeptosComponentDescriptor");
        let name = String::elicit(communicator).await?;
        let props = Vec::<LeptosPropDescriptor>::elicit(communicator).await?;
        let has_children = bool::elicit(communicator).await?;
        let island = bool::elicit(communicator).await?;
        let body = String::elicit(communicator).await?;
        Ok(Self { name, props, has_children, island, body })
    }
    fn kani_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::kani_proof() }
    fn verus_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::verus_proof() }
    fn creusot_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::creusot_proof() }
}
impl ElicitIntrospect for LeptosComponentDescriptor {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Survey }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::LeptosComponentDescriptor",
            description: Self::prompt(),
            details: PatternDetails::Survey { fields: vec![
                FieldInfo { name: "name", type_name: "String", prompt: None },
                FieldInfo { name: "props", type_name: "Vec<LeptosPropDescriptor>", prompt: None },
                FieldInfo { name: "has_children", type_name: "bool", prompt: None },
                FieldInfo { name: "island", type_name: "bool", prompt: None },
                FieldInfo { name: "body", type_name: "String", prompt: None },
            ]},
        }
    }
}
impl crate::ElicitPromptTree for LeptosComponentDescriptor {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "LeptosComponentDescriptor".to_string(),
            fields: vec![
                ("name".to_string(), Box::new(String::prompt_tree())),
                ("props".to_string(), Box::new(LeptosPropDescriptor::prompt_tree())),
                ("has_children".to_string(), Box::new(bool::prompt_tree())),
                ("island".to_string(), Box::new(bool::prompt_tree())),
                ("body".to_string(), Box::new(String::prompt_tree())),
            ],
        }
    }
}

// --- LeptosViewNode (recursive) ---------------------------------------------

impl Prompt for LeptosViewNode {
    fn prompt() -> Option<&'static str> { Some("Describe a Leptos view node:") }
}
crate::default_style!(LeptosViewNode => LeptosViewNodeStyle);
impl Elicitation for LeptosViewNode {
    type Style = LeptosViewNodeStyle;
    fn elicit<C: ElicitCommunicator>(
        communicator: &C,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ElicitResult<Self>> + Send + '_>> {
        Box::pin(async move {
            tracing::debug!("Eliciting LeptosViewNode");
            let tag = String::elicit(communicator).await?;
            let text = Option::<String>::elicit(communicator).await?;
            let reactive_expr = Option::<String>::elicit(communicator).await?;
            Ok(Self { tag, attrs: vec![], on_events: vec![], children: vec![], text, reactive_expr })
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::kani_proof() }
    fn verus_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::verus_proof() }
    fn creusot_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::creusot_proof() }
}
impl ElicitIntrospect for LeptosViewNode {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Survey }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::LeptosViewNode",
            description: Self::prompt(),
            details: PatternDetails::Survey { fields: vec![
                FieldInfo { name: "tag", type_name: "String", prompt: None },
                FieldInfo { name: "text", type_name: "Option<String>", prompt: None },
                FieldInfo { name: "reactive_expr", type_name: "Option<String>", prompt: None },
            ]},
        }
    }
}
impl crate::ElicitPromptTree for LeptosViewNode {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "LeptosViewNode".to_string(),
            fields: vec![
                ("tag".to_string(), Box::new(String::prompt_tree())),
                ("text".to_string(), Box::new(String::prompt_tree())),
                ("reactive_expr".to_string(), Box::new(String::prompt_tree())),
            ],
        }
    }
}
impl crate::emit_code::ToCodeLiteral for LeptosViewNode {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let tag = self.tag.to_code_literal();
        quote::quote! { elicitation::LeptosViewNode::element(#tag) }
    }
}

// --- LeptosRouteDescriptor (recursive) --------------------------------------

impl Prompt for LeptosRouteDescriptor {
    fn prompt() -> Option<&'static str> { Some("Describe a Leptos route:") }
}
crate::default_style!(LeptosRouteDescriptor => LeptosRouteDescriptorStyle);
impl Elicitation for LeptosRouteDescriptor {
    type Style = LeptosRouteDescriptorStyle;
    fn elicit<C: ElicitCommunicator>(
        communicator: &C,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ElicitResult<Self>> + Send + '_>> {
        Box::pin(async move {
            tracing::debug!("Eliciting LeptosRouteDescriptor");
            let path = String::elicit(communicator).await?;
            let view = String::elicit(communicator).await?;
            Ok(Self { path, view, nested: vec![] })
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::kani_proof() }
    fn verus_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::verus_proof() }
    fn creusot_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::creusot_proof() }
}
impl ElicitIntrospect for LeptosRouteDescriptor {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Survey }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::LeptosRouteDescriptor",
            description: Self::prompt(),
            details: PatternDetails::Survey { fields: vec![
                FieldInfo { name: "path", type_name: "String", prompt: None },
                FieldInfo { name: "view", type_name: "String", prompt: None },
            ]},
        }
    }
}
impl crate::ElicitPromptTree for LeptosRouteDescriptor {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "LeptosRouteDescriptor".to_string(),
            fields: vec![
                ("path".to_string(), Box::new(String::prompt_tree())),
                ("view".to_string(), Box::new(String::prompt_tree())),
            ],
        }
    }
}
impl crate::emit_code::ToCodeLiteral for LeptosRouteDescriptor {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let path = self.path.to_code_literal();
        let view = self.view.to_code_literal();
        quote::quote! {
            elicitation::LeptosRouteDescriptor { path: #path, view: #view, nested: vec![] }
        }
    }
}

// --- LeptosAppDescriptor ----------------------------------------------------

impl Prompt for LeptosAppDescriptor {
    fn prompt() -> Option<&'static str> { Some("Describe the top-level Leptos application:") }
}
crate::default_style!(LeptosAppDescriptor => LeptosAppDescriptorStyle);
impl Elicitation for LeptosAppDescriptor {
    type Style = LeptosAppDescriptorStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting LeptosAppDescriptor");
        let package_name = String::elicit(communicator).await?;
        let mode = crate::LeptosMode::elicit(communicator).await?;
        let components = Vec::<LeptosComponentDescriptor>::elicit(communicator).await?;
        let routes = Vec::<LeptosRouteDescriptor>::elicit(communicator).await?;
        Ok(Self { package_name, mode, components, routes })
    }
    fn kani_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::kani_proof() }
    fn verus_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::verus_proof() }
    fn creusot_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::creusot_proof() }
}
impl ElicitIntrospect for LeptosAppDescriptor {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Survey }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::LeptosAppDescriptor",
            description: Self::prompt(),
            details: PatternDetails::Survey { fields: vec![
                FieldInfo { name: "package_name", type_name: "String", prompt: None },
                FieldInfo { name: "mode", type_name: "LeptosMode", prompt: None },
                FieldInfo { name: "components", type_name: "Vec<LeptosComponentDescriptor>", prompt: None },
                FieldInfo { name: "routes", type_name: "Vec<LeptosRouteDescriptor>", prompt: None },
            ]},
        }
    }
}
impl crate::ElicitPromptTree for LeptosAppDescriptor {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "LeptosAppDescriptor".to_string(),
            fields: vec![
                ("package_name".to_string(), Box::new(String::prompt_tree())),
                ("mode".to_string(), Box::new(crate::LeptosMode::prompt_tree())),
                ("components".to_string(), Box::new(LeptosComponentDescriptor::prompt_tree())),
                ("routes".to_string(), Box::new(LeptosRouteDescriptor::prompt_tree())),
            ],
        }
    }
}
