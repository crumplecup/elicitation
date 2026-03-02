//! [`ElicitSpec`](crate::ElicitSpec) implementations for float contract types.

use crate::verification::types::{
    F32Finite, F32NonNegative, F32Positive, F64Finite, F64NonNegative, F64Positive,
};
use crate::{
    ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
    TypeSpecInventoryKey,
};

macro_rules! impl_float_contract_spec {
    (
        type     = $ty:ty,
        name     = $name:literal,
        base     = $base:literal,
        summary  = $summary:literal,
        requires = [$( ($req_label:literal, $req_desc:literal, $req_expr:literal) ),+ $(,)?] $(,)?
    ) => {
        impl ElicitSpec for $ty {
            fn type_spec() -> TypeSpec {
                let requires = SpecCategoryBuilder::default()
                    .name("requires".to_string())
                    .entries(vec![
                        $( SpecEntryBuilder::default()
                            .label($req_label.to_string())
                            .description($req_desc.to_string())
                            .expression(Some($req_expr.to_string()))
                            .build()
                            .expect("valid SpecEntry"), )+
                    ])
                    .build()
                    .expect("valid SpecCategory");
                let bounds = SpecCategoryBuilder::default()
                    .name("bounds".to_string())
                    .entries(vec![
                        SpecEntryBuilder::default()
                            .label("base_type".to_string())
                            .description(concat!("Underlying primitive is ", $base, ".").to_string())
                            .expression(None)
                            .build()
                            .expect("valid SpecEntry"),
                    ])
                    .build()
                    .expect("valid SpecCategory");
                TypeSpecBuilder::default()
                    .type_name($name.to_string())
                    .summary($summary.to_string())
                    .categories(vec![requires, bounds])
                    .build()
                    .expect("valid TypeSpec")
            }
        }

        inventory::submit!(TypeSpecInventoryKey::new($name, <$ty as ElicitSpec>::type_spec, std::any::TypeId::of::<$ty>));
    };
}

// ── f32 contract types ────────────────────────────────────────────────────────

impl_float_contract_spec!(
    type    = F32Positive,
    name    = "F32Positive",
    base    = "f32",
    summary = "A positive finite 32-bit float (value > 0.0 and finite).",
    requires = [
        ("finite", "Value must be finite (not NaN or infinite).", "value.is_finite()"),
        ("positive", "Value must be strictly greater than zero.", "value > 0.0"),
    ],
);

impl_float_contract_spec!(
    type    = F32NonNegative,
    name    = "F32NonNegative",
    base    = "f32",
    summary = "A non-negative finite 32-bit float (value >= 0.0 and finite).",
    requires = [
        ("finite", "Value must be finite (not NaN or infinite).", "value.is_finite()"),
        ("non_negative", "Value must be zero or greater.", "value >= 0.0"),
    ],
);

impl_float_contract_spec!(
    type    = F32Finite,
    name    = "F32Finite",
    base    = "f32",
    summary = "A finite 32-bit float (not NaN or infinite).",
    requires = [
        ("finite", "Value must be finite (not NaN or infinite).", "value.is_finite()"),
    ],
);

// ── f64 contract types ────────────────────────────────────────────────────────

impl_float_contract_spec!(
    type    = F64Positive,
    name    = "F64Positive",
    base    = "f64",
    summary = "A positive finite 64-bit float (value > 0.0 and finite).",
    requires = [
        ("finite", "Value must be finite (not NaN or infinite).", "value.is_finite()"),
        ("positive", "Value must be strictly greater than zero.", "value > 0.0"),
    ],
);

impl_float_contract_spec!(
    type    = F64NonNegative,
    name    = "F64NonNegative",
    base    = "f64",
    summary = "A non-negative finite 64-bit float (value >= 0.0 and finite).",
    requires = [
        ("finite", "Value must be finite (not NaN or infinite).", "value.is_finite()"),
        ("non_negative", "Value must be zero or greater.", "value >= 0.0"),
    ],
);

impl_float_contract_spec!(
    type    = F64Finite,
    name    = "F64Finite",
    base    = "f64",
    summary = "A finite 64-bit float (not NaN or infinite).",
    requires = [
        ("finite", "Value must be finite (not NaN or infinite).", "value.is_finite()"),
    ],
);
