//! `RegexWorkflowPlugin` — contract-verified regex composition tools.
//!
//! While the [`Regex`](crate::Regex) newtype exposes instance-level methods via
//! `#[reflect_methods]`, this plugin provides **phrase-level** stateless tools:
//! compile, match, search, replace, and capture extraction — each carrying a
//! proposition proof so agents can reason about what was established.
//!
//! # Typestate Design
//!
//! ```text
//! UnvalidatedPattern ──compile()──→ CompiledPattern + Established<RegexValid>
//!                                          │
//!                                   is_match(text)
//!                                          │
//!                                          ↓
//!                                   bool + Established<PatternMatched>  (if true)
//! ```
//!
//! # Propositions and Contracts
//!
//! ```text
//! compile:        RegexValid
//! is_match:       RegexValid ∧ PatternMatched   (on true result)
//! find_all:       RegexValid
//! replace_all:    RegexValid
//! capture_groups: RegexValid
//! ```
//!
//! # Code Recovery (N → 1)
//!
//! All tools participate in `emit_binary` code recovery via `emit = Auto`.
//! The `#[elicit_tool]` macro generates both `impl EmitCode` (with `crate_deps`
//! read from `Cargo.toml` at macro-expansion time) and `register_emit!`
//! inventory submission — no hand-written emit code is needed.
//!
//! Registered under the `"regex_workflow"` namespace.

use elicitation::contracts::{And, Established};
use elicitation::{ElicitPlugin, Prop, elicit_tool};
use regex::Regex as InnerRegex;
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: the pattern string compiled into a valid `Regex`.
#[derive(Prop)]
pub struct RegexValid;

/// Proposition: the regex found at least one match in the target text.
#[derive(Prop)]
pub struct PatternMatched;

/// Composite: the pattern is valid AND it matched.
pub type MatchProof = And<RegexValid, PatternMatched>;

// ── Typestate structs ─────────────────────────────────────────────────────────

/// An unvalidated pattern string — the initial state.
pub struct UnvalidatedPattern {
    src: String,
}

/// A successfully compiled regex — carries the `Regex` internally.
pub struct CompiledPattern {
    /// The inner compiled regex.
    pub inner: InnerRegex,
}

// ── Typestate transitions ─────────────────────────────────────────────────────

impl UnvalidatedPattern {
    /// Wrap a raw string as an unvalidated regex pattern.
    pub fn new(src: impl Into<String>) -> Self {
        Self { src: src.into() }
    }

    /// Compile the pattern, establishing `RegexValid` proof on success.
    pub fn compile(self) -> Result<(CompiledPattern, Established<RegexValid>), String> {
        InnerRegex::new(&self.src)
            .map(|inner| (CompiledPattern { inner }, Established::assert()))
            .map_err(|e| format!("RegexValid not established: {e}"))
    }
}

impl CompiledPattern {
    /// Return the inner compiled regex.
    pub fn into_inner(self) -> InnerRegex {
        self.inner
    }

    /// Assert the regex matches somewhere in `text`, establishing `MatchProof`.
    pub fn assert_matches(
        self,
        text: &str,
        valid: Established<RegexValid>,
    ) -> Result<(CompiledPattern, Established<MatchProof>), String> {
        if self.inner.is_match(text) {
            let proof =
                elicitation::contracts::both(valid, Established::<PatternMatched>::assert());
            Ok((self, proof))
        } else {
            Err(format!(
                "PatternMatched not established: pattern `{}` did not match",
                self.inner.as_str()
            ))
        }
    }
}

// ── Params structs ────────────────────────────────────────────────────────────

/// Parameters for [`RegexWorkflowPlugin::compile`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct CompileParams {
    /// The regex pattern to compile (e.g. `"^hello\\s+world$"`).
    pub pattern: String,
}

/// Parameters for [`RegexWorkflowPlugin::is_match`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct IsMatchParams {
    /// The regex pattern.
    pub pattern: String,
    /// The text to search.
    pub text: String,
}

/// Parameters for [`RegexWorkflowPlugin::find_all`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct FindAllParams {
    /// The regex pattern.
    pub pattern: String,
    /// The text to search.
    pub text: String,
}

/// Parameters for [`RegexWorkflowPlugin::replace_all`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ReplaceAllParams {
    /// The regex pattern.
    pub pattern: String,
    /// The text to transform.
    pub text: String,
    /// Replacement string; use `$1`, `$name` to reference capture groups.
    pub replacement: String,
}

/// Parameters for [`RegexWorkflowPlugin::capture_groups`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct CaptureGroupsParams {
    /// The regex pattern (must contain at least one capture group).
    pub pattern: String,
    /// The text to search for captures.
    pub text: String,
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin providing verified regex workflow tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "regex_workflow")]
pub struct RegexWorkflowPlugin;

// ── Tool handlers ─────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "regex_workflow",
    name = "compile",
    description = "Compile a regex pattern to validate it. \
                   Establishes: RegexValid. \
                   Returns a confirmation message or a descriptive compile error."
)]
#[instrument(skip_all)]
async fn compile(p: CompileParams) -> Result<CallToolResult, ErrorData> {
    match UnvalidatedPattern::new(&p.pattern).compile() {
        Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
            "RegexValid established.\npattern: {}",
            p.pattern
        ))])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e)])),
    }
}

#[elicit_tool(
    plugin = "regex_workflow",
    name = "is_match",
    description = "Test whether a pattern matches anywhere in a text. \
                   Returns `true` or `false`. \
                   Establishes: RegexValid ∧ PatternMatched on a true result."
)]
#[instrument(skip_all)]
async fn is_match(p: IsMatchParams) -> Result<CallToolResult, ErrorData> {
    let (compiled, valid) = match UnvalidatedPattern::new(&p.pattern).compile() {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    let matched = compiled.inner.is_match(&p.text);
    let summary = if matched {
        let _proof = elicitation::contracts::both(valid, Established::<PatternMatched>::assert());
        "RegexValid ∧ PatternMatched established.\nresult: true".to_string()
    } else {
        "RegexValid established.\nresult: false".to_string()
    };
    Ok(CallToolResult::success(vec![Content::text(summary)]))
}

#[elicit_tool(
    plugin = "regex_workflow",
    name = "find_all",
    description = "Return all non-overlapping matches of a pattern in text as a JSON array. \
                   Establishes: RegexValid."
)]
#[instrument(skip_all)]
async fn find_all(p: FindAllParams) -> Result<CallToolResult, ErrorData> {
    let (compiled, _proof) = match UnvalidatedPattern::new(&p.pattern).compile() {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    let matches: Vec<&str> = compiled
        .inner
        .find_iter(&p.text)
        .map(|m| m.as_str())
        .collect();
    match serde_json::to_string(&matches) {
        Ok(json) => Ok(CallToolResult::success(vec![Content::text(format!(
            "RegexValid established.\nmatches: {json}"
        ))])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "regex_workflow",
    name = "replace_all",
    description = "Replace all non-overlapping matches of a pattern in text. \
                   Use `$1`, `$name` in `replacement` to back-reference capture groups. \
                   Establishes: RegexValid."
)]
#[instrument(skip_all)]
async fn replace_all(p: ReplaceAllParams) -> Result<CallToolResult, ErrorData> {
    let (compiled, _proof) = match UnvalidatedPattern::new(&p.pattern).compile() {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    let result = compiled
        .inner
        .replace_all(&p.text, p.replacement.as_str())
        .into_owned();
    Ok(CallToolResult::success(vec![Content::text(format!(
        "RegexValid established.\nresult: {result}"
    ))]))
}

#[elicit_tool(
    plugin = "regex_workflow",
    name = "capture_groups",
    description = "Extract all capture groups from every match of a pattern in text. \
                   Returns a JSON array of arrays — one inner array per match, each element \
                   being a capture group string or null if the group did not participate. \
                   Establishes: RegexValid."
)]
#[instrument(skip_all)]
async fn capture_groups(p: CaptureGroupsParams) -> Result<CallToolResult, ErrorData> {
    let (compiled, _proof) = match UnvalidatedPattern::new(&p.pattern).compile() {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    let all_captures: Vec<Vec<Option<String>>> = compiled
        .inner
        .captures_iter(&p.text)
        .map(|caps| {
            caps.iter()
                .skip(1) // skip whole-match group 0
                .map(|m| m.map(|g| g.as_str().to_owned()))
                .collect()
        })
        .collect();
    match serde_json::to_string(&all_captures) {
        Ok(json) => Ok(CallToolResult::success(vec![Content::text(format!(
            "RegexValid established.\ncaptures: {json}"
        ))])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}
