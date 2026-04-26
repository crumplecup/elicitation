//! Derive macros for elicitation patterns.
//!
//! This crate provides the `#[derive(Elicit)]` macro for automatically
//! implementing elicitation traits on enums and structs.
//!
//! # Enum Derivation (Select Pattern)
//!
//! ## Unit Variants (Simple Selection)
//!
//! ```rust,ignore
//! use elicitation::Elicit;
//!
//! #[derive(Elicit)]
//! enum Mode {
//!     Fast,
//!     Safe,
//! }
//! ```
//!
//! User sees: "Fast", "Safe" - single selection.
//!
//! ## Tuple Variants (Select + Field Elicitation)
//!
//! ```rust,ignore
//! use elicitation::Elicit;
//!
//! #[derive(Elicit)]
//! enum MediaSource {
//!     Url(String),
//!     Base64(String),
//! }
//! ```
//!
//! User: 1) Selects "Url" or "Base64", 2) Provides String value.
//!
//! ## Struct Variants (Select + Multi-Field Survey)
//!
//! ```rust,ignore
//! use elicitation::Elicit;
//!
//! #[derive(Elicit)]
//! enum Input {
//!     Image {
//!         mime: Option<String>,
//!         source: MediaSource,
//!     },
//! }
//! ```
//!
//! User: 1) Selects "Image", 2) Provides each field (mime, then source).
//!
//! ## Mixed Variants
//!
//! All three variant types can appear in the same enum:
//!
//! ```rust,ignore
//! use elicitation::Elicit;
//!
//! #[derive(Elicit)]
//! enum Status {
//!     Ok,                                     // Unit variant
//!     Warning(String),                        // Tuple variant
//!     Error { code: i32, msg: String },      // Struct variant
//! }
//! ```
//!
//! # Struct Derivation (Survey Pattern)
//!
//! ```rust,ignore
//! use elicitation::Elicit;
//!
//! // Derives Survey pattern for structs
//! #[derive(Elicit)]
//! struct Config {
//!     #[prompt("Enter timeout in seconds:")]
//!     timeout: u32,
//!     mode: Mode,
//! }
//! ```

#![forbid(unsafe_code)]

extern crate proc_macro;

mod contract_type;
mod derive_elicit;
mod derive_elicit_plugin;
mod derive_prop;
mod derive_to_code_literal;
mod derive_vsm;
mod elicit_tool;
mod emit_rewriter;
mod enum_impl;
mod formal_method;
mod method_reflection;
mod rand_contract_parser;
mod rand_generator_impl;
mod struct_impl;
mod tool_gen;
mod trait_reflection;

use proc_macro::TokenStream;

/// Derive the Elicit trait for enums (→ Select) or structs (→ Survey).
///
/// **Important:** You must also add `#[derive(schemars::JsonSchema)]` to your type.
/// This is required for MCP tool compatibility (the generated `elicit_checked()` function).
///
/// # Example
///
/// ```rust,ignore
/// use elicitation::Elicit;
/// use schemars::JsonSchema;
///
/// #[derive(Debug, Clone, Elicit, JsonSchema)]
/// struct Config {
///     host: String,
///     port: u16,
/// }
/// ```
///
/// # Supported Attributes
///
/// - `#[prompt("...")]` - Custom prompt text (applies to type or fields)
/// - `#[alts([...])]` - Synonym mappings for enum variants (planned for v0.3.0)
/// - `#[skip]` - Skip a struct field during elicitation
///
/// # Enum Derivation
///
/// For enums, generates implementations of:
/// - `Prompt` - Provides prompt text
/// - `Select` - Finite options pattern
/// - `Elicit` - Calls `elicit_select` MCP tool, then elicits fields
///
/// Supports three variant types:
///
/// ## Unit Variants
///
/// ```rust,ignore
/// #[derive(Elicit, JsonSchema)]
/// enum Role {
///     System,
///     User,
///     Assistant,
/// }
/// ```
///
/// ## Tuple Variants
///
/// ```rust,ignore
/// #[derive(Elicit, JsonSchema)]
/// enum MediaSource {
///     Url(String),
///     Base64(String),
///     Binary(Vec<u8>),
/// }
/// ```
///
/// ## Struct Variants
///
/// ```rust,ignore
/// #[derive(Elicit, JsonSchema)]
/// enum Input {
///     Text(String),
///     Image {
///         mime: Option<String>,
///         source: MediaSource,
///     },
/// }
/// ```
///
/// All three variant types can coexist in the same enum.
///
/// # Struct Derivation
///
/// For structs, generates implementations of:
/// - `Prompt` - Provides prompt text
/// - `Survey` - Multi-field elicitation pattern
/// - `Elicit` - Sequential field elicitation
///
/// # Examples
///
/// ```rust,ignore
/// use elicitation::Elicit;
/// use schemars::JsonSchema;
///
/// #[derive(Elicit, JsonSchema)]
/// enum Status {
///     Active,
///     Inactive,
/// }
///
/// #[derive(Elicit, JsonSchema)]
/// #[prompt("Select your favorite color:")]
/// enum Color {
///     Red,
///     Green,
///     Blue,
/// }
/// ```
#[proc_macro_derive(Elicit, attributes(prompt, alts, skip, spec_summary, spec_requires))]
pub fn derive_elicit(input: TokenStream) -> TokenStream {
    derive_elicit::expand(input)
}

/// Annotates a type with verification contract metadata.
///
/// This allows the `#[derive(Elicit)]` macro to extract and compose
/// verification requirements from field types.
///
/// # Attributes
///
/// - `requires = "expr"` - Precondition string (validated at construction)
/// - `ensures = "expr"` - Postcondition string (guaranteed after construction)
///
/// # Example
///
/// ```rust,ignore
/// use elicitation_derive::contract_type;
///
/// #[contract_type(
///     requires = "value > 0",
///     ensures = "result.get() > 0"
/// )]
/// pub struct I8Positive(i8);
///
/// // Generates const fns for metadata:
/// // I8Positive::__contract_requires() -> &'static str
/// // I8Positive::__contract_ensures() -> &'static str
/// ```
///
/// The metadata is queried at compile time by the derive macro when composing
/// struct-level verification contracts.
///
/// # Usage with Derive
///
/// ```rust,ignore
/// use elicitation::{Elicit, verification::types::*};
///
/// #[derive(Elicit)]
/// pub struct User {
///     name: StringNonEmpty,  // has contract metadata
///     age: I8Positive,       // has contract metadata
/// }
///
/// // With --features verify-kani, generates:
/// // #[kani::requires(name.get().len() > 0 && age.get() > 0)]
/// // fn __make_User(name: StringNonEmpty, age: I8Positive) -> User { ... }
/// ```
#[proc_macro_attribute]
pub fn contract_type(args: TokenStream, input: TokenStream) -> TokenStream {
    contract_type::contract_type_impl(args, input)
}

/// Derive macro for contract-aware random generation.
///
/// Generates a `Generator` implementation that respects the type's contract.
///
/// # Example
///
/// ```rust,ignore
/// #[derive(Rand)]
/// #[rand(bounded(1, 100))]
/// struct Score(u32);
/// ```
///
/// Generates:
/// ```rust,ignore
/// impl Score {
///     pub fn random_generator(seed: u64) -> impl Generator<Target = Self> {
///         MapGenerator::new(
///             UniformGenerator::with_seed(seed, 1, 100),
///             |v| Score(v)
///         )
///     }
/// }
/// ```
///
/// # Supported Contracts
///
/// - `#[rand(bounded(L, H))]` - Uniform distribution in [L, H)
/// - `#[rand(positive)]` - Positive values only
/// - `#[rand(nonzero)]` - Non-zero values
/// - `#[rand(even)]` - Even values only
/// - `#[rand(odd)]` - Odd values only
/// - `#[rand(and(...))]` - Composition of contracts
/// - `#[rand(or(...))]` - Alternative contracts
#[proc_macro_derive(Rand, attributes(rand))]
pub fn derive_rand(input: TokenStream) -> TokenStream {
    use syn::parse_macro_input;
    let input = parse_macro_input!(input as syn::DeriveInput);

    rand_generator_impl::expand_derive_rand(&input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Automatically discovers and wraps methods for MCP tool generation.
///
/// This attribute macro enables automatic method reflection on newtype wrappers,
/// generating parameter structs and MCP tool interfaces.
///
/// # Basic Usage
///
/// ```rust,ignore
/// use elicitation::elicit_newtype;
/// use elicitation_derive::reflect_methods;
///
/// // Create newtype wrapper
/// elicit_newtype!(::std::path::PathBuf, as PathBuf);
///
/// // Auto-generate MCP tools for methods
/// #[reflect_methods]
/// impl PathBuf {
///     // Add method signatures to wrap
///     pub fn exists(&self) -> bool { self.0.exists() }
/// }
/// ```
///
/// # What It Generates
///
/// For each public method in the impl block:
/// 1. A parameter struct with `#[derive(Elicit, JsonSchema)]`
/// 2. A wrapper method marked with `#[tool]` for MCP registration
/// 3. Automatic type conversions (`&str` → `String`, `&[T]` → `Vec<T>`)
///
/// # Example Expansion
///
/// Input:
/// ```rust,ignore
/// #[reflect_methods]
/// impl Client {
///     pub async fn get(&self, url: &str) -> Result<Response, Error> {
///         self.0.get(url).await
///     }
/// }
/// ```
///
/// Generates:
/// ```rust,ignore
/// #[derive(Debug, Clone, Elicit, JsonSchema)]
/// pub struct GetParams {
///     pub url: String,  // &str converted to String
/// }
///
/// impl Client {
///     #[tool(description = "Get resource from URL")]
///     pub async fn get(
///         &self,
///         params: Parameters<GetParams>,
///     ) -> Result<Json<Response>, ErrorData> {
///         self.0.get(params.url.as_str())
///             .await
///             .map(Json)
///             .map_err(|e| ErrorData::internal_error(e.to_string(), None))
///     }
/// }
/// ```
///
/// # Configuration Attributes
///
/// ```rust,ignore
/// #[reflect_methods(
///     warn_clone_threshold = 1024,  // Warn for clones > 1KB
///     allow_large_clones = false,   // Show warnings (default)
/// )]
/// impl Client { }
/// ```
///
/// # Type Conversions
///
/// - `&str` → `String` (no warnings)
/// - `&[T]` → `Vec<T>` (warn if large)
/// - `&T` → `T` (warn if large, requires Clone)
///
/// # Limitations
///
/// - Currently requires explicit method signatures in impl block
/// - Automatic discovery of external type methods not yet implemented
/// - Generic methods require JsonSchema bounds (Milestone 3)
#[proc_macro_attribute]
pub fn reflect_methods(attr: TokenStream, item: TokenStream) -> TokenStream {
    method_reflection::expand(attr, item)
}

/// Generate a [`ToolDescriptor`] companion function from an async tool handler.
///
/// Place this attribute on an `async fn` that accepts a typed params struct and
/// returns `Result<CallToolResult, ErrorData>`.  The macro emits the original
/// function unchanged plus a companion `{fn_name}_descriptor() -> ToolDescriptor`
/// constructor, ready for use in a [`DescriptorPlugin`].
///
/// [`ToolDescriptor`]: elicitation::plugin::ToolDescriptor
/// [`DescriptorPlugin`]: elicitation::plugin::DescriptorPlugin
///
/// # Arguments
///
/// - `name = "..."` — bare tool name (no namespace prefix)
/// - `description = "..."` — human-readable description shown to the agent
///
/// # Example
///
/// ```rust,ignore
/// use elicitation_derive::elicit_tool;
/// use rmcp::model::{CallToolResult, Content};
/// use rmcp::ErrorData;
/// use schemars::JsonSchema;
/// use serde::Deserialize;
///
/// #[derive(Deserialize, JsonSchema)]
/// struct PingParams { message: String }
///
/// #[elicit_tool(name = "ping", description = "Echo a message")]
/// async fn ping(p: PingParams) -> Result<CallToolResult, ErrorData> {
///     Ok(CallToolResult::success(vec![Content::text(p.message)]))
/// }
///
/// // Generates: pub fn ping_descriptor() -> elicitation::plugin::ToolDescriptor { ... }
/// ```
///
/// The descriptor can then be returned from a [`DescriptorPlugin::descriptors`] impl:
///
/// ```rust,ignore
/// impl DescriptorPlugin for MyPlugin {
///     fn name(&self) -> &'static str { "my_plugin" }
///     fn descriptors(&self) -> &[ToolDescriptor] { &self.tools }
/// }
/// ```
/// Derive an [`ElicitPlugin`] implementation for a unit struct using inventory.
///
/// Requires `#[plugin(name = "...")]` on the struct.  At runtime, iterates all
/// [`PluginToolRegistration`]s submitted via `#[elicit_tool(plugin = "...")]`
/// and filters by plugin name — eliminating all handwritten `list_tools` and
/// `call_tool` dispatch.
///
/// [`ElicitPlugin`]: elicitation::plugin::ElicitPlugin
/// [`PluginToolRegistration`]: elicitation::plugin::PluginToolRegistration
///
/// # Example
///
/// ```rust,ignore
/// use elicitation_derive::ElicitPlugin;
///
/// #[derive(ElicitPlugin)]
/// #[plugin(name = "my_plugin")]
/// pub struct MyPlugin;
/// ```
///
/// Combined with `#[elicit_tool(plugin = "my_plugin", ...)]` on each handler,
/// the full plugin is:
///
/// ```rust,ignore
/// #[derive(ElicitPlugin)]
/// #[plugin(name = "my_plugin")]
/// pub struct MyPlugin;
///
/// #[elicit_tool(plugin = "my_plugin", name = "ping", description = "Echo")]
/// async fn ping(p: PingParams) -> Result<CallToolResult, ErrorData> { ... }
/// ```
#[proc_macro_derive(ElicitPlugin, attributes(plugin))]
pub fn derive_elicit_plugin(input: TokenStream) -> TokenStream {
    derive_elicit_plugin::expand(input.into()).into()
}

#[proc_macro_attribute]
pub fn elicit_tool(args: TokenStream, item: TokenStream) -> TokenStream {
    elicit_tool::expand(args.into(), item.into()).into()
}

/// Mark a function as a contract-honoring formal method and generate
/// backend verification harnesses.
///
/// # Syntax
///
/// ```rust,ignore
/// use elicitation::formal_method;
///
/// #[formal_method(contracts = [InvariantHolds])]
/// fn advance(state: MyState, proof: Established<InvariantHolds>)
///     -> (MyState, Established<InvariantHolds>)
/// {
///     (state.next(), proof)
/// }
/// ```
///
/// The `contracts = [...]` argument is optional. The macro adds a doc
/// annotation and emits a `#[cfg(kani)] #[kani::proof]` harness.
/// Type enforcement is already provided by the [`FormalMethod`] blanket impl.
///
/// [`FormalMethod`]: elicitation::contracts::FormalMethod
#[proc_macro_attribute]
pub fn formal_method(args: TokenStream, item: TokenStream) -> TokenStream {
    formal_method::expand(args.into(), item.into())
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// Derive the [`Prop`](elicitation::contracts::Prop) trait for a zero-cost typestate marker.
///
/// Generates trivial but non-empty, uniquely-named proof harnesses
/// (`kani_proof`, `verus_proof`, `creusot_proof`) for the proposition.
/// The harness function names are derived from the struct name in `snake_case`,
/// so multiple derived propositions can coexist in the same verification target.
///
/// Use this for unit structs that serve as typestate markers in workflows.
/// For propositions with meaningful semantic content (e.g., `DbConnected` which
/// models a real connection attempt), write a manual `impl Prop` with real harnesses.
///
/// # Example
///
/// ```rust,ignore
/// use elicitation_derive::Prop;
///
/// #[derive(Debug, Clone, Copy, Prop)]
/// pub struct UrlParsed;
///
/// #[derive(Debug, Clone, Copy, Prop)]
/// pub struct HttpsRequired;
/// ```
///
/// The generated Kani harness for `UrlParsed` is equivalent to:
///
/// ```rust,ignore
/// #[kani::proof]
/// fn verify_url_parsed_prop_marker() {
///     let established: bool = true;
///     assert!(established);
/// }
/// ```
#[proc_macro_derive(Prop)]
pub fn derive_prop(input: TokenStream) -> TokenStream {
    derive_prop::expand(input)
}

/// Derive `VerifiedStateMachine` for a unit struct, inferring `State` and
/// `Invariant` from naming conventions and wiring `transition_harnesses()` from
/// a `#[vsm(transitions = [...])]` attribute.
///
/// # Naming conventions
///
/// Given `struct FooBarMachine`, the macro infers:
/// - `type State = FooBarState`
/// - `type Invariant = FooBarConsistent`
///
/// Override either with `#[vsm(state = MyState)]` or `#[vsm(invariant = MyInvariant)]`.
///
/// # Example
///
/// ```rust,ignore
/// #[derive(VerifiedStateMachine)]
/// #[vsm(transitions = [begin_connect_sql, disconnect, reconnect])]
/// pub struct ArchiveConnectionMachine;
/// ```
#[proc_macro_derive(VerifiedStateMachine, attributes(vsm))]
pub fn derive_verified_state_machine(input: TokenStream) -> TokenStream {
    derive_vsm::expand(input)
}

///
/// This generates a trivial impl where `type Proxy = Self`, meaning the type
/// is its own proxy — no conversion needed.  Use this on any type that already
/// satisfies `Serialize + DeserializeOwned + JsonSchema`.
///
/// # Example
///
/// ```rust,ignore
/// use elicitation_derive::ElicitProxy;
///
/// #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit, ElicitProxy)]
/// pub struct MyConfig {
///     pub name: String,
///     pub value: i32,
/// }
/// ```
///
/// The generated code is equivalent to:
///
/// ```rust,ignore
/// impl ::elicitation::ElicitProxy for MyConfig {
///     type Proxy = MyConfig;
///     fn into_proxy(self) -> MyConfig { self }
///     fn from_proxy(proxy: MyConfig) -> MyConfig { proxy }
/// }
/// ```
#[proc_macro_derive(ElicitProxy)]
pub fn derive_elicit_proxy(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote::quote! {
        impl #impl_generics ::elicitation::ElicitProxy for #name #ty_generics
        #where_clause
        {
            type Proxy = #name #ty_generics;
            fn into_proxy(self) -> #name #ty_generics { self }
            fn from_proxy(proxy: #name #ty_generics) -> #name #ty_generics { proxy }
        }
    }
    .into()
}

/// Derive `ToCodeLiteral` for structs and enums.
///
/// Generates a `#[cfg(feature = "emit")]`-gated implementation that
/// converts each field/variant into a `TokenStream` expression
/// reconstructing the value.
///
/// # Structs
///
/// ```rust,ignore
/// #[derive(ToCodeLiteral)]
/// struct ColorJson { r: f32, g: f32, b: f32, a: f32 }
/// ```
///
/// # Enums
///
/// Handles unit, tuple, and struct variants:
///
/// ```rust,ignore
/// #[derive(ToCodeLiteral)]
/// enum Align { Min, Center, Max }
/// ```
///
/// Recursive types work automatically via existing `Vec<T>` / `Option<T>`
/// blanket impls.
#[proc_macro_derive(ToCodeLiteral, attributes(to_code_literal, skip))]
pub fn derive_to_code_literal(input: TokenStream) -> TokenStream {
    derive_to_code_literal::expand(input)
}

/// Attribute macro to add tracing instrumentation to impl blocks.
///
/// Apply to `impl` blocks to automatically instrument all public methods.
///
/// # Strategy
///
/// - **Constructors** (`new`, `from_*`, `try_*`): `#[instrument(ret, err)]`
/// - **Accessors** (`get`, `into_inner`, `as_*`, `to_*`): `#[instrument(level = "trace", ret)]`
/// - **Other methods**: `#[instrument(skip(self))]`
///
/// # Kani Compatibility
///
/// When compiling under Kani (formal verification), this macro becomes a no-op.
/// Instrumentation is for runtime observability, not formal verification.
#[proc_macro_attribute]
pub fn instrumented_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    use syn::{ImplItem, ItemImpl, parse_macro_input};
    use quote::quote;

    let impl_block = parse_macro_input!(item as ItemImpl);

    // Under Kani, return impl block unchanged (no instrumentation needed)
    #[cfg(kani)]
    {
        return TokenStream::from(quote! { #impl_block });
    }

    #[cfg(not(kani))]
    {
        let mut impl_block = impl_block;

        for item in &mut impl_block.items {
            if let ImplItem::Fn(method) = item {
                if matches!(method.vis, syn::Visibility::Public(_)) {
                    let method_name = method.sig.ident.to_string();
                    let has_generics = !method.sig.generics.params.is_empty();

                    let instrument_attr = if instrumented_impl_is_constructor(&method_name) {
                        if has_generics {
                            let param_names: Vec<_> = method
                                .sig
                                .inputs
                                .iter()
                                .filter_map(|arg| {
                                    if let syn::FnArg::Typed(pat_type) = arg
                                        && let syn::Pat::Ident(ident) = &*pat_type.pat
                                    {
                                        return Some(ident.ident.clone());
                                    }
                                    None
                                })
                                .collect();
                            quote! { #[tracing::instrument(skip(#(#param_names),*), err)] }
                        } else {
                            quote! { #[tracing::instrument(err)] }
                        }
                    } else if instrumented_impl_is_accessor(&method_name) {
                        quote! { #[tracing::instrument(level = "trace", ret)] }
                    } else {
                        quote! { #[tracing::instrument(skip(self))] }
                    };

                    let attr: syn::Attribute = syn::parse_quote! { #instrument_attr };
                    method.attrs.insert(0, attr);
                }
            }
        }

        TokenStream::from(quote! { #impl_block })
    }
}

fn instrumented_impl_is_constructor(name: &str) -> bool {
    name == "new" || name.starts_with("from_") || name.starts_with("try_") || name == "default"
}

fn instrumented_impl_is_accessor(name: &str) -> bool {
    name == "get"
        || name == "into_inner"
        || name.starts_with("as_")
        || name.starts_with("to_")
        || name.starts_with("get_")
}

/// Generates elicitation tool methods inside an impl block for rmcp tool_router.
///
/// For each type T, generates an `elicit_T` method with `#[tool]` marker.
#[proc_macro_attribute]
pub fn elicit_tools(attr: TokenStream, item: TokenStream) -> TokenStream {
    use syn::{ItemImpl, parse_macro_input};
    use quote::quote;

    let impl_block = parse_macro_input!(item as ItemImpl);

    let types_input = attr.to_string();
    let types: Vec<&str> = types_input
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if types.is_empty() {
        return syn::Error::new_spanned(
            &impl_block,
            "elicit_tools requires at least one type: #[elicit_tools(Type1, Type2)]",
        )
        .to_compile_error()
        .into();
    }

    let mut new_impl = impl_block.clone();

    for ty_str in types {
        let ty: syn::Type = match syn::parse_str(ty_str) {
            Ok(t) => t,
            Err(e) => {
                return syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Failed to parse type '{}': {}", ty_str, e),
                )
                .to_compile_error()
                .into();
            }
        };

        let method_name = elicit_tools_to_snake_case(ty_str);
        let method_ident = syn::Ident::new(
            &format!("elicit_{}", method_name),
            proc_macro2::Span::call_site(),
        );

        let tool_description = format!("Elicit {} via MCP", ty_str);
        let method: syn::ImplItemFn = syn::parse_quote! {
            #[doc = concat!("Elicit `", #ty_str, "` via MCP.")]
            #[tool(description = #tool_description)]
            pub async fn #method_ident(
                peer: ::rmcp::service::Peer<::rmcp::service::RoleServer>,
            ) -> ::std::result::Result<
                ::rmcp::handler::server::wrapper::Json<::elicitation::ElicitToolOutput<#ty>>,
                ::rmcp::ErrorData
            > {
                let value = #ty::elicit_checked(peer).await
                    .map_err(|e| ::rmcp::ErrorData::internal_error(e.to_string(), None))?;
                Ok(::rmcp::handler::server::wrapper::Json(::elicitation::ElicitToolOutput::new(value)))
            }
        };

        new_impl.items.push(syn::ImplItem::Fn(method));
    }

    TokenStream::from(quote! { #new_impl })
}

fn elicit_tools_to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_was_lowercase = false;
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 && prev_was_lowercase {
                result.push('_');
            }
            result.push(ch.to_ascii_lowercase());
            prev_was_lowercase = false;
        } else {
            result.push(ch);
            prev_was_lowercase = ch.is_lowercase();
        }
    }
    result
}

/// Generates MCP tool wrappers for trait methods, delegating to a named field.
#[proc_macro_attribute]
pub fn elicit_trait_tools_router(attr: TokenStream, item: TokenStream) -> TokenStream {
    use syn::{ItemImpl, parse_macro_input};
    use quote::quote;

    let attr_str = attr.to_string();
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut bracket_depth = 0;

    for ch in attr_str.chars() {
        match ch {
            '[' => { bracket_depth += 1; current.push(ch); }
            ']' => { bracket_depth -= 1; current.push(ch); }
            ',' if bracket_depth == 0 => { parts.push(current.trim().to_string()); current.clear(); }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        parts.push(current.trim().to_string());
    }

    if parts.len() != 3 {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "elicit_trait_tools_router requires three arguments: #[elicit_trait_tools_router(TraitName, field_name, [method1, method2])]",
        )
        .to_compile_error()
        .into();
    }

    let _trait_name = &parts[0];
    let field_name = &parts[1];
    let methods_str = &parts[2];

    let methods_str = methods_str.trim_start_matches('[').trim_end_matches(']');
    let methods: Vec<&str> = methods_str
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if methods.is_empty() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "elicit_trait_tools_router requires at least one method in the list",
        )
        .to_compile_error()
        .into();
    }

    let mut impl_block = parse_macro_input!(item as ItemImpl);

    for method_name in methods {
        let pascal_case = elicit_trait_tools_to_pascal_case(method_name);
        let params_type = format!("{}Params", pascal_case);
        let result_type = format!("{}Result", pascal_case);

        let params_ty: syn::Type = match syn::parse_str(&params_type) {
            Ok(t) => t,
            Err(e) => {
                return syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Failed to parse params type '{}': {}", params_type, e),
                )
                .to_compile_error()
                .into();
            }
        };

        let result_ty: syn::Type = match syn::parse_str(&result_type) {
            Ok(t) => t,
            Err(e) => {
                return syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Failed to parse result type '{}': {}", result_type, e),
                )
                .to_compile_error()
                .into();
            }
        };

        let method_ident = syn::Ident::new(method_name, proc_macro2::Span::call_site());
        let field_ident = syn::Ident::new(field_name, proc_macro2::Span::call_site());
        let tool_description = format!("{} operation", method_name.replace('_', " "));

        let method: syn::ImplItemFn = syn::parse_quote! {
            #[doc = concat!("`", #method_name, "` operation via trait method delegation.")]
            #[::rmcp::tool(description = #tool_description)]
            pub async fn #method_ident(
                &self,
                params: ::rmcp::handler::server::wrapper::Parameters<#params_ty>,
            ) -> ::std::result::Result<
                ::rmcp::handler::server::wrapper::Json<#result_ty>,
                ::rmcp::ErrorData
            > {
                self.#field_ident.#method_ident(params).await
            }
        };

        impl_block.items.push(syn::ImplItem::Fn(method));
    }

    TokenStream::from(quote! { #impl_block })
}

fn elicit_trait_tools_to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

/// Capture a third-party trait's methods as MCP tools.
///
/// Apply to an `impl` block that names the factory struct and lists the
/// method signatures of the third-party trait you want to wrap.
#[proc_macro_attribute]
pub fn reflect_trait(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr2 = proc_macro2::TokenStream::from(attr);
    let item2 = proc_macro2::TokenStream::from(item);
    match trait_reflection::expand(attr2, item2) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
