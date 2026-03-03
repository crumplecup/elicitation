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
        let step_tokens: Vec<TokenStream> = self.steps.iter().map(|s| s.emit_code()).collect();

        let tracing_init = if self.with_tracing {
            quote::quote! { tracing_subscriber::fmt::init(); }
        } else {
            TokenStream::new()
        };

        quote::quote! {
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
                        format!(r#"{} = {{ path = "{}" }}"#, d.name, path.display())
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
                $( let $T = self.$idx.emit_code(); )+
                quote::quote! { ( $( #$T ),+ ) }
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
