//! Code recovery — emit verified workflows as Rust source.
//!
//! [`EmitCode`] is the reverse of the MCP transport layer: where the MCP layer
//! serializes Rust calls into JSON tool calls for agents, `EmitCode` recovers
//! the original Rust source from a concrete parameterized invocation.
//!
//! # The Core Idea
//!
//! Every MCP tool call is a thin wrapper over real Rust. An agent composing
//! `parse_and_focus → validate_object → pointer_update` has authored a verified
//! Rust program without knowing it. `EmitCode` materializes that program.
//!
//! The emitted source:
//! - Calls our library's verified APIs directly (not reimplementing logic)
//! - Preserves the full typestate ceremony: proof tokens, `Established<P>`, etc.
//! - Compiles against our pinned workspace crates as dependencies
//! - Inherits formal verification by virtue of calling verified action trait impls
//!
//! # Usage
//!
//! ```rust,ignore
//! use elicitation::emit_code::{BinaryScaffold, CrateDep, EmitCode};
//!
//! // A workflow params struct implements EmitCode
//! let step = ParseFocusParams { json: r#"{"name":"Alice"}"#.into(), pointer: "/name".into() };
//!
//! let scaffold = BinaryScaffold::new(vec![Box::new(step)], true);
//! let source = scaffold.to_source(); // formatted Rust source as String
//! ```
//!
//! # Primitive impls
//!
//! All numeric primitives, `bool`, `char`, and `String` are covered by an
//! internal macro that delegates to `quote::ToTokens`. Compound types
//! (`Vec<T>`, `Option<T>`, `PathBuf`, `Duration`, tuples) have explicit impls.

use proc_macro2::TokenStream;

// ── Global emit registry ──────────────────────────────────────────────────────

/// An inventory entry connecting a tool name to an [`EmitCode`] constructor.
///
/// Register via `inventory::submit!(EmitEntry { ... })` in each crate that
/// exposes tools with code-recovery support.  The global [`dispatch_emit`]
/// function collects all registered entries at runtime.
///
/// # Example
///
/// ```rust,ignore
/// # use elicitation::emit_code::{EmitEntry, EmitCode};
/// fn make_my_emit(v: serde_json::Value) -> Result<Box<dyn EmitCode>, String> {
///     serde_json::from_value::<MyParams>(v)
///         .map(|p| Box::new(p) as Box<dyn EmitCode>)
///         .map_err(|e| e.to_string())
/// }
///
/// inventory::submit! {
///     EmitEntry { tool: "my_tool", constructor: make_my_emit }
/// }
/// ```
pub struct EmitEntry {
    /// Bare tool name (no namespace prefix), e.g. `"parse_url"`.
    pub tool: &'static str,
    /// Crate that registered this entry, e.g. `"elicit_chrono"`.
    pub crate_name: &'static str,
    /// Deserialize params from JSON and box as [`EmitCode`].
    pub constructor: fn(serde_json::Value) -> Result<Box<dyn EmitCode>, String>,
}

inventory::collect!(EmitEntry);

/// Look up a tool name in the global emit registry and deserialize its params.
///
/// Returns a boxed [`EmitCode`] ready to pass to [`BinaryScaffold`], or an
/// error string if the tool is not registered or params fail to deserialize.
///
/// This replaces the per-crate `dispatch_*_emit` chain that was previously
/// required in [`EmitBinaryPlugin`](crate::plugin::ElicitPlugin).
pub fn dispatch_emit(tool: &str, params: serde_json::Value) -> Result<Box<dyn EmitCode>, String> {
    inventory::iter::<EmitEntry>()
        .find(|e| e.tool == tool)
        .ok_or_else(|| format!("unknown emit tool: '{tool}'"))
        .and_then(|e| (e.constructor)(params))
}

/// Look up a tool registered by a specific crate and deserialize its params.
///
/// Use this when multiple crates define a tool with the same name
/// (e.g. `"assert_future"` in `elicit_chrono`, `elicit_jiff`, `elicit_time`).
pub fn dispatch_emit_from(
    tool: &str,
    crate_name: &str,
    params: serde_json::Value,
) -> Result<Box<dyn EmitCode>, String> {
    inventory::iter::<EmitEntry>()
        .find(|e| e.tool == tool && e.crate_name == crate_name)
        .ok_or_else(|| format!("unknown emit tool: '{crate_name}::{tool}'"))
        .and_then(|e| (e.constructor)(params))
}

// ── Registration helper macro ─────────────────────────────────────────────────

/// Register a params type with the global emit registry under a tool name.
///
/// Generates a named constructor function (to satisfy `inventory`'s requirement
/// for `'static` function pointers) and submits an [`EmitEntry`].
///
/// Only active when the `emit` feature is enabled.
///
/// # Example
///
/// ```rust,ignore
/// // In workflow.rs, under #[cfg(feature = "emit")]:
/// register_emit!("parse_url", ParseUrlParams);
/// register_emit!("assert_https", AssertHttpsParams);
/// ```
#[macro_export]
macro_rules! register_emit {
    ($tool:literal, $T:ty) => {
        const _: () = {
            fn __emit_constructor(
                v: elicitation::serde_json::Value,
            ) -> ::std::result::Result<
                ::std::boxed::Box<dyn elicitation::emit_code::EmitCode>,
                ::std::string::String,
            > {
                elicitation::serde_json::from_value::<$T>(v)
                    .map(|p| {
                        ::std::boxed::Box::new(p)
                            as ::std::boxed::Box<dyn elicitation::emit_code::EmitCode>
                    })
                    .map_err(|e| e.to_string())
            }
            elicitation::inventory::submit! {
                elicitation::emit_code::EmitEntry {
                    tool: $tool,
                    crate_name: env!("CARGO_PKG_NAME"),
                    constructor: __emit_constructor,
                }
            }
        };
    };
}

/// Convert a value to a Rust source expression that constructs it.
///
/// Unlike [`EmitCode`] which for workflow step types emits a full statement
/// sequence, `ToCodeLiteral` emits a single *expression* that reproduces this
/// value. Used by `#[elicit_tool]`-generated `impl EmitCode` blocks to bind
/// field values.
pub trait ToCodeLiteral {
    /// Return a `TokenStream` containing a single Rust expression whose
    /// evaluation produces a value equal to `self`.
    fn to_code_literal(&self) -> TokenStream;

    /// Token stream for the concrete type name (used to annotate `None::<T>`).
    ///
    /// The default returns `_`, which works when context provides enough
    /// inference — but for `Option<T>` `None` cases, a concrete type avoids
    /// "type annotations needed" errors.
    fn type_tokens() -> TokenStream
    where
        Self: Sized,
    {
        quote::quote! { _ }
    }
}

/// Trait-based escape hatch for handlers the rewriter cannot auto-derive.
///
/// Implement this on a zero-sized type and annotate the handler with
/// `#[elicit_tool(emit = MyType)]`. The macro generates an [`EmitCode`] impl
/// that delegates `emit_code` to `MyType` and derives `crate_deps` automatically
/// from the crate's `Cargo.toml`.
///
/// # Example
///
/// ```rust,ignore
/// struct FetchJsonEmit;
/// impl CustomEmit<FetchJsonParams> for FetchJsonEmit {
///     fn emit_code(params: &FetchJsonParams) -> proc_macro2::TokenStream {
///         let url = params.url.to_code_literal();
///         quote::quote! { /* ... */ }
///     }
/// }
/// ```
pub trait CustomEmit<P> {
    /// Emit the Rust token stream for this step, given concrete params.
    fn emit_code(params: &P) -> TokenStream;
}

/// A type that knows how to recover itself as Rust source code.
///
/// Two roles:
///
/// - **Value emission** (primitives, std types): emit the literal that produces
///   this value. `42i32` emits `42i32`. `"hello"` emits `"hello".to_string()`.
/// - **Step emission** (workflow params): emit the full typestate sequence this
///   params struct drives. `ParseFocusParams` emits `RawJson::new → .parse() →
///   .focus() → .extract()`.
///
/// Implement this trait for any type whose construction should be recoverable
/// as part of an emitted binary.
pub trait EmitCode {
    /// Emit the Rust token stream for this item.
    ///
    /// The emitted code runs in an `async` context with `?` available.
    /// For value emission, emit a single expression. For step emission,
    /// emit a statement sequence.
    fn emit_code(&self) -> TokenStream;

    /// Crate dependencies required by the emitted code.
    ///
    /// The default impl returns an empty vec (for primitive/std types that
    /// need no external deps). Override for types that emit calls into
    /// workspace crates.
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![]
    }

    /// Whether this step's emitted code shares the outer function scope with
    /// adjacent steps.
    ///
    /// When `false` (default), `BinaryScaffold` wraps the step in `{ }` so
    /// local variables and type-inference contexts stay isolated.
    ///
    /// When `true`, the step is emitted directly into the function body — its
    /// bindings (e.g. `let pool = ...`) are visible to subsequent steps.
    /// Use this for workflow steps that intentionally pass state through
    /// variable names (e.g. sqlx `connect` → `execute` → `begin` → `commit`).
    fn shared_scope(&self) -> bool {
        false
    }
}

/// A pre-rendered token stream fragment received across an MCP boundary.
///
/// Emit tools return source fragments as plain strings.  When an agent passes
/// those strings to an [`AssembleParams`](crate::emit_code) step (or nests
/// one fragment inside another tool's parameters), this wrapper parses the
/// string back into a live [`TokenStream`] so it can participate in
/// [`BinaryScaffold`] assembly.
///
/// # Example
///
/// ```rust
/// use elicitation::emit_code::{EmitCode, RawFragment};
///
/// let fragment = RawFragment("format!(\"x = {}\", value)".into());
/// let ts = fragment.emit_code();
/// assert!(!ts.is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct RawFragment(pub String);

impl EmitCode for RawFragment {
    fn emit_code(&self) -> TokenStream {
        self.0
            .parse()
            .unwrap_or_else(|_| quote::quote!(/* fragment parse error */))
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![]
    }
}

/// Parse a Rust expression string into a token stream for literal emission.
///
/// This is used by derive-generated `ToCodeLiteral` impls for proxy fields that
/// store authored Rust expressions as strings but must emit them back as live
/// syntax rather than string literals.
pub fn parse_expr_tokens<S>(src: S, context: &str) -> TokenStream
where
    S: AsRef<str>,
{
    let src = src.as_ref();
    let expr = syn::parse_str::<syn::Expr>(src)
        .unwrap_or_else(|error| panic!("invalid {context} expression `{src}`: {error}"));
    quote::quote! { #expr }
}

/// A Cargo dependency descriptor with pinned version.
///
/// Each `EmitCode` impl that calls into a workspace crate returns `CrateDep`
/// entries so the scaffold can generate a correct `Cargo.toml`.
///
/// Versions are pinned by the impl author (co-located with the `EmitCode`
/// impl) — we know the correct versions because they are our workspace.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrateDep {
    /// Crate name as it appears in `Cargo.toml` (e.g. `"elicit_serde_json"`).
    pub name: &'static str,
    /// Semver version string (e.g. `"0.8"`).
    pub version: &'static str,
    /// Optional feature flags (e.g. `&["full"]`).
    pub features: &'static [&'static str],
}

impl CrateDep {
    /// Construct a dependency with no extra features.
    pub const fn new(name: &'static str, version: &'static str) -> Self {
        Self {
            name,
            version,
            features: &[],
        }
    }

    /// Construct a dependency with feature flags.
    pub const fn with_features(
        name: &'static str,
        version: &'static str,
        features: &'static [&'static str],
    ) -> Self {
        Self {
            name,
            version,
            features,
        }
    }

    /// Render as a TOML dependency line.
    ///
    /// ```rust
    /// use elicitation::emit_code::CrateDep;
    /// let dep = CrateDep::new("elicit_serde_json", "0.8");
    /// assert_eq!(dep.to_toml_line(), r#"elicit_serde_json = "0.8""#);
    /// ```
    pub fn to_toml_line(&self) -> String {
        if self.features.is_empty() {
            format!(r#"{} = "{}""#, self.name, self.version)
        } else {
            let feats = self
                .features
                .iter()
                .map(|f| format!(r#""{}""#, f))
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                r#"{} = {{ version = "{}", features = [{}] }}"#,
                self.name, self.version, feats
            )
        }
    }
}

// ── Blanket impl for ToTokens ─────────────────────────────────────────────────

/// Value-emission impl for primitive types via [`quote::ToTokens`].
///
/// We enumerate these explicitly (rather than a blanket `impl<T: ToTokens>`)
/// to avoid conflicts with our specific impls for `Vec`, `Option`, `PathBuf`, etc.
macro_rules! impl_emit_totokens {
    ($($T:ty),+ $(,)?) => {
        $(
            impl EmitCode for $T {
                fn emit_code(&self) -> TokenStream {
                    let mut ts = TokenStream::new();
                    quote::ToTokens::to_tokens(self, &mut ts);
                    ts
                }
            }
        )+
    };
}

impl_emit_totokens!(
    bool, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize, f32, f64, char, String,
);

// ── BinaryScaffold ────────────────────────────────────────────────────────────

/// A sequence of [`EmitCode`] steps wrapped in a `#[tokio::main]` binary scaffold.
///
/// Assembles multiple steps (each emitting async statement sequences) into a
/// complete, compilable Rust program with `main()`, optional tracing init, and
/// correct `Cargo.toml` dependencies.
///
/// # Example
///
/// ```rust,ignore
/// let scaffold = BinaryScaffold::new(vec![Box::new(step1), Box::new(step2)], true);
/// let source: String = scaffold.to_source(); // pretty-printed Rust
/// scaffold.emit_to_disk(std::path::Path::new("./output"))?;
/// ```
pub struct BinaryScaffold {
    steps: Vec<Box<dyn EmitCode>>,
    with_tracing: bool,
    /// When set, `elicit_*` / `elicitation` deps are emitted as path deps
    /// instead of crates.io version refs — enabling pre-publish dev/test.
    ///
    /// Falls back to the `ELICIT_WORKSPACE_ROOT` environment variable when
    /// `None`. Integrates cleanly with crates like `config` that manage env.
    workspace_root: Option<std::path::PathBuf>,
}

impl BinaryScaffold {
    /// Create a new scaffold from ordered steps.
    ///
    /// - `steps`: Ordered list of [`EmitCode`] items. Each step's emitted code
    ///   runs sequentially inside `main()`.
    /// - `with_tracing`: If true, inserts `tracing_subscriber::fmt::init();`
    ///   at the top of `main()`.
    pub fn new(steps: Vec<Box<dyn EmitCode>>, with_tracing: bool) -> Self {
        Self {
            steps,
            with_tracing,
            workspace_root: None,
        }
    }

    /// Override elicit workspace crates with local path deps instead of
    /// crates.io version strings.
    ///
    /// Use this during development / pre-publish testing so the emitted
    /// `Cargo.toml` resolves against your local checkout rather than the
    /// registry. Falls back to the `ELICIT_WORKSPACE_ROOT` env var when not
    /// set explicitly.
    pub fn with_workspace_root(mut self, root: impl Into<std::path::PathBuf>) -> Self {
        self.workspace_root = Some(root.into());
        self
    }

    /// Resolve the effective workspace root: explicit field → env var → None.
    fn resolved_workspace_root(&self) -> Option<std::path::PathBuf> {
        self.workspace_root
            .clone()
            .or_else(|| std::env::var("ELICIT_WORKSPACE_ROOT").ok().map(Into::into))
    }

    /// Collect all crate dependencies from all steps, deduplicated by name.
    ///
    /// When two steps declare the same crate name, the first declaration wins.
    /// Always includes `tokio` and `tracing-subscriber` (required by scaffold).
    pub fn all_deps(&self) -> Vec<CrateDep> {
        let mut seen = std::collections::HashSet::new();
        let mut deps = Vec::new();

        // Scaffold always needs these
        let scaffold_deps = [
            CrateDep::with_features("tokio", "1", &["full"]),
            CrateDep::new("tracing-subscriber", "0.3"),
            CrateDep::new("tracing", "0.1"),
        ];
        for dep in scaffold_deps {
            if seen.insert(dep.name) {
                deps.push(dep);
            }
        }

        for step in &self.steps {
            for dep in step.crate_deps() {
                if seen.insert(dep.name) {
                    deps.push(dep);
                }
            }
        }
        deps
    }

    /// Emit the raw token stream for the full `main.rs`.
    pub fn render(&self) -> TokenStream {
        let step_tokens: Vec<TokenStream> = self
            .steps
            .iter()
            .map(|s| {
                let code = s.emit_code();
                if s.shared_scope() {
                    // Steps that pass state (like `let pool`) to later steps
                    // must be emitted directly into the function body. The
                    // trailing `;` ensures a trailing expression is a statement.
                    quote::quote! { #code ; }
                } else {
                    // Wrap in a block for scope + type-inference isolation.
                    // This lets steps that end in `Ok(...)` work correctly.
                    quote::quote! { { #code } }
                }
            })
            .collect();

        let tracing_init = if self.with_tracing {
            quote::quote! { tracing_subscriber::fmt::init(); }
        } else {
            TokenStream::new()
        };

        // Wildcard imports for workflow crates so their types (e.g. UnvalidatedUrl,
        // both, Established) are in scope without fully-qualified paths.
        // `elicitation` uses a sub-module import to avoid shadowing workflow
        // crate prop structs that share names with elicitation verification types
        // (e.g. `elicit_reqwest::UrlValid` vs `elicitation::UrlValid`).
        let mut use_stmts: Vec<TokenStream> = self
            .all_deps()
            .into_iter()
            .filter(|d| d.name.starts_with("elicit"))
            .map(|d| {
                let krate: TokenStream = d.name.parse().expect("valid ident");
                if d.name == "elicitation" {
                    quote::quote! { use #krate::contracts::*; }
                } else {
                    quote::quote! { use #krate::*; }
                }
            })
            .collect();

        // When `reqwest` is a direct dep (e.g. from emit_ctx substitutions that
        // reference `reqwest::Client::new()`), bring `reqwest::header::HeaderMap`
        // into scope so handler bodies that call `HeaderMap::new()` compile cleanly
        // (the `elicit_reqwest::HeaderMap` newtype is a different type).
        if self.all_deps().iter().any(|d| d.name == "reqwest") {
            use_stmts.push(quote::quote! { use reqwest::header::HeaderMap; });
        }

        // `elicit_reqwest` handlers use `HashMap` for headers/query params.
        if self.all_deps().iter().any(|d| d.name == "elicit_reqwest") {
            use_stmts.push(quote::quote! { use std::collections::HashMap; });
        }

        quote::quote! {
            #( #use_stmts )*
            #[tokio::main]
            async fn main() -> Result<(), Box<dyn std::error::Error>> {
                #tracing_init
                #( #step_tokens )*
                Ok(())
            }
        }
    }

    /// Emit formatted Rust source for `main.rs`.
    ///
    /// Uses `prettyplease` to format the token stream into readable source.
    /// Returns an error if the token stream is not valid Rust syntax.
    pub fn to_source(&self) -> Result<String, syn::Error> {
        let tokens = self.render();
        let file: syn::File = syn::parse2(tokens)?;
        Ok(prettyplease::unparse(&file))
    }

    /// Emit the `Cargo.toml` content as a string.
    ///
    /// When a workspace root is available (via [`Self::with_workspace_root`]
    /// or the `ELICIT_WORKSPACE_ROOT` env var), any `elicit_*` / `elicitation*`
    /// dep is emitted as `{ path = "<root>/crates/<name>" }` instead of a
    /// crates.io version string. All other deps use their declared versions.
    pub fn to_cargo_toml(&self, package_name: &str) -> String {
        let ws_root = self.resolved_workspace_root();
        let deps = self.all_deps();
        let dep_lines: String = deps
            .iter()
            .map(|d| {
                let line = if let Some(ref root) = ws_root {
                    if d.name == "elicitation"
                        || d.name.starts_with("elicit_")
                        || d.name.starts_with("elicitation_")
                    {
                        let path = root.join("crates").join(d.name);
                        // Use forward slashes — TOML treats `\` as an escape
                        // character, so Windows paths would be invalid otherwise.
                        let path_str = path.to_string_lossy().replace('\\', "/");
                        format!(r#"{} = {{ path = "{}" }}"#, d.name, path_str)
                    } else {
                        d.to_toml_line()
                    }
                } else {
                    d.to_toml_line()
                };
                format!("{line}\n")
            })
            .collect();

        format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

# Prevent cargo from treating this as a member of any parent workspace.
[workspace]

[dependencies]
{}
"#,
            package_name, dep_lines
        )
    }

    /// Write `src/main.rs` and `Cargo.toml` to `output_dir`.
    ///
    /// Creates the directory structure if it does not exist.
    /// Returns the path to `src/main.rs` on success.
    pub fn emit_to_disk(
        &self,
        output_dir: &std::path::Path,
        package_name: &str,
    ) -> Result<std::path::PathBuf, EmitError> {
        let src_dir = output_dir.join("src");
        std::fs::create_dir_all(&src_dir)?;

        let source = self.to_source().map_err(EmitError::Syntax)?;
        let main_rs = src_dir.join("main.rs");
        std::fs::write(&main_rs, &source)?;

        let cargo_toml = output_dir.join("Cargo.toml");
        std::fs::write(&cargo_toml, self.to_cargo_toml(package_name))?;

        Ok(main_rs)
    }
}

// ── Artifact compilation ──────────────────────────────────────────────────────

/// Compile the generated project with `cargo build --release`.
///
/// Returns the path to the compiled binary on success, or a [`CompileError`]
/// containing stderr output on failure.
pub fn compile(project_dir: &std::path::Path) -> Result<std::path::PathBuf, CompileError> {
    let output = std::process::Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(project_dir)
        .output()
        .map_err(|e| CompileError::Io(e.to_string()))?;

    if output.status.success() {
        // Conventional release binary location
        let binary = project_dir.join("target/release").join(
            project_dir
                .file_name()
                .unwrap_or(std::ffi::OsStr::new("generated_workflow")),
        );
        Ok(binary)
    } else {
        Err(CompileError::CargoFailed(
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ))
    }
}

// ── Error types ───────────────────────────────────────────────────────────────

/// Error emitting source to disk.
#[derive(Debug, derive_more::Display, derive_more::Error)]
pub enum EmitError {
    /// The emitted token stream was not valid Rust syntax.
    #[display("Syntax error in emitted code: {}", _0)]
    Syntax(#[error(not(source))] syn::Error),
    /// File system error writing source or Cargo.toml.
    #[display("IO error: {}", _0)]
    Io(#[error(not(source))] std::io::Error),
}

impl From<std::io::Error> for EmitError {
    fn from(e: std::io::Error) -> Self {
        EmitError::Io(e)
    }
}

/// Error compiling the generated project.
#[derive(Debug, derive_more::Display, derive_more::Error)]
pub enum CompileError {
    /// `cargo build` exited with non-zero status. Contains stderr.
    #[display("Compilation failed:\n{}", _0)]
    CargoFailed(#[error(not(source))] String),
    /// Could not spawn the `cargo` process.
    #[display("Could not launch cargo: {}", _0)]
    Io(#[error(not(source))] String),
}

// ── Specific impls for std types not covered by blanket ───────────────────────

/// `Vec<T>` emits `vec![elem0, elem1, ...]`
impl<T: EmitCode> EmitCode for Vec<T> {
    fn emit_code(&self) -> TokenStream {
        let elems: Vec<TokenStream> = self.iter().map(|e| e.emit_code()).collect();
        quote::quote! { vec![ #( #elems ),* ] }
    }
}

/// `Option<T>` emits `Some(inner)` or `None`
impl<T: EmitCode> EmitCode for Option<T> {
    fn emit_code(&self) -> TokenStream {
        match self {
            Some(inner) => {
                let inner_ts = inner.emit_code();
                quote::quote! { Some(#inner_ts) }
            }
            None => quote::quote! { None },
        }
    }
}

/// `PathBuf` emits `std::path::PathBuf::from("...")`
impl EmitCode for std::path::PathBuf {
    fn emit_code(&self) -> TokenStream {
        let s = self.to_string_lossy();
        let s = s.as_ref();
        quote::quote! { std::path::PathBuf::from(#s) }
    }
}

/// `std::time::Duration` emits `std::time::Duration::from_nanos(n)`
impl EmitCode for std::time::Duration {
    fn emit_code(&self) -> TokenStream {
        let nanos = self.as_nanos() as u64;
        quote::quote! { std::time::Duration::from_nanos(#nanos) }
    }
}

// Tuples — macro to stamp out (A, B), (A, B, C), (A, B, C, D)
macro_rules! impl_emit_tuple {
    ( $( $T:ident ),+ ; $( $idx:tt ),+ ) => {
        impl< $( $T: EmitCode ),+ > EmitCode for ( $( $T, )+ ) {
            fn emit_code(&self) -> TokenStream {
                paste::paste! {
                    $( let [<$T:lower _val>] = self.$idx.emit_code(); )+
                    quote::quote! { ( $( #[<$T:lower _val>] ),+ ) }
                }
            }
        }
    };
}

impl_emit_tuple!(A, B; 0, 1);
impl_emit_tuple!(A, B, C; 0, 1, 2);
impl_emit_tuple!(A, B, C, D; 0, 1, 2, 3);

// ── Feature-gated impls ───────────────────────────────────────────────────────

/// `serde_json::Value` emits `serde_json::json!(...)` via the literal repr.
#[cfg(feature = "serde_json")]
impl EmitCode for serde_json::Value {
    fn emit_code(&self) -> TokenStream {
        let s = self.to_string();
        quote::quote! {
            serde_json::from_str(#s).expect("valid json literal")
        }
    }
}

/// `url::Url` emits `url::Url::parse("...").unwrap()`
#[cfg(feature = "url")]
impl EmitCode for url::Url {
    fn emit_code(&self) -> TokenStream {
        let s = self.as_str();
        quote::quote! { url::Url::parse(#s).expect("valid URL") }
    }
}

/// `uuid::Uuid` emits `uuid::Uuid::parse_str("...").unwrap()`
#[cfg(feature = "uuid")]
impl EmitCode for uuid::Uuid {
    fn emit_code(&self) -> TokenStream {
        let s = self.to_string();
        quote::quote! { uuid::Uuid::parse_str(#s).expect("valid UUID") }
    }
}

/// `std::net::IpAddr` emits `"...".parse::<std::net::IpAddr>().unwrap()`
impl EmitCode for std::net::IpAddr {
    fn emit_code(&self) -> TokenStream {
        let s = self.to_string();
        quote::quote! { #s.parse::<std::net::IpAddr>().expect("valid IP") }
    }
}

/// `std::net::Ipv4Addr`
impl EmitCode for std::net::Ipv4Addr {
    fn emit_code(&self) -> TokenStream {
        let s = self.to_string();
        quote::quote! { #s.parse::<std::net::Ipv4Addr>().expect("valid IPv4") }
    }
}

/// `std::net::Ipv6Addr`
impl EmitCode for std::net::Ipv6Addr {
    fn emit_code(&self) -> TokenStream {
        let s = self.to_string();
        quote::quote! { #s.parse::<std::net::Ipv6Addr>().expect("valid IPv6") }
    }
}

/// `chrono::DateTime<Utc>` emits RFC 3339 parse
#[cfg(feature = "chrono")]
impl EmitCode for chrono::DateTime<chrono::Utc> {
    fn emit_code(&self) -> TokenStream {
        let s = self.to_rfc3339();
        quote::quote! {
            chrono::DateTime::parse_from_rfc3339(#s)
                .expect("valid RFC3339 datetime")
                .with_timezone(&chrono::Utc)
        }
    }
}

/// `chrono::NaiveDateTime`
#[cfg(feature = "chrono")]
impl EmitCode for chrono::NaiveDateTime {
    fn emit_code(&self) -> TokenStream {
        let s = self.format("%Y-%m-%dT%H:%M:%S%.f").to_string();
        quote::quote! {
            chrono::NaiveDateTime::parse_from_str(#s, "%Y-%m-%dT%H:%M:%S%.f")
                .expect("valid NaiveDateTime")
        }
    }
}

/// `time::OffsetDateTime`
#[cfg(feature = "time")]
impl EmitCode for time::OffsetDateTime {
    fn emit_code(&self) -> TokenStream {
        let s = self
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap_or_default();
        quote::quote! {
            time::OffsetDateTime::parse(#s, &time::format_description::well_known::Rfc3339)
                .expect("valid OffsetDateTime")
        }
    }
}

/// `time::PrimitiveDateTime`
#[cfg(feature = "time")]
impl EmitCode for time::PrimitiveDateTime {
    fn emit_code(&self) -> TokenStream {
        // Iso8601::DEFAULT uses FormattedComponents::DateTimeOffset, which PrimitiveDateTime
        // cannot provide. Use a DateTime-only config instead.
        const PRIM_FMT: time::format_description::well_known::Iso8601<
            {
                time::format_description::well_known::iso8601::Config::DEFAULT
                .set_formatted_components(
                    time::format_description::well_known::iso8601::FormattedComponents::DateTime,
                )
                .encode()
            },
        > = time::format_description::well_known::Iso8601;
        let s = self.format(&PRIM_FMT).unwrap_or_default();
        quote::quote! {
            {
                const PRIM_FMT: time::format_description::well_known::Iso8601<{
                    time::format_description::well_known::iso8601::Config::DEFAULT
                        .set_formatted_components(
                            time::format_description::well_known::iso8601::FormattedComponents::DateTime,
                        )
                        .encode()
                }> = time::format_description::well_known::Iso8601;
                time::PrimitiveDateTime::parse(#s, &PRIM_FMT).expect("valid PrimitiveDateTime")
            }
        }
    }
}

/// `time::Time`
#[cfg(feature = "time")]
impl EmitCode for time::Time {
    fn emit_code(&self) -> TokenStream {
        let h = self.hour();
        let m = self.minute();
        let s = self.second();
        let ns = self.nanosecond();
        quote::quote! {
            time::Time::from_hms_nano(#h, #m, #s, #ns).expect("valid Time")
        }
    }
}

/// `jiff::Timestamp`
#[cfg(feature = "jiff")]
impl EmitCode for jiff::Timestamp {
    fn emit_code(&self) -> TokenStream {
        let s = self.to_string();
        quote::quote! {
            #s.parse::<jiff::Timestamp>().expect("valid Timestamp")
        }
    }
}

/// `jiff::Zoned` — emits parse from its Display representation.
#[cfg(feature = "jiff")]
impl EmitCode for jiff::Zoned {
    fn emit_code(&self) -> TokenStream {
        let s = self.to_string();
        quote::quote! {
            #s.parse::<jiff::Zoned>().expect("valid Zoned")
        }
    }
}

/// `jiff::civil::DateTime`
#[cfg(feature = "jiff")]
impl EmitCode for jiff::civil::DateTime {
    fn emit_code(&self) -> TokenStream {
        let s = self.to_string();
        quote::quote! {
            #s.parse::<jiff::civil::DateTime>().expect("valid civil DateTime")
        }
    }
}

/// `reqwest::StatusCode`
#[cfg(feature = "reqwest")]
impl EmitCode for reqwest::StatusCode {
    fn emit_code(&self) -> TokenStream {
        let n = self.as_u16();
        quote::quote! {
            reqwest::StatusCode::from_u16(#n).expect("valid status code")
        }
    }
}

// ── ToCodeLiteral impls ───────────────────────────────────────────────────────

macro_rules! impl_to_code_literal_totokens {
    ($($T:ty),+ $(,)?) => {
        $(
            impl ToCodeLiteral for $T {
                fn to_code_literal(&self) -> TokenStream {
                    let mut ts = TokenStream::new();
                    quote::ToTokens::to_tokens(self, &mut ts);
                    ts
                }
                fn type_tokens() -> TokenStream {
                    quote::quote! { $T }
                }
            }
        )+
    };
}

impl_to_code_literal_totokens!(
    bool, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize, f32, f64, char,
);

impl ToCodeLiteral for String {
    fn to_code_literal(&self) -> TokenStream {
        let s = self.as_str();
        quote::quote! { #s.to_string() }
    }
    fn type_tokens() -> TokenStream {
        quote::quote! { String }
    }
}

impl<T: ToCodeLiteral> ToCodeLiteral for Option<T> {
    fn to_code_literal(&self) -> TokenStream {
        match self {
            Some(v) => {
                let inner = v.to_code_literal();
                quote::quote! { ::std::option::Option::Some(#inner) }
            }
            None => {
                let t = <T as ToCodeLiteral>::type_tokens();
                quote::quote! { None::<#t> }
            }
        }
    }
}

impl<T: ToCodeLiteral> ToCodeLiteral for Vec<T> {
    fn type_tokens() -> TokenStream {
        let t = <T as ToCodeLiteral>::type_tokens();
        quote::quote! { ::std::vec::Vec<#t> }
    }

    fn to_code_literal(&self) -> TokenStream {
        let elems: Vec<_> = self.iter().map(|v| v.to_code_literal()).collect();
        quote::quote! { ::std::vec![#(#elems),*] }
    }
}

impl<T: ToCodeLiteral, const N: usize> ToCodeLiteral for [T; N] {
    fn type_tokens() -> TokenStream {
        let t = <T as ToCodeLiteral>::type_tokens();
        let n = proc_macro2::Literal::usize_suffixed(N);
        quote::quote! { [#t; #n] }
    }

    fn to_code_literal(&self) -> TokenStream {
        let elements: Vec<_> = self.iter().map(|e| e.to_code_literal()).collect();
        quote::quote! { [#(#elements),*] }
    }
}

impl<V: ToCodeLiteral> ToCodeLiteral for std::collections::HashMap<String, V> {
    fn type_tokens() -> TokenStream {
        let v = <V as ToCodeLiteral>::type_tokens();
        quote::quote! { ::std::collections::HashMap<::std::string::String, #v> }
    }

    fn to_code_literal(&self) -> TokenStream {
        let entries: Vec<_> = self
            .iter()
            .map(|(k, v)| {
                let v_ts = v.to_code_literal();
                quote::quote! { (#k.to_string(), #v_ts) }
            })
            .collect();
        quote::quote! {
            [#(#entries),*].into_iter().collect::<::std::collections::HashMap<_, _>>()
        }
    }
}

impl<V: ToCodeLiteral> ToCodeLiteral for std::collections::BTreeMap<String, V> {
    fn type_tokens() -> TokenStream {
        let v = <V as ToCodeLiteral>::type_tokens();
        quote::quote! { ::std::collections::BTreeMap<::std::string::String, #v> }
    }

    fn to_code_literal(&self) -> TokenStream {
        let entries: Vec<_> = self
            .iter()
            .map(|(k, v)| {
                let v_ts = v.to_code_literal();
                quote::quote! { (#k.to_string(), #v_ts) }
            })
            .collect();
        quote::quote! {
            [#(#entries),*].into_iter().collect::<::std::collections::BTreeMap<_, _>>()
        }
    }
}

impl<T: ToCodeLiteral> ToCodeLiteral for Box<T> {
    fn type_tokens() -> TokenStream {
        let inner = <T as ToCodeLiteral>::type_tokens();
        quote::quote! { ::std::boxed::Box<#inner> }
    }

    fn to_code_literal(&self) -> TokenStream {
        let inner = (**self).to_code_literal();
        quote::quote! { ::std::boxed::Box::new(#inner) }
    }
}

impl<A: ToCodeLiteral, B: ToCodeLiteral> ToCodeLiteral for (A, B) {
    fn type_tokens() -> TokenStream {
        let a = <A as ToCodeLiteral>::type_tokens();
        let b = <B as ToCodeLiteral>::type_tokens();
        quote::quote! { (#a, #b) }
    }

    fn to_code_literal(&self) -> TokenStream {
        let a = self.0.to_code_literal();
        let b = self.1.to_code_literal();
        quote::quote! { (#a, #b) }
    }
}

impl<A: ToCodeLiteral, B: ToCodeLiteral, C: ToCodeLiteral> ToCodeLiteral for (A, B, C) {
    fn type_tokens() -> TokenStream {
        let a = <A as ToCodeLiteral>::type_tokens();
        let b = <B as ToCodeLiteral>::type_tokens();
        let c = <C as ToCodeLiteral>::type_tokens();
        quote::quote! { (#a, #b, #c) }
    }

    fn to_code_literal(&self) -> TokenStream {
        let a = self.0.to_code_literal();
        let b = self.1.to_code_literal();
        let c = self.2.to_code_literal();
        quote::quote! { (#a, #b, #c) }
    }
}

impl<A: ToCodeLiteral, B: ToCodeLiteral, C: ToCodeLiteral, D: ToCodeLiteral> ToCodeLiteral
    for (A, B, C, D)
{
    fn type_tokens() -> TokenStream {
        let a = <A as ToCodeLiteral>::type_tokens();
        let b = <B as ToCodeLiteral>::type_tokens();
        let c = <C as ToCodeLiteral>::type_tokens();
        let d = <D as ToCodeLiteral>::type_tokens();
        quote::quote! { (#a, #b, #c, #d) }
    }

    fn to_code_literal(&self) -> TokenStream {
        let a = self.0.to_code_literal();
        let b = self.1.to_code_literal();
        let c = self.2.to_code_literal();
        let d = self.3.to_code_literal();
        quote::quote! { (#a, #b, #c, #d) }
    }
}

// ToCodeLiteral for std types that already have EmitCode: delegate directly.

impl ToCodeLiteral for std::net::IpAddr {
    fn to_code_literal(&self) -> TokenStream {
        EmitCode::emit_code(self)
    }
}

impl ToCodeLiteral for std::net::Ipv4Addr {
    fn to_code_literal(&self) -> TokenStream {
        EmitCode::emit_code(self)
    }
}

impl ToCodeLiteral for std::net::Ipv6Addr {
    fn to_code_literal(&self) -> TokenStream {
        EmitCode::emit_code(self)
    }
}

impl ToCodeLiteral for std::path::PathBuf {
    fn to_code_literal(&self) -> TokenStream {
        EmitCode::emit_code(self)
    }
}

impl ToCodeLiteral for std::time::Duration {
    fn to_code_literal(&self) -> TokenStream {
        EmitCode::emit_code(self)
    }
}

#[cfg(feature = "serde_json")]
impl ToCodeLiteral for serde_json::Value {
    fn to_code_literal(&self) -> TokenStream {
        EmitCode::emit_code(self)
    }
}

#[cfg(feature = "url")]
impl ToCodeLiteral for url::Url {
    fn to_code_literal(&self) -> TokenStream {
        EmitCode::emit_code(self)
    }
}

#[cfg(feature = "url")]
impl ToCodeLiteral for url::SyntaxViolation {
    fn to_code_literal(&self) -> TokenStream {
        use url::SyntaxViolation::*;
        let variant = match self {
            Backslash => quote::quote! { url::SyntaxViolation::Backslash },
            C0SpaceIgnored => quote::quote! { url::SyntaxViolation::C0SpaceIgnored },
            EmbeddedCredentials => quote::quote! { url::SyntaxViolation::EmbeddedCredentials },
            ExpectedDoubleSlash => quote::quote! { url::SyntaxViolation::ExpectedDoubleSlash },
            ExpectedFileDoubleSlash => {
                quote::quote! { url::SyntaxViolation::ExpectedFileDoubleSlash }
            }
            FileWithHostAndWindowsDrive => {
                quote::quote! { url::SyntaxViolation::FileWithHostAndWindowsDrive }
            }
            NonUrlCodePoint => quote::quote! { url::SyntaxViolation::NonUrlCodePoint },
            NullInFragment => quote::quote! { url::SyntaxViolation::NullInFragment },
            PercentDecode => quote::quote! { url::SyntaxViolation::PercentDecode },
            TabOrNewlineIgnored => quote::quote! { url::SyntaxViolation::TabOrNewlineIgnored },
            UnencodedAtSign => quote::quote! { url::SyntaxViolation::UnencodedAtSign },
            _ => unreachable!("unknown SyntaxViolation variant"),
        };
        variant
    }
}

#[cfg(feature = "uuid")]
impl ToCodeLiteral for uuid::Uuid {
    fn to_code_literal(&self) -> TokenStream {
        EmitCode::emit_code(self)
    }
}

#[cfg(feature = "chrono")]
impl ToCodeLiteral for chrono::DateTime<chrono::Utc> {
    fn to_code_literal(&self) -> TokenStream {
        EmitCode::emit_code(self)
    }
}

#[cfg(feature = "chrono")]
impl ToCodeLiteral for chrono::NaiveDateTime {
    fn to_code_literal(&self) -> TokenStream {
        EmitCode::emit_code(self)
    }
}

#[cfg(feature = "chrono")]
impl ToCodeLiteral for chrono::DateTime<chrono::FixedOffset> {
    fn to_code_literal(&self) -> TokenStream {
        let s = self.to_rfc3339();
        quote::quote! {
            chrono::DateTime::parse_from_rfc3339(#s)
                .expect("valid RFC3339 datetime")
        }
    }

    fn type_tokens() -> TokenStream {
        quote::quote! { chrono::DateTime<chrono::FixedOffset> }
    }
}

#[cfg(feature = "chrono")]
impl ToCodeLiteral for chrono::TimeDelta {
    fn to_code_literal(&self) -> TokenStream {
        let secs = self.num_seconds();
        let sub_nanos = self.subsec_nanos();
        if sub_nanos == 0 {
            quote::quote! {
                chrono::TimeDelta::try_seconds(#secs).expect("valid seconds")
            }
        } else {
            // Build as whole-second base + nanosecond remainder
            quote::quote! {
                chrono::TimeDelta::try_seconds(#secs).expect("valid seconds")
                    + chrono::TimeDelta::nanoseconds(#sub_nanos as i64)
            }
        }
    }

    fn type_tokens() -> TokenStream {
        quote::quote! { chrono::TimeDelta }
    }
}

#[cfg(feature = "time")]
impl ToCodeLiteral for time::OffsetDateTime {
    fn to_code_literal(&self) -> TokenStream {
        EmitCode::emit_code(self)
    }
}

#[cfg(feature = "time")]
impl ToCodeLiteral for time::PrimitiveDateTime {
    fn to_code_literal(&self) -> TokenStream {
        EmitCode::emit_code(self)
    }
}

#[cfg(feature = "time")]
impl ToCodeLiteral for time::Time {
    fn to_code_literal(&self) -> TokenStream {
        EmitCode::emit_code(self)
    }
}

#[cfg(feature = "jiff")]
impl ToCodeLiteral for jiff::Timestamp {
    fn to_code_literal(&self) -> TokenStream {
        EmitCode::emit_code(self)
    }
}

#[cfg(feature = "jiff")]
impl ToCodeLiteral for jiff::Zoned {
    fn to_code_literal(&self) -> TokenStream {
        EmitCode::emit_code(self)
    }
}

#[cfg(feature = "jiff")]
impl ToCodeLiteral for jiff::civil::DateTime {
    fn to_code_literal(&self) -> TokenStream {
        EmitCode::emit_code(self)
    }
}

#[cfg(feature = "reqwest")]
impl ToCodeLiteral for reqwest::StatusCode {
    fn to_code_literal(&self) -> TokenStream {
        EmitCode::emit_code(self)
    }
}

// ── Atomic types ─────────────────────────────────────────────────────────────

/// Generate `ToCodeLiteral` for an atomic type by loading the current value
/// and emitting `::std::sync::atomic::Atomic*::new(value)`.
macro_rules! impl_atomic_to_code_literal {
    ($($atomic:ident => $prim:ty),+ $(,)?) => {
        $(
            impl ToCodeLiteral for ::std::sync::atomic::$atomic {
                fn to_code_literal(&self) -> TokenStream {
                    use ::std::sync::atomic::Ordering;
                    let val = self.load(Ordering::SeqCst);
                    let val_lit = <$prim as ToCodeLiteral>::to_code_literal(&val);
                    let ty: TokenStream =
                        concat!("::std::sync::atomic::", stringify!($atomic))
                            .parse()
                            .expect("valid atomic type path");
                    quote::quote! { #ty::new(#val_lit) }
                }

                fn type_tokens() -> TokenStream {
                    concat!("::std::sync::atomic::", stringify!($atomic))
                        .parse()
                        .expect("valid atomic type path")
                }
            }
        )+
    };
}

impl_atomic_to_code_literal!(
    AtomicBool => bool,
    AtomicI8 => i8,
    AtomicI16 => i16,
    AtomicI32 => i32,
    AtomicI64 => i64,
    AtomicIsize => isize,
    AtomicU8 => u8,
    AtomicU16 => u16,
    AtomicU32 => u32,
    AtomicU64 => u64,
    AtomicUsize => usize,
);
