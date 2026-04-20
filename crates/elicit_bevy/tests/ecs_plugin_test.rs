//! Integration tests for `BevyEcsPlugin`.

use elicit_bevy::{
    AddPluginsParams, AddSystemsParams, BevyEcsPlugin, ChainParams, DespawnParams, PipeParams,
    QueryForParams, QueryItemSpec, WithChildrenParams,
};
use elicitation::ElicitPlugin;
use elicitation::emit_code::{EmitCode, dispatch_emit_from};

fn normalize(source: &str) -> String {
    source.chars().filter(|c| !c.is_whitespace()).collect()
}

#[test]
fn ecs_plugin_lists_expected_tools() {
    let plugin = BevyEcsPlugin::new();
    let mut names = plugin
        .list_tools()
        .into_iter()
        .map(|tool| tool.name.to_string())
        .collect::<Vec<_>>();
    names.sort();

    assert_eq!(
        names,
        vec![
            "add_event",
            "add_plugins",
            "add_systems",
            "chain",
            "despawn",
            "in_set",
            "init_resource",
            "insert_component",
            "insert_resource",
            "observer",
            "pipe",
            "query_for",
            "register_type",
            "remove_component",
            "run_criteria",
            "spawn_bundle",
            "spawn_entity",
            "trigger",
            "with_children",
        ]
    );
}

#[test]
fn add_systems_params_emit_configured_systems() {
    let params = AddSystemsParams {
        app_var: "app".into(),
        schedule: "Update".into(),
        systems: vec!["move_player".into(), "animate_player".into()],
        chain: true,
        run_if: Some("in_state(AppState::InGame)".into()),
        in_set: Some("GameplaySystems::Movement".into()),
        before: vec!["GameplaySystems::Cleanup".into()],
        after: vec!["GameplaySystems::Input".into()],
    };
    let source = normalize(&params.emit_code().to_string());

    assert!(source.contains("app.add_systems"));
    assert!(source.contains("(move_player,animate_player).chain()"));
    assert!(source.contains(".run_if(in_state(AppState::InGame))"));
    assert!(source.contains(".in_set(GameplaySystems::Movement)"));
    assert!(source.contains(".before(GameplaySystems::Cleanup)"));
    assert!(source.contains(".after(GameplaySystems::Input)"));
}

#[test]
fn add_plugins_params_emit_tuple_for_multiple_plugins() {
    let params = AddPluginsParams {
        app_var: "app".into(),
        plugins: vec![
            "DefaultPlugins".into(),
            "MyGameplayPlugin".into(),
            "MyUiPlugin".into(),
        ],
    };
    let source = normalize(&params.emit_code().to_string());

    assert!(source.contains("app.add_plugins((DefaultPlugins,MyGameplayPlugin,MyUiPlugin))"));
}

#[test]
fn query_for_params_emit_mutable_and_filtered_query() {
    let params = QueryForParams {
        items: vec![
            QueryItemSpec {
                ty: "Transform".into(),
                mutable: false,
            },
            QueryItemSpec {
                ty: "Velocity".into(),
                mutable: true,
            },
        ],
        filters: vec!["With<Player>".into(), "Without<Frozen>".into()],
    };
    let source = normalize(&params.emit_code().to_string());

    assert!(source.contains("Query<(&Transform,&mutVelocity),(With<Player>,Without<Frozen>)>"));
}

#[test]
fn despawn_params_emit_recursive_children_cleanup_for_bevy_018() {
    let params = DespawnParams {
        commands_var: "commands".into(),
        entity_expr: "entity".into(),
        recursive: true,
    };
    let source = normalize(&params.emit_code().to_string());

    assert!(source.contains("commands.entity(entity).despawn_children().despawn()"));
}

#[test]
fn with_children_params_emit_parent_and_child_spawns() {
    let params = WithChildrenParams {
        commands_var: "commands".into(),
        parent_expr: "SpatialBundle::default()".into(),
        children: vec!["Name::new(\"ChildA\")".into(), "Sprite::default()".into()],
    };
    let source = normalize(&params.emit_code().to_string());

    assert!(source.contains("commands.spawn(SpatialBundle::default()).with_children"));
    assert!(source.contains("parent.spawn(Name::new(\"ChildA\"));"));
    assert!(source.contains("parent.spawn(Sprite::default());"));
}

#[test]
fn dispatch_emit_pipe_uses_registered_emit_entry() {
    let emitter = dispatch_emit_from(
        "pipe",
        "elicit_bevy",
        serde_json::json!({
            "left": "gather_input",
            "right": "apply_velocity",
        }),
    )
    .unwrap();
    let source = normalize(&emitter.emit_code().to_string());

    assert!(source.contains("(gather_input).pipe(apply_velocity)"));
}

#[test]
fn params_emit_code_support_chain_and_pipe_fragments() {
    let chain = ChainParams {
        systems: vec!["first".into(), "second".into(), "third".into()],
    };
    let pipe = PipeParams {
        left: "read_input".into(),
        right: "produce_command".into(),
    };

    let chain_source = normalize(&chain.emit_code().to_string());
    let pipe_source = normalize(&pipe.emit_code().to_string());

    assert!(chain_source.contains("(first,second,third).chain()"));
    assert!(pipe_source.contains("(read_input).pipe(produce_command)"));
}
