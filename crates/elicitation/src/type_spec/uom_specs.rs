//! [`ElicitSpec`](crate::ElicitSpec) and [`ElicitComplete`](crate::ElicitComplete)
//! implementations for uom (units of measurement) types.

use crate::{
    ElicitComplete, ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
    TypeSpecInventoryKey, UomFormula, UomQuantityKind, UomStep, UomUnitSystem,
};

macro_rules! uom_select_spec {
    (
        type    = $ty:ty,
        name    = $name:literal,
        summary = $summary:literal,
        variants = [$(($label:literal, $desc:literal)),+ $(,)?]
    ) => {
        impl ElicitSpec for $ty {
            fn type_spec() -> TypeSpec {
                let variants = SpecCategoryBuilder::default()
                    .name("variants".to_string())
                    .entries(vec![$(
                        SpecEntryBuilder::default()
                            .label($label.to_string())
                            .description($desc.to_string())
                            .build()
                            .expect("valid SpecEntry"),
                    )+])
                    .build()
                    .expect("valid SpecCategory");
                TypeSpecBuilder::default()
                    .type_name($name.to_string())
                    .summary($summary.to_string())
                    .categories(vec![variants])
                    .build()
                    .expect("valid TypeSpec")
            }
        }

        inventory::submit!(TypeSpecInventoryKey::new(
            $name,
            <$ty as ElicitSpec>::type_spec,
            std::any::TypeId::of::<$ty>
        ));

        impl ElicitComplete for $ty {}
    };
}

macro_rules! uom_survey_spec {
    (
        type    = $ty:ty,
        name    = $name:literal,
        summary = $summary:literal,
        fields  = [$(($label:literal, $desc:literal)),+ $(,)?]
    ) => {
        impl ElicitSpec for $ty {
            fn type_spec() -> TypeSpec {
                let fields = SpecCategoryBuilder::default()
                    .name("fields".to_string())
                    .entries(vec![$(
                        SpecEntryBuilder::default()
                            .label($label.to_string())
                            .description($desc.to_string())
                            .build()
                            .expect("valid SpecEntry"),
                    )+])
                    .build()
                    .expect("valid SpecCategory");
                TypeSpecBuilder::default()
                    .type_name($name.to_string())
                    .summary($summary.to_string())
                    .categories(vec![fields])
                    .build()
                    .expect("valid TypeSpec")
            }
        }

        inventory::submit!(TypeSpecInventoryKey::new(
            $name,
            <$ty as ElicitSpec>::type_spec,
            std::any::TypeId::of::<$ty>
        ));

        impl ElicitComplete for $ty {}
    };
}

uom_select_spec!(
    type    = UomQuantityKind,
    name    = "elicitation::UomQuantityKind",
    summary = "One of 18 registered physical quantity kinds (7 SI base + 11 derived).",
    variants = [
        ("length",              "Base SI — metre (m)"),
        ("mass",                "Base SI — kilogram (kg)"),
        ("time",                "Base SI — second (s)"),
        ("temperature",         "Base SI — kelvin (K)"),
        ("electric_current",    "Base SI — ampere (A)"),
        ("amount_of_substance", "Base SI — mole (mol)"),
        ("luminous_intensity",  "Base SI — candela (cd)"),
        ("velocity",            "Derived — metre per second (m/s)"),
        ("acceleration",        "Derived — metre per second squared (m/s²)"),
        ("force",               "Derived — newton (kg⋅m/s²)"),
        ("energy",              "Derived — joule (kg⋅m²/s²)"),
        ("power",               "Derived — watt (J/s)"),
        ("pressure",            "Derived — pascal (N/m²)"),
        ("frequency",           "Derived — hertz (1/s)"),
        ("area",                "Derived — square metre (m²)"),
        ("volume",              "Derived — cubic metre (m³)"),
        ("density",             "Derived — kilogram per cubic metre (kg/m³)"),
        ("angle",               "Derived — radian (rad)"),
    ]
);

uom_select_spec!(
    type    = UomUnitSystem,
    name    = "elicitation::UomUnitSystem",
    summary = "Unit system used for a physical quantity.",
    variants = [
        ("si",       "International System of Units (SI)"),
        ("imperial", "Imperial / US customary units"),
        ("natural",  "Natural units"),
    ]
);

uom_survey_spec!(
    type    = UomStep,
    name    = "elicitation::UomStep",
    summary = "A single step in a unit-of-measurement computation workflow.",
    fields  = [
        ("description",  "String — human-readable description of the step"),
        ("kind",         "UomQuantityKind — the quantity kind produced by this step"),
        ("code_snippet", "String — Rust code snippet that implements this step"),
    ]
);

uom_survey_spec!(
    type    = UomFormula,
    name    = "elicitation::UomFormula",
    summary = "Descriptor for a physics formula involving unit quantities.",
    fields  = [
        ("name",        "String — formula name (e.g. \"KineticEnergy\")"),
        ("formula",     "String — symbolic formula string (e.g. \"E = ½mv²\")"),
        ("description", "String — what the formula computes"),
        ("params",      "Vec<(String, UomQuantityKind)> — parameter names and their quantity kinds"),
        ("result_kind", "UomQuantityKind — the quantity kind of the result"),
    ]
);
