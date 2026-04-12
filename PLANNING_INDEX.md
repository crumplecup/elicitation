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

### archive — Feature Parity with pgAdmin / DBeaver

**Document:** [ARCHIVE_PARITY_PLAN.md](ARCHIVE_PARITY_PLAN.md)

**Status:** 🔲 Planning

**Description:** Roadmap to bring the `archive` pgAdmin-style database browser
from MVP to full feature parity with pgAdmin 4, DBeaver CE, TablePlus, and
DataGrip.  Six phases: interactive data browsing (data grid, SQL editor, live
refresh, object search), rich object inspection (DDL, FK relationships,
constraints, column stats, EXPLAIN), power-user features (row editing, query
history, saved queries, export, multi-connection), Postgres-specific object
types (functions, triggers, sequences, enums, extensions), monitoring /
administration dashboard, and ERD diagram generation.

**Architecture principle:** All actions flow through `Established<P>` proof
chains via `ArchiveDbBackend` traits. All three frontends (ratatui, browser,
egui) consume the same `ArchiveNavModel` + descriptor types via the
AccessKit IR layer.

**Phases:**
- **Phase 1 — Interactive data browsing:** data grid, SQL editor, live refresh, object search
- **Phase 2 — Rich object inspection:** DDL viewer, FK descriptors, constraints, column stats, EXPLAIN
- **Phase 3 — Power-user:** row edit, query history, saved queries, CSV/JSON export, multi-connection
- **Phase 4 — Advanced PG object types:** functions, triggers, sequences, enums, extensions
- **Phase 5 — Monitoring & admin:** live activity, role matrix, backup status, server settings
- **Phase 6 — ERD diagram:** petgraph-based layout, SVG export, interactive egui/browser render

---

### elicit_geo_types Shadow Crate

**Document:** [ELICIT_GEO_TYPES_PLAN.md](ELICIT_GEO_TYPES_PLAN.md)

**Status:** 🔲 Planning

**Description:** Complete the geo-types elicitation vocabulary (3 of 12 types done) and ship
the `elicit_geo_types` shadow crate. The remaining 9 types (`Point`, `Triangle`, `LineString`,
`Polygon`, `MultiPoint`, `MultiLineString`, `MultiPolygon`, `GeometryCollection`, `Geometry`
enum) all compose cleanly from existing primitives: variable-length types delegate to
`Vec<T>::elicit()`, `Polygon` is a two-field Survey, `Geometry` is a Select over variants.

**Key Insight:** `Vec<T: Elicitation>` already has a full bool-gated loop impl — `LineString`,
`MultiPoint`, etc. are essentially free once their inner types are elicitable. No registry
pattern needed; pure value-type composition throughout.

**Coverage:**
- **Phase 2 (elicitation):** 9 new primitives in `crates/elicitation/src/primitives/geo_types/`
- **Phase 4 (shadow crate):** `elicit_newtype!` + `reflect_methods` for all 12 types
- **Phase 5 (MCP tools):** ~32 tools across 4 plugins (primitives, shapes, collections, geometry)
- **Phase 6 (verification):** Kani roundtrip harnesses, Creusot trusted constructors, Verus structural proofs

**Foundation for:** `elicit_geo` (algorithms), `elicit_geojson`, `elicit_wkt`, `elicit_wkb`,
`elicit_georaster`, `elicit_rstar` — all depend on this vocabulary.

---

### elicit_geojson Shadow Crate

**Document:** [ELICIT_GEOJSON_PLAN.md](ELICIT_GEOJSON_PLAN.md)

**Status:** 🔲 Planning

**Description:** Faithful shadow of the `geojson` crate’s actual document/value API:
`GeoJson`, `Geometry`, `Value`, `Feature`, `FeatureCollection`, and
`feature::Id`, plus explicit `geo-types` conversion tools. This is a document-format
vocabulary, not a custom layout/export framework.

**Key Insight:** GeoJSON should sit beside `geo_types`, `wkt`, and `wkb` as a
serialization/document alphabet. The shadow crate should mirror upstream parse,
display, constructor, property, and conversion behavior rather than inventing a
parallel model.

**Coverage:**
- **Phase 2 (elicitation):** feature-gated support for upstream GeoJSON document/value types
- **Phase 3 (shadow crate):** `elicit_geojson` wrappers + upstream-shaped workflow plugins
- **Phase 4 (verification):** Kani / Creusot / Verus wrapper proofs and runner wiring

---

### elicit_georaster Shadow Crate

**Document:** [ELICIT_GEORASTER_PLAN.md](ELICIT_GEORASTER_PLAN.md)

**Status:** 🔲 Planning

**Description:** Faithful shadow of the current `georaster` crate, which is centered on
GeoTIFF/COG reading rather than a broad raster algebra model. The plan targets the
real upstream surface: `Coordinate` re-export plus
`geotiff::{GeoTiffReader, ImageInfo, Pixels, RasterValue}`.

**Key Insight:** The earlier georaster plan overreached by inventing `Raster<T>`,
`GeoTransform`, terrain analysis, and `ndarray` workflows that do not exist in the
upstream crate. This integration should start with the actual reader/value API and
defer aspirational raster processing abstractions.

**Coverage:**
- **Phase 2 (elicitation):** feature-gated support for upstream value types such as
  `RasterValue` and `ImageInfo`
- **Phase 3 (shadow crate):** `elicit_georaster` wrappers + reader/sampling MCP tools
- **Phase 4 (verification):** Kani / Creusot / Verus proof wiring for the upstream-shaped surface

---

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

**Status:** ✅ Complete

**Description:** Harvesting of the axum web framework (Router, extractors, responses,
handlers, middleware, Tower integration) as MCP tools. Descriptor-registry + factory
pattern. `elicit_tower` + `elicit_axum` both complete.

---

### elicit_polars Shadow Crate

**Document:** [ELICIT_POLARS_PLAN.md](ELICIT_POLARS_PLAN.md)

**Status:** ✅ Complete

**Description:** 72 tools across 4 plugins (PolarsExprPlugin, PolarsDataFramePlugin,
PolarsPipelinePlugin, PolarsSqlPlugin). Runtime Expr registry + dual-tracking, DataFrame
runtime execution, pipeline descriptor + emit_main, SQLContext.

---

### elicit_uom Shadow Crate

**Document:** [ELICIT_UOM_PLAN.md](ELICIT_UOM_PLAN.md)

**Status:** ✅ Complete

**Description:** Units of Measurement (uom 0.38). Phase 3F.6 multi-parameter factory —
18 registered quantities each with typed `HashMap<Uuid, uom::si::f64::Q>`. Shared
QuantityBus enables cross-registration arithmetic (Length/Time→Velocity). ~55 tools.

**Coverage:**
- **UomQuantityPlugin (~40 tools):** 36 per-registration (18×new+emit), 5 query, 12 arithmetic, 1 convert
- **UomCodePlugin (~15 tools):** 5 emit (conversion, calculation, formula, main, snippet) + 10 catalog
- **Physics constants:** c, G, h, kB, NA, e, g, ε0
- **Named formulas:** KineticEnergy, GravitationalPE, OhmsLaw, IdealGas, Momentum

---

### elicit_leptos Shadow Crate

**Document:** [ELICIT_LEPTOS_PLAN.md](ELICIT_LEPTOS_PLAN.md)

**Status:** ✅ Complete (84 tools, 2 plugins)

**Description:** Leptos 0.8 reactive web framework. StatefulPlugin for server-side reactive
primitives + DescriptorPlugin for code generation. ~75 tools.

**Architecture:**
- **`LeptosReactivePlugin`** (StatefulPlugin): `LeptosReactiveContext` holds an `Owner` scope,
  `HashMap<Uuid, RwSignal<Value>>` signals, memos, actions. Uses `leptos ssr` feature — fully
  server-side, no WASM. Tools: signal CRUD, memo derivation, context provide/use, actions.
- **`LeptosCodePlugin`** (DescriptorPlugin): Pure code generation for all macro surfaces.
  `LeptosComponentDescriptor`, `LeptosViewNode`, `LeptosRouteDescriptor`, `LeptosAppDescriptor`
  descriptors feed into emit tools.

**Coverage:**
- **Reactive (~22 tools):** signals (8), memos (4), context (4), actions (4), owner (2)
- **Components (~8 tools):** descriptor create/build/emit, `#[component]`, `#[island]`
- **View (~12 tools):** parametric `element_emit`, Show, For, Suspense, Transition, ErrorBoundary, bindings, closures
- **Server fns (~7 tools):** `#[server]`, Resource, Action, ServerAction, ActionForm
- **Routing (~8 tools):** Router/Routes/Route descriptors, use_params, use_navigate
- **Meta (~4 tools):** Title, Meta, Link, Stylesheet via leptos_meta
- **Scaffolding (~8 tools):** App descriptor → emit_main_rs, emit_cargo_toml, emit_all
- **Catalog (~6 tools):** HTML tags, leptos components, events, features, starter templates

**Key decisions:**
- One parametric `element_emit(tag, attrs, events, children)` replaces 140 per-element tools
- `leptos features = ["ssr"]` — reactive_graph works server-side, no browser/WASM needed
- Macros (`#[component]`, `view!`, `#[server]`, `#[island]`) are emit tools, not runtime wrappers
- Closures (`on:click`, `class:active`, `{move || ...}`) follow closure-as-fragment pattern


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

---

### elicit_db Interface Crate

**Document:** [ELICIT_DB_PLAN.md](ELICIT_DB_PLAN.md)

**Status:** 📋 Planning

**Role:** Interface crate (like `elicit_ui`, not a shadow crate). Defines the domain
boundary for database interactions — Props, typestate markers, and traits that both
DB implementations and consumers (axum, leptos server fns, UI) program against.
No DB driver dependency. No MCP tools. Pure contracts vocabulary.

**Standards stack:**
- ISO/IEC 9075 (SQL semantics — DDL/DML/constraints/views)
- ANSI isolation model (Read Committed → Serializable, phenomena P0–P3)
- PostgreSQL docs (MVCC, advisory locks, WAL — execution truth)
- ISO/IEC 27001 (access control, audit, least privilege, encryption)
- PostgreSQL wire protocol + IETF RFC 7159 (transport)
- OpenTelemetry specification (observability)

**Contract modules:** `iso_sql`, `isolation`, `postgres`, `information_schema`,
`security`, `recovery`, `transport`, `observability`

**Key types:**
- Props: `TableCreated`, `ConstraintSatisfied`, `Serializable`, `AuditLogged`,
  `MVCCSnapshotValid`, `AdvisoryLockHeld`, `WALReplayable`, `TraceEmitted`, etc.
- Typestate: `Transaction<Open/Committed/RolledBack>`, `Query<Prepared/Executed>`
- Traits: `DbConnection`, `DbTransaction`
- Composites: `AcidCommitted`, `PgSafeWrite`

**elicitation primitives** (`db-types` feature): `IsolationLevel`, `DbOperation`,
`DbQueryDescriptor`, `DbSchemaDescriptor`, `DbMigrationDescriptor`

**Deferred:** `elicit_sqlx` (MCP tools), ORM wrappers, migration tooling, connection pooling
