# elicit_uom

MCP shadow crate for [`uom`](https://crates.io/crates/uom) 0.38 — provides ~55 Model
Context Protocol tools across two plugins for dimension-safe quantity arithmetic,
unit conversion, and Rust code generation.

## Plugins

### `UomQuantityPlugin` — `uom_{registration}__*` + `uom_qty__*` (~40 tools)

Stores live `uom::si::f64::*` quantities server-side using the Phase 3F.6 multi-parameter
factory pattern. Every registration gets its own typed `HashMap<Uuid, Q>` — no type
erasure. A shared **QuantityBus** enables cross-registration arithmetic (`Length / Time →
Velocity`) by routing through SI f64 values and the dimension derivation table.

### `UomCodePlugin` — `uom_code__*` (~15 tools)

Pure code generation and catalog browsing. Uses the shared QuantityBus so emitted
code reflects quantities created by `UomQuantityPlugin`.

## Usage

```rust,no_run
use elicit_uom::{UomQuantityPlugin, UomCodePlugin};

let qty = UomQuantityPlugin::new();
let code = UomCodePlugin::with_bus(qty.bus());
// Register both plugins with your MCP server
```

---

## `UomQuantityPlugin` tools

### Per-registration tools (18 quantities × 2 = 36 tools)

Each registered quantity `{reg}` exposes two tools:

#### `uom_{reg}__new`

Create a new quantity from a value and unit string. Returns a UUID for subsequent use.

| Param | Type | Description |
|---|---|---|
| `value` | `f64` | Numeric value in the given unit |
| `unit` | `String` | Unit string (e.g. `"kilometer"`, `"foot"`) |

**Returns** `{ id: String, registration: String, si_value: f64, code_snippet: String }`

#### `uom_{reg}__emit`

Emit Rust source code for one or more stored quantities of this type.

| Param | Type | Description |
|---|---|---|
| `ids` | `Vec<String>` | UUIDs of quantities to include |

**Returns** a Rust code string using `uom::si::f64::*`

---

### Registered quantities (18)

| Registration | uom Rust type | SI base unit | Example units |
|---|---|---|---|
| `length` | `Length` | meter | meter, kilometer, foot, inch, yard, mile, nautical_mile, light_year |
| `mass` | `Mass` | kilogram | kilogram, gram, pound, ounce, tonne |
| `time` | `Time` | second | second, millisecond, minute, hour, day, year |
| `temperature` | `ThermodynamicTemperature` | kelvin | kelvin, degree_celsius, degree_fahrenheit |
| `electric_current` | `ElectricCurrent` | ampere | ampere, milliampere, microampere |
| `amount_of_substance` | `AmountOfSubstance` | mole | mole, millimole |
| `luminous_intensity` | `LuminousIntensity` | candela | candela |
| `velocity` | `Velocity` | meter_per_second | meter_per_second, kilometer_per_hour, mile_per_hour, knot, speed_of_light |
| `acceleration` | `Acceleration` | meter_per_second_squared | meter_per_second_squared, foot_per_second_squared, standard_gravity |
| `force` | `Force` | newton | newton, kilonewton, pound_force, dyne |
| `energy` | `Energy` | joule | joule, kilojoule, kilocalorie, kilowatt_hour, electronvolt |
| `power` | `Power` | watt | watt, kilowatt, megawatt, horsepower |
| `pressure` | `Pressure` | pascal | pascal, kilopascal, bar, atmosphere, psi, torr |
| `frequency` | `Frequency` | hertz | hertz, kilohertz, megahertz, gigahertz |
| `area` | `Area` | square_meter | square_meter, hectare, acre, square_foot, square_mile |
| `volume` | `Volume` | cubic_meter | cubic_meter, liter, milliliter, gallon, pint, fluid_ounce |
| `density` | `MassDensity` | kilogram_per_cubic_meter | kilogram_per_cubic_meter, gram_per_cubic_centimeter |
| `angle` | `Angle` | radian | radian, degree, revolution, arcminute, arcsecond |

---

### Shared query tools (5)

| Tool | Params | Description |
|---|---|---|
| `uom_qty__value` | `id, unit` | Extract value in any supported unit |
| `uom_qty__describe` | `id` | Registration, SI value, SI unit, code snippet |
| `uom_qty__list` | — | All stored quantities with registration and SI value |
| `uom_qty__delete` | `id` | Remove a stored quantity |
| `uom_qty__registration` | `id` | Get registration name (`"length"`, `"mass"`, etc.) |

### Arithmetic tools (12, via QuantityBus)

Dimension-aware: `add`/`sub` require matching registrations; `mul`/`div` derive the
result registration from the dimension table (e.g. `length / time → velocity`).

| Tool | Params | Description |
|---|---|---|
| `uom_qty__add` | `lhs_id, rhs_id` | Add two same-registration quantities |
| `uom_qty__sub` | `lhs_id, rhs_id` | Subtract |
| `uom_qty__mul` | `lhs_id, rhs_id` | Multiply — derives result (length × length → area) |
| `uom_qty__div` | `lhs_id, rhs_id` | Divide — derives result (length / time → velocity) |
| `uom_qty__neg` | `id` | Negate |
| `uom_qty__abs` | `id` | Absolute value |
| `uom_qty__scale` | `id, factor: f64` | Multiply by dimensionless scalar |
| `uom_qty__recip` | `id` | 1/x (frequency → time, etc.) |
| `uom_qty__sqrt` | `id` | Square root (area → length) |
| `uom_qty__powi` | `id, n: i32` | Integer power (length² → area) |
| `uom_qty__compare` | `lhs_id, rhs_id` | Returns `"less"`, `"equal"`, or `"greater"` |
| `uom_qty__approx_eq` | `lhs_id, rhs_id, tolerance?` | Relative equality check |

**Dimension derivation table:**

| Operation | LHS | RHS | Result |
|---|---|---|---|
| mul | length | length | area |
| mul | area | length | volume |
| mul | mass | acceleration | force |
| mul | force | length | energy |
| mul | velocity | time | length |
| div | length | time | velocity |
| div | velocity | time | acceleration |
| div | energy | time | power |
| div | force | area | pressure |
| div | mass | volume | density |
| sqrt | area | — | length |
| powi(2) | length | — | area |
| recip | frequency | — | time |

### Convert tool (1)

| Tool | Params | Description |
|---|---|---|
| `uom_qty__convert` | `id, to_unit` | Value in target unit (no new UUID) |

---

## `UomCodePlugin` tools

### Emit tools (5)

| Tool | Params | Description |
|---|---|---|
| `uom_code__emit_conversion` | `registration, value, from_unit, to_unit, var_name?` | Emit a single unit-conversion snippet |
| `uom_code__emit_calculation` | `lhs_id, op, rhs_id, result_var?` | Emit arithmetic using stored quantities (`op`: `add`, `sub`, `mul`, `div`) |
| `uom_code__emit_physics_formula` | `formula` | Emit a named physics formula stub |
| `uom_code__emit_main` | `ids, display_style?` | Emit a complete `fn main()` for the given quantity UUIDs |
| `uom_code__emit_snippet` | `ids, units?` | Emit let-bindings for one or more quantities |

**Supported `formula` values:** `KineticEnergy`, `GravitationalPE`, `OhmsLaw`,
`IdealGas`, `Momentum`

**`display_style`:** `"abbreviation"` (default, e.g. `100 km`) or `"description"`
(e.g. `100 kilometers`)

### Catalog tools (10)

| Tool | Params | Description |
|---|---|---|
| `uom_code__catalog_quantities` | `filter?` | List all 18 registrations (`"base"`, `"derived"`, or `"all"`) |
| `uom_code__catalog_units` | `registration` | All supported unit strings for a registration |
| `uom_code__catalog_unit_info` | `unit` | SI factor, dimension, and which registrations accept this unit |
| `uom_code__catalog_suggest_unit` | `registration, system?` | Recommend a unit (`"si"`, `"imperial"`, or `"any"`) |
| `uom_code__catalog_conversion_table` | `registration, si_value` | Table of all units for the SI value |
| `uom_code__catalog_formula_list` | — | All supported physics formulas with parameter descriptions |
| `uom_code__catalog_dimension` | `registration` | Dimensional analysis string (e.g. `L T⁻¹` for velocity) |
| `uom_code__catalog_compatible_units` | `registration, unit` | Units from the same measurement system |
| `uom_code__catalog_physics_constants` | — | Speed of light, G, h, kB, NA, e, g, ε0 |
| `uom_code__catalog_base_quantities` | — | The 7 ISQ base quantities with SI units |

---

## Example workflow

```text
1. uom_length__new(100.0, "kilometer")   → uuid-A
2. uom_time__new(2.0, "hour")            → uuid-B
3. uom_qty__div(uuid-A, uuid-B)          → uuid-C  (Velocity, 13.888... m/s)
4. uom_qty__convert(uuid-C, "kilometer_per_hour")  → 50.0
5. uom_velocity__emit([uuid-C])          → Rust code string
6. uom_code__emit_main([uuid-A, uuid-B, uuid-C])   → complete fn main()
```

---

## Architecture

- **Multi-parameter factory (Phase 3F.6)**: `register_quantity::<uom::si::f64::Length>("length")`
  creates a typed `HashMap<Uuid, Length>`. No type erasure — `uom_length__emit` knows the
  exact `(D, U, V)` triple and generates perfect `Length::new::<kilometer>(100.0)` code.
- **QuantityBus**: `Arc<Mutex<HashMap<Uuid, QuantityBusEntry { registration, si_value }>>>`
  shared between all registrations and `UomCodePlugin`. Enables cross-registration
  arithmetic (different typed HashMaps, same bus) and code emission from any plugin.
- **Dimension table** (`src/dimension.rs`): pure function mapping `(lhs_reg, op, rhs_reg) →
  result_reg`. Arithmetic errors include dimensional context.
- **uom internals**: all values stored in SI base units. Unit conversion via
  `qty.get::<foot>()` calls uom's internal conversion factors — no manual constants.

## Cargo features

```toml
elicit_uom = { version = "...", features = ["emit"] }
```

The `emit` feature gates code-generation helpers in the `elicitation` primitives crate.
