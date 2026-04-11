//! Leptos component, view, route, and app descriptor types.
//!
//! Available with the `leptos-types` feature.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::LeptosMode;

/// Describes a single prop on a Leptos component.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
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
