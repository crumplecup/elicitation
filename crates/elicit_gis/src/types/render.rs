//! Renderer-agnostic GIS render descriptors.
//!
//! These sidecars deliberately reuse upstream render IRs where credible
//! proxies already exist:
//!
//! - `geo-types` and `geojson` for vector payloads
//! - `kurbo::Stroke` for joins, caps, miter limits, and dash patterns
//! - `peniko` paint / fill primitives for renderable material semantics
//! - `elicit_ui` text IR for label text and typography
//! - `url::Url` for externally sourced render assets
//!
//! The remaining types model the map-specific semantics and world/render-space
//! policies that the upstream crates do not provide directly.

use std::{borrow::Cow, ops::Deref};

use elicit_ui::{ParagraphText, TextAlign, TextStyle, UiColor};
use geo_types::Geometry;
use geojson::FeatureCollection;
use georaster::geotiff::ImageInfo;
use kurbo::Stroke;
use peniko::{Brush, Fill, Gradient};
use schemars::{JsonSchema, SchemaGenerator, json_schema};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

/// Renderer-agnostic paint backed by `peniko::Brush`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RenderBrush(pub Brush);

impl JsonSchema for RenderBrush {
    fn schema_name() -> Cow<'static, str> {
        "RenderBrush".into()
    }

    fn json_schema(_schema_gen: &mut SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "description": "Serialized peniko::Brush paint. The concrete JSON follows peniko's serde representation and supports Solid, Gradient, and Image variants.",
            "oneOf": [
                { "type": "object", "required": ["Solid"] },
                { "type": "object", "required": ["Gradient"] },
                { "type": "object", "required": ["Image"] }
            ]
        })
    }
}

impl Deref for RenderBrush {
    type Target = Brush;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Brush> for RenderBrush {
    fn from(value: Brush) -> Self {
        Self(value)
    }
}

impl From<RenderBrush> for Brush {
    fn from(value: RenderBrush) -> Self {
        value.0
    }
}

/// Fill-rule wrapper backed by `peniko::Fill`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RenderFillRule(pub Fill);

impl JsonSchema for RenderFillRule {
    fn schema_name() -> Cow<'static, str> {
        "RenderFillRule".into()
    }

    fn json_schema(_schema_gen: &mut SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "string",
            "enum": ["NonZero", "EvenOdd"],
            "description": "Serialized peniko::Fill rule."
        })
    }
}

impl Deref for RenderFillRule {
    type Target = Fill;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Fill> for RenderFillRule {
    fn from(value: Fill) -> Self {
        Self(value)
    }
}

impl From<RenderFillRule> for Fill {
    fn from(value: RenderFillRule) -> Self {
        value.0
    }
}

/// Gradient wrapper backed by `peniko::Gradient`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RenderGradient(pub Gradient);

impl JsonSchema for RenderGradient {
    fn schema_name() -> Cow<'static, str> {
        "RenderGradient".into()
    }

    fn json_schema(_schema_gen: &mut SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "object",
            "description": "Serialized peniko::Gradient paint. The concrete JSON follows peniko's serde representation and includes kind, extend mode, interpolation settings, and color stops."
        })
    }
}

impl Deref for RenderGradient {
    type Target = Gradient;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Gradient> for RenderGradient {
    fn from(value: Gradient) -> Self {
        Self(value)
    }
}

impl From<RenderGradient> for Gradient {
    fn from(value: RenderGradient) -> Self {
        value.0
    }
}

/// World up-axis used by the renderer's local scene space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderUpAxis {
    /// Y is treated as vertical/up.
    Y,
    /// Z is treated as vertical/up.
    Z,
}

/// Policy for establishing a local render origin.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderWorldOrigin {
    /// Center the render world on the active view center.
    ViewCenter,
    /// Center the render world on the centroid of the active layer set.
    LayerCentroid,
    /// Use an explicit local origin in scene units.
    Explicit {
        /// Local origin X.
        x: f64,
        /// Local origin Y.
        y: f64,
        /// Local origin Z.
        z: f64,
    },
}

/// Renderer-local world-space policy shared across a scene or view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderWorldSpaceDescriptor {
    /// Strategy for choosing the local origin.
    pub origin: RenderWorldOrigin,
    /// Which axis the renderer interprets as "up".
    pub up_axis: RenderUpAxis,
    /// Local render units per physical meter.
    pub units_per_meter: f64,
    /// Vertical exaggeration factor for heights and terrain.
    pub vertical_exaggeration: f64,
}

/// Externally resolved asset reference usable by downstream renderers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderAssetColorSpace {
    /// Color data interpreted in the standard sRGB transfer space.
    Srgb,
    /// Color data interpreted in a linear-light working space.
    Linear,
    /// Non-color payload such as height, masks, normals, or lookup data.
    Data,
}

/// Residency intent for externally resolved render assets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderAssetResidencyPolicy {
    /// Load only when first needed.
    OnDemand,
    /// Request early loading before first use.
    Preload,
    /// Keep the asset resident once loaded.
    KeepResident,
    /// Stream the asset progressively as needed.
    Streaming,
}

/// Expected update/access pattern for an external render asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderAssetAccessPattern {
    /// Asset content is effectively immutable once loaded.
    Static,
    /// Asset content may be updated over time.
    Dynamic,
    /// Asset content is expected to stream incrementally.
    Streaming,
}

/// Digest algorithm used to identify immutable asset content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderAssetIntegrityAlgorithm {
    /// SHA-256 digest.
    Sha256,
    /// SHA-512 digest.
    Sha512,
    /// BLAKE3 digest.
    Blake3,
}

/// Integrity metadata for externally sourced assets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RenderAssetIntegrityDescriptor {
    /// Digest algorithm used to compute the integrity value.
    pub algorithm: RenderAssetIntegrityAlgorithm,
    /// Lowercase hexadecimal digest string.
    pub digest_hex: String,
}

/// Externally resolved asset reference usable by downstream renderers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderAssetReference {
    /// Stable URI for the asset.
    pub uri: Url,
    /// Optional media type such as `image/png` or `image/tiff`.
    pub media_type: Option<String>,
    /// Optional expected byte length.
    pub byte_length: Option<u64>,
    /// Optional declared color-space intent for image-like assets.
    pub color_space: Option<RenderAssetColorSpace>,
    /// Optional residency intent for downstream asset managers.
    pub residency: Option<RenderAssetResidencyPolicy>,
    /// Optional access/update pattern for downstream asset managers.
    pub access_pattern: Option<RenderAssetAccessPattern>,
    /// Optional stable revision token or version string for cache invalidation.
    pub revision: Option<String>,
    /// Optional declared content-integrity metadata.
    pub integrity: Option<RenderAssetIntegrityDescriptor>,
    /// Optional logical width in renderer-neutral scene units.
    pub logical_width: Option<f64>,
    /// Optional logical height in renderer-neutral scene units.
    pub logical_height: Option<f64>,
}

impl JsonSchema for RenderAssetReference {
    fn schema_name() -> Cow<'static, str> {
        "RenderAssetReference".into()
    }

    fn json_schema(_schema_gen: &mut SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "object",
            "properties": {
                "uri": {
                    "type": "string",
                    "format": "uri"
                },
                "media_type": {
                    "type": ["string", "null"]
                },
                "byte_length": {
                    "type": ["integer", "null"],
                    "minimum": 0
                },
                "color_space": {
                    "type": ["string", "null"],
                    "enum": ["Srgb", "Linear", "Data", null]
                },
                "residency": {
                    "type": ["string", "null"],
                    "enum": ["OnDemand", "Preload", "KeepResident", "Streaming", null]
                },
                "access_pattern": {
                    "type": ["string", "null"],
                    "enum": ["Static", "Dynamic", "Streaming", null]
                },
                "revision": {
                    "type": ["string", "null"]
                },
                "integrity": {
                    "type": ["object", "null"]
                },
                "logical_width": {
                    "type": ["number", "null"]
                },
                "logical_height": {
                    "type": ["number", "null"]
                }
            },
            "required": ["uri"]
        })
    }
}

/// High-level material or pipeline family expected by a renderable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderMaterialFamily {
    /// Simple 2D line or outline rendering.
    Linework2d,
    /// Simple filled 2D vector rendering.
    Flat2d,
    /// Sprite or billboard rendering backed by an image asset.
    Sprite,
    /// Standard lit surface shading.
    StandardSurface,
    /// Terrain-oriented lit surface shading.
    TerrainSurface,
    /// Decal-style projected surface rendering.
    Decal,
    /// Volume-oriented rendering.
    Volume,
    /// Full-screen post-process rendering.
    PostProcess,
}

/// Alpha-compositing family for backend material setup.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderMaterialAlphaMode {
    /// Fully opaque rendering.
    Opaque,
    /// Alpha-mask rendering with an explicit discard threshold.
    Mask {
        /// Threshold in the closed unit interval.
        threshold: f32,
    },
    /// Straight alpha blending.
    Blend,
    /// Premultiplied-alpha blending.
    Premultiplied,
    /// Additive blending.
    Add,
    /// Multiplicative blending.
    Multiply,
}

/// Face-culling policy for material-backed surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderFaceCulling {
    /// Disable face culling.
    None,
    /// Cull front faces.
    Front,
    /// Cull back faces.
    Back,
}

/// Semantic meaning of one material texture or lookup input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderMaterialTextureSemantic {
    /// Base-color / albedo input.
    BaseColor,
    /// Emissive color input.
    Emissive,
    /// Combined metallic-roughness input.
    MetallicRoughness,
    /// Normal-map input.
    Normal,
    /// Ambient-occlusion input.
    Occlusion,
    /// Height or displacement input.
    Height,
    /// Lookup texture such as a LUT or transfer function.
    Lookup,
    /// Environment or reflection-map input.
    Environment,
    /// Transmission or thickness control input.
    Transmission,
    /// Density or volume-shaping input.
    Density,
    /// Renderer-neutral generic data texture input.
    GenericData,
}

/// Broad render-pipeline domain targeted by one material-backed payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderPipelineDomain {
    /// Screen-space 2D rendering.
    Screen2d,
    /// World-space 2D rendering.
    World2d,
    /// World-space 3D rendering.
    World3d,
    /// Decal-style projected rendering.
    Decal,
    /// Volume-oriented rendering.
    Volume,
    /// Full-screen post-process rendering.
    PostProcess,
}

/// Opaque rendering strategy for pipeline domains that support multiple paths.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderOpaqueMethod {
    /// Let the backend select the best opaque method.
    Auto,
    /// Force a forward opaque path.
    Forward,
    /// Force a deferred opaque path.
    Deferred,
}

/// Optional prepass requirements that a backend can exploit mechanically.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderPrepassKind {
    /// Depth-only prepass.
    Depth,
    /// Normal prepass.
    Normal,
    /// Motion-vector prepass.
    MotionVector,
    /// Deferred-material prepass.
    Deferred,
}

/// Renderer-neutral pipeline placement and prepass intent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderPipelineIntentDescriptor {
    /// Broad render-pipeline domain expected by the renderer.
    pub domain: RenderPipelineDomain,
    /// Optional opaque rendering method when the chosen domain supports it.
    pub opaque_method: Option<RenderOpaqueMethod>,
    /// Explicit prepass requirements for the payload.
    pub prepasses: Vec<RenderPrepassKind>,
}

/// One texture or lookup asset bound into a render material.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderMaterialTextureBindingDescriptor {
    /// Meaning of the bound texture.
    pub semantic: RenderMaterialTextureSemantic,
    /// Asset used for the texture or lookup input.
    pub asset: RenderAssetReference,
}

/// Renderer-neutral material specialization and pipeline-selection hints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderMaterialIntentDescriptor {
    /// High-level material family expected by the renderer.
    pub family: RenderMaterialFamily,
    /// Optional alpha-compositing policy.
    pub alpha_mode: Option<RenderMaterialAlphaMode>,
    /// Optional face-culling policy.
    pub cull_mode: Option<RenderFaceCulling>,
    /// Optional unlit hint for families that support it.
    pub unlit: Option<bool>,
    /// Optional explicit depth-write override.
    pub depth_write: Option<bool>,
    /// Optional fixed depth bias used to reduce surface fighting.
    pub depth_bias: Option<f32>,
    /// Optional stable specialization key used to group compatible materials.
    pub specialization_key: Option<String>,
    /// Optional pipeline-placement and prepass intent for this material family.
    pub pipeline_intent: Option<RenderPipelineIntentDescriptor>,
    /// Explicit texture and lookup inputs required by the material family.
    pub textures: Vec<RenderMaterialTextureBindingDescriptor>,
}

/// Stroke styling for linework and polygon outlines.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderStrokeDescriptor {
    /// Paint applied to the stroke.
    pub brush: RenderBrush,
    /// Stroke geometry backed by `kurbo::Stroke`.
    pub stroke: Stroke,
}

impl JsonSchema for RenderStrokeDescriptor {
    fn schema_name() -> Cow<'static, str> {
        "RenderStrokeDescriptor".into()
    }

    fn json_schema(schema_gen: &mut SchemaGenerator) -> schemars::Schema {
        let brush_schema = schema_gen.subschema_for::<RenderBrush>();
        json_schema!({
            "type": "object",
            "properties": {
                "brush": brush_schema,
                "stroke": {
                    "type": "object",
                    "description": "Serialized kurbo::Stroke geometry. The concrete JSON follows kurbo's serde representation and includes width, join, miter_limit, start_cap, end_cap, dash_pattern, and dash_offset."
                }
            },
            "required": ["brush", "stroke"]
        })
    }
}

/// Fill styling for polygon interiors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderFillDescriptor {
    /// Paint applied to the fill.
    pub brush: RenderBrush,
    /// Fill rule for self-intersecting paths.
    pub fill_rule: RenderFillRule,
}

/// Point-symbol primitive used for vector point features.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderPointShape {
    /// Circular marker.
    Circle,
    /// Square marker.
    Square,
    /// Triangular marker.
    Triangle,
    /// Diamond marker.
    Diamond,
    /// Cross marker.
    Cross,
}

/// Point-symbol content for vector point features.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderPointSymbolContent {
    /// Primitive geometry marker.
    Primitive(RenderPointShape),
    /// Glyph or rich-text marker sourced from the shared UI text IR.
    Glyph {
        /// Glyph content to render at the point location.
        text: ParagraphText,
        /// Typography and color for the glyph marker.
        style: TextStyle,
    },
    /// Asset-backed image, sprite, or signed-distance icon marker.
    Asset {
        /// Resolved asset reference for the symbol.
        asset: RenderAssetReference,
        /// Optional tint applied by backends that support it.
        tint: Option<RenderBrush>,
    },
}

/// Point-symbol styling for point features.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderPointSymbolDescriptor {
    /// Point-marker content.
    pub content: RenderPointSymbolContent,
    /// Symbol size in renderer-neutral scene units.
    pub size: RenderNumericExpression,
    /// Optional clockwise rotation in degrees.
    pub rotation_degrees: Option<RenderNumericExpression>,
    /// Horizontal symbol offset in renderer-neutral scene units.
    pub offset_x: RenderNumericExpression,
    /// Vertical symbol offset in renderer-neutral scene units.
    pub offset_y: RenderNumericExpression,
    /// Optional outline styling for the marker.
    pub stroke: Option<RenderStrokeDescriptor>,
    /// Optional interior styling for the marker.
    pub fill: Option<RenderFillDescriptor>,
}

/// Label-placement policy independent of any concrete renderer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderLabelPlacement {
    /// Place labels at feature centroids.
    Centroid,
    /// Place labels at robust interior points for polygons.
    InteriorPoint,
    /// Place labels on geometry vertices.
    Vertex,
    /// Place labels along line midpoints.
    Midpoint,
    /// Follow the primary line path when the backend supports curved text.
    AlongPath,
    /// Interpret labels in a screen-space overlay plane.
    ScreenSpace,
}

/// Label anchor and alignment policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderLabelAnchor {
    /// Center the text on the anchor point.
    Center,
    /// Anchor the text above the anchor point.
    Top,
    /// Anchor the text below the anchor point.
    Bottom,
    /// Anchor the text to the left of the anchor point.
    Left,
    /// Anchor the text to the right of the anchor point.
    Right,
}

/// Source of label content for renderer-agnostic feature annotations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderLabelTextSource {
    /// Render a fixed label string or rich text block for every feature.
    Literal(ParagraphText),
    /// Read the label content from a feature property.
    FeatureProperty {
        /// Name of the property supplying text content.
        property_name: String,
    },
}

/// Label styling for feature-driven annotations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderLabelDescriptor {
    /// Source supplying the label text.
    pub text: RenderLabelTextSource,
    /// Typography and color sourced from the shared UI text IR.
    pub style: TextStyle,
    /// Placement policy.
    pub placement: RenderLabelPlacement,
    /// Anchor/alignment around the placement target.
    pub anchor: RenderLabelAnchor,
    /// Paragraph alignment within the label block.
    pub text_align: Option<TextAlign>,
    /// Relative placement/decluttering priority.
    pub priority: RenderNumericExpression,
    /// Horizontal label offset in renderer-neutral scene units.
    pub offset_x: RenderNumericExpression,
    /// Vertical label offset in renderer-neutral scene units.
    pub offset_y: RenderNumericExpression,
    /// Optional clockwise label rotation in degrees.
    pub rotation_degrees: Option<RenderNumericExpression>,
    /// Optional label halo / outline styling.
    pub halo_stroke: Option<RenderStrokeDescriptor>,
    /// Optional max line width for wrapping.
    pub wrap_width: Option<RenderNumericExpression>,
    /// Optional maximum number of rendered lines.
    pub max_lines: Option<u16>,
    /// Optional minimum repeat distance for line labels.
    pub repeat_distance: Option<RenderNumericExpression>,
    /// Whether line-following labels may render upside down.
    pub allow_upside_down: bool,
    /// Optional decluttering group name.
    pub collision_group: Option<String>,
}

/// Label collision / decluttering policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderLabelCollisionPolicy {
    /// Render labels even when they overlap.
    AllowOverlap,
    /// Suppress lower-priority labels on collision.
    PreferHigherPriority,
    /// Hide the label if it collides with an existing label.
    HideOnCollision,
}

/// Predicate language for property-driven or scale-dependent styling rules.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderFeaturePredicate {
    /// Predicate that always matches.
    Always,
    /// Property must be present on the feature.
    HasProperty {
        /// Property name to inspect.
        property_name: String,
    },
    /// Property must equal a literal JSON value.
    PropertyEquals {
        /// Property name to inspect.
        property_name: String,
        /// Expected literal value.
        value: Value,
    },
    /// Property must not equal a literal JSON value.
    PropertyNotEquals {
        /// Property name to inspect.
        property_name: String,
        /// Rejected literal value.
        value: Value,
    },
    /// Property must be greater than a numeric literal.
    PropertyGreaterThan {
        /// Property name to inspect.
        property_name: String,
        /// Threshold value.
        value: f64,
    },
    /// Property must be greater than or equal to a numeric literal.
    PropertyGreaterThanOrEqual {
        /// Property name to inspect.
        property_name: String,
        /// Threshold value.
        value: f64,
    },
    /// Property must be less than a numeric literal.
    PropertyLessThan {
        /// Property name to inspect.
        property_name: String,
        /// Threshold value.
        value: f64,
    },
    /// Property must be less than or equal to a numeric literal.
    PropertyLessThanOrEqual {
        /// Property name to inspect.
        property_name: String,
        /// Threshold value.
        value: f64,
    },
    /// Property value must match one of the supplied literals.
    PropertyIn {
        /// Property name to inspect.
        property_name: String,
        /// Accepted literal values.
        values: Vec<Value>,
    },
    /// Match by top-level geometry family.
    GeometryType {
        /// Geometry family name such as `Point`, `LineString`, or `Polygon`.
        geometry_type: String,
    },
    /// Logical AND over multiple predicates.
    All(Vec<RenderFeaturePredicate>),
    /// Logical OR over multiple predicates.
    Any(Vec<RenderFeaturePredicate>),
    /// Logical negation of another predicate.
    Not(Box<RenderFeaturePredicate>),
}

/// Interpolation family for numeric expressions with stops.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderNumericInterpolationMode {
    /// Piecewise linear interpolation between stops.
    Linear,
    /// Exponential interpolation with the supplied positive base.
    Exponential {
        /// Positive interpolation base.
        base: f64,
    },
    /// Discrete jumps between stop values.
    Discrete,
}

/// Step stop for a stepped numeric expression.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderNumericStepStopDescriptor {
    /// Threshold at which the stop value becomes active.
    pub input: f64,
    /// Output value used once the threshold is reached.
    pub value: f64,
}

/// Interpolation stop for a continuous numeric expression.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderNumericInterpolationStopDescriptor {
    /// Input coordinate for the stop.
    pub input: f64,
    /// Output value at the stop.
    pub value: f64,
}

/// Numeric feature expression used for heights, offsets, and similar values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderNumericExpression {
    /// Use a fixed literal number.
    Literal(f64),
    /// Use the active map resolution as the input value.
    MapResolution,
    /// Read the numeric value from a feature property.
    FeatureProperty {
        /// Name of the property supplying the numeric value.
        property_name: String,
    },
    /// Read the numeric value from a feature property, falling back to a default.
    FeaturePropertyOrDefault {
        /// Name of the property supplying the numeric value.
        property_name: String,
        /// Default value used when the property is absent or null.
        default_value: f64,
    },
    /// Negate the nested numeric expression.
    Negate(Box<RenderNumericExpression>),
    /// Add the supplied expressions in order.
    Add(Vec<RenderNumericExpression>),
    /// Multiply the supplied expressions in order.
    Multiply(Vec<RenderNumericExpression>),
    /// Subtract the right expression from the left expression.
    Subtract {
        /// Left-hand input.
        left: Box<RenderNumericExpression>,
        /// Right-hand input.
        right: Box<RenderNumericExpression>,
    },
    /// Divide the numerator by the denominator.
    Divide {
        /// Numerator input.
        numerator: Box<RenderNumericExpression>,
        /// Denominator input.
        denominator: Box<RenderNumericExpression>,
    },
    /// Clamp the input to the inclusive range `[min, max]`.
    Clamp {
        /// Input expression.
        input: Box<RenderNumericExpression>,
        /// Minimum output value.
        min: f64,
        /// Maximum output value.
        max: f64,
    },
    /// Return the first usable value from the supplied expressions.
    Coalesce(Vec<RenderNumericExpression>),
    /// Stepped lookup over a monotone sequence of thresholds.
    Step {
        /// Input expression evaluated against the stops.
        input: Box<RenderNumericExpression>,
        /// Output value used before the first stop.
        default_value: f64,
        /// Ordered step thresholds and replacement values.
        stops: Vec<RenderNumericStepStopDescriptor>,
    },
    /// Continuous or discrete interpolation over ordered stops.
    Interpolate {
        /// Input expression evaluated against the stops.
        input: Box<RenderNumericExpression>,
        /// Interpolation family.
        mode: RenderNumericInterpolationMode,
        /// Ordered interpolation stops.
        stops: Vec<RenderNumericInterpolationStopDescriptor>,
    },
}

/// Visibility range in renderer-neutral map resolution units.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderScaleRangeDescriptor {
    /// Optional minimum visible resolution.
    pub min_resolution: Option<f64>,
    /// Optional maximum visible resolution.
    pub max_resolution: Option<f64>,
}

/// Altitude interpretation for vector features in world space.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderAltitudeMode {
    /// Clamp features to the terrain or ground plane.
    ClampToGround,
    /// Interpret heights relative to the terrain or ground plane.
    RelativeToGround,
    /// Interpret heights as absolute world elevations.
    Absolute,
}

/// Billboard policy for symbols or labels in 3D renderers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderBillboardMode {
    /// Face the screen plane directly.
    ScreenAligned,
    /// Face the active camera view while preserving local vertical orientation.
    ViewAligned,
    /// Remain fixed in world orientation.
    WorldFixed,
}

/// Shadow-participation policy for shadow-capable render content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RenderShadowParticipationDescriptor {
    /// Whether the content casts shadows.
    pub casts_shadows: bool,
    /// Whether the content receives ordinary shadows.
    pub receives_shadows: bool,
    /// Whether the content receives diffuse transmission shadows.
    pub receives_transmitted_shadows: bool,
}

/// World-space placement policy for vector features.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderFeaturePlacementDescriptor {
    /// How feature elevations should be interpreted.
    pub altitude_mode: RenderAltitudeMode,
    /// Optional height offset.
    pub height_offset: Option<RenderNumericExpression>,
    /// Optional billboard mode for point-centric content.
    pub billboard_mode: Option<RenderBillboardMode>,
    /// Optional per-feature z-index within a layer.
    pub z_index: Option<i32>,
}

/// Extrusion policy for polygons or bars in 2.5D/3D renderers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderExtrusionDescriptor {
    /// Expression supplying the extrusion height.
    pub height: RenderNumericExpression,
    /// Optional base height for the extrusion.
    pub base_height: Option<RenderNumericExpression>,
    /// Whether the geometry should be treated as a closed solid volume.
    pub closed_volume: bool,
}

/// Resolved symbolizer for vector features.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct RenderFeatureSymbolizerDescriptor {
    /// Optional stroke styling for lines and polygon outlines.
    pub stroke: Option<RenderStrokeDescriptor>,
    /// Optional fill styling for polygon interiors.
    pub fill: Option<RenderFillDescriptor>,
    /// Optional point styling for point geometries.
    pub point: Option<RenderPointSymbolDescriptor>,
    /// Optional label styling for feature annotations.
    pub label: Option<RenderLabelDescriptor>,
    /// Collision handling for labels contributed by this symbolizer.
    pub label_collision_policy: Option<RenderLabelCollisionPolicy>,
    /// Optional world-space placement policy.
    pub placement: Option<RenderFeaturePlacementDescriptor>,
    /// Optional extrusion policy.
    pub extrusion: Option<RenderExtrusionDescriptor>,
    /// Optional shadow-participation policy.
    pub shadow_participation: Option<RenderShadowParticipationDescriptor>,
    /// Optional material and pipeline hints for mesh, sprite, or surface backends.
    pub material_intent: Option<RenderMaterialIntentDescriptor>,
}

/// Property-driven or scale-dependent style override rule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderFeatureStyleRuleDescriptor {
    /// Predicate controlling when the rule applies.
    pub predicate: RenderFeaturePredicate,
    /// Optional visibility range for the rule.
    pub resolution_range: Option<RenderScaleRangeDescriptor>,
    /// Symbolizer applied when the rule matches.
    pub symbolizer: RenderFeatureSymbolizerDescriptor,
}

/// Full rule-aware style for vector features.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderFeatureStyleDescriptor {
    /// Default symbolizer used when no rule overrides apply.
    pub default_symbolizer: RenderFeatureSymbolizerDescriptor,
    /// Ordered style rules evaluated by the backend in declared order.
    pub rules: Vec<RenderFeatureStyleRuleDescriptor>,
}

/// Band-selection policy for raster rendering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
pub enum RenderRasterBandSelection {
    /// Let the source metadata or backend choose the default interpretation.
    #[default]
    SourceDefault,
    /// Render a single 1-based band.
    Single {
        /// 1-based band index.
        band: u8,
    },
    /// Compose an RGB raster from explicit 1-based bands.
    Rgb {
        /// Red channel band.
        red: u8,
        /// Green channel band.
        green: u8,
        /// Blue channel band.
        blue: u8,
    },
    /// Compose an RGBA raster from explicit 1-based bands.
    Rgba {
        /// Red channel band.
        red: u8,
        /// Green channel band.
        green: u8,
        /// Blue channel band.
        blue: u8,
        /// Alpha channel band.
        alpha: u8,
    },
}

/// Sampling/interpolation policy for raster layers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderRasterSampling {
    /// Nearest-neighbour sampling.
    Nearest,
    /// Bilinear interpolation.
    Bilinear,
    /// Bicubic interpolation.
    Bicubic,
}

/// Value-domain color ramp for single-band raster rendering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderRasterColorRampDescriptor {
    /// Gradient used to colorize sampled raster values.
    pub gradient: RenderGradient,
    /// Minimum source value covered by the ramp.
    pub domain_min: f64,
    /// Maximum source value covered by the ramp.
    pub domain_max: f64,
    /// Whether values outside the domain should clamp to the nearest stop.
    pub clamp: bool,
}

/// Numeric transform applied to raster sample values before colorization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderRasterValueTransform {
    /// Normalize values from a declared source range into the unit interval.
    Normalize {
        /// Minimum input value.
        source_min: f64,
        /// Maximum input value.
        source_max: f64,
        /// Clamp values outside the declared range.
        clamp: bool,
    },
    /// Apply a linear scale/offset transform.
    ScaleOffset {
        /// Multiplicative scale factor.
        scale: f64,
        /// Additive offset.
        offset: f64,
    },
    /// Apply gamma correction to normalized values.
    Gamma {
        /// Gamma exponent.
        gamma: f64,
    },
}

/// Derived raster product requested before styling.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderRasterDerivedProduct {
    /// Use the source band values directly.
    Raw,
    /// Derive analytic hillshade from an elevation raster.
    Hillshade {
        /// Illumination azimuth in degrees.
        azimuth_degrees: f64,
        /// Illumination altitude in degrees.
        altitude_degrees: f64,
        /// Z-factor applied during hillshade computation.
        z_factor: f64,
    },
    /// Derive slope magnitude from an elevation raster.
    Slope,
    /// Derive aspect/azimuth from an elevation raster.
    Aspect,
    /// Derive contour isolines from an elevation raster.
    Contours {
        /// Elevation interval between adjacent contour lines.
        interval: f64,
        /// Optional contour base elevation from which intervals are measured.
        base: Option<f64>,
        /// Whether contour paths should be smoothed before rendering.
        smooth: bool,
    },
    /// Derive a tangent-space normal map from an elevation raster.
    NormalMap {
        /// Strength multiplier applied when computing normals.
        strength: f64,
    },
    /// Interpret the raster as a terrain heightfield.
    Heightfield,
}

/// Symbolizer for raster layers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderRasterStyleDescriptor {
    /// Overall raster opacity.
    pub opacity: f32,
    /// Raster band-selection policy.
    pub band_selection: RenderRasterBandSelection,
    /// Sampling/interpolation policy.
    pub sampling: RenderRasterSampling,
    /// Optional value transforms applied after band selection.
    pub value_transforms: Vec<RenderRasterValueTransform>,
    /// Optional derived product to compute from the selected source data.
    pub derived_product: Option<RenderRasterDerivedProduct>,
    /// Optional single-band color ramp applied after band selection.
    pub color_ramp: Option<RenderRasterColorRampDescriptor>,
    /// Whether to respect the source photometric interpretation / colormap.
    pub respect_source_colormap: bool,
    /// Whether pixels marked as no-data should render transparent.
    pub nodata_transparent: bool,
    /// Optional material and pipeline hints for raster-backed surfaces.
    pub material_intent: Option<RenderMaterialIntentDescriptor>,
}

/// Serialisable raster header summary derived from `georaster::geotiff::ImageInfo`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderRasterImageMetadataDescriptor {
    /// Raster width in pixels, if known.
    pub width: Option<u32>,
    /// Raster height in pixels, if known.
    pub height: Option<u32>,
    /// Number of samples/bands.
    pub sample_count: u8,
    /// Raster color model if the source declared one.
    pub color_type: Option<String>,
    /// Photometric interpretation string when present.
    pub photometric_interpretation: Option<String>,
    /// Planar-configuration string when present.
    pub planar_configuration: Option<String>,
}

impl From<ImageInfo> for RenderRasterImageMetadataDescriptor {
    fn from(value: ImageInfo) -> Self {
        let (width, height) = value.dimensions.map(|(width, height)| (Some(width), Some(height))).unwrap_or((None, None));
        Self {
            width,
            height,
            sample_count: value.samples,
            color_type: value.colortype.map(|color| format!("{color:?}")),
            photometric_interpretation: value
                .photometric_interpretation
                .map(|value| format!("{value:?}")),
            planar_configuration: value.planar_config.map(|value| format!("{value:?}")),
        }
    }
}

/// Georeferencing sidecar required to place a raster in world space.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderRasterGeoreferenceDescriptor {
    /// Optional CRS authority name such as `EPSG`.
    pub crs_authority: Option<String>,
    /// Optional CRS code such as `4326` or `3857`.
    pub crs_code: Option<String>,
    /// Minimum X coordinate of the raster extent.
    pub min_x: f64,
    /// Minimum Y coordinate of the raster extent.
    pub min_y: f64,
    /// Maximum X coordinate of the raster extent.
    pub max_x: f64,
    /// Maximum Y coordinate of the raster extent.
    pub max_y: f64,
}

/// Externally sourced raster payload descriptor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderRasterSourceDescriptor {
    /// Resolved asset reference for the raster bytes.
    pub asset: RenderAssetReference,
    /// World-space georeferencing sidecar.
    pub georeference: RenderRasterGeoreferenceDescriptor,
}

/// Tile addressing scheme used by a tile source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderTileAddressingScheme {
    /// XYZ addressing with origin at the upper-left.
    Xyz,
    /// TMS addressing with origin at the lower-left.
    Tms,
    /// WMTS matrix-based addressing.
    Wmts,
}

/// Tile payload family served by a tile source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderTilePayloadKind {
    /// Imagery or raster tiles.
    Raster,
    /// Vector-feature tiles.
    Vector,
}

/// Fallback behavior when a requested tile is unavailable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderTileMissingTilePolicy {
    /// Render nothing for the missing tile.
    Transparent,
    /// Reuse an already available lower-resolution ancestor tile.
    UseAncestor,
    /// Keep the previously displayed tile content until replacement arrives.
    HoldPrevious,
}

/// Refinement priority policy for visible map tiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderTileRefinementPriority {
    /// Balance coarse coverage and local detail.
    Balanced,
    /// Prioritize loading the tile under the camera or view center first.
    CenterFirst,
    /// Prioritize maintaining complete coarse coverage before local detail.
    CoarseCoverageFirst,
}

/// Retry policy for transient tile-request failures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RenderTileRetryPolicyDescriptor {
    /// Maximum number of retry attempts after the initial request.
    pub max_retries: u8,
    /// Backoff interval in milliseconds between retry attempts.
    pub backoff_millis: u32,
}

/// Streaming and refinement policy for a map-tile source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderTileStreamingDescriptor {
    /// Whether the tileset should wrap horizontally across the antimeridian.
    pub wrap_horizontally: bool,
    /// Number of neighboring tile rings to prefetch around visible tiles.
    pub prefetch_ring_count: u8,
    /// Maximum in-flight requests the backend should schedule at once.
    pub max_concurrent_requests: u16,
    /// Whether lower-resolution tiles may remain visible while refinements load.
    pub retain_lower_resolution_tiles: bool,
    /// Relative refinement priority for visible tiles.
    pub refinement_priority: RenderTileRefinementPriority,
    /// Optional cache budget expressed as the maximum number of retained tiles.
    pub max_cached_tiles: Option<u32>,
    /// Optional retry policy for transient request failures.
    pub retry: Option<RenderTileRetryPolicyDescriptor>,
    /// Optional cross-fade duration for tile replacement transitions.
    pub transition_fade_millis: Option<u32>,
    /// Fallback policy used when a tile request fails or is absent.
    pub missing_tile_policy: RenderTileMissingTilePolicy,
}

/// Externally sourced map-tile descriptor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderTileSourceDescriptor {
    /// URI template with placeholders such as `{z}`, `{x}`, and `{y}`.
    pub uri_template: String,
    /// Optional metadata endpoint describing the tileset.
    pub metadata_uri: Option<Url>,
    /// Tile addressing scheme.
    pub scheme: RenderTileAddressingScheme,
    /// Payload family produced by this tileset.
    pub payload_kind: RenderTilePayloadKind,
    /// Optional CRS authority name such as `EPSG`.
    pub crs_authority: Option<String>,
    /// Optional CRS code such as `3857`.
    pub crs_code: Option<String>,
    /// Minimum available zoom level.
    pub min_zoom: u8,
    /// Maximum available zoom level.
    pub max_zoom: u8,
    /// Tile width in pixels.
    pub tile_width: u32,
    /// Tile height in pixels.
    pub tile_height: u32,
    /// Optional streaming/refinement policy for the tileset.
    pub streaming: Option<RenderTileStreamingDescriptor>,
    /// Optional attribution or credit string.
    pub attribution: Option<String>,
}

impl JsonSchema for RenderTileSourceDescriptor {
    fn schema_name() -> Cow<'static, str> {
        "RenderTileSourceDescriptor".into()
    }

    fn json_schema(_schema_gen: &mut SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "object",
            "properties": {
                "uri_template": { "type": "string" },
                "metadata_uri": {
                    "type": ["string", "null"],
                    "format": "uri"
                },
                "scheme": { "type": "string" },
                "payload_kind": { "type": "string" },
                "crs_authority": { "type": ["string", "null"] },
                "crs_code": { "type": ["string", "null"] },
                "min_zoom": {
                    "type": "integer",
                    "minimum": 0
                },
                "max_zoom": {
                    "type": "integer",
                    "minimum": 0
                },
                "tile_width": {
                    "type": "integer",
                    "minimum": 1
                },
                "tile_height": {
                    "type": "integer",
                    "minimum": 1
                },
                "streaming": {
                    "type": ["object", "null"]
                },
                "attribution": { "type": ["string", "null"] }
            },
            "required": [
                "uri_template",
                "scheme",
                "payload_kind",
                "min_zoom",
                "max_zoom",
                "tile_width",
                "tile_height"
            ]
        })
    }
}

/// Styling policy for a tile layer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderTileStyleDescriptor {
    /// Overall tile-layer opacity.
    pub opacity: f32,
    /// Optional raster imagery styling for raster tiles.
    pub raster_style: Option<RenderRasterStyleDescriptor>,
    /// Optional feature styling for vector tiles.
    pub vector_style: Option<RenderFeatureStyleDescriptor>,
    /// Optional material and pipeline hints for tile-backed surfaces.
    pub material_intent: Option<RenderMaterialIntentDescriptor>,
}

/// Terrain meshing strategy for elevation rasters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderTerrainMeshMode {
    /// Regular grid heightfield mesh.
    Heightfield,
    /// Explicit grid mesh suitable for terrain LOD.
    GridMesh,
    /// Pre-triangulated terrain mesh.
    TriangleMesh,
}

/// Terrain shading family requested by the scene.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderTerrainShadingMode {
    /// Unlit terrain.
    Unlit,
    /// Lit terrain with simple diffuse shading.
    Lambert,
    /// Lit terrain with physically based shading inputs.
    Pbr,
}

/// Drape strategy used when projecting imagery or overlays onto terrain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderTerrainDrapeMode {
    /// Use the source georeference directly when sampling or projecting.
    Georeferenced,
    /// Project the content using terrain-local meter coordinates.
    LocalMeters,
    /// Sample the content in Web Mercator space.
    WebMercator,
}

/// Blend mode used when compositing terrain overlays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderTerrainOverlayBlendMode {
    /// Replace the underlying surface contribution.
    Replace,
    /// Alpha-blend over the underlying surface.
    AlphaBlend,
    /// Multiply against the underlying surface.
    Multiply,
    /// Add to the underlying surface.
    Add,
}

/// Terrain LOD and stitch policy for elevation-driven meshes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderTerrainLodDescriptor {
    /// Maximum tolerated screen-space error before refinement.
    pub max_screen_space_error: f64,
    /// Optional skirt height used to hide cracks between terrain tiles or chunks.
    pub skirt_height: Option<f64>,
    /// Whether the backend may morph between adjacent LOD levels.
    pub morph_between_levels: bool,
}

/// One composited overlay source draped or projected onto the terrain surface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderTerrainOverlaySource {
    /// Texture or imagery asset consumed directly by the backend.
    Texture(RenderAssetReference),
    /// Raster style sidecar interpreted against the terrain source.
    RasterStyle(RenderRasterStyleDescriptor),
}

/// One composited overlay draped or projected onto the terrain surface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderTerrainOverlayDescriptor {
    /// Overlay-source payload.
    pub source: RenderTerrainOverlaySource,
    /// Overlay opacity in the unit interval.
    pub opacity: f32,
    /// Drape/projection strategy for the overlay.
    pub drape_mode: RenderTerrainDrapeMode,
    /// Blend mode used when compositing the overlay.
    pub blend_mode: RenderTerrainOverlayBlendMode,
    /// Height bias used to avoid z-fighting against the terrain surface.
    pub height_bias: f64,
}

/// Terrain rendering policy derived from an elevation raster.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderTerrainStyleDescriptor {
    /// Vertical scale applied to height samples.
    pub vertical_scale: f64,
    /// Base height offset applied to the terrain mesh.
    pub base_height_offset: f64,
    /// Meshing strategy for the terrain surface.
    pub mesh_mode: RenderTerrainMeshMode,
    /// Requested shading family.
    pub shading_mode: RenderTerrainShadingMode,
    /// Optional texture asset to drape over the terrain.
    pub surface_texture: Option<RenderAssetReference>,
    /// Optional raster-style sidecar used to derive surface color from the source.
    pub surface_raster_style: Option<RenderRasterStyleDescriptor>,
    /// Optional drape policy for the primary surface contribution.
    pub surface_drape_mode: Option<RenderTerrainDrapeMode>,
    /// Additional ordered overlays composited onto the terrain.
    pub overlays: Vec<RenderTerrainOverlayDescriptor>,
    /// Optional LOD/stitch policy for large terrain surfaces.
    pub lod: Option<RenderTerrainLodDescriptor>,
    /// Whether the terrain should render in wireframe mode.
    pub wireframe: bool,
    /// Whether back faces should remain visible.
    pub double_sided: bool,
    /// Optional shadow-participation policy.
    pub shadow_participation: Option<RenderShadowParticipationDescriptor>,
    /// Optional material and pipeline hints for terrain surfaces.
    pub material_intent: Option<RenderMaterialIntentDescriptor>,
}

/// Camera projection policy for a render view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderCameraProjectionDescriptor {
    /// Orthographic projection suitable for 2D or 2.5D map views.
    Orthographic {
        /// Near clipping plane in renderer-local scene units.
        near: f64,
        /// Far clipping plane in renderer-local scene units.
        far: f64,
    },
    /// Perspective projection suitable for 3D or oblique views.
    Perspective {
        /// Vertical field of view in degrees.
        vertical_fov_degrees: f64,
        /// Near clipping plane in renderer-local scene units.
        near: f64,
        /// Far clipping plane in renderer-local scene units.
        far: f64,
    },
}

/// Tonemapping operator applied to the rendered camera output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderTonemappingDescriptor {
    /// Leave HDR color values un-tonemapped.
    None,
    /// Apply Reinhard tonemapping.
    Reinhard,
    /// Apply luminance-aware Reinhard tonemapping.
    ReinhardLuminance,
    /// Apply the ACES fitted curve.
    AcesFitted,
    /// Apply AgX tonemapping.
    AgX,
    /// Apply Bevy's "SomewhatBoringDisplayTransform".
    SomewhatBoringDisplayTransform,
    /// Apply the TonyMcMapface curve.
    TonyMcMapface,
    /// Apply Blender's filmic curve.
    BlenderFilmic,
}

/// Multi-sample anti-aliasing policy for a render view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderMsaaDescriptor {
    /// Disable MSAA.
    Off,
    /// Use 2x MSAA.
    Sample2,
    /// Use 4x MSAA.
    Sample4,
    /// Use 8x MSAA.
    Sample8,
}

/// Shadow-edge filtering family for shadow-aware cameras.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderShadowFilteringMethod {
    /// Minimal hardware 2x2 filtering.
    Hardware2x2,
    /// Gaussian shadow filtering.
    Gaussian,
    /// Temporal shadow filtering.
    Temporal,
}

/// Per-tone-range color grading parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderColorGradingSectionDescriptor {
    /// Saturation multiplier.
    pub saturation: f32,
    /// Contrast multiplier.
    pub contrast: f32,
    /// Gamma exponent.
    pub gamma: f32,
    /// Gain multiplier.
    pub gain: f32,
    /// Lift offset.
    pub lift: f32,
}

/// Whole-image color grading parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderColorGradingGlobalDescriptor {
    /// Exposure value offset in stops.
    pub exposure: f32,
    /// White-balance temperature offset.
    pub temperature: f32,
    /// White-balance tint offset.
    pub tint: f32,
    /// Hue rotation in radians.
    pub hue: f32,
    /// Post-tonemapping saturation multiplier.
    pub post_saturation: f32,
    /// Lower bound of the midtones luminance range.
    pub midtones_range_start: f32,
    /// Upper bound of the midtones luminance range.
    pub midtones_range_end: f32,
}

/// Filmic color-grading policy for a render view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderColorGradingDescriptor {
    /// Global whole-image grading.
    pub global: RenderColorGradingGlobalDescriptor,
    /// Shadow-region grading.
    pub shadows: RenderColorGradingSectionDescriptor,
    /// Midtone-region grading.
    pub midtones: RenderColorGradingSectionDescriptor,
    /// Highlight-region grading.
    pub highlights: RenderColorGradingSectionDescriptor,
}

/// Named fixed-exposure baselines aligned with common realtime-renderer presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderExposurePreset {
    /// Daylight exterior baseline.
    Sunlight,
    /// Bright but overcast exterior baseline.
    Overcast,
    /// Typical interior baseline.
    Indoor,
    /// Blender-style default baseline.
    Blender,
}

/// Fixed camera exposure policy for a render view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderExposureDescriptor {
    /// Use a named exposure baseline.
    Preset(RenderExposurePreset),
    /// Use an explicit EV100 value.
    Ev100(f32),
}

/// Automatic exposure adaptation policy for a render view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderAutoExposureDescriptor {
    /// Minimum exposure value in EV stops.
    pub range_start: f32,
    /// Maximum exposure value in EV stops.
    pub range_end: f32,
    /// Lower percentile of the luminance filter window.
    pub filter_start: f32,
    /// Upper percentile of the luminance filter window.
    pub filter_end: f32,
    /// Adaptation speed when brightening.
    pub speed_brighten: f32,
    /// Adaptation speed when darkening.
    pub speed_darken: f32,
    /// Transition distance between linear and exponential adaptation.
    pub exponential_transition_distance: f32,
    /// Optional luminance metering mask asset.
    pub metering_mask: Option<RenderAssetReference>,
    /// Optional auto-exposure compensation-curve asset.
    pub compensation_curve: Option<RenderAssetReference>,
}

/// Bloom composite policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderBloomCompositeMode {
    /// Preserve energy when recombining bloom.
    EnergyConserving,
    /// Additively blend bloom back into the image.
    Additive,
}

/// Bloom prefilter controls.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderBloomPrefilterDescriptor {
    /// Threshold above which pixels contribute to bloom.
    pub threshold: f32,
    /// Softness applied around the threshold.
    pub threshold_softness: f32,
}

/// Bloom post-process policy for a render view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderBloomDescriptor {
    /// Bloom intensity multiplier.
    pub intensity: f32,
    /// Low-frequency boost amount.
    pub low_frequency_boost: f32,
    /// Curvature of the low-frequency boost response.
    pub low_frequency_boost_curvature: f32,
    /// High-pass frequency used by the bloom prefilter.
    pub high_pass_frequency: f32,
    /// Prefilter controls.
    pub prefilter: RenderBloomPrefilterDescriptor,
    /// Bloom composite mode.
    pub composite_mode: RenderBloomCompositeMode,
    /// Maximum mip dimension used during bloom downsampling.
    pub max_mip_dimension: u32,
    /// Bloom scale factor along X.
    pub scale_x: f32,
    /// Bloom scale factor along Y.
    pub scale_y: f32,
}

/// Depth-of-field blur kernel family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderDepthOfFieldMode {
    /// Physically styled bokeh blur.
    Bokeh,
    /// Gaussian blur approximation.
    Gaussian,
}

/// Depth-of-field post-process policy for a render view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderDepthOfFieldDescriptor {
    /// Depth-of-field blur family.
    pub mode: RenderDepthOfFieldMode,
    /// Focal distance in meters.
    pub focal_distance: f32,
    /// Virtual camera sensor height in millimeters.
    pub sensor_height: f32,
    /// Aperture expressed as f-stops.
    pub aperture_f_stops: f32,
    /// Maximum circle-of-confusion diameter in normalized screen units.
    pub max_circle_of_confusion_diameter: f32,
    /// Maximum depth distance participating in the effect.
    pub max_depth: f32,
}

/// Motion-blur post-process policy for a render view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderMotionBlurDescriptor {
    /// Shutter angle controlling blur intensity.
    pub shutter_angle: f32,
    /// Number of motion-blur samples.
    pub samples: u32,
}

/// Chromatic-aberration post-process policy for a render view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderChromaticAberrationDescriptor {
    /// Aberration intensity multiplier.
    pub intensity: f32,
    /// Maximum chromatic-aberration sample count.
    pub max_samples: u32,
    /// Optional lookup texture controlling channel distortion.
    pub color_lut: Option<RenderAssetReference>,
}

/// Screen-space-reflection post-process policy for a render view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderScreenSpaceReflectionsDescriptor {
    /// Upper roughness threshold eligible for the effect.
    pub perceptual_roughness_threshold: Option<f32>,
    /// Assumed surface thickness used when resolving intersections.
    pub thickness: Option<f32>,
    /// Number of initial linear ray-marching steps.
    pub linear_steps: Option<u32>,
    /// Exponent applied to the linear-march sample distribution.
    pub linear_march_exponent: Option<f32>,
    /// Number of refinement steps used after the initial march.
    pub bisection_steps: Option<u32>,
    /// Whether secant refinement is enabled.
    pub use_secant: Option<bool>,
}

/// Quality tier for screen-space ambient occlusion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RenderScreenSpaceAmbientOcclusionQuality {
    /// Minimal sample budget.
    Low,
    /// Balanced default sample budget.
    Medium,
    /// High-quality sample budget.
    High,
    /// Maximum practical sample budget.
    Ultra,
    /// Explicit custom sample budget.
    Custom,
}

/// Screen-space ambient-occlusion post-process policy for a render view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderScreenSpaceAmbientOcclusionDescriptor {
    /// Quality tier for the effect.
    pub quality: RenderScreenSpaceAmbientOcclusionQuality,
    /// Optional custom slice count used when `quality` is `Custom`.
    pub slice_count: Option<u32>,
    /// Optional custom samples-per-slice-side used when `quality` is `Custom`.
    pub samples_per_slice_side: Option<u32>,
    /// Optional constant estimated object thickness.
    pub constant_object_thickness: Option<f32>,
}

/// Quality hint for screen-space transmission / refraction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderScreenSpaceTransmissionQuality {
    /// Minimal sampling budget.
    Low,
    /// Balanced default sampling budget.
    Medium,
    /// High-quality sampling budget.
    High,
    /// Maximum practical sampling budget.
    Ultra,
}

/// Destination surface targeted by one render view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderOutputTargetDescriptor {
    /// The renderer's primary presentation surface.
    PrimarySurface,
    /// A named or externally managed presentation surface.
    SurfaceId {
        /// Stable symbolic target identifier.
        surface_id: String,
    },
    /// An offscreen image asset target.
    ImageAsset {
        /// Asset bound as the offscreen image target.
        asset: RenderAssetReference,
    },
    /// A texture-view asset target.
    TextureViewAsset {
        /// Asset bound as the explicit texture-view target.
        asset: RenderAssetReference,
    },
}

/// Color-clear policy applied before rendering one view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderClearColorPolicy {
    /// Use the scene environment's background color when available.
    UseSceneBackground,
    /// Use a per-view explicit clear color.
    ClearColor(UiColor),
    /// Preserve previous contents and do not clear color.
    Preserve,
}

/// Per-view clear behavior for color and optional depth buffers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderClearPolicyDescriptor {
    /// Color-clear policy for the view.
    pub color: RenderClearColorPolicy,
    /// Optional depth clear value.
    pub depth: Option<f32>,
}

/// Camera-output policy for a render view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderViewOutputDescriptor {
    /// Whether the view should render in HDR.
    pub hdr: bool,
    /// Optional explicit output target override.
    pub target: Option<RenderOutputTargetDescriptor>,
    /// Optional per-view clear policy override.
    pub clear_policy: Option<RenderClearPolicyDescriptor>,
    /// Optional tonemapping operator.
    pub tonemapping: Option<RenderTonemappingDescriptor>,
    /// Optional MSAA policy.
    pub msaa: Option<RenderMsaaDescriptor>,
    /// Optional mip-bias override.
    pub mip_bias: Option<f32>,
    /// Optional fixed exposure policy.
    pub exposure: Option<RenderExposureDescriptor>,
    /// Optional color-grading policy.
    pub color_grading: Option<RenderColorGradingDescriptor>,
    /// Optional automatic exposure policy.
    pub auto_exposure: Option<RenderAutoExposureDescriptor>,
    /// Optional bloom policy.
    pub bloom: Option<RenderBloomDescriptor>,
    /// Optional depth-of-field policy.
    pub depth_of_field: Option<RenderDepthOfFieldDescriptor>,
    /// Optional motion-blur policy.
    pub motion_blur: Option<RenderMotionBlurDescriptor>,
    /// Optional chromatic-aberration policy.
    pub chromatic_aberration: Option<RenderChromaticAberrationDescriptor>,
    /// Optional screen-space reflection policy.
    pub screen_space_reflections: Option<RenderScreenSpaceReflectionsDescriptor>,
    /// Optional screen-space ambient-occlusion policy.
    pub screen_space_ambient_occlusion:
        Option<RenderScreenSpaceAmbientOcclusionDescriptor>,
    /// Optional shadow-edge filtering policy.
    pub shadow_filtering_method: Option<RenderShadowFilteringMethod>,
    /// Optional quality hint for screen-space transmission.
    pub screen_space_transmission_quality:
        Option<RenderScreenSpaceTransmissionQuality>,
}

/// Renderer-agnostic map view.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderViewDescriptor {
    /// Optional CRS authority name such as `EPSG`.
    pub crs_authority: Option<String>,
    /// Optional CRS code such as `4326` or `3857`.
    pub crs_code: Option<String>,
    /// View center X coordinate in the declared CRS.
    pub center_x: f64,
    /// View center Y coordinate in the declared CRS.
    pub center_y: f64,
    /// Map resolution / world-units-per-pixel scale.
    pub resolution: f64,
    /// Viewport width in logical pixels.
    pub viewport_width: u32,
    /// Viewport height in logical pixels.
    pub viewport_height: u32,
    /// Clockwise rotation in degrees.
    pub rotation_degrees: f64,
    /// Pitch/tilt angle in degrees.
    pub pitch_degrees: f64,
    /// Camera projection policy used to interpret the view.
    pub projection: RenderCameraProjectionDescriptor,
    /// Optional camera-output policy for tonemapping and post-processing.
    pub output: Option<RenderViewOutputDescriptor>,
    /// Optional world-space mapping policy for 2.5D/3D renderers.
    pub world_space: Option<RenderWorldSpaceDescriptor>,
}

/// Per-layer metadata supplied before payload validation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderLayerSpec {
    /// Stable layer identifier.
    pub id: String,
    /// User-facing or programmatic layer name.
    pub name: String,
    /// Deterministic draw order. Lower values render first.
    pub draw_order: i32,
    /// Whether the layer should be visible by default.
    pub visible: bool,
    /// Overall layer opacity.
    pub opacity: f32,
}

/// Per-scene metadata supplied before scene validation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderSceneSpec {
    /// Stable scene identifier.
    pub scene_id: String,
    /// Optional scene environment and lighting policy.
    pub environment: Option<RenderSceneEnvironmentDescriptor>,
}

/// Renderer-neutral ambient light descriptor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderAmbientLightDescriptor {
    /// Ambient light color.
    pub color: UiColor,
    /// Ambient brightness multiplier.
    pub brightness: f32,
}

/// Renderer-neutral directional light descriptor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderDirectionalLightDescriptor {
    /// Directional light color.
    pub color: UiColor,
    /// Light illuminance/intensity.
    pub illuminance: f32,
    /// Horizontal sun azimuth in degrees.
    pub azimuth_degrees: f32,
    /// Vertical sun elevation in degrees.
    pub elevation_degrees: f32,
    /// Whether shadow casting is enabled.
    pub shadows_enabled: bool,
    /// Optional directional cascade-shadow policy.
    pub cascade_shadows: Option<RenderCascadeShadowConfigDescriptor>,
}

/// Directional-light cascade-shadow policy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderCascadeShadowConfigDescriptor {
    /// Optional explicit cascade far bounds.
    pub bounds: Option<Vec<f32>>,
    /// Optional overlap proportion between cascades.
    pub overlap_proportion: Option<f32>,
    /// Optional minimum shadow distance.
    pub minimum_distance: Option<f32>,
    /// Optional maximum shadow distance.
    pub maximum_distance: Option<f32>,
    /// Optional far bound of the first cascade.
    pub first_cascade_far_bound: Option<f32>,
    /// Optional number of cascades for builder-style consumers.
    pub num_cascades: Option<u32>,
}

/// Renderer-neutral fog descriptor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderFogFalloffDescriptor {
    /// Linear fog ramp between two distances.
    Linear {
        /// Distance at which fog begins.
        start: f32,
        /// Distance at which fog reaches full strength.
        end: f32,
    },
    /// Exponential fog with a single density coefficient.
    Exponential {
        /// Exponential density coefficient.
        density: f32,
    },
    /// Squared exponential fog with a single density coefficient.
    ExponentialSquared {
        /// Squared exponential density coefficient.
        density: f32,
    },
    /// Atmospheric fog with extinction and inscattering coefficients.
    Atmospheric {
        /// Red extinction coefficient.
        extinction_r: f32,
        /// Green extinction coefficient.
        extinction_g: f32,
        /// Blue extinction coefficient.
        extinction_b: f32,
        /// Red inscattering coefficient.
        inscattering_r: f32,
        /// Green inscattering coefficient.
        inscattering_g: f32,
        /// Blue inscattering coefficient.
        inscattering_b: f32,
    },
}

/// Renderer-neutral fog descriptor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderFogDescriptor {
    /// Fog color.
    pub color: UiColor,
    /// Optional directional-light contribution color.
    pub directional_light_color: Option<UiColor>,
    /// Optional directional-light scatter exponent.
    pub directional_light_exponent: Option<f32>,
    /// Fog attenuation model.
    pub falloff: RenderFogFalloffDescriptor,
}

/// Camera-based volumetric fog policy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderVolumetricFogDescriptor {
    /// Optional ambient volumetric color.
    pub ambient_color: Option<UiColor>,
    /// Optional ambient volumetric intensity.
    pub ambient_intensity: Option<f32>,
    /// Optional ray-origin jitter distance in meters.
    pub jitter: Option<f32>,
    /// Optional volumetric raymarch step count.
    pub step_count: Option<u32>,
}

/// Axis-aligned localized probe region in renderer-local scene space.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderProbeRegionDescriptor {
    /// Region center X coordinate.
    pub center_x: f32,
    /// Region center Y coordinate.
    pub center_y: f32,
    /// Region center Z coordinate.
    pub center_z: f32,
    /// Full width of the probe region along X.
    pub size_x: f32,
    /// Full height of the probe region along Y.
    pub size_y: f32,
    /// Full depth of the probe region along Z.
    pub size_z: f32,
}

/// Irradiance-volume payload used for localized indirect lighting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderIrradianceVolumeDescriptor {
    /// Voxel texture asset containing irradiance data.
    pub voxels: RenderAssetReference,
    /// Optional non-negative radiance scale factor.
    pub intensity: Option<f32>,
    /// Whether the probe should affect lightmapped meshes when supported.
    pub affects_lightmapped_meshes: Option<bool>,
}

/// Localized indirect-light probe region embedded in scene space.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderLightProbeDescriptor {
    /// Bounded region influenced by this probe.
    pub region: RenderProbeRegionDescriptor,
    /// Optional irradiance-volume payload for the probe region.
    pub irradiance_volume: Option<RenderIrradianceVolumeDescriptor>,
}

/// Offset applied to a local fog volume's density texture.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderFogVolumeOffsetDescriptor {
    /// Offset X component.
    pub x: f32,
    /// Offset Y component.
    pub y: f32,
    /// Offset Z component.
    pub z: f32,
}

/// Localized volumetric medium embedded in world space.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderFogVolumeDescriptor {
    /// Optional fog albedo / tint color.
    pub fog_color: Option<UiColor>,
    /// Optional overall density multiplier.
    pub density_factor: Option<f32>,
    /// Optional density texture controlling heterogeneous media.
    pub density_texture: Option<RenderAssetReference>,
    /// Optional density-texture offset in local volume coordinates.
    pub density_texture_offset: Option<RenderFogVolumeOffsetDescriptor>,
    /// Optional absorption coefficient.
    pub absorption: Option<f32>,
    /// Optional scattering coefficient.
    pub scattering: Option<f32>,
    /// Optional anisotropy factor.
    pub scattering_asymmetry: Option<f32>,
    /// Optional artist-facing light tint.
    pub light_tint: Option<UiColor>,
    /// Optional nonphysical light intensity multiplier.
    pub light_intensity: Option<f32>,
}

/// Scene-level environment and lighting policy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderSceneEnvironmentDescriptor {
    /// Optional background clear color.
    pub background_color: Option<UiColor>,
    /// Optional skybox / skydome environment asset.
    pub skybox: Option<RenderSkyboxDescriptor>,
    /// Optional image-based lighting policy derived from environment assets.
    pub image_based_lighting: Option<RenderImageBasedLightingDescriptor>,
    /// Optional participating-media atmosphere policy.
    pub atmosphere: Option<RenderAtmosphereDescriptor>,
    /// Explicit localized indirect-light probe regions.
    pub light_probes: Vec<RenderLightProbeDescriptor>,
    /// Optional ambient-light policy.
    pub ambient_light: Option<RenderAmbientLightDescriptor>,
    /// Optional directional light policy.
    pub sun_light: Option<RenderDirectionalLightDescriptor>,
    /// Optional fog policy.
    pub fog: Option<RenderFogDescriptor>,
    /// Optional volumetric fog policy.
    pub volumetric_fog: Option<RenderVolumetricFogDescriptor>,
    /// Explicit localized fog volumes embedded in the scene.
    pub fog_volumes: Vec<RenderFogVolumeDescriptor>,
}

/// Source material used to derive image-based lighting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderImageBasedLightingSourceDescriptor {
    /// Explicit diffuse/specular cubemap pair.
    Cubemaps {
        /// Diffuse irradiance cubemap asset.
        diffuse_map: RenderAssetReference,
        /// Specular reflection cubemap asset.
        specular_map: RenderAssetReference,
    },
    /// One upstream environment map asset from which lighting data is derived.
    EnvironmentMap {
        /// Environment map asset used to derive lighting.
        environment_map: RenderAssetReference,
    },
    /// Lighting generated from the declared atmosphere.
    Atmosphere {
        /// Optional generated cubemap dimensions.
        cubemap_size: Option<[u32; 2]>,
    },
}

/// Scene-level image-based lighting policy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderImageBasedLightingDescriptor {
    /// Source used to derive image-based lighting.
    pub source: RenderImageBasedLightingSourceDescriptor,
    /// Optional non-negative lighting intensity multiplier.
    pub intensity: Option<f32>,
    /// Optional yaw rotation in degrees.
    pub rotation_degrees: Option<f32>,
    /// Whether diffuse light should affect lightmapped meshes when supported.
    pub affects_lightmapped_mesh_diffuse: Option<bool>,
}

/// Atmosphere density falloff policy for a scattering term.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderAtmosphereFalloffDescriptor {
    /// Linear falloff from the base shell outward.
    Linear,
    /// Exponential falloff controlled by a proportional scale.
    Exponential {
        /// Non-negative exponential scale factor.
        scale: f32,
    },
    /// Tent-shaped falloff centered within the normalized shell depth.
    Tent {
        /// Normalized center of the tent peak.
        center: f32,
        /// Normalized total width of the tent peak.
        width: f32,
    },
}

/// Scattering phase function used by an atmosphere term.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderAtmospherePhaseFunctionDescriptor {
    /// Uniform scattering in all directions.
    Isotropic,
    /// Rayleigh scattering approximation.
    Rayleigh,
    /// Mie scattering approximation with a directional asymmetry factor.
    Mie {
        /// Henyey-Greenstein asymmetry factor in the normalized interval.
        asymmetry: f32,
    },
}

/// One scattering term contributing to the scene atmosphere.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderAtmosphereScatteringTermDescriptor {
    /// RGB absorption coefficients for this medium term.
    pub absorption: [f32; 3],
    /// RGB scattering coefficients for this medium term.
    pub scattering: [f32; 3],
    /// Density falloff policy across the shell depth.
    pub falloff: RenderAtmosphereFalloffDescriptor,
    /// Phase function describing scattering directionality.
    pub phase: RenderAtmospherePhaseFunctionDescriptor,
}

/// Participating-media atmosphere policy for the scene.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderAtmosphereDescriptor {
    /// Radius of the lower atmosphere shell in world units.
    pub bottom_radius: f32,
    /// Radius of the upper atmosphere shell in world units.
    pub top_radius: f32,
    /// Ground albedo used when light reflects from the planet surface.
    pub ground_albedo: [f32; 3],
    /// Resolution used when precomputing falloff tables.
    pub falloff_resolution: u32,
    /// Resolution used when precomputing phase-function tables.
    pub phase_resolution: u32,
    /// At least one participating scattering / absorption term.
    pub terms: Vec<RenderAtmosphereScatteringTermDescriptor>,
    /// Optional density multiplier applied to the combined medium.
    pub density_multiplier: Option<f32>,
    /// Optional visible solar disk for atmosphere-aware renderers.
    pub sun_disk: Option<RenderSunDiskDescriptor>,
}

/// Visible solar-disk policy for atmosphere-aware renderers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderSunDiskDescriptor {
    /// Optional angular size in radians.
    pub angular_size: Option<f32>,
    /// Optional non-negative intensity multiplier.
    pub intensity: Option<f32>,
}

/// Renderer-neutral skybox descriptor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderSkyboxDescriptor {
    /// Environment asset backing the skybox.
    pub asset: RenderAssetReference,
    /// Non-negative brightness multiplier.
    pub brightness: f32,
    /// Optional yaw rotation in degrees.
    pub rotation_degrees: Option<f32>,
}

/// Geometry payload used for annotation anchors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RenderAnnotationGeometryAnchor(pub Geometry<f64>);

impl JsonSchema for RenderAnnotationGeometryAnchor {
    fn schema_name() -> Cow<'static, str> {
        "RenderAnnotationGeometryAnchor".into()
    }

    fn json_schema(_schema_gen: &mut SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "object",
            "description": "Serialized geo_types::Geometry<f64> anchor payload for annotation placement."
        })
    }
}

/// Anchor target for renderer-agnostic annotations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderAnnotationAnchorDescriptor {
    /// Anchor in world space at an explicit point.
    WorldPoint {
        /// World X coordinate.
        x: f64,
        /// World Y coordinate.
        y: f64,
        /// Optional world Z coordinate.
        z: Option<f64>,
    },
    /// Anchor in screen space at an explicit point.
    ScreenPoint {
        /// Screen X coordinate in logical pixels.
        x: f64,
        /// Screen Y coordinate in logical pixels.
        y: f64,
    },
    /// Anchor at the centroid of the supplied geometry.
    GeometryCentroid(RenderAnnotationGeometryAnchor),
    /// Anchor at an interior point of the supplied geometry.
    GeometryInteriorPoint(RenderAnnotationGeometryAnchor),
}

/// Explicit annotation rendered independently from feature payloads.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderAnnotationDescriptor {
    /// Stable annotation identifier.
    pub id: String,
    /// Whether the annotation is visible by default.
    pub visible: bool,
    /// How the annotation is anchored.
    pub anchor: RenderAnnotationAnchorDescriptor,
    /// Optional visibility range for the annotation.
    pub resolution_range: Option<RenderScaleRangeDescriptor>,
    /// Symbolizer used to render the annotation.
    pub symbolizer: RenderFeatureSymbolizerDescriptor,
}

/// Vector payload carried directly in the render IR.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RenderGeometryPayload(pub Vec<Geometry<f64>>);

impl JsonSchema for RenderGeometryPayload {
    fn schema_name() -> Cow<'static, str> {
        "RenderGeometryPayload".into()
    }

    fn json_schema(_schema_gen: &mut SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "array",
            "description": "Serialized geo_types::Geometry<f64> payload. The concrete JSON follows geo-types serde representation for Geometry variants."
        })
    }
}

/// Geometry payload used by property-bearing feature records.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RenderFeatureGeometry(pub Geometry<f64>);

impl JsonSchema for RenderFeatureGeometry {
    fn schema_name() -> Cow<'static, str> {
        "RenderFeatureGeometry".into()
    }

    fn json_schema(_schema_gen: &mut SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "object",
            "description": "Serialized geo_types::Geometry<f64> payload for a feature record."
        })
    }
}

/// Property-bearing vector feature payload carried directly in the render IR.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderFeatureRecord {
    /// Optional stable feature identifier.
    pub id: Option<String>,
    /// Feature geometry.
    pub geometry: RenderFeatureGeometry,
    /// Arbitrary feature properties used by style predicates and expressions.
    pub properties: serde_json::Map<String, Value>,
}

/// Property-bearing vector payload carried directly in the render IR.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct RenderFeatureRecordPayload(pub Vec<RenderFeatureRecord>);

/// GeoJSON feature-collection payload carried directly in the render IR.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RenderFeatureCollectionPayload(pub FeatureCollection);

impl JsonSchema for RenderFeatureCollectionPayload {
    fn schema_name() -> Cow<'static, str> {
        "RenderFeatureCollectionPayload".into()
    }

    fn json_schema(_schema_gen: &mut SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "object",
            "description": "Serialized geojson::FeatureCollection payload following RFC 7946 / geojson serde representation.",
            "required": ["features"]
        })
    }
}

/// Source vector payload for a render layer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderVectorLayerPayload {
    /// Payload emitted from direct `geo-types` geometry input.
    Geometries(RenderGeometryPayload),
    /// Payload emitted from property-bearing feature records.
    FeatureRecords(RenderFeatureRecordPayload),
    /// Payload emitted from a GeoJSON feature collection.
    FeatureCollection(RenderFeatureCollectionPayload),
}

/// Coarse batching strategy for vector features in a render layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderFeatureBatchingStrategy {
    /// Let the backend choose the most suitable batching strategy.
    Automatic,
    /// Batch features primarily by geometry family.
    ByGeometryFamily,
    /// Batch features by their fully resolved symbolizer family.
    BySymbolizer,
    /// Batch features by explicit property-grouping keys.
    ByPropertyKeys,
}

/// Optional batching and instancing hints for vector-feature payloads.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderFeatureBatchingDescriptor {
    /// High-level batching strategy the backend should prefer.
    pub strategy: RenderFeatureBatchingStrategy,
    /// Whether repeated features may be emitted through GPU instancing.
    pub allow_gpu_instancing: bool,
    /// Whether instance identities should remain stable across scene updates.
    pub stable_instance_ids: bool,
    /// Optional upper bound on features per backend batch.
    pub max_features_per_batch: Option<u32>,
    /// Optional property names used to derive stable batch-grouping keys.
    pub group_key_properties: Vec<String>,
}

/// Full vector layer descriptor containing payload and styling.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderVectorLayerDescriptor {
    /// Stable layer identifier.
    pub id: String,
    /// User-facing layer name.
    pub name: String,
    /// Deterministic draw order.
    pub draw_order: i32,
    /// Default visibility.
    pub visible: bool,
    /// Overall layer opacity.
    pub opacity: f32,
    /// Carried georust vector payload.
    pub payload: RenderVectorLayerPayload,
    /// Optional batching and instancing hints for the backend.
    pub batching: Option<RenderFeatureBatchingDescriptor>,
    /// Fully resolved vector style set.
    pub style: RenderFeatureStyleDescriptor,
}

/// Full raster layer descriptor containing source and styling.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderRasterLayerDescriptor {
    /// Stable layer identifier.
    pub id: String,
    /// User-facing layer name.
    pub name: String,
    /// Deterministic draw order.
    pub draw_order: i32,
    /// Default visibility.
    pub visible: bool,
    /// Overall layer opacity.
    pub opacity: f32,
    /// Externally resolvable raster source.
    pub source: RenderRasterSourceDescriptor,
    /// Raster header metadata.
    pub image: RenderRasterImageMetadataDescriptor,
    /// Fully resolved raster style.
    pub style: RenderRasterStyleDescriptor,
}

/// Full tile layer descriptor containing source and styling.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderTileLayerDescriptor {
    /// Stable layer identifier.
    pub id: String,
    /// User-facing layer name.
    pub name: String,
    /// Deterministic draw order.
    pub draw_order: i32,
    /// Default visibility.
    pub visible: bool,
    /// Overall layer opacity.
    pub opacity: f32,
    /// Externally resolvable tile source.
    pub source: RenderTileSourceDescriptor,
    /// Fully resolved tile style.
    pub style: RenderTileStyleDescriptor,
}

/// Full terrain layer descriptor containing source and styling.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderTerrainLayerDescriptor {
    /// Stable layer identifier.
    pub id: String,
    /// User-facing layer name.
    pub name: String,
    /// Deterministic draw order.
    pub draw_order: i32,
    /// Default visibility.
    pub visible: bool,
    /// Overall layer opacity.
    pub opacity: f32,
    /// Externally resolvable elevation source.
    pub source: RenderRasterSourceDescriptor,
    /// Raster header metadata.
    pub image: RenderRasterImageMetadataDescriptor,
    /// Fully resolved terrain style.
    pub style: RenderTerrainStyleDescriptor,
}

/// Full annotation layer descriptor containing anchored symbolizers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderAnnotationLayerDescriptor {
    /// Stable layer identifier.
    pub id: String,
    /// User-facing layer name.
    pub name: String,
    /// Deterministic draw order.
    pub draw_order: i32,
    /// Default visibility.
    pub visible: bool,
    /// Overall layer opacity.
    pub opacity: f32,
    /// Explicit annotations rendered by the backend.
    pub annotations: Vec<RenderAnnotationDescriptor>,
}

/// Discriminant describing which payload class produced a render layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderLayerKind {
    /// Vector payload rendered from georust geometries or GeoJSON features.
    Vector,
    /// Raster payload rendered from a raster source and metadata.
    Raster,
    /// Raster or vector map tiles rendered from a tileset source.
    Tile,
    /// Terrain mesh rendered from elevation raster data.
    Terrain,
    /// Explicit annotations rendered independently from feature layers.
    Annotation,
}

/// Full validated render layer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderLayerDescriptor {
    /// Vector layer.
    Vector(RenderVectorLayerDescriptor),
    /// Raster layer.
    Raster(RenderRasterLayerDescriptor),
    /// Tile layer.
    Tile(RenderTileLayerDescriptor),
    /// Terrain layer.
    Terrain(RenderTerrainLayerDescriptor),
    /// Annotation layer.
    Annotation(RenderAnnotationLayerDescriptor),
}

/// Full validated render scene.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RenderSceneDescriptor {
    /// Stable scene identifier.
    pub scene_id: String,
    /// The view used to interpret the scene.
    pub view: RenderViewDescriptor,
    /// Optional scene environment and lighting policy.
    pub environment: Option<RenderSceneEnvironmentDescriptor>,
    /// Fully described render layers in draw order.
    pub layers: Vec<RenderLayerDescriptor>,
    /// Number of vector layers.
    pub vector_layer_count: usize,
    /// Number of raster layers.
    pub raster_layer_count: usize,
    /// Number of tile layers.
    pub tile_layer_count: usize,
    /// Number of terrain layers.
    pub terrain_layer_count: usize,
    /// Number of annotation layers.
    pub annotation_layer_count: usize,
    /// Whether any layer in the scene includes labels.
    pub has_feature_labels: bool,
}

/// Orthogonal kind for incremental scene updates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum RenderSceneUpdateKind {
    /// Replace the active scene environment.
    ReplaceEnvironment,
    /// Clear the active scene environment.
    ClearEnvironment,
    /// Replace the active scene view.
    ReplaceView,
    /// Insert a new layer into the scene.
    InsertLayer,
    /// Replace an existing layer in the scene.
    ReplaceLayer,
    /// Remove a layer from the scene.
    RemoveLayer,
    /// Reorder a layer relative to another layer.
    MoveLayerBefore,
    /// Update one layer's visibility.
    SetLayerVisibility,
    /// Update one layer's opacity.
    SetLayerOpacity,
    /// Update one layer's draw order.
    SetLayerDrawOrder,
}

/// Incremental scene mutation for backends that support updates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum RenderSceneUpdateDescriptor {
    /// Replace the active scene environment.
    ReplaceEnvironment {
        /// Target scene identifier.
        scene_id: String,
        /// Replacement environment descriptor.
        environment: RenderSceneEnvironmentDescriptor,
    },
    /// Clear the active scene environment.
    ClearEnvironment {
        /// Target scene identifier.
        scene_id: String,
    },
    /// Replace the active scene view.
    ReplaceView {
        /// Target scene identifier.
        scene_id: String,
        /// Replacement view descriptor.
        view: RenderViewDescriptor,
    },
    /// Insert a new validated layer, optionally before another layer.
    InsertLayer {
        /// Target scene identifier.
        scene_id: String,
        /// New layer descriptor.
        layer: RenderLayerDescriptor,
        /// Optional layer identifier before which to insert.
        before_layer_id: Option<String>,
    },
    /// Replace an existing layer with a new validated layer.
    ReplaceLayer {
        /// Target scene identifier.
        scene_id: String,
        /// Layer to replace.
        target_layer_id: String,
        /// Replacement layer descriptor.
        layer: RenderLayerDescriptor,
    },
    /// Remove a layer from the scene.
    RemoveLayer {
        /// Target scene identifier.
        scene_id: String,
        /// Layer to remove.
        target_layer_id: String,
    },
    /// Reorder a layer relative to another layer.
    MoveLayerBefore {
        /// Target scene identifier.
        scene_id: String,
        /// Layer to move.
        target_layer_id: String,
        /// Optional layer identifier before which to insert.
        before_layer_id: Option<String>,
    },
    /// Update a layer's visibility.
    SetLayerVisibility {
        /// Target scene identifier.
        scene_id: String,
        /// Layer to update.
        target_layer_id: String,
        /// Replacement visibility flag.
        visible: bool,
    },
    /// Update a layer's opacity.
    SetLayerOpacity {
        /// Target scene identifier.
        scene_id: String,
        /// Layer to update.
        target_layer_id: String,
        /// Replacement opacity.
        opacity: f32,
    },
    /// Update a layer's draw order.
    SetLayerDrawOrder {
        /// Target scene identifier.
        scene_id: String,
        /// Layer to update.
        target_layer_id: String,
        /// Replacement draw order.
        draw_order: i32,
    },
}
