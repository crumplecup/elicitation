IMPLEMENTATION PLAN
Macro-Driven MCP Tool System for Elicitation

Revised based on codebase analysis (March 2026).

─────────────────────────────────────────────────────
CURRENT STATE & WHAT THIS PLAN ACTUALLY SOLVES
─────────────────────────────────────────────────────

A typical workflow plugin today (e.g. secure_fetch.rs, 332 lines) breaks
down roughly as:

  ~50 lines  param structs (the real work)
  ~30 lines  typed_tool::<T>() wiring in list_tools()
  ~40 lines  match dispatch in call_tool()
  ~100 lines EmitCode impls (cfg-gated, duplicating param fields)
  ~80 lines  impl EmitCode boilerplate per param type
  ~30 lines  dispatch_*_emit() fn in emit_plugin.rs

The params and the actual async impl body are the only non-mechanical parts.
Everything else is boilerplate this plan can eliminate.

Existing infrastructure to build on:
  • inventory crate already used (ElicitToolDescriptor, TypeSpecInventoryKey)
  • util::{parse_args, typed_tool} already extracted in elicit_server
  • EmitCode trait + BinaryScaffold already in elicitation/emit_code.rs
  • ElicitPlugin trait already defines list_tools() / call_tool() contract
  • #[tool] and #[tool_router] are rmcp macros — NAME CONFLICT, use
    #[elicit_tool] for our attribute to avoid shadowing

─────────────────────────────────────────────────────
PHASE 1 — ToolDescriptor: eliminate manual dispatch
─────────────────────────────────────────────────────

Goal
Replace the manual match in call_tool() and the manual list_tools()
wiring with a data-driven ToolDescriptor that carries its own handler.

This is pure library code — no macros yet.

Define in elicitation/src/plugin.rs (alongside ElicitPlugin):

    pub struct ToolDescriptor {
        pub name: &'static str,
        pub description: &'static str,
        pub schema: schemars::schema::RootSchema,
        pub handler: for<'a> fn(
            &'a CallToolRequestParams,
        ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>>,
    }

Helper constructor (lives in elicitation::plugin):

    pub fn make_descriptor<T, F>(
        name: &'static str,
        description: &'static str,
        handler: F,
    ) -> ToolDescriptor
    where
        T: DeserializeOwned + JsonSchema + 'static,
        F: Fn(T) -> BoxFuture<'static, Result<CallToolResult, ErrorData>>
            + Send + Sync + 'static

Add a blanket ElicitPlugin impl driven by ToolDescriptor slices:

    pub trait PluginFromDescriptors: Send + Sync {
        fn name(&self) -> &'static str;
        fn descriptors(&self) -> &'static [ToolDescriptor];
    }

    impl<T: PluginFromDescriptors> ElicitPlugin for T {
        fn list_tools(&self) -> Vec<Tool> { /* from descriptors */ }
        fn call_tool(...) -> BoxFuture { /* dispatch by name */ }
    }

Migration: convert SecureFetchPlugin first as pilot. Measure line count.
Gate behind no feature flag — this is just API ergonomics.

─────────────────────────────────────────────────────
PHASE 2 — #[elicit_tool] attribute macro
─────────────────────────────────────────────────────

Goal
Derive ToolDescriptor from a function signature. The attribute name is
#[elicit_tool] (not #[tool]) to avoid shadowing rmcp's existing #[tool]
macro which already appears throughout this codebase.

    #[elicit_tool(
        name = "secure_fetch",
        description = "Assert HTTPS and fetch a URL"
    )]
    async fn secure_fetch(p: SecureFetchParams) -> Result<CallToolResult, ErrorData> {
        secure_fetch_impl(p).await
    }

Macro generates:

    fn secure_fetch_descriptor() -> ToolDescriptor {
        make_descriptor::<SecureFetchParams, _>(
            "secure_fetch",
            "Assert HTTPS and fetch a URL",
            |p| Box::pin(secure_fetch(p)),
        )
    }

The static TOOLS array becomes a one-liner:

    static TOOLS: &[ToolDescriptor] = &[
        secure_fetch_descriptor(),
        validated_api_call_descriptor(),
    ];

Home: new crate elicitation_macros (already exists) or extend
elicitation_derive with a non-derive proc-macro entry point.
The latter is simpler — one crate, one proc-macro compile unit.

─────────────────────────────────────────────────────
PHASE 3 — #[derive(ElicitPlugin)] + inventory
─────────────────────────────────────────────────────

Goal
Eliminate the PluginFromDescriptors impl boilerplate.

    #[derive(ElicitPlugin)]
    #[plugin(name = "secure_fetch")]
    pub struct SecureFetchPlugin;

inventory::collect!(ToolDescriptor) lets #[elicit_tool] register at
link time. The derive impl becomes:

    fn list_tools(&self) -> Vec<Tool> {
        inventory::iter::<ToolDescriptor>()
            .filter(|d| d.plugin == "secure_fetch")
            .map(|d| typed_tool_from_descriptor(d))
            .collect()
    }

    fn call_tool(...) {
        inventory::iter::<ToolDescriptor>()
            .find(|d| d.name == params.name)
            .ok_or_else(|| ErrorData::method_not_found(...))?
            .dispatch(params)
    }

Important: inventory works at binary link time. Plugin prefix filtering
replaces the match dispatch entirely. No handwritten routing remains.

Caveat: we already use inventory for ElicitToolDescriptor and
TypeSpecInventoryKey — add a ToolDescriptor collect!() as a third
bucket, keeping them separate.

─────────────────────────────────────────────────────
PHASE 4 — Context injection (move up from Phase 7)
─────────────────────────────────────────────────────

Goal
reqwest::Client is re-created per-call today. This wastes connection
pool benefits and blocks future shared telemetry. Solve this before
Phase 5's EmitCode derive to avoid baking the stateless pattern in.

    pub struct PluginContext {
        pub http: reqwest::Client,
        // future: pub tracer: opentelemetry::Tracer,
    }

    impl Default for PluginContext {
        fn default() -> Self {
            Self { http: reqwest::Client::new() }
        }
    }

Plugins hold Arc<PluginContext>. #[elicit_tool] functions receive it:

    async fn secure_fetch(
        ctx: Arc<PluginContext>,
        p: SecureFetchParams,
    ) -> Result<CallToolResult, ErrorData>

The ToolDescriptor handler fn pointer becomes:

    fn(&Arc<PluginContext>, &CallToolRequestParams) -> BoxFuture<...>

Moved before Phase 5 because EmitCode templates need to know whether
context fields exist when generating replay code.

─────────────────────────────────────────────────────
PHASE 5 — #[derive(EmitCode)]
─────────────────────────────────────────────────────

Goal
Remove manual impl EmitCode blocks. Each EmitCode impl today is
~20–40 lines of quote! that reconstructs the tool call from its
param fields. The pattern is entirely mechanical.

    #[derive(EmitCode)]
    #[emit(template = "reqwest_https")]
    pub struct SecureFetchParams {
        pub url: String,
        pub timeout_secs: f64,
    }

The template name maps to a quote! block registered in
elicitation_macros/src/emit_templates.rs. The derive macro injects
field references by matching field names to template placeholders.

Template registration (compile-time, in the macro crate):

    // emit_templates.rs
    macro_rules! define_template {
        ($name:ident, $code:expr) => { ... }
    }

    define_template!(reqwest_https, quote! {
        let (parsed, proof) =
            elicit_url::UnvalidatedUrl::new(#url.to_string())
                .parse()
                .map_err(|e| format!("URL parse: {e}"))?;
        let (_secure, _https_proof) =
            parsed.assert_https(proof)
                .map_err(|e| format!("HTTPS required: {e}"))?;
        ctx.http
            .get(_secure.as_str())
            .timeout(std::time::Duration::from_secs_f64(#timeout_secs))
            .send()
            .await
            .map_err(|e| format!("request failed: {e}"))?
    });

No external template files — everything lives in the macro crate,
verifiable at compile time.

─────────────────────────────────────────────────────
PHASE 6 — Global emit registry
─────────────────────────────────────────────────────

Goal
Remove the per-plugin dispatch_*_emit() functions in emit_plugin.rs.
Currently emit_plugin.rs dispatches to 8 separate crate-level functions,
each doing the same thing with different param types.

inventory::submit! registers each param type's EmitCode constructor:

    struct EmitEntry {
        tool:        &'static str,
        constructor: fn(serde_json::Value) -> Result<Box<dyn EmitCode>, String>,
    }

    inventory::collect!(EmitEntry);

#[derive(EmitCode)] or #[elicit_tool] submits the entry automatically.

Global dispatcher in elicitation::emit_code:

    pub fn dispatch_emit(
        tool: &str,
        params: serde_json::Value,
    ) -> Result<Box<dyn EmitCode>, String> {
        inventory::iter::<EmitEntry>()
            .find(|e| e.tool == tool)
            .ok_or_else(|| format!("unknown tool: {tool}"))?
            .constructor(params)
    }

EmitBinaryPlugin in elicit_server then becomes a thin wrapper over this.

─────────────────────────────────────────────────────
PHASE 7 — SUPERSEDED
─────────────────────────────────────────────────────

The guard-attribute approach (`#[requires_https(url)]` etc.) was replaced
by CONTRACT_PARAMS_PLAN.md — contract-carrying param types.

Reason: guard attributes hide proof tokens, proliferate per-concept
attributes, and push validation to the wrong layer. The newtype approach
embeds the proof chain in `Deserialize`, making each type self-validating.
No new macros or attributes needed.

See CONTRACT_PARAMS_PLAN.md for the replacement plan.

    #[elicit_tool(name = "secure_fetch", description = "...")]
    #[requires_https(url)]
    async fn secure_fetch(
        ctx: Arc<PluginContext>,
        p: SecureFetchParams,
    ) -> Result<CallToolResult, ErrorData> {
        // url is already validated + https-enforced here
        ctx.http.get(&p.url).send().await?;
        ...
    }

#[requires_https(url)] expands to the UnvalidatedUrl → UrlParsed →
HttpsRequired proof chain before the function body executes. The tool
author sees a constraint, not a procedure.

This phase is where elicitation's typestate philosophy reaches the tool
authoring layer. Tools become declarative specifications of their
preconditions rather than imperative proof constructions.

Guards to define first:
  #[requires_https(field)]   — url parsed + HTTPS enforced
  #[requires_json(field)]    — string parsed as valid JSON
  #[requires_positive(field)] — numeric > 0 checked

─────────────────────────────────────────────────────
TARGET DEVELOPER EXPERIENCE (after all phases)
─────────────────────────────────────────────────────

    #[derive(ElicitPlugin)]
    #[plugin(name = "secure_fetch")]
    pub struct SecureFetchPlugin;

    #[elicit_tool(
        name = "secure_fetch",
        description = "Assert HTTPS and fetch a URL",
        plugin = "secure_fetch",
    )]
    #[requires_https(url)]
    #[derive(EmitCode)]
    #[emit(template = "reqwest_https")]
    pub struct SecureFetchParams {
        pub url: String,
        pub timeout_secs: f64,
    }

    async fn secure_fetch(
        ctx: Arc<PluginContext>,
        p: SecureFetchParams,
    ) -> Result<CallToolResult, ErrorData> {
        let response = ctx.http
            .get(&p.url)
            .timeout(Duration::from_secs_f64(p.timeout_secs))
            .send()
            .await?;
        Ok(tool_success(response))
    }

Everything else — schema wiring, dispatch, emit registration,
proof chain injection, context provisioning — is generated.

─────────────────────────────────────────────────────
EXPECTED RESULTS
─────────────────────────────────────────────────────

                     Before    After
secure_fetch.rs       332 L    ~50 L
fetch_and_parse.rs    289 L    ~45 L
elicit_url/workflow   587 L    ~80 L
elicit_chrono/workflow 575 L   ~80 L
emit_plugin.rs (elicit_server)  ~100 L  ~15 L

─────────────────────────────────────────────────────
SEQUENCING NOTE
─────────────────────────────────────────────────────

Phases 1–2 can be done incrementally against current plugins.
Phase 3 requires Phase 2 stable.
Phase 4 (context) should land before Phase 5 (EmitCode derive) to avoid
baking stateless assumptions into templates.
Phases 5–6 are independent once Phase 4 is in.
Phase 7 is last — it builds on everything and is the highest-value
payoff for elicitation's typestate design philosophy.

─────────────────────────────────────────────────────
LONG-TERM: VERIFIED TOOL DSL
─────────────────────────────────────────────────────

Phases 7 and a future Phase 8 converge toward:

    #[verified_tool]
    async fn secure_fetch(url: HttpsUrl, timeout: Seconds) { ... }

where the function signature IS the contract. The compiler emits:
  • MCP tool schema
  • Kani proof harness (via proofs feature)
  • Replay binary scaffold (via emit feature)
  • Typestate proof chain

A "tool" becomes a formally constrained executable program.

