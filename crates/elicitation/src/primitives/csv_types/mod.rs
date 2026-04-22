//! Trenchcoat wrappers for `csv` public types.
//!
//! Each type adds [`schemars::JsonSchema`] so it can cross the MCP boundary.
//! Bidirectional `From` conversions to/from upstream `csv` types are provided
//! when the `csv-types` feature is enabled.
//!
//! # Types
//!
//! - [`CsvQuoteStyle`] — shadow of `csv::QuoteStyle`
//! - [`CsvTerminator`] — shadow of `csv::Terminator`
//! - [`CsvTrim`] — shadow of `csv::Trim`
//! - [`CsvPosition`] — shadow of `csv::Position`
//! - [`CsvStringRecord`] — shadow of `csv::StringRecord` (newtype `Vec<String>`)
//! - [`CsvByteRecord`] — shadow of `csv::ByteRecord` (newtype `Vec<Vec<u8>>`)
//! - [`CsvErrorKind`] — shadow of `csv::ErrorKind` for display/reporting
//!
//! # Enabled by the `csv-types` feature

mod byte_record;
mod error_kind;
mod position;
mod quote_style;
mod string_record;
mod terminator;
mod trim;

pub use byte_record::CsvByteRecord;
pub use error_kind::CsvErrorKind;
pub use position::CsvPosition;
pub use quote_style::CsvQuoteStyle;
pub use string_record::CsvStringRecord;
pub use terminator::CsvTerminator;
pub use trim::CsvTrim;
