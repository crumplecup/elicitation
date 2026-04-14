# elicit_uom Implementation Plan

`uom` 0.38 — Units of Measurement with compile-time dimensional analysis.

---

## Why uom Is Different

`uom` is a **type-system-first** library. Its core value — dimensional homogeneity checked
at compile time — lives entirely in Rust's type system. `Quantity<D, U, V>` carries phantom
type parameters for dimension (`D`) and unit system (`U`), so no type-erased runtime registry
(Phase 3D) is possible: you cannot write `HashMap<Uuid, Quantity<???, ???, f64>>`.

**Our solution**: store `(QuantityKind, f64)` pairs where `f64` is always the SI base value.
Use `uom` internally for all conversion math — no reimplementation of factors. The library
does the heavy lifting; we manage the registry.

This is the **runtime enum-wrapper pattern** — a variant of Phase 3D adapted for phantom-generic types.

---

## Architecture: 2 Plugins, ~55 Tools

### Pattern Mapping

| Pattern | Plugin | Why |
|---|---|---|
| Multi-param factory (Phase 3F.6) | `UomQuantityPlugin` | Each of 18 registered quantities gets its own typed `HashMap<Uuid, Q>` |
| QuantityBus (cross-type arithmetic) | shared Arc | f64 SI routing layer between typed registrations |
| Descriptor + emit_main | `UomCodePlugin` | Emit complete uom Rust programs |
| Static catalog | `UomCodePlugin` | Unit listing, conversion table, formula suggestions |

### Key Design: Factory over Enum-Wrapper

The initial plan proposed storing `UomValue` (enum of 18 variants × f64). This works but
loses type information at the plugin boundary. The factory pattern does better:

- Each `register_quantity::<uom::si::f64::Length>("length")` call creates a typed
  `HashMap<Uuid, uom::si::f64::Length>` — **no type erasure**
- `length__emit(ids)` can produce perfect `Length::new::<meter>(...)` code because the
  type is known statically
- A shared **QuantityBus** (`Arc<Mutex<HashMap<Uuid, (name, f64_si)>>>`) enables
  cross-registration arithmetic without destroying the typed storage

`uom::si::f64::Length` IS `Quantity<LengthDimension, SI<f64>, f64>` — registering the
type alias is "three-generic factory with one call-site type param." The factory
monomorphizes tools for the exact (D, U, V) triple the alias resolves to.

---

## Supported Quantities (18)

### Base Quantities (7 — ISQ)

| Kind | SI Base Unit | Key Non-SI Units |
|---|---|---|
| `Length` | meter (m) | foot, inch, yard, mile, km, nautical_mile, light_year, parsec |
| `Mass` | kilogram (kg) | gram, pound, ounce, tonne, short_ton, stone |
| `Time` | second (s) | minute, hour, day, week, millisecond, microsecond, nanosecond |
| `Temperature` | kelvin (K) | degree_celsius, degree_fahrenheit, degree_rankine |
| `ElectricCurrent` | ampere (A) | milliampere, microampere, kiloampere |
| `AmountOfSubstance` | mole (mol) | millimole, micromole, kilomole |
| `LuminousIntensity` | candela (cd) | millicandela |

### Derived Quantities (11)

| Kind | SI Unit | Derivation |
|---|---|---|
| `Velocity` | m/s | Length / Time |
| `Acceleration` | m/s² | Velocity / Time |
| `Force` | newton (N = kg⋅m⋅s⁻²) | Mass × Acceleration |
| `Energy` | joule (J = kg⋅m²⋅s⁻²) | Force × Length |
| `Power` | watt (W = J/s) | Energy / Time |
| `Pressure` | pascal (Pa = N/m²) | Force / Area |
| `Frequency` | hertz (Hz = s⁻¹) | 1 / Time |
| `Area` | m² | Length × Length |
| `Volume` | m³ | Area × Length |
| `Density` | kg/m³ | Mass / Volume |
| `Angle` | radian (rad) | dimensionless ratio |

---

## Plugin 1: `UomQuantityPlugin` (~40 tools)

Factory registration — each quantity gets its own typed storage:

```rust
let plugin = UomQuantityPlugin::builder()
    // 7 base quantities
    .register_quantity::<uom::si::f64::Length>("length")
    .register_quantity::<uom::si::f64::Mass>("mass")
    .register_quantity::<uom::si::f64::Time>("time")
    .register_quantity::<uom::si::f64::ThermodynamicTemperature>("temperature")
    .register_quantity::<uom::si::f64::ElectricCurrent>("electric_current")
    .register_quantity::<uom::si::f64::AmountOfSubstance>("amount_of_substance")
    .register_quantity::<uom::si::f64::LuminousIntensity>("luminous_intensity")
    // 11 derived quantities
    .register_quantity::<uom::si::f64::Velocity>("velocity")
    .register_quantity::<uom::si::f64::Acceleration>("acceleration")
    .register_quantity::<uom::si::f64::Force>("force")
    .register_quantity::<uom::si::f64::Energy>("energy")
    .register_quantity::<uom::si::f64::Power>("power")
    .register_quantity::<uom::si::f64::Pressure>("pressure")
    .register_quantity::<uom::si::f64::Frequency>("frequency")
    .register_quantity::<uom::si::f64::Area>("area")
    .register_quantity::<uom::si::f64::Volume>("volume")
    .register_quantity::<uom::si::f64::MassDensity>("density")
    .register_quantity::<uom::si::f64::Angle>("angle")
    .build();
```

Each registration produces typed `HashMap<Uuid, Q>` plus writes to the shared QuantityBus.

### QuantityBus

```rust
pub type QuantityBus = Arc<Mutex<HashMap<Uuid, QuantityBusEntry>>>;

pub struct QuantityBusEntry {
    pub registration: &'static str,  // "length", "velocity", etc.
    pub si_value: f64,               // always SI base units
}
```

### Per-registration tools (18 registrations × ~2 tools = 36)

Each registration `{name}` generates:

```
{name}__new(value: f64, unit: String) → Uuid
{name}__emit(ids: Vec<Uuid>) → String    // Rust source using uom::si::f64::{Type}
```

Example for "length":

```
length__new(100.0, "foot") → uuid-1     // stores Length::new::<foot>(100.0) → typed HashMap
length__emit([uuid-1]) → "let length_0 = Length::new::<foot>(100.0);"
```

### Shared query tools (5)

```
qty__value(id: Uuid, unit: String) → f64    // from bus: si_value converted to requested unit
qty__describe(id: Uuid) → JSON              // registration, si_value, human display
qty__list() → Vec<{id, registration, si_value}>
qty__delete(id: Uuid) → bool
qty__registration(id: Uuid) → String        // "length", "mass", etc.
```

### Arithmetic tools (12, via QuantityBus)

```
qty__add(id1: Uuid, id2: Uuid) → Uuid      // same registration required
qty__sub(id1: Uuid, id2: Uuid) → Uuid
qty__mul(id1: Uuid, id2: Uuid) → Uuid      // Length × Length → Area (bus routing)
qty__div(id1: Uuid, id2: Uuid) → Uuid      // Length / Time → Velocity
qty__neg(id: Uuid) → Uuid
qty__abs(id: Uuid) → Uuid
qty__scale(id: Uuid, factor: f64) → Uuid
qty__recip(id: Uuid) → Uuid                // 1/Frequency → Time
qty__sqrt(id: Uuid) → Uuid                 // √Area → Length
qty__powi(id: Uuid, n: i32) → Uuid         // Length² → Area
qty__compare(id1: Uuid, id2: Uuid) → Ordering
qty__approx_eq(id1: Uuid, id2: Uuid, eps: f64) → bool
```

Arithmetic resolves result registration from the dimension derivation table.
Unknown combinations return an error with the dimensional explanation.

### Convert tool (1)

```
qty__convert(id: Uuid, to_unit: String) → f64   // SI value converted to requested unit
```

---

## Plugin 2: `UomCodePlugin` (~15 tools)

No registry. Pure code generation + static catalog.

### Emit tools (5)

```
code__emit_conversion(quantity_type, value, from_unit, to_unit) → String
  // → "let x = Length::new::<foot>(100.0); println!("{} m", x.get::<meter>());"

code__emit_calculation(steps: Vec<UomStep>) → String
  // → full fn body with typed lets and println

code__emit_physics_formula(formula: UomFormula) → String
  // → named formula: kinetic_energy, ohms_law, ideal_gas, etc.

code__emit_main(steps: Vec<UomStep>) → String
  // → complete main.rs with use statements, calculation chain, output

code__emit_snippet(id_sequence: Vec<Uuid>) → String
  // Replay qty__ operations from registry as Rust source
  // (parallel code-tracking like PolarsExprPlugin)
```

### Catalog tools (10)

```
catalog__quantities() → Vec<{name, dimension, si_unit, si_abbreviation}>
catalog__units(quantity_type) → Vec<{name, abbreviation, singular, plural, factor}>
catalog__unit_info(quantity_type, unit) → {abbreviation, singular, plural, factor_to_si}
catalog__suggest_unit(quantity_type, si_value) → String  // human-appropriate scale
catalog__conversion_table(quantity_type, value, from_unit) → Vec<{unit, value}>
catalog__formula_list() → Vec<{name, description, inputs, output}>
catalog__dimension(quantity_type) → String  // "L¹ M⁰ T⁻¹ ..."
catalog__compatible_units(id: Uuid) → Vec<String>  // units this quantity can convert to
catalog__physics_constants() → Vec<{name, symbol, value, unit, quantity_type}>
catalog__base_quantities() → Vec<{name, si_unit}>
```

### Physics constants (via `catalog__physics_constants`)

| Constant | Value | Units |
|---|---|---|
| Speed of light | 2.997924×10⁸ | m/s |
| Gravitational constant | 6.674×10⁻¹¹ | m³/(kg⋅s²) |
| Planck constant | 6.626×10⁻³⁴ | J⋅s |
| Boltzmann constant | 1.380649×10⁻²³ | J/K |
| Avogadro constant | 6.02214×10²³ | mol⁻¹ |
| Elementary charge | 1.602176×10⁻¹⁹ | C |
| Standard gravity | 9.80665 | m/s² |

### Named physics formulas (via `code__emit_physics_formula`)

- `KineticEnergy`: E = ½mv²
- `GravitationalPE`: E = mgh
- `OhmsLaw`: V = IR
- `IdealGas`: PV = nRT
- `NewtonSecondLaw`: F = ma
- `Momentum`: p = mv
- `Power`: P = Fv = W/t
- `Pressure`: P = F/A
- `Frequency`: f = 1/T

---

## Code Generation Output Style

Emitted programs use `uom::si::f64::*` (the ergonomic prelude):

```rust
use uom::si::f64::*;
use uom::si::length::{meter, foot, kilometer};
use uom::si::time::second;
use uom::si::velocity::meter_per_second;
use uom::fmt::DisplayStyle::Abbreviation;

fn main() {
    let distance = Length::new::<meter>(100.0);
    let time = Time::new::<second>(9.58);
    let speed: Velocity = distance / time;
    println!(
        "{}",
        speed.into_format_args(meter_per_second, Abbreviation)
    );
    // Output: "10.438... m/s"
}
```

Cargo.toml snippet emitted alongside:

```toml
[dependencies]
uom = { version = "0.38", default-features = true }
```

---

## Parallel Code Tracking (code__emit_snippet)

Like `PolarsExprPlugin`, the `UomQuantityPlugin` optionally tracks the Rust code equivalent of each operation alongside the registry entry:

```rust
struct UomRegistryEntry {
    value: UomValue,
    code: String,   // e.g. "Length::new::<foot>(100.0)"
    var_name: String,  // e.g. "qty_0"
}
```

`code__emit_snippet(ids)` reconstructs the full Rust program by replaying these code fragments in dependency order.

---

## Cargo.toml

```toml
[package]
name = "elicit_uom"
version = "0.10.0"
edition = "2021"

[dependencies]
uom = { version = "0.38", features = ["serde", "f32", "f64", "si", "autoconvert", "std"] }
elicitation = { workspace = true }
tokio = { workspace = true }
uuid = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
schemars = { workspace = true }
tracing = { workspace = true }
rmcp = { workspace = true }
```

---

## elicitation Primitives (feature: `uom-types`)

```
crates/elicitation/src/primitives/uom_types/
├── mod.rs
├── enums.rs        # UomQuantityKind, UomUnitSystem, UomValueType (for catalog/emit)
└── descriptors.rs  # UomStep, UomFormula (for pipeline/emit)
```

Note: no `UomValue` enum needed — the factory pattern stores actual `uom::si::f64::*` types
in typed HashMaps. The primitives here are for the catalog and code-generation tools only.

```rust
/// Which dimension a quantity belongs to (for catalog, bus routing, error messages).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema,
         strum::EnumIter, derive_more::Display, ToCodeLiteral)]
pub enum UomQuantityKind {
    Length, Mass, Time, Temperature, ElectricCurrent,
    AmountOfSubstance, LuminousIntensity,
    Velocity, Acceleration, Force, Energy, Power, Pressure,
    Frequency, Area, Volume, Density, Angle,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct UomStep {
    pub registration: String,  // "length", "velocity", etc.
    pub operation: String,     // "new(100.0, meter)", "div(id1, id2)", etc.
    pub output_var: String,    // emitted variable name
}
```

---

## Testing Plan

| Test | Coverage |
|---|---|
| `test_length_new_and_convert` | meter↔foot, meter↔kilometer |
| `test_temperature_scales` | K↔°C↔°F roundtrip |
| `test_velocity_derivation` | Length/Time → Velocity mul/div |
| `test_force_derivation` | Mass × Acceleration → Force |
| `test_energy_derivation` | Force × Length → Energy |
| `test_add_homogeneous` | Length + Length |
| `test_add_heterogeneous_error` | Length + Mass → error |
| `test_emit_main` | complete Rust program output |
| `test_emit_physics_formula` | kinetic energy formula |
| `test_catalog_units` | list_units(length) includes meter, foot |
| `test_catalog_physics_constants` | speed of light has correct value |
| `test_code_snippet_roundtrip` | qty operations → emitted code |

---

## Sequence Diagram

```
Agent                      UomQuantityPlugin        UomCodePlugin
  │                              │                        │
  │─ qty__length_new(100, foot) ─>│                        │
  │<─ UUID-1 ─────────────────────│                        │
  │                              │                        │
  │─ qty__time_new(9.58, second) ─>│                       │
  │<─ UUID-2 ──────────────────────│                       │
  │                              │                        │
  │─ qty__div(UUID-1, UUID-2) ───>│                        │
  │  [Length/Time = Velocity]    │                        │
  │<─ UUID-3 ─────────────────────│                        │
  │                              │                        │
  │─ qty__value(UUID-3, mph) ────>│                        │
  │<─ 21.38 (miles per hour) ─────│                        │
  │                              │                        │
  │─ code__emit_snippet([1,2,3]) ─────────────────────────>│
  │<─ complete Rust source ────────────────────────────────│
```

---

## Status

- [ ] elicitation primitives (`uom-types` feature)
- [ ] `UomQuantityPlugin` (18 creation + 5 query + 12 arithmetic + 1 convert = 36 tools)
- [ ] `UomCodePlugin` (5 emit + 10 catalog = 15 tools)
- [ ] Integration tests (12)
- [ ] README
