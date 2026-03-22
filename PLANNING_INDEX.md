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

---

### Trait Reflection (`#[reflect_trait]`)

**Document:** [REFLECT_TRAIT_PLAN.md](REFLECT_TRAIT_PLAN.md)

**Status:** 🔲 Planning / Review

**Description:** A `#[reflect_trait(SourceTrait)]` attribute macro complementing
`#[reflect_methods]`. Where `#[reflect_methods]` works on `impl Type { fn bodies; }`,
`#[reflect_trait]` works on `impl WrapperTrait for Type { fn sigs; }` with bare
method signatures — the macro generates delegation bodies and MCP tool wrappers.

**Key features:**

- One impl block per concrete type instead of a full newtype file per type
- Delegation via `<Type as SourceTrait>::method()` — no recursive dispatch
- Tool names derived from `for T` type path (e.g. `clap::ColorChoice` → `color_choice_labels`)
- `as = Name` override for namespaced types
- Lives in `elicitation_macros` (attribute macro, per project convention)
- Target application: `elicit_clap` Select enums (5 files → 5 one-liner impls)

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

### PluginContext Refactor

**Document:** [PLUGIN_CONTEXT_REFACTOR_PLAN.md](PLUGIN_CONTEXT_REFACTOR_PLAN.md)

**Status:** 🔲 Planning

**Description:** Replace the monolithic concrete `PluginContext` struct (which accumulates
feature-gated fields from every shadow crate) with a `PluginContext` **trait**. Each shadow
crate defines its own context type. `ToolDescriptor<Ctx>` becomes generic, `ElicitPlugin`
gains `type Context: PluginContext`, and a new object-safe `ErasedElicitPlugin` blanket
impl handles type erasure at the registry boundary. `elicit_reqwest` migrates first as the
canonical example (`HttpContext { http: reqwest::Client }`); all context-free plugins get
`type Context = NoContext`.

**Phases:**

- Phase A: Core refactor in `crates/elicitation/` — trait, `NoContext`, generic `ToolDescriptor`, `ErasedElicitPlugin`
- Phase B: `elicit_reqwest` migration — `HttpContext`, typed handler signatures
- Phase C: Derive macro updates — `#[elicit_tool]` heuristic, `#[derive(ElicitPlugin)]`
- Phase D: Mechanical update of all other `elicit_*` crates to `type Context = NoContext`
- Phase E: Update `ELICIT_SQLX_PLAN.md` (SqlxContext was already the motivation)

### elicit_sqlx Shadow Crate

**Document:** [ELICIT_SQLX_PLAN.md](ELICIT_SQLX_PLAN.md)

**Status:** 🔲 Planning

**Description:** Shadow crate exposing sqlx as MCP tools for agent-driven database
programming. Mirrors sqlx's own vocabulary exactly (`AnyPool`, `AnyRow`, `AnyColumn`,
`AnyQueryResult`). Backend-agnostic via `sqlx::Any` — the connection URL selects Postgres,
SQLite, or MySQL at runtime. Covers runtime tools (5 pool methods), fragment tools (4
macros: `query!`, `query_as!`, `query_scalar!`, `migrate!`), a `FromRow` trait factory,
and Kani + Creusot verification harnesses.

**Phases:**

- Phase 1: Workspace wiring (sqlx dep + elicit_sqlx member)
- Phase 2: Elicitation primitives under `sqlx-types` feature flag
- Phase 3: `crates/elicit_sqlx/` — types, runtime workflow plugin, fragment tools
- Phase 3B: `FromRow` trait factory
- Phase 4: Kani harnesses
- Phase 5: Creusot proofs

---

### elicit_tokio Shadow Crate

**Document:** [ELICIT_TOKIO_PLAN.md](ELICIT_TOKIO_PLAN.md)

**Status:** 🔲 Planning

**Description:** Complete harvesting of the tokio 1.49.0 public API surface (100+ async
functions, 80+ types, 10+ core traits, 6 macros, 9 primary modules) as MCP tools for
agent composition. Full completionist approach exposing the entire library without filtering.

**Coverage:**

- **Runtime & Tasks:** Runtime, Builder, Handle, spawn, JoinHandle, JoinSet, LocalSet
- **Sync Primitives:** Mutex, RwLock, Semaphore, Barrier, Notify, OnceCell, all channel types
- **Channels:** oneshot, mpsc, broadcast, watch (all variants with permits/weak handles)
- **Time:** sleep, interval, timeout, Instant, test utilities
- **I/O Traits:** AsyncRead, AsyncWrite, AsyncSeek, AsyncBufRead + all extension methods
- **I/O Utilities:** BufReader, BufWriter, BufStream, copy, split, duplex, simplex, etc.
- **Filesystem:** File, OpenOptions, DirBuilder, ReadDir, all async fs functions
- **Networking:** TCP, UDP, Unix domain sockets, Windows named pipes, DNS lookup
- **Process:** Command, Child, stdio handles
- **Signals:** ctrl_c, Unix signals (all variants), Windows events
- **Platform-Specific:** Unix AsyncFd, Windows-specific APIs

**Strategy:**
- Single crate (`elicit_tokio`) with feature flags mirroring upstream
- Newtype wrappers for all types (Arc for shared ownership)
- Method reflection for all instance methods
- Trait reflection for core async traits
- Macro equivalents as runtime builders
- Kani/Creusot verification for async contracts

**Timeline:** 6 weeks, 10 phases, 300-400 MCP tools, ~15,000-20,000 LOC

---

### elicit_axum Shadow Crate

**Document:** [ELICIT_AXUM_PLAN.md](ELICIT_AXUM_PLAN.md)

**Status:** 🔲 Planning

**Description:** Complete harvesting of the axum web framework (Router, 20+ extractors,
responses, handlers, middleware, Tower integration) as MCP tools for agent composition of
web services. Three-crate architecture: `elicit_tower` (Service/Layer + 20+ middleware),
`elicit_axum_core` (FromRequest/IntoResponse traits), `elicit_axum` (Router, handlers, serve).

**Key Challenge:** Axum is trait-heavy with type-level composition - handlers inferred from
function signatures, extractors composed via type parameters. Solution: dual representation
with both **code generation tools** (emit Rust handlers/middleware) and **runtime tools**
(manipulate pre-compiled components).

**Coverage:**
- **Routing:** Router, MethodRouter, MethodFilter, nesting, merging, fallbacks
- **Extractors:** Path, Query, Json, Form, Multipart, WebSocket, State, 20+ built-ins
- **Responses:** Json, Html, Redirect, SSE, AppendHeaders, tuple responses
- **Handlers:** Handler trait, HandlerService, code generation builder
- **Middleware:** from_fn, from_extractor, map_request/response, Next, builder
- **Tower:** Service/Layer traits, 20+ tower-http middleware (CORS, compression, tracing, etc.)
- **Server:** serve(), Listener trait, graceful shutdown, IncomingStream
- **HTTP:** Full http crate re-exports (StatusCode, HeaderMap, Method, Uri)

**Strategy:**
- Three shadow crates with clear separation of concerns
- Trait reflection for FromRequest/IntoResponse/Handler/Service/Layer
- Workflow-based tool design (create service, add endpoint, apply middleware)
- Code generation for handlers and middleware (agents emit Rust code)
- Runtime composition of pre-compiled handlers
- Kani/Creusot verification for composition contracts

**Timeline:** 6 weeks, 11 phases, 400-500 MCP tools, ~20,000-25,000 LOC

---

### elicit_polars Shadow Crate

**Document:** [ELICIT_POLARS_PLAN.md](ELICIT_POLARS_PLAN.md)

**Status:** 🔲 Planning

**Description:** Pragmatic harvesting of polars DataFrame library (~70-80% of API is
JSON-serializable). Four-plugin architecture: DataFrame operations (eager), LazyFrame
query builder (lazy), Expr composition DSL, and SQL interface. Unlike closure-heavy
libraries, polars was designed for serialization with full serde support.

**Key Advantage:** Polars' `Expr` type is a **serializable AST** - agents can build
complex queries by composing JSON-serializable expressions. No code generation needed,
just data structure composition.

**Coverage:**
- **DataFrame (eager):** 40+ operations - select, filter, join, group_by, I/O (CSV/Parquet/JSON/IPC)
- **LazyFrame (lazy):** 25+ operations - scan, transform, optimize, collect, streaming
- **Expr DSL:** 30+ tools - col, lit, binary ops, aggregations, string/temporal/list methods
- **SQL Interface:** 5 tools - context management, table registration, query execution
- **Data Types:** Full dtype system (numeric, temporal, nested, categorical)

**What's Serializable:**
- ✅ All DataFrame/LazyFrame operations (params are primitives/structs)
- ✅ Expr is `#[derive(Serialize, Deserialize)]` - full AST composition
- ✅ ~200 built-in functions (sum, mean, string ops, temporal ops)
- ✅ SQL interface (string → LazyFrame)
- ✅ I/O operations (file paths + option structs)

**What's NOT (closures):**
- ❌ `df.apply(|series| custom(series))` - ~20% of API
- ❌ `expr.map(|col| custom(col))` - custom UDFs
- ❌ Object columns (require trait impls)

**Strategy:**
- UUID-keyed registries for DataFrame/LazyFrame handles
- Direct Expr serialization (agents build JSON ASTs)
- SQL as high-level escape hatch for complex queries
- Built-in function dispatcher for ~200 operations
- Arrow IPC for efficient data transfer

**Timeline:** 6 weeks, 5 phases, ~100 MCP tools, ~8,000-10,000 LOC

---

### elicit_nalgebra Shadow Crate

**Document:** [ELICIT_NALGEBRA_PLAN.md](ELICIT_NALGEBRA_PLAN.md)

**Status:** 🔲 Planning

**Description:** Completionist harvesting of nalgebra linear algebra library (480 MCP tools).
Single-crate architecture exposing matrices, vectors, geometric types (rotations, transforms),
and decompositions (SVD, QR, LU, Cholesky, eigenvalues) through dual-mode tools, fragment
tools, and UUID-keyed handles.

**Key Advantage:** "More straightforward" than other shadow crates - no macros to harvest (unlike
Leptos), no trait-heavy API requiring factory pattern (unlike num-traits), no async abstractions
(unlike Axum). Matrix/vector serialization is natural (nested JSON arrays), and ~73% of tools
are dual-mode (both runtime execution and code emission).

**Coverage:**
- **Matrix Operations (120 dual-mode):** Creation, arithmetic, transformations, slicing, properties, solvers, norms
- **Vector Operations (80 dual-mode):** Creation, arithmetic, geometric (dot, cross, normalize), properties
- **Geometric Types (80 dual-mode):** Rotations (2D/3D, quaternions, Euler angles), translations, isometries, similarities, transforms, projections
- **Decompositions (70 dual-mode):** SVD, QR, LU, Cholesky, Schur, symmetric eigenvalue
- **Fragment Tools (70):** Generic dimension code (const generics), scalar type code (RealField/ComplexField), complete assembly
- **Runtime Handles (60):** UUID registries for persistent matrices, vectors, decompositions, transforms

**Strategy:**
- Single shadow crate: `elicit_nalgebra`
- Dual-mode dominance: 350/480 tools (73%) with `emit = Auto` + CustomEmit
- Natural JSON serialization: matrices → nested arrays, vectors → arrays, rotations → quaternions
- UUID-keyed handles for stateful workflows and decomposition chains
- Fragment tools for generic const dimension code (`SMatrix<T, R, C>`)
- Minimal factory pattern (deferred unless needed for RealField/ComplexField traits)

**Timeline:** 7 phases, 480 MCP tools

---

### elicit_ndarray Shadow Crate

**Document:** [ELICIT_NDARRAY_PLAN.md](ELICIT_NDARRAY_PLAN.md)

**Status:** 🔲 Planning

**Description:** Completionist harvesting of ndarray N-dimensional array library (520 MCP tools).
Single-crate architecture exposing array creation, indexing, slicing, arithmetic, broadcasting,
aggregations, linear algebra, and manipulation operations through dual-mode tools, fragment
tools, and UUID-keyed handles.

**Key Advantage:** "Similar but also widely used" — shares nalgebra's straightforward characteristics
(natural JSON serialization, synchronous operations, concrete methods) but focuses on general
N-dimensional arrays like NumPy rather than linear algebra. Wider adoption as the foundation of
Rust's scientific computing ecosystem (ndarray-linalg, ndarray-stats, polars).

**Coverage:**
- **Array Creation (60 dual-mode):** From data, ranges, special values (zeros/ones/eye), random, iterators
- **Indexing & Slicing (50 dual-mode):** Element access, slicing with s! macro, views, iteration
- **Arithmetic (50 dual-mode):** Element-wise binary/unary ops, scalar ops, comparisons, logical
- **Broadcasting (30 dual-mode):** Automatic broadcasting, manual broadcast, shape operations
- **Aggregations (40 dual-mode):** Full reductions (sum, mean, var, std), axis reductions, cumulative
- **Linear Algebra (40 dual-mode):** Matrix ops (dot, transpose), norms (references ndarray-linalg for SVD/QR)
- **Manipulation (60 dual-mode):** Concatenation, splitting, reshape, axis ops, flipping, cloning
- **I/O (30 dual-mode):** CSV, binary serialization, display formatting
- **Fragment Tools (80):** Generic dimension code (Dimension trait), parallel operations (rayon), broadcasting, assembly
- **Runtime Handles (40):** UUID registries for persistent arrays (ArcArray), views, iterators

**Strategy:**
- Single shadow crate: `elicit_ndarray`
- Dual-mode dominance: 400/520 tools (77%) with `emit = Auto` + CustomEmit
- Natural JSON serialization: arrays → nested arrays, shape metadata, row-major layout
- Broadcasting semantics: Automatic shape alignment (like NumPy)
- Zero-copy views: UUID handles for efficient slicing workflows
- Parallel operations: Fragment tools generate rayon code
- Generic dimensions: Support both static (Ix1, Ix2, Ix3) and dynamic (IxDyn)
- NumPy compatibility: Familiar API for Python → Rust migrations

**Comparison to nalgebra:**
- nalgebra: Linear algebra focus, geometric types (rotations, quaternions), decompositions
- ndarray: General N-D arrays, broadcasting, NumPy-style API, scientific computing foundation
- Both "straightforward": Natural serialization, synchronous ops, concrete methods, clear taxonomy

**Timeline:** 7 phases, 520 MCP tools

---

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
