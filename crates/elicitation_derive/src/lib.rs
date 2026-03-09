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
mod elicit_tool;
mod emit_rewriter;
mod enum_impl;
mod method_reflection;
mod rand_contract_parser;
mod rand_generator_impl;
mod struct_impl;
mod tool_gen;

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
