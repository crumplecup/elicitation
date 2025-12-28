Elicitation

Macros for common elicitation patterns in Rust.

The library uses traits to implement a common interface over elicitation requirements, so that handling individual types is monomorphized with compile-time guarantees, while the trait methods provide a common API that is amenable to applying proc macros.

For MCP clients, elicitation provides guardrails to a conversation, designed to produce type guarantees against the content result. Clients use elicitation tools to provide stronger guarantees on responses from users than open ended conversation provides. The best analogy for guiding user input would be the various UI widgets used to sanitize user inputs. A calendar picker ensures dates are valid, a dropdown menu ensures the user selects a valid option from a list of products, radio buttons ensure at least one valid shipping option is selected. Elicitations tools are the MCP equivalent to this process, and may directly employ or lean on the UI methods mentioned here.

At the most basic level, we provide elicitation impls for basic Rust types. These impls expose elicitation methods on arbitrary rust types, with parameters that can be configured by the user through traditional macro decorators. But what makes this library exciting and powerful are the sets of macros for structs and enums that drive our advanced elicitation patterns for categorical types and sequential transitions. Combined, these pattern operators provide a functional grammar that allows the construction of complex elicitation state machines that can satisfy strict design or business requirements.

Imagine a trait named Elicit with a method named elicit.

```{rust}
trait Elicit {
	type Response;
	type Error;
	fn elicit() -> Result<Response, Error>;
}
```

Note that we need to call elicit on things we don’t have yet. If we knew what we wanted, we wouldn’t have to elicit the response from the user. We can’t make any guarantees on what the elicit method will produce, because we don’t know anything about the target type, it could be anything. It may help to think of Response as code for the target type.

Let’s imagine the simplest type in Rust, the marker or unit type. Creating a new instance is trivial, If we were to produce a prompt for the user, it could be something like “Would you like to create an instance of X?”, we would know what code to write, and it would automatically succeed. So this code:

```
#[elicit]
struct Marker;

let instance = Marker::elicit()?;
```

Could prompt the user, would you like to create this? If the user says no, the operation fails, if yes, it succeeds.

Let’s talk about a struct with fields:

```
#[elicit]
struct Person {
	#[prompt(“What is your name?”)]
	name: String,
	#[prompt(“What is your age?”)]
	age: u8,
}

let person = Person::elicit()?;
```

Here I have introduced a _prompt_ decorator that will presumably guide the user toward a valid response. The details of extracting a valid response are intentionally glossed over here as an implementation detail for the library author. The trait simply acknowledges that elicitation is taking place, and provides an interface for exchange.

The only types where the elicitation library needs to worry about are the primitive types and basics like String from the standard library. The strictest way to handle it is sort of already defined for us, if the input is not strictly the type of the target output, we have a failure, otherwise a success. Less strictly, can we call into() on it to get the target type? Beyond that, we can’t really make any assumptions.

With multiple fields, a reasonable default would be to prompt the user for each field in sequence, although nothing prevents us from eliciting fields in random order through, for eg, an async interface. Multiple fields introduce the concept of state transition, in that we must traverse through multiple generation phases before arriving at a final state. Async models these state transitions for us, but the elicitation process lends itself naturally to state machines as an expression pattern, and we will exploit that there.

Before we do, let’s explore the idea of categorical elicitation using enums:

```
#[elicit]
struct Person {
	#[prompt(“What is your name?”)]
	name: String,
	#[prompt(“What is your age?”)]
	age: u8,
	#[prompt(“What is your favorite color?”]
	favorite_color: Color,
}

#[elicit]
enum Color {
	Red,
	Orange,
	Yellow,
	Green,
	Blue,
}
```

Enum elicitation is analogous to the dropdown menu, where the user has a limited range of acceptable responses. The user may prefer purple, or be too clever about it and say “chartreuse” instead of green, enums keep the options to valid variants only. We might include some alts to facilitate mapping, so the developer can hedge their bets:

```
#[elicit]
enum Color {
	#[alts([‘cherry’, ‘rose’, ‘pink’, ‘fire’, ‘ruby’])]
	Red,
	#[alts([‘brown’, ‘umber’, ‘amber’])]
	Orange,
	#[alts([‘honey’, ‘sunshine’, ‘topaz’])]
	Yellow,
	#[alts([‘lime’, ‘leaf’, ‘forest’, ‘emerald’])]
	Green,
	#[alts([‘sky’, ‘sea’, ‘saphire’])]
	Blue,
}
```

Ultimately, the strong type interface is again our friend, as the success or failure of the elicitation operation is entirely based on whether the result is a valid variant, so program invariants are directly represented in the type system.

To use another metaphor, elicitation patterns are similar to the builder patterns, in that they provide a series of methods to help the user construct a valid final type incrementally, but these methods are conversationally guided by the LLM instead of run in a script, so it provides a “builder pattern” that you can use when conversing with an LLM. Using the elicitation builder pattern, we can construct more complex types with stronger validity guarantees by putting the user on rails defined by elicitation macros.

For structs, the elicit macro goes field by field and calls elicit on each value, querying the user about their name, age and favorite color, in the case of our Person type. Behind the scenes we have a state machine that looks for a valid name before proceeding to age, then color, ensuring a valid Person at the end, providing the same validity guarantees as a builder pattern with the added state machine promise that all fields will be elicited, ensuring no details are forgotten. For complex structures, like config structs, state machines can help the LLM to track progress and stay on topic during a long elicitation chain.

State machines can also be composed through nested structs to expose more complex state transitions:

```
#[derive(Elicit)]
struct Persons(Vec<Person>);
```

# Interaction Paradigms: A Vocabulary for Conversational Elicitation

Up to this point, we’ve focused on _what_ elicitation produces: strongly typed Rust values constructed through guided interaction with an LLM. But equally important is _how_ those values are elicited. Just as graphical user interfaces rely on a set of well‑understood interaction primitives—dropdowns, checkboxes, dialogs, wizards—conversational elicitation requires its own vocabulary of interaction patterns.

These patterns are not tied to specific Rust types. Instead, they describe the _mode of interaction_ used to obtain a value. This distinction is crucial: the same type may be elicited through different paradigms depending on context, and the same paradigm may apply to many different types. To capture this orthogonality, the elicitation library introduces a set of traits representing core conversational interaction modes.

These traits form the foundation of a “conversational UI toolkit” for LLM‑driven applications.

---

## Core Interaction Paradigms

### Select

`Select` represents the act of choosing one value from a finite set of options. This is the conversational analogue of a dropdown menu or a radio‑button group.

It is the natural elicitation mode for:

- enums
- categorical fields
- permission menus
- configuration presets
- any situation where the user must choose from a constrained set

A `Select` interaction guarantees that the result is one of the valid variants, and MCP tools can enforce this constraint through schemas and structured responses.

---

### Affirm

`Affirm` captures binary confirmation: yes/no, true/false, allow/deny. This is the conversational equivalent of a checkbox or a confirmation dialog.

It is used for:

- boolean fields
- confirmation prompts
- safety checks
- “are you sure?” interactions

Although simple, `Affirm` is one of the most important paradigms because natural language contains many ways to express agreement or refusal. The elicitation tool handles synonym mapping, ambiguity detection, and reprompting, ensuring a clean boolean result.

---

### Survey

`Survey` represents a structured, multi‑question interaction. This is the conversational form of a traditional form or wizard: a sequence of prompts that together produce a composite value.

It is the natural elicitation mode for:

- structs
- configuration objects
- onboarding flows
- multi‑field data entry
- nested elicitation sequences

A `Survey` interaction is internally modeled as a state machine: each field is elicited in turn, with the ability to track progress, handle errors, and maintain conversational focus across multiple steps.

---

### Authorize

`Authorize` represents permission‑granting interactions, where the user chooses a policy rather than a simple yes/no answer. This is the conversational equivalent of a permission dialog with multiple options.

For example:

1. Allow this action once
2. Allow this action for the entire session
3. Decline and provide an alternative

This pattern appears frequently in MCP workflows, where tools may require explicit user approval. `Authorize` generalizes this into a reusable elicitation mode that can be applied anywhere a permission or policy decision is needed.

---

## Why Interaction Paradigms Matter

These paradigms give the elicitation system expressive power beyond simple type conversion. They allow developers to describe _how_ a value should be obtained, not just _what_ the value should be. This separation of concerns has several benefits:

### 1. Stronger Guarantees

Each paradigm maps to an MCP tool with a well‑defined schema. This ensures that:

- the LLM stays within the allowed response space
- invalid or ambiguous answers are caught early
- the final value satisfies the type’s invariants

### 2. Better User Experience

Different kinds of questions require different conversational structures. A yes/no question should not feel like a multiple‑choice menu, and a multi‑field struct should not be elicited in a single blob of text. Interaction paradigms let the developer choose the right tool for the job.

### 3. Extensibility

Because paradigms are expressed as traits, new interaction modes can be added without modifying existing types or macros. Future paradigms—such as ranked choice, multi‑select, or conditional branching—can be introduced incrementally.

### 4. Declarative Developer API

Developers can annotate fields with attributes that specify the desired interaction mode:

```rust
#[affirm("Enable logging?")]
logging: bool,

#[select("Choose a mode", options = ["Fast", "Safe", "Debug"])]
mode: Mode,

#[survey]
config: Config,

#[authorize("May the LLM use this tool?")]
permission: PermissionPolicy,
```

The derive macro uses these annotations to select the appropriate paradigm and generate the corresponding elicitation logic.

---

## A Roadmap for Growth

The initial set of paradigms—**Select**, **Affirm**, **Survey**, and **Authorize**—covers the majority of real‑world elicitation needs. They provide a solid foundation for building structured, reliable conversational workflows.

As the library evolves, additional paradigms can be introduced:

- multi‑select
- ranked choice
- numeric input with ranges
- freeform text with validation
- repeatable blocks (`Vec<T>`)
- optional fields (`Option<T>`)
- branching surveys

Because the system is built on traits and macros, these additions can be made without breaking existing code or changing the core elicitation model.

---

## Conclusion

Interaction paradigms give elicitation its expressive power. They transform the raw idea of “ask the user for a value” into a structured, predictable, and type‑safe process. By modeling conversational patterns as reusable traits, the elicitation library provides a flexible foundation for building complex, guided interactions that remain grounded in Rust’s strong type system.

These paradigms are the missing vocabulary that turns elicitation from a clever trick into a coherent design framework—one that can scale from simple prompts to full conversational state machines.

# Trait‑Based Metadata: The Foundation of Elicitation

Elicitation relies on a simple but powerful idea: **types describe how they should be elicited through traits**.  
Instead of storing metadata in sidecar objects or descriptor structs, we use Rust’s trait system to express the information needed to drive conversational interaction. This keeps the design fully static, compile‑time, and idiomatic.

In this model:

- **Traits define the interaction paradigm** (Select, Affirm, Survey, Authorize)
- **Traits also define the metadata required for that paradigm**
- **Default trait methods provide fallback behavior**
- **Types override only what they need**
- **The derive macro generates trait implementations automatically**

This pattern eliminates duplication, avoids runtime reflection, and produces a clean, extensible architecture.

---

## The Prompt Trait: Shared Metadata for All Interactions

Many elicitation patterns require a prompt. Instead of repeating this requirement across multiple traits, we define a single shared trait:

```rust
pub trait Prompt {
    fn prompt() -> Option<&'static str> {
        None
    }
}
```

Any type can override this to provide a custom prompt.  
All interaction paradigms depend on `Prompt`, so they automatically inherit this behavior.

This avoids duplication and centralizes prompt logic in one place.

---

## Interaction Paradigms as Traits

Each interaction paradigm is expressed as a trait that **extends `Prompt`** and defines only the metadata specific to that paradigm.

### Select

```rust
pub trait Select: Prompt + Sized {
    fn options() -> &'static [Self];
    fn labels() -> &'static [&'static str] {
        &[]
    }
}
```

`Select` requires:

- a list of valid options
- optional labels or synonyms

The prompt comes from the `Prompt` trait.

---

### Affirm

```rust
pub trait Affirm: Prompt {}
```

`Affirm` needs no additional metadata beyond the prompt.  
The elicitation logic handles yes/no parsing, synonyms, and validation.

---

### Survey

```rust
pub trait Survey: Prompt {
    fn fields() -> &'static [FieldInfo];
}
```

`Survey` describes multi‑field elicitation (structs).  
The derive macro generates the `fields()` implementation automatically.

---

### Authorize

```rust
pub trait Authorize: Prompt {
    fn policies() -> &'static [Self];
}
```

`Authorize` is a specialized form of `Select` used for permission dialogs and policy decisions.

---

## How Traits Replace Metadata Objects

In many systems, metadata is stored in runtime structures like descriptor objects.  
In Rust, traits _are_ the metadata.

For example, instead of:

```rust
descriptor.prompt
descriptor.options
descriptor.range
```

We simply call:

```rust
T::prompt()
T::options()
T::range()
```

This approach has several advantages:

- **Compile‑time guarantees**
- **No heap allocation or dynamic dispatch**
- **No reflection or runtime metadata**
- **Monomorphized, optimized code**
- **Macros generate trait impls, not data structures**

Traits give us a static, type‑driven way to describe elicitation behavior.

---

## How the Derive Macro Uses This Pattern

When a user writes:

```rust
#[derive(Elicit)]
struct Person {
    #[prompt("What is your name?")]
    name: String,

    #[prompt("What is your age?")]
    age: u8,

    #[prompt("What is your favorite color?")]
    favorite_color: Color,
}
```

The derive macro generates:

- `impl Prompt for Person`
- `impl Survey for Person`
- `impl Prompt for each field type (if annotated)`
- `impl Select for enums`
- `impl Affirm for bool`
- etc.

The macro does not generate metadata objects.  
It generates **trait implementations**, which the elicitation engine calls directly.

---

## How MCP Tools Consume Trait Metadata

Each interaction paradigm maps to an MCP tool.  
The tool receives the metadata by calling the trait methods.

Example for `Select`:

```rust
fn elicit<T: Select + Elicit>() -> Result<T, ElicitError> {
    let prompt = T::prompt().unwrap_or("Please select an option:");
    let labels = T::labels();

    let raw = call_mcp_tool("elicit_select", json!({
        "prompt": prompt,
        "options": labels,
    }))?;

    T::from_label(raw)
}
```

The tool enforces:

- schema validation
- allowed values
- structured responses

The type provides:

- options
- labels
- prompt

The interaction trait provides:

- the elicitation logic

This separation of concerns keeps the system clean and modular.

---

## Why This Pattern Works

### 1. Zero Duplication

Prompt logic lives in one trait.  
Interaction traits define only what they need.

### 2. Fully Static

All metadata is known at compile time.  
No dynamic descriptors or runtime reflection.

### 3. Macro‑Friendly

Macros generate trait impls, not data structures.

### 4. Extensible

Adding a new interaction paradigm is as simple as adding a new trait.

### 5. Idiomatic Rust

Traits express capability and metadata.  
Types opt into behaviors by implementing traits.

---

## Conclusion

The trait‑based metadata pattern is the backbone of the elicitation system.  
It provides a clean, idiomatic way to describe how types should be elicited, without relying on runtime metadata or descriptor objects. By expressing interaction paradigms as traits and using default methods to supply optional metadata, the system remains flexible, composable, and fully static.

This pattern allows the elicitation library to define behavior for primitive and standard library types, while giving users the power to extend elicitation naturally to their own types through trait implementations and derive macros.

```
// --------------------------------------
// Core traits (defined by the elicitation crate)
// --------------------------------------

pub trait Prompt {
    fn prompt() -> Option<&'static str> {
        None
    }
}

pub trait Elicit: Sized + Prompt {
    async fn elicit(client: &pmcp::Client) -> Result<Self, ElicitError>;
}

// --------------------------------------
// Canary implementation for i8
// (this is OUR implementation, shipped in the crate)
// --------------------------------------

impl Prompt for i8 {
    fn prompt() -> Option<&'static str> {
        Some("Please enter an integer between -128 and 127.")
    }
}

impl Elicit for i8 {
    async fn elicit(client: &pmcp::Client) -> Result<Self, ElicitError> {
        let prompt = Self::prompt().unwrap();

        // pmcp handles transport, JSON, and tool invocation.
        let response = client
            .call_tool("elicit_number", pmcp::json!({
                "prompt": prompt,
                "min": i8::MIN,
                "max": i8::MAX,
            }))
            .await
            .map_err(ElicitError::ToolError)?;

        // Parse the result into an i8.
        match response {
            pmcp::Value::Number(n) => {
                let v = n.as_i64().ok_or(ElicitError::InvalidFormat)?;
                i8::try_from(v).map_err(|_| ElicitError::OutOfRange)
            }
            pmcp::Value::String(s) => {
                s.trim().parse::<i8>().map_err(|_| ElicitError::InvalidFormat)
            }
            _ => Err(ElicitError::InvalidFormat),
        }
    }
}

// --------------------------------------
// Error type for the elicitation crate
// --------------------------------------

#[derive(Debug)]
pub enum ElicitError {
    InvalidFormat,
    OutOfRange,
    ToolError(pmcp::Error),
}
```

# Trait Architecture for Generic, Extensible Elicitation

The elicitation system relies on a simple but powerful design principle:

> **Traits provide the metadata required to construct MCP tool calls.  
> The elicitation engine provides the behavior.  
> Macros and generics provide the scalability.**

This separation of concerns allows the elicitation crate to implement all primitive and standard library types once, while enabling downstream developers to extend elicitation to their own types with minimal effort.

This chapter describes the trait architecture that makes this possible.

---

## Goals of the Trait Architecture

The traits must:

- express only the metadata required to elicit a value
- avoid embedding type‑specific logic
- avoid embedding MCP tool logic
- be composable and minimal
- support generic implementations for entire families of types
- support derive‑based implementations for user‑defined types

The result is a system where:

- **we** (the elicitation crate authors) implement primitives and standard types
- **users** implement nothing unless they want custom prompts or behaviors
- **derive macros** generate trait impls for enums and structs
- **generic macros** implement all integer and float types
- **the elicitation engine** handles parsing, retries, and MCP integration

---

## Core Traits

The core of the system consists of three categories of traits:

1. **Shared metadata traits**
2. **The umbrella elicitation trait**
3. **Interaction paradigm traits**

Each trait is intentionally small and declarative.

---

## 1. Shared Metadata: `Prompt`

Many elicitation flows require a prompt. Instead of duplicating this across multiple traits, we define a single shared trait:

```rust
pub trait Prompt {
    fn prompt() -> Option<&'static str> {
        None
    }
}
```

Types may override this directly, or the derive macro may generate an implementation based on field attributes.

All interaction paradigms inherit this behavior.

---

## 2. The Umbrella Trait: `Elicit`

This is the trait that end users interact with:

```rust
pub trait Elicit: Sized + Prompt {
    async fn elicit(client: &pmcp::Client) -> Result<Self, ElicitError>;
}
```

Every type that can be elicited implements this trait.  
Primitive types implement it in the elicitation crate.  
User types get it via `#[derive(Elicit)]`.

The `Elicit` trait does **not** define how elicitation works.  
It simply defines the entry point.

---

## 3. Interaction Paradigm Traits

Each interaction pattern is expressed as a trait that provides only the metadata required for that pattern. The elicitation engine uses these traits to construct MCP tool calls.

### Select

```rust
pub trait Select: Prompt + Sized {
    fn options() -> &'static [Self];
    fn labels() -> &'static [&'static str];
}
```

Used for enums and other finite-choice types.

### Affirm

```rust
pub trait Affirm: Prompt {}
```

Used for booleans and yes/no confirmations.

### Survey

```rust
pub trait Survey: Prompt {
    fn fields() -> &'static [FieldInfo];
}
```

Used for structs and multi-step elicitation flows.

### Authorize

```rust
pub trait Authorize: Prompt {
    fn policies() -> &'static [Self];
}
```

Used for permission and policy decisions.

These traits contain **no behavior** — only metadata.  
This is what makes generic implementations possible.

---

## Why This Design Enables Generic Implementations

Because the traits expose only metadata, not behavior, we can implement entire families of types with a single macro.

### Example: All integer types

```rust
impl_integer_elicit!(i8);
impl_integer_elicit!(i16);
impl_integer_elicit!(i32);
impl_integer_elicit!(u8);
impl_integer_elicit!(u16);
impl_integer_elicit!(u32);
```

Each expansion:

- implements `Prompt` with a default prompt
- implements `Elicit` using the same MCP tool (`elicit_number`)
- uses the type’s own `MIN` and `MAX`
- uses generic parsing logic

No type-specific logic is required.

### Example: All enums

The derive macro generates:

- `impl Prompt`
- `impl Select`
- `impl Elicit`

based on the enum’s variants and attributes.

### Example: All structs

The derive macro generates:

- `impl Prompt`
- `impl Survey`
- `impl Elicit`

based on the struct’s fields and attributes.

### Example: Containers

`Option<T>` and `Vec<T>` can be implemented generically because they simply delegate to `T`.

---

## What the Traits Do _Not_ Do

Traits do **not**:

- validate input
- parse JSON
- retry on failure
- call MCP tools
- enforce min/max
- generate state machines

All of that lives in the elicitation engine.

Traits only describe what the type _is_, not how to elicit it.

This is the key to making the system scalable and maintainable.

---

## Summary

The trait architecture is intentionally minimal:

- `Prompt` provides shared metadata
- `Elicit` provides the entry point
- Interaction traits provide pattern-specific metadata

This design enables:

- generic implementations for primitive families
- derive-based implementations for user types
- clean separation between metadata and behavior
- a small, elegant core implementation
- a powerful, extensible system for downstream developers

By keeping traits declarative and metadata-only, the elicitation crate can implement all standard types once, while giving users a simple and ergonomic way to extend elicitation to their own types.

# Core Implementations for Standard Types

The elicitation crate provides first‑class implementations of `Elicit` (and the relevant interaction traits) for all primitive and standard library types. These implementations form the foundation that downstream developers rely on when deriving elicitation for their own types.

This chapter describes how the crate implements these types, how generics and macros reduce duplication, and how the trait architecture enables a clean, scalable design.

---

## Design Principles

The core implementations follow a few simple rules:

### 1. **Traits provide metadata, not behavior**

Primitive types implement:

- `Prompt` (to supply a default prompt)
- `Elicit` (to define the MCP tool call)

They do **not** implement parsing, retries, or validation.  
Those responsibilities belong to the elicitation engine and the MCP tools.

### 2. **Families of types share generic implementations**

All integer types share the same elicitation pattern.  
All floating‑point types share another.  
All enums share another.  
All structs share another.

Macros and generics eliminate duplication.

### 3. **Defaults are sensible but overridable**

Primitive types provide default prompts, but developers can override them using attributes on their own types.

### 4. **MCP tools define the interaction**

The elicitation crate constructs the tool call; the tool handles:

- user interaction
- validation
- retries
- structured responses

---

## Integer Types

All integer types (`i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`) share the same elicitation pattern:

- They use the `elicit_number` MCP tool.
- They provide a default prompt.
- They pass their type’s `MIN` and `MAX` to the tool.
- They parse the result using a generic helper.

### Macro Implementation

The crate defines a macro to implement all integer types at once:

```rust
macro_rules! impl_integer_elicit {
    ($t:ty) => {
        impl Prompt for $t {
            fn prompt() -> Option<&'static str> {
                Some(concat!("Please enter a ", stringify!($t), "."))
            }
        }

        impl Elicit for $t {
            async fn elicit(client: &pmcp::Client) -> Result<Self, ElicitError> {
                let prompt = Self::prompt().unwrap();

                let raw = client
                    .call_tool("elicit_number", pmcp::json!({
                        "prompt": prompt,
                        "min": <$t>::MIN,
                        "max": <$t>::MAX,
                    }))
                    .await
                    .map_err(ElicitError::ToolError)?;

                parse_integer::<$t>(raw)
            }
        }
    };
}
```

The crate then applies this macro to all integer types:

```rust
impl_integer_elicit!(i8);
impl_integer_elicit!(i16);
impl_integer_elicit!(i32);
impl_integer_elicit!(i64);
impl_integer_elicit!(u8);
impl_integer_elicit!(u16);
impl_integer_elicit!(u32);
impl_integer_elicit!(u64);
```

### Parsing Helper

The parsing logic is centralized:

```rust
fn parse_integer<T: TryFrom<i64>>(raw: pmcp::Value) -> Result<T, ElicitError> {
    match raw {
        pmcp::Value::Number(n) => {
            let v = n.as_i64().ok_or(ElicitError::InvalidFormat)?;
            T::try_from(v).map_err(|_| ElicitError::OutOfRange)
        }
        pmcp::Value::String(s) => {
            s.trim().parse::<i64>()
                .ok()
                .and_then(|v| T::try_from(v).ok())
                .ok_or(ElicitError::InvalidFormat)
        }
        _ => Err(ElicitError::InvalidFormat),
    }
}
```

---

## Floating‑Point Types

Floating‑point types (`f32`, `f64`) follow a similar pattern but use a different MCP tool:

```rust
impl_float_elicit!(f32);
impl_float_elicit!(f64);
```

The macro:

- provides a default prompt
- calls `elicit_float`
- parses JSON numbers or strings into the target float type

---

## Boolean Type

Booleans use the **Affirm** interaction pattern:

```rust
impl Prompt for bool {
    fn prompt() -> Option<&'static str> {
        Some("Please answer yes or no.")
    }
}

impl Affirm for bool {}

impl Elicit for bool {
    async fn elicit(client: &pmcp::Client) -> Result<Self, ElicitError> {
        let prompt = Self::prompt().unwrap();

        let raw = client
            .call_tool("elicit_bool", pmcp::json!({ "prompt": prompt }))
            .await
            .map_err(ElicitError::ToolError)?;

        parse_bool(raw)
    }
}
```

Parsing handles common yes/no variants.

---

## Strings

Strings use a freeform text elicitation tool:

```rust
impl Prompt for String {
    fn prompt() -> Option<&'static str> {
        Some("Please enter text.")
    }
}

impl Elicit for String {
    async fn elicit(client: &pmcp::Client) -> Result<Self, ElicitError> {
        let prompt = Self::prompt().unwrap();

        let raw = client
            .call_tool("elicit_text", pmcp::json!({ "prompt": prompt }))
            .await
            .map_err(ElicitError::ToolError)?;

        parse_string(raw)
    }
}
```

---

## Enums

Enums use the **Select** interaction pattern.  
The derive macro generates:

- `impl Prompt`
- `impl Select`
- `impl Elicit`

Example:

```rust
#[derive(Elicit)]
enum Mode {
    Fast,
    Safe,
}
```

Expands to:

```rust
impl Select for Mode {
    fn options() -> &'static [Self] {
        &[Mode::Fast, Mode::Safe]
    }

    fn labels() -> &'static [&'static str] {
        &["Fast", "Safe"]
    }
}
```

The elicitation engine calls the `elicit_select` MCP tool.

---

## Structs

Structs use the **Survey** interaction pattern.  
The derive macro generates:

- a `fields()` array describing each field
- a state machine that elicits each field in order
- an `impl Elicit` that drives the survey

Example:

```rust
#[derive(Elicit)]
struct Config {
    retries: i8,
    mode: Mode,
}
```

The macro generates a multi‑step elicitation flow that calls `.elicit()` on each field.

---

## Containers

### Option\<T\>

```rust
impl<T: Elicit> Elicit for Option<T> {
    async fn elicit(client: &pmcp::Client) -> Result<Self, ElicitError> {
        let yes = bool::elicit(client).await?;
        if yes { T::elicit(client).await.map(Some) } else { Ok(None) }
    }
}
```

### Vec\<T\>

```rust
impl<T: Elicit> Elicit for Vec<T> {
    async fn elicit(client: &pmcp::Client) -> Result<Self, ElicitError> {
        let mut items = Vec::new();
        loop {
            let add_more = bool::elicit(client).await?;
            if !add_more { break }
            items.push(T::elicit(client).await?);
        }
        Ok(items)
    }
}
```

---

## Summary

The elicitation crate provides complete, canonical implementations for all standard types. These implementations:

- rely on the trait architecture for metadata
- use generics and macros to eliminate duplication
- call MCP tools through pmcp
- centralize parsing and error handling
- provide sensible defaults that users can override

This foundation allows downstream developers to derive elicitation for their own types with minimal effort, while ensuring consistent, predictable behavior across the entire ecosystem.
