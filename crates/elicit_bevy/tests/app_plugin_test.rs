//! Integration tests for `BevyAppPlugin`.

use elicit_bevy::BevyAppPlugin;
use elicitation::ElicitPlugin;

fn result_text(result: &rmcp::model::CallToolResult) -> String {
    result
        .content
        .iter()
        .filter_map(|content| content.as_text().map(|text| text.text.clone()))
        .collect()
}

fn normalize(source: &str) -> String {
    source.chars().filter(|c| !c.is_whitespace()).collect()
}

#[test]
fn app_plugin_lists_expected_tools() {
    let plugin = BevyAppPlugin::new();
    let mut names = plugin
        .list_tools()
        .into_iter()
        .map(|tool| tool.name.to_string())
        .collect::<Vec<_>>();
    names.sort();

    assert_eq!(
        names,
        vec![
            "add_default_plugins",
            "add_plugin",
            "add_schedule",
            "describe",
            "emit",
            "new",
            "plugin_group",
            "plugin_struct",
            "set_runner",
            "state_machine",
        ]
    );
}

#[tokio::test]
async fn app_descriptor_roundtrip_emits_main_rs() {
    let plugin = BevyAppPlugin::new();

    let created = plugin
        .invoke_tool("new", serde_json::json!({ "name": "SpaceGame" }))
        .await
        .unwrap();
    let created_json: serde_json::Value = serde_json::from_str(&result_text(&created)).unwrap();
    let config_id = created_json["config_id"].as_str().unwrap().to_string();

    plugin
        .invoke_tool(
            "add_default_plugins",
            serde_json::json!({
                "config_id": config_id,
                "window_plugin_expr": "WindowPlugin { primary_window: Some(Window::default()), ..default() }"
            }),
        )
        .await
        .unwrap();

    plugin
        .invoke_tool(
            "add_plugin",
            serde_json::json!({
                "config_id": config_id,
                "plugin_expr": "MyGameplayPlugin"
            }),
        )
        .await
        .unwrap();

    plugin
        .invoke_tool(
            "add_schedule",
            serde_json::json!({
                "config_id": config_id,
                "label_expr": "MySchedule",
                "after": "Update"
            }),
        )
        .await
        .unwrap();

    plugin
        .invoke_tool(
            "set_runner",
            serde_json::json!({
                "config_id": config_id,
                "runner_expr": "|mut app: App| { app.update(); AppExit::Success }"
            }),
        )
        .await
        .unwrap();

    let described = plugin
        .invoke_tool("describe", serde_json::json!({ "config_id": config_id }))
        .await
        .unwrap();
    let desc_json: serde_json::Value = serde_json::from_str(&result_text(&described)).unwrap();
    assert_eq!(desc_json["name"], "SpaceGame");
    assert_eq!(desc_json["plugins"][0], "MyGameplayPlugin");

    let emitted = plugin
        .invoke_tool("emit", serde_json::json!({ "config_id": config_id }))
        .await
        .unwrap();
    let source = normalize(&result_text(&emitted));

    assert!(source.contains("letmutapp=::bevy::app::App::new();"));
    assert!(source.contains("app.add_plugins(::bevy::DefaultPlugins.set(WindowPlugin{primary_window:Some(Window::default()),..default()}));"));
    assert!(source.contains("app.add_plugins(MyGameplayPlugin);"));
    assert!(source.contains("app.add_schedule(::bevy::ecs::schedule::Schedule::new(MySchedule));"));
    assert!(source.contains(
        "resource_mut::<::bevy::app::MainScheduleOrder>().insert_after(Update,MySchedule);"
    ));
    assert!(source.contains("app.set_runner(|mutapp:App|{app.update();AppExit::Success});"));
    assert!(source.contains("app.run();"));
}

#[tokio::test]
async fn plugin_struct_tool_emits_plugin_skeleton() {
    let plugin = BevyAppPlugin::new();
    let result = plugin
        .invoke_tool(
            "plugin_struct",
            serde_json::json!({
                "name": "GameplayPlugin",
                "body": "app.add_systems(Update, gameplay_system);"
            }),
        )
        .await
        .unwrap();
    let source = normalize(&result_text(&result));

    assert!(source.contains("pubstructGameplayPlugin;"));
    assert!(source.contains("impl::bevy::app::PluginforGameplayPlugin"));
    assert!(source.contains("app.add_systems(Update,gameplay_system);"));
}

#[tokio::test]
async fn plugin_group_tool_emits_group_builder() {
    let plugin = BevyAppPlugin::new();
    let result = plugin
        .invoke_tool(
            "plugin_group",
            serde_json::json!({
                "name": "GamePlugins",
                "plugins": ["GameplayPlugin", "UiPlugin"]
            }),
        )
        .await
        .unwrap();
    let source = normalize(&result_text(&result));

    assert!(source.contains("pubstructGamePlugins;"));
    assert!(source.contains("PluginGroupBuilder::start::<Self>()"));
    assert!(source.contains(".add(GameplayPlugin)"));
    assert!(source.contains(".add(UiPlugin)"));
}

#[tokio::test]
async fn state_machine_tool_emits_states_enum_and_hooks() {
    let plugin = BevyAppPlugin::new();
    let result = plugin
        .invoke_tool(
            "state_machine",
            serde_json::json!({
                "app_var": "app",
                "enum_name": "AppState",
                "variants": ["Loading", "InGame", "Paused"],
                "initial_state": "Loading",
                "on_enter": [
                    { "state": "Loading", "system_expr": "setup_loading" }
                ],
                "on_exit": [
                    { "state": "Loading", "system_expr": "teardown_loading" }
                ]
            }),
        )
        .await
        .unwrap();
    let source = normalize(&result_text(&result));

    assert!(
        source.contains("#[derive(::bevy::prelude::States,Debug,Clone,PartialEq,Eq,Hash,Default)]")
    );
    assert!(source.contains("#[default]"));
    assert!(source.contains("pubenumAppState{"));
    assert!(source.contains("app.init_state::<AppState>();"));
    assert!(
        source.contains(
            "app.add_systems(::bevy::prelude::OnEnter(AppState::Loading),setup_loading);"
        )
    );
    assert!(
        source.contains(
            "app.add_systems(::bevy::prelude::OnExit(AppState::Loading),teardown_loading);"
        )
    );
}
