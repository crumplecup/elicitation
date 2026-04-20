//! Integration tests for `BevyQueryPlugin`.

use elicit_bevy::{
    BevyQueryFilterKind, BevyQueryItemAccess, BevyQueryItemSpec, BevyQueryPlugin,
    DefineComponentQueryParams, DefineHandleParams, DefineResourceParams, DefineTimeParams,
    FilterParams,
};
use elicitation::ElicitPlugin;
use elicitation::emit_code::{EmitCode, dispatch_emit_from};

fn normalize(source: &str) -> String {
    source.chars().filter(|c| !c.is_whitespace()).collect()
}

#[test]
fn query_plugin_lists_expected_tools() {
    let plugin = BevyQueryPlugin::new();
    let mut names = plugin
        .list_tools()
        .into_iter()
        .map(|tool| tool.name.to_string())
        .collect::<Vec<_>>();
    names.sort();

    assert_eq!(
        names,
        vec![
            "define_component_query",
            "define_event_reader",
            "define_event_writer",
            "define_handle",
            "define_local",
            "define_resource",
            "define_time",
            "filter",
            "system_signature",
        ]
    );
}

#[test]
fn filter_params_emit_changed_filter_type() {
    let params = FilterParams {
        kind: BevyQueryFilterKind::Changed,
        type_name: "Transform".into(),
    };
    let source = normalize(&params.emit_code().to_string());

    assert!(source.contains("::bevy::ecs::query::Changed<Transform>"));
}

#[test]
fn define_component_query_params_emit_mixed_query_signature() {
    let params = DefineComponentQueryParams {
        binding: "query".into(),
        mutable_binding: true,
        items: vec![
            BevyQueryItemSpec {
                ty: "Entity".into(),
                access: BevyQueryItemAccess::Value,
            },
            BevyQueryItemSpec {
                ty: "Transform".into(),
                access: BevyQueryItemAccess::Shared,
            },
            BevyQueryItemSpec {
                ty: "Velocity".into(),
                access: BevyQueryItemAccess::Mutable,
            },
        ],
        filters: vec!["With<Player>".into(), "Changed<Transform>".into()],
    };
    let source = normalize(&params.emit_code().to_string());

    assert!(source.contains("mutquery:::bevy::ecs::system::Query<"));
    assert!(source.contains("(Entity,&Transform,&mutVelocity)"));
    assert!(source.contains("(With<Player>,Changed<Transform>)"));
}

#[test]
fn define_resource_and_time_emit_mutable_system_params() {
    let resource = DefineResourceParams {
        binding: "config".into(),
        resource_type: "GameConfig".into(),
        mutable: true,
    };
    let time = DefineTimeParams {
        binding: "fixed_time".into(),
        time_generic: Some("Fixed".into()),
        mutable: true,
    };

    let resource_source = normalize(&resource.emit_code().to_string());
    let time_source = normalize(&time.emit_code().to_string());

    assert!(resource_source.contains("mutconfig:::bevy::ecs::system::ResMut<GameConfig>"));
    assert!(
        time_source
            .contains("mutfixed_time:::bevy::ecs::system::ResMut<::bevy::prelude::Time<Fixed>>")
    );
}

#[test]
fn define_handle_emits_field_declaration() {
    let params = DefineHandleParams {
        visibility: Some("pub".into()),
        binding: "texture".into(),
        asset_type: "Image".into(),
    };
    let source = normalize(&params.emit_code().to_string());

    assert!(source.contains("pubtexture:::bevy::asset::Handle<Image>"));
}

#[test]
fn dispatch_emit_system_signature_uses_registered_emit_entry() {
    let emitter = dispatch_emit_from(
        "system_signature",
        "elicit_bevy",
        serde_json::json!({
            "visibility": "pub",
            "function_name": "move_player",
            "params": [
                "mut query: Query<&mut Transform, With<Player>>",
                "time: Res<Time>",
                "mut moved: EventWriter<PlayerMoved>"
            ],
            "body": "let _ = (&mut query, &time, &mut moved);"
        }),
    )
    .unwrap();
    let source = normalize(&emitter.emit_code().to_string());

    assert!(source.contains("pubfnmove_player("));
    assert!(source.contains("mutquery:Query<&mutTransform,With<Player>>"));
    assert!(source.contains("time:Res<Time>"));
    assert!(source.contains("mutmoved:EventWriter<PlayerMoved>"));
    assert!(source.contains("let_=(&mutquery,&time,&mutmoved);"));
}
