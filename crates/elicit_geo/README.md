# elicit_geo

Elicitation-enabled geo algorithms MCP tools — predicates, measurements, transformations.

Provides **72 MCP tools** across **8 plugins** operating on `geo-types` primitives via the
[elicitation](https://crates.io/crates/elicitation) framework.

## Plugins

| Plugin | Namespace | Tools | Algorithms |
|--------|-----------|-------|------------|
| `GeoPredicatesPlugin` | `geo_predicates` | 12 | Contains, Intersects, Within, Covers |
| `GeoMeasurementsPlugin` | `geo_measurements` | 10 | Area, EuclideanDistance, EuclideanLength, HausdorffDistance |
| `GeoGeodesicPlugin` | `geo_geodesic` | 6 | HaversineDistance, GeodesicDistance, bearings, destinations |
| `GeoCalculationsPlugin` | `geo_calculations` | 10 | Centroid, BoundingRect, ConvexHull |
| `GeoTransformationsPlugin` | `geo_transformations` | 10 | Rotate, Scale, Translate, Simplify, RemoveRepeatedPoints |
| `GeoValidationPlugin` | `geo_validation` | 6 | IsClosed, IsConvex, CoordsCount, TypeName |
| `GeoBooleanOpsPlugin` | `geo_boolean_ops` | 8 | Union, Intersection, Difference, XOR (Polygon + MultiPolygon) |
| `GeoWorkflowPlugin` | `geo_workflow` | 10 | PointInPolygon, ClosestPoint, Interpolate, GeodesicLength, FréchetDistance |

## Usage

Register the plugins with an MCP server:

```rust
use elicit_geo::{
    GeoBooleanOpsPlugin, GeoCalculationsPlugin, GeoGeodesicPlugin, GeoMeasurementsPlugin,
    GeoPredicatesPlugin, GeoTransformationsPlugin, GeoValidationPlugin, GeoWorkflowPlugin,
};
```

All tools accept and return [elicit_geo_types](https://crates.io/crates/elicit_geo_types)
wrappers (Arc-wrapped `geo-types` primitives with `serde` / `schemars` support).

## Tool Reference

### `geo_predicates` (12 tools)

| Tool | Input | Output |
|------|-------|--------|
| `rect_contains_point` | `rect`, `point` | `"true"` / `"false"` |
| `polygon_contains_point` | `polygon`, `point` | `"true"` / `"false"` |
| `polygon_contains_linestring` | `polygon`, `linestring` | `"true"` / `"false"` |
| `polygon_contains_polygon` | `container`, `geometry` | `"true"` / `"false"` |
| `rect_intersects_rect` | `rect1`, `rect2` | `"true"` / `"false"` |
| `polygon_intersects_linestring` | `polygon`, `linestring` | `"true"` / `"false"` |
| `polygon_intersects_polygon` | `polygon1`, `polygon2` | `"true"` / `"false"` |
| `linestring_intersects_linestring` | `linestring1`, `linestring2` | `"true"` / `"false"` |
| `point_within_polygon` | `point`, `polygon` | `"true"` / `"false"` |
| `point_within_rect` | `point`, `rect` | `"true"` / `"false"` |
| `polygon_covers_point` | `polygon`, `point` | `"true"` / `"false"` |
| `polygon_covers_linestring` | `polygon`, `linestring` | `"true"` / `"false"` |

### `geo_measurements` (10 tools)

| Tool | Input | Output |
|------|-------|--------|
| `polygon_area` | `polygon` | f64 string |
| `polygon_signed_area` | `polygon` | f64 string |
| `rect_area` | `rect` | f64 string |
| `multipolygon_area` | `multipolygon` | f64 string |
| `line_euclidean_length` | `line` | f64 string |
| `linestring_euclidean_length` | `linestring` | f64 string |
| `euclidean_distance_point_point` | `from`, `to` | f64 string |
| `euclidean_distance_point_linestring` | `point`, `linestring` | f64 string |
| `euclidean_distance_point_polygon` | `point`, `polygon` | f64 string |
| `hausdorff_distance_linestrings` | `linestring1`, `linestring2` | f64 string |

### `geo_geodesic` (6 tools)

| Tool | Input | Output |
|------|-------|--------|
| `haversine_distance_points` | `from`, `to` | f64 string (meters) |
| `geodesic_distance_points` | `from`, `to` | f64 string (meters) |
| `haversine_bearing` | `from`, `to` | f64 string (degrees) |
| `geodesic_bearing` | `from`, `to` | f64 string (degrees) |
| `haversine_destination` | `origin`, `bearing_degrees`, `distance_meters` | JSON Point |
| `geodesic_destination` | `origin`, `bearing_degrees`, `distance_meters` | JSON Point |

### `geo_calculations` (10 tools)

| Tool | Input | Output |
|------|-------|--------|
| `polygon_centroid` | `polygon` | JSON Point or `"null"` |
| `linestring_centroid` | `linestring` | JSON Point or `"null"` |
| `multipoint_centroid` | `multipoint` | JSON Point or `"null"` |
| `multipolygon_centroid` | `multipolygon` | JSON Point or `"null"` |
| `polygon_bounding_rect` | `polygon` | JSON Rect or `"null"` |
| `linestring_bounding_rect` | `linestring` | JSON Rect or `"null"` |
| `multipoint_bounding_rect` | `multipoint` | JSON Rect or `"null"` |
| `polygon_convex_hull` | `polygon` | JSON Polygon |
| `multipoint_convex_hull` | `multipoint` | JSON Polygon |
| `linestring_convex_hull` | `linestring` | JSON Polygon |

### `geo_transformations` (10 tools)

| Tool | Input | Output |
|------|-------|--------|
| `translate_point` | `point`, `x_offset`, `y_offset` | JSON Point |
| `translate_linestring` | `linestring`, `x_offset`, `y_offset` | JSON LineString |
| `translate_polygon` | `polygon`, `x_offset`, `y_offset` | JSON Polygon |
| `rotate_polygon_around_centroid` | `polygon`, `degrees` | JSON Polygon |
| `rotate_linestring_around_centroid` | `linestring`, `degrees` | JSON LineString |
| `scale_polygon` | `polygon`, `scale_x`, `scale_y` | JSON Polygon |
| `simplify_linestring` | `linestring`, `epsilon` | JSON LineString |
| `simplify_polygon` | `polygon`, `epsilon` | JSON Polygon |
| `simplify_linestring_vw` | `linestring`, `epsilon` | JSON LineString |
| `remove_repeated_points_linestring` | `linestring` | JSON LineString |

### `geo_validation` (6 tools)

| Tool | Input | Output |
|------|-------|--------|
| `linestring_is_closed` | `linestring` | `"true"` / `"false"` |
| `linestring_is_convex` | `linestring` | `"true"` / `"false"` |
| `linestring_coords_count` | `linestring` | usize string |
| `polygon_coords_count` | `polygon` | usize string |
| `geometry_type_name` | `geometry` | type name string |
| `polygon_exterior_coords_count` | `polygon` | usize string |

### `geo_boolean_ops` (8 tools)

| Tool | Input | Output |
|------|-------|--------|
| `polygon_union` | `polygon1`, `polygon2` | JSON MultiPolygon |
| `polygon_intersection` | `polygon1`, `polygon2` | JSON MultiPolygon |
| `polygon_difference` | `polygon1`, `polygon2` | JSON MultiPolygon |
| `polygon_xor` | `polygon1`, `polygon2` | JSON MultiPolygon |
| `multipolygon_union` | `mp1`, `mp2` | JSON MultiPolygon |
| `multipolygon_intersection` | `mp1`, `mp2` | JSON MultiPolygon |
| `multipolygon_difference` | `mp1`, `mp2` | JSON MultiPolygon |
| `multipolygon_xor` | `mp1`, `mp2` | JSON MultiPolygon |

### `geo_workflow` (10 tools)

| Tool | Input | Output |
|------|-------|--------|
| `point_in_polygon` | `point`, `polygon` | `"true"` / `"false"` |
| `nearest_point_on_linestring` | `point`, `linestring` | JSON Point |
| `line_interpolate_point` | `linestring`, `fraction` | JSON Point or `"null"` |
| `line_locate_point` | `linestring`, `point` | f64 string or `"null"` |
| `polygon_exterior_length` | `polygon` | f64 string |
| `geodesic_length_linestring` | `linestring` | f64 string (meters) |
| `haversine_length_linestring` | `linestring` | f64 string (meters) |
| `frechet_distance_linestrings` | `linestring1`, `linestring2` | f64 string |
| `remove_repeated_points_polygon` | `polygon` | JSON Polygon |
| `polygon_interior_point` | `polygon` | JSON Point or `"null"` |

## License

Licensed under either of [Apache License, Version 2.0](../LICENSE-APACHE) or
[MIT license](../LICENSE-MIT) at your option.
