# Planning Documents

This file tracks all planning documents for the elicitation project.

## Archive Notice — v0.10.0

**All planning documents archived as of v0.10.0** (see last-commit hashes below)

All planning documents from the 0.8–0.10 development cycle have been deleted
from the working tree. Their complete content is preserved in git history.
To retrieve any document:

```bash
git show <last-commit>:<filename>
```

| Document | Last Commit |
|----------|-------------|
| ANODIZED_SPEC_EXPLORER_PLAN.md | `cc6ce2cf` |
| BUG_EMPTY_PROOF_UNIT_VARIANT_ENUM.md | `8385292a` |
| CONTRACT_PARAMS_PLAN.md | `cc6ce2cf` |
| ELICIT_ACCESSKIT_PLAN.md | `5f71e320` |
| ELICIT_AXUM_PLAN.md | `de26d43e` |
| ELICIT_EGUI_PLAN.md | `8a5aac05` |
| ELICIT_GEOJSON_PLAN.md | `7c9af6ae` |
| ELICIT_GEO_PLAN.md | `7c9af6ae` |
| ELICIT_GEORASTER_PLAN.md | `62a2acdc` |
| ELICIT_GEO_TYPES_PLAN.md | `7c9af6ae` |
| ELICIT_LEPTOS_PLAN.md | `375363ae` |
| ELICIT_NALGEBRA_PLAN.md | `375363ae` |
| ELICIT_NDARRAY_PLAN.md | `375363ae` |
| ELICIT_NUM_PLAN.md | `de26d43e` |
| ELICIT_PARLEY_PLAN.md | `bacb7c13` |
| ELICIT_POLARS_PLAN.md | `de26d43e` |
| ELICIT_PROJ_PLAN.md | `62a2acdc` |
| ELICIT_RATATUI_PLAN.md | `5f71e320` |
| ELICIT_REQWEST_PLAN.md | `cc6ce2cf` |
| ELICIT_RSTAR_PLAN.md | `7c9af6ae` |
| ELICIT_SQLX_PLAN.md | `cc6ce2cf` |
| ELICIT_TAFFY_PLAN.md | `ffc83d50` |
| ELICIT_TOKIO_PLAN.md | `93e6a83d` |
| ELICIT_UI_GEORUST_PLAN.md | `b98d1917` |
| ELICIT_WGPU_PLAN.md | `951a0629` |
| ELICIT_WINIT_PLAN.md | `66369ef0` |
| ELICIT_WKB_PLAN.md | `62a2acdc` |
| ELICIT_WKT_PLAN.md | `62a2acdc` |
| EMIT_AUTODERIVE_PLAN.md | `2ada0a01` |
| FORMAL_VERIFICATION_LEGOS.md | `b6f89040` |
| LEDGER_WORKFLOW_TEST_PLAN.md | `61b6f39d` |
| MACRO_TOOL_GEN_PLAN.md | `cc6ce2cf` |
| METHOD_REFLECTION_PLAN.md | `cc6ce2cf` |
| PLUGIN_CONTEXT_REFACTOR_PLAN.md | `cc6ce2cf` |
| PROMPT_TREE_PLAN.md | `8385292a` |
| PROOF_EMISSION_PLAN.md | `cc6ce2cf` |
| REFLECT_TRAIT_PLAN.md | `cc6ce2cf` |
| TYPE_GRAPH_PLAN.md | `90383302` |
| TYPESTATE_LEDGER_DESIGN.md | `fa85d9e2` |
| TYPESTATE_UI_DESIGN.md | `2089cc81` |

---

## Archive Notice — v0.7.0

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

### GAAP Ledger Integration

**Document:** [GAAP_LEDGER_INTEGRATION.md](GAAP_LEDGER_INTEGRATION.md)

**Status:** ✅ Complete (Phases 1-3, 5)

**Description:** Applying Generally Accepted Accounting Principles (GAAP) to the typestate
ledger system through proof-carrying propositions with formal spec references. Mirrors the
WCAG constraint pattern from `elicit_ui` - where UI verification references WCAG standards,
ledger verification will reference FASB ASC (Accounting Standards Codification).

**Completed:** 9 GAAP proposition types (P0/P1/P2 priority levels), validation functions,
comprehensive test suite (39 tests), research document. Phase 4 (Transfer integration)
deferred - validation functions work independently and can be integrated when needed.

**Key Deliverables:**
- ✅ Research document: GAAP principles applicable to double-entry bookkeeping
- ✅ `src/ledger/gaap.rs`: Proposition types with ASC references (887 lines)
- ✅ Validation functions returning `Result<Established<P>, ValidationError>`
- ⏸️ Integration with `Transfer<T>` typestate workflow (deferred)
- ✅ Composite proof types demonstrated in tests
- ⏸️ User guide (inline documentation is comprehensive)

**Commit:** 2d64d721

---

### GAAP-Native Ledger Architecture

**Document:** [GAAP_NATIVE_LEDGER.md](GAAP_NATIVE_LEDGER.md)

**Status:** 📋 Planning - Ready for Review

**Description:** Evolution from POC ledger to production-grade accounting system where
**GAAP is the IR** (Intermediate Representation). Account types (Asset/Liability/Equity/
Revenue/Expense) are primitive, not validation layers. Typesafe state machines constrain
financial transactions to provable transitions. Building broad foundation for full
accounting services: chart of accounts, financial statements, period closing, multi-entity,
AI-assisted classification, audit trail.

**Vision:**
1. **GAAP as IR** - Account classes are foundational types, not post-hoc validation
2. **Typesafe state machines** - `JournalEntry<Draft/Balanced/Posted/Closed>`
3. **Double-entry by construction** - Builder pattern enforces balance before creation
4. **Matching principle by design** - Revenue recognition types enforce temporal matching
5. **AI-assisted accounting** - Classify transactions, suggest journal entries, match expenses
6. **Audit-ready** - Every entry carries GAAP proof from construction through commit

**Architecture:**
- Chart of Accounts with GAAP account types (Asset/Liability/Equity/Revenue/Expense)
- Journal Entry builder enforcing double-entry balance
- Financial statements by projection (Balance Sheet, Income Statement, Cash Flow)
- Period closing with retained earnings calculation
- Multi-entity support with consolidation
- AI-powered transaction classification (ASC 606 revenue recognition)
- Complete audit trail with GAAP compliance metadata

**Timeline:** 20 weeks (5 months), 10 phases

**Migration:** Coexist with POC ledger (`ledger` vs `ledger2`), migrate incrementally,
adapter pattern for `Transfer → JournalEntry` conversion

**Next Step:** Architecture review and approval, then begin Phase 1 (GAAP Types foundation)

---

### Visualization Guide

**Document:** [VISUALIZATION_GUIDE.md](VISUALIZATION_GUIDE.md)

**Status:** ✅ Complete

**Description:** Unified reference tying together all visualization components:
the structural type graph, the prompt tree, the assembled-prompt flat view, and
the AccessKit bridge. Explains how the three views interlock, which feature flag
enables each, and when to reach for each API.
