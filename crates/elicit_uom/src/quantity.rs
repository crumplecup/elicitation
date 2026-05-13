//! `UomQuantityPlugin` — MCP tools for uom quantity creation, arithmetic, and conversion.
//!
//! # Tool namespace: `uom_qty__*` and per-registration `uom_{name}__*`

use std::collections::HashMap;
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
use uuid::Uuid;

use crate::{
    QuantityBus, QuantityBusEntry, UomError, UomErrorKind, derive_div, derive_mul, derive_pow,
    derive_recip, derive_sqrt, new_bus,
};

// ── Helpers ───────────────────────────────────────────────────────────────────

fn tool_err(msg: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(msg.to_string(), None)
}

fn ok_json<T: Serialize>(v: &T) -> Result<CallToolResult, ErrorData> {
    serde_json::to_string(v)
        .map(|s| CallToolResult::success(vec![Content::text(s)]))
        .map_err(|e| tool_err(format!("serialise: {e}")))
}

fn uom_err_to_mcp(e: UomError) -> ErrorData {
    ErrorData::invalid_params(e.to_string(), None)
}

// ── Per-registration unit parsing ─────────────────────────────────────────────

/// Parse a unit string for `length` and return the SI value in metres.
fn parse_length(value: f64, unit: &str) -> Result<(f64, String), UomError> {
    use uom::si::f64::Length;
    use uom::si::length::*;
    let q = match unit {
        "meter" | "metre" | "m" => Length::new::<meter>(value),
        "kilometer" | "kilometre" | "km" => Length::new::<kilometer>(value),
        "centimeter" | "centimetre" | "cm" => Length::new::<centimeter>(value),
        "millimeter" | "millimetre" | "mm" => Length::new::<millimeter>(value),
        "micrometer" | "micrometre" | "um" | "μm" => Length::new::<micrometer>(value),
        "nanometer" | "nanometre" | "nm" => Length::new::<nanometer>(value),
        "foot" | "ft" => Length::new::<foot>(value),
        "inch" | "in" => Length::new::<inch>(value),
        "yard" | "yd" => Length::new::<yard>(value),
        "mile" | "mi" => Length::new::<mile>(value),
        "nautical_mile" | "nmi" => Length::new::<nautical_mile>(value),
        "astronomical_unit" | "au" => Length::new::<astronomical_unit>(value),
        "light_year" | "ly" => Length::new::<light_year>(value),
        _ => {
            return Err(UomError::new(UomErrorKind::UnknownUnit {
                unit: unit.to_string(),
                registration: "length".to_string(),
            }));
        }
    };
    let si = q.get::<meter>();
    let snippet = format!(
        "let length = Length::new::<{unit}>({value});",
        unit = unit,
        value = value
    );
    Ok((si, snippet))
}

fn parse_mass(value: f64, unit: &str) -> Result<(f64, String), UomError> {
    use uom::si::f64::Mass;
    use uom::si::mass::*;
    let q = match unit {
        "kilogram" | "kg" => Mass::new::<kilogram>(value),
        "gram" | "g" => Mass::new::<gram>(value),
        "milligram" | "mg" => Mass::new::<milligram>(value),
        "microgram" | "ug" | "μg" => Mass::new::<microgram>(value),
        "tonne" | "t" => Mass::new::<ton>(value),
        "pound" | "lb" => Mass::new::<pound>(value),
        "ounce" | "oz" => Mass::new::<ounce>(value),
        "ton" => Mass::new::<ton_short>(value),
        _ => {
            return Err(UomError::new(UomErrorKind::UnknownUnit {
                unit: unit.to_string(),
                registration: "mass".to_string(),
            }));
        }
    };
    let si = q.get::<kilogram>();
    Ok((si, format!("let mass = Mass::new::<{unit}>({value});")))
}

fn parse_time(value: f64, unit: &str) -> Result<(f64, String), UomError> {
    use uom::si::f64::Time;
    use uom::si::time::*;
    let q = match unit {
        "second" | "s" => Time::new::<second>(value),
        "millisecond" | "ms" => Time::new::<millisecond>(value),
        "microsecond" | "us" | "μs" => Time::new::<microsecond>(value),
        "nanosecond" | "ns" => Time::new::<nanosecond>(value),
        "minute" | "min" => Time::new::<minute>(value),
        "hour" | "h" => Time::new::<hour>(value),
        "day" | "d" => Time::new::<day>(value),
        "week" | "wk" => Time::new::<second>(value * 604_800.0),
        "year" | "yr" => Time::new::<year>(value),
        _ => {
            return Err(UomError::new(UomErrorKind::UnknownUnit {
                unit: unit.to_string(),
                registration: "time".to_string(),
            }));
        }
    };
    let si = q.get::<second>();
    Ok((si, format!("let time = Time::new::<{unit}>({value});")))
}

fn parse_temperature(value: f64, unit: &str) -> Result<(f64, String), UomError> {
    use uom::si::f64::ThermodynamicTemperature;
    use uom::si::thermodynamic_temperature::*;
    let q = match unit {
        "kelvin" | "K" => ThermodynamicTemperature::new::<kelvin>(value),
        "degree_celsius" | "celsius" | "°C" | "degC" | "C" => {
            ThermodynamicTemperature::new::<degree_celsius>(value)
        }
        "degree_fahrenheit" | "fahrenheit" | "°F" | "degF" | "F" => {
            ThermodynamicTemperature::new::<degree_fahrenheit>(value)
        }
        _ => {
            return Err(UomError::new(UomErrorKind::UnknownUnit {
                unit: unit.to_string(),
                registration: "temperature".to_string(),
            }));
        }
    };
    let si = q.get::<kelvin>();
    Ok((
        si,
        format!("let temperature = ThermodynamicTemperature::new::<{unit}>({value});"),
    ))
}

fn parse_electric_current(value: f64, unit: &str) -> Result<(f64, String), UomError> {
    use uom::si::electric_current::*;
    use uom::si::f64::ElectricCurrent;
    let q = match unit {
        "ampere" | "A" => ElectricCurrent::new::<ampere>(value),
        "milliampere" | "mA" => ElectricCurrent::new::<milliampere>(value),
        "microampere" | "uA" | "μA" => ElectricCurrent::new::<microampere>(value),
        _ => {
            return Err(UomError::new(UomErrorKind::UnknownUnit {
                unit: unit.to_string(),
                registration: "electric_current".to_string(),
            }));
        }
    };
    let si = q.get::<ampere>();
    Ok((
        si,
        format!("let electric_current = ElectricCurrent::new::<{unit}>({value});"),
    ))
}

fn parse_amount_of_substance(value: f64, unit: &str) -> Result<(f64, String), UomError> {
    use uom::si::amount_of_substance::*;
    use uom::si::f64::AmountOfSubstance;
    let q = match unit {
        "mole" | "mol" => AmountOfSubstance::new::<mole>(value),
        "millimole" | "mmol" => AmountOfSubstance::new::<millimole>(value),
        _ => {
            return Err(UomError::new(UomErrorKind::UnknownUnit {
                unit: unit.to_string(),
                registration: "amount_of_substance".to_string(),
            }));
        }
    };
    let si = q.get::<mole>();
    Ok((
        si,
        format!("let amount_of_substance = AmountOfSubstance::new::<{unit}>({value});"),
    ))
}

fn parse_luminous_intensity(value: f64, unit: &str) -> Result<(f64, String), UomError> {
    use uom::si::f64::LuminousIntensity;
    use uom::si::luminous_intensity::*;
    let q = match unit {
        "candela" | "cd" => LuminousIntensity::new::<candela>(value),
        _ => {
            return Err(UomError::new(UomErrorKind::UnknownUnit {
                unit: unit.to_string(),
                registration: "luminous_intensity".to_string(),
            }));
        }
    };
    let si = q.get::<candela>();
    Ok((
        si,
        format!("let luminous_intensity = LuminousIntensity::new::<{unit}>({value});"),
    ))
}

fn parse_velocity(value: f64, unit: &str) -> Result<(f64, String), UomError> {
    use uom::si::f64::Velocity;
    use uom::si::velocity::*;
    let q = match unit {
        "meter_per_second" | "m/s" | "mps" => Velocity::new::<meter_per_second>(value),
        "kilometer_per_hour" | "km/h" | "kph" => Velocity::new::<kilometer_per_hour>(value),
        "mile_per_hour" | "mph" => Velocity::new::<mile_per_hour>(value),
        "foot_per_second" | "ft/s" | "fps" => Velocity::new::<foot_per_second>(value),
        "knot" | "kn" => Velocity::new::<knot>(value),
        "speed_of_light" | "c" => Velocity::new::<speed_of_light_in_vacuum>(value),
        _ => {
            return Err(UomError::new(UomErrorKind::UnknownUnit {
                unit: unit.to_string(),
                registration: "velocity".to_string(),
            }));
        }
    };
    let si = q.get::<meter_per_second>();
    Ok((
        si,
        format!("let velocity = Velocity::new::<{unit}>({value});"),
    ))
}

fn parse_acceleration(value: f64, unit: &str) -> Result<(f64, String), UomError> {
    use uom::si::acceleration::*;
    use uom::si::f64::Acceleration;
    let q = match unit {
        "meter_per_second_squared" | "m/s2" | "m/s²" => {
            Acceleration::new::<meter_per_second_squared>(value)
        }
        "foot_per_second_squared" | "ft/s2" | "ft/s²" => {
            Acceleration::new::<foot_per_second_squared>(value)
        }
        "standard_gravity" | "g" | "G" => Acceleration::new::<standard_gravity>(value),
        _ => {
            return Err(UomError::new(UomErrorKind::UnknownUnit {
                unit: unit.to_string(),
                registration: "acceleration".to_string(),
            }));
        }
    };
    let si = q.get::<meter_per_second_squared>();
    Ok((
        si,
        format!("let acceleration = Acceleration::new::<{unit}>({value});"),
    ))
}

fn parse_force(value: f64, unit: &str) -> Result<(f64, String), UomError> {
    use uom::si::f64::Force;
    use uom::si::force::*;
    let q = match unit {
        "newton" | "N" => Force::new::<newton>(value),
        "kilonewton" | "kN" => Force::new::<kilonewton>(value),
        "meganewton" | "MN" => Force::new::<meganewton>(value),
        "pound_force" | "lbf" => Force::new::<pound_force>(value),
        "dyne" | "dyn" => Force::new::<dyne>(value),
        _ => {
            return Err(UomError::new(UomErrorKind::UnknownUnit {
                unit: unit.to_string(),
                registration: "force".to_string(),
            }));
        }
    };
    let si = q.get::<newton>();
    Ok((si, format!("let force = Force::new::<{unit}>({value});")))
}

fn parse_energy(value: f64, unit: &str) -> Result<(f64, String), UomError> {
    use uom::si::energy::*;
    use uom::si::f64::Energy;
    let q = match unit {
        "joule" | "J" => Energy::new::<joule>(value),
        "kilojoule" | "kJ" => Energy::new::<kilojoule>(value),
        "megajoule" | "MJ" => Energy::new::<megajoule>(value),
        "calorie" | "cal" => Energy::new::<calorie>(value),
        "kilocalorie" | "kcal" => Energy::new::<kilocalorie>(value),
        "kilowatt_hour" | "kWh" => Energy::new::<kilowatt_hour>(value),
        "electronvolt" | "eV" => Energy::new::<electronvolt>(value),
        _ => {
            return Err(UomError::new(UomErrorKind::UnknownUnit {
                unit: unit.to_string(),
                registration: "energy".to_string(),
            }));
        }
    };
    let si = q.get::<joule>();
    Ok((si, format!("let energy = Energy::new::<{unit}>({value});")))
}

fn parse_power(value: f64, unit: &str) -> Result<(f64, String), UomError> {
    use uom::si::f64::Power;
    use uom::si::power::*;
    let q = match unit {
        "watt" | "W" => Power::new::<watt>(value),
        "kilowatt" | "kW" => Power::new::<kilowatt>(value),
        "megawatt" | "MW" => Power::new::<megawatt>(value),
        "horsepower" | "hp" => Power::new::<horsepower>(value),
        "milliwatt" | "mW" => Power::new::<milliwatt>(value),
        _ => {
            return Err(UomError::new(UomErrorKind::UnknownUnit {
                unit: unit.to_string(),
                registration: "power".to_string(),
            }));
        }
    };
    let si = q.get::<watt>();
    Ok((si, format!("let power = Power::new::<{unit}>({value});")))
}

fn parse_pressure(value: f64, unit: &str) -> Result<(f64, String), UomError> {
    use uom::si::f64::Pressure;
    use uom::si::pressure::*;
    let q = match unit {
        "pascal" | "Pa" => Pressure::new::<pascal>(value),
        "kilopascal" | "kPa" => Pressure::new::<kilopascal>(value),
        "megapascal" | "MPa" => Pressure::new::<megapascal>(value),
        "bar" => Pressure::new::<bar>(value),
        "atmosphere" | "atm" => Pressure::new::<atmosphere>(value),
        "psi" => Pressure::new::<psi>(value),
        "torr" => Pressure::new::<torr>(value),
        _ => {
            return Err(UomError::new(UomErrorKind::UnknownUnit {
                unit: unit.to_string(),
                registration: "pressure".to_string(),
            }));
        }
    };
    let si = q.get::<pascal>();
    Ok((
        si,
        format!("let pressure = Pressure::new::<{unit}>({value});"),
    ))
}

fn parse_frequency(value: f64, unit: &str) -> Result<(f64, String), UomError> {
    use uom::si::f64::Frequency;
    use uom::si::frequency::*;
    let q = match unit {
        "hertz" | "Hz" => Frequency::new::<hertz>(value),
        "kilohertz" | "kHz" => Frequency::new::<kilohertz>(value),
        "megahertz" | "MHz" => Frequency::new::<megahertz>(value),
        "gigahertz" | "GHz" => Frequency::new::<gigahertz>(value),
        "terahertz" | "THz" => Frequency::new::<terahertz>(value),
        _ => {
            return Err(UomError::new(UomErrorKind::UnknownUnit {
                unit: unit.to_string(),
                registration: "frequency".to_string(),
            }));
        }
    };
    let si = q.get::<hertz>();
    Ok((
        si,
        format!("let frequency = Frequency::new::<{unit}>({value});"),
    ))
}

fn parse_area(value: f64, unit: &str) -> Result<(f64, String), UomError> {
    use uom::si::area::*;
    use uom::si::f64::Area;
    let q = match unit {
        "square_meter" | "square_metre" | "m2" | "m²" => Area::new::<square_meter>(value),
        "square_kilometer" | "square_kilometre" | "km2" | "km²" => {
            Area::new::<square_kilometer>(value)
        }
        "square_centimeter" | "square_centimetre" | "cm2" | "cm²" => {
            Area::new::<square_centimeter>(value)
        }
        "square_millimeter" | "square_millimetre" | "mm2" | "mm²" => {
            Area::new::<square_millimeter>(value)
        }
        "hectare" | "ha" => Area::new::<hectare>(value),
        "acre" | "ac" => Area::new::<acre>(value),
        "square_foot" | "sq_ft" | "ft2" | "ft²" => Area::new::<square_foot>(value),
        "square_inch" | "sq_in" | "in2" | "in²" => Area::new::<square_inch>(value),
        "square_yard" | "sq_yd" | "yd2" | "yd²" => Area::new::<square_yard>(value),
        "square_mile" | "sq_mi" | "mi2" | "mi²" => Area::new::<square_mile>(value),
        _ => {
            return Err(UomError::new(UomErrorKind::UnknownUnit {
                unit: unit.to_string(),
                registration: "area".to_string(),
            }));
        }
    };
    let si = q.get::<square_meter>();
    Ok((si, format!("let area = Area::new::<{unit}>({value});")))
}

fn parse_volume(value: f64, unit: &str) -> Result<(f64, String), UomError> {
    use uom::si::f64::Volume;
    use uom::si::volume::*;
    let q = match unit {
        "cubic_meter" | "cubic_metre" | "m3" | "m³" => Volume::new::<cubic_meter>(value),
        "liter" | "litre" | "L" => Volume::new::<liter>(value),
        "milliliter" | "millilitre" | "mL" | "ml" => Volume::new::<milliliter>(value),
        "cubic_centimeter" | "cubic_centimetre" | "cm3" | "cm³" => {
            Volume::new::<cubic_centimeter>(value)
        }
        "cubic_foot" | "ft3" | "ft³" => Volume::new::<cubic_foot>(value),
        "cubic_inch" | "in3" | "in³" => Volume::new::<cubic_inch>(value),
        "gallon" | "gal" => Volume::new::<gallon>(value),
        "quart" | "qt" => Volume::new::<quart_liquid>(value),
        "pint" | "pt" => Volume::new::<pint_liquid>(value),
        "fluid_ounce" | "fl_oz" | "floz" => Volume::new::<fluid_ounce>(value),
        _ => {
            return Err(UomError::new(UomErrorKind::UnknownUnit {
                unit: unit.to_string(),
                registration: "volume".to_string(),
            }));
        }
    };
    let si = q.get::<cubic_meter>();
    Ok((si, format!("let volume = Volume::new::<{unit}>({value});")))
}

fn parse_density(value: f64, unit: &str) -> Result<(f64, String), UomError> {
    use uom::si::f64::MassDensity;
    use uom::si::mass_density::*;
    let q = match unit {
        "kilogram_per_cubic_meter" | "kg/m3" | "kg/m³" => {
            MassDensity::new::<kilogram_per_cubic_meter>(value)
        }
        "gram_per_cubic_centimeter" | "g/cm3" | "g/cm³" => {
            MassDensity::new::<gram_per_cubic_centimeter>(value)
        }
        "pound_per_cubic_foot" | "lb/ft3" | "lb/ft³" => {
            MassDensity::new::<pound_per_cubic_foot>(value)
        }
        _ => {
            return Err(UomError::new(UomErrorKind::UnknownUnit {
                unit: unit.to_string(),
                registration: "density".to_string(),
            }));
        }
    };
    let si = q.get::<kilogram_per_cubic_meter>();
    Ok((
        si,
        format!("let density = MassDensity::new::<{unit}>({value});"),
    ))
}

fn parse_angle(value: f64, unit: &str) -> Result<(f64, String), UomError> {
    use uom::si::angle::*;
    use uom::si::f64::Angle;
    let q = match unit {
        "radian" | "rad" => Angle::new::<radian>(value),
        "degree" | "deg" | "°" => Angle::new::<degree>(value),
        "revolution" | "rev" => Angle::new::<revolution>(value),
        "arcminute" | "'" => Angle::new::<minute>(value),
        "arcsecond" | "\"" => Angle::new::<second>(value),
        _ => {
            return Err(UomError::new(UomErrorKind::UnknownUnit {
                unit: unit.to_string(),
                registration: "angle".to_string(),
            }));
        }
    };
    let si = q.get::<radian>();
    Ok((si, format!("let angle = Angle::new::<{unit}>({value});")))
}

// ── Conversion helpers ────────────────────────────────────────────────────────

/// Convert a stored SI value to a target unit for a given registration.
pub fn convert_to_unit(registration: &str, si_value: f64, to_unit: &str) -> Result<f64, UomError> {
    match registration {
        "length" => {
            use uom::si::f64::Length;
            use uom::si::length::*;
            let q = Length::new::<meter>(si_value);
            match to_unit {
                "meter" | "metre" | "m" => Ok(q.get::<meter>()),
                "kilometer" | "kilometre" | "km" => Ok(q.get::<kilometer>()),
                "centimeter" | "centimetre" | "cm" => Ok(q.get::<centimeter>()),
                "millimeter" | "millimetre" | "mm" => Ok(q.get::<millimeter>()),
                "micrometer" | "micrometre" | "um" | "μm" => Ok(q.get::<micrometer>()),
                "nanometer" | "nanometre" | "nm" => Ok(q.get::<nanometer>()),
                "foot" | "ft" => Ok(q.get::<foot>()),
                "inch" | "in" => Ok(q.get::<inch>()),
                "yard" | "yd" => Ok(q.get::<yard>()),
                "mile" | "mi" => Ok(q.get::<mile>()),
                "nautical_mile" | "nmi" => Ok(q.get::<nautical_mile>()),
                "astronomical_unit" | "au" => Ok(q.get::<astronomical_unit>()),
                "light_year" | "ly" => Ok(q.get::<light_year>()),
                _ => Err(UomError::new(UomErrorKind::UnknownUnit {
                    unit: to_unit.to_string(),
                    registration: "length".to_string(),
                })),
            }
        }
        "mass" => {
            use uom::si::f64::Mass;
            use uom::si::mass::*;
            let q = Mass::new::<kilogram>(si_value);
            match to_unit {
                "kilogram" | "kg" => Ok(q.get::<kilogram>()),
                "gram" | "g" => Ok(q.get::<gram>()),
                "milligram" | "mg" => Ok(q.get::<milligram>()),
                "microgram" | "ug" | "μg" => Ok(q.get::<microgram>()),
                "tonne" | "t" => Ok(q.get::<ton>()),
                "pound" | "lb" => Ok(q.get::<pound>()),
                "ounce" | "oz" => Ok(q.get::<ounce>()),
                "ton" => Ok(q.get::<ton_short>()),
                _ => Err(UomError::new(UomErrorKind::UnknownUnit {
                    unit: to_unit.to_string(),
                    registration: "mass".to_string(),
                })),
            }
        }
        "time" => {
            use uom::si::f64::Time;
            use uom::si::time::*;
            let q = Time::new::<second>(si_value);
            match to_unit {
                "second" | "s" => Ok(q.get::<second>()),
                "millisecond" | "ms" => Ok(q.get::<millisecond>()),
                "microsecond" | "us" | "μs" => Ok(q.get::<microsecond>()),
                "nanosecond" | "ns" => Ok(q.get::<nanosecond>()),
                "minute" | "min" => Ok(q.get::<minute>()),
                "hour" | "h" => Ok(q.get::<hour>()),
                "day" | "d" => Ok(q.get::<day>()),
                "week" | "wk" => Ok(si_value / 604_800.0),
                "year" | "yr" => Ok(q.get::<year>()),
                _ => Err(UomError::new(UomErrorKind::UnknownUnit {
                    unit: to_unit.to_string(),
                    registration: "time".to_string(),
                })),
            }
        }
        "temperature" => {
            use uom::si::f64::ThermodynamicTemperature;
            use uom::si::thermodynamic_temperature::*;
            let q = ThermodynamicTemperature::new::<kelvin>(si_value);
            match to_unit {
                "kelvin" | "K" => Ok(q.get::<kelvin>()),
                "degree_celsius" | "celsius" | "°C" | "degC" | "C" => {
                    Ok(q.get::<degree_celsius>())
                }
                "degree_fahrenheit" | "fahrenheit" | "°F" | "degF" | "F" => {
                    Ok(q.get::<degree_fahrenheit>())
                }
                _ => Err(UomError::new(UomErrorKind::UnknownUnit {
                    unit: to_unit.to_string(),
                    registration: "temperature".to_string(),
                })),
            }
        }
        "velocity" => {
            use uom::si::f64::Velocity;
            use uom::si::velocity::*;
            let q = Velocity::new::<meter_per_second>(si_value);
            match to_unit {
                "meter_per_second" | "m/s" | "mps" => Ok(q.get::<meter_per_second>()),
                "kilometer_per_hour" | "km/h" | "kph" => Ok(q.get::<kilometer_per_hour>()),
                "mile_per_hour" | "mph" => Ok(q.get::<mile_per_hour>()),
                "foot_per_second" | "ft/s" | "fps" => Ok(q.get::<foot_per_second>()),
                "knot" | "kn" => Ok(q.get::<knot>()),
                "speed_of_light" | "c" => Ok(q.get::<speed_of_light_in_vacuum>()),
                _ => Err(UomError::new(UomErrorKind::UnknownUnit {
                    unit: to_unit.to_string(),
                    registration: "velocity".to_string(),
                })),
            }
        }
        "acceleration" => {
            use uom::si::acceleration::*;
            use uom::si::f64::Acceleration;
            let q = Acceleration::new::<meter_per_second_squared>(si_value);
            match to_unit {
                "meter_per_second_squared" | "m/s2" | "m/s²" => {
                    Ok(q.get::<meter_per_second_squared>())
                }
                "foot_per_second_squared" | "ft/s2" | "ft/s²" => {
                    Ok(q.get::<foot_per_second_squared>())
                }
                "standard_gravity" | "g" | "G" => Ok(q.get::<standard_gravity>()),
                _ => Err(UomError::new(UomErrorKind::UnknownUnit {
                    unit: to_unit.to_string(),
                    registration: "acceleration".to_string(),
                })),
            }
        }
        "force" => {
            use uom::si::f64::Force;
            use uom::si::force::*;
            let q = Force::new::<newton>(si_value);
            match to_unit {
                "newton" | "N" => Ok(q.get::<newton>()),
                "kilonewton" | "kN" => Ok(q.get::<kilonewton>()),
                "meganewton" | "MN" => Ok(q.get::<meganewton>()),
                "pound_force" | "lbf" => Ok(q.get::<pound_force>()),
                "dyne" | "dyn" => Ok(q.get::<dyne>()),
                _ => Err(UomError::new(UomErrorKind::UnknownUnit {
                    unit: to_unit.to_string(),
                    registration: "force".to_string(),
                })),
            }
        }
        "energy" => {
            use uom::si::energy::*;
            use uom::si::f64::Energy;
            let q = Energy::new::<joule>(si_value);
            match to_unit {
                "joule" | "J" => Ok(q.get::<joule>()),
                "kilojoule" | "kJ" => Ok(q.get::<kilojoule>()),
                "megajoule" | "MJ" => Ok(q.get::<megajoule>()),
                "calorie" | "cal" => Ok(q.get::<calorie>()),
                "kilocalorie" | "kcal" => Ok(q.get::<kilocalorie>()),
                "kilowatt_hour" | "kWh" => Ok(q.get::<kilowatt_hour>()),
                "electronvolt" | "eV" => Ok(q.get::<electronvolt>()),
                _ => Err(UomError::new(UomErrorKind::UnknownUnit {
                    unit: to_unit.to_string(),
                    registration: "energy".to_string(),
                })),
            }
        }
        "power" => {
            use uom::si::f64::Power;
            use uom::si::power::*;
            let q = Power::new::<watt>(si_value);
            match to_unit {
                "watt" | "W" => Ok(q.get::<watt>()),
                "kilowatt" | "kW" => Ok(q.get::<kilowatt>()),
                "megawatt" | "MW" => Ok(q.get::<megawatt>()),
                "horsepower" | "hp" => Ok(q.get::<horsepower>()),
                "milliwatt" | "mW" => Ok(q.get::<milliwatt>()),
                _ => Err(UomError::new(UomErrorKind::UnknownUnit {
                    unit: to_unit.to_string(),
                    registration: "power".to_string(),
                })),
            }
        }
        "pressure" => {
            use uom::si::f64::Pressure;
            use uom::si::pressure::*;
            let q = Pressure::new::<pascal>(si_value);
            match to_unit {
                "pascal" | "Pa" => Ok(q.get::<pascal>()),
                "kilopascal" | "kPa" => Ok(q.get::<kilopascal>()),
                "megapascal" | "MPa" => Ok(q.get::<megapascal>()),
                "bar" => Ok(q.get::<bar>()),
                "atmosphere" | "atm" => Ok(q.get::<atmosphere>()),
                "psi" => Ok(q.get::<psi>()),
                "torr" => Ok(q.get::<torr>()),
                _ => Err(UomError::new(UomErrorKind::UnknownUnit {
                    unit: to_unit.to_string(),
                    registration: "pressure".to_string(),
                })),
            }
        }
        "frequency" => {
            use uom::si::f64::Frequency;
            use uom::si::frequency::*;
            let q = Frequency::new::<hertz>(si_value);
            match to_unit {
                "hertz" | "Hz" => Ok(q.get::<hertz>()),
                "kilohertz" | "kHz" => Ok(q.get::<kilohertz>()),
                "megahertz" | "MHz" => Ok(q.get::<megahertz>()),
                "gigahertz" | "GHz" => Ok(q.get::<gigahertz>()),
                "terahertz" | "THz" => Ok(q.get::<terahertz>()),
                _ => Err(UomError::new(UomErrorKind::UnknownUnit {
                    unit: to_unit.to_string(),
                    registration: "frequency".to_string(),
                })),
            }
        }
        "area" => {
            use uom::si::area::*;
            use uom::si::f64::Area;
            let q = Area::new::<square_meter>(si_value);
            match to_unit {
                "square_meter" | "square_metre" | "m2" | "m²" => Ok(q.get::<square_meter>()),
                "square_kilometer" | "square_kilometre" | "km2" | "km²" => {
                    Ok(q.get::<square_kilometer>())
                }
                "square_centimeter" | "square_centimetre" | "cm2" | "cm²" => {
                    Ok(q.get::<square_centimeter>())
                }
                "square_millimeter" | "square_millimetre" | "mm2" | "mm²" => {
                    Ok(q.get::<square_millimeter>())
                }
                "hectare" | "ha" => Ok(q.get::<hectare>()),
                "acre" | "ac" => Ok(q.get::<acre>()),
                "square_foot" | "sq_ft" | "ft2" | "ft²" => Ok(q.get::<square_foot>()),
                "square_inch" | "sq_in" | "in2" | "in²" => Ok(q.get::<square_inch>()),
                "square_yard" | "sq_yd" | "yd2" | "yd²" => Ok(q.get::<square_yard>()),
                "square_mile" | "sq_mi" | "mi2" | "mi²" => Ok(q.get::<square_mile>()),
                _ => Err(UomError::new(UomErrorKind::UnknownUnit {
                    unit: to_unit.to_string(),
                    registration: "area".to_string(),
                })),
            }
        }
        "volume" => {
            use uom::si::f64::Volume;
            use uom::si::volume::*;
            let q = Volume::new::<cubic_meter>(si_value);
            match to_unit {
                "cubic_meter" | "cubic_metre" | "m3" | "m³" => Ok(q.get::<cubic_meter>()),
                "liter" | "litre" | "L" => Ok(q.get::<liter>()),
                "milliliter" | "millilitre" | "mL" | "ml" => Ok(q.get::<milliliter>()),
                "cubic_centimeter" | "cubic_centimetre" | "cm3" | "cm³" => {
                    Ok(q.get::<cubic_centimeter>())
                }
                "cubic_foot" | "ft3" | "ft³" => Ok(q.get::<cubic_foot>()),
                "cubic_inch" | "in3" | "in³" => Ok(q.get::<cubic_inch>()),
                "gallon" | "gal" => Ok(q.get::<gallon>()),
                "quart" | "qt" => Ok(q.get::<quart_liquid>()),
                "pint" | "pt" => Ok(q.get::<pint_liquid>()),
                "fluid_ounce" | "fl_oz" | "floz" => Ok(q.get::<fluid_ounce>()),
                _ => Err(UomError::new(UomErrorKind::UnknownUnit {
                    unit: to_unit.to_string(),
                    registration: "volume".to_string(),
                })),
            }
        }
        "density" => {
            use uom::si::f64::MassDensity;
            use uom::si::mass_density::*;
            let q = MassDensity::new::<kilogram_per_cubic_meter>(si_value);
            match to_unit {
                "kilogram_per_cubic_meter" | "kg/m3" | "kg/m³" => {
                    Ok(q.get::<kilogram_per_cubic_meter>())
                }
                "gram_per_cubic_centimeter" | "g/cm3" | "g/cm³" => {
                    Ok(q.get::<gram_per_cubic_centimeter>())
                }
                "pound_per_cubic_foot" | "lb/ft3" | "lb/ft³" => {
                    Ok(q.get::<pound_per_cubic_foot>())
                }
                _ => Err(UomError::new(UomErrorKind::UnknownUnit {
                    unit: to_unit.to_string(),
                    registration: "density".to_string(),
                })),
            }
        }
        "angle" => {
            use uom::si::angle::*;
            use uom::si::f64::Angle;
            let q = Angle::new::<radian>(si_value);
            match to_unit {
                "radian" | "rad" => Ok(q.get::<radian>()),
                "degree" | "deg" | "°" => Ok(q.get::<degree>()),
                "revolution" | "rev" => Ok(q.get::<revolution>()),
                "arcminute" | "'" => Ok(q.get::<minute>()),
                "arcsecond" | "\"" => Ok(q.get::<second>()),
                _ => Err(UomError::new(UomErrorKind::UnknownUnit {
                    unit: to_unit.to_string(),
                    registration: "angle".to_string(),
                })),
            }
        }
        "electric_current" => {
            use uom::si::electric_current::*;
            use uom::si::f64::ElectricCurrent;
            let q = ElectricCurrent::new::<ampere>(si_value);
            match to_unit {
                "ampere" | "A" => Ok(q.get::<ampere>()),
                "milliampere" | "mA" => Ok(q.get::<milliampere>()),
                "microampere" | "uA" | "μA" => Ok(q.get::<microampere>()),
                _ => Err(UomError::new(UomErrorKind::UnknownUnit {
                    unit: to_unit.to_string(),
                    registration: "electric_current".to_string(),
                })),
            }
        }
        "amount_of_substance" => {
            use uom::si::amount_of_substance::*;
            use uom::si::f64::AmountOfSubstance;
            let q = AmountOfSubstance::new::<mole>(si_value);
            match to_unit {
                "mole" | "mol" => Ok(q.get::<mole>()),
                "millimole" | "mmol" => Ok(q.get::<millimole>()),
                _ => Err(UomError::new(UomErrorKind::UnknownUnit {
                    unit: to_unit.to_string(),
                    registration: "amount_of_substance".to_string(),
                })),
            }
        }
        "luminous_intensity" => {
            use uom::si::f64::LuminousIntensity;
            use uom::si::luminous_intensity::*;
            let q = LuminousIntensity::new::<candela>(si_value);
            match to_unit {
                "candela" | "cd" => Ok(q.get::<candela>()),
                _ => Err(UomError::new(UomErrorKind::UnknownUnit {
                    unit: to_unit.to_string(),
                    registration: "luminous_intensity".to_string(),
                })),
            }
        }
        _ => Err(UomError::new(UomErrorKind::UnknownUnit {
            unit: to_unit.to_string(),
            registration: registration.to_string(),
        })),
    }
}

/// Parse a value+unit string for any registration, returning the SI value and snippet.
pub fn parse_any(registration: &str, value: f64, unit: &str) -> Result<(f64, String), UomError> {
    match registration {
        "length" => parse_length(value, unit),
        "mass" => parse_mass(value, unit),
        "time" => parse_time(value, unit),
        "temperature" => parse_temperature(value, unit),
        "electric_current" => parse_electric_current(value, unit),
        "amount_of_substance" => parse_amount_of_substance(value, unit),
        "luminous_intensity" => parse_luminous_intensity(value, unit),
        "velocity" => parse_velocity(value, unit),
        "acceleration" => parse_acceleration(value, unit),
        "force" => parse_force(value, unit),
        "energy" => parse_energy(value, unit),
        "power" => parse_power(value, unit),
        "pressure" => parse_pressure(value, unit),
        "frequency" => parse_frequency(value, unit),
        "area" => parse_area(value, unit),
        "volume" => parse_volume(value, unit),
        "density" => parse_density(value, unit),
        "angle" => parse_angle(value, unit),
        _ => Err(UomError::new(UomErrorKind::UnknownUnit {
            unit: unit.to_string(),
            registration: registration.to_string(),
        })),
    }
}

// ── Supported units catalog ───────────────────────────────────────────────────

/// Return the list of supported unit strings for a registration.
pub fn supported_units(registration: &str) -> &'static [&'static str] {
    match registration {
        "length" => &[
            "meter",
            "kilometer",
            "centimeter",
            "millimeter",
            "micrometer",
            "nanometer",
            "foot",
            "inch",
            "yard",
            "mile",
            "nautical_mile",
            "astronomical_unit",
            "light_year",
        ],
        "mass" => &[
            "kilogram",
            "gram",
            "milligram",
            "microgram",
            "tonne",
            "pound",
            "ounce",
            "ton",
        ],
        "time" => &[
            "second",
            "millisecond",
            "microsecond",
            "nanosecond",
            "minute",
            "hour",
            "day",
            "week",
            "year",
        ],
        "temperature" => &["kelvin", "degree_celsius", "degree_fahrenheit"],
        "electric_current" => &["ampere", "milliampere", "microampere"],
        "amount_of_substance" => &["mole", "millimole"],
        "luminous_intensity" => &["candela"],
        "velocity" => &[
            "meter_per_second",
            "kilometer_per_hour",
            "mile_per_hour",
            "foot_per_second",
            "knot",
            "speed_of_light",
        ],
        "acceleration" => &[
            "meter_per_second_squared",
            "foot_per_second_squared",
            "standard_gravity",
        ],
        "force" => &["newton", "kilonewton", "meganewton", "pound_force", "dyne"],
        "energy" => &[
            "joule",
            "kilojoule",
            "megajoule",
            "calorie",
            "kilocalorie",
            "kilowatt_hour",
            "electronvolt",
        ],
        "power" => &["watt", "kilowatt", "megawatt", "horsepower", "milliwatt"],
        "pressure" => &[
            "pascal",
            "kilopascal",
            "megapascal",
            "bar",
            "atmosphere",
            "psi",
            "torr",
        ],
        "frequency" => &["hertz", "kilohertz", "megahertz", "gigahertz", "terahertz"],
        "area" => &[
            "square_meter",
            "square_kilometer",
            "square_centimeter",
            "square_millimeter",
            "hectare",
            "acre",
            "square_foot",
            "square_inch",
            "square_yard",
            "square_mile",
        ],
        "volume" => &[
            "cubic_meter",
            "liter",
            "milliliter",
            "cubic_centimeter",
            "cubic_foot",
            "cubic_inch",
            "gallon",
            "quart",
            "pint",
            "fluid_ounce",
        ],
        "density" => &[
            "kilogram_per_cubic_meter",
            "gram_per_cubic_centimeter",
            "pound_per_cubic_foot",
        ],
        "angle" => &["radian", "degree", "revolution", "arcminute", "arcsecond"],
        _ => &[],
    }
}

/// All 18 registration names.
pub const ALL_REGISTRATIONS: &[&str] = &[
    "length",
    "mass",
    "time",
    "temperature",
    "electric_current",
    "amount_of_substance",
    "luminous_intensity",
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

/// Map a registration name to its Rust uom type name.
pub fn rust_type_name(registration: &str) -> &'static str {
    match registration {
        "length" => "Length",
        "mass" => "Mass",
        "time" => "Time",
        "temperature" => "ThermodynamicTemperature",
        "electric_current" => "ElectricCurrent",
        "amount_of_substance" => "AmountOfSubstance",
        "luminous_intensity" => "LuminousIntensity",
        "velocity" => "Velocity",
        "acceleration" => "Acceleration",
        "force" => "Force",
        "energy" => "Energy",
        "power" => "Power",
        "pressure" => "Pressure",
        "frequency" => "Frequency",
        "area" => "Area",
        "volume" => "Volume",
        "density" => "MassDensity",
        "angle" => "Angle",
        _ => "Unknown",
    }
}

/// Map a registration name to its SI base unit name.
pub fn si_unit_name(registration: &str) -> &'static str {
    match registration {
        "length" => "meter",
        "mass" => "kilogram",
        "time" => "second",
        "temperature" => "kelvin",
        "electric_current" => "ampere",
        "amount_of_substance" => "mole",
        "luminous_intensity" => "candela",
        "velocity" => "meter_per_second",
        "acceleration" => "meter_per_second_squared",
        "force" => "newton",
        "energy" => "joule",
        "power" => "watt",
        "pressure" => "pascal",
        "frequency" => "hertz",
        "area" => "square_meter",
        "volume" => "cubic_meter",
        "density" => "kilogram_per_cubic_meter",
        "angle" => "radian",
        _ => "unknown",
    }
}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for all `uom_qty__*` tool calls.
pub struct UomQuantityCtx {
    /// Central bus storing all quantity values by UUID.
    pub(crate) bus: QuantityBus,
}

impl UomQuantityCtx {
    fn new() -> Self {
        Self { bus: new_bus() }
    }
}

impl elicitation::PluginContext for UomQuantityCtx {}

// ── Param/result structs ──────────────────────────────────────────────────────

/// Parameters for `uom_{registration}__new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct QtyNewParams {
    /// Numeric value in the specified unit.
    pub value: f64,
    /// Unit string (e.g. `"kilometer"`, `"foot"`).
    pub unit: String,
}

/// Parameters for `uom_{registration}__emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct QtyEmitParams {
    /// UUID(s) of quantities to include in the emitted snippet.
    pub ids: Vec<String>,
}

/// Parameters for `uom_qty__value`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct QtyValueParams {
    /// UUID of the quantity.
    pub id: String,
    /// Target unit to express the value in.
    pub unit: String,
}

/// Parameters for `uom_qty__describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct QtyDescribeParams {
    /// UUID of the quantity.
    pub id: String,
}

/// Parameters for `uom_qty__delete`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct QtyDeleteParams {
    /// UUID of the quantity to remove.
    pub id: String,
}

/// Parameters for `uom_qty__registration`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct QtyRegistrationParams {
    /// UUID of the quantity.
    pub id: String,
}

/// Parameters for binary arithmetic tools.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct QtyBinaryParams {
    /// UUID of the left-hand operand.
    pub lhs_id: String,
    /// UUID of the right-hand operand.
    pub rhs_id: String,
}

/// Parameters for `uom_qty__scale`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct QtyScaleParams {
    /// UUID of the quantity to scale.
    pub id: String,
    /// Scalar multiplier.
    pub factor: f64,
}

/// Parameters for `uom_qty__powi`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct QtyPowiParams {
    /// UUID of the quantity.
    pub id: String,
    /// Integer exponent.
    pub n: i32,
}

/// Parameters for `uom_qty__compare`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct QtyCompareParams {
    /// UUID of the left-hand operand.
    pub lhs_id: String,
    /// UUID of the right-hand operand.
    pub rhs_id: String,
}

/// Parameters for `uom_qty__approx_eq`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct QtyApproxEqParams {
    /// UUID of the first quantity.
    pub lhs_id: String,
    /// UUID of the second quantity.
    pub rhs_id: String,
    /// Relative tolerance (default 1e-9).
    pub tolerance: Option<f64>,
}

/// Parameters for `uom_qty__convert`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct QtyConvertParams {
    /// UUID of the quantity.
    pub id: String,
    /// Target unit string.
    pub to_unit: String,
}

/// Parameters for `uom_qty__neg`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct QtyNegParams {
    /// UUID of the quantity.
    pub id: String,
}

/// Parameters for `uom_qty__abs`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct QtyAbsParams {
    /// UUID of the quantity.
    pub id: String,
}

/// Parameters for `uom_qty__recip`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct QtyRecipParams {
    /// UUID of the quantity.
    pub id: String,
}

/// Parameters for `uom_qty__sqrt`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct QtySqrtParams {
    /// UUID of the quantity.
    pub id: String,
}

/// Result carrying a single UUID.
#[derive(Debug, Serialize)]
pub struct QtyIdResult {
    /// UUID of the newly created quantity.
    pub id: String,
    /// Registration name.
    pub registration: String,
    /// SI value.
    pub si_value: f64,
}

/// Result carrying SI value.
#[derive(Debug, Serialize)]
pub struct QtyValueResult {
    /// UUID of the quantity.
    pub id: String,
    /// Value in the requested unit.
    pub value: f64,
    /// Unit string used.
    pub unit: String,
}

/// Result for describe.
#[derive(Debug, Serialize)]
pub struct QtyDescribeResult {
    /// UUID.
    pub id: String,
    /// Registration name.
    pub registration: String,
    /// SI value.
    pub si_value: f64,
    /// SI unit name.
    pub si_unit: String,
    /// Rust code snippet.
    pub code_snippet: String,
}

/// Result for compare.
#[derive(Debug, Serialize)]
pub struct QtyCompareResult {
    /// Ordering: "less", "equal", or "greater".
    pub ordering: String,
    /// LHS SI value.
    pub lhs_si: f64,
    /// RHS SI value.
    pub rhs_si: f64,
}

/// Result for approx_eq.
#[derive(Debug, Serialize)]
pub struct QtyApproxEqResult {
    /// Whether the values are approximately equal.
    pub approximately_equal: bool,
    /// Absolute difference between SI values.
    pub abs_difference: f64,
    /// Tolerance used.
    pub tolerance: f64,
}

// ── Plugin struct ─────────────────────────────────────────────────────────────

/// MCP plugin providing quantity creation, arithmetic, and conversion tools.
///
/// Each of the 18 registered quantity kinds gets two dedicated tools:
/// `uom_{name}__new` and `uom_{name}__emit`. Additionally, 12 arithmetic tools,
/// 5 shared query tools, and 1 conversion tool are provided under `uom_qty__*`.
pub struct UomQuantityPlugin(Arc<UomQuantityCtx>);

impl UomQuantityPlugin {
    /// Create a new `UomQuantityPlugin` with an empty bus.
    pub fn new() -> Self {
        Self(Arc::new(UomQuantityCtx::new()))
    }

    /// Return a clone of the shared quantity bus.
    pub fn bus(&self) -> QuantityBus {
        self.0.bus.clone()
    }
}

impl Default for UomQuantityPlugin {
    fn default() -> Self {
        Self::new()
    }
}

// ── Tool handler implementations ──────────────────────────────────────────────

async fn handle_qty_new(
    ctx: Arc<UomQuantityCtx>,
    registration: &'static str,
    value: f64,
    unit: &str,
) -> Result<CallToolResult, ErrorData> {
    let (si_value, snippet) = parse_any(registration, value, unit).map_err(uom_err_to_mcp)?;
    let id = Uuid::new_v4();
    ctx.bus.lock().await.insert(
        id,
        QuantityBusEntry {
            registration,
            si_value,
            code_snippet: snippet,
        },
    );
    ok_json(&QtyIdResult {
        id: id.to_string(),
        registration: registration.to_string(),
        si_value,
    })
}

async fn handle_qty_emit(
    ctx: Arc<UomQuantityCtx>,
    ids: &[String],
) -> Result<CallToolResult, ErrorData> {
    let bus = ctx.bus.lock().await;
    let mut snippets = Vec::new();
    for id_str in ids {
        let id: Uuid = id_str
            .parse()
            .map_err(|_| tool_err(format!("invalid UUID: {id_str}")))?;
        match bus.get(&id) {
            Some(entry) => snippets.push(entry.code_snippet.clone()),
            None => snippets.push(format!("// UUID not found: {id}")),
        }
    }
    Ok(CallToolResult::success(vec![Content::text(
        snippets.join("\n"),
    )]))
}

// ── Shared query tools ────────────────────────────────────────────────────────

#[instrument(skip(ctx, p))]
async fn qty_value(
    ctx: Arc<UomQuantityCtx>,
    p: QtyValueParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let bus = ctx.bus.lock().await;
    let entry = bus
        .get(&id)
        .ok_or_else(|| tool_err(format!("not found: {id}")))?;
    let value =
        convert_to_unit(entry.registration, entry.si_value, &p.unit).map_err(uom_err_to_mcp)?;
    ok_json(&QtyValueResult {
        id: p.id,
        value,
        unit: p.unit,
    })
}

#[instrument(skip(ctx, p))]
async fn qty_describe(
    ctx: Arc<UomQuantityCtx>,
    p: QtyDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let bus = ctx.bus.lock().await;
    let entry = bus
        .get(&id)
        .ok_or_else(|| tool_err(format!("not found: {id}")))?;
    let si_unit = si_unit_name(entry.registration).to_string();
    ok_json(&QtyDescribeResult {
        id: p.id,
        registration: entry.registration.to_string(),
        si_value: entry.si_value,
        si_unit,
        code_snippet: entry.code_snippet.clone(),
    })
}

#[instrument(skip(ctx))]
async fn qty_list(ctx: Arc<UomQuantityCtx>) -> Result<CallToolResult, ErrorData> {
    let bus = ctx.bus.lock().await;
    let items: Vec<serde_json::Value> = bus
        .iter()
        .map(|(id, entry)| {
            serde_json::json!({
                "id": id.to_string(),
                "registration": entry.registration,
                "si_value": entry.si_value,
            })
        })
        .collect();
    ok_json(&items)
}

#[instrument(skip(ctx, p))]
async fn qty_delete(
    ctx: Arc<UomQuantityCtx>,
    p: QtyDeleteParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let removed = ctx.bus.lock().await.remove(&id).is_some();
    ok_json(&serde_json::json!({ "deleted": removed, "id": p.id }))
}

#[instrument(skip(ctx, p))]
async fn qty_registration(
    ctx: Arc<UomQuantityCtx>,
    p: QtyRegistrationParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let bus = ctx.bus.lock().await;
    let entry = bus
        .get(&id)
        .ok_or_else(|| tool_err(format!("not found: {id}")))?;
    ok_json(&serde_json::json!({
        "id": p.id,
        "registration": entry.registration,
    }))
}

// ── Arithmetic tools ──────────────────────────────────────────────────────────

async fn get_bus_entry(
    bus: &tokio::sync::MutexGuard<'_, HashMap<Uuid, QuantityBusEntry>>,
    id_str: &str,
) -> Result<(Uuid, QuantityBusEntry), ErrorData> {
    let id: Uuid = id_str
        .parse()
        .map_err(|_| tool_err(format!("invalid UUID: {id_str}")))?;
    let entry = bus
        .get(&id)
        .cloned()
        .ok_or_else(|| tool_err(format!("not found: {id}")))?;
    Ok((id, entry))
}

async fn store_result(
    bus: &mut tokio::sync::MutexGuard<'_, HashMap<Uuid, QuantityBusEntry>>,
    registration: &'static str,
    si_value: f64,
    snippet: String,
) -> Uuid {
    let id = Uuid::new_v4();
    bus.insert(
        id,
        QuantityBusEntry {
            registration,
            si_value,
            code_snippet: snippet,
        },
    );
    id
}

fn reg_static(name: &str) -> Option<&'static str> {
    ALL_REGISTRATIONS.iter().copied().find(|r| *r == name)
}

#[instrument(skip(ctx, p))]
async fn qty_add(
    ctx: Arc<UomQuantityCtx>,
    p: QtyBinaryParams,
) -> Result<CallToolResult, ErrorData> {
    let mut bus = ctx.bus.lock().await;
    let (_, lhs) = get_bus_entry(&bus, &p.lhs_id).await?;
    let (_, rhs) = get_bus_entry(&bus, &p.rhs_id).await?;
    if lhs.registration != rhs.registration {
        return Err(tool_err(format!(
            "add requires same registration: {} ≠ {}",
            lhs.registration, rhs.registration
        )));
    }
    let reg = reg_static(lhs.registration)
        .ok_or_else(|| tool_err(format!("unknown registration: {}", lhs.registration)))?;
    let si = lhs.si_value + rhs.si_value;
    let snippet = format!(
        "// {} + {} = {} {}",
        lhs.si_value,
        rhs.si_value,
        si,
        si_unit_name(reg)
    );
    let id = store_result(&mut bus, reg, si, snippet).await;
    ok_json(&QtyIdResult {
        id: id.to_string(),
        registration: reg.to_string(),
        si_value: si,
    })
}

#[instrument(skip(ctx, p))]
async fn qty_sub(
    ctx: Arc<UomQuantityCtx>,
    p: QtyBinaryParams,
) -> Result<CallToolResult, ErrorData> {
    let mut bus = ctx.bus.lock().await;
    let (_, lhs) = get_bus_entry(&bus, &p.lhs_id).await?;
    let (_, rhs) = get_bus_entry(&bus, &p.rhs_id).await?;
    if lhs.registration != rhs.registration {
        return Err(tool_err(format!(
            "sub requires same registration: {} ≠ {}",
            lhs.registration, rhs.registration
        )));
    }
    let reg = reg_static(lhs.registration)
        .ok_or_else(|| tool_err(format!("unknown registration: {}", lhs.registration)))?;
    let si = lhs.si_value - rhs.si_value;
    let snippet = format!(
        "// {} - {} = {} {}",
        lhs.si_value,
        rhs.si_value,
        si,
        si_unit_name(reg)
    );
    let id = store_result(&mut bus, reg, si, snippet).await;
    ok_json(&QtyIdResult {
        id: id.to_string(),
        registration: reg.to_string(),
        si_value: si,
    })
}

#[instrument(skip(ctx, p))]
async fn qty_mul(
    ctx: Arc<UomQuantityCtx>,
    p: QtyBinaryParams,
) -> Result<CallToolResult, ErrorData> {
    let mut bus = ctx.bus.lock().await;
    let (_, lhs) = get_bus_entry(&bus, &p.lhs_id).await?;
    let (_, rhs) = get_bus_entry(&bus, &p.rhs_id).await?;
    let result_reg = derive_mul(lhs.registration, rhs.registration).ok_or_else(|| {
        tool_err(format!(
            "no derivation for {} × {}",
            lhs.registration, rhs.registration
        ))
    })?;
    let reg = reg_static(result_reg).unwrap_or(result_reg);
    let si = lhs.si_value * rhs.si_value;
    let snippet = format!(
        "// {} × {} = {} {}",
        lhs.si_value,
        rhs.si_value,
        si,
        si_unit_name(reg)
    );
    let id = store_result(&mut bus, reg, si, snippet).await;
    ok_json(&QtyIdResult {
        id: id.to_string(),
        registration: reg.to_string(),
        si_value: si,
    })
}

#[instrument(skip(ctx, p))]
async fn qty_div(
    ctx: Arc<UomQuantityCtx>,
    p: QtyBinaryParams,
) -> Result<CallToolResult, ErrorData> {
    let mut bus = ctx.bus.lock().await;
    let (_, lhs) = get_bus_entry(&bus, &p.lhs_id).await?;
    let (_, rhs) = get_bus_entry(&bus, &p.rhs_id).await?;
    if rhs.si_value == 0.0 {
        return Err(tool_err("division by zero"));
    }
    let result_reg = derive_div(lhs.registration, rhs.registration).ok_or_else(|| {
        tool_err(format!(
            "no derivation for {} ÷ {}",
            lhs.registration, rhs.registration
        ))
    })?;
    let reg = reg_static(result_reg).unwrap_or(result_reg);
    let si = lhs.si_value / rhs.si_value;
    let snippet = format!(
        "// {} ÷ {} = {} {}",
        lhs.si_value,
        rhs.si_value,
        si,
        si_unit_name(reg)
    );
    let id = store_result(&mut bus, reg, si, snippet).await;
    ok_json(&QtyIdResult {
        id: id.to_string(),
        registration: reg.to_string(),
        si_value: si,
    })
}

#[instrument(skip(ctx, p))]
async fn qty_neg(ctx: Arc<UomQuantityCtx>, p: QtyNegParams) -> Result<CallToolResult, ErrorData> {
    let mut bus = ctx.bus.lock().await;
    let (_, entry) = get_bus_entry(&bus, &p.id).await?;
    let reg = reg_static(entry.registration)
        .ok_or_else(|| tool_err(format!("unknown: {}", entry.registration)))?;
    let si = -entry.si_value;
    let snippet = format!("// -{} {}", entry.si_value, si_unit_name(reg));
    let id = store_result(&mut bus, reg, si, snippet).await;
    ok_json(&QtyIdResult {
        id: id.to_string(),
        registration: reg.to_string(),
        si_value: si,
    })
}

#[instrument(skip(ctx, p))]
async fn qty_abs(ctx: Arc<UomQuantityCtx>, p: QtyAbsParams) -> Result<CallToolResult, ErrorData> {
    let mut bus = ctx.bus.lock().await;
    let (_, entry) = get_bus_entry(&bus, &p.id).await?;
    let reg = reg_static(entry.registration)
        .ok_or_else(|| tool_err(format!("unknown: {}", entry.registration)))?;
    let si = entry.si_value.abs();
    let snippet = format!("// |{}| = {} {}", entry.si_value, si, si_unit_name(reg));
    let id = store_result(&mut bus, reg, si, snippet).await;
    ok_json(&QtyIdResult {
        id: id.to_string(),
        registration: reg.to_string(),
        si_value: si,
    })
}

#[instrument(skip(ctx, p))]
async fn qty_scale(
    ctx: Arc<UomQuantityCtx>,
    p: QtyScaleParams,
) -> Result<CallToolResult, ErrorData> {
    let mut bus = ctx.bus.lock().await;
    let (_, entry) = get_bus_entry(&bus, &p.id).await?;
    let reg = reg_static(entry.registration)
        .ok_or_else(|| tool_err(format!("unknown: {}", entry.registration)))?;
    let si = entry.si_value * p.factor;
    let snippet = format!(
        "// {} × {} = {} {}",
        entry.si_value,
        p.factor,
        si,
        si_unit_name(reg)
    );
    let id = store_result(&mut bus, reg, si, snippet).await;
    ok_json(&QtyIdResult {
        id: id.to_string(),
        registration: reg.to_string(),
        si_value: si,
    })
}

#[instrument(skip(ctx, p))]
async fn qty_recip(
    ctx: Arc<UomQuantityCtx>,
    p: QtyRecipParams,
) -> Result<CallToolResult, ErrorData> {
    let mut bus = ctx.bus.lock().await;
    let (_, entry) = get_bus_entry(&bus, &p.id).await?;
    if entry.si_value == 0.0 {
        return Err(tool_err("reciprocal of zero"));
    }
    let result_reg = derive_recip(entry.registration)
        .ok_or_else(|| tool_err(format!("no recip derivation for {}", entry.registration)))?;
    let reg = reg_static(result_reg).unwrap_or(result_reg);
    let si = 1.0 / entry.si_value;
    let snippet = format!("// 1/{} = {} {}", entry.si_value, si, si_unit_name(reg));
    let id = store_result(&mut bus, reg, si, snippet).await;
    ok_json(&QtyIdResult {
        id: id.to_string(),
        registration: reg.to_string(),
        si_value: si,
    })
}

#[instrument(skip(ctx, p))]
async fn qty_sqrt(ctx: Arc<UomQuantityCtx>, p: QtySqrtParams) -> Result<CallToolResult, ErrorData> {
    let mut bus = ctx.bus.lock().await;
    let (_, entry) = get_bus_entry(&bus, &p.id).await?;
    let result_reg = derive_sqrt(entry.registration)
        .ok_or_else(|| tool_err(format!("no sqrt derivation for {}", entry.registration)))?;
    let reg = reg_static(result_reg).unwrap_or(result_reg);
    let si = entry.si_value.sqrt();
    let snippet = format!("// √{} = {} {}", entry.si_value, si, si_unit_name(reg));
    let id = store_result(&mut bus, reg, si, snippet).await;
    ok_json(&QtyIdResult {
        id: id.to_string(),
        registration: reg.to_string(),
        si_value: si,
    })
}

#[instrument(skip(ctx, p))]
async fn qty_powi(ctx: Arc<UomQuantityCtx>, p: QtyPowiParams) -> Result<CallToolResult, ErrorData> {
    let mut bus = ctx.bus.lock().await;
    let (_, entry) = get_bus_entry(&bus, &p.id).await?;
    let result_reg = derive_pow(entry.registration, p.n).ok_or_else(|| {
        tool_err(format!(
            "no pow derivation for {}^{}",
            entry.registration, p.n
        ))
    })?;
    let reg = reg_static(result_reg).unwrap_or(result_reg);
    let si = entry.si_value.powi(p.n);
    let snippet = format!(
        "// {}^{} = {} {}",
        entry.si_value,
        p.n,
        si,
        si_unit_name(reg)
    );
    let id = store_result(&mut bus, reg, si, snippet).await;
    ok_json(&QtyIdResult {
        id: id.to_string(),
        registration: reg.to_string(),
        si_value: si,
    })
}

#[instrument(skip(ctx, p))]
async fn qty_compare(
    ctx: Arc<UomQuantityCtx>,
    p: QtyCompareParams,
) -> Result<CallToolResult, ErrorData> {
    let bus = ctx.bus.lock().await;
    let (_, lhs) = get_bus_entry(&bus, &p.lhs_id).await?;
    let (_, rhs) = get_bus_entry(&bus, &p.rhs_id).await?;
    if lhs.registration != rhs.registration {
        return Err(tool_err(format!(
            "compare requires same registration: {} ≠ {}",
            lhs.registration, rhs.registration
        )));
    }
    let ordering = match lhs.si_value.partial_cmp(&rhs.si_value) {
        Some(std::cmp::Ordering::Less) => "less",
        Some(std::cmp::Ordering::Equal) => "equal",
        Some(std::cmp::Ordering::Greater) => "greater",
        None => "unordered",
    };
    ok_json(&QtyCompareResult {
        ordering: ordering.to_string(),
        lhs_si: lhs.si_value,
        rhs_si: rhs.si_value,
    })
}

#[instrument(skip(ctx, p))]
async fn qty_approx_eq(
    ctx: Arc<UomQuantityCtx>,
    p: QtyApproxEqParams,
) -> Result<CallToolResult, ErrorData> {
    let bus = ctx.bus.lock().await;
    let (_, lhs) = get_bus_entry(&bus, &p.lhs_id).await?;
    let (_, rhs) = get_bus_entry(&bus, &p.rhs_id).await?;
    if lhs.registration != rhs.registration {
        return Err(tool_err(format!(
            "approx_eq requires same registration: {} ≠ {}",
            lhs.registration, rhs.registration
        )));
    }
    let tol = p.tolerance.unwrap_or(1e-9);
    let diff = (lhs.si_value - rhs.si_value).abs();
    let max_abs = lhs.si_value.abs().max(rhs.si_value.abs());
    let rel_diff = if max_abs == 0.0 { diff } else { diff / max_abs };
    ok_json(&QtyApproxEqResult {
        approximately_equal: rel_diff <= tol,
        abs_difference: diff,
        tolerance: tol,
    })
}

#[instrument(skip(ctx, p))]
async fn qty_convert(
    ctx: Arc<UomQuantityCtx>,
    p: QtyConvertParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let bus = ctx.bus.lock().await;
    let entry = bus
        .get(&id)
        .ok_or_else(|| tool_err(format!("not found: {id}")))?;
    let value =
        convert_to_unit(entry.registration, entry.si_value, &p.to_unit).map_err(uom_err_to_mcp)?;
    ok_json(&QtyValueResult {
        id: p.id,
        value,
        unit: p.to_unit,
    })
}

// ── Tool dispatch table ───────────────────────────────────────────────────────

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

fn make_new_tool(registration: &'static str) -> Tool {
    let name = format!("uom_{registration}__new");
    let desc = format!(
        "Create a new `{rust_type}` quantity from a numeric value and unit string. \
         Returns a UUID handle. Supported units: {units}.",
        rust_type = rust_type_name(registration),
        units = supported_units(registration).join(", "),
    );
    build_tool(name, desc, schema_of::<QtyNewParams>())
}

fn make_emit_tool(registration: &'static str) -> Tool {
    let name = format!("uom_{registration}__emit");
    let desc = format!(
        "Emit Rust code snippets for the given `{rust_type}` quantity UUIDs.",
        rust_type = rust_type_name(registration),
    );
    build_tool(name, desc, schema_of::<QtyEmitParams>())
}

impl elicitation::ElicitPlugin for UomQuantityPlugin {
    fn name(&self) -> &'static str {
        "uom_qty"
    }

    fn list_tools(&self) -> Vec<Tool> {
        let mut tools = Vec::new();
        // Per-registration tools
        for &reg in ALL_REGISTRATIONS {
            tools.push(make_new_tool(reg));
            tools.push(make_emit_tool(reg));
        }
        // Shared tools
        tools.push(build_tool(
            "uom_qty__value",
            "Get the value of a quantity in a specified unit.",
            schema_of::<QtyValueParams>(),
        ));
        tools.push(build_tool(
            "uom_qty__describe",
            "Describe a quantity: registration, SI value, SI unit, code snippet.",
            schema_of::<QtyDescribeParams>(),
        ));
        tools.push(build_tool(
            "uom_qty__list",
            "List all stored quantities with their registration and SI value.",
            serde_json::json!({"type":"object","properties":{}}),
        ));
        tools.push(build_tool(
            "uom_qty__delete",
            "Delete a stored quantity by UUID.",
            schema_of::<QtyDeleteParams>(),
        ));
        tools.push(build_tool(
            "uom_qty__registration",
            "Get the registration name of a quantity by UUID.",
            schema_of::<QtyRegistrationParams>(),
        ));
        // Arithmetic
        tools.push(build_tool(
            "uom_qty__add",
            "Add two quantities of the same registration.",
            schema_of::<QtyBinaryParams>(),
        ));
        tools.push(build_tool(
            "uom_qty__sub",
            "Subtract two quantities of the same registration.",
            schema_of::<QtyBinaryParams>(),
        ));
        tools.push(build_tool(
            "uom_qty__mul",
            "Multiply two quantities, deriving the result registration from the dimension table.",
            schema_of::<QtyBinaryParams>(),
        ));
        tools.push(build_tool(
            "uom_qty__div",
            "Divide two quantities, deriving the result registration from the dimension table.",
            schema_of::<QtyBinaryParams>(),
        ));
        tools.push(build_tool(
            "uom_qty__neg",
            "Negate a quantity.",
            schema_of::<QtyNegParams>(),
        ));
        tools.push(build_tool(
            "uom_qty__abs",
            "Take the absolute value of a quantity.",
            schema_of::<QtyAbsParams>(),
        ));
        tools.push(build_tool(
            "uom_qty__scale",
            "Multiply a quantity by a dimensionless scalar.",
            schema_of::<QtyScaleParams>(),
        ));
        tools.push(build_tool(
            "uom_qty__recip",
            "Compute the reciprocal of a quantity (1/x).",
            schema_of::<QtyRecipParams>(),
        ));
        tools.push(build_tool(
            "uom_qty__sqrt",
            "Compute the square root of a quantity.",
            schema_of::<QtySqrtParams>(),
        ));
        tools.push(build_tool(
            "uom_qty__powi",
            "Raise a quantity to an integer power.",
            schema_of::<QtyPowiParams>(),
        ));
        tools.push(build_tool("uom_qty__compare", "Compare two quantities of the same registration. Returns ordering: less/equal/greater.", schema_of::<QtyCompareParams>()));
        tools.push(build_tool(
            "uom_qty__approx_eq",
            "Check if two quantities are approximately equal within a relative tolerance.",
            schema_of::<QtyApproxEqParams>(),
        ));
        // Convert
        tools.push(build_tool(
            "uom_qty__convert",
            "Convert a quantity to a different unit, returning the numeric value.",
            schema_of::<QtyConvertParams>(),
        ));
        tools
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
            dispatch_tool(ctx, name, &params).await
        })
    }
}

impl UomQuantityPlugin {
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
        dispatch_tool(self.0.clone(), name, &params).await
    }
}

async fn dispatch_tool(
    ctx: Arc<UomQuantityCtx>,
    name: &str,
    params: &CallToolRequestParams,
) -> Result<CallToolResult, ErrorData> {
    // Per-registration new tools
    for &reg in ALL_REGISTRATIONS {
        let new_name = format!("uom_{reg}__new");
        let emit_name = format!("uom_{reg}__emit");
        if name == new_name {
            let p: QtyNewParams = parse_params(params)?;
            return handle_qty_new(ctx, reg, p.value, &p.unit).await;
        }
        if name == emit_name {
            let p: QtyEmitParams = parse_params(params)?;
            return handle_qty_emit(ctx, &p.ids).await;
        }
    }
    // Shared and arithmetic tools
    match name {
        "uom_qty__value" => qty_value(ctx, parse_params(params)?).await,
        "uom_qty__describe" => qty_describe(ctx, parse_params(params)?).await,
        "uom_qty__list" => qty_list(ctx).await,
        "uom_qty__delete" => qty_delete(ctx, parse_params(params)?).await,
        "uom_qty__registration" => qty_registration(ctx, parse_params(params)?).await,
        "uom_qty__add" => qty_add(ctx, parse_params(params)?).await,
        "uom_qty__sub" => qty_sub(ctx, parse_params(params)?).await,
        "uom_qty__mul" => qty_mul(ctx, parse_params(params)?).await,
        "uom_qty__div" => qty_div(ctx, parse_params(params)?).await,
        "uom_qty__neg" => qty_neg(ctx, parse_params(params)?).await,
        "uom_qty__abs" => qty_abs(ctx, parse_params(params)?).await,
        "uom_qty__scale" => qty_scale(ctx, parse_params(params)?).await,
        "uom_qty__recip" => qty_recip(ctx, parse_params(params)?).await,
        "uom_qty__sqrt" => qty_sqrt(ctx, parse_params(params)?).await,
        "uom_qty__powi" => qty_powi(ctx, parse_params(params)?).await,
        "uom_qty__compare" => qty_compare(ctx, parse_params(params)?).await,
        "uom_qty__approx_eq" => qty_approx_eq(ctx, parse_params(params)?).await,
        "uom_qty__convert" => qty_convert(ctx, parse_params(params)?).await,
        _ => Err(ErrorData::invalid_params(
            format!("unknown tool: {name}"),
            None,
        )),
    }
}

/// Parse the `arguments` field of a [`CallToolRequestParams`] into a typed struct.
///
/// Returns an [`ErrorData`] with `invalid_params` code if deserialization fails.
pub fn parse_params<T: serde::de::DeserializeOwned>(
    params: &CallToolRequestParams,
) -> Result<T, ErrorData> {
    let raw = params
        .arguments
        .as_ref()
        .map(|a| serde_json::Value::Object(a.clone()))
        .unwrap_or(serde_json::Value::Object(Default::default()));
    serde_json::from_value(raw)
        .map_err(|e| ErrorData::invalid_params(format!("param parse: {e}"), None))
}
