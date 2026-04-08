# elicit_geo_types

Elicitation-enabled geo-types shadow crate providing MCP tools for geometric primitives.

## Overview

`elicit_geo_types` wraps every type from the [geo-types](https://crates.io/crates/geo-types)
spatial library in elicitation-enabled newtypes with full MCP tool support.

## Plugins

| Plugin | Namespace | Tools |
|--------|-----------|-------|
| `GeoTypesPrimitivesPlugin` | `geo_types_primitives` | 8 |
| `GeoTypesShapesPlugin` | `geo_types_shapes` | 8 |
| `GeoTypesCollectionsPlugin` | `geo_types_collections` | 10 |
| `GeoTypesGeometryPlugin` | `geo_types_geometry` | 6 |

### Primitives (8 tools)

| Tool | Description |
|------|-------------|
| `create_coord` | Create a `Coord` from x, y |
| `create_point` | Create a `Point` from x, y |
| `create_line` | Create a `Line` from two endpoints |
| `create_triangle` | Create a `Triangle` from three vertices |
| `coord_x` | Get x component of a `Coord` |
| `coord_y` | Get y component of a `Coord` |
| `point_x` | Get x component of a `Point` |
| `point_y` | Get y component of a `Point` |

### Shapes (8 tools)

| Tool | Description |
|------|-------------|
| `create_rect` | Create a `Rect` from min/max corners |
| `rect_width` | Get width of a `Rect` |
| `rect_height` | Get height of a `Rect` |
| `rect_center` | Get center `Coord` of a `Rect` |
| `rect_min` | Get minimum corner of a `Rect` |
| `rect_max` | Get maximum corner of a `Rect` |
| `create_polygon` | Create a `Polygon` from exterior + holes |
| `polygon_interiors_count` | Count interior rings of a `Polygon` |

### Collections (10 tools)

| Tool | Description |
|------|-------------|
| `create_line_string` | Create a `LineString` from coords |
| `line_string_coords_count` | Count coords in a `LineString` |
| `line_string_is_closed` | Test if a `LineString` is closed |
| `create_multi_point` | Create a `MultiPoint` from points |
| `multi_point_count` | Count points in a `MultiPoint` |
| `create_multi_line_string` | Create a `MultiLineString` |
| `multi_line_string_count` | Count lines in a `MultiLineString` |
| `create_multi_polygon` | Create a `MultiPolygon` from polygons |
| `multi_polygon_count` | Count polygons in a `MultiPolygon` |
| `geometry_collection_count` | Count items in a `GeometryCollection` |

### Geometry (6 tools)

| Tool | Description |
|------|-------------|
| `create_point_geometry` | Create `Geometry::Point` from x, y |
| `create_rect_geometry` | Create `Geometry::Rect` from corners |
| `geometry_type` | Get variant name of a `Geometry` |
| `is_point` | Test if `Geometry` is a `Point` |
| `is_collection` | Test if `Geometry` is a collection |
| `create_empty_collection` | Create empty `GeometryCollection` geometry |

## Quick Example

```rust
use elicit_geo_types::{Coord, Point};

// Create a point
let p = Point::new(-122.4194, 37.7749); // San Francisco

// Inspect via methods
println!("lng: {}, lat: {}", p.lng(), p.lat());

// Serialise (transparent Serde)
let json = serde_json::to_string(&p).unwrap();
let p2: Point = serde_json::from_str(&json).unwrap();
assert_eq!(p.x(), p2.x());
```

## License

Apache-2.0 OR MIT
