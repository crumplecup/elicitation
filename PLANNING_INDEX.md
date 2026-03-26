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

### elicit_taffy Shadow Crate

**Document:** [ELICIT_TAFFY_PLAN.md](ELICIT_TAFFY_PLAN.md)

**Status:** 🔲 Planning

**Description:** Completionist harvesting of taffy flexbox/grid/block layout engine (340 MCP tools).
Single-crate architecture exposing tree management, style properties, layout computation, and code
generation through runtime-only tools (stateful tree), dual-mode tools (style creation), and
fragment tools (layout code gen).

**Key Advantage:** Straightforward like nalgebra/ndarray (natural serialization, synchronous, no
lifetimes) but unique in being **stateful by design** (tree of nodes, not pure functions). Smaller
API surface (340 tools) focused on CSS layout algorithms. Foundation for UI libraries (bevy_ui,
dioxus, xilem, cosmic).

**Coverage:**
- **Runtime Tree Management (180 tools):** Tree creation/deletion, node lifecycle, hierarchy manipulation, style setters, layout computation, context/measurement, tree traversal
- **Dual-Mode Style Tools (120 tools):** Style builders (flexbox, grid, block), dimension types (length, percent, auto), spacing (margin, padding, border, gap), alignment/distribution, grid templates, layout serialization
- **Fragment Tools (40 tools):** Style code generation, tree construction code, layout computation code, complete assembly

**Strategy:**
- Single shadow crate: `elicit_taffy`
- Runtime-heavy: 180/340 tools (53%) are runtime-only due to stateful TaffyTree
- UUID-keyed registry: TaffyTree instances mapped to UUIDs, internal NodeId ↔ external UUID translation
- Dual-mode style tools: 120/340 (35%) with `emit = Auto` + CustomEmit for Style construction
- Fragment tools: 40/340 (12%) for generating complete layout code
- Natural JSON serialization: Style properties → JSON, Layout results → JSON, Tree structure → UUID handles

**CSS Layout Algorithms:**
- Flexbox: Single-axis flexible layouts (main/cross axis, wrapping, alignment)
- CSS Grid: Two-dimensional grid systems (template rows/columns, auto-placement, grid areas)
- Block: Traditional document flow layout (text alignment, inline-block)

**Comparison to nalgebra/ndarray:**
- **Shared "straightforward" traits:** Natural JSON serialization, synchronous operations, no async/lifetimes/closures, clear API flow
- **Unique characteristic:** Stateful by design (tree mutations) vs stateless math (pure functions)
- **Tool distribution:** Runtime 53% (vs 0% for nalgebra/ndarray), Dual-mode 35% (vs 73-77%), Fragment 12%
- **API size:** 340 tools (smaller, domain-specific) vs 480-520 (general math)
- **Use case:** UI layout for frameworks vs scientific/graphics computing

**Integration:**
- bevy_ui: AI-driven UI layout generation, responsive design composition
- dioxus: Component layout optimization, pattern libraries
- xilem: Declarative layout DSL generation, snapshot testing

**Timeline:** 6 phases, 340 MCP tools

---

### elicit_parley Shadow Crate

**Document:** [ELICIT_PARLEY_PLAN.md](ELICIT_PARLEY_PLAN.md)

**Status:** 🔲 Planning

**Description:** Completionist harvesting of parley rich text layout and shaping library (380 MCP tools).
Single-crate architecture exposing font context, layout building, text shaping, line breaking,
bidirectional text handling, and glyph positioning through runtime-only tools (stateful contexts),
dual-mode tools (style/layout), and fragment tools (text layout code gen).

**Key Advantage:** Straightforward like taffy (stateful, CSS-like properties, visual domain) but focuses
on **text layout and typography** rather than box layout. Full typography stack: HarfBuzz shaping,
OpenType features/variations, bidirectional text (RTL/LTR), Unicode line breaking, kerning, ligatures.
Foundation for linebender ecosystem (xilem, masonry, vello).

**Coverage:**
- **Runtime Context Management (160 tools):** FontContext/LayoutContext creation, builder workflow (one-shot operations due to lifetimes), layout operations (break lines, align), line/run/glyph queries, font database inspection
- **Dual-Mode Style Tools (180 tools):** 23 StyleProperty variants (font family, size, weight, style, variations, features, underline, strikethrough, line height, letter spacing, word spacing, locale, brush/color), text ranges, alignment types, line break rules, layout serialization (positioned glyphs with coordinates)
- **Fragment Tools (40 tools):** Context construction code, builder code generation, layout computation code, complete assembly

**Strategy:**
- Single shadow crate: `elicit_parley`
- Runtime-heavy: 160/380 tools (42%) due to stateful contexts + lifetime-bound builders
- Dual-mode emphasis: 180/380 (47%) for style creation and layout serialization
- UUID-keyed registry: FontContext/LayoutContext → UUIDs, builders are one-shot (create → use → build → delete)
- Natural JSON serialization: StyleProperty → JSON, Layout output → positioned glyphs with XY coordinates
- SimpleBrush: Fixed RGBA color type (avoids generic Brush trait complexity)

**Typography Features:**
- **Text Shaping:** HarfBuzz integration, glyph positioning, kerning, ligatures
- **OpenType:** Font features (kern, liga, calt, etc.), variation axes (weight, width)
- **Bidirectional:** RTL/LTR text, Arabic, Hebrew, Unicode normalization
- **Line Breaking:** Unicode line break algorithm, word/character breaking, emergency breaking
- **Font Control:** Font family stacks, weight (100-900), style (normal/italic/oblique), stretch

**Comparison to taffy:**
- **Shared traits:** Stateful by design, CSS-like properties, synchronous operations, natural JSON serialization, visual domains
- **Different domains:** Box layout (flexbox/grid) vs text layout (shaping/breaking)
- **Output:** Box positions (x/y/width/height) vs glyph positions (x/y/advance/cluster)
- **Complexity:** CSS layout algorithms vs typography (HarfBuzz, OpenType, bidi)
- **Tool distribution:** taffy runtime 53%, parley runtime 42%; taffy dual-mode 35%, parley dual-mode 47%

**Integration:**
- xilem/masonry: AI-generated rich text layouts, typography exploration
- vello: GPU-rendered text generation, vector text for design tools
- Custom renderers: PDF generation, canvas rendering, game engine text, terminal UI

**Timeline:** 6 phases, 380 MCP tools

---

### elicit_accesskit Shadow Crate

**Document:** [ELICIT_ACCESSKIT_PLAN.md](ELICIT_ACCESSKIT_PLAN.md)

**Status:** 🔲 Planning

**Description:** Completionist harvesting of accesskit UI accessibility infrastructure (450 MCP tools).
Single-crate architecture exposing accessibility tree construction (182 semantic roles, 22 actions,
50+ node properties) through dual-mode tools (primary), runtime-only tools (tree state), and
fragment tools (accessibility code gen).

**Key Advantage:** Unique among shadow crates — **pure data schema** with no computation (unlike
nalgebra math, taffy/parley layout/shaping). Highest dual-mode ratio (87% vs 35-47% for others)
because entire accessibility tree serializes to JSON. Platform abstraction for screen readers
(VoiceOver, JAWS, NVDA, Narrator). Foundation for accessible UI frameworks (xilem, egui, dioxus).

**Coverage:**
- **Dual-Mode Tools (390 tools, 87%):** Node creation (30), node properties (215 across content/structure/states/relationships/text/table/list), actions (30), roles (182 variants - code generated), tree management (20), serialization (10)
- **Runtime Tools (30 tools, 7%):** Tree registry (UUID → Tree/Node mapping), platform integration (screen reader bridges)
- **Fragment Tools (30 tools, 7%):** Node construction code, tree construction code, complete assembly

**Strategy:**
- Single shadow crate: `elicit_accesskit`
- Dual-mode dominated: 390/450 (87%) due to pure data schema (everything serializes)
- Minimal runtime: 30/450 (7%) for optional tree state + platform bridges
- Natural JSON serialization: Node/Tree/TreeUpdate → JSON, entire accessibility tree is data
- 182 role tools (code generated): One tool per Role variant (Button, TextInput, Table, Grid, etc.)
- Complete semantic vocabulary: 22 actions, 50+ properties, relationships, states

**Accessibility Features:**
- **182 Semantic Roles:** Button, TextInput, Checkbox, Slider, Table, Grid, List, Heading, Link, Image, etc.
- **22 Actions:** Click, Focus, SetValue, ScrollIntoView, Increment, Decrement, etc.
- **50+ Properties:** name, value, description, bounds, states (disabled/selected/expanded), relationships (labelled_by, described_by, controls), text (font, color, alignment), table (row/column indices)
- **ARIA Support:** aria-current, aria-live, aria-invalid, aria-orientation
- **Platform Integration:** Cross-platform (Windows/macOS/Linux) screen reader support

**Comparison to taffy/parley:**
- **Shared traits:** Natural JSON serialization, synchronous operations, visual domain
- **Unique characteristic:** Pure data (no computation) vs computational (layout algorithms, text shaping)
- **Tool distribution:** accesskit dual-mode 87% (highest), taffy 35%, parley 47%; accesskit runtime 7% (lowest), taffy 53%, parley 42%
- **Purpose:** Semantic metadata (what UI means) vs layout/typography (how UI looks)

**Use Cases:**
- AI accessibility auditing (check for missing labels, WCAG compliance)
- Automated testing (simulate screen reader navigation, verify keyboard support)
- Semantic-first UI generation (describe intent → generate accessible structure)
- Documentation generation (extract UI structure from accessibility tree)
- Compliance checking (WCAG 2.1, Section 508)

**Integration:**
- xilem: AI-generated accessible UIs, automated audits
- egui: Accessibility layer generation, screen reader testing
- dioxus: Accessible component generation, semantic HTML mapping

**Timeline:** 8 phases, 450 MCP tools

---

### elicit_egui Shadow Crate

**Document:** [ELICIT_EGUI_PLAN.md](ELICIT_EGUI_PLAN.md)

**Status:** 🔲 Planning

**Description:** Completionist harvesting of egui immediate mode GUI framework (420 MCP tools).
Single-crate architecture exposing widget creation (50+ widget types), containers (windows, panels,
scroll areas), layout management, styling, response checking, and complete app generation through
dual-mode tools (primary), runtime-only tools (UI display - feature-gated), and fragment tools
(app code gen).

**Key Advantage:** Unique as a **complete GUI framework** (not just one piece like layout/text/accessibility).
**Immediate mode** pattern (stateless widgets recreated every frame, not retained mode). High dual-mode
ratio (81%) because widget descriptions serialize well. Feature-gated runtime (can run headless for
code gen, or with eframe for display). Cross-platform (native Windows/macOS/Linux + web WASM).

**Coverage:**
- **Dual-Mode Tools (340 tools, 81%):** Basic widgets (50: button, label, checkbox, link, image, separator, spinner), text input (20: singleline, multiline, code editor), numeric (30: slider, drag value, angle), color (15: sRGB, HSV, RGB pickers), progress (10: progress bar, spinner), containers (40: window, panel, scroll area, collapsing, group), layout (35: horizontal, vertical, grid, spacing, indent), grid (20: columns, striped, spacing), styling (40: visuals, text styles, colors, rounding), response (30: clicked, hovered, dragged, focus), menus/tooltips (20: context menu, hover text), input/events (10: key press, modifiers, pointer)
- **Runtime Tools (50 tools, 12%):** Context management (15: create, begin/end frame, run), app state registry (20: persistent state between frames), platform integration (15: eframe native, egui_web, clipboard) — all feature-gated behind `runtime` feature
- **Fragment Tools (30 tools, 7%):** Widget code generation (10), container code generation (10), complete app assembly (10: eframe native app, web app, app struct, update method)

**Strategy:**
- Single shadow crate: `elicit_egui`
- Dual-mode dominated: 340/420 (81%) for widget/container/layout descriptions
- Feature-gated runtime: 50/420 (12%) behind `runtime` feature (requires eframe for actual UI display)
- Fragment tools for complete apps: Generate full eframe native/web applications from UI trees
- Natural JSON serialization: Widget descriptions → JSON (button text, slider range, window size)
- Immediate mode pattern: UI rebuilt every frame, user manages state between frames

**Immediate Mode Characteristics:**
- **Stateless widgets:** No persistent button objects, only `Response` output per frame
- **User-managed state:** Application stores values (counters, text) between frames
- **~60 FPS rebuild:** Entire UI recreated every frame
- **Simple mental model:** Imperative API (function calls), not declarative markup
- **Response-driven:** Every widget returns interaction state (clicked, hovered, dragged)

**Comparison to Linebender Stack:**
- **egui:** All-in-one immediate mode framework (complete UIs, tools, editors, dashboards)
- **Linebender (taffy + parley + accesskit + vello):** Composable pieces (custom renderers, game engines, specialized UIs)
- **Both support accesskit:** egui has built-in support, Linebender uses it directly
- **Different philosophies:** egui immediate (rebuild every frame) vs linebender retained (update tree)

**Comparison to Other Shadow Crates:**
- **taffy:** Layout engine only (box positioning) vs egui complete framework (widgets + layout + rendering)
- **accesskit:** Accessibility metadata only vs egui includes accessibility support
- **Tool distribution:** egui dual-mode 81% (similar to accesskit 87%), runtime 12% (lower than taffy 53%)
- **Purpose:** egui is end-to-end (complete apps) vs others are components (one piece of the stack)

**Use Cases:**
- AI-generated UIs from natural language (agent creates settings panel, dashboard, form)
- Interactive tools (database query builder, log viewer, config editor, asset browser)
- Dashboard generation (monitoring, metrics, system status, alerts)
- Form builders (data entry UIs from schemas: registration, surveys, admin panels, CRUD)
- Debug UIs (inspection panels, profilers, diagnostic tools)
- Rapid prototyping (quick UI iteration without external dependencies)

**Integration:**
- Pure Rust, no external dependencies (no OS widgets required)
- Cross-platform: Native (via eframe) + Web (via egui_web/eframe web)
- Optional accesskit integration for screen reader support
- Can combine with vello for custom graphics overlays

**Timeline:** 8 phases, 420 MCP tools

---

### elicit_ratatui Shadow Crate

**Document:** [ELICIT_RATATUI_PLAN.md](ELICIT_RATATUI_PLAN.md)

**Status:** 🔲 Planning

**Description:** Completionist harvesting of ratatui terminal UI framework (380 MCP tools).
Single-crate architecture exposing widget creation (Block, Paragraph, List, Table, Chart, Gauge,
BarChart, Sparkline, Calendar, Tabs, Scrollbar), constraint-based layout, styling (colors, modifiers),
and complete TUI app generation through dual-mode tools (primary), runtime-only tools (TUI display -
feature-gated), and fragment tools (TUI app code gen).

**Key Advantage:** **Terminal-based immediate mode GUI** (similar to egui but outputs ANSI/text instead
of pixels). High dual-mode ratio (84%) because widget descriptions serialize well. SSH-friendly,
low bandwidth (~20 MB RAM), remote admin perfect. Feature-gated runtime (crossterm/termion/termwiz
backends). Works in tmux/screen, over SSH, no X11 needed.

**Coverage:**
- **Dual-Mode Tools (320 tools, 84%):** Core widgets (30: Block, Paragraph, List, Table, Chart, BarChart, Sparkline, Gauge, Tabs, Scrollbar, Calendar), widget properties (190: Block borders/title/padding, Paragraph text/wrap/scroll, List items/highlight, Table rows/columns/widths, Chart datasets/axes, Gauge progress/label), layout (30: vertical/horizontal splits, constraints: Length/Percentage/Ratio/Min/Max/Fill), styling (30: foreground/background colors, modifiers: Bold/Italic/Underlined/Dim/Reversed), text types (20: spans, lines, styled text)
- **Runtime Tools (40 tools, 11%):** Terminal management (15: create, draw, clear, cursor, size), app state registry (15: persistent TUI state, ListState, TableState, ScrollbarState), event handling (10: key press, mouse, resize) — all feature-gated behind `runtime` feature with backend choice (crossterm default, termion, termwiz)
- **Fragment Tools (20 tools, 5%):** Widget code generation (10), complete TUI app assembly (10: generate main loop, event handler, app struct, draw method)

**Strategy:**
- Single shadow crate: `elicit_ratatui`
- Dual-mode dominated: 320/380 (84%) for widget/layout descriptions
- Feature-gated runtime: 40/380 (11%) behind `runtime` feature + backend selection
- Backend abstraction: crossterm (default, cross-platform), termion (Unix-only), termwiz (alternative)
- Fragment tools for complete TUIs: Generate full TUI apps with event loops from UI trees
- Natural JSON serialization: Widget descriptions → JSON (table rows, chart data, constraint specs)
- Immediate mode pattern: UI rebuilt every frame (~60 FPS), user manages state between frames

**Terminal UI Characteristics:**
- **Text/ANSI output:** 80×24 character grid (or larger), ANSI escape codes for colors/styles
- **Low bandwidth:** SSH-friendly, works over slow connections
- **No X11 required:** Pure terminal, runs in console, SSH sessions
- **Immediate mode:** Like egui (rebuild every frame), not retained mode
- **Constraint layout:** Responsive TUIs that adapt to terminal size (Layout with Percentage/Ratio/Fill)
- **Rich widgets:** Tables with headers/selection, line charts, bar charts, gauges, calendars
- **Backend agnostic:** crossterm (Linux/macOS/Windows), termion (Unix), termwiz

**Comparison to egui:**
- **Shared:** Both immediate mode, user-managed state, high dual-mode ratio (81% vs 84%), response checking
- **Different output:** egui pixels (GUI windows, web canvas) vs ratatui chars (terminal, ANSI)
- **Different widgets:** egui buttons/sliders/color pickers vs ratatui blocks/tables/charts/gauges
- **Use cases:** egui desktop apps/tools vs ratatui CLI tools/SSH/remote admin

**Use Cases:**
- System monitors (htop-style CPU/memory, process tables, real-time charts)
- Log viewers (tail -f with filters, search/highlight, level filtering)
- Database clients (table browsers, query editors, result tables)
- Development tools (test runners, build monitors, progress tracking)
- SSH tools (remote server admin, configuration editors)
- Interactive CLIs (setup wizards, TUI menus, selection UIs)

**Terminal Integration:**
- Works over SSH (no X11 forwarding needed)
- Compatible with tmux/screen (multiple TUI apps in panes)
- Shell integration (TUI apps as CLI commands with arguments)
- Low resources (production apps <20 MB RAM even with complex UIs)

**Timeline:** 7 phases, 380 MCP tools

---

### Ledger Workflow Test (End-to-End Validation)

**Document:** [LEDGER_WORKFLOW_TEST_PLAN.md](LEDGER_WORKFLOW_TEST_PLAN.md)

**Status:** 🔲 Planning (Phase 1 Ready to Implement)

**Description:** End-to-end validation that agent workflow composition → emit_binary → compiled
executable works correctly. Uses a double-entry ledger as the test domain because it requires
transactions, demonstrates contract composition, and has meaningful error cases (unlike a simple
todo app).

**Why a Ledger:**
- Transactions mandatory (debit + credit must balance atomically)
- Contract composition: `And<DbConnected, And<TransactionOpen, TransactionCommitted>>`
- Meaningful errors (insufficient funds, negative amounts, concurrent races)
- Complex queries (balance = SUM aggregation)
- Relatable domain (everyone understands money)

**Test Architecture:**
1. Agent composes workflow (13 tools: sqlx + tokio_net)
2. emit_binary generates code (BinaryScaffold → main.rs + Cargo.toml)
3. cargo build --release (compiles generated code)
4. Run binary + validate with reqwest (HTTP server responds correctly)

**Phases:**
- Phase 1: Smoke test (hardcoded transfer, manual HTTP strings, SQLite in-memory)
- Phase 2: Balance query endpoint (SQL aggregation, read path)
- Phase 3: Dynamic transfers (JSON parsing, parameterized queries)
- Phase 4: Contract types (ValidatedTransfer, amount > 0, sufficient funds)
- Phase 5: Typestate state machine (Transfer<Pending> → <Validated> → <Committed>)
- Phase 6: Concurrent transfers (proper transaction isolation, race conditions)

**Success Criteria:**
- ✅ Generated code compiles without errors
- ✅ Binary runs and accepts TCP connections
- ✅ Transactions commit successfully
- ✅ HTTP responses validated via reqwest
- ✅ Test passes in CI

**Location:** `crates/elicit_server/tests/ledger_*.rs`

**Progress:**
- ✅ Phase 1 (Smoke): Basic emit pipeline validation - COMPLETE
- ✅ Phase 2 (Query): SQL aggregation and balance queries - COMPLETE
- ✅ Phase 3 (Dynamic): Parameterized queries with runtime binding - COMPLETE
- ✅ Phase 4 (Contracts): Pre-transfer validation pattern - COMPLETE
- 🚧 Phase 5 (Typestate): State machine implementation - IN PROGRESS
  - ✅ Phase 5a: Typestate types and validation functions - COMPLETE
  - 🔲 Phase 5b: Workflow code generation integration
  - 🔲 Phase 5c: End-to-end typestate test
- 🔲 Phase 6 (Concurrent): Transaction isolation under load

---

### Typestate Ledger Design

**Document:** [TYPESTATE_LEDGER_DESIGN.md](TYPESTATE_LEDGER_DESIGN.md)

**Status:** 🚧 Implementation In Progress

**Description:** Design document for implementing a double-entry ledger using elicitation
framework's typestate state machines with proof-carrying contracts. Builds on the proven
emit pipeline from Phases 1-4, following patterns from strictly_games/tictactoe.

**Implementation Location:** `crates/elicit_server/src/ledger/` (moved from elicitation core to avoid circular dependencies)

**Design Principles:**
- **Typestate phases**: `Transfer<Pending>` → `Transfer<Validated>` → `Transfer<Committed>` / `Transfer<Rejected>`
- **Propositions**: `AmountPositive`, `SufficientFunds`, `AccountsDistinct`, `BalancedEntries`
- **Composite props**: `ValidTransfer = And<AmountPositive, And<SufficientFunds, AccountsDistinct>>`
- **Validation functions**: Return `Established<Prop>` on success, error otherwise
- **Proof-carrying execution**: `commit()` requires `Established<ValidTransfer>` proof
- **Transitions consume self**: `pending.validate()` consumes and returns `Validated`

**Key Features:**
- Compile-time guarantees (can't commit without validation)
- Zero-cost proofs (`Established<P>` is `PhantomData`)
- Compositional verification (Kani checks proof composition)
- Type-driven design (state machine encoded in types)
- Audit trail (each state captures relevant data)
- Integration with sqlx_workflow tools (proven in Phases 1-4)

**Proof Composition:**
```rust
// Level 1: Basic propositions
AmountPositive, SufficientFunds, AccountsDistinct

// Level 2: Valid transfer
ValidTransfer = And<AmountPositive, And<SufficientFunds, AccountsDistinct>>

// Level 3: Committable transfer
CommittableTransfer = And<ValidTransfer, TransactionOpen>

// Level 4: Committed transfer
CommittedTransfer = And<CommittableTransfer, BalancedEntries>
```

**Implementation Plan:**
- Phase 5a: Typestate types & validation (AccountId, Amount, Transfer<S>, propositions)
- Phase 5b: Commit logic & workflow integration (emit test with typestate)

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
