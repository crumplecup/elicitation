//! Wrapper for [`geo_types::Geometry<f64>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};

use super::{
    coord::GeoCoord, line::GeoLine, line_string::GeoLineString,
    multi_line_string::GeoMultiLineString, multi_point::GeoMultiPoint,
    multi_polygon::GeoMultiPolygon, point::GeoPoint, polygon::GeoPolygon, rect::GeoRect,
    triangle::GeoTriangle,
};

// Forward declaration — GeoGeometryCollection is in a sibling module that
// depends on GeoGeometry, so we import it lazily via the module path.
use super::geometry_collection::GeoGeometryCollection;

use geo_types::Geometry;

/// Elicitable representation of [`geo_types::Geometry<f64>`].
///
/// A tagged-union enum covering every concrete geometry variant in the
/// geo-types crate. Elicitation first presents a variant selector, then
/// recursively elicits the chosen variant's inner data.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", content = "geometry")]
pub enum GeoGeometry {
    /// A single geographic point.
    Point(GeoPoint),
    /// A line segment between two coordinates.
    Line(GeoLine),
    /// An ordered sequence of coordinates.
    LineString(GeoLineString),
    /// A polygon with an exterior ring and optional holes.
    Polygon(GeoPolygon),
    /// A collection of points.
    MultiPoint(GeoMultiPoint),
    /// A collection of line strings.
    MultiLineString(GeoMultiLineString),
    /// A collection of polygons.
    MultiPolygon(GeoMultiPolygon),
    /// An axis-aligned bounding rectangle.
    Rect(GeoRect),
    /// A triangle defined by three vertices.
    Triangle(GeoTriangle),
    /// A heterogeneous collection of geometry values.
    GeometryCollection(GeoGeometryCollection),
}

impl From<Geometry<f64>> for GeoGeometry {
    fn from(g: Geometry<f64>) -> Self {
        match g {
            Geometry::Point(p) => Self::Point(GeoPoint::from(p)),
            Geometry::Line(l) => Self::Line(GeoLine::from(l)),
            Geometry::LineString(ls) => Self::LineString(GeoLineString::from(ls)),
            Geometry::Polygon(p) => Self::Polygon(GeoPolygon::from(p)),
            Geometry::MultiPoint(mp) => Self::MultiPoint(GeoMultiPoint::from(mp)),
            Geometry::MultiLineString(mls) => Self::MultiLineString(GeoMultiLineString::from(mls)),
            Geometry::MultiPolygon(mp) => Self::MultiPolygon(GeoMultiPolygon::from(mp)),
            Geometry::Rect(r) => Self::Rect(GeoRect::from(r)),
            Geometry::Triangle(t) => Self::Triangle(GeoTriangle::from(t)),
            Geometry::GeometryCollection(gc) => {
                Self::GeometryCollection(GeoGeometryCollection::from(gc))
            }
        }
    }
}

impl From<GeoGeometry> for Geometry<f64> {
    fn from(g: GeoGeometry) -> Self {
        match g {
            GeoGeometry::Point(p) => Geometry::Point(p.into()),
            GeoGeometry::Line(l) => Geometry::Line(l.into()),
            GeoGeometry::LineString(ls) => Geometry::LineString(ls.into()),
            GeoGeometry::Polygon(p) => Geometry::Polygon(p.into()),
            GeoGeometry::MultiPoint(mp) => Geometry::MultiPoint(mp.into()),
            GeoGeometry::MultiLineString(mls) => Geometry::MultiLineString(mls.into()),
            GeoGeometry::MultiPolygon(mp) => Geometry::MultiPolygon(mp.into()),
            GeoGeometry::Rect(r) => Geometry::Rect(r.into()),
            GeoGeometry::Triangle(t) => Geometry::Triangle(t.into()),
            GeoGeometry::GeometryCollection(gc) => Geometry::GeometryCollection(gc.into()),
        }
    }
}

crate::default_style!(GeoGeometry => GeoGeometryStyle);

impl Prompt for GeoGeometry {
    fn prompt() -> Option<&'static str> {
        Some("Choose a geometry type:")
    }
}

impl Select for GeoGeometry {
    fn options() -> Vec<Self> {
        vec![
            Self::Point(GeoPoint {
                coord: GeoCoord { x: 0.0, y: 0.0 },
            }),
            Self::Line(GeoLine {
                start: GeoCoord { x: 0.0, y: 0.0 },
                end: GeoCoord { x: 1.0, y: 1.0 },
            }),
            Self::LineString(GeoLineString(vec![])),
            Self::Polygon(GeoPolygon {
                exterior: GeoLineString(vec![]),
                interiors: vec![],
            }),
            Self::MultiPoint(GeoMultiPoint(vec![])),
            Self::MultiLineString(GeoMultiLineString(vec![])),
            Self::MultiPolygon(GeoMultiPolygon(vec![])),
            Self::Rect(GeoRect {
                min: GeoCoord { x: 0.0, y: 0.0 },
                max: GeoCoord { x: 1.0, y: 1.0 },
            }),
            Self::Triangle(GeoTriangle {
                v1: GeoCoord { x: 0.0, y: 0.0 },
                v2: GeoCoord { x: 1.0, y: 0.0 },
                v3: GeoCoord { x: 0.0, y: 1.0 },
            }),
            Self::GeometryCollection(GeoGeometryCollection(vec![])),
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Point".to_string(),
            "Line".to_string(),
            "LineString".to_string(),
            "Polygon".to_string(),
            "MultiPoint".to_string(),
            "MultiLineString".to_string(),
            "MultiPolygon".to_string(),
            "Rect".to_string(),
            "Triangle".to_string(),
            "GeometryCollection".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Point" => Some(Self::Point(GeoPoint {
                coord: GeoCoord { x: 0.0, y: 0.0 },
            })),
            "Line" => Some(Self::Line(GeoLine {
                start: GeoCoord { x: 0.0, y: 0.0 },
                end: GeoCoord { x: 1.0, y: 1.0 },
            })),
            "LineString" => Some(Self::LineString(GeoLineString(vec![]))),
            "Polygon" => Some(Self::Polygon(GeoPolygon {
                exterior: GeoLineString(vec![]),
                interiors: vec![],
            })),
            "MultiPoint" => Some(Self::MultiPoint(GeoMultiPoint(vec![]))),
            "MultiLineString" => Some(Self::MultiLineString(GeoMultiLineString(vec![]))),
            "MultiPolygon" => Some(Self::MultiPolygon(GeoMultiPolygon(vec![]))),
            "Rect" => Some(Self::Rect(GeoRect {
                min: GeoCoord { x: 0.0, y: 0.0 },
                max: GeoCoord { x: 1.0, y: 1.0 },
            })),
            "Triangle" => Some(Self::Triangle(GeoTriangle {
                v1: GeoCoord { x: 0.0, y: 0.0 },
                v2: GeoCoord { x: 1.0, y: 0.0 },
                v3: GeoCoord { x: 0.0, y: 1.0 },
            })),
            "GeometryCollection" => Some(Self::GeometryCollection(GeoGeometryCollection(vec![]))),
            _ => None,
        }
    }
}

impl Elicitation for GeoGeometry {
    type Style = GeoGeometryStyle;

    // Use explicit fn + Box::pin to break the mutual-recursion cycle between
    // GeoGeometry and GeoGeometryCollection. Without boxing, the Rust compiler
    // cannot verify `Send` for the recursive opaque Future type.
    //
    // Note: #[tracing::instrument] is intentionally absent here — it wraps the
    // return in a new impl Future and re-evaluates Send bounds, which recurses
    // infinitely through GeoGeometry→GeoGeometryCollection→Vec<GeoGeometry>.
    // Manual tracing::debug! calls below provide equivalent observability.
    fn elicit<C: ElicitCommunicator>(
        communicator: &C,
    ) -> impl std::future::Future<Output = ElicitResult<Self>> + Send {
        let comm = communicator.clone();
        Box::pin(async move {
            let communicator = &comm;
            tracing::debug!("Eliciting GeoGeometry variant");
            let params = mcp::select_params(
                Self::prompt().unwrap_or("Choose a geometry type:"),
                &Self::labels(),
            );
            let result = communicator
                .call_tool(
                    rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                        .with_arguments(params),
                )
                .await?;
            let value = mcp::extract_value(result)?;
            let label = mcp::parse_string(value)?;
            tracing::debug!(variant = %label, "GeoGeometry variant selected");

            match label.as_str() {
                "Point" => Ok(Self::Point(GeoPoint::elicit(communicator).await?)),
                "Line" => Ok(Self::Line(GeoLine::elicit(communicator).await?)),
                "LineString" => Ok(Self::LineString(GeoLineString::elicit(communicator).await?)),
                "Polygon" => Ok(Self::Polygon(GeoPolygon::elicit(communicator).await?)),
                "MultiPoint" => Ok(Self::MultiPoint(GeoMultiPoint::elicit(communicator).await?)),
                "MultiLineString" => Ok(Self::MultiLineString(
                    GeoMultiLineString::elicit(communicator).await?,
                )),
                "MultiPolygon" => Ok(Self::MultiPolygon(
                    GeoMultiPolygon::elicit(communicator).await?,
                )),
                "Rect" => Ok(Self::Rect(GeoRect::elicit(communicator).await?)),
                "Triangle" => Ok(Self::Triangle(GeoTriangle::elicit(communicator).await?)),
                "GeometryCollection" => Ok(Self::GeometryCollection(
                    GeoGeometryCollection::elicit(communicator).await?,
                )),
                _ => Err(ElicitError::new(ElicitErrorKind::ParseError(format!(
                    "Invalid GeoGeometry variant: {}",
                    label
                )))),
            }
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("GeoGeometry", "Point")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("GeoGeometry", "Point")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("GeoGeometry", "Point")
    }
}

impl ElicitIntrospect for GeoGeometry {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "geo_types::Geometry<f64>",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for GeoGeometry {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose a geometry type:")
                .to_string(),
            type_name: "GeoGeometry".to_string(),
            options: Self::labels(),
            branches: vec![
                Some(Box::new(GeoPoint::prompt_tree())),
                Some(Box::new(GeoLine::prompt_tree())),
                Some(Box::new(GeoLineString::prompt_tree())),
                Some(Box::new(GeoPolygon::prompt_tree())),
                Some(Box::new(GeoMultiPoint::prompt_tree())),
                Some(Box::new(GeoMultiLineString::prompt_tree())),
                Some(Box::new(GeoMultiPolygon::prompt_tree())),
                Some(Box::new(GeoRect::prompt_tree())),
                Some(Box::new(GeoTriangle::prompt_tree())),
                Some(Box::new(GeoGeometryCollection::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for GeoGeometry {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            GeoGeometry::Point(p) => {
                let inner = p.to_code_literal();
                quote::quote! { elicitation::GeoGeometry::Point(#inner) }
            }
            GeoGeometry::Line(l) => {
                let inner = l.to_code_literal();
                quote::quote! { elicitation::GeoGeometry::Line(#inner) }
            }
            GeoGeometry::LineString(ls) => {
                let inner = ls.to_code_literal();
                quote::quote! { elicitation::GeoGeometry::LineString(#inner) }
            }
            GeoGeometry::Polygon(p) => {
                let inner = p.to_code_literal();
                quote::quote! { elicitation::GeoGeometry::Polygon(#inner) }
            }
            GeoGeometry::MultiPoint(mp) => {
                let inner = mp.to_code_literal();
                quote::quote! { elicitation::GeoGeometry::MultiPoint(#inner) }
            }
            GeoGeometry::MultiLineString(mls) => {
                let inner = mls.to_code_literal();
                quote::quote! { elicitation::GeoGeometry::MultiLineString(#inner) }
            }
            GeoGeometry::MultiPolygon(mp) => {
                let inner = mp.to_code_literal();
                quote::quote! { elicitation::GeoGeometry::MultiPolygon(#inner) }
            }
            GeoGeometry::Rect(r) => {
                let inner = r.to_code_literal();
                quote::quote! { elicitation::GeoGeometry::Rect(#inner) }
            }
            GeoGeometry::Triangle(t) => {
                let inner = t.to_code_literal();
                quote::quote! { elicitation::GeoGeometry::Triangle(#inner) }
            }
            GeoGeometry::GeometryCollection(gc) => {
                let inner = gc.to_code_literal();
                quote::quote! { elicitation::GeoGeometry::GeometryCollection(#inner) }
            }
        }
    }
}
