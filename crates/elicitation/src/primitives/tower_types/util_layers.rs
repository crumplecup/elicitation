//! Closure/fn-based tower util layer descriptors.
//!
//! Each type is a serializable mirror storing the function/closure as a Rust
//! identifier string.  No `From` impl is provided because closures cannot be
//! reconstructed from a string at runtime.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

// ── Macro ────────────────────────────────────────────────────────────────────

macro_rules! tower_fn_layer {
    (
        $name:ident,
        type_name  = $type_name:literal,
        field      = $field:ident,
        prompt     = $prompt:literal,
        field_prompt = $field_prompt:literal $(,)?
    ) => {
        #[doc = concat!("Serializable descriptor for [`", $type_name, "`].")]
        #[derive(
            Debug,
            Clone,
            PartialEq,
            serde::Serialize,
            serde::Deserialize,
            schemars::JsonSchema,
            elicitation_derive::ToCodeLiteral,
        )]
        pub struct $name {
            /// Rust expression for the mapping/transform fn (closure or named fn).
            pub $field: String,
        }

        paste::paste! {
            crate::default_style!($name => [<$name Style>]);
        }

        impl Prompt for $name {
            fn prompt() -> Option<&'static str> {
                Some($prompt)
            }
        }

        impl Elicitation for $name {
            type Style = paste::paste! { [<$name Style>] };

            #[tracing::instrument(skip(communicator))]
            async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                tracing::debug!(concat!("Eliciting ", stringify!($name)));
                let $field = String::elicit(communicator).await?;
                Ok(Self { $field })
            }

            fn kani_proof() -> proc_macro2::TokenStream {
                <String as Elicitation>::kani_proof()
            }

            fn verus_proof() -> proc_macro2::TokenStream {
                <String as Elicitation>::verus_proof()
            }

            fn creusot_proof() -> proc_macro2::TokenStream {
                <String as Elicitation>::creusot_proof()
            }
        }

        impl ElicitIntrospect for $name {
            fn pattern() -> ElicitationPattern {
                ElicitationPattern::Survey
            }

            fn metadata() -> TypeMetadata {
                TypeMetadata {
                    type_name: $type_name,
                    description: Self::prompt(),
                    details: PatternDetails::Survey {
                        fields: vec![FieldInfo {
                            name: stringify!($field),
                            type_name: "String",
                            prompt: Some($field_prompt),
                        }],
                    },
                }
            }
        }

        impl crate::ElicitPromptTree for $name {
            fn prompt_tree() -> crate::PromptTree {
                crate::PromptTree::Survey {
                    prompt: Self::prompt().map(str::to_string),
                    type_name: stringify!($name).to_string(),
                    fields: vec![(
                        stringify!($field).to_string(),
                        Box::new(String::prompt_tree()),
                    )],
                }
            }
        }
    };
}

// ── Single-fn-field types ────────────────────────────────────────────────────

tower_fn_layer!(
    TowerMapErrLayer,
    type_name = "tower::util::MapErrLayer<F>",
    field = mapper_fn,
    prompt = "Configure MapErr layer (error mapper function):",
    field_prompt = "Rust expression: closure or fn name (e.g. `|e| e.to_string()`):",
);

tower_fn_layer!(
    TowerMapRequestLayer,
    type_name = "tower::util::MapRequestLayer<F>",
    field = mapper_fn,
    prompt = "Configure MapRequest layer (request mapper function):",
    field_prompt = "Rust expression: closure or fn name (e.g. `|req| req.with_header(...)`):",
);

tower_fn_layer!(
    TowerMapResponseLayer,
    type_name = "tower::util::MapResponseLayer<F>",
    field = mapper_fn,
    prompt = "Configure MapResponse layer (response mapper function):",
    field_prompt = "Rust expression: closure or fn name (e.g. `|res| res.map(|b| b.collect())`):",
);

tower_fn_layer!(
    TowerMapResultLayer,
    type_name = "tower::util::MapResultLayer<F>",
    field = mapper_fn,
    prompt = "Configure MapResult layer (result mapper function):",
    field_prompt = "Rust expression: closure or fn name (e.g. `|r| r.map_err(Into::into)`):",
);

tower_fn_layer!(
    TowerAndThenLayer,
    type_name = "tower::util::AndThenLayer<F>",
    field = f,
    prompt = "Configure AndThen layer (async combinator on Ok responses):",
    field_prompt =
        "Rust expression: async closure or fn (e.g. `|res| async move { process(res).await }`):",
);

tower_fn_layer!(
    TowerThenLayer,
    type_name = "tower::util::ThenLayer<F>",
    field = f,
    prompt = "Configure Then layer (async combinator on all responses):",
    field_prompt =
        "Rust expression: async closure or fn (e.g. `|r| async move { transform(r).await }`):",
);

// ── Three-field BoxService configs ───────────────────────────────────────────

/// Factory config for `tower::util::BoxService<Req, Resp, Err>`.
///
/// Stores the three type-parameter names so code-recovery tooling can
/// reconstruct the type annotation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerBoxServiceConfig {
    /// Rust type expression for the request type `Req`.
    pub req_type: String,
    /// Rust type expression for the response type `Resp`.
    pub resp_type: String,
    /// Rust type expression for the error type `Err`.
    pub err_type: String,
}

crate::default_style!(TowerBoxServiceConfig => TowerBoxServiceConfigStyle);

impl Prompt for TowerBoxServiceConfig {
    fn prompt() -> Option<&'static str> {
        Some("Configure BoxService type parameters:")
    }
}

impl Elicitation for TowerBoxServiceConfig {
    type Style = TowerBoxServiceConfigStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerBoxServiceConfig");
        let req_type = String::elicit(communicator).await?;
        let resp_type = String::elicit(communicator).await?;
        let err_type = String::elicit(communicator).await?;
        Ok(Self {
            req_type,
            resp_type,
            err_type,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <String as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <String as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <String as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for TowerBoxServiceConfig {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower::util::BoxService<Req,Resp,Err>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "req_type",
                        type_name: "String",
                        prompt: Some("Request type (Rust expression):"),
                    },
                    FieldInfo {
                        name: "resp_type",
                        type_name: "String",
                        prompt: Some("Response type (Rust expression):"),
                    },
                    FieldInfo {
                        name: "err_type",
                        type_name: "String",
                        prompt: Some("Error type (Rust expression):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerBoxServiceConfig {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerBoxServiceConfig".to_string(),
            fields: vec![
                ("req_type".to_string(), Box::new(String::prompt_tree())),
                ("resp_type".to_string(), Box::new(String::prompt_tree())),
                ("err_type".to_string(), Box::new(String::prompt_tree())),
            ],
        }
    }
}

/// Factory config for `tower::util::BoxCloneService<Req, Resp, Err>`.
///
/// Stores the three type-parameter names so code-recovery tooling can
/// reconstruct the type annotation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerBoxCloneServiceConfig {
    /// Rust type expression for the request type `Req`.
    pub req_type: String,
    /// Rust type expression for the response type `Resp`.
    pub resp_type: String,
    /// Rust type expression for the error type `Err`.
    pub err_type: String,
}

crate::default_style!(TowerBoxCloneServiceConfig => TowerBoxCloneServiceConfigStyle);

impl Prompt for TowerBoxCloneServiceConfig {
    fn prompt() -> Option<&'static str> {
        Some("Configure BoxCloneService type parameters:")
    }
}

impl Elicitation for TowerBoxCloneServiceConfig {
    type Style = TowerBoxCloneServiceConfigStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerBoxCloneServiceConfig");
        let req_type = String::elicit(communicator).await?;
        let resp_type = String::elicit(communicator).await?;
        let err_type = String::elicit(communicator).await?;
        Ok(Self {
            req_type,
            resp_type,
            err_type,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <String as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <String as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <String as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for TowerBoxCloneServiceConfig {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower::util::BoxCloneService<Req,Resp,Err>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "req_type",
                        type_name: "String",
                        prompt: Some("Request type (Rust expression):"),
                    },
                    FieldInfo {
                        name: "resp_type",
                        type_name: "String",
                        prompt: Some("Response type (Rust expression):"),
                    },
                    FieldInfo {
                        name: "err_type",
                        type_name: "String",
                        prompt: Some("Error type (Rust expression):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerBoxCloneServiceConfig {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerBoxCloneServiceConfig".to_string(),
            fields: vec![
                ("req_type".to_string(), Box::new(String::prompt_tree())),
                ("resp_type".to_string(), Box::new(String::prompt_tree())),
                ("err_type".to_string(), Box::new(String::prompt_tree())),
            ],
        }
    }
}
