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
//!     let age: i32 = i32::elicit(communicator).await?;
//!
//!     // Elicit an optional value
//!     let nickname: Option<String> = Option::<String>::elicit(communicator).await?;
//!
//!     // Elicit a collection
//!     let scores: Vec<i32> = Vec::<i32>::elicit(communicator).await?;
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
// Allow ::elicitation::... paths in proc-macro generated code inside this crate.
extern crate self as elicitation;

mod client;
mod communicator;
mod mcp_wrapper; // Generic wrapper for MCP tool outputs
mod server; // Server-side wrapper (analogous to ElicitClient)
// Unified trait for client/server communication

// Verification framework imports

mod collections;
mod complete;
mod containers;
mod default_style;
mod error;
pub mod type_spec;
pub mod verification;

pub mod type_graph;

pub mod prompt_tree;
pub use prompt_tree::{
    AssembledPrompt, ElicitPromptTree, PromptKind, PromptTree, collect_assembled_prompts,
};

#[cfg(feature = "cli")]
pub mod cli;

pub mod contracts;

pub mod emit_code;
pub mod mcp;
mod paradigm;
mod primitives;
mod proxy;
pub mod style;
pub mod tool;
mod tool_registry;
mod verified_workflow;

// Router macro module (declarative macro)
#[macro_use]
mod router_macro;

// Newtype wrapper macro module (declarative macros)
#[macro_use]
mod newtype_macro;

// Newtype methods macro module (declarative macros with method delegation)
#[macro_use]
mod newtype_methods_macro;

// Select trenchcoat macro: wraps foreign Select enums with JsonSchema + serde
#[macro_use]
mod select_trenchcoat_macro;

mod traits;

mod elicit_json;
pub use elicit_json::ElicitJson;
pub mod plugin;
mod plugin_registry;
pub use plugin::{
    DescriptorPlugin, ElicitPlugin, NoContext, PluginContext, PluginToolRegistration,
    StatefulPlugin, ToolDescriptor, make_descriptor, make_descriptor_ctx,
};
pub use plugin_registry::{PluginRegistry, Toolchain};

pub mod dynamic;
pub use dynamic::{
    AnyToolFactory, AnyToolSlot, DynamicToolDescriptor, DynamicToolRegistry,
    ToolFactoryRegistration, TypedSlot,
};

#[cfg(feature = "serde_json")]
mod value_impl;

#[cfg(any(feature = "chrono", feature = "time", feature = "jiff"))]
mod datetime_common;

#[cfg(feature = "chrono")]
pub mod datetime_chrono;

#[cfg(feature = "time")]
pub mod datetime_time;

#[cfg(feature = "jiff")]
pub mod datetime_jiff;

#[cfg(feature = "rand")]
pub mod rand_rng;

mod elicitation_style;

// Error types
pub use error::{ElicitError, ElicitErrorKind, ElicitResult, JsonError, RmcpError, ServiceError};
pub use verification::types::ValidationError;

// Core client and server
pub use client::ElicitClient;
pub use communicator::{ElicitCommunicator, ElicitationContext, StyleContext};
pub use mcp_wrapper::ElicitToolOutput;
pub use server::ElicitServer;

// Core traits
pub use elicitation_style::StyleMarker;
pub use traits::{
    ElicitBuilder, ElicitIntrospect, Elicitation, ElicitationPattern, Generator, PatternDetails,
    Prompt, TypeMetadata, VariantMetadata,
};

// Type graph visualization — registry always available; builder/renderers gated on `graph`
#[cfg(feature = "graph")]
pub use type_graph::{
    DotRenderer, GraphEdge, GraphNode, GraphRenderer, MermaidDirection, MermaidRenderer, NodeKind,
    TypeGraph, TypeGraphError, TypeGraphPlugin,
};
pub use type_graph::{TypeGraphKey, all_graphable_types, lookup_type_graph};

// Type spec layer (agent-browsable contracts)
pub use type_spec::{
    ElicitSpec, SpecCategory, SpecCategoryBuilder, SpecEntry, SpecEntryBuilder, TypeSpec,
    TypeSpecBuilder, TypeSpecInventoryKey, lookup_type_spec, lookup_type_spec_by_id,
    type_spec_plugin::TypeSpecPlugin,
};

// Contracts (proof-carrying composition)
pub use contracts::{
    And, Established, Implies, InVariant, Is, Prop, Refines, both, downcast, fst, snd,
};

// Completion marker — enforces all elicitation obligations at compile time
pub use complete::ElicitComplete;

// Workflow verification marker — enforces all proposition obligations
pub use verified_workflow::VerifiedWorkflow;

// Tools (contract-based MCP tools)
pub use tool::{Tool, True, both_tools, then};

// Tool registry (automatic discovery)
pub use tool_registry::{ElicitToolDescriptor, collect_all_elicit_tools};

// Interaction paradigm traits
pub use paradigm::{Affirm, Authorize, FieldInfo, Filter, Select, Survey};
pub use proxy::ElicitProxy;

// Dynamic collections
pub use collections::ChoiceSet;

// Re-export rmcp for user convenience
pub use rmcp;

// Re-export futures for derive macro (BoxFuture in ElicitPlugin blanket impls)
#[doc(hidden)]
pub use futures;

// Re-export serde_json for derive macro (needed in elicit_checked)
#[doc(hidden)]
pub use serde_json;

// Re-export paste for macro usage
#[doc(hidden)]
pub use paste;

// Re-export inventory for derive macro usage
#[doc(hidden)]
pub use inventory;

// Re-export async_trait for derive macro and trait impls
#[doc(hidden)]
pub use async_trait;

// Re-export proc_macro2 so generated code can use elicitation::proc_macro2
// instead of requiring proc_macro2 as a direct dep of downstream crates.
#[doc(hidden)]
pub use proc_macro2;
#[doc(hidden)]
pub use quote;

// Re-export derive macros with user-friendly names
pub use elicitation_derive::{Elicit, ElicitPlugin, elicit_tool};
// Prop derive macro (trait lives at elicitation::contracts::Prop; both can coexist)
pub use elicitation_derive::Prop;
// ToCodeLiteral derive (trait lives at elicitation::emit_code::ToCodeLiteral)
pub use elicitation_derive::ToCodeLiteral;

// Re-export verification contract types at crate level (for kani_proofs imports)
// EXPLICIT exports - no globs (helps compiler show what's missing)
pub use verification::Contract;

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
    VecAllSatisfy,
    VecDequeNonEmpty,
    // Collections
    VecNonEmpty,
};

// UUIDs (feature-gated on uuid)
#[cfg(feature = "uuid")]
pub use verification::types::{UuidNonNil, UuidV4};

#[cfg(feature = "uuid")]
pub use primitives::uuid::{UuidGenerationMode, UuidGenerator};

// SystemTime (standard library)
pub use primitives::systemtime::{SystemTimeGenerationMode, SystemTimeGenerator};

// Duration (standard library)
pub use primitives::duration::{DurationGenerationMode, DurationGenerator};

// String style variants
pub use primitives::StringStyle;

// Unit structs (standard library)
pub use primitives::unit_structs::{Formatter, Parser, Validator};

// Error generators (for testing)
pub use primitives::errors::{IoErrorGenerationMode, IoErrorGenerator};

#[cfg(feature = "serde_json")]
pub use primitives::errors::{JsonErrorGenerationMode, JsonErrorGenerator};

// DateTime generators (feature-gated)
#[cfg(feature = "chrono")]
pub use datetime_chrono::{
    DateTimeUtcGenerationMode, DateTimeUtcGenerator, NaiveDateTimeGenerationMode,
    NaiveDateTimeGenerator,
};

#[cfg(feature = "time")]
pub use datetime_time::{
    InstantGenerationMode, InstantGenerator, OffsetDateTimeGenerationMode, OffsetDateTimeGenerator,
};

#[cfg(feature = "jiff")]
pub use datetime_jiff::{TimestampGenerationMode, TimestampGenerator};

// DateTimes (feature-gated on chrono/time/jiff)
#[cfg(feature = "chrono")]
pub use verification::types::{DateTimeUtcAfter, DateTimeUtcBefore, NaiveDateTimeAfter};

#[cfg(feature = "time")]
pub use verification::types::{OffsetDateTimeAfter, OffsetDateTimeBefore};

#[cfg(feature = "jiff")]
pub use verification::types::{TimestampAfter, TimestampBefore};

// Values (JSON - feature-gated)
#[cfg(feature = "serde_json")]
pub use verification::types::{ValueArray, ValueNonNull, ValueObject};

// URLs (feature-gated)
#[cfg(feature = "url")]
pub use verification::types::{UrlCanBeBase, UrlHttp, UrlHttps, UrlValid, UrlWithHost};

// Regexes (feature-gated)
#[cfg(feature = "regex")]
pub use verification::types::{
    RegexCaseInsensitive, RegexMultiline, RegexSetNonEmpty, RegexSetValid, RegexValid,
};

// Mechanisms
pub use verification::mechanisms::{
    AffirmReturnsBoolean, InputNonEmpty, MechanismWithType, NumericReturnsValid,
    SurveyReturnsValidVariant, TextReturnsNonEmpty, TextReturnsString,
};

// Reqwest HTTP types (feature-gated)
#[cfg(feature = "reqwest")]
pub use primitives::http::{
    ClientStyle, HeaderMapStyle, MethodStyle, RequestBuilderStyle, ResponseStyle, StatusCodeStyle,
    VersionStyle,
};

#[cfg(feature = "reqwest")]
pub use verification::types::StatusCodeValid;

// clap types (feature-gated on clap-types)
#[cfg(feature = "clap-types")]
pub use primitives::clap_types::{
    ArgActionStyle, ArgGroupStyle, ArgStyle, ColorChoiceStyle, CommandStyle, ErrorKindStyle,
    IdStyle, PossibleValueStyle, ValueHintStyle, ValueRangeStyle, ValueSourceStyle,
};

// sqlx types (feature-gated on sqlx-types)
#[cfg(feature = "sqlx-types")]
pub use primitives::sqlx_types::{
    AnyQueryResultStyle, AnyTypeInfoStyle, ColumnDescriptorStyle, ColumnEntryStyle, RowDataStyle,
};
#[cfg(feature = "sqlx-types")]
pub use primitives::sqlx_types::{
    AnyTypeInfoKindStyle, ColumnValueStyle, DriverKindStyle, SqlTypeKindStyle, SqlxErrorKindStyle,
};
#[cfg(feature = "sqlx-types")]
pub use primitives::sqlx_types::{
    ColumnDescriptor, ColumnEntry, ColumnValue, DriverKind, RowData, SqlTypeKind,
};

// accesskit types (feature-gated on accesskit)
#[cfg(feature = "accesskit")]
pub use primitives::accesskit_types::{
    ActionStyle, AriaCurrentStyle, AutoCompleteStyle, HasPopupStyle, InvalidStyle, ListStyleStyle,
    LiveStyle, OrientationStyle, RoleStyle, ScrollHintStyle, ScrollUnitStyle, SortDirectionStyle,
    TextAlignStyle, TextDecorationStyleStyle, TextDirectionStyle, ToggledStyle,
    VerticalOffsetStyle,
};

// egui types (feature-gated on egui-types)
#[cfg(feature = "egui-types")]
pub use primitives::egui_types::{
    // Select trenchcoat wrappers
    AlignSelect,
    // Select enum styles
    AlignStyle,
    CursorIconSelect,
    CursorIconStyle,
    DirectionSelect,
    DirectionStyle,
    // Composite struct wrappers
    EguiColor32,
    EguiColor32Style,
    EguiCornerRadius,
    EguiCornerRadiusStyle,
    EguiFontId,
    EguiFontIdStyle,
    EguiMargin,
    EguiMarginStyle,
    EguiPos2,
    EguiPos2Style,
    EguiRect,
    EguiRectStyle,
    EguiShadow,
    EguiShadowStyle,
    EguiStroke,
    EguiStrokeStyle,
    EguiVec2,
    EguiVec2Style,
    FontFamilySelect,
    FontFamilyStyle,
    KeySelect,
    KeyStyle,
    OrderSelect,
    OrderStyle,
    PointerButtonSelect,
    PointerButtonStyle,
    TextStyleSelect,
    TextStyleStyle,
    TextWrapModeSelect,
    TextWrapModeStyle,
    TextureFilterSelect,
    TextureFilterStyle,
    TextureWrapModeSelect,
    TextureWrapModeStyle,
    ThemePreferenceSelect,
    ThemePreferenceStyle,
    ThemeSelect,
    ThemeStyle,
    TouchPhaseSelect,
    TouchPhaseStyle,
    UiKindSelect,
    UiKindStyle,
    WidgetTypeSelect,
    WidgetTypeStyle,
};

// ratatui types (feature-gated on ratatui)
#[cfg(feature = "ratatui")]
pub use primitives::ratatui_types::{
    // Trenchcoat wrappers (add JsonSchema for ElicitComplete)
    AlignmentSelect,
    // Select enum styles
    AlignmentStyle,
    BorderTypeSelect,
    BorderTypeStyle,
    BordersSelect,
    ColorSelect,
    ColorStyle,
    RatatuiDirectionSelect,
    RatatuiDirectionStyle,
    // Composite struct wrappers
    RatatuiMargin,
    RatatuiMarginStyle,
    RatatuiPadding,
    RatatuiPaddingStyle,
    RatatuiStyle,
    RatatuiStyleStyle,
    ScrollbarOrientationSelect,
};

// geo-types (feature-gated on geo-types)
#[cfg(feature = "geo-types")]
pub use primitives::geo_types::{
    GeoCoord, GeoCoordStyle, GeoLine, GeoLineStyle, GeoRect, GeoRectStyle,
};

// palette (feature-gated on palette)
#[cfg(feature = "palette")]
pub use primitives::palette_types::{PaletteSrgb, PaletteSrgbStyle};
