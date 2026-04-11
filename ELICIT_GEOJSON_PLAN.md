# ELICIT_GEOJSON_PLAN.md

## Goal

Shadow the `geojson` crate faithfully as the next GeoRust document-format crate:

1. **Phase 2 core support** in `crates/elicitation/` for the real upstream `geojson`
   document/value types.
2. **Phase 3 shadow crate** `crates/elicit_geojson/` exposing those types,
   constructors, parsing, formatting, and `geo-types` conversions as MCP tools.
3. **Phase 4 verification** proving our wrapper/conversion behavior while trusting
   upstream `geojson` invariants.

This is a **dictionary, not a redesign**. The plan must mirror upstream names and
capabilities instead of inventing a parallel “layout export” model.

Reference guides:
- `SHADOW_CRATE_MOTIVATION.md`
- `THIRD_PARTY_SUPPORT_GUIDE.md`
- `crates/elicit_wkt/`
- `crates/elicit_wkb/`

---

## Design guardrails

1. **Shadow, do not reshape.**
   - Keep the real top-level document types: `GeoJson`, `Geometry`, `Value`,
     `Feature`, `FeatureCollection`, and `feature::Id`.
   - Do not replace GeoJSON documents with a custom UI/layout abstraction.

2. **Separate document format from geometry model.**
   - `geo_types` remains the computation vocabulary.
   - `geojson` is the serialized document vocabulary.
   - Conversion tools bridge them; they do not collapse the two domains.

3. **Keep consumer workflows layered above the core crate.**
   - Layout export/import, map pipelines, and DB ingestion are downstream consumers.
   - `elicit_geojson` itself should focus on the actual upstream GeoJSON API.

4. **Follow current workspace conventions.**
   - Feature name should be `geojson-types`, not `geojson`.
   - `lib.rs` files contain only `mod` and `pub use`.
   - Tests live in `tests/`, not inline modules.

---

## Actual upstream API to mirror

The `geojson` crate is structured around GeoJSON RFC 7946 document types.

### Core document/value types

| Upstream type | Notes |
|---|---|
| `geojson::GeoJson` | Top-level enum: `Geometry`, `Feature`, `FeatureCollection` |
| `geojson::Geometry` | Struct with `bbox`, `value`, `foreign_members` |
| `geojson::Value` | Enum for Point / MultiPoint / LineString / MultiLineString / Polygon / MultiPolygon / GeometryCollection |
| `geojson::Feature` | Struct with `bbox`, `geometry`, `id`, `properties`, `foreign_members` |
| `geojson::FeatureCollection` | Struct with `bbox`, `features`, `foreign_members` |
| `geojson::feature::Id` | Enum: `String(String)` or `Number(serde_json::Number)` |

### Important upstream behavior

- `GeoJson`, `Geometry`, `Feature`, and `FeatureCollection` implement `FromStr` and `Display`.
- `Geometry` exposes concise constructors like `Geometry::new_point(...)`.
- `Feature` supports property helpers (`property`, `contains_property`, `set_property`, `remove_property`, iteration helpers).
- `FeatureCollection::new(...)` constructs collections from iterables.
- With the upstream `geo-types` feature enabled:
  - `From<&geo_types::...>` exists for `Geometry`, `Value`, and `FeatureCollection`
  - `TryFrom<GeoJson|Geometry|Value|Feature|FeatureCollection>` exists back to `geo_types` types

These upstream names and conversions should drive the shadow-crate surface.

---

## Scope for the first implementation

### In scope

1. Workspace wiring for `geojson`
2. `elicitation` feature-gated support for the upstream document/value types
3. `elicit_geojson` wrappers for:
   - `GeoJson`
   - `Geometry`
   - `Value`
   - `Feature`
   - `FeatureCollection`
   - `Id`
4. Parsing/formatting tools
5. Construction tools for the main document/value types
6. `geo-types` conversion helpers and workflow tools
7. Verification wiring in Kani / Creusot / Verus

### Explicitly out of scope for the initial landing

1. Custom UI-layout semantics
2. File I/O helpers unless the upstream crate itself provides a real API surface worth shadowing
3. CRS extensions or non-RFC compatibility layers beyond what `geojson` already accepts
4. Invented registries or descriptor systems

---

## Phase 1 — Workspace wiring

### Root `Cargo.toml`

Add:

```toml
geojson = "0.24"
elicit_geojson = { path = "crates/elicit_geojson", version = "0.10.0" }
```

and add the new workspace member:

```toml
"crates/elicit_geojson",
```

### `crates/elicitation/Cargo.toml`

Add:

```toml
[dependencies]
geojson = { workspace = true, optional = true }
```

Feature:

```toml
geojson-types = ["dep:geojson", "geo-types", "serde_json"]
```

Update `full` once the implementation lands.

---

## Phase 2 — `crates/elicitation/` core support

Follow `THIRD_PARTY_SUPPORT_GUIDE.md` literally: use a `primitives/geojson_types/`
module and a `type_spec/geojson_specs.rs` file rather than a one-off
`geojson_support.rs`.

### Files

```text
crates/elicitation/src/primitives/geojson_types/
├── mod.rs
├── geojson.rs
├── geometry.rs
├── geometry_value.rs
├── feature.rs
├── feature_collection.rs
└── id.rs

crates/elicitation/src/type_spec/geojson_specs.rs
```

### Phase 2 strategy

Implement `Elicitation` for the real upstream types where practical:

- `geojson::GeoJson`
- `geojson::Geometry`
- `geojson::Value`
- `geojson::Feature`
- `geojson::FeatureCollection`
- `geojson::feature::Id`

Recommended elicitation patterns:

| Type | Pattern |
|---|---|
| `GeoJson` | `Select` over document variants |
| `Value` | `Select` over geometry variants |
| `Geometry` | Survey: `value`, optional `bbox`, optional `foreign_members` |
| `Feature` | Survey: optional geometry, optional id, optional properties, optional bbox, optional foreign members |
| `FeatureCollection` | Survey: features + optional bbox/foreign members |
| `Id` | `Select` over string vs number |

### Important constraint

Do not invent a separate enum of pseudo-GeoJSON geometry names if the upstream
`Value` already carries that information. The wrapper should expose the
actual `Value` variants.

### Module wiring

`primitives/mod.rs`:

```rust
#[cfg(feature = "geojson-types")]
mod geojson_types;
#[cfg(feature = "geojson-types")]
pub use geojson_types::{Feature, FeatureCollection, GeoJson, Geometry, Id, Value};
```

`lib.rs` should only re-export from `primitives`, consistent with workspace rules.

### TypeSpec

Add `geojson_specs.rs` so agents can browse the document/value vocabulary.

---

## Phase 3 — `crates/elicit_geojson/` shadow crate

### Crate shape

Mirror upstream concepts, not downstream use cases.

```text
crates/elicit_geojson/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── geojson.rs
│   ├── geometry.rs
│   ├── geometry_value.rs
│   ├── feature.rs
│   ├── feature_collection.rs
│   ├── workflow/
│   │   ├── mod.rs
│   │   ├── document_plugin.rs
│   │   ├── geometry_plugin.rs
│   │   ├── feature_plugin.rs
│   │   └── conversion_plugin.rs
└── tests/
    ├── smoke_test.rs
    └── workflow_test.rs
```

### `lib.rs`

Only:

- `mod ...`
- `pub use ...`

### Wrapper strategy

Use the established newtype/shadow pattern:

- serde + schemars-friendly wrappers over the upstream types
- `From` / `Into` or `TryFrom` bridges back to upstream
- `#[reflect_methods]` for stable upstream-style methods
- `#[reflect_trait]` only where a real third-party trait is being surfaced

### Expected plugins

Keep the first pass tight:

| Plugin | Namespace | Purpose |
|---|---|---|
| `GeoJsonDocumentPlugin` | `geojson_document__*` | Parse, inspect, and convert top-level `GeoJson` documents |
| `GeoJsonGeometryPlugin` | `geojson_geometry__*` | Construct and inspect `Geometry` / `Value` |
| `GeoJsonFeaturePlugin` | `geojson_feature__*` | Construct and inspect `Feature` / `FeatureCollection` / `Id` |
| `GeoJsonConversionPlugin` | `geojson_conversion__*` | Bridge to and from `geo_types` |

Do **not** start with `io_plugin` or custom “workflow” plugins unless we later find
upstream-shaped behavior that belongs there.

### Conversion surface

The important consumer-facing value is accurate conversion support:

- `geo_types::Geometry<f64>` → `geojson::Geometry`
- `geo_types::{Point, LineString, Polygon, Multi*, GeometryCollection}` → `Geometry` / `Value`
- `GeoJson` / `Geometry` / `Value` / `Feature` / `FeatureCollection` → `geo_types` via fallible conversions

This should be exposed explicitly and tested carefully.

---

## Phase 4 — Verification

Follow the current third-party proof policy:

> Trust the source. Verify the wrapper.

### Kani

Add:

```text
crates/elicitation_kani/src/geojson_types.rs
```

Focus on:
- enum coverage for `GeoJson`, `Value`, and `Id`
- wrapper roundtrips
- known-value parse/format checks
- conversion harnesses for simple shapes (point, line string, polygon)

### Creusot

Add:

```text
crates/elicitation_creusot/src/geojson_types.rs
```

Use trusted wrapper constructors and structural postconditions. Do not attempt to
re-prove RFC 7946 parsing internals.

### Verus

Add:

```text
crates/elicitation_verus/src/geojson_types.rs
```

Keep proofs lightweight and structural, matching the WKT/WKB pattern.

### Runner wiring

Update:

```text
crates/elicitation/src/verification/runner.rs
crates/elicitation/src/verification/creusot_runner.rs
crates/elicitation/src/verification/verus_runner.rs
crates/elicitation/tests/proof_non_empty_test.rs
```

---

## Validation plan

### Core support

```bash
cargo check -p elicitation --features geojson-types
```

### Shadow crate

```bash
just check elicit_geojson
just test-package elicit_geojson
just check-all elicit_geojson
```

### Proof crates

```bash
cargo check -p elicitation_kani --features 'kani,geojson-types'
cargo check -p elicitation_creusot --features 'geojson-types'
cargo check --manifest-path crates/elicitation_verus/Cargo.toml
cargo test -p elicitation --features geojson-types --test proof_non_empty_test
```

---

## Recommended implementation order

1. Workspace wiring + `geojson-types` feature
2. Phase 2 upstream-type elicitation in `elicitation`
3. Minimal `elicit_geojson` wrappers and parsing/construction tools
4. `geo-types` conversion tools
5. Shadow-crate tests
6. Kani / Creusot / Verus wiring
7. Only then consider higher-level GeoJSON consumer workflows

---

## Success criteria

This plan is complete when:

1. `geojson` is wired into the workspace with a `geojson-types` feature
2. Agents can elicit and inspect the real GeoJSON document/value types
3. `elicit_geojson` exposes a faithful upstream-shaped MCP vocabulary
4. `geo-types` ↔ GeoJSON conversions work through the shadow layer
5. Proof wiring covers the wrapper vocabulary without reshaping upstream semantics
