//! `#[reflect_trait]` factories for clap derive traits.
//!
//! These factories enable agents to call methods from [`clap::ValueEnum`],
//! [`clap::CommandFactory`], [`clap::Subcommand`], and [`clap::Args`] on any
//! user type that derives those traits — without the user needing to write
//! extra glue code.
//!
//! # Orphan rule note
//!
//! `ElicitProxy for clap::X` cannot be written in `elicit_clap` (orphan rule:
//! both the trait and the type are foreign).  Instead, `type_map(...)` in the
//! `#[reflect_trait]` attribute declares the substitution explicitly, using
//! the `From` impls that `elicit_newtype!` already provides.
//!
//! # Deferred: `FromArgMatches` / `Parser`
//!
//! `FromArgMatches` takes `&ArgMatches` (not Serialize/Clone) and
//! `Parser` extends it.  These cannot be wrapped as MCP tools.
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
/// - `augment_subcommands(cmd)` — augment a [`Command`] with this type's subcommands
/// - `augment_subcommands_for_update(cmd)` — augment for update
/// - `has_subcommand(name)` — check whether a subcommand with the given name exists
#[reflect_trait(clap::Subcommand,
    type_map(clap::Command => crate::Command))]
pub trait SubcommandTools {
    /// Augment `cmd` so it can instantiate this type's subcommands.
    fn augment_subcommands(cmd: clap::Command) -> clap::Command;
    /// Augment `cmd` for update from this type's subcommands.
    fn augment_subcommands_for_update(cmd: clap::Command) -> clap::Command;
    /// Returns `true` if a subcommand with `name` is registered on this type.
    fn has_subcommand(name: &str) -> bool;
}

// ── clap::ValueEnum ───────────────────────────────────────────────────────────

/// Expose [`clap::ValueEnum`] as agent-callable MCP tools.
///
/// Methods:
/// - `value_variants()` — all valid variants as a JSON array
/// - `to_possible_value(&self)` — the canonical [`PossibleValue`] for this variant
/// - `from_str(input, ignore_case)` — parse a string back to this enum value
#[reflect_trait(clap::ValueEnum,
    type_map(clap::builder::PossibleValue => crate::PossibleValue))]
pub trait ValueEnumTools {
    /// All valid variants of this enum, in display order.
    fn value_variants() -> &'static [Self];
    /// Returns the canonical [`PossibleValue`] for this variant, if any.
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue>;
    /// Parse `input` as a value of this enum, optionally ignoring case.
    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String>;
}

// ── clap::Args ────────────────────────────────────────────────────────────────

/// Expose [`clap::Args`] as agent-callable MCP tools.
///
/// Methods:
/// - `augment_args(cmd)` — augment a [`Command`] with this type's arguments
/// - `augment_args_for_update(cmd)` — augment for update
/// - `group_id()` — the [`Id`] of the [`clap::ArgGroup`] for this set, if any
#[reflect_trait(clap::Args,
    type_map(clap::Command => crate::Command, clap::Id => crate::Id))]
pub trait ArgsTools {
    /// Augment `cmd` with this type's arguments.
    fn augment_args(cmd: clap::Command) -> clap::Command;
    /// Augment `cmd` for update from this type's arguments.
    fn augment_args_for_update(cmd: clap::Command) -> clap::Command;
    /// Returns the [`Id`] of the [`clap::ArgGroup`] for this argument set, if any.
    fn group_id() -> Option<clap::Id>;
}
