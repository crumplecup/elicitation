//! Conversational elicitation of strongly-typed Rust values via MCP.
//!
//! The `elicitation` library provides a trait-based system for eliciting
//! strongly-typed values from users through conversational interaction via
//! the Model Context Protocol (MCP). It transforms LLM conversations into
//! type-safe Rust values with compile-time guarantees.
//!
//! # MCP Setup Required
//!
//! This library runs as an **MCP server** and requires an **MCP client**
//! (like Claude Desktop or Claude CLI) to provide the elicitation tools.
//! Your application won't work standalone - it must be invoked by an MCP client.
//!
//! See the [README](https://github.com/crumplecup/elicitation) for setup instructions.
//!
//! # Core Concepts
//!
//! ## Traits
//!
//! - [`Prompt`] - Provides prompt metadata for a type
//! - [`Elicit`] - Main trait for eliciting values
//!
//! ## Interaction Paradigms
//!
//! - [`Select`] - Choose from finite options (enum pattern)
//! - [`Affirm`] - Yes/no confirmation (bool pattern)
//! - [`Survey`] - Multi-field elicitation (struct pattern)
//! - [`Authorize`] - Permission policies (planned for v0.2.0)
//!
//! # Example
//!
//! ```rust,ignore
//! use elicitation::{Elicitation, ElicitResult};
//! use rmcp::service::{Peer, RoleClient};
//!
//! async fn example(client: &Peer<RoleClient>) -> ElicitResult<()> {
//!     // Elicit a simple integer
//!     let age: i32 = i32::elicit(client).await?;
//!
//!     // Elicit an optional value
//!     let nickname: Option<String> = Option::<String>::elicit(client).await?;
//!
//!     // Elicit a collection
//!     let scores: Vec<i32> = Vec::<i32>::elicit(client).await?;
//!     Ok(())
//! }
//! ```
//!
//! # Derive Macros
//!
//! The library provides derive macros for automatic implementation:
//!
//! ```rust,ignore
//! use elicitation::Elicit;
//!
//! // Enums automatically use the Select paradigm
//! #[derive(Elicit)]
//! enum Color {
//!     Red,
//!     Green,
//!     Blue,
//! }
//!
//! // Structs automatically use the Survey paradigm
//! #[derive(Elicit)]
//! struct Person {
//!     #[prompt("What is your name?")]
//!     name: String,
//!     #[prompt("What is your age?")]
//!     age: u8,
//! }
//! ```
//!
//! # MCP Integration
//!
//! The library uses the [rmcp](https://crates.io/crates/rmcp) crate - the
//! official Rust MCP SDK - for MCP client integration. All elicitation
//! happens through asynchronous MCP tool calls.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod client;
// Verification framework imports

mod collections;
mod containers;
mod default_style;
mod error;
pub mod verification;

#[cfg(kani)]
mod kani_tests;

#[cfg(feature = "cli")]
pub mod cli;

pub mod mcp;
mod paradigm;
mod primitives;
pub mod style;
mod traits;

#[cfg(feature = "serde_json")]
mod value_impl;

#[cfg(any(feature = "chrono", feature = "time", feature = "jiff"))]
mod datetime_common;

#[cfg(feature = "chrono")]
mod datetime_chrono;

#[cfg(feature = "time")]
pub mod datetime_time;

#[cfg(feature = "jiff")]
mod datetime_jiff;

mod elicitation_style;

// Error types
pub use error::{ElicitError, ElicitErrorKind, ElicitResult, JsonError, RmcpError, ServiceError};

// Core client
pub use client::ElicitClient;

// Core traits
pub use elicitation_style::ElicitationStyle;
pub use traits::{ElicitBuilder, Elicitation, Generator, Prompt};

// Interaction paradigm traits
pub use paradigm::{Affirm, Authorize, FieldInfo, Select, Survey};

// Re-export rmcp for user convenience
pub use rmcp;

// Re-export derive macro with user-friendly name
pub use elicitation_derive::Elicit;

// Re-export verification contract types at crate level (for kani_proofs imports)
// EXPLICIT exports - no globs (helps compiler show what's missing)
#[cfg(any(feature = "verification", kani))]
pub use verification::types::{
    ArcNonNull,
    ArcSatisfies,
    ArrayAllSatisfy,
    BTreeMapNonEmpty,
    BTreeSetNonEmpty,
    BoolFalse,
    // Bools
    BoolTrue,
    BoxNonNull,
    BoxSatisfies,
    // Chars
    CharAlphabetic,
    CharAlphanumeric,
    CharNumeric,
    DurationNonZero,
    // Durations
    DurationPositive,
    F32Finite,
    F32NonNegative,
    // Floats
    F32Positive,
    F64Finite,
    F64NonNegative,
    F64Positive,
    HashMapNonEmpty,
    HashSetNonEmpty,
    I8NonNegative,
    I8NonZero,
    I8NonZeroStyle,
    // Integers - i8 family
    I8Positive,
    I8Range,
    I8RangeStyle,
    I16NonNegative,
    I16NonZero,
    I16NonZeroStyle,
    // i16 family
    I16Positive,
    I16Range,
    I16RangeStyle,
    I32NonNegative,
    I32NonZero,
    // i32 family
    I32Positive,
    I32Range,
    I32RangeStyle,
    I64NonNegative,
    I64NonZero,
    // i64 family
    I64Positive,
    I64Range,
    I64RangeStyle,
    I128NonNegative,
    I128NonZero,
    // i128 family
    I128Positive,
    I128Range,
    I128RangeStyle,
    // Networks
    IpPrivate,
    IpPublic,
    IpV4,
    IpV6,
    Ipv4Loopback,
    Ipv6Loopback,
    IsizeNonNegative,
    IsizeNonZero,
    // isize family
    IsizePositive,
    IsizeRange,
    IsizeRangeStyle,
    LinkedListNonEmpty,
    OptionSome,
    // Paths
    PathBufExists,
    PathBufIsDir,
    PathBufIsFile,
    PathBufReadable,
    RcNonNull,
    RcSatisfies,
    ResultOk,
    // Strings
    StringNonEmpty,
    // Tuples
    Tuple2,
    Tuple3,
    Tuple4,
    U8NonZero,
    // u8 family
    U8Positive,
    U8Range,
    U8RangeStyle,
    U16NonZero,
    // u16 family
    U16Positive,
    U16Range,
    U16RangeStyle,
    U32NonZero,
    // u32 family
    U32Positive,
    U32Range,
    U32RangeStyle,
    U64NonZero,
    // u64 family
    U64Positive,
    U64Range,
    U64RangeStyle,
    U128NonZero,
    // u128 family
    U128Positive,
    U128Range,
    U128RangeStyle,
    UsizeNonZero,
    // usize family
    UsizePositive,
    UsizeRange,
    UsizeRangeStyle,
    // ValidationError
    ValidationError,
    VecAllSatisfy,
    VecDequeNonEmpty,
    // Collections
    VecNonEmpty,
};

// UUIDs (feature-gated on uuid)
#[cfg(all(any(feature = "verification", kani), feature = "uuid"))]
pub use verification::types::{UuidNonNil, UuidV4};

#[cfg(feature = "uuid")]
pub use primitives::uuid::{UuidGenerationMode, UuidGenerator};

// SystemTime (standard library)
pub use primitives::systemtime::{
    SystemTimeGenerationMode, SystemTimeGenerator,
};

// DateTimes (feature-gated on chrono/time/jiff)
#[cfg(all(any(feature = "verification", kani), feature = "chrono"))]
pub use verification::types::{DateTimeUtcAfter, DateTimeUtcBefore, NaiveDateTimeAfter};

#[cfg(all(any(feature = "verification", kani), feature = "time"))]
pub use verification::types::{OffsetDateTimeAfter, OffsetDateTimeBefore};

#[cfg(all(any(feature = "verification", kani), feature = "jiff"))]
pub use verification::types::{TimestampAfter, TimestampBefore};

// Values (JSON - feature-gated)
#[cfg(all(any(feature = "verification", kani), feature = "serde_json"))]
pub use verification::types::{ValueArray, ValueNonNull, ValueObject};

// URLs (feature-gated)
#[cfg(all(any(feature = "verification", kani), feature = "url"))]
pub use verification::types::{UrlCanBeBase, UrlHttp, UrlHttps, UrlValid, UrlWithHost};

// Regexes (feature-gated)
#[cfg(all(any(feature = "verification", kani), feature = "regex"))]
pub use verification::types::{
    RegexCaseInsensitive, RegexMultiline, RegexSetNonEmpty, RegexSetValid, RegexValid,
};

// Mechanisms
#[cfg(any(feature = "verification", kani))]
pub use verification::mechanisms::{
    AffirmReturnsBoolean, InputNonEmpty, MechanismWithType, NumericReturnsValid,
    SurveyReturnsValidVariant, TextReturnsNonEmpty, TextReturnsString,
};
