//! Declarative macro for wrapping foreign [`Select`](crate::Select) enums.
//!
//! Foreign enum types often lack `schemars::JsonSchema` (and sometimes
//! `serde::Serialize` / `serde::Deserialize`), which prevents them from
//! satisfying [`ElicitComplete`](crate::ElicitComplete) and from being
//! registered as MCP tool parameters.
//!
//! The **trenchcoat pattern** solves this: we create a local newtype wrapper
//! that derives the missing traits, delegates elicitation to the inner type,
//! and unwraps back to `T` when the agent is done.
//!
//! ```text
//! Agent → select_trenchcoat wrapper → validate → unwrap → foreign T
//! ```
//!
//! # Variants
//!
//! | Syntax | When to use |
//! |--------|------------|
//! | `select_trenchcoat!(T, as Name, serde)` | Inner type has `Serialize + Deserialize` but no `JsonSchema` |
//! | `select_trenchcoat!(T, as Name)` | Inner type has neither serde nor `JsonSchema` |
//!
//! # Example
//!
//! ```rust,ignore
//! use elicitation::select_trenchcoat;
//!
//! // egui::Align has serde but no JsonSchema
//! select_trenchcoat!(egui::Align, as AlignSelect, serde);
//!
//! // egui::UiKind has neither serde nor JsonSchema
//! select_trenchcoat!(egui::UiKind, as UiKindSelect);
//!
//! // The wrapper satisfies ElicitComplete and can be unwrapped:
//! let wrapper: AlignSelect = AlignSelect::from(egui::Align::Center);
//! let inner: egui::Align = wrapper.into_inner();
//! ```

/// Generates a Select-aware newtype wrapper for a foreign enum.
///
/// The wrapper implements `Serialize`, `Deserialize`, and `JsonSchema`
/// so the type can satisfy [`ElicitComplete`](crate::ElicitComplete)
/// and register with MCP tools.
///
/// The `JsonSchema` implementation emits a `{"type": "string", "enum": [...]}`
/// schema derived from `<T as Select>::labels()`, giving agents full visibility
/// into the available choices.
///
/// # Generated code
///
/// - Newtype struct wrapping `T` with `Debug`, `Clone`, `Copy` (when inner is `Copy`)
/// - `JsonSchema` impl with string-enum schema from `Select::labels()`
/// - `Serialize` / `Deserialize` (transparent or manual depending on variant)
/// - `PartialEq`, `Eq`, `Hash` (delegated to inner)
/// - `From<T>`, `into_inner()`, `Deref`, `AsRef`
/// - `Prompt` and `Select` forwarded to inner type
///
/// # Variants
///
/// | Syntax | `Serialize`/`Deserialize` | `JsonSchema` |
/// |--------|--------------------------|-------------|
/// | `select_trenchcoat!(T, as Name)` | Manual via `Select::labels()`/`from_label()` | String-enum from labels |
/// | `select_trenchcoat!(T, as Name, serde)` | Transparent delegation to `T` | String-enum from labels |
#[macro_export]
macro_rules! select_trenchcoat {
    // ── Variant WITHOUT inner serde ──────────────────────────────────────
    //
    // For foreign Select enums that lack both serde and JsonSchema.
    // Serialize/Deserialize use Select::labels()/from_label() for string
    // round-tripping.
    ($inner_path:path, as $wrapper_name:ident) => {
        $crate::_select_trenchcoat_common!($inner_path, $wrapper_name);
        $crate::_select_trenchcoat_manual_serde!($inner_path, $wrapper_name);
    };

    // ── Variant WITH inner serde ─────────────────────────────────────────
    //
    // For foreign Select enums that have Serialize + Deserialize but lack
    // JsonSchema. Serde is delegated transparently to the inner type.
    ($inner_path:path, as $wrapper_name:ident, serde) => {
        $crate::_select_trenchcoat_common!($inner_path, $wrapper_name);
        $crate::_select_trenchcoat_transparent_serde!($inner_path, $wrapper_name);
    };
}

/// Common code shared by both `select_trenchcoat!` variants.
///
/// Generates the struct, JsonSchema, conversions, Deref, Prompt, and Select.
///
/// The wrapper derives `Debug` and `Clone` unconditionally. Additional traits
/// (`Copy`, `PartialEq`, `Eq`, `Hash`) depend on the inner type — use
/// `select_trenchcoat_traits!` to opt in to what the inner type supports.
#[doc(hidden)]
#[macro_export]
macro_rules! _select_trenchcoat_common {
    ($inner_path:path, $wrapper_name:ident) => {
        #[doc = concat!(
            "Select-trenchcoat wrapper around [`",
            stringify!($inner_path),
            "`].\n\n",
            "Provides `Serialize`, `Deserialize`, and `JsonSchema` so the\n",
            "foreign enum can satisfy `ElicitComplete` and register with MCP tools.\n\n",
            "Use `into_inner()` to unwrap back to the original type."
        )]
        #[derive(Debug, Clone)]
        pub struct $wrapper_name($inner_path);

        // ── JsonSchema: string enum from Select::labels() ───────────────
        impl ::schemars::JsonSchema for $wrapper_name {
            fn schema_name() -> ::std::borrow::Cow<'static, str> {
                stringify!($wrapper_name).into()
            }

            fn json_schema(
                _gen: &mut ::schemars::SchemaGenerator,
            ) -> ::schemars::Schema {
                let labels = <$inner_path as $crate::Select>::labels();
                let enum_values: Vec<::serde_json::Value> = labels
                    .into_iter()
                    .map(::serde_json::Value::String)
                    .collect();
                ::serde_json::from_value(::serde_json::json!({
                    "type": "string",
                    "enum": enum_values,
                    "description": <$inner_path as $crate::Prompt>::prompt()
                        .unwrap_or(concat!("Select a ", stringify!($inner_path), " variant"))
                }))
                .expect("valid JSON Schema")
            }
        }

        // ── Conversions ─────────────────────────────────────────────────
        impl ::std::convert::From<$inner_path> for $wrapper_name {
            fn from(inner: $inner_path) -> Self {
                Self(inner)
            }
        }

        impl $wrapper_name {
            /// Unwrap to the inner foreign type.
            pub fn into_inner(self) -> $inner_path {
                self.0
            }
        }

        impl ::std::ops::Deref for $wrapper_name {
            type Target = $inner_path;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::std::convert::AsRef<$inner_path> for $wrapper_name {
            fn as_ref(&self) -> &$inner_path {
                &self.0
            }
        }

        // ── Forward Prompt + Select ─────────────────────────────────────
        impl $crate::Prompt for $wrapper_name {
            fn prompt() -> ::std::option::Option<&'static str> {
                <$inner_path as $crate::Prompt>::prompt()
            }
        }

        impl $crate::Select for $wrapper_name {
            fn options() -> ::std::vec::Vec<Self> {
                <$inner_path as $crate::Select>::options()
                    .into_iter()
                    .map(Self::from)
                    .collect()
            }

            fn labels() -> ::std::vec::Vec<::std::string::String> {
                <$inner_path as $crate::Select>::labels()
            }

            fn from_label(label: &str) -> ::std::option::Option<Self> {
                <$inner_path as $crate::Select>::from_label(label).map(Self::from)
            }
        }

        // ── Forward Elicitation (delegate to inner, wrap result) ────────
        impl $crate::Elicitation for $wrapper_name {
            type Style = <$inner_path as $crate::Elicitation>::Style;

            #[tracing::instrument(skip(communicator))]
            async fn elicit<C: $crate::ElicitCommunicator>(
                communicator: &C,
            ) -> $crate::ElicitResult<Self> {
                <$inner_path as $crate::Elicitation>::elicit(communicator)
                    .await
                    .map(Self::from)
            }

            fn kani_proof() -> proc_macro2::TokenStream {
                <$inner_path as $crate::Elicitation>::kani_proof()
            }

            fn verus_proof() -> proc_macro2::TokenStream {
                <$inner_path as $crate::Elicitation>::verus_proof()
            }

            fn creusot_proof() -> proc_macro2::TokenStream {
                <$inner_path as $crate::Elicitation>::creusot_proof()
            }
        }

        // ── Forward ElicitIntrospect ────────────────────────────────────
        impl $crate::ElicitIntrospect for $wrapper_name {
            fn pattern() -> $crate::ElicitationPattern {
                <$inner_path as $crate::ElicitIntrospect>::pattern()
            }

            fn metadata() -> $crate::TypeMetadata {
                <$inner_path as $crate::ElicitIntrospect>::metadata()
            }
        }

        // ── Forward ElicitPromptTree ──────────────────────────────────────
        #[cfg(feature = "prompt-tree")]
        impl $crate::ElicitPromptTree for $wrapper_name {
            fn prompt_tree() -> $crate::PromptTree {
                let labels = <Self as $crate::Select>::labels();
                let branch_count = labels.len();
                $crate::PromptTree::Select {
                    prompt: <Self as $crate::Prompt>::prompt()
                        .unwrap_or(concat!("Select a ", stringify!($wrapper_name), " variant"))
                        .to_string(),
                    type_name: stringify!($wrapper_name).to_string(),
                    options: labels,
                    branches: vec![None; branch_count],
                }
            }
        }

        // ── ToCodeLiteral: emit `WrapperName::from_label("variant").unwrap()` ──
        impl $crate::emit_code::ToCodeLiteral for $wrapper_name {
            fn to_code_literal(&self) -> $crate::proc_macro2::TokenStream {
                // Use Select labels zipped with options to find the matching label
                // by comparing debug representations (inner type may not impl PartialEq)
                let inner_debug = format!("{:?}", self.0);
                let label = <$inner_path as $crate::Select>::labels()
                    .into_iter()
                    .zip(<$inner_path as $crate::Select>::options().iter())
                    .find(|(_, opt)| format!("{:?}", opt) == inner_debug)
                    .map(|(l, _)| l)
                    .unwrap_or_else(|| inner_debug.clone());
                $crate::quote::quote! {
                    $wrapper_name::from_label(#label).expect("valid label")
                }
            }

            fn type_tokens() -> $crate::proc_macro2::TokenStream {
                $crate::quote::quote! { $wrapper_name }
            }
        }
    };
}

/// Manual serde for foreign types without Serialize/Deserialize.
///
/// Serializes as the label string, deserializes via `Select::from_label()`.
#[doc(hidden)]
#[macro_export]
macro_rules! _select_trenchcoat_manual_serde {
    ($inner_path:path, $wrapper_name:ident) => {
        impl ::serde::Serialize for $wrapper_name {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                // Find the label for our current value by matching against options
                let options = <$inner_path as $crate::Select>::options();
                let labels = <$inner_path as $crate::Select>::labels();
                let label = options
                    .iter()
                    .zip(labels.iter())
                    .find(|(opt, _)| *opt == &self.0)
                    .map(|(_, label)| label.as_str())
                    .unwrap_or("Unknown");
                serializer.serialize_str(label)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $wrapper_name {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let label =
                    <::std::string::String as ::serde::Deserialize>::deserialize(deserializer)?;
                <$inner_path as $crate::Select>::from_label(&label)
                    .map(Self::from)
                    .ok_or_else(|| {
                        ::serde::de::Error::custom(::std::format!(
                            "invalid {} label: {}",
                            stringify!($inner_path),
                            label
                        ))
                    })
            }
        }
    };
}

/// Transparent serde for foreign types that already have Serialize/Deserialize.
#[doc(hidden)]
#[macro_export]
macro_rules! _select_trenchcoat_transparent_serde {
    ($inner_path:path, $wrapper_name:ident) => {
        impl ::serde::Serialize for $wrapper_name {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                <$inner_path as ::serde::Serialize>::serialize(&self.0, serializer)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $wrapper_name {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                <$inner_path as ::serde::Deserialize>::deserialize(deserializer).map(Self::from)
            }
        }
    };
}

/// Forwards standard library traits from the inner type to a `select_trenchcoat!` wrapper.
///
/// Similar to [`elicit_newtype_traits!`](crate::elicit_newtype_traits) but for
/// direct (non-Arc) wrappers.
///
/// # Flags
///
/// | Flag | Traits generated |
/// |------|-----------------|
/// | `copy` | `Copy` |
/// | `eq` | `PartialEq + Eq` |
/// | `hash` | `Hash` |
///
/// # Example
///
/// ```rust,ignore
/// select_trenchcoat!(egui::Align, as AlignSelect, serde);
/// select_trenchcoat_traits!(AlignSelect, egui::Align, [copy, eq, hash]);
/// ```
#[macro_export]
macro_rules! select_trenchcoat_traits {
    // Base case: empty list
    ($name:ident, $inner:path, []) => {};

    // Peel one flag and recurse
    ($name:ident, $inner:path, [$flag:ident $(, $rest:ident)*]) => {
        $crate::_select_trenchcoat_trait_flag!($name, $inner, $flag);
        $crate::select_trenchcoat_traits!($name, $inner, [$($rest),*]);
    };
}

/// Generates one trait impl for a select_trenchcoat wrapper.
#[doc(hidden)]
#[macro_export]
macro_rules! _select_trenchcoat_trait_flag {
    ($name:ident, $inner:path, copy) => {
        impl ::std::marker::Copy for $name {}
    };

    ($name:ident, $inner:path, eq) => {
        impl ::std::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl ::std::cmp::Eq for $name {}
    };

    ($name:ident, $inner:path, hash) => {
        impl ::std::hash::Hash for $name {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }
    };
}
