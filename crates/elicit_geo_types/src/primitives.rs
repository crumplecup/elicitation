//! Primitive geometric types: `Coord`, `Line`, `Point`, `Triangle`.

use elicitation::{GeoCoord, GeoLine, GeoPoint, GeoTriangle, elicit_newtype};
use elicitation_derive::reflect_methods;
use tracing::instrument;

// ── Coord ────────────────────────────────────────────────────────────────────

elicit_newtype!(GeoCoord, as Coord, serde);

impl Coord {
    /// Creates a coordinate from x and y values.
    #[instrument]
    pub fn new(x: f64, y: f64) -> Self {
        GeoCoord { x, y }.into()
    }
}

#[reflect_methods]
impl Coord {
    /// Returns the x (longitude) component.
    #[instrument(skip(self))]
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Returns the y (latitude) component.
    #[instrument(skip(self))]
    pub fn y(&self) -> f64 {
        self.y
    }
}

impl elicitation::ElicitComplete for Coord {}

mod coord_emit {
    use super::Coord;
    impl elicitation::emit_code::ToCodeLiteral for Coord {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(self).expect("Coord is serializable");
            quote::quote! {
                ::elicit_geo_types::Coord::from(
                    ::serde_json::from_str::<::elicitation::GeoCoord>(#json)
                        .expect("valid Coord JSON")
                )
            }
        }
    }
}

// ── Line ──────────────────────────────────────────────────────────────────────

elicit_newtype!(GeoLine, as Line, serde);

impl Line {
    /// Creates a line segment from start and end coordinates.
    #[instrument]
    pub fn new(start: Coord, end: Coord) -> Self {
        GeoLine {
            start: *start,
            end: *end,
        }
        .into()
    }
}

#[reflect_methods]
impl Line {
    /// Returns the start coordinate.
    #[instrument(skip(self))]
    pub fn start(&self) -> Coord {
        Coord::from(self.start)
    }

    /// Returns the end coordinate.
    #[instrument(skip(self))]
    pub fn end(&self) -> Coord {
        Coord::from(self.end)
    }

    /// Returns the x-extent (end.x − start.x).
    #[instrument(skip(self))]
    pub fn dx(&self) -> f64 {
        self.end.x - self.start.x
    }

    /// Returns the y-extent (end.y − start.y).
    #[instrument(skip(self))]
    pub fn dy(&self) -> f64 {
        self.end.y - self.start.y
    }
}

impl elicitation::ElicitComplete for Line {}

mod line_emit {
    use super::Line;
    impl elicitation::emit_code::ToCodeLiteral for Line {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(self).expect("Line is serializable");
            quote::quote! {
                ::elicit_geo_types::Line::from(
                    ::serde_json::from_str::<::elicitation::GeoLine>(#json)
                        .expect("valid Line JSON")
                )
            }
        }
    }
}

// ── Point ─────────────────────────────────────────────────────────────────────

elicit_newtype!(GeoPoint, as Point, serde);

impl Point {
    /// Creates a point from x and y values.
    #[instrument]
    pub fn new(x: f64, y: f64) -> Self {
        GeoPoint {
            coord: GeoCoord { x, y },
        }
        .into()
    }
}

#[reflect_methods]
impl Point {
    /// Returns the x component.
    #[instrument(skip(self))]
    pub fn x(&self) -> f64 {
        self.coord.x
    }

    /// Returns the y component.
    #[instrument(skip(self))]
    pub fn y(&self) -> f64 {
        self.coord.y
    }

    /// Returns the longitude (x component).
    #[instrument(skip(self))]
    pub fn lng(&self) -> f64 {
        self.coord.x
    }

    /// Returns the latitude (y component).
    #[instrument(skip(self))]
    pub fn lat(&self) -> f64 {
        self.coord.y
    }
}

impl elicitation::ElicitComplete for Point {}

mod point_emit {
    use super::Point;
    impl elicitation::emit_code::ToCodeLiteral for Point {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(self).expect("Point is serializable");
            quote::quote! {
                ::elicit_geo_types::Point::from(
                    ::serde_json::from_str::<::elicitation::GeoPoint>(#json)
                        .expect("valid Point JSON")
                )
            }
        }
    }
}

// ── Triangle ─────────────────────────────────────────────────────────────────

elicit_newtype!(GeoTriangle, as Triangle, serde);

impl Triangle {
    /// Creates a triangle from three vertex coordinates.
    #[instrument]
    pub fn new(v1: Coord, v2: Coord, v3: Coord) -> Self {
        GeoTriangle {
            v1: *v1,
            v2: *v2,
            v3: *v3,
        }
        .into()
    }
}

#[reflect_methods]
impl Triangle {
    /// Returns the first vertex.
    #[instrument(skip(self))]
    pub fn v1(&self) -> Coord {
        Coord::from(self.v1)
    }

    /// Returns the second vertex.
    #[instrument(skip(self))]
    pub fn v2(&self) -> Coord {
        Coord::from(self.v2)
    }

    /// Returns the third vertex.
    #[instrument(skip(self))]
    pub fn v3(&self) -> Coord {
        Coord::from(self.v3)
    }
}

impl elicitation::ElicitComplete for Triangle {}

mod triangle_emit {
    use super::Triangle;
    impl elicitation::emit_code::ToCodeLiteral for Triangle {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(self).expect("Triangle is serializable");
            quote::quote! {
                ::elicit_geo_types::Triangle::from(
                    ::serde_json::from_str::<::elicitation::GeoTriangle>(#json)
                        .expect("valid Triangle JSON")
                )
            }
        }
    }
}
