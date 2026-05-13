//! Integration tests for `BevyUiPlugin`.

use elicit_bevy::{
    BevyGridPlacementParams, BevyUiButtonBundleParams, BevyUiGridContainerParams,
    BevyUiImageParams, BevyUiNodeLiteralParams, BevyUiNodeParams, BevyUiPlugin, BevyUiRectParams,
    BevyUiTextParams,
};
use elicitation::ElicitPlugin;
use elicitation::emit_code::{EmitCode, dispatch_emit_from};

fn normalize(source: &str) -> String {
    source.chars().filter(|c| !c.is_whitespace()).collect()
}

fn from_json<T: serde::de::DeserializeOwned>(value: serde_json::Value) -> T {
    serde_json::from_value(value).expect("valid test json")
}

#[test]
fn ui_plugin_lists_expected_tools() {
    let plugin = BevyUiPlugin::new();
    let mut names = plugin
        .list_tools()
        .into_iter()
        .map(|tool| tool.name.to_string())
        .collect::<Vec<_>>();
    names.sort();

    assert_eq!(
        names,
        vec![
            "button_bundle",
            "flex_container",
            "grid_container",
            "grid_placement",
            "image",
            "node",
            "text",
            "ui_rect",
        ]
    );
}

#[test]
fn ui_rect_params_emit_literal_with_overrides() {
    let params = BevyUiRectParams {
        left_expr: Some("Val::Px(8.0)".into()),
        right_expr: Some("Val::Px(12.0)".into()),
        top_expr: None,
        bottom_expr: Some("Val::Percent(5.0)".into()),
    };
    let source = normalize(&params.emit_code().to_string());

    assert!(source.contains("::bevy::ui::UiRect{"));
    assert!(source.contains("left:Val::Px(8.0)"));
    assert!(source.contains("right:Val::Px(12.0)"));
    assert!(source.contains("top:::bevy::ui::Val::Auto"));
    assert!(source.contains("bottom:Val::Percent(5.0)"));
}

#[test]
fn grid_placement_params_emit_start_span_constructor() {
    let params: BevyGridPlacementParams = from_json(serde_json::json!({
        "kind": "start_span",
        "start": 2,
        "span": 3
    }));
    let source = normalize(&params.emit_code().to_string());

    assert!(source.contains("::bevy::ui::GridPlacement::start_span"));
    assert!(source.contains("2i16"));
    assert!(source.contains("3u16"));
}

#[test]
fn node_and_text_params_emit_current_bevy_ui_components() {
    let node = BevyUiNodeLiteralParams {
        node: BevyUiNodeParams {
            width_expr: Some("Val::Percent(100.0)".into()),
            height_expr: Some("Val::Px(48.0)".into()),
            justify_content_expr: Some("JustifyContent::Center".into()),
            align_items_expr: Some("AlignItems::Center".into()),
            ..Default::default()
        },
    };
    let text = BevyUiTextParams {
        value: "Play".into(),
        font_handle_expr: Some("asset_server.load(\"fonts/FiraSans-Bold.ttf\")".into()),
        font_size: Some(28.0),
        color_expr: Some("Color::WHITE".into()),
        justify_expr: Some("Justify::Center".into()),
        linebreak_expr: Some("LineBreak::NoWrap".into()),
    };

    let node_source = normalize(&node.emit_code().to_string());
    let text_source = normalize(&text.emit_code().to_string());

    assert!(node_source.contains("::bevy::ui::Node{"));
    assert!(node_source.contains("width:Val::Percent(100.0)"));
    assert!(node_source.contains("height:Val::Px(48.0)"));
    assert!(node_source.contains("justify_content:JustifyContent::Center"));
    assert!(text_source.contains("::bevy::ui::widget::Text::new(\"Play\")"));
    assert!(text_source.contains("with_font(asset_server.load(\"fonts/FiraSans-Bold.ttf\"))"));
    assert!(text_source.contains("with_font_size(28"));
    assert!(text_source.contains("::bevy::text::TextColor((Color::WHITE).into())"));
    assert!(text_source.contains("with_justify(Justify::Center)"));
    assert!(text_source.contains("with_linebreak(LineBreak::NoWrap)"));
}

#[test]
fn image_and_button_bundle_emit_current_widget_model() {
    let image = BevyUiImageParams {
        image_expr: Some("asset_server.load(\"ui/panel.png\")".into()),
        color_expr: Some("Color::WHITE".into()),
        flip_x: true,
        flip_y: false,
    };
    let button = BevyUiButtonBundleParams {
        commands_var: "commands".into(),
        label: "Start".into(),
        node: BevyUiNodeParams {
            width_expr: Some("Val::Px(220.0)".into()),
            height_expr: Some("Val::Px(64.0)".into()),
            justify_content_expr: Some("JustifyContent::Center".into()),
            align_items_expr: Some("AlignItems::Center".into()),
            ..Default::default()
        },
        background_color_expr: Some("Color::srgb(0.2, 0.3, 0.8)".into()),
        font_handle_expr: None,
        font_size: Some(24.0),
        text_color_expr: Some("Color::WHITE".into()),
    };

    let image_source = normalize(&image.emit_code().to_string());
    let button_source = normalize(&button.emit_code().to_string());

    assert!(image_source.contains("::bevy::ui::widget::ImageNode::new"));
    assert!(image_source.contains("with_color((Color::WHITE).into())"));
    assert!(image_source.contains("with_flip_x()"));
    assert!(button_source.contains("::bevy::ui::widget::Button"));
    assert!(
        button_source.contains("::bevy::ui::BackgroundColor((Color::srgb(0.2,0.3,0.8)).into())")
    );
    assert!(button_source.contains("parent.spawn(("));
    assert!(button_source.contains("::bevy::ui::widget::Text::new(\"Start\")"));
}

#[test]
fn dispatch_emit_flex_and_grid_container_use_registered_emit_entries() {
    let flex = dispatch_emit_from(
        "flex_container",
        "elicit_bevy",
        serde_json::json!({
            "commands_var": "commands",
            "node": {
                "flex_direction_expr": "FlexDirection::Column",
                "row_gap_expr": "Val::Px(12.0)"
            },
            "children": ["make_button()", "make_label()"]
        }),
    )
    .unwrap();
    let grid = BevyUiGridContainerParams {
        commands_var: "commands".into(),
        node: BevyUiNodeParams {
            grid_template_columns_exprs: vec![
                "RepeatedGridTrack::flex(1, 1.0)".into(),
                "RepeatedGridTrack::px(1, 240.0)".into(),
            ],
            column_gap_expr: Some("Val::Px(8.0)".into()),
            ..Default::default()
        },
        children: vec!["make_card()".into(), "make_sidebar()".into()],
    };

    let flex_source = normalize(&flex.emit_code().to_string());
    let grid_source = normalize(&grid.emit_code().to_string());

    assert!(flex_source.contains("display:::bevy::ui::Display::Flex"));
    assert!(flex_source.contains("flex_direction:FlexDirection::Column"));
    assert!(flex_source.contains("row_gap:Val::Px(12.0)"));
    assert!(flex_source.contains("parent.spawn(make_button());"));
    assert!(grid_source.contains("display:::bevy::ui::Display::Grid"));
    assert!(grid_source.contains(
        "grid_template_columns:vec![RepeatedGridTrack::flex(1,1.0),RepeatedGridTrack::px(1,240.0)]"
    ));
    assert!(grid_source.contains("column_gap:Val::Px(8.0)"));
    assert!(grid_source.contains("parent.spawn(make_sidebar());"));
}
