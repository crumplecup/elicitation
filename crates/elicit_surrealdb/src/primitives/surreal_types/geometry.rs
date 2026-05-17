//! Trenchcoat wrapper for [`surrealdb_types::Geometry`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/// A SurrealDB geometry value.
///
/// Mirrors `surrealdb_types::Geometry`. GeoJSON-compatible representation
/// for spatial data crossing the MCP boundary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", content = "coordinates")]
pub enum Geometry {
    /// A single 2D point `[longitude, latitude]`.
    Point([f64; 2]),
    /// A line string — ordered sequence of coordinate pairs.
    Line(Vec<[f64; 2]>),
    /// A polygon — outer ring plus optional holes, each a list of coordinate pairs.
    Polygon(Vec<Vec<[f64; 2]>>),
    /// Multiple points.
    MultiPoint(Vec<[f64; 2]>),
    /// Multiple line strings.
    MultiLine(Vec<Vec<[f64; 2]>>),
    /// Multiple polygons.
    MultiPolygon(Vec<Vec<Vec<[f64; 2]>>>),
    /// A collection of geometry values.
    Collection(Vec<Geometry>),
}

/// Extract a flat coordinate list from a geo LineString.
fn linestring_to_coords(ls: &geo_types::LineString) -> Vec<[f64; 2]> {
    ls.coords().map(|c| [c.x, c.y]).collect()
}

/// Build a geo LineString from a flat coordinate list.
fn coords_to_linestring(coords: Vec<[f64; 2]>) -> geo_types::LineString {
    geo_types::LineString(
        coords
            .into_iter()
            .map(|[x, y]| geo_types::Coord { x, y })
            .collect(),
    )
}

/// Build a geo Polygon from a rings list (first ring is exterior, rest are holes).
fn rings_to_polygon(rings: Vec<Vec<[f64; 2]>>) -> geo_types::Polygon {
    let mut iter = rings.into_iter();
    let exterior = iter
        .next()
        .map(coords_to_linestring)
        .unwrap_or_else(|| geo_types::LineString::new(vec![]));
    let interiors: Vec<_> = iter.map(coords_to_linestring).collect();
    geo_types::Polygon::new(exterior, interiors)
}

impl From<surrealdb_types::Geometry> for Geometry {
    fn from(g: surrealdb_types::Geometry) -> Self {
        match g {
            surrealdb_types::Geometry::Point(p) => Geometry::Point([p.x(), p.y()]),
            surrealdb_types::Geometry::Line(ls) => Geometry::Line(linestring_to_coords(&ls)),
            surrealdb_types::Geometry::Polygon(poly) => {
                let mut rings = vec![linestring_to_coords(poly.exterior())];
                rings.extend(poly.interiors().iter().map(linestring_to_coords));
                Geometry::Polygon(rings)
            }
            surrealdb_types::Geometry::MultiPoint(mp) => {
                Geometry::MultiPoint(mp.0.iter().map(|p| [p.x(), p.y()]).collect())
            }
            surrealdb_types::Geometry::MultiLine(mls) => {
                Geometry::MultiLine(mls.0.iter().map(linestring_to_coords).collect())
            }
            surrealdb_types::Geometry::MultiPolygon(mpoly) => Geometry::MultiPolygon(
                mpoly
                    .0
                    .iter()
                    .map(|poly| {
                        let mut rings = vec![linestring_to_coords(poly.exterior())];
                        rings.extend(poly.interiors().iter().map(linestring_to_coords));
                        rings
                    })
                    .collect(),
            ),
            surrealdb_types::Geometry::Collection(coll) => {
                Geometry::Collection(coll.into_iter().map(Geometry::from).collect())
            }
        }
    }
}

impl From<Geometry> for surrealdb_types::Geometry {
    fn from(g: Geometry) -> Self {
        use geo_types::{Coord, MultiLineString, MultiPoint, MultiPolygon, Point};
        match g {
            Geometry::Point([x, y]) => surrealdb_types::Geometry::Point(Point(Coord { x, y })),
            Geometry::Line(coords) => surrealdb_types::Geometry::Line(coords_to_linestring(coords)),
            Geometry::Polygon(rings) => surrealdb_types::Geometry::Polygon(rings_to_polygon(rings)),
            Geometry::MultiPoint(pts) => surrealdb_types::Geometry::MultiPoint(MultiPoint(
                pts.into_iter()
                    .map(|[x, y]| Point(Coord { x, y }))
                    .collect(),
            )),
            Geometry::MultiLine(lines) => surrealdb_types::Geometry::MultiLine(MultiLineString(
                lines.into_iter().map(coords_to_linestring).collect(),
            )),
            Geometry::MultiPolygon(polys) => surrealdb_types::Geometry::MultiPolygon(MultiPolygon(
                polys.into_iter().map(rings_to_polygon).collect(),
            )),
            Geometry::Collection(coll) => surrealdb_types::Geometry::Collection(
                coll.into_iter()
                    .map(surrealdb_types::Geometry::from)
                    .collect(),
            ),
        }
    }
}

use elicitation::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, TypeMetadata, VariantMetadata, mcp,
};

elicitation::default_style!(Geometry => GeometryStyle);

impl Prompt for Geometry {
    fn prompt() -> Option<&'static str> {
        Some("Choose the SurrealDB geometry variant:")
    }
}

impl Elicitation for Geometry {
    type Style = GeometryStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Geometry");
        let type_labels: Vec<String> = vec![
            "Point".to_string(),
            "Line".to_string(),
            "Polygon".to_string(),
            "MultiPoint".to_string(),
            "MultiLine".to_string(),
            "MultiPolygon".to_string(),
            "Collection (JSON)".to_string(),
        ];
        let type_params = mcp::select_params(
            Self::prompt().unwrap_or("Choose the geometry variant:"),
            &type_labels,
        );
        let type_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(type_params),
            )
            .await?;
        let variant = mcp::parse_string(mcp::extract_value(type_result)?)?;
        tracing::debug!(variant = %variant, "Selected Geometry variant");

        match variant.as_str() {
            "Point" => {
                let coords = ask_coord_json(
                    communicator,
                    "Enter a coordinate as JSON [lon, lat] (e.g. [1.0, 2.0]):",
                )
                .await?;
                let pair = parse_coord_pair(&coords)?;
                Ok(Geometry::Point(pair))
            }
            "Line" => {
                let coords = ask_coord_json(
                    communicator,
                    "Enter line coordinates as JSON [[lon, lat], …]:",
                )
                .await?;
                Ok(Geometry::Line(parse_coord_list(&coords)?))
            }
            "Polygon" => {
                let coords = ask_coord_json(
                    communicator,
                    "Enter polygon rings as JSON [[[lon, lat], …], …] (first ring is outer boundary):",
                )
                .await?;
                Ok(Geometry::Polygon(parse_ring_list(&coords)?))
            }
            "MultiPoint" => {
                let coords = ask_coord_json(
                    communicator,
                    "Enter multiple points as JSON [[lon, lat], …]:",
                )
                .await?;
                Ok(Geometry::MultiPoint(parse_coord_list(&coords)?))
            }
            "MultiLine" => {
                let coords = ask_coord_json(
                    communicator,
                    "Enter multiple lines as JSON [[[lon, lat], …], …]:",
                )
                .await?;
                Ok(Geometry::MultiLine(parse_ring_list(&coords)?))
            }
            "MultiPolygon" => {
                let coords = ask_coord_json(
                    communicator,
                    "Enter multiple polygons as JSON [[[[lon, lat], …], …], …]:",
                )
                .await?;
                Ok(Geometry::MultiPolygon(parse_polygon_list(&coords)?))
            }
            _ => {
                // Collection — ask for the full geometry as JSON
                let json_params =
                    mcp::text_params("Enter the collection as a JSON array of Geometry objects:");
                let json_result = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                            .with_arguments(json_params),
                    )
                    .await?;
                let json_str = mcp::parse_string(mcp::extract_value(json_result)?)?;
                let coll: Vec<Geometry> = serde_json::from_str(json_str.trim()).map_err(|e| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid geometry collection JSON: {}",
                        e
                    )))
                })?;
                Ok(Geometry::Collection(coll))
            }
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        elicitation::verification::proof_helpers::kani_trusted_opaque("geometry")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        elicitation::verification::proof_helpers::verus_trusted_opaque("geometry")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        elicitation::verification::proof_helpers::creusot_trusted_opaque("geometry")
    }
}

impl ElicitIntrospect for Geometry {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "SurrealGeometry",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: [
                    "Point",
                    "Line",
                    "Polygon",
                    "MultiPoint",
                    "MultiLine",
                    "MultiPolygon",
                    "Collection",
                ]
                .into_iter()
                .map(|label| VariantMetadata {
                    label: label.to_string(),
                    fields: vec![],
                })
                .collect(),
            },
        }
    }
}

impl elicitation::ElicitPromptTree for Geometry {
    fn prompt_tree() -> elicitation::PromptTree {
        let opts: Vec<String> = [
            "Point",
            "Line",
            "Polygon",
            "MultiPoint",
            "MultiLine",
            "MultiPolygon",
            "Collection (JSON)",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let n = opts.len();
        elicitation::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose the SurrealDB geometry variant:")
                .to_string(),
            type_name: "SurrealGeometry".to_string(),
            options: opts,
            branches: vec![None; n],
        }
    }
}

impl elicitation::emit_code::ToCodeLiteral for Geometry {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let json = serde_json::to_string(self).expect("Geometry should serialize");
        quote::quote! {
            ::serde_json::from_str::<elicit_surrealdb::SurrealGeometry>(#json)
                .expect("serialized SurrealGeometry should deserialize")
        }
    }
}

/// Ask the communicator for a JSON coordinate string.
async fn ask_coord_json<C: ElicitCommunicator>(
    communicator: &C,
    prompt: &str,
) -> ElicitResult<String> {
    let params = mcp::text_params(prompt);
    let result = communicator
        .call_tool(
            rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                .with_arguments(params),
        )
        .await?;
    mcp::parse_string(mcp::extract_value(result)?)
}

fn parse_coord_pair(s: &str) -> ElicitResult<[f64; 2]> {
    let arr: Vec<f64> = serde_json::from_str(s.trim()).map_err(|e| {
        ElicitError::new(ElicitErrorKind::ParseError(format!(
            "Invalid coordinate pair JSON (expected [lon, lat]): {}",
            e
        )))
    })?;
    if arr.len() != 2 {
        return Err(ElicitError::new(ElicitErrorKind::ParseError(format!(
            "Expected exactly 2 values, got {}",
            arr.len()
        ))));
    }
    Ok([arr[0], arr[1]])
}

fn parse_coord_list(s: &str) -> ElicitResult<Vec<[f64; 2]>> {
    let arr: Vec<Vec<f64>> = serde_json::from_str(s.trim()).map_err(|e| {
        ElicitError::new(ElicitErrorKind::ParseError(format!(
            "Invalid coordinate list JSON: {}",
            e
        )))
    })?;
    arr.into_iter()
        .map(|pair| {
            if pair.len() != 2 {
                return Err(ElicitError::new(ElicitErrorKind::ParseError(
                    "Each coordinate must have exactly 2 values".to_string(),
                )));
            }
            Ok([pair[0], pair[1]])
        })
        .collect()
}

fn parse_ring_list(s: &str) -> ElicitResult<Vec<Vec<[f64; 2]>>> {
    let arr: Vec<Vec<Vec<f64>>> = serde_json::from_str(s.trim()).map_err(|e| {
        ElicitError::new(ElicitErrorKind::ParseError(format!(
            "Invalid ring list JSON: {}",
            e
        )))
    })?;
    arr.into_iter()
        .map(|ring| {
            ring.into_iter()
                .map(|pair| {
                    if pair.len() != 2 {
                        return Err(ElicitError::new(ElicitErrorKind::ParseError(
                            "Each coordinate must have exactly 2 values".to_string(),
                        )));
                    }
                    Ok([pair[0], pair[1]])
                })
                .collect()
        })
        .collect()
}

fn parse_polygon_list(s: &str) -> ElicitResult<Vec<Vec<Vec<[f64; 2]>>>> {
    let arr: Vec<Vec<Vec<Vec<f64>>>> = serde_json::from_str(s.trim()).map_err(|e| {
        ElicitError::new(ElicitErrorKind::ParseError(format!(
            "Invalid polygon list JSON: {}",
            e
        )))
    })?;
    arr.into_iter()
        .map(|poly| {
            poly.into_iter()
                .map(|ring| {
                    ring.into_iter()
                        .map(|pair| {
                            if pair.len() != 2 {
                                return Err(ElicitError::new(ElicitErrorKind::ParseError(
                                    "Each coordinate must have exactly 2 values".to_string(),
                                )));
                            }
                            Ok([pair[0], pair[1]])
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}
