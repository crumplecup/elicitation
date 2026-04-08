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

## Type Hierarchy

```
Geometry (enum)
├── Point         ← wraps geo_types::Point<f64>
├── Line          ← wraps geo_types::Line<f64>
├── LineString    ← wraps geo_types::LineString<f64>
├── Polygon       ← wraps geo_types::Polygon<f64>
├── MultiPoint    ← wraps geo_types::MultiPoint<f64>
├── MultiLineString ← wraps geo_types::MultiLineString<f64>
├── MultiPolygon  ← wraps geo_types::MultiPolygon<f64>
├── Rect          ← wraps geo_types::Rect<f64>
├── Triangle      ← wraps geo_types::Triangle<f64>
└── GeometryCollection ← wraps geo_types::GeometryCollection<f64>

Leaf types (compose into all above)
├── Coord         ← wraps geo_types::Coord<f64>  (x, y pair)
└── f64           ← primitive via F64Default elicitation
```

## Usage Examples

### Construct and inspect

```rust
use elicit_geo_types::{Coord, Line, Point, Rect};

// Coordinate (leaf type)
let origin = Coord::new(0.0, 0.0);
let sf     = Coord::new(-122.4194, 37.7749);

// Point
let p = Point::new(-122.4194, 37.7749);
println!("lng={}, lat={}", p.lng(), p.lat());

// Line between two coords
let line = Line::new(origin.clone(), sf.clone());

// Rect from min/max corners
let bbox = Rect::new(
    Coord::new(-130.0, 30.0),
    Coord::new(-60.0, 50.0),
);
println!("w={}, h={}", bbox.width(), bbox.height());
```

### Use with geo algorithms

All newtypes implement `Deref` to their underlying `geo_types` value, so
they compose directly with the `geo` algorithm crate:

```rust
use elicit_geo_types::{LineString, Point};
use geo::Contains; // from the `geo` crate

let ls = LineString::new(vec![
    elicit_geo_types::Coord::new(0.0, 0.0),
    elicit_geo_types::Coord::new(1.0, 0.0),
    elicit_geo_types::Coord::new(1.0, 1.0),
    elicit_geo_types::Coord::new(0.0, 0.0),
]);

let p = Point::new(0.5, 0.1);
// Deref to geo_types::LineString and geo_types::Point
let ring: geo_types::LineString<f64> = ls.to_geo();
```

### Serialise and deserialise

All types derive `Serialize` / `Deserialize` with transparent layout:

```rust
use elicit_geo_types::Point;

let p = Point::new(-122.4194, 37.7749);
let json = serde_json::to_string(&p).unwrap();
let p2: Point = serde_json::from_str(&json).unwrap();
assert!((p.x() - p2.x()).abs() < f64::EPSILON);
```

## Integration with `elicit_ui`

`elicit_geo_types::Rect` and `elicit_ui::EguiRect` represent bounding
boxes in different coordinate systems (geographic vs. screen pixels).
They share the same elicitation composition chain (`f64`/`f32` leaf
types), making it straightforward to bridge them:

```rust
// Convert a geographic bounding box to a screen rect (illustrative)
fn geo_rect_to_egui(geo: &elicit_geo_types::Rect, scale: f32) -> egui::Rect {
    let min = geo.min();
    let max = geo.max();
    egui::Rect::from_min_max(
        egui::pos2(min.x() as f32 * scale, min.y() as f32 * scale),
        egui::pos2(max.x() as f32 * scale, max.y() as f32 * scale),
    )
}
```

## License

Apache-2.0 OR MIT
