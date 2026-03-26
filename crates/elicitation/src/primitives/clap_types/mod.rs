//! Elicitation implementations for [`clap`] types.
//!
//! Provides [`Elicitation`](crate::Elicitation) for the clap 4 types that
//! can be interactively constructed — enumeration types via
//! [`Select`](crate::Select), simple value types as primitives, and builder
//! types as multi-field surveys.
//!
//! # Enabled by the `clap-types` feature
//!
//! ```toml
//! elicitation = { version = "*", features = ["clap-types"] }
//! ```
//!
//! # Supported types
//!
//! | Type | Pattern | Notes |
//! |------|---------|-------|
//! | [`clap::ColorChoice`] | Select | Never / Auto / Always |
//! | [`clap::ArgAction`] | Select | Set / Append / SetTrue / … |
//! | [`clap::ValueHint`] | Select | 13 completion hint variants |
//! | [`clap::parser::ValueSource`] | Select | Default / Env / CLI |
//! | [`clap::error::ErrorKind`] | Select | 15 error kind variants |
//! | [`clap::Id`] | Primitive | Newtype around `String` |
//! | [`clap::builder::PossibleValue`] | Primitive | Name + optional help |
//! | [`clap::builder::ValueRange`] | Primitive | min + optional max |
//! | [`clap::Arg`] | Survey | Core fields: name, action, hint, required |
//! | [`clap::ArgGroup`] | Survey | id, member args, required |
//! | [`clap::Command`] | Survey | name, version, about, args (partial) |
//!
//! # Not supported
//!
//! - `ValueParser` — erased trait object, no `Clone`
//! - `ArgMatches` — runtime parse result, no `Clone`
//! - `Error` — runtime error state, no `Clone`
//! - `Parser`, `Args`, `Subcommand`, `ValueEnum` — traits, not types

mod arg;
mod arg_action;
mod arg_group;
mod color_choice;
mod command;
mod error_kind;
mod id;
mod possible_value;
mod value_hint;
mod value_range;
mod value_source;

pub use arg::ArgStyle;
pub use arg_action::ArgActionStyle;
pub use arg_group::ArgGroupStyle;
pub use color_choice::ColorChoiceStyle;
pub use command::CommandStyle;
pub use error_kind::ErrorKindStyle;
pub use id::IdStyle;
pub use possible_value::PossibleValueStyle;
pub use value_hint::ValueHintStyle;
pub use value_range::ValueRangeStyle;
pub use value_source::ValueSourceStyle;
