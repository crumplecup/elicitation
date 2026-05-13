//! RFC 7946 GeoJSON construction, inspection, and document traits.
//!
//! Three-role taxonomy applied to RFC 7946 GeoJSON:
//!
//! ## Role 1 — Factory / builder traits
//!
//! Each factory accepts raw [`GeoJsonPosition`] values (and, for composite
//! geometry types, `Established<P>` proof tokens from already-validated
//! sub-objects) and returns either an error or a descriptor together with an
//! `Established<P>` postcondition token.  The method signatures encode the
//! RFC 7946 validity dependency graph — positions must be validated before
//! they can be assembled into geometries; geometries must be validated before
//! they can be placed in Features.
//!
//! | Trait | Builds | Postcondition |
//! |---|---|---|
//! | [`GeoJsonGeometryFactory`] | Point | [`GeoJsonPointValid`] |
//! | [`GeoJsonGeometryFactory`] | MultiPoint | [`GeoJsonMultiPointValid`] |
//! | [`GeoJsonGeometryFactory`] | LineString | [`GeoJsonLineStringValid`] |
//! | [`GeoJsonGeometryFactory`] | MultiLineString | [`GeoJsonMultiLineStringValid`] |
//! | [`GeoJsonGeometryFactory`] | Polygon | [`GeoJsonPolygonValid`] |
//! | [`GeoJsonGeometryFactory`] | MultiPolygon | [`GeoJsonMultiPolygonValid`] |
//! | [`GeoJsonGeometryFactory`] | GeometryCollection | [`GeoJsonGeometryCollectionValid`] |
//! | [`GeoJsonGeometryFactory`] | any geometry (parse) | [`GeoJsonGeometryValid`] |
//! | [`GeoJsonFeatureFactory`] | Feature | [`FeatureValid`] |
//! | [`GeoJsonFeatureFactory`] | FeatureCollection | [`FeatureCollectionValid`] |
//! | [`GeoJsonFeatureFactory`] | any GeoJSON text (parse) | [`GeoJsonDocumentValid`] |
//!
//! ## Role 2 — Orthogonal concern traits
//!
//! These traits operate independently of structural validity — they report
//! facts that are always accessible regardless of whether a full validation
//! chain has been run.
//!
//! | Trait | Reports |
//! |---|---|
//! | [`GeoJsonObjectMeta`] | "type" field, bbox presence and values, foreign member keys |
//! | [`GeoJsonFeatureMeta`] | feature id, geometry/properties null flags, feature count |
//!
//! ## Role 3 — Abstraction supertrait
//!
//! [`GeoJsonGeometryFactory`], [`GeoJsonFeatureFactory`],
//! [`GeoJsonObjectMeta`], and [`GeoJsonFeatureMeta`] are composed into
//! [`GeoJsonBackend`], which presents RFC 7946 GeoJSON construction and
//! inspection as a single coherent interface.
//!
//! `GeoJsonBackend` is also wired into [`crate::GisBackend`], since GeoJSON
//! is the primary interchange format of the web GIS ecosystem.
//!
//! Source: RFC 7946 — The GeoJSON Format.
//!
//! [`GeoJsonPosition`]: crate::GeoJsonPosition
//! [`GeoJsonPointValid`]: crate::GeoJsonPointValid
//! [`GeoJsonMultiPointValid`]: crate::GeoJsonMultiPointValid
//! [`GeoJsonLineStringValid`]: crate::GeoJsonLineStringValid
//! [`GeoJsonMultiLineStringValid`]: crate::GeoJsonMultiLineStringValid
//! [`GeoJsonPolygonValid`]: crate::GeoJsonPolygonValid
//! [`GeoJsonMultiPolygonValid`]: crate::GeoJsonMultiPolygonValid
//! [`GeoJsonGeometryCollectionValid`]: crate::GeoJsonGeometryCollectionValid
//! [`GeoJsonGeometryValid`]: crate::GeoJsonGeometryValid
//! [`FeatureValid`]: crate::FeatureValid
//! [`FeatureCollectionValid`]: crate::FeatureCollectionValid
//! [`GeoJsonDocumentValid`]: crate::GeoJsonDocumentValid

use elicitation::Established;
use serde_json::Value as JsonValue;

use crate::{
    FeatureCollectionValid, FeatureValid, GeoJsonDocumentDescriptor, GeoJsonDocumentValid,
    GeoJsonFeatureCollectionDescriptor, GeoJsonFeatureDescriptor, GeoJsonFeatureId,
    GeoJsonGeometryCollectionValid, GeoJsonGeometryDescriptor, GeoJsonGeometryValid,
    GeoJsonLineStringValid, GeoJsonMultiLineStringValid, GeoJsonMultiPointValid,
    GeoJsonMultiPolygonValid, GeoJsonPointValid, GeoJsonPolygonValid, GeoJsonPosition, GisResult,
    PositionValid,
};

// ── Role 1: Factory / builder traits ─────────────────────────────────────────

/// Build and validate RFC 7946 GeoJSON geometry objects.
///
/// Positions are the atomic unit: call [`validate_position`] first, then
/// compose validated positions (and validated sub-geometries) into the
/// geometry type you need.  The method signatures enforce the RFC 7946
/// validation dependency graph in the type system:
///
/// ```text
/// GeoJsonPosition
///   → validate_position → Established<PositionValid>
///   → build_geojson_point / build_geojson_line_string / …
///   → Established<GeoJsonPointValid> / Established<GeoJsonLineStringValid> / …
///   → build_geojson_multi_… / build_geojson_geometry_collection
///   → Established<GeoJsonGeometryValid>
/// ```
///
/// # Leaf validators
///
/// [`validate_position`] stands at the base of the dependency graph.  It
/// validates `PositionHasAtLeastTwoElements`, `PositionElementsAreJsonNumbers`,
/// `PositionLongitudeIsFinite`, `PositionLatitudeIsFinite`,
/// `PositionLongitudeInRange`, `PositionLatitudeInRange`, and
/// (when altitude is present) `PositionAltitudeIsFinite`.
///
/// # Leaf geometry builders
///
/// [`build_geojson_point`], [`build_geojson_multi_point`], and
/// [`build_geojson_line_string`] accept pre-validated positions.
///
/// # Composite geometry builders
///
/// [`build_geojson_multi_line_string`] requires `Established<GeoJsonLineStringValid>`
/// tokens; [`build_geojson_polygon`] requires validated positions for each
/// ring; [`build_geojson_multi_polygon`] requires `Established<GeoJsonPolygonValid>`
/// tokens; [`build_geojson_geometry_collection`] requires
/// `Established<GeoJsonGeometryValid>` tokens.
///
/// # Parse factory
///
/// [`geometry_from_geojson_str`] constructs a geometry from raw JSON text and
/// emits `Established<GeoJsonGeometryValid>` on success.
///
/// # Object safety
///
/// All method signatures use only concrete types.  The trait is
/// `dyn`-compatible.
///
/// Source: RFC 7946 §3.1 — Geometry Objects.
///
/// [`validate_position`]: GeoJsonGeometryFactory::validate_position
/// [`build_geojson_point`]: GeoJsonGeometryFactory::build_geojson_point
/// [`build_geojson_multi_point`]: GeoJsonGeometryFactory::build_geojson_multi_point
/// [`build_geojson_line_string`]: GeoJsonGeometryFactory::build_geojson_line_string
/// [`build_geojson_multi_line_string`]: GeoJsonGeometryFactory::build_geojson_multi_line_string
/// [`build_geojson_polygon`]: GeoJsonGeometryFactory::build_geojson_polygon
/// [`build_geojson_multi_polygon`]: GeoJsonGeometryFactory::build_geojson_multi_polygon
/// [`build_geojson_geometry_collection`]: GeoJsonGeometryFactory::build_geojson_geometry_collection
/// [`geometry_from_geojson_str`]: GeoJsonGeometryFactory::geometry_from_geojson_str
pub trait GeoJsonGeometryFactory: Send + Sync {
    // ── Leaf validator ────────────────────────────────────────────────────

    /// Validate a single GeoJSON position.
    ///
    /// Validates: `PositionIsJsonArray`, `PositionHasAtLeastTwoElements`,
    /// `PositionElementsAreJsonNumbers`, `PositionElementZeroIsLongitude`,
    /// `PositionElementOneIsLatitude`, `PositionLongitudeIsFinite`,
    /// `PositionLatitudeIsFinite`, `PositionLongitudeInRange`,
    /// `PositionLatitudeInRange`, `PositionAltitudeIsFinite` (when altitude
    /// is present) → `PositionValid`.
    ///
    /// Source: RFC 7946 §3.1.1 — Position.
    fn validate_position(&self, position: GeoJsonPosition)
    -> GisResult<Established<PositionValid>>;

    // ── Leaf geometry builders ────────────────────────────────────────────

    /// Build a GeoJSON Point from a validated position.
    ///
    /// Precondition: `Established<PositionValid>` from [`validate_position`].
    ///
    /// Validates: `PointTypeEqualsPoint`, `PointHasCoordinatesMember`,
    /// `PointCoordinatesIsSinglePosition`, `PointCoordinatesIsNotNull`,
    /// `PointCoordinatesHasMinTwoElements` → `GeoJsonPointValid`.
    ///
    /// Source: RFC 7946 §3.1.2 — Point.
    ///
    /// [`validate_position`]: GeoJsonGeometryFactory::validate_position
    fn build_geojson_point(
        &self,
        position: Established<PositionValid>,
    ) -> GisResult<(GeoJsonGeometryDescriptor, Established<GeoJsonPointValid>)>;

    /// Build a GeoJSON MultiPoint from zero or more validated positions.
    ///
    /// Preconditions: `Established<PositionValid>` for each position.
    ///
    /// Validates: `MultiPointTypeEqualsMultiPoint`,
    /// `MultiPointHasCoordinatesMember`,
    /// `MultiPointCoordinatesIsArrayOfPositions`,
    /// `MultiPointEachElementIsValidPosition`,
    /// `MultiPointCoordinatesMayBeEmpty`,
    /// `MultiPointCoordinatesIsNotNull` → `GeoJsonMultiPointValid`.
    ///
    /// Source: RFC 7946 §3.1.3 — MultiPoint.
    fn build_geojson_multi_point(
        &self,
        positions: Vec<Established<PositionValid>>,
    ) -> GisResult<(
        GeoJsonGeometryDescriptor,
        Established<GeoJsonMultiPointValid>,
    )>;

    /// Build a GeoJSON LineString from two or more validated positions.
    ///
    /// Preconditions: `Established<PositionValid>` for each position.
    ///
    /// Validates: `LineStringTypeEqualsLineString`,
    /// `LineStringHasCoordinatesMember`,
    /// `LineStringCoordinatesIsNotNull`,
    /// `LineStringCoordinatesHasMinTwoPositions`,
    /// `LineStringCoordinatesIsNotEmpty`,
    /// `LineStringEachElementIsValidPosition`,
    /// `LineStringMinTwoPositionsFormPath` → `GeoJsonLineStringValid`.
    ///
    /// Source: RFC 7946 §3.1.4 — LineString.
    fn build_geojson_line_string(
        &self,
        positions: Vec<Established<PositionValid>>,
    ) -> GisResult<(
        GeoJsonGeometryDescriptor,
        Established<GeoJsonLineStringValid>,
    )>;

    // ── Composite geometry builders ───────────────────────────────────────

    /// Build a GeoJSON MultiLineString from zero or more validated LineStrings.
    ///
    /// Preconditions: `Established<GeoJsonLineStringValid>` for each component
    /// LineString.
    ///
    /// Validates: `MultiLineStringTypeEqualsMultiLineString`,
    /// `MultiLineStringHasCoordinatesMember`,
    /// `MultiLineStringCoordinatesIsNotNull`,
    /// `MultiLineStringCoordinatesIsArray`,
    /// `MultiLineStringEachElementIsArray`,
    /// `MultiLineStringEachLineStringHasMinTwoPositions`,
    /// `MultiLineStringEachPositionIsValid`,
    /// `GeoJsonMultiLineStringMayBeEmpty` → `GeoJsonMultiLineStringValid`.
    ///
    /// Source: RFC 7946 §3.1.5 — MultiLineString.
    fn build_geojson_multi_line_string(
        &self,
        lines: Vec<Established<GeoJsonLineStringValid>>,
    ) -> GisResult<(
        GeoJsonGeometryDescriptor,
        Established<GeoJsonMultiLineStringValid>,
    )>;

    /// Build a GeoJSON Polygon from an exterior ring and zero or more hole rings.
    ///
    /// Each ring is a sequence of pre-validated positions.  The factory
    /// validates ring closure, minimum four positions, winding order, and
    /// hole topology.
    ///
    /// Preconditions: `Established<PositionValid>` for every position in
    /// `exterior` and in each element of `holes`.
    ///
    /// Validates: `PolygonTypeEqualsPolygon`, `PolygonHasCoordinatesMember`,
    /// `PolygonCoordinatesIsNotNull`, `PolygonCoordinatesHasAtLeastOneRing`,
    /// `PolygonLinearRingIsClosedLineString`,
    /// `PolygonLinearRingHasMinFourPositions`,
    /// `PolygonLinearRingFirstAndLastAreIdentical`,
    /// `PolygonRingDoesNotSelfIntersect`,
    /// `PolygonExteriorRingIsCounterclockwise`,
    /// `PolygonHoleRingsAreClockwise`,
    /// `PolygonHolesAreInteriorToExteriorRing`,
    /// `PolygonHolesDoNotOverlap`,
    /// `PolygonFirstRingIsExteriorBoundary`,
    /// `PolygonSubsequentRingsAreInteriorHoles` → `GeoJsonPolygonValid`.
    ///
    /// Source: RFC 7946 §3.1.6 — Polygon.
    fn build_geojson_polygon(
        &self,
        exterior: Vec<Established<PositionValid>>,
        holes: Vec<Vec<Established<PositionValid>>>,
    ) -> GisResult<(GeoJsonGeometryDescriptor, Established<GeoJsonPolygonValid>)>;

    /// Build a GeoJSON MultiPolygon from zero or more validated Polygons.
    ///
    /// Preconditions: `Established<GeoJsonPolygonValid>` for each component
    /// Polygon.
    ///
    /// Validates: `MultiPolygonTypeEqualsMultiPolygon`,
    /// `MultiPolygonHasCoordinatesMember`,
    /// `MultiPolygonCoordinatesIsArray`,
    /// `MultiPolygonCoordinatesIsNotNull`,
    /// `GeoJsonMultiPolygonMayBeEmpty`,
    /// `MultiPolygonEachElementIsPolygonCoordinates`,
    /// `MultiPolygonEachPolygonObeysPolygonRules` → `GeoJsonMultiPolygonValid`.
    ///
    /// Source: RFC 7946 §3.1.7 — MultiPolygon.
    fn build_geojson_multi_polygon(
        &self,
        polygons: Vec<Established<GeoJsonPolygonValid>>,
    ) -> GisResult<(
        GeoJsonGeometryDescriptor,
        Established<GeoJsonMultiPolygonValid>,
    )>;

    /// Build a GeoJSON GeometryCollection from zero or more validated
    /// sub-geometries of any type.
    ///
    /// Preconditions: `Established<GeoJsonGeometryValid>` for each component.
    ///
    /// Validates: `GeometryCollectionTypeEqualsGeometryCollection`,
    /// `GeometryCollectionHasGeometriesMember`,
    /// `GeometryCollectionMustNotHaveCoordinatesMember`,
    /// `GeometryCollectionGeometriesIsJsonArray`,
    /// `GeometryCollectionGeometriesIsNotNull`,
    /// `GeometryCollectionEachElementIsValidGeometry`,
    /// `GeometryCollectionEachGeometryHasTypeMember`,
    /// `GeoJsonGeometryCollectionMayBeEmpty` → `GeoJsonGeometryCollectionValid`.
    ///
    /// Source: RFC 7946 §3.1.8 — GeometryCollection.
    fn build_geojson_geometry_collection(
        &self,
        geometries: Vec<Established<GeoJsonGeometryValid>>,
    ) -> GisResult<(
        GeoJsonGeometryDescriptor,
        Established<GeoJsonGeometryCollectionValid>,
    )>;

    // ── Parse factory ─────────────────────────────────────────────────────

    /// Parse a GeoJSON geometry object from raw JSON text and validate it.
    ///
    /// Accepts any of the seven RFC 7946 geometry types.  On success emits
    /// `Established<GeoJsonGeometryValid>` certifying that the parsed geometry
    /// satisfies all applicable RFC 7946 §3.1 constraints for its concrete
    /// type.
    ///
    /// Exercises: `GeoJsonTextIsSingleJsonValue`, `GeoJsonRootIsObject`,
    /// `GeoJsonObjectHasTypeMember`, `GeoJsonTypeMemberIsString`,
    /// `GeoJsonTypeIsOneOfNineValues`, `GeoJsonTypeIsCaseSensitive`,
    /// plus all per-type validators for the geometry kind found.
    ///
    /// Source: RFC 7946 §3.1 — Geometry Object.
    fn geometry_from_geojson_str(
        &self,
        json: &str,
    ) -> GisResult<(GeoJsonGeometryDescriptor, Established<GeoJsonGeometryValid>)>;
}

// ── Role 1: Feature and document factory ─────────────────────────────────────

/// Build and validate RFC 7946 GeoJSON Feature and FeatureCollection objects,
/// and parse complete GeoJSON documents.
///
/// # Feature construction
///
/// [`build_geojson_feature`] assembles a Feature from an optional id, an
/// optional (possibly null) geometry token, and optional JSON properties.
/// The geometry argument accepts `None` for the absent-geometry case and
/// `Some(None)` for the JSON-null (unlocated-feature) case.
///
/// # FeatureCollection construction
///
/// [`build_geojson_feature_collection`] assembles a FeatureCollection from
/// zero or more `Established<FeatureValid>` tokens.
///
/// # Document parse factory
///
/// [`document_from_geojson_str`] parses any RFC 7946 GeoJSON text (geometry,
/// feature, or feature collection) and emits `Established<GeoJsonDocumentValid>`.
///
/// # Object safety
///
/// All method signatures use only concrete types.  The trait is
/// `dyn`-compatible.
///
/// Source: RFC 7946 §3.2–§3.3 — Feature Object and FeatureCollection.
///
/// [`build_geojson_feature`]: GeoJsonFeatureFactory::build_geojson_feature
/// [`build_geojson_feature_collection`]: GeoJsonFeatureFactory::build_geojson_feature_collection
/// [`document_from_geojson_str`]: GeoJsonFeatureFactory::document_from_geojson_str
pub trait GeoJsonFeatureFactory: Send + Sync {
    /// Build a GeoJSON Feature from an id, geometry, and properties.
    ///
    /// - `id`: Optional feature identifier (string or number, never null).
    /// - `geometry`: `None` means the "geometry" member is JSON null (an
    ///   unlocated feature).  `Some(token)` means a validated geometry is
    ///   present.
    /// - `properties`: `None` means the "properties" member is JSON null.
    ///   `Some(value)` must be a JSON object.
    ///
    /// Validates: `FeatureTypeEqualsFeature`, `FeatureHasGeometryMember`,
    /// `FeatureHasPropertiesMember`,
    /// `FeatureGeometryIsGeometryObjectOrNull`,
    /// `FeatureGeometryWhenPresentIsValidGeometry`,
    /// `FeatureNullGeometryMeansUnlocated`,
    /// `FeaturePropertiesIsJsonObjectOrNull`,
    /// `FeatureIdWhenPresentIsStringOrNumber`,
    /// `FeatureIdMustNotBeNull`,
    /// `NullGeometryDistinctFromAbsentGeometry` → `FeatureValid`.
    ///
    /// Source: RFC 7946 §3.2 — Feature Object.
    fn build_geojson_feature(
        &self,
        id: Option<GeoJsonFeatureId>,
        geometry: Option<Established<GeoJsonGeometryValid>>,
        properties: Option<JsonValue>,
    ) -> GisResult<(GeoJsonFeatureDescriptor, Established<FeatureValid>)>;

    /// Build a GeoJSON FeatureCollection from zero or more validated Features.
    ///
    /// Preconditions: `Established<FeatureValid>` for each Feature.
    ///
    /// Validates: `FeatureCollectionTypeEqualsFeatureCollection`,
    /// `FeatureCollectionHasFeaturesMember`,
    /// `FeatureCollectionFeaturesIsJsonArray`,
    /// `FeatureCollectionFeaturesIsNotNull`,
    /// `FeatureCollectionEachElementIsFeatureObject`,
    /// `FeatureCollectionEachElementIsNotNull`,
    /// `FeatureCollectionMayBeEmpty`,
    /// `FeatureCollectionFeaturesIsNotSingleFeature` → `FeatureCollectionValid`.
    ///
    /// Source: RFC 7946 §3.3 — FeatureCollection Object.
    fn build_geojson_feature_collection(
        &self,
        features: Vec<Established<FeatureValid>>,
    ) -> GisResult<(
        GeoJsonFeatureCollectionDescriptor,
        Established<FeatureCollectionValid>,
    )>;

    /// Parse any GeoJSON text (geometry, feature, or feature collection) and
    /// validate it as a complete RFC 7946 document.
    ///
    /// On success emits `Established<GeoJsonDocumentValid>` certifying that
    /// the text satisfies `GeoJsonTextIsSingleJsonValue`, `GeoJsonRootIsObject`,
    /// `GeoJsonObjectHasTypeMember`, `GeoJsonTypeMemberIsString`,
    /// `GeoJsonTypeMemberIsNotNull`, `GeoJsonTypeIsOneOfNineValues`,
    /// `GeoJsonTypeIsCaseSensitive`, `GeoJsonTypeIsNotExtensible`, and all
    /// per-type constraints for the root object.
    ///
    /// Source: RFC 7946 §2 — GeoJSON Text.
    fn document_from_geojson_str(
        &self,
        json: &str,
    ) -> GisResult<(GeoJsonDocumentDescriptor, Established<GeoJsonDocumentValid>)>;
}

// ── Role 2: Orthogonal concern traits ─────────────────────────────────────────

/// Inspect the RFC 7946 "type" member, "bbox" member, and foreign members of
/// any GeoJSON object.
///
/// This trait is an **orthogonal concern**: all methods report facts about the
/// top-level JSON object and are callable regardless of structural validity.
/// A GeoJSON value whose geometry has not been validated (or is malformed)
/// still has a "type" string and may have a "bbox" array.
///
/// The trait is `dyn`-compatible: all methods use concrete return types.
///
/// Source: RFC 7946 §3 — GeoJSON Object; §5 — Bounding Box; §6.1 — Foreign
/// Members.
pub trait GeoJsonObjectMeta: Send + Sync {
    /// Return the value of the "type" member.
    ///
    /// Always a non-null string for any conformant GeoJSON object.
    ///
    /// Exercises: `GeoJsonObjectHasTypeMember`, `GeoJsonTypeMemberIsString`,
    /// `GeoJsonTypeMemberIsNotNull`, `GeoJsonTypeIsCaseSensitive`.
    ///
    /// Source: RFC 7946 §3 — GeoJSON Object.
    fn geojson_type(&self) -> &str;

    /// Return `true` if this object has a "bbox" member.
    ///
    /// Exercises: `GeoJsonBboxMemberIsOptional`, `BboxMemberIsOptional`.
    ///
    /// Source: RFC 7946 §3 — GeoJSON Object / §5 — Bounding Box.
    fn has_bbox(&self) -> bool;

    /// Return the bbox elements when the "bbox" member is present.
    ///
    /// Returns `None` when the member is absent, or the flat numeric array
    /// when it is present.  The returned slice has either 4 elements (2D) or
    /// 6 elements (3D) for well-formed objects.
    ///
    /// Exercises: `BboxWhenPresentIsJsonArray`, `BboxAllElementsAreJsonNumbers`,
    /// `BboxElementsAreFinite`, `Bbox2dHasExactlyFourElements`,
    /// `Bbox3dHasExactlySixElements`, `Bbox2dOrderIsMinLonMinLatMaxLonMaxLat`.
    ///
    /// Source: RFC 7946 §5 — Bounding Box.
    fn bbox_elements(&self) -> GisResult<Option<Vec<f64>>>;

    /// Return the keys of all foreign members present in this object.
    ///
    /// Returns an empty `Vec` when no foreign members are present.  Keys are
    /// not normalised or deduplicated — the raw JSON key strings are returned.
    ///
    /// Exercises: `ForeignMembersAtAnyLevelShouldBeIgnored`,
    /// `ForeignMembersDoNotAlterSemantics`,
    /// `ForeignMembersCannotOverrideTypeMember`.
    ///
    /// Source: RFC 7946 §6.1 — Foreign Members.
    fn foreign_member_keys(&self) -> GisResult<Vec<String>>;
}

/// Inspect RFC 7946 Feature and FeatureCollection metadata independently of
/// structural validity.
///
/// This trait is an **orthogonal concern**: the id, geometry null flag, and
/// properties null flag of a Feature are accessible even if the geometry has
/// not been validated.  A FeatureCollection's feature count is accessible even
/// if individual features have not been checked.
///
/// The trait is `dyn`-compatible.
///
/// Source: RFC 7946 §3.2 — Feature Object; §3.3 — FeatureCollection Object.
pub trait GeoJsonFeatureMeta: Send + Sync {
    /// Return the optional feature id, or `None` if absent.
    ///
    /// When present, the id is either a `GeoJsonFeatureId::String` or a
    /// `GeoJsonFeatureId::Number`; all other JSON types are invalid per
    /// RFC 7946 §3.2.
    ///
    /// Exercises: `FeatureIdMemberIsOptional`,
    /// `FeatureIdWhenPresentIsStringOrNumber`, `FeatureIdStringIsAllowed`,
    /// `FeatureIdNumberIsAllowed`, `FeatureIdMustNotBeNull`.
    ///
    /// Source: RFC 7946 §3.2 — Feature Object.
    fn feature_id(&self) -> GisResult<Option<GeoJsonFeatureId>>;

    /// Return `true` if the "geometry" member is JSON null.
    ///
    /// `false` means either the member is a valid geometry object (when the
    /// feature has a known location) or the member is absent (which is also
    /// valid — absence is distinct from null per `NullGeometryDistinctFromAbsentGeometry`).
    ///
    /// Exercises: `FeatureGeometryIsGeometryObjectOrNull`,
    /// `FeatureNullGeometryMeansUnlocated`,
    /// `NullGeometryDistinctFromAbsentGeometry`.
    ///
    /// Source: RFC 7946 §3.2 — Feature Object.
    fn geometry_is_null(&self) -> GisResult<bool>;

    /// Return `true` if the "properties" member is JSON null.
    ///
    /// `false` means the member is either a JSON object or is absent.
    ///
    /// Exercises: `FeaturePropertiesIsJsonObjectOrNull`.
    ///
    /// Source: RFC 7946 §3.2 — Feature Object.
    fn properties_is_null(&self) -> GisResult<bool>;

    /// Return the number of features if this is a FeatureCollection, or `None`
    /// if this is a Feature.
    ///
    /// `Some(0)` indicates a valid but empty FeatureCollection.
    ///
    /// Exercises: `FeatureCollectionMayBeEmpty`,
    /// `FeatureCollectionEachElementIsFeatureObject`.
    ///
    /// Source: RFC 7946 §3.3 — FeatureCollection Object.
    fn feature_count(&self) -> GisResult<Option<usize>>;
}

// ── Role 3: Abstraction supertrait ─────────────────────────────────────────

/// Complete RFC 7946 GeoJSON backend — blanket supertrait.
///
/// Any type that implements all four RFC 7946 sub-traits automatically
/// implements `GeoJsonBackend`.  Use the individual object-safe sub-traits
/// (`dyn GeoJsonGeometryFactory`, `dyn GeoJsonObjectMeta`, etc.) for dynamic
/// dispatch at architectural boundaries.
///
/// Source: RFC 7946 — The GeoJSON Format.
pub trait GeoJsonBackend:
    GeoJsonGeometryFactory
    + GeoJsonFeatureFactory
    + GeoJsonObjectMeta
    + GeoJsonFeatureMeta
    + Send
    + Sync
{
}

impl<T> GeoJsonBackend for T where
    T: GeoJsonGeometryFactory
        + GeoJsonFeatureFactory
        + GeoJsonObjectMeta
        + GeoJsonFeatureMeta
        + Send
        + Sync
{
}
