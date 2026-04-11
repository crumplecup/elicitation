//! Elicitation support for [`geojson::GeometryValue`].

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, GeoGeometryCollection, GeoLineString, GeoMultiLineString, GeoMultiPoint,
    GeoMultiPolygon, GeoPoint, GeoPolygon, PatternDetails, Prompt, Select, TypeMetadata,
    VariantMetadata, mcp,
};
use geo_types::{
    GeometryCollection as GeoTypesGeometryCollection, LineString as GeoTypesLineString,
    MultiLineString as GeoTypesMultiLineString, MultiPoint as GeoTypesMultiPoint,
    MultiPolygon as GeoTypesMultiPolygon, Point as GeoTypesPoint, Polygon as GeoTypesPolygon,
};
use geojson::Value as GeometryValue;

use super::helpers::serde_json_code_literal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GeoJsonGeometryValueKind {
    Point,
    MultiPoint,
    LineString,
    MultiLineString,
    Polygon,
    MultiPolygon,
    GeometryCollection,
}

impl Prompt for GeoJsonGeometryValueKind {
    fn prompt() -> Option<&'static str> {
        Some("Choose a GeoJSON geometry value type:")
    }
}

impl Select for GeoJsonGeometryValueKind {
    fn options() -> Vec<Self> {
        vec![
            Self::Point,
            Self::MultiPoint,
            Self::LineString,
            Self::MultiLineString,
            Self::Polygon,
            Self::MultiPolygon,
            Self::GeometryCollection,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Point".to_string(),
            "MultiPoint".to_string(),
            "LineString".to_string(),
            "MultiLineString".to_string(),
            "Polygon".to_string(),
            "MultiPolygon".to_string(),
            "GeometryCollection".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Point" => Some(Self::Point),
            "MultiPoint" => Some(Self::MultiPoint),
            "LineString" => Some(Self::LineString),
            "MultiLineString" => Some(Self::MultiLineString),
            "Polygon" => Some(Self::Polygon),
            "MultiPolygon" => Some(Self::MultiPolygon),
            "GeometryCollection" => Some(Self::GeometryCollection),
            _ => None,
        }
    }
}

crate::default_style!(GeometryValue => GeoJsonGeometryValueStyle);

impl Prompt for GeometryValue {
    fn prompt() -> Option<&'static str> {
        Some("Choose a GeoJSON geometry value:")
    }
}

impl Elicitation for GeometryValue {
    type Style = GeoJsonGeometryValueStyle;

    #[tracing::instrument(skip(communicator))]
    fn elicit<C: ElicitCommunicator>(
        communicator: &C,
    ) -> impl std::future::Future<Output = ElicitResult<Self>> + Send {
        let comm = communicator.clone();
        Box::pin(async move {
            let communicator = &comm;
            let result = communicator
                .call_tool(
                    rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                        .with_arguments(mcp::select_params(
                            GeoJsonGeometryValueKind::prompt()
                                .unwrap_or("Choose a GeoJSON geometry value type:"),
                            &GeoJsonGeometryValueKind::labels(),
                        )),
                )
                .await?;
            let value = mcp::extract_value(result)?;
            let label = mcp::parse_string(value)?;
            match GeoJsonGeometryValueKind::from_label(&label) {
                Some(GeoJsonGeometryValueKind::Point) => {
                    let point: GeoTypesPoint<f64> = GeoPoint::elicit(communicator).await?.into();
                    Ok(Self::from(&point))
                }
                Some(GeoJsonGeometryValueKind::MultiPoint) => {
                    let multi_point: GeoTypesMultiPoint<f64> =
                        GeoMultiPoint::elicit(communicator).await?.into();
                    Ok(Self::from(&multi_point))
                }
                Some(GeoJsonGeometryValueKind::LineString) => {
                    let line_string: GeoTypesLineString<f64> =
                        GeoLineString::elicit(communicator).await?.into();
                    Ok(Self::from(&line_string))
                }
                Some(GeoJsonGeometryValueKind::MultiLineString) => {
                    let multi_line_string: GeoTypesMultiLineString<f64> =
                        GeoMultiLineString::elicit(communicator).await?.into();
                    Ok(Self::from(&multi_line_string))
                }
                Some(GeoJsonGeometryValueKind::Polygon) => {
                    let polygon: GeoTypesPolygon<f64> =
                        GeoPolygon::elicit(communicator).await?.into();
                    Ok(Self::from(&polygon))
                }
                Some(GeoJsonGeometryValueKind::MultiPolygon) => {
                    let multi_polygon: GeoTypesMultiPolygon<f64> =
                        GeoMultiPolygon::elicit(communicator).await?.into();
                    Ok(Self::from(&multi_polygon))
                }
                Some(GeoJsonGeometryValueKind::GeometryCollection) => {
                    let collection: GeoTypesGeometryCollection<f64> =
                        GeoGeometryCollection::elicit(communicator).await?.into();
                    Ok(Self::GeometryCollection(
                        collection
                            .into_iter()
                            .map(|geometry| geojson::Geometry::from(&geometry))
                            .collect(),
                    ))
                }
                None => Err(ElicitError::new(ElicitErrorKind::InvalidSelection(label))),
            }
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("geojson::GeometryValue", "Point")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("geojson::GeometryValue", "Point")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "geojson::GeometryValue",
            "Point",
        )
    }
}

impl ElicitIntrospect for GeometryValue {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "geojson::Value",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: GeoJsonGeometryValueKind::labels()
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

impl crate::ElicitPromptTree for GeometryValue {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose a GeoJSON geometry value:")
                .to_string(),
            type_name: "geojson::Value".to_string(),
            options: GeoJsonGeometryValueKind::labels(),
            branches: vec![
                Some(Box::new(GeoPoint::prompt_tree())),
                Some(Box::new(GeoMultiPoint::prompt_tree())),
                Some(Box::new(GeoLineString::prompt_tree())),
                Some(Box::new(GeoMultiLineString::prompt_tree())),
                Some(Box::new(GeoPolygon::prompt_tree())),
                Some(Box::new(GeoMultiPolygon::prompt_tree())),
                Some(Box::new(crate::PromptTree::Leaf {
                    prompt: "Enter a GeoJSON geometry collection member list:".to_string(),
                    type_name: "Vec<geojson::Geometry>".to_string(),
                })),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for GeometryValue {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        serde_json_code_literal(self, quote::quote!(geojson::Value))
    }
}
