//! `UomCodePlugin` — MCP tools for uom code emission and catalog queries.
//!
//! # Tool namespace: `uom_code__*`

use std::sync::Arc;

use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    ALL_REGISTRATIONS, QuantityBus, new_bus, parse_params, rust_type_name, si_unit_name,
    supported_units,
};

// ── Helpers ───────────────────────────────────────────────────────────────────

fn ok_text(s: impl Into<String>) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(s.into())]))
}

fn ok_json<T: Serialize>(v: &T) -> Result<CallToolResult, ErrorData> {
    serde_json::to_string(v)
        .map(|s| CallToolResult::success(vec![Content::text(s)]))
        .map_err(|e| ErrorData::invalid_params(format!("serialise: {e}"), None))
}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for `uom_code__*` tools.
pub struct UomCodeCtx {
    /// Access to the shared quantity bus for emit tools that reference stored values.
    pub(crate) bus: QuantityBus,
}

impl UomCodeCtx {
    fn new(bus: QuantityBus) -> Self {
        Self { bus }
    }
}

impl elicitation::PluginContext for UomCodeCtx {}

// ── Param structs ─────────────────────────────────────────────────────────────

/// Parameters for `uom_code__emit_conversion`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EmitConversionParams {
    /// Registration name (e.g. `"length"`).
    pub registration: String,
    /// Numeric value.
    pub value: f64,
    /// Source unit.
    pub from_unit: String,
    /// Target unit.
    pub to_unit: String,
    /// Variable name to use in emitted code.
    pub var_name: Option<String>,
}

/// Parameters for `uom_code__emit_calculation`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EmitCalculationParams {
    /// First quantity UUID.
    pub lhs_id: String,
    /// Operation: `"add"`, `"sub"`, `"mul"`, `"div"`.
    pub op: String,
    /// Second quantity UUID.
    pub rhs_id: String,
    /// Variable name for result.
    pub result_var: Option<String>,
}

/// Parameters for `uom_code__emit_physics_formula`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EmitPhysicsFormulaParams {
    /// Formula name: `"KineticEnergy"`, `"GravitationalPE"`, `"OhmsLaw"`,
    /// `"IdealGas"`, `"Momentum"`.
    pub formula: String,
}

/// Parameters for `uom_code__emit_main`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EmitMainParams {
    /// UUIDs of quantities to include in `fn main()`.
    pub ids: Vec<String>,
    /// Print style: `"abbreviation"` (default) or `"description"`.
    pub display_style: Option<String>,
}

/// Parameters for `uom_code__emit_snippet`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EmitSnippetParams {
    /// One or more quantity UUIDs.
    pub ids: Vec<String>,
    /// Optional target unit for each (parallel array, may be shorter).
    pub units: Option<Vec<String>>,
}

/// Parameters for `uom_code__catalog_quantities`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CatalogQuantitiesParams {
    /// Filter to base, derived, or all (default).
    pub filter: Option<String>,
}

/// Parameters for `uom_code__catalog_units`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CatalogUnitsParams {
    /// Registration name to list units for.
    pub registration: String,
}

/// Parameters for `uom_code__catalog_unit_info`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CatalogUnitInfoParams {
    /// Unit string to look up.
    pub unit: String,
}

/// Parameters for `uom_code__catalog_suggest_unit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CatalogSuggestUnitParams {
    /// Registration name.
    pub registration: String,
    /// Preferred system: `"si"`, `"imperial"`, or `"any"` (default).
    pub system: Option<String>,
}

/// Parameters for `uom_code__catalog_conversion_table`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CatalogConversionTableParams {
    /// Registration name.
    pub registration: String,
    /// Reference value in SI base unit.
    pub si_value: f64,
}

/// Parameters for `uom_code__catalog_dimension`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CatalogDimensionParams {
    /// Registration name.
    pub registration: String,
}

/// Parameters for `uom_code__catalog_compatible_units`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CatalogCompatibleUnitsParams {
    /// Registration name.
    pub registration: String,
    /// Unit string to find compatibles for.
    pub unit: String,
}

// ── Tool implementations ──────────────────────────────────────────────────────

#[instrument(skip(_bus, p))]
async fn emit_conversion(
    _bus: &QuantityBus,
    p: EmitConversionParams,
) -> Result<CallToolResult, ErrorData> {
    let var = p.var_name.as_deref().unwrap_or(&p.registration);
    let rust_type = rust_type_name(&p.registration);
    // Use the imported uom module path.
    let code = format!(
        r#"use uom::si::f64::{rust_type};
use uom::si::{reg}::*;

let {var} = {rust_type}::new::<{from}>({value});
let {var}_in_{to} = {var}.get::<{to}>();
println!("{{}}", {var}_in_{to});"#,
        rust_type = rust_type,
        reg = registration_module(&p.registration),
        var = var,
        from = p.from_unit,
        value = p.value,
        to = p.to_unit,
    );
    ok_text(code)
}

#[instrument(skip(bus, p))]
async fn emit_calculation(
    bus: &QuantityBus,
    p: EmitCalculationParams,
) -> Result<CallToolResult, ErrorData> {
    let guard = bus.lock().await;
    let lhs_id: uuid::Uuid = p
        .lhs_id
        .parse()
        .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.lhs_id), None))?;
    let rhs_id: uuid::Uuid = p
        .rhs_id
        .parse()
        .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.rhs_id), None))?;
    let lhs = guard
        .get(&lhs_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("not found: {lhs_id}"), None))?;
    let rhs = guard
        .get(&rhs_id)
        .ok_or_else(|| ErrorData::invalid_params(format!("not found: {rhs_id}"), None))?;
    let result_var = p.result_var.as_deref().unwrap_or("result");
    let op_sym = match p.op.as_str() {
        "add" => "+",
        "sub" => "-",
        "mul" => "*",
        "div" => "/",
        other => {
            return Err(ErrorData::invalid_params(
                format!("unknown op: {other}"),
                None,
            ));
        }
    };
    let code = format!(
        "// lhs: {lhs_snippet}\n// rhs: {rhs_snippet}\nlet {result_var} = lhs {op} rhs;",
        lhs_snippet = lhs.code_snippet,
        rhs_snippet = rhs.code_snippet,
        result_var = result_var,
        op = op_sym,
    );
    ok_text(code)
}

#[instrument(skip(_bus, p))]
async fn emit_physics_formula(
    _bus: &QuantityBus,
    p: EmitPhysicsFormulaParams,
) -> Result<CallToolResult, ErrorData> {
    let code = match p.formula.as_str() {
        "KineticEnergy" => r#"use uom::si::f64::{Mass, Velocity, Energy};
use uom::si::mass::kilogram;
use uom::si::velocity::meter_per_second;
use uom::si::energy::joule;

// E = ½mv²
let mass = Mass::new::<kilogram>(/* mass_kg */);
let velocity = Velocity::new::<meter_per_second>(/* v_m_per_s */);
let energy = Energy::new::<joule>(
    0.5 * mass.get::<kilogram>() * velocity.get::<meter_per_second>().powi(2)
);
println!("KE = {} J", energy.get::<joule>());"#
            .to_string(),
        "GravitationalPE" => r#"use uom::si::f64::{Mass, Length, Energy};
use uom::si::mass::kilogram;
use uom::si::length::meter;
use uom::si::energy::joule;

// U = mgh  (g = 9.80665 m/s²)
let mass = Mass::new::<kilogram>(/* mass_kg */);
let height = Length::new::<meter>(/* height_m */);
let g = 9.806_65_f64; // m/s²
let energy = Energy::new::<joule>(
    mass.get::<kilogram>() * g * height.get::<meter>()
);
println!("PE = {} J", energy.get::<joule>());"#
            .to_string(),
        "OhmsLaw" => r#"use uom::si::f64::ElectricCurrent;
use uom::si::electric_current::ampere;

// V = IR  (resistance in ohms as plain f64)
let current = ElectricCurrent::new::<ampere>(/* current_A */);
let resistance_ohms: f64 = /* R_ohms */0.0;
let voltage_volts = current.get::<ampere>() * resistance_ohms;
println!("V = {} V", voltage_volts);"#
            .to_string(),
        "IdealGas" => {
            r#"use uom::si::f64::{AmountOfSubstance, ThermodynamicTemperature, Pressure, Volume};
use uom::si::amount_of_substance::mole;
use uom::si::thermodynamic_temperature::kelvin;
use uom::si::pressure::pascal;
use uom::si::volume::cubic_meter;

// PV = nRT  (R = 8.314 J/(mol·K))
let n = AmountOfSubstance::new::<mole>(/* moles */);
let temp = ThermodynamicTemperature::new::<kelvin>(/* kelvin */);
let r = 8.314_f64; // J/(mol·K)
let pressure = Pressure::new::<pascal>(/* P_Pa */);
let volume = Volume::new::<cubic_meter>(
    n.get::<mole>() * r * temp.get::<kelvin>() / pressure.get::<pascal>()
);
println!("V = {} m³", volume.get::<cubic_meter>());"#
                .to_string()
        }
        "Momentum" => r#"use uom::si::f64::{Mass, Velocity};
use uom::si::mass::kilogram;
use uom::si::velocity::meter_per_second;

// p = mv
let mass = Mass::new::<kilogram>(/* mass_kg */);
let velocity = Velocity::new::<meter_per_second>(/* v_m_per_s */);
let momentum_kg_m_per_s = mass.get::<kilogram>() * velocity.get::<meter_per_second>();
println!("p = {} kg·m/s", momentum_kg_m_per_s);"#
            .to_string(),
        other => {
            return Err(ErrorData::invalid_params(
                format!(
                    "unknown formula: {other}. Available: KineticEnergy, GravitationalPE, OhmsLaw, IdealGas, Momentum"
                ),
                None,
            ));
        }
    };
    ok_text(code)
}

#[instrument(skip(bus, p))]
async fn emit_main(bus: &QuantityBus, p: EmitMainParams) -> Result<CallToolResult, ErrorData> {
    let style = p.display_style.as_deref().unwrap_or("abbreviation");
    let display_style = if style == "description" {
        "uom::fmt::DisplayStyle::Description"
    } else {
        "uom::fmt::DisplayStyle::Abbreviation"
    };

    let mut imports = std::collections::BTreeSet::new();
    let mut declarations = Vec::new();
    let mut prints = Vec::new();

    let guard = bus.lock().await;
    for id_str in &p.ids {
        let id: uuid::Uuid = id_str
            .parse()
            .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {id_str}"), None))?;
        if let Some(entry) = guard.get(&id) {
            let rust_type = rust_type_name(entry.registration);
            let si_unit = si_unit_name(entry.registration);
            imports.insert(format!("use uom::si::f64::{rust_type};"));
            imports.insert(format!(
                "use uom::si::{}::*;",
                registration_module(entry.registration)
            ));
            let var = format!("qty_{}", &id_str[..8]);
            declarations.push(format!(
                "    let {var} = {rust_type}::new::<{si_unit}>({si_value});",
                var = var,
                rust_type = rust_type,
                si_unit = si_unit,
                si_value = entry.si_value,
            ));
            prints.push(format!(
                "    println!(\"{{}}\", {var}.into_format_args({si_unit}, {style}));",
                var = var,
                si_unit = si_unit,
                style = display_style,
            ));
        }
    }

    let code = format!(
        "use uom::fmt::DisplayStyle;\n{imports}\n\nfn main() {{\n{decls}\n{prints}\n}}",
        imports = imports.into_iter().collect::<Vec<_>>().join("\n"),
        decls = declarations.join("\n"),
        prints = prints.join("\n"),
    );
    ok_text(code)
}

#[instrument(skip(bus, p))]
async fn emit_snippet(
    bus: &QuantityBus,
    p: EmitSnippetParams,
) -> Result<CallToolResult, ErrorData> {
    let guard = bus.lock().await;
    let mut lines = Vec::new();
    for (i, id_str) in p.ids.iter().enumerate() {
        let id: uuid::Uuid = id_str
            .parse()
            .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {id_str}"), None))?;
        if let Some(entry) = guard.get(&id) {
            let target_unit = p.units.as_ref().and_then(|u| u.get(i));
            if let Some(unit) = target_unit {
                let rust_type = rust_type_name(entry.registration);
                let si_unit = si_unit_name(entry.registration);
                lines.push(format!(
                    "let qty = {rust_type}::new::<{si_unit}>({si_value});\nlet in_{unit} = qty.get::<{unit}>();",
                    rust_type = rust_type,
                    si_unit = si_unit,
                    si_value = entry.si_value,
                    unit = unit,
                ));
            } else {
                lines.push(entry.code_snippet.clone());
            }
        }
    }
    ok_text(lines.join("\n"))
}

#[instrument(skip(p))]
async fn catalog_quantities(p: CatalogQuantitiesParams) -> Result<CallToolResult, ErrorData> {
    let filter = p.filter.as_deref().unwrap_or("all");
    let base = &[
        "length",
        "mass",
        "time",
        "temperature",
        "electric_current",
        "amount_of_substance",
        "luminous_intensity",
    ];
    let derived = &[
        "velocity",
        "acceleration",
        "force",
        "energy",
        "power",
        "pressure",
        "frequency",
        "area",
        "volume",
        "density",
        "angle",
    ];
    let list: Vec<serde_json::Value> = match filter {
        "base" => base.iter().map(|r| quantity_info(r)).collect(),
        "derived" => derived.iter().map(|r| quantity_info(r)).collect(),
        _ => ALL_REGISTRATIONS.iter().map(|r| quantity_info(r)).collect(),
    };
    ok_json(&list)
}

fn quantity_info(reg: &str) -> serde_json::Value {
    serde_json::json!({
        "registration": reg,
        "rust_type": rust_type_name(reg),
        "si_unit": si_unit_name(reg),
        "units": supported_units(reg),
    })
}

#[instrument(skip(p))]
async fn catalog_units(p: CatalogUnitsParams) -> Result<CallToolResult, ErrorData> {
    let units = supported_units(&p.registration);
    if units.is_empty() {
        return Err(ErrorData::invalid_params(
            format!("unknown registration: {}", p.registration),
            None,
        ));
    }
    ok_json(&serde_json::json!({
        "registration": p.registration,
        "units": units,
    }))
}

#[instrument(skip(p))]
async fn catalog_unit_info(p: CatalogUnitInfoParams) -> Result<CallToolResult, ErrorData> {
    // Find which registration this unit belongs to
    for &reg in ALL_REGISTRATIONS {
        if supported_units(reg).contains(&p.unit.as_str()) {
            return ok_json(&serde_json::json!({
                "unit": p.unit,
                "registration": reg,
                "rust_type": rust_type_name(reg),
                "si_unit": si_unit_name(reg),
            }));
        }
    }
    Err(ErrorData::invalid_params(
        format!("unit not found in any registration: {}", p.unit),
        None,
    ))
}

#[instrument(skip(p))]
async fn catalog_suggest_unit(p: CatalogSuggestUnitParams) -> Result<CallToolResult, ErrorData> {
    let units = supported_units(&p.registration);
    if units.is_empty() {
        return Err(ErrorData::invalid_params(
            format!("unknown registration: {}", p.registration),
            None,
        ));
    }
    let system = p.system.as_deref().unwrap_or("si");
    let suggestion = match system {
        "imperial" => imperial_default(&p.registration),
        _ => si_unit_name(&p.registration),
    };
    ok_json(&serde_json::json!({
        "registration": p.registration,
        "suggested_unit": suggestion,
        "system": system,
    }))
}

fn imperial_default(reg: &str) -> &'static str {
    match reg {
        "length" => "foot",
        "mass" => "pound",
        "temperature" => "degree_fahrenheit",
        "velocity" => "mile_per_hour",
        "acceleration" => "foot_per_second_squared",
        "force" => "pound_force",
        "energy" => "kilocalorie",
        "power" => "horsepower",
        "pressure" => "psi",
        "area" => "square_foot",
        "volume" => "gallon",
        "density" => "pound_per_cubic_foot",
        other => si_unit_name(other),
    }
}

#[instrument(skip(p))]
async fn catalog_conversion_table(
    p: CatalogConversionTableParams,
) -> Result<CallToolResult, ErrorData> {
    use crate::convert_to_unit;
    let units = supported_units(&p.registration);
    if units.is_empty() {
        return Err(ErrorData::invalid_params(
            format!("unknown registration: {}", p.registration),
            None,
        ));
    }
    let mut table = Vec::new();
    for &unit in units {
        if let Ok(v) = convert_to_unit(&p.registration, p.si_value, unit) {
            table.push(serde_json::json!({ "unit": unit, "value": v }));
        }
    }
    ok_json(&serde_json::json!({
        "registration": p.registration,
        "si_value": p.si_value,
        "si_unit": si_unit_name(&p.registration),
        "conversions": table,
    }))
}

#[instrument]
async fn catalog_formula_list(_p: serde_json::Value) -> Result<CallToolResult, ErrorData> {
    let formulas = serde_json::json!([
        {
            "name": "KineticEnergy",
            "formula": "E = ½mv²",
            "params": ["mass_id", "velocity_id"],
            "result": "energy",
        },
        {
            "name": "GravitationalPE",
            "formula": "U = mgh",
            "params": ["mass_id", "height_id", "g_constant"],
            "result": "energy",
        },
        {
            "name": "OhmsLaw",
            "formula": "V = IR",
            "params": ["current_id", "resistance_ohms"],
            "result": "voltage_volts_f64",
        },
        {
            "name": "IdealGas",
            "formula": "PV = nRT",
            "params": ["n_id", "T_id", "P_id"],
            "result": "volume",
        },
        {
            "name": "Momentum",
            "formula": "p = mv",
            "params": ["mass_id", "velocity_id"],
            "result": "momentum_kg_m_per_s_f64",
        },
    ]);
    ok_json(&formulas)
}

#[instrument(skip(p))]
async fn catalog_dimension(p: CatalogDimensionParams) -> Result<CallToolResult, ErrorData> {
    let dim = dimension_string(&p.registration)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown: {}", p.registration), None))?;
    ok_json(&serde_json::json!({
        "registration": p.registration,
        "dimension": dim,
        "si_unit": si_unit_name(&p.registration),
    }))
}

fn dimension_string(reg: &str) -> Option<&'static str> {
    match reg {
        "length" => Some("L"),
        "mass" => Some("M"),
        "time" => Some("T"),
        "temperature" => Some("Θ"),
        "electric_current" => Some("I"),
        "amount_of_substance" => Some("N"),
        "luminous_intensity" => Some("J"),
        "velocity" => Some("L T⁻¹"),
        "acceleration" => Some("L T⁻²"),
        "force" => Some("M L T⁻²"),
        "energy" => Some("M L² T⁻²"),
        "power" => Some("M L² T⁻³"),
        "pressure" => Some("M L⁻¹ T⁻²"),
        "frequency" => Some("T⁻¹"),
        "area" => Some("L²"),
        "volume" => Some("L³"),
        "density" => Some("M L⁻³"),
        "angle" => Some("dimensionless (rad)"),
        _ => None,
    }
}

#[instrument(skip(p))]
async fn catalog_compatible_units(
    p: CatalogCompatibleUnitsParams,
) -> Result<CallToolResult, ErrorData> {
    let units = supported_units(&p.registration);
    ok_json(&serde_json::json!({
        "registration": p.registration,
        "reference_unit": p.unit,
        "compatible_units": units,
    }))
}

#[instrument]
async fn catalog_physics_constants() -> Result<CallToolResult, ErrorData> {
    let constants = serde_json::json!([
        { "name": "speed_of_light", "symbol": "c", "value": 299_792_458.0, "unit": "m/s", "description": "Speed of light in vacuum" },
        { "name": "gravitational_constant", "symbol": "G", "value": 6.674e-11, "unit": "N·m²/kg²", "description": "Newtonian gravitational constant" },
        { "name": "planck_constant", "symbol": "h", "value": 6.626e-34, "unit": "J·s", "description": "Planck constant" },
        { "name": "boltzmann_constant", "symbol": "kB", "value": 1.381e-23, "unit": "J/K", "description": "Boltzmann constant" },
        { "name": "avogadro_constant", "symbol": "NA", "value": 6.022e23, "unit": "mol⁻¹", "description": "Avogadro constant" },
        { "name": "elementary_charge", "symbol": "e", "value": 1.602e-19, "unit": "C", "description": "Elementary charge" },
        { "name": "standard_gravity", "symbol": "g", "value": 9.806_65, "unit": "m/s²", "description": "Standard acceleration of gravity" },
        { "name": "vacuum_permittivity", "symbol": "ε0", "value": 8.854e-12, "unit": "F/m", "description": "Vacuum permittivity" },
    ]);
    ok_json(&constants)
}

#[instrument]
async fn catalog_base_quantities() -> Result<CallToolResult, ErrorData> {
    let base = serde_json::json!([
        { "registration": "length", "dimension": "L", "si_unit": "meter", "rust_type": "Length" },
        { "registration": "mass", "dimension": "M", "si_unit": "kilogram", "rust_type": "Mass" },
        { "registration": "time", "dimension": "T", "si_unit": "second", "rust_type": "Time" },
        { "registration": "temperature", "dimension": "Θ", "si_unit": "kelvin", "rust_type": "ThermodynamicTemperature" },
        { "registration": "electric_current", "dimension": "I", "si_unit": "ampere", "rust_type": "ElectricCurrent" },
        { "registration": "amount_of_substance", "dimension": "N", "si_unit": "mole", "rust_type": "AmountOfSubstance" },
        { "registration": "luminous_intensity", "dimension": "J", "si_unit": "candela", "rust_type": "LuminousIntensity" },
    ]);
    ok_json(&base)
}

// ── Module name helper ────────────────────────────────────────────────────────

fn registration_module(reg: &str) -> &'static str {
    match reg {
        "temperature" => "thermodynamic_temperature",
        "density" => "mass_density",
        "length" => "length",
        "mass" => "mass",
        "time" => "time",
        "electric_current" => "electric_current",
        "amount_of_substance" => "amount_of_substance",
        "luminous_intensity" => "luminous_intensity",
        "velocity" => "velocity",
        "acceleration" => "acceleration",
        "force" => "force",
        "energy" => "energy",
        "power" => "power",
        "pressure" => "pressure",
        "frequency" => "frequency",
        "area" => "area",
        "volume" => "volume",
        "angle" => "angle",
        _ => "unknown",
    }
}

// ── Plugin struct ─────────────────────────────────────────────────────────────

/// MCP plugin providing code emission and catalog query tools.
pub struct UomCodePlugin(Arc<UomCodeCtx>);

impl UomCodePlugin {
    /// Create a new `UomCodePlugin` with its own quantity bus.
    pub fn new() -> Self {
        Self(Arc::new(UomCodeCtx::new(new_bus())))
    }

    /// Create a `UomCodePlugin` that shares a bus with a [`UomQuantityPlugin`].
    pub fn with_bus(bus: QuantityBus) -> Self {
        Self(Arc::new(UomCodeCtx::new(bus)))
    }

    /// Invoke a tool by name with a JSON arguments object (no `RequestContext` needed).
    ///
    /// Convenience for tests and direct integration.
    pub async fn invoke_tool(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let owned: String = name.to_string();
        let params = if let Some(m) = args.as_object().cloned() {
            CallToolRequestParams::new(owned).with_arguments(m)
        } else {
            CallToolRequestParams::new(owned)
        };
        dispatch_code_tool(self.0.clone(), name, &params).await
    }
}

impl Default for UomCodePlugin {
    fn default() -> Self {
        Self::new()
    }
}

fn build_tool(
    name: impl Into<std::borrow::Cow<'static, str>>,
    description: impl Into<std::borrow::Cow<'static, str>>,
    schema: serde_json::Value,
) -> Tool {
    use std::sync::Arc;
    let schema_obj: Arc<rmcp::model::JsonObject> = match schema {
        serde_json::Value::Object(m) => Arc::new(m),
        _ => Arc::new(Default::default()),
    };
    Tool::new(name, description, schema_obj)
}

fn schema_of<T: schemars::JsonSchema>() -> serde_json::Value {
    serde_json::to_value(schemars::schema_for!(T)).unwrap_or_default()
}

fn empty_schema() -> serde_json::Value {
    serde_json::json!({"type": "object", "properties": {}})
}

impl elicitation::ElicitPlugin for UomCodePlugin {
    fn name(&self) -> &'static str {
        "uom_code"
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![
            build_tool(
                "uom_code__emit_conversion",
                "Emit Rust code for a unit conversion.",
                schema_of::<EmitConversionParams>(),
            ),
            build_tool(
                "uom_code__emit_calculation",
                "Emit Rust code for a binary arithmetic calculation on two stored quantities.",
                schema_of::<EmitCalculationParams>(),
            ),
            build_tool(
                "uom_code__emit_physics_formula",
                "Emit Rust code for a named physics formula (KineticEnergy, GravitationalPE, OhmsLaw, IdealGas, Momentum).",
                schema_of::<EmitPhysicsFormulaParams>(),
            ),
            build_tool(
                "uom_code__emit_main",
                "Emit a complete `fn main()` that creates and prints quantities.",
                schema_of::<EmitMainParams>(),
            ),
            build_tool(
                "uom_code__emit_snippet",
                "Emit Rust code snippets for stored quantity UUIDs.",
                schema_of::<EmitSnippetParams>(),
            ),
            build_tool(
                "uom_code__catalog_quantities",
                "List all registered quantity kinds with rust type, SI unit, and available units.",
                schema_of::<CatalogQuantitiesParams>(),
            ),
            build_tool(
                "uom_code__catalog_units",
                "List all supported units for a given registration.",
                schema_of::<CatalogUnitsParams>(),
            ),
            build_tool(
                "uom_code__catalog_unit_info",
                "Look up which registration a unit belongs to.",
                schema_of::<CatalogUnitInfoParams>(),
            ),
            build_tool(
                "uom_code__catalog_suggest_unit",
                "Suggest the most natural unit for a registration and unit system.",
                schema_of::<CatalogSuggestUnitParams>(),
            ),
            build_tool(
                "uom_code__catalog_conversion_table",
                "Generate a conversion table for a registration at a reference SI value.",
                schema_of::<CatalogConversionTableParams>(),
            ),
            build_tool(
                "uom_code__catalog_formula_list",
                "List all available physics formulas.",
                empty_schema(),
            ),
            build_tool(
                "uom_code__catalog_dimension",
                "Get the dimensional analysis string for a registration.",
                schema_of::<CatalogDimensionParams>(),
            ),
            build_tool(
                "uom_code__catalog_compatible_units",
                "List all units compatible with (same registration as) a given unit.",
                schema_of::<CatalogCompatibleUnitsParams>(),
            ),
            build_tool(
                "uom_code__catalog_physics_constants",
                "List fundamental physics constants with values and units.",
                empty_schema(),
            ),
            build_tool(
                "uom_code__catalog_base_quantities",
                "List the 7 SI base quantities.",
                empty_schema(),
            ),
        ]
    }

    #[tracing::instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<rmcp::RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        let ctx = self.0.clone();
        Box::pin(async move {
            let name = params.name.as_ref();
            dispatch_code_tool(ctx, name, &params).await
        })
    }
}

async fn dispatch_code_tool(
    ctx: Arc<UomCodeCtx>,
    name: &str,
    params: &CallToolRequestParams,
) -> Result<CallToolResult, ErrorData> {
    match name {
        "uom_code__emit_conversion" => emit_conversion(&ctx.bus, parse_params(params)?).await,
        "uom_code__emit_calculation" => emit_calculation(&ctx.bus, parse_params(params)?).await,
        "uom_code__emit_physics_formula" => {
            emit_physics_formula(&ctx.bus, parse_params(params)?).await
        }
        "uom_code__emit_main" => emit_main(&ctx.bus, parse_params(params)?).await,
        "uom_code__emit_snippet" => emit_snippet(&ctx.bus, parse_params(params)?).await,
        "uom_code__catalog_quantities" => catalog_quantities(parse_params(params)?).await,
        "uom_code__catalog_units" => catalog_units(parse_params(params)?).await,
        "uom_code__catalog_unit_info" => catalog_unit_info(parse_params(params)?).await,
        "uom_code__catalog_suggest_unit" => catalog_suggest_unit(parse_params(params)?).await,
        "uom_code__catalog_conversion_table" => {
            catalog_conversion_table(parse_params(params)?).await
        }
        "uom_code__catalog_formula_list" => {
            let _: serde_json::Value = parse_params(params).unwrap_or(serde_json::json!({}));
            catalog_formula_list(serde_json::json!({})).await
        }
        "uom_code__catalog_dimension" => catalog_dimension(parse_params(params)?).await,
        "uom_code__catalog_compatible_units" => {
            catalog_compatible_units(parse_params(params)?).await
        }
        "uom_code__catalog_physics_constants" => catalog_physics_constants().await,
        "uom_code__catalog_base_quantities" => catalog_base_quantities().await,
        _ => Err(ErrorData::invalid_params(
            format!("unknown tool: {name}"),
            None,
        )),
    }
}
