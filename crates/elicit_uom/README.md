# elicit_uom

MCP tools for the [`uom`](https://crates.io/crates/uom) units-of-measurement library.

## Plugins

- **`UomQuantityPlugin`** — create typed quantities, do arithmetic, convert units
- **`UomCodePlugin`** — emit Rust code, browse the unit catalog

## Usage

```rust,no_run
use elicit_uom::{UomQuantityPlugin, UomCodePlugin};

let qty = UomQuantityPlugin::new();
let code = UomCodePlugin::with_bus(qty.bus());
```

## Registered quantities (18)

| Registration | Rust type | SI unit |
|---|---|---|
| `length` | `Length` | meter |
| `mass` | `Mass` | kilogram |
| `time` | `Time` | second |
| `temperature` | `ThermodynamicTemperature` | kelvin |
| `electric_current` | `ElectricCurrent` | ampere |
| `amount_of_substance` | `AmountOfSubstance` | mole |
| `luminous_intensity` | `LuminousIntensity` | candela |
| `velocity` | `Velocity` | meter_per_second |
| `acceleration` | `Acceleration` | meter_per_second_squared |
| `force` | `Force` | newton |
| `energy` | `Energy` | joule |
| `power` | `Power` | watt |
| `pressure` | `Pressure` | pascal |
| `frequency` | `Frequency` | hertz |
| `area` | `Area` | square_meter |
| `volume` | `Volume` | cubic_meter |
| `density` | `MassDensity` | kilogram_per_cubic_meter |
| `angle` | `Angle` | radian |
