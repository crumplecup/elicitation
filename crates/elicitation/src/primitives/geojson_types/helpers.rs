use crate::{ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitResult, Elicitation, mcp};
use geojson::{Bbox, JsonObject};
use serde::Serialize;
use serde_json::Value;

pub(super) async fn confirm<C: ElicitCommunicator>(
    communicator: &C,
    prompt: &str,
) -> ElicitResult<bool> {
    let result = communicator
        .call_tool(
            rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_bool())
                .with_arguments(mcp::bool_params(prompt)),
        )
        .await?;
    let value = mcp::extract_value(result)?;
    mcp::parse_bool(value)
}

pub(super) async fn elicit_optional_bbox<C: ElicitCommunicator>(
    communicator: &C,
    prompt: &str,
) -> ElicitResult<Option<Bbox>> {
    if confirm(communicator, prompt).await? {
        Ok(Some(Vec::<f64>::elicit(communicator).await?))
    } else {
        Ok(None)
    }
}

pub(super) async fn elicit_optional_json_object<C: ElicitCommunicator>(
    communicator: &C,
    prompt: &str,
) -> ElicitResult<Option<JsonObject>> {
    if confirm(communicator, prompt).await? {
        let value = Value::elicit(communicator).await?;
        match value {
            Value::Object(map) => Ok(Some(map)),
            _ => Err(ElicitError::new(ElicitErrorKind::ParseError(
                "Expected a JSON object".to_string(),
            ))),
        }
    } else {
        Ok(None)
    }
}

pub(super) async fn elicit_optional_geometry<C: ElicitCommunicator>(
    communicator: &C,
    prompt: &str,
) -> ElicitResult<Option<geojson::Geometry>> {
    if confirm(communicator, prompt).await? {
        Ok(Some(geojson::Geometry::elicit(communicator).await?))
    } else {
        Ok(None)
    }
}

pub(super) async fn elicit_optional_id<C: ElicitCommunicator>(
    communicator: &C,
    prompt: &str,
) -> ElicitResult<Option<geojson::feature::Id>> {
    if confirm(communicator, prompt).await? {
        Ok(Some(geojson::feature::Id::elicit(communicator).await?))
    } else {
        Ok(None)
    }
}

pub(super) fn serde_json_code_literal<T: Serialize>(
    value: &T,
    type_path: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let json = serde_json::to_string(value).expect("GeoJSON value should serialize");
    quote::quote! {
        ::serde_json::from_str::<#type_path>(#json)
            .expect("serialized GeoJSON value should deserialize")
    }
}

pub(super) fn optional_json_object_prompt_tree(prompt: &str) -> crate::PromptTree {
    crate::PromptTree::Survey {
        prompt: Some(prompt.to_string()),
        type_name: "Option<geojson::JsonObject>".to_string(),
        fields: vec![
            (
                "include".to_string(),
                Box::new(crate::PromptTree::Affirm {
                    prompt: prompt.to_string(),
                    type_name: "bool".to_string(),
                }),
            ),
            (
                "object".to_string(),
                Box::new(crate::PromptTree::Leaf {
                    prompt: "Enter a JSON object:".to_string(),
                    type_name: "serde_json::Value".to_string(),
                }),
            ),
        ],
    }
}
