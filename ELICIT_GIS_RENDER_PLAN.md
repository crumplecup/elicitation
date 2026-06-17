# ELICIT_GIS_RENDER_PLAN.md

## Goal

Introduce a renderer-agnostic render contract seam in `elicit_gis` so
geospatial producers can emit proof-carrying render IR and consumer crates
such as `elicit_bevy` can render that IR mechanistically.

The shared IR is the upstream georust ecosystem itself (`geo-types`,
`geojson`, `georaster`, `proj`-adjacent CRS identifiers), not the
`elicit_*` shadow crates. `elicit_gis` owns the contract language, evidence
bundles, descriptor types, and trait seams that make those exchanges auditable
and compiler-enforced.

## Phase 1: Foundation in `elicit_gis`

1. Add a new `render` domain under `contracts/`, `traits/`, and `types/`.
2. Define renderer-agnostic propositions for:
   - view validity
   - symbol/style validity
   - vector layer renderability
   - raster layer renderability
   - whole-scene renderability
3. Compose those propositions through explicit evidence bundles and
   `ProvableFrom` impls, culminating in `Established<RenderableSceneValid>`.
4. Expose object-safe trait families whose method signatures refer directly to
   upstream georust types such as `geo_types::Geometry<f64>`,
   `geojson::FeatureCollection`, and `georaster::geotiff::ImageInfo`.

## Phase 2: Producer Implementations

1. Implement the new render traits in the georust-facing `elicit_*` crates.
2. Keep the proof minting surface narrow: trait methods validate or attest to
   render invariants and return proof tokens plus receipts.
3. Preserve renderer ignorance on the producer side: no Bevy-specific types or
   semantics leak into the GIS contracts.

## Phase 3: Consumer Implementations

1. Implement the render consumer seam in `elicit_bevy`.
2. Consume proof-carrying georust IR and spawn Bevy render state
   mechanistically from validated scene, layer, and style descriptors.
3. Add Bevy-local proofs only where they describe Bevy-specific concerns
   rather than general GIS renderability.
