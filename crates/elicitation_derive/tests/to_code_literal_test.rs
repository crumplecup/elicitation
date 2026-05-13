use elicitation::emit_code::ToCodeLiteral;

mod demo {
    pub enum UpstreamEnum {
        Alpha,
        Beta,
    }

    pub struct Marker;
    pub struct AmbientLight;
    pub struct PerspectiveProjection;
    pub struct Viewport;
    pub struct Mesh3d;
    pub struct MaterialWrapper;
    pub struct Position2d;
    pub struct AlphaMode;
}

fn uvec2_tokens(value: &[u32; 2]) -> elicitation::proc_macro2::TokenStream {
    let [x, y] = *value;
    elicitation::quote::quote! { ::bevy::math::UVec2::new(#x, #y) }
}

#[derive(Debug, Clone, Copy, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "crate::demo::UpstreamEnum")]
enum UpstreamEnumProxy {
    Alpha,
    Beta,
}

#[derive(Debug, Clone, elicitation::ToCodeLiteral)]
#[to_code_literal(transparent)]
struct TransparentNamed {
    inner: UpstreamEnumProxy,
}

#[derive(Debug, Clone, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "crate::demo::Marker")]
struct MarkerProxy;

#[derive(Debug, Clone, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "crate::demo::AmbientLight", default_update)]
struct AmbientLightProxy {
    #[to_code_literal(rename = "color", expr, optional)]
    color_expr: Option<String>,
    #[to_code_literal(optional)]
    intensity: Option<f32>,
}

#[derive(Debug, Clone, elicitation::ToCodeLiteral)]
#[to_code_literal(
    path = "crate::demo::PerspectiveProjection",
    update = "crate::demo::PerspectiveProjection::default()",
    default_expr = "crate::demo::PerspectiveProjection::default()"
)]
struct PerspectiveProjectionProxy {
    #[to_code_literal(optional)]
    fov: Option<f32>,
    #[to_code_literal(rename = "near_clip_plane", expr, optional)]
    near_clip_plane_expr: Option<String>,
}

#[derive(Debug, Clone, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "crate::demo::Viewport")]
struct ViewportProxy {
    #[to_code_literal(rename = "physical_position", to_tokens = "crate::uvec2_tokens")]
    physical_position: [u32; 2],
}

#[derive(Debug, Clone, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "crate::demo::Mesh3d", transparent)]
struct MeshExprProxy {
    #[to_code_literal(expr)]
    mesh_expr: String,
}

#[derive(Debug, Clone, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "crate::demo::Position2d", transparent)]
struct PositionProxy(#[to_code_literal(to_tokens = "crate::uvec2_tokens")] [u32; 2]);

#[derive(Debug, Clone, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "crate::demo::Mesh3d", tuple)]
struct NamedTupleExprProxy {
    #[to_code_literal(expr)]
    mesh_expr: String,
}

#[derive(Debug, Clone, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "crate::demo::MaterialWrapper", tuple)]
struct NamedTupleTokenHelperProxy {
    #[to_code_literal(to_tokens = "crate::uvec2_tokens")]
    coords: [u32; 2],
}

#[derive(Debug, Clone, elicitation::ToCodeLiteral)]
#[to_code_literal(raw_tuple)]
struct RawTupleProxy {
    first: UpstreamEnumProxy,
    second: MarkerProxy,
}

#[derive(Debug, Clone, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "crate::demo::Mesh3d")]
enum EnumTupleExprProxy {
    Mesh(#[to_code_literal(expr)] String),
}

#[derive(Debug, Clone, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "crate::demo::MaterialWrapper")]
enum EnumTupleTokenHelperProxy {
    Value(#[to_code_literal(to_tokens = "crate::uvec2_tokens")] [u32; 2]),
}

#[derive(Debug, Clone, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "crate::demo::AlphaMode")]
enum EnumStructTupleExprProxy {
    #[to_code_literal(tuple)]
    Mask {
        #[to_code_literal(expr)]
        threshold_expr: String,
    },
}

#[derive(Debug, Clone, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "crate::demo::MaterialWrapper")]
enum EnumStructTupleTokenHelperProxy {
    #[to_code_literal(tuple)]
    Value {
        #[to_code_literal(to_tokens = "crate::uvec2_tokens")]
        coords: [u32; 2],
    },
}

#[derive(Debug, Clone, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "crate::demo::Viewport")]
enum EnumStructFieldTokenHelperProxy {
    Positioned {
        #[to_code_literal(to_tokens = "crate::uvec2_tokens")]
        physical_position: [u32; 2],
    },
}

#[test]
fn derive_supports_custom_type_paths() {
    let _ = demo::UpstreamEnum::Alpha;
    let _ = demo::UpstreamEnum::Beta;
    let tokens = UpstreamEnumProxy::Beta.to_code_literal().to_string();
    assert_eq!(tokens, "crate :: demo :: UpstreamEnum :: Beta");
}

#[test]
fn derive_supports_transparent_wrappers() {
    let tokens = TransparentNamed {
        inner: UpstreamEnumProxy::Alpha,
    }
    .to_code_literal()
    .to_string();
    assert_eq!(tokens, "crate :: demo :: UpstreamEnum :: Alpha");
}

#[test]
fn derive_supports_unit_struct_path_overrides() {
    let _ = demo::Marker;
    let tokens = MarkerProxy.to_code_literal().to_string();
    assert_eq!(tokens, "crate :: demo :: Marker");
}

#[test]
fn derive_supports_optional_expr_fields_with_default_update() {
    let _ = demo::AmbientLight;
    let tokens = AmbientLightProxy {
        color_expr: Some("Color::WHITE".into()),
        intensity: Some(1.5),
    }
    .to_code_literal()
    .to_string();
    assert_eq!(
        tokens,
        "crate :: demo :: AmbientLight { color : Color :: WHITE , intensity : 1.5f32 , .. :: std :: default :: Default :: default () }"
    );
}

#[test]
fn derive_supports_default_expr_when_all_optional_fields_are_empty() {
    let _ = demo::PerspectiveProjection;
    let tokens = PerspectiveProjectionProxy {
        fov: None,
        near_clip_plane_expr: None,
    }
    .to_code_literal()
    .to_string();
    assert_eq!(
        tokens,
        "crate :: demo :: PerspectiveProjection :: default ()"
    );
}

#[test]
fn derive_supports_field_token_helpers() {
    let _ = demo::Viewport;
    let tokens = ViewportProxy {
        physical_position: [32, 64],
    }
    .to_code_literal()
    .to_string();
    assert_eq!(
        tokens,
        "crate :: demo :: Viewport { physical_position : :: bevy :: math :: UVec2 :: new (32u32 , 64u32) , }"
    );
}

#[test]
fn derive_supports_transparent_expr_wrappers() {
    let _ = demo::Mesh3d;
    let tokens = MeshExprProxy {
        mesh_expr: "meshes.add(Cuboid::default())".into(),
    }
    .to_code_literal()
    .to_string();
    assert_eq!(tokens, "meshes . add (Cuboid :: default ())");
}

#[test]
fn derive_supports_transparent_token_helpers() {
    let _ = demo::Position2d;
    let tokens = PositionProxy([16, 24]).to_code_literal().to_string();
    assert_eq!(tokens, ":: bevy :: math :: UVec2 :: new (16u32 , 24u32)");
}

#[test]
fn derive_supports_named_tuple_constructor_expr_fields() {
    let _ = demo::Mesh3d;
    let tokens = NamedTupleExprProxy {
        mesh_expr: "meshes.add(Cuboid::default())".into(),
    }
    .to_code_literal()
    .to_string();
    assert_eq!(
        tokens,
        "crate :: demo :: Mesh3d (meshes . add (Cuboid :: default ()))"
    );
}

#[test]
fn derive_supports_named_tuple_constructor_token_helpers() {
    let _ = demo::MaterialWrapper;
    let tokens = NamedTupleTokenHelperProxy { coords: [8, 13] }
        .to_code_literal()
        .to_string();
    assert_eq!(
        tokens,
        "crate :: demo :: MaterialWrapper (:: bevy :: math :: UVec2 :: new (8u32 , 13u32))"
    );
}

#[test]
fn derive_supports_named_raw_tuple_literals() {
    let _ = demo::Marker;
    let tokens = RawTupleProxy {
        first: UpstreamEnumProxy::Alpha,
        second: MarkerProxy,
    }
    .to_code_literal()
    .to_string();
    assert_eq!(
        tokens,
        "(crate :: demo :: UpstreamEnum :: Alpha , crate :: demo :: Marker)"
    );
}

#[test]
fn derive_supports_enum_tuple_expr_fields() {
    let _ = demo::Mesh3d;
    let tokens = EnumTupleExprProxy::Mesh("meshes.add(Cuboid::default())".into())
        .to_code_literal()
        .to_string();
    assert_eq!(
        tokens,
        "crate :: demo :: Mesh3d :: Mesh (meshes . add (Cuboid :: default ()))"
    );
}

#[test]
fn derive_supports_enum_tuple_token_helpers() {
    let _ = demo::MaterialWrapper;
    let tokens = EnumTupleTokenHelperProxy::Value([3, 21])
        .to_code_literal()
        .to_string();
    assert_eq!(
        tokens,
        "crate :: demo :: MaterialWrapper :: Value (:: bevy :: math :: UVec2 :: new (3u32 , 21u32))"
    );
}

#[test]
fn derive_supports_enum_struct_variants_emitting_tuple_syntax() {
    let _ = demo::AlphaMode;
    let tokens = EnumStructTupleExprProxy::Mask {
        threshold_expr: "0.33".into(),
    }
    .to_code_literal()
    .to_string();
    assert_eq!(tokens, "crate :: demo :: AlphaMode :: Mask (0.33)");
}

#[test]
fn derive_supports_enum_struct_variant_token_helpers_with_tuple_syntax() {
    let _ = demo::MaterialWrapper;
    let tokens = EnumStructTupleTokenHelperProxy::Value { coords: [5, 8] }
        .to_code_literal()
        .to_string();
    assert_eq!(
        tokens,
        "crate :: demo :: MaterialWrapper :: Value (:: bevy :: math :: UVec2 :: new (5u32 , 8u32))"
    );
}

#[test]
fn derive_supports_enum_struct_variant_field_token_helpers() {
    let _ = demo::Viewport;
    let tokens = EnumStructFieldTokenHelperProxy::Positioned {
        physical_position: [11, 17],
    }
    .to_code_literal()
    .to_string();
    assert_eq!(
        tokens,
        "crate :: demo :: Viewport :: Positioned { physical_position : :: bevy :: math :: UVec2 :: new (11u32 , 17u32) }"
    );
}
