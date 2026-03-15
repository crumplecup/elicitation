# Planning Documents

This file tracks all planning documents for the elicitation project.

## Archive Notice

**All planning documents archived as of v0.7.0** (commit `98ad6f91b10ee273027ea07d5069da4d90a37e97`)

All previously tracked planning documents have been deleted from the working tree as they are now out of date. The complete history of all planning documents is preserved in git history. To view any archived document:

```bash
git show 98ad6f91b10ee273027ea07d5069da4d90a37e97:<filename>
```

Example:

```bash
git show 98ad6f91b10ee273027ea07d5069da4d90a37e97:UTF8_VERIFICATION_STRATEGY.md
git show 98ad6f91b10ee273027ea07d5069da4d90a37e97:KANI_VERIFICATION_PATTERNS.md
git show 98ad6f91b10ee273027ea07d5069da4d90a37e97:elicitation_vision.md
```

---

## Current Active Plans

### Third-Party Crate Support Guide

**Document:** [THIRD_PARTY_SUPPORT_GUIDE.md](THIRD_PARTY_SUPPORT_GUIDE.md)

**Status:** ✅ Active Reference

**Description:** Step-by-step checklist covering all six locations that must be updated
when adding elicitation support for a third-party crate: workspace wiring, core trait
impls (feature-gated), `elicit_*` newtype wrapper crate, Kani proofs, Creusot proofs,
and Verus proofs. Includes a per-type-category table, full code templates for each
pattern, and a copy-paste checklist. The `clap` integration is the canonical reference.

---

### Method Reflection (v0.9.0+)

**Document:** [METHOD_REFLECTION_PLAN.md](METHOD_REFLECTION_PLAN.md)

**Status:** ✅ Implementation Complete (Generic Support Added)

**Description:** Automatic MCP tool generation for third-party crate methods through newtype-based method reflection. Enables one-line integration of any Rust library as verified AI tools.

**Key Features:**

- `elicit_newtype!` macro for transparent wrapper generation
- `#[reflect_methods]` attribute for automatic method discovery
- Smart &T → T conversion for borrowed parameters
- JsonSchema-bounded generic support
- Seamless integration with existing `#[derive(Elicit)]`

**Completed:** Generic method support fully implemented across all derive macros.

### elicit_reqwest Shadow Crate (Integration Test)

**Document:** [ELICIT_REQWEST_PLAN.md](ELICIT_REQWEST_PLAN.md)

**Status:** Planning / Review

**Description:** Comprehensive integration test demonstrating all macro capabilities by wrapping the reqwest HTTP client library. Serves as both a real-world example and validation of generic support.

**Key Features:**

- Tests all three macro types: `elicit_newtype!`, `elicit_newtype_methods!`, `#[reflect_methods]`
- Mixed macro usage on same types (non-generic + generic methods)
- Generic trait bounds preservation (IntoUrl, Serialize, DeserializeOwned)
- Real HTTP client functionality
- Complete MCP tool integration example

**Timeline:** 4 phases (structure, non-generic, generic, integration)

### Proof Emission Redesign

**Document:** [PROOF_EMISSION_PLAN.md](PROOF_EMISSION_PLAN.md)

**Status:** 🔴 Not Started

**Description:** Replace the tautological `#[cfg(kani)] fn kani_proof()` stubs with
`TokenStream`-returning `emit_kani_proof()` / `emit_verus_proof()` / `emit_creusot_proof()`
methods on the `Elicitation` trait, feature-gated behind `"emit"`. Each primitive type
becomes a proof code generator. The derive macro composes field proofs into complete
composite harnesses. A new `ProofPlugin` exposes these as MCP tools.

New plans can be added here as needed for future development.

### Macro-Driven MCP Tool System

**Document:** [MACRO_TOOL_GEN_PLAN.md](MACRO_TOOL_GEN_PLAN.md)

**Status:** 🟡 Phase 1 In Progress

**Description:** Seven-phase plan to eliminate plugin boilerplate via `ToolDescriptor`,
`#[elicit_tool]`, `#[derive(ElicitPlugin)]`, and context injection. Phase 1 introduces
`ToolDescriptor` + `make_descriptor` + `DescriptorPlugin` blanket impl; `SecureFetchPlugin`
serves as the canary conversion.

**Phase 1 Progress:**

- ✅ `ToolDescriptor` + `make_descriptor` in `plugin/descriptor.rs`
- ✅ `DescriptorPlugin` blanket impl in `plugin/descriptor_plugin.rs`
- ✅ `SecureFetchPlugin` converted (canary validates design)

**Phase 2 Progress:**

- ✅ `#[elicit_tool]` attribute macro in `elicitation_derive/src/elicit_tool.rs`
- ✅ Re-exported as `elicitation::elicit_tool`
- ✅ `SecureFetchPlugin` canary updated: `#[elicit_tool]` on both handlers, `make_descriptor` calls eliminated

**Phase 3 Progress:**

- ✅ `PluginToolRegistration` + `inventory::collect!` in `plugin/descriptor.rs`
- ✅ `#[elicit_tool]` updated: optional `plugin = "..."` emits `inventory::submit!`
- ✅ `#[derive(ElicitPlugin)]` in `elicitation_derive/src/derive_elicit_plugin.rs`
- ✅ `elicitation::futures` re-exported (needed by generated code)
- ✅ `SecureFetchPlugin` is now a plain unit struct — 332 lines → ~75 lines of non-boilerplate

**Phase 4 Progress:**

- ✅ `PluginContext` in `plugin/context.rs` — feature-gated `http: reqwest::Client`
- ✅ `ToolDescriptor` handler type updated to `Arc<dyn Fn(Arc<PluginContext>, ...) -> ...>`
- ✅ `make_descriptor` (ctx-free) + `make_descriptor_ctx` (ctx-aware) constructors
- ✅ `#[derive(ElicitPlugin)]` detects unit vs newtype struct; unit → fresh context, newtype → `self.0.clone()`
- ✅ `#[elicit_tool]` detects `ctx: Arc<PluginContext>` first param; emits `make_descriptor_ctx`
- ✅ `SecureFetchPlugin(Arc<PluginContext>)` newtype; handlers use `ctx.http`; connection pool shared

**Phase 6 Progress (global emit registry):**

- ✅ `EmitEntry { tool, constructor }` + `inventory::collect!` in `elicitation::emit_code`
- ✅ `emit_code::dispatch_emit()` global lookup via inventory
- ✅ `register_emit!` macro using `elicitation::serde_json` and `elicitation::inventory`
- ✅ `register_emit!` calls added to all 8 workflow crates (37 registrations)
- ✅ `emit_plugin.rs` `dispatch_step` collapsed to single `elicitation::emit_code::dispatch_emit` call
- ✅ Phase 7 (guard attributes) superseded — see CONTRACT_PARAMS_PLAN.md

---

### Contract-Carrying Param Types

**Document:** [CONTRACT_PARAMS_PLAN.md](CONTRACT_PARAMS_PLAN.md)

**Status:** 🟡 Planning

**Description:** Replaces Phase 7 (guard attributes). Proof chains move into `Deserialize`
implementations on newtype param primitives — the type *is* the contract. No new attributes
or macros required. Tool bodies lose their validation ceremony; the JSON schema gains
machine-readable constraint metadata.

**Phases:**

- A: `elicitation::params` — `PositiveF64`, `NonNegativeF64`, `PositiveU32`, `NonEmptyString`, `BoundedUsize<MIN, MAX>`
- B: `elicit_url` contract types — `HttpsUrl` (wraps `SecureUrlState`), `ParsedUrl`
- C: Canary — `SecureFetchParams.url: HttpsUrl`; proof ceremony removed from handlers
- D: Propagation — apply contract types across all workflow params structs
- E: Kani harnesses for constructor correctness

EMIT_AUTODERIVE_PLAN.md

### Type Graph Visualization

**Document:** [TYPE_GRAPH_PLAN.md](TYPE_GRAPH_PLAN.md)

**Status:** ✅ Complete

**Guide:** [TYPE_GRAPH_GUIDE.md](TYPE_GRAPH_GUIDE.md)

**Description:** Framework-level workflow visualization via an inventory-based
`TypeGraphKey` registry and Mermaid/DOT renderers. Upgraded
`PatternDetails::Select` to carry full variant field structure.
CLI `graph` subcommand and `TypeGraphPlugin` MCP tool ship in the `graph` feature.

**Phases:**

- ✅ A-0: Upgrade `PatternDetails::Select` to `variants: Vec<VariantMetadata>`
- ✅ A-1: `TypeGraphKey` registry + `#[derive(Elicit)]` emission
- ✅ B: `TypeGraph` builder (BFS, cycle detection, qualified variant nodes)
- ✅ C: Mermaid + DOT renderers behind `GraphRenderer` trait
- ✅ D: `elicitation graph` CLI subcommand
- ✅ E: `TypeGraphPlugin` MCP tool (3 tools: list, graph, describe_edges)
