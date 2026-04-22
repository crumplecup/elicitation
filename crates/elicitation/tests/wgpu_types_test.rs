//! Tests for wgpu 29 third-party support in `elicitation`.

#![cfg(feature = "wgpu-types")]

use elicitation::{
    ElicitComplete, ElicitIntrospect, Elicitation, ElicitationPattern, Select,
    WgpuAddressMode, WgpuBackend, WgpuBlendFactor, WgpuBlendOperation, WgpuBufferUsages, WgpuColor,
    WgpuColorWrites, WgpuCompareFunctionSelect, WgpuCompositeAlphaMode, WgpuExtent3d, WgpuFace,
    WgpuFilterMode, WgpuFrontFace, WgpuIndexFormat, WgpuOrigin3d, WgpuPolygonMode,
    WgpuPowerPreference, WgpuPresentMode, WgpuPrimitiveTopology, WgpuSamplerBorderColor,
    WgpuShaderStages, WgpuStencilOperation, WgpuTextureDimension, WgpuTextureFormat,
    WgpuTextureUsages, WgpuTextureViewDimension, WgpuVertexFormat, WgpuVertexStepMode,
    lookup_type_spec,
};

#[track_caller]
fn assert_proofs_non_empty<T: Elicitation>(label: &str) {
    assert!(!T::kani_proof().is_empty(), "{label}: empty kani proof");
    assert!(!T::verus_proof().is_empty(), "{label}: empty verus proof");
    assert!(
        !T::creusot_proof().is_empty(),
        "{label}: empty creusot proof"
    );
}

fn assert_elicit_complete<T: ElicitComplete>() {}

// ── Wrapper type compile-time assertions ─────────────────────────────────────

#[test]
fn wgpu_wrappers_are_elicit_complete() {
    assert_elicit_complete::<WgpuTextureFormat>();
    assert_elicit_complete::<WgpuPresentMode>();
    assert_elicit_complete::<WgpuPowerPreference>();
    assert_elicit_complete::<WgpuTextureDimension>();
    assert_elicit_complete::<WgpuTextureViewDimension>();
    assert_elicit_complete::<WgpuPrimitiveTopology>();
    assert_elicit_complete::<WgpuFrontFace>();
    assert_elicit_complete::<WgpuFace>();
    assert_elicit_complete::<WgpuPolygonMode>();
    assert_elicit_complete::<WgpuCompareFunctionSelect>();
    assert_elicit_complete::<WgpuBlendFactor>();
    assert_elicit_complete::<WgpuBlendOperation>();
    assert_elicit_complete::<WgpuIndexFormat>();
    assert_elicit_complete::<WgpuStencilOperation>();
    assert_elicit_complete::<WgpuVertexStepMode>();
    assert_elicit_complete::<WgpuVertexFormat>();
    assert_elicit_complete::<WgpuAddressMode>();
    assert_elicit_complete::<WgpuFilterMode>();
    assert_elicit_complete::<WgpuSamplerBorderColor>();
    assert_elicit_complete::<WgpuCompositeAlphaMode>();
    assert_elicit_complete::<WgpuBackend>();
    assert_elicit_complete::<WgpuBufferUsages>();
    assert_elicit_complete::<WgpuTextureUsages>();
    assert_elicit_complete::<WgpuShaderStages>();
    assert_elicit_complete::<WgpuColorWrites>();
    assert_elicit_complete::<WgpuExtent3d>();
    assert_elicit_complete::<WgpuColor>();
    assert_elicit_complete::<WgpuOrigin3d>();
}

// ── Introspection patterns ────────────────────────────────────────────────────

mod introspect {
    use super::*;

    #[test]
    fn enum_wrappers_are_select_pattern() {
        assert_eq!(WgpuTextureFormat::pattern(), ElicitationPattern::Select);
        assert_eq!(WgpuPrimitiveTopology::pattern(), ElicitationPattern::Select);
        assert_eq!(WgpuBlendFactor::pattern(), ElicitationPattern::Select);
        assert_eq!(WgpuBufferUsages::pattern(), ElicitationPattern::Select);
        assert_eq!(WgpuShaderStages::pattern(), ElicitationPattern::Select);
    }

    #[test]
    fn struct_wrappers_are_survey_pattern() {
        assert_eq!(WgpuExtent3d::pattern(), ElicitationPattern::Survey);
        assert_eq!(WgpuColor::pattern(), ElicitationPattern::Survey);
        assert_eq!(WgpuOrigin3d::pattern(), ElicitationPattern::Survey);
    }

    #[test]
    fn texture_format_has_many_options() {
        let labels = WgpuTextureFormat::labels();
        assert!(
            labels.len() >= 20,
            "expected many texture formats, got {}",
            labels.len()
        );
        assert!(
            labels.iter().any(|l| l == "rgba8unorm"),
            "expected rgba8unorm in labels"
        );
        assert!(
            labels.iter().any(|l| l == "depth24plus"),
            "expected depth24plus in labels"
        );
    }

    #[test]
    fn vertex_format_has_many_options() {
        let labels = WgpuVertexFormat::labels();
        assert!(labels.len() >= 30, "expected many vertex formats");
    }

    #[test]
    fn present_mode_labels_are_camel_case() {
        let labels = WgpuPresentMode::labels();
        assert!(
            labels.iter().any(|l| l == "Fifo"),
            "expected Fifo in present mode labels"
        );
        assert!(
            labels.iter().any(|l| l == "AutoVsync"),
            "expected AutoVsync in labels"
        );
    }

    #[test]
    fn power_preference_labels() {
        let labels = WgpuPowerPreference::labels();
        assert!(labels.contains(&"none".to_string()), "expected none");
        assert!(
            labels.contains(&"high-performance".to_string()),
            "expected high-performance"
        );
    }

    #[test]
    fn bitflag_buffer_usages_labels() {
        let labels = WgpuBufferUsages::labels();
        assert!(
            labels.iter().any(|l| l == "UNIFORM"),
            "expected UNIFORM in BufferUsages labels"
        );
        assert!(
            labels.iter().any(|l| l == "VERTEX"),
            "expected VERTEX in BufferUsages labels"
        );
    }

    #[test]
    fn bitflag_shader_stages_labels() {
        let labels = WgpuShaderStages::labels();
        assert!(
            labels.iter().any(|l| l == "VERTEX"),
            "expected VERTEX in ShaderStages"
        );
        assert!(
            labels.iter().any(|l| l == "COMPUTE"),
            "expected COMPUTE in ShaderStages"
        );
    }
}

// ── Serde round-trips ─────────────────────────────────────────────────────────

mod serde_roundtrip {
    use super::*;

    #[test]
    fn texture_format_roundtrip() {
        let f = WgpuTextureFormat::from_label("rgba8unorm").expect("rgba8unorm");
        assert_eq!(
            serde_json::to_string(&*f).unwrap().trim_matches('"'),
            "rgba8unorm"
        );
    }

    #[test]
    fn texture_format_srgb_roundtrip() {
        let f = WgpuTextureFormat::from_label("bgra8unorm-srgb").expect("bgra8unorm-srgb");
        assert_eq!(
            serde_json::to_string(&*f).unwrap().trim_matches('"'),
            "bgra8unorm-srgb"
        );
    }

    #[test]
    fn primitive_topology_roundtrip() {
        let t = WgpuPrimitiveTopology::from_label("triangle-list").expect("triangle-list");
        assert_eq!(
            serde_json::to_string(&*t).unwrap().trim_matches('"'),
            "triangle-list"
        );
    }

    #[test]
    fn power_preference_roundtrip() {
        let p = WgpuPowerPreference::from_label("high-performance").expect("high-performance");
        assert_eq!(
            serde_json::to_string(&*p).unwrap().trim_matches('"'),
            "high-performance"
        );
    }

    #[test]
    fn buffer_usages_roundtrip() {
        let u = WgpuBufferUsages::from_label("UNIFORM").expect("UNIFORM");
        assert_eq!(
            serde_json::to_string(&*u).unwrap().trim_matches('"'),
            "UNIFORM"
        );
    }

    #[test]
    fn color_serde_roundtrip() {
        let c = WgpuColor {
            r: 1.0,
            g: 0.5,
            b: 0.0,
            a: 1.0,
        };
        let json = serde_json::to_string(&c).unwrap();
        let back: WgpuColor = serde_json::from_str(&json).unwrap();
        assert!((back.r - 1.0).abs() < 1e-9);
        assert!((back.g - 0.5).abs() < 1e-9);
    }

    #[test]
    fn extent3d_serde_roundtrip() {
        let e = WgpuExtent3d {
            width: 1024,
            height: 768,
            depth_or_array_layers: 1,
        };
        let json = serde_json::to_string(&e).unwrap();
        // wgpu serializes as camelCase
        assert!(
            json.contains("depthOrArrayLayers"),
            "expected camelCase key"
        );
        let back: WgpuExtent3d = serde_json::from_str(&json).unwrap();
        assert_eq!(back.width, 1024);
        assert_eq!(back.depth_or_array_layers, 1);
    }

    #[test]
    fn origin3d_serde_roundtrip() {
        let o = WgpuOrigin3d { x: 4, y: 8, z: 0 };
        let json = serde_json::to_string(&o).unwrap();
        let back: WgpuOrigin3d = serde_json::from_str(&json).unwrap();
        assert_eq!(back.x, 4);
        assert_eq!(back.y, 8);
    }
}

// ── TypeSpec registration ─────────────────────────────────────────────────────

mod specs {
    use super::*;

    #[test]
    fn wgpu_specs_are_registered() {
        for name in [
            "wgpu::TextureFormat",
            "wgpu::PresentMode",
            "wgpu::PowerPreference",
            "wgpu::TextureDimension",
            "wgpu::TextureViewDimension",
            "wgpu::PrimitiveTopology",
            "wgpu::FrontFace",
            "wgpu::Face",
            "wgpu::PolygonMode",
            "wgpu::CompareFunction",
            "wgpu::BlendFactor",
            "wgpu::BlendOperation",
            "wgpu::IndexFormat",
            "wgpu::StencilOperation",
            "wgpu::VertexStepMode",
            "wgpu::VertexFormat",
            "wgpu::AddressMode",
            "wgpu::FilterMode",
            "wgpu::SamplerBorderColor",
            "wgpu::CompositeAlphaMode",
            "wgpu::Backend",
            "wgpu::BufferUsages",
            "wgpu::TextureUsages",
            "wgpu::ShaderStages",
            "wgpu::ColorWrites",
        ] {
            let spec = lookup_type_spec(name);
            assert!(spec.is_some(), "missing TypeSpec for {name}");
            let spec = spec.unwrap();
            assert!(!spec.categories().is_empty(), "{name}: no categories");
        }
    }

    #[test]
    fn wgpu_struct_specs_are_registered() {
        for name in ["WgpuExtent3d", "WgpuColor", "WgpuOrigin3d"] {
            let spec = lookup_type_spec(name);
            assert!(spec.is_some(), "missing TypeSpec for {name}");
        }
    }

    #[test]
    fn texture_format_spec_has_variants() {
        let spec = lookup_type_spec("wgpu::TextureFormat").unwrap();
        let variants_cat = spec.categories().iter().find(|c| c.name() == "variants");
        assert!(variants_cat.is_some(), "no variants category");
        let cat = variants_cat.unwrap();
        assert!(
            cat.entries().len() >= 20,
            "expected many texture format variants"
        );
    }

    #[test]
    fn extent3d_spec_has_three_fields() {
        let spec = lookup_type_spec("WgpuExtent3d").unwrap();
        let fields_cat = spec.categories().iter().find(|c| c.name() == "fields");
        let cat = fields_cat.expect("fields category");
        assert_eq!(cat.entries().len(), 3, "expected 3 fields for Extent3d");
    }
}

// ── Proof generation ──────────────────────────────────────────────────────────

mod proofs {
    use super::*;

    #[test]
    fn wgpu_enum_proofs_non_empty() {
        assert_proofs_non_empty::<WgpuTextureFormat>("WgpuTextureFormat");
        assert_proofs_non_empty::<WgpuPrimitiveTopology>("WgpuPrimitiveTopology");
        assert_proofs_non_empty::<WgpuBufferUsages>("WgpuBufferUsages");
        assert_proofs_non_empty::<WgpuShaderStages>("WgpuShaderStages");
    }

    #[test]
    fn wgpu_struct_proofs_non_empty() {
        assert_proofs_non_empty::<WgpuExtent3d>("WgpuExtent3d");
        assert_proofs_non_empty::<WgpuColor>("WgpuColor");
        assert_proofs_non_empty::<WgpuOrigin3d>("WgpuOrigin3d");
    }
}
