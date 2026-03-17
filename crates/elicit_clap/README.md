# elicit_clap

MCP tool transport for [`clap`](https://docs.rs/clap) — expose clap derive
traits and type introspection as agent-callable MCP tools.

## What this crate provides

Two complementary layers:

1. **Newtype wrappers** — `serde` + `JsonSchema`-enabled newtypes for every
   clap type that lacks those impls, so clap values can cross the MCP boundary
2. **Trait factories** — `#[reflect_trait]` factories for the four clap derive
   traits, so agents can call `command()`, `value_variants()`, etc. on any user
   type that derives those traits

---

## Newtype wrappers

| Wrapper | Inner type | Pattern |
|---|---|---|
| [`ColorChoice`] | `clap::ColorChoice` | Select enum |
| [`ArgAction`] | `clap::ArgAction` | Select enum |
| [`ValueHint`] | `clap::ValueHint` | Select enum |
| [`ValueSource`] | `clap::parser::ValueSource` | Select enum |
| [`ErrorKind`] | `clap::error::ErrorKind` | Select enum |
| [`Id`] | `clap::Id` | Primitive |
| [`PossibleValue`] | `clap::builder::PossibleValue` | Survey |
| [`ValueRange`] | `clap::builder::ValueRange` | Survey |
| [`Arg`] | `clap::Arg` | Survey builder |
| [`ArgGroup`] | `clap::ArgGroup` | Survey builder |
| [`Command`] | `clap::Command` | Survey builder |

Every wrapper implements `Deref`/`DerefMut`, `From`/`Into` (lossless
round-trip), `Serialize`, `Deserialize`, and `JsonSchema`.  The `Arg`,
`ArgGroup`, and `Command` wrappers also expose introspection methods via
`#[reflect_methods]` (e.g. `get_id()`, `get_long()`, `get_help()`).

### Why wrappers are needed

Clap types intentionally do not implement `serde` or `schemars::JsonSchema`
(they live at the CLI boundary, not the data boundary).  Without those impls,
the types cannot appear in MCP tool parameter schemas.  The wrappers add the
missing impls without modifying upstream code, and `type_map(...)` in the
`#[reflect_trait]` attribute wires them into factory dispatch.

---

## Trait factories

The four clap derive traits become per-type MCP tool factories.  Registration
follows the standard three-step lifecycle:

1. **Prime** — store the vtable for `T` in the global registry
2. **register_type** — claim a prefix for `T` in a `DynamicToolRegistry`
3. **instantiate** — materialise the per-type tools and notify the agent

### `clap::CommandFactory`

`#[derive(clap::Parser)]` implies `CommandFactory`.  Exposes:

| Tool | Description |
|---|---|
| `{prefix}__command` | Build the `Command` that can instantiate this type |
| `{prefix}__command_for_update` | Build the `Command` used when updating an existing instance |

```rust,no_run
use elicit_clap::trait_factories::prime_clap__command_factory;
use elicitation::DynamicToolRegistry;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use elicitation_derive::Elicit;

#[derive(clap::Parser, Serialize, Deserialize, JsonSchema, Elicit)]
struct MyCli {
    /// Output path
    #[arg(short, long)]
    output: String,
}

#[tokio::main]
async fn main() {
    prime_clap__command_factory::<MyCli>();
    let registry = DynamicToolRegistry::new().register_type::<MyCli>("cli");
    registry.instantiate("clap::CommandFactory", "cli").await.unwrap();
    // Exposes: cli__command, cli__command_for_update
}
```

### `clap::ValueEnum`

For enums that represent a fixed set of CLI values.  Exposes:

| Tool | Description |
|---|---|
| `{prefix}__value_variants` | All valid variants as a JSON array |
| `{prefix}__to_possible_value` | The canonical `PossibleValue` for a given variant |
| `{prefix}__from_str` | Parse a string back to this enum value |

```rust,no_run
use elicit_clap::trait_factories::prime_clap__value_enum;
use elicitation::DynamicToolRegistry;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use elicitation_derive::Elicit;

#[derive(clap::ValueEnum, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
enum OutputFormat { Text, Json, Yaml }

#[tokio::main]
async fn main() {
    prime_clap__value_enum::<OutputFormat>();
    let registry = DynamicToolRegistry::new().register_type::<OutputFormat>("fmt");
    registry.instantiate("clap::ValueEnum", "fmt").await.unwrap();
    // Exposes: fmt__value_variants, fmt__to_possible_value, fmt__from_str
}
```

### `clap::Args`

For structs that contribute a group of arguments to a parent command.  Exposes:

| Tool | Description |
|---|---|
| `{prefix}__augment_args` | Augment a `Command` with this type's arguments |
| `{prefix}__augment_args_for_update` | Augment for update |
| `{prefix}__group_id` | The `Id` of the `ArgGroup` for this set, if any |

```rust,no_run
use elicit_clap::trait_factories::prime_clap__args;
use elicitation::DynamicToolRegistry;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use elicitation_derive::Elicit;

#[derive(clap::Args, Serialize, Deserialize, JsonSchema, Elicit)]
struct CommonArgs {
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() {
    prime_clap__args::<CommonArgs>();
    let registry = DynamicToolRegistry::new().register_type::<CommonArgs>("common");
    registry.instantiate("clap::Args", "common").await.unwrap();
    // Exposes: common__augment_args, common__augment_args_for_update, common__group_id
}
```

### `clap::Subcommand`

For enums whose variants are subcommands.  Exposes:

| Tool | Description |
|---|---|
| `{prefix}__augment_subcommands` | Augment a `Command` with this type's subcommands |
| `{prefix}__augment_subcommands_for_update` | Augment for update |
| `{prefix}__has_subcommand` | Check whether a named subcommand exists |

```rust,no_run
use elicit_clap::trait_factories::prime_clap__subcommand;
use elicitation::DynamicToolRegistry;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use elicitation_derive::Elicit;

#[derive(clap::Subcommand, Serialize, Deserialize, JsonSchema, Elicit)]
enum Subcmd { Build, Test, Run }

#[tokio::main]
async fn main() {
    prime_clap__subcommand::<Subcmd>();
    let registry = DynamicToolRegistry::new().register_type::<Subcmd>("sub");
    registry.instantiate("clap::Subcommand", "sub").await.unwrap();
    // Exposes: sub__augment_subcommands, sub__augment_subcommands_for_update, sub__has_subcommand
}
```

---

## The `type_map` mechanism

`clap` methods return and accept native clap types (`clap::Command`,
`clap::builder::PossibleValue`, `clap::Id`).  These types don't implement
`JsonSchema` or `serde`, so they cannot cross the MCP boundary as-is.

The `#[reflect_trait]` attribute accepts a `type_map(A => B)` argument that
declares substitutions: wherever the factory sees type `A` in a method
signature, it uses wrapper `B` instead.  The `From` impls on the wrappers
handle the conversion transparently.

```rust,ignore
#[reflect_trait(clap::CommandFactory,
    type_map(clap::Command => crate::Command))]
pub trait CommandFactoryTools { ... }
```

The orphan rule prevents writing `ElicitProxy for clap::Command` directly
(both trait and type are foreign), but `type_map` sidesteps this entirely —
the substitution happens at the factory layer, not via a trait impl.

---

## Deferred: `FromArgMatches` / `Parser`

`clap::FromArgMatches` takes `&ArgMatches` (not `Serialize`/`Clone`) and
`clap::Parser` extends it.  `ArgMatches` cannot be passed over an MCP tool
boundary in a meaningful way, so these two traits are not wrapped.

---

## Dependency

```toml
[dependencies]
elicit_clap = { path = "../elicit_clap" }
```
