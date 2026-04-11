//! Select enum mirroring [`wkt::Wkt<f64>`] for elicitation.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};

use super::{
    coord::WktCoord, geometry_collection::WktGeometryCollection, linestring::WktLineString,
    multilinestring::WktMultiLineString, multipoint::WktMultiPoint, multipolygon::WktMultiPolygon,
    point::WktPoint, polygon::WktPolygon,
};

/// Elicitable enum mirroring [`wkt::Wkt<f64>`].
///
/// A tagged union covering all WKT geometry variants. Elicitation first
/// presents a variant selector, then recursively elicits the chosen variant.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", content = "geometry")]
pub enum WktGeom {
    /// A WKT point.
    Point(WktPoint),
    /// A WKT line string.
    LineString(WktLineString),
    /// A WKT polygon.
    Polygon(WktPolygon),
    /// A WKT multi-point.
    MultiPoint(WktMultiPoint),
    /// A WKT multi-line-string.
    MultiLineString(WktMultiLineString),
    /// A WKT multi-polygon.
    MultiPolygon(WktMultiPolygon),
    /// A WKT geometry collection.
    GeometryCollection(WktGeometryCollection),
}

impl From<wkt::Wkt<f64>> for WktGeom {
    fn from(w: wkt::Wkt<f64>) -> Self {
        match w {
            wkt::Wkt::Point(p) => Self::Point(WktPoint::from(p)),
            wkt::Wkt::LineString(ls) => Self::LineString(WktLineString::from(ls)),
            wkt::Wkt::Polygon(p) => Self::Polygon(WktPolygon::from(p)),
            wkt::Wkt::MultiPoint(mp) => Self::MultiPoint(WktMultiPoint::from(mp)),
            wkt::Wkt::MultiLineString(mls) => Self::MultiLineString(WktMultiLineString::from(mls)),
            wkt::Wkt::MultiPolygon(mp) => Self::MultiPolygon(WktMultiPolygon::from(mp)),
            wkt::Wkt::GeometryCollection(gc) => {
                Self::GeometryCollection(WktGeometryCollection::from(gc))
            }
        }
    }
}

impl From<WktGeom> for wkt::Wkt<f64> {
    fn from(g: WktGeom) -> Self {
        match g {
            WktGeom::Point(p) => wkt::Wkt::Point(p.into()),
            WktGeom::LineString(ls) => wkt::Wkt::LineString(ls.into()),
            WktGeom::Polygon(p) => wkt::Wkt::Polygon(p.into()),
            WktGeom::MultiPoint(mp) => wkt::Wkt::MultiPoint(mp.into()),
            WktGeom::MultiLineString(mls) => wkt::Wkt::MultiLineString(mls.into()),
            WktGeom::MultiPolygon(mp) => wkt::Wkt::MultiPolygon(mp.into()),
            WktGeom::GeometryCollection(gc) => wkt::Wkt::GeometryCollection(gc.into()),
        }
    }
}

crate::default_style!(WktGeom => WktGeomStyle);

impl Prompt for WktGeom {
    fn prompt() -> Option<&'static str> {
        Some("Choose a WKT geometry type:")
    }
}

impl Select for WktGeom {
    fn options() -> Vec<Self> {
        vec![
            Self::Point(WktPoint {
                coord: Some(WktCoord {
                    x: 0.0,
                    y: 0.0,
                    z: None,
                    m: None,
                }),
            }),
            Self::LineString(WktLineString { coords: vec![] }),
            Self::Polygon(WktPolygon {
                exterior: WktLineString { coords: vec![] },
                interiors: vec![],
            }),
            Self::MultiPoint(WktMultiPoint { points: vec![] }),
            Self::MultiLineString(WktMultiLineString { lines: vec![] }),
            Self::MultiPolygon(WktMultiPolygon { polygons: vec![] }),
            Self::GeometryCollection(WktGeometryCollection { geometries: vec![] }),
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Point".to_string(),
            "LineString".to_string(),
            "Polygon".to_string(),
            "MultiPoint".to_string(),
            "MultiLineString".to_string(),
            "MultiPolygon".to_string(),
            "GeometryCollection".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Point" => Some(Self::Point(WktPoint {
                coord: Some(WktCoord {
                    x: 0.0,
                    y: 0.0,
                    z: None,
                    m: None,
                }),
            })),
            "LineString" => Some(Self::LineString(WktLineString { coords: vec![] })),
            "Polygon" => Some(Self::Polygon(WktPolygon {
                exterior: WktLineString { coords: vec![] },
                interiors: vec![],
            })),
            "MultiPoint" => Some(Self::MultiPoint(WktMultiPoint { points: vec![] })),
            "MultiLineString" => Some(Self::MultiLineString(WktMultiLineString { lines: vec![] })),
            "MultiPolygon" => Some(Self::MultiPolygon(WktMultiPolygon { polygons: vec![] })),
            "GeometryCollection" => Some(Self::GeometryCollection(WktGeometryCollection {
                geometries: vec![],
            })),
            _ => None,
        }
    }
}

impl Elicitation for WktGeom {
    type Style = WktGeomStyle;

    // Use explicit fn + Box::pin to break the mutual-recursion cycle between
    // WktGeom and WktGeometryCollection.
    #[tracing::instrument(skip(communicator))]
    fn elicit<C: ElicitCommunicator>(
        communicator: &C,
    ) -> impl std::future::Future<Output = ElicitResult<Self>> + Send {
        let comm = communicator.clone();
        Box::pin(async move {
            let communicator = &comm;
            tracing::debug!("Eliciting WktGeom variant");
            let params = mcp::select_params(
                Self::prompt().unwrap_or("Choose a WKT geometry type:"),
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
            match label.as_str() {
                "Point" => Ok(Self::Point(WktPoint::elicit(communicator).await?)),
                "LineString" => Ok(Self::LineString(WktLineString::elicit(communicator).await?)),
                "Polygon" => Ok(Self::Polygon(WktPolygon::elicit(communicator).await?)),
                "MultiPoint" => Ok(Self::MultiPoint(WktMultiPoint::elicit(communicator).await?)),
                "MultiLineString" => Ok(Self::MultiLineString(
                    WktMultiLineString::elicit(communicator).await?,
                )),
                "MultiPolygon" => Ok(Self::MultiPolygon(
                    WktMultiPolygon::elicit(communicator).await?,
                )),
                "GeometryCollection" => Ok(Self::GeometryCollection(
                    WktGeometryCollection::elicit(communicator).await?,
                )),
                _ => Err(crate::ElicitError::new(crate::ElicitErrorKind::ParseError(
                    format!("Unknown WKT geometry variant: {label}"),
                ))),
            }
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("WktGeom", "Point")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("WktGeom", "Point")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("WktGeom", "Point")
    }
}

impl ElicitIntrospect for WktGeom {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "wkt::Wkt<f64>",
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

impl crate::ElicitPromptTree for WktGeom {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose a WKT geometry type:")
                .to_string(),
            type_name: "WktGeom".to_string(),
            options: Self::labels(),
            branches: vec![
                Some(Box::new(WktPoint::prompt_tree())),
                Some(Box::new(WktLineString::prompt_tree())),
                Some(Box::new(WktPolygon::prompt_tree())),
                Some(Box::new(WktMultiPoint::prompt_tree())),
                Some(Box::new(WktMultiLineString::prompt_tree())),
                Some(Box::new(WktMultiPolygon::prompt_tree())),
                Some(Box::new(WktGeometryCollection::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for WktGeom {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            WktGeom::Point(p) => {
                let inner = p.to_code_literal();
                quote::quote! { elicitation::WktGeom::Point(#inner) }
            }
            WktGeom::LineString(ls) => {
                let inner = ls.to_code_literal();
                quote::quote! { elicitation::WktGeom::LineString(#inner) }
            }
            WktGeom::Polygon(p) => {
                let inner = p.to_code_literal();
                quote::quote! { elicitation::WktGeom::Polygon(#inner) }
            }
            WktGeom::MultiPoint(mp) => {
                let inner = mp.to_code_literal();
                quote::quote! { elicitation::WktGeom::MultiPoint(#inner) }
            }
            WktGeom::MultiLineString(mls) => {
                let inner = mls.to_code_literal();
                quote::quote! { elicitation::WktGeom::MultiLineString(#inner) }
            }
            WktGeom::MultiPolygon(mp) => {
                let inner = mp.to_code_literal();
                quote::quote! { elicitation::WktGeom::MultiPolygon(#inner) }
            }
            WktGeom::GeometryCollection(gc) => {
                let inner = gc.to_code_literal();
                quote::quote! { elicitation::WktGeom::GeometryCollection(#inner) }
            }
        }
    }
}
