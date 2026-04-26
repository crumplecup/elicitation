//! Wrapper for [`wkb::reader::GeometryType`].

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};

/// Serializable mirror of [`wkb::reader::GeometryType`].
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub enum WkbGeometryType {
    /// A WKB point.
    Point,
    /// A WKB line string.
    LineString,
    /// A WKB polygon.
    Polygon,
    /// A WKB multi-point.
    MultiPoint,
    /// A WKB multi-line string.
    MultiLineString,
    /// A WKB multi-polygon.
    MultiPolygon,
    /// A WKB geometry collection.
    GeometryCollection,
}

impl TryFrom<wkb::reader::GeometryType> for WkbGeometryType {
    type Error = String;

    fn try_from(value: wkb::reader::GeometryType) -> Result<Self, Self::Error> {
        match value {
            wkb::reader::GeometryType::Point => Ok(Self::Point),
            wkb::reader::GeometryType::LineString => Ok(Self::LineString),
            wkb::reader::GeometryType::Polygon => Ok(Self::Polygon),
            wkb::reader::GeometryType::MultiPoint => Ok(Self::MultiPoint),
            wkb::reader::GeometryType::MultiLineString => Ok(Self::MultiLineString),
            wkb::reader::GeometryType::MultiPolygon => Ok(Self::MultiPolygon),
            wkb::reader::GeometryType::GeometryCollection => Ok(Self::GeometryCollection),
            _ => Err("Unsupported wkb::reader::GeometryType variant".to_string()),
        }
    }
}

impl From<WkbGeometryType> for wkb::reader::GeometryType {
    fn from(value: WkbGeometryType) -> Self {
        match value {
            WkbGeometryType::Point => Self::Point,
            WkbGeometryType::LineString => Self::LineString,
            WkbGeometryType::Polygon => Self::Polygon,
            WkbGeometryType::MultiPoint => Self::MultiPoint,
            WkbGeometryType::MultiLineString => Self::MultiLineString,
            WkbGeometryType::MultiPolygon => Self::MultiPolygon,
            WkbGeometryType::GeometryCollection => Self::GeometryCollection,
        }
    }
}

crate::default_style!(WkbGeometryType => WkbGeometryTypeStyle);

impl Prompt for WkbGeometryType {
    fn prompt() -> Option<&'static str> {
        Some("Choose the WKB geometry type:")
    }
}

impl Select for WkbGeometryType {
    fn options() -> Vec<Self> {
        vec![
            Self::Point,
            Self::LineString,
            Self::Polygon,
            Self::MultiPoint,
            Self::MultiLineString,
            Self::MultiPolygon,
            Self::GeometryCollection,
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
            "Point" => Some(Self::Point),
            "LineString" => Some(Self::LineString),
            "Polygon" => Some(Self::Polygon),
            "MultiPoint" => Some(Self::MultiPoint),
            "MultiLineString" => Some(Self::MultiLineString),
            "MultiPolygon" => Some(Self::MultiPolygon),
            "GeometryCollection" => Some(Self::GeometryCollection),
            _ => None,
        }
    }
}

impl Elicitation for WkbGeometryType {
    type Style = WkbGeometryTypeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose the WKB geometry type:"),
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
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid WkbGeometryType: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("WkbGeometryType", "Point")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("WkbGeometryType", "Point")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("WkbGeometryType", "Point")
    }
}

impl ElicitIntrospect for WkbGeometryType {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "wkb::reader::GeometryType",
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

impl crate::ElicitPromptTree for WkbGeometryType {
    fn prompt_tree() -> crate::PromptTree {
        let opts = Self::labels();
        let n = opts.len();
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose geometry type:")
                .to_string(),
            type_name: "WkbGeometryType".to_string(),
            options: opts,
            branches: vec![None; n],
        }
    }
}
