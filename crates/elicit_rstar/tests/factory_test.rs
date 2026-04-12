//! Integration tests for `elicit_rstar` dynamic factories.

use elicit_rstar::{RstarTree, prime_point_distance_tree, prime_rtree_object_tree};
use elicitation::{DynamicToolRegistry, ElicitPlugin, RstarRectangle, ToolFactoryRegistration};

fn result_text(result: rmcp::model::CallToolResult) -> String {
    result
        .content
        .first()
        .and_then(|content| match &content.raw {
            rmcp::model::RawContent::Text(text) => Some(text.text.clone()),
            _ => None,
        })
        .unwrap_or_default()
}

fn rectangle(lower: [f64; 2], upper: [f64; 2]) -> RstarRectangle {
    rstar::primitives::Rectangle::from_corners(lower, upper).into()
}

#[test]
fn factories_are_in_inventory() {
    let names: Vec<&'static str> = inventory::iter::<ToolFactoryRegistration>
        .into_iter()
        .map(|registration| registration.trait_name)
        .collect();
    assert!(names.contains(&"elicit_rstar::RTreeObjectFactory"));
    assert!(names.contains(&"elicit_rstar::PointDistanceFactory"));
}

#[test]
fn meta_tools_are_visible() {
    let registry = DynamicToolRegistry::new();
    let names: Vec<String> = registry
        .list_tools()
        .iter()
        .map(|tool| tool.name.to_string())
        .collect();
    assert!(
        names.contains(&"instantiate_elicit_rstar___r_tree_object_factory".to_string()),
        "missing object factory meta-tool: {names:?}"
    );
    assert!(
        names.contains(&"instantiate_elicit_rstar___point_distance_factory".to_string()),
        "missing point-distance factory meta-tool: {names:?}"
    );
}

#[tokio::test]
async fn rectangle_tree_factories_instantiate_and_query() {
    prime_rtree_object_tree::<RstarRectangle>();
    prime_point_distance_tree::<RstarRectangle>();

    let registry = DynamicToolRegistry::new().register_type::<RstarTree<RstarRectangle>>("rects");

    registry
        .instantiate("elicit_rstar::PointDistanceFactory", "rects")
        .await
        .expect("point-distance factory should instantiate");

    let bulk_load = registry
        .invoke_dynamic(
            "rects__bulk_load",
            serde_json::json!({
                "items": [
                    rectangle([0.0, 0.0], [1.0, 1.0]),
                    rectangle([2.0, 2.0], [3.0, 3.0])
                ]
            }),
        )
        .await
        .expect("bulk_load tool should exist")
        .expect("bulk_load should succeed");
    let target: serde_json::Value =
        serde_json::from_str(&result_text(bulk_load)).expect("bulk_load should return a tree");

    let size = registry
        .invoke_dynamic(
            "rects__size",
            serde_json::json!({ "target": target.clone() }),
        )
        .await
        .expect("size tool should exist")
        .expect("size should succeed");
    assert_eq!(result_text(size), "2");

    let contained = registry
        .invoke_dynamic(
            "rects__locate_in_envelope",
            serde_json::json!({
                "target": target.clone(),
                "envelope": {
                    "lower": [0.0, 0.0],
                    "upper": [1.5, 1.5]
                }
            }),
        )
        .await
        .expect("locate_in_envelope tool should exist")
        .expect("locate_in_envelope should succeed");
    let contained: Vec<RstarRectangle> =
        serde_json::from_str(&result_text(contained)).expect("query should return rectangles");
    assert_eq!(contained.len(), 1);

    let nearest = registry
        .invoke_dynamic(
            "rects__nearest_neighbor",
            serde_json::json!({
                "target": target,
                "point": [2.2, 2.2]
            }),
        )
        .await
        .expect("nearest_neighbor tool should exist")
        .expect("nearest_neighbor should succeed");
    let nearest: Option<RstarRectangle> =
        serde_json::from_str(&result_text(nearest)).expect("nearest should deserialize");
    assert_eq!(nearest, Some(rectangle([2.0, 2.0], [3.0, 3.0])));
}
