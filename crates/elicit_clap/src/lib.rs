//! `elicit_clap` — elicitation-enabled wrappers around `clap` types.
//!
//! Provides newtype wrappers for the clap types that have [`Elicitation`](elicitation::Elicitation)
//! impls in the main crate (behind the `clap-types` feature). Each wrapper:
//!
//! - Implements [`schemars::JsonSchema`] so the type can appear in MCP tool schemas
//! - Provides `Deref`/`DerefMut` for transparent access to the inner clap type
//! - Converts losslessly via `From`/`Into`
//! - Exposes useful methods via `#[reflect_methods]` for MCP tool discovery
//!
//! # Supported types
//!
//! | Wrapper | Inner type | Pattern |
//! |---------|-----------|---------|
//! | [`ColorChoice`] | `clap::ColorChoice` | Select enum |
//! | [`ArgAction`] | `clap::ArgAction` | Select enum |
//! | [`ValueHint`] | `clap::ValueHint` | Select enum |
//! | [`ValueSource`] | `clap::parser::ValueSource` | Select enum |
//! | [`ErrorKind`] | `clap::error::ErrorKind` | Select enum |
//! | [`Id`] | `clap::Id` | Primitive |
//! | [`PossibleValue`] | `clap::builder::PossibleValue` | Survey |
//! | [`ValueRange`] | `clap::builder::ValueRange` | Survey |
//! | [`Arg`] | `clap::Arg` | Survey builder |
//! | [`ArgGroup`] | `clap::ArgGroup` | Survey builder |
//! | [`Command`] | `clap::Command` | Survey builder |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

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

pub use arg::Arg;
pub use arg_action::ArgAction;
pub use arg_group::ArgGroup;
pub use color_choice::ColorChoice;
pub use command::Command;
pub use error_kind::ErrorKind;
pub use id::Id;
pub use possible_value::PossibleValue;
pub use value_hint::ValueHint;
pub use value_range::ValueRange;
pub use value_source::ValueSource;
