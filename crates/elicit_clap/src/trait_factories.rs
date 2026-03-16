//! `#[reflect_trait]` factories for clap derive traits.
//!
//! These factories enable agents to call methods from [`clap::ValueEnum`],
//! [`clap::CommandFactory`], and [`clap::Subcommand`] on any user type that
//! derives those traits — without the user needing to write extra glue code.
//!
//! # Orphan rule note
//!
//! `ElicitProxy for clap::X` cannot be written in `elicit_clap` (orphan rule:
//! both the trait and the type are foreign).  Instead, `type_map(...)` in the
//! `#[reflect_trait]` attribute declares the substitution explicitly, using
//! the `From` impls that `elicit_newtype!` already provides.
//!
//! # Usage
//!
//! At server startup, for each user type `T` implementing the clap trait,
//! call the generated `prime_*::<T>()` function alongside
//! `DynamicToolRegistry::register_type::<T>(prefix)`.
//!
//! ```rust,ignore
//! prime_clap__commandfactory::<MyParser>();
//! registry.register_type::<MyParser>("myapp").await;
//! ```

use elicitation_macros::reflect_trait;

// ── clap::CommandFactory ──────────────────────────────────────────────────────

/// Expose [`clap::CommandFactory`] as agent-callable MCP tools.
///
/// Methods:
/// - `command()` — build the [`Command`] for this type
/// - `command_for_update()` — build the update [`Command`] for this type
#[reflect_trait(clap::CommandFactory,
    type_map(clap::Command => crate::Command))]
pub trait CommandFactoryTools {
    /// Build the [`Command`] that can instantiate this type.
    fn command() -> clap::Command;
    /// Build the [`Command`] used when updating an existing instance.
    fn command_for_update() -> clap::Command;
}

// ── clap::Subcommand ──────────────────────────────────────────────────────────

/// Expose [`clap::Subcommand`] as agent-callable MCP tools.
///
/// Methods:
/// - `has_subcommand(name)` — check whether a subcommand with the given name exists
#[reflect_trait(clap::Subcommand)]
pub trait SubcommandTools {
    /// Returns `true` if a subcommand with `name` is registered on this type.
    fn has_subcommand(name: &str) -> bool;
}

// ── clap::ValueEnum ───────────────────────────────────────────────────────────

/// Expose [`clap::ValueEnum`] as agent-callable MCP tools.
///
/// Methods:
/// - `to_possible_value(&self)` — the canonical [`PossibleValue`] for this variant
/// - `from_str(input, ignore_case)` — parse a string back to this enum value
///
/// Note: `value_variants()` returns `&'a [Self]` (a reference to a static
/// slice).  This return type cannot be handled by the `type_map` or
/// `ElicitProxy` systems, so it is exposed instead via `#[reflect_methods]`
/// on each individual newtype (see e.g. `ColorChoice`).
#[reflect_trait(clap::ValueEnum,
    type_map(clap::builder::PossibleValue => crate::PossibleValue))]
pub trait ValueEnumTools {
    /// Returns the canonical [`PossibleValue`] for this variant, if any.
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue>;
    /// Parse `input` as a value of this enum, optionally ignoring case.
    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String>;
}
