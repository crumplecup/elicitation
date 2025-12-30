# elicitation_derive

Derive macros for the [elicitation](https://crates.io/crates/elicitation) library.

[![Crates.io](https://img.shields.io/crates/v/elicitation_derive.svg)](https://crates.io/crates/elicitation_derive)
[![Documentation](https://docs.rs/elicitation_derive/badge.svg)](https://docs.rs/elicitation_derive)
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](../../LICENSE-APACHE)

This crate provides procedural macros for automatically implementing elicitation traits on custom types. It's typically used through the main `elicitation` crate.

## Features

- **`#[derive(Elicit)]`** - Automatic implementation of elicitation traits
- **Enum Support** - Generates `Select` pattern for unit variant enums
- **Struct Support** - Generates `Survey` pattern for structs
- **Attribute Support** - `#[prompt("...")]`, `#[skip]`, and more

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
elicitation = "0.2"
```

The derive macro is re-exported through the main crate:

```rust
use elicitation::Elicit;

#[derive(Debug, Elicit)]
enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Elicit)]
struct Task {
    #[prompt("What's the task title?")]
    title: String,

    priority: Priority,
}
```

## Attributes

### `#[prompt("...")]`

Customize the prompt text for types or fields:

```rust
#[derive(Elicit)]
#[prompt("Choose your favorite color:")]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Elicit)]
struct Config {
    #[prompt("Enter the server hostname:")]
    host: String,
}
```

### `#[skip]`

Skip a field during elicitation (uses `Default::default()`):

```rust
use chrono::{DateTime, Utc};

#[derive(Default, Elicit)]
struct Task {
    title: String,

    #[skip]
    created_at: DateTime<Utc>,
}
```

## Generated Implementations

### For Enums (Select Pattern)

```rust
#[derive(Elicit)]
enum Status {
    Active,
    Inactive,
}
```

Generates:
- `impl Prompt for Status`
- `impl Select for Status`
- `impl Elicitation for Status`

### For Structs (Survey Pattern)

```rust
#[derive(Elicit)]
struct Person {
    name: String,
    age: u8,
}
```

Generates:
- `impl Prompt for Person`
- `impl Survey for Person`
- `impl Elicitation for Person`

## Requirements

- Enum variants must be unit variants (no fields) in v0.2.0
- Struct fields must implement `Elicitation`
- Struct must implement `Default` if using `#[skip]` attribute

## Version History

See [CHANGELOG.md](../../CHANGELOG.md) for version history.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](../../LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Links

- [Main elicitation crate](https://crates.io/crates/elicitation)
- [Documentation](https://docs.rs/elicitation_derive)
- [Repository](https://github.com/crumplecup/elicitation)
