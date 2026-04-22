//! Elicitation shadow crate for the `csv` reader/writer library.
//!
//! Exposes `csv` 1.x as MCP tools via a [`CsvPlugin`] that holds live builder,
//! reader, and writer instances keyed by UUID.
//!
//! # Plugins
//!
//! - [`CsvPlugin`] — holds live `ReaderBuilder`, `WriterBuilder`, `Reader`, and `Writer`
//!   instances keyed by UUID; each public method on each type is its own MCP tool.
//!
//! # Factories
//!
//! - [`prime_csv_deserialize`] — registers typed deserialization tools for any
//!   `D: ElicitComplete + DeserializeOwned`
//! - [`prime_csv_serialize`] — registers typed serialization tools for any
//!   `T: ElicitComplete + Serialize`

#![forbid(unsafe_code)]

mod factory;
mod plugin;
mod reader_builder_tools;
mod reader_tools;
mod record_tools;
mod utility_tools;
mod writer_builder_tools;
mod writer_tools;

pub use factory::{prime_csv_deserialize, prime_csv_serialize};
pub use plugin::{CsvCtx, CsvPlugin};
