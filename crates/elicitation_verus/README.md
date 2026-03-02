# elicitation_verus

Verus formal verification proofs for [`elicitation`] contract types.

This crate contains **proof functions** for every contract type in the
elicitation framework using [Verus] — a Rust verification tool that uses
Z3 as its SMT back-end and supports explicit `requires`/`ensures` contracts,
executable specifications (`spec fn`), and ghost variables, all written
directly in Rust syntax inside a `verus! { … }` macro block.

> **Note:** This crate is excluded from the Cargo workspace and requires a
> separate Verus toolchain installation. It produces no compiled library
> output — only verification artifacts.

## What Verus does

Verus performs *unbounded* symbolic verification: given a function annotated
with a precondition (`requires`) and a postcondition (`ensures`), it
translates the code to its own VIR (Verus Intermediate Representation),
then passes the resulting SMT obligations to Z3. Unlike bounded model
checking, this is not limited by loop depth or input size — the proof holds
for all mathematically possible inputs satisfying the precondition.

```rust
verus! {
    pub fn new(value: i32) -> (result: Result<I32Positive, ValidationError>)
        ensures
            value > 0 ==> (result matches Ok(p) && p.value == value),
            value <= 0 ==> (result matches Err(_)),
    {
        if value > 0 { Ok(I32Positive { value }) }
        else { Err(ValidationError::NotPositive) }
    }
}
```

Z3 discharges both obligations — the success branch and the failure branch
— for every possible `i32`, giving an unbounded proof of constructor
correctness.

## Proof patterns

### Requires / ensures on constructors

Every contract type has a constructor proof covering both `Ok` and `Err`
outcomes:

```rust
ensures
    value > 0 ==> (result matches Ok(p) && p.value == value),
    value <= 0 ==> (result matches Err(ValidationError::NotPositiveI32(v)) && v == value),
```

The `==>` (implies) combinator lets each branch of the match be stated as a
separate logical obligation.

### Accessor proofs

Accessor methods carry `ensures` clauses that tie their return value back to
the construction parameter:

```rust
pub fn get(&self) -> (result: i32)
    requires self.value > 0,
    ensures  result == self.value,
```

### Ghost state

Some types carry a `validated: bool` ghost field used purely for proof
bookkeeping — it tracks whether the construction contract was satisfied and
lets Verus reason about compound types without re-proving sub-contracts:

```rust
pub struct VecNonEmpty { pub validated: bool }

pub fn is_validated(&self) -> (result: bool)
    ensures result == self.validated,
{ self.validated }
```

### Compositional proofs

Tuple types compose validation flags with logical conjunction, proving that
a pair type is valid exactly when both components are valid:

```rust
pub fn is_validated(&self) -> (result: bool)
    ensures result == (self.first_validated && self.second_validated),
```

### Abstraction for external crates

Operations that depend on external crates (regex parsing, UUID construction,
IEEE 754 float semantics, Unicode character properties) are abstracted as
boolean parameters rather than called directly. We prove the *wrapper logic*
without trying to verify the external library:

```rust
// Instead of calling regex::Regex::new(pattern):
pub fn new(compiles: bool) -> (result: Result<RegexValid, ValidationError>)
    ensures
        compiles  ==> (result matches Ok(_)),
        !compiles ==> (result matches Err(_)),
```

This is the **castle on cloud** principle applied to Verus: trust the
foundations (external crates), verify the walls (our contract logic).

## Coverage

| Module | Types proved |
|---|---|
| `primitives` | `i8`–`i64`, `u8`–`u64`, `f32`, `f64`, `bool`, `char` base proofs |
| `bools` | `BoolTrue`, `BoolFalse` |
| `chars` | `CharAlphabetic`, `CharAlphanumeric`, `CharNumeric` |
| `integers` | `I8`/`I16`/`U8`/`U16` Positive, NonNegative, NonZero |
| `floats` | `F32`/`F64` Positive, NonNegative, Finite |
| `strings` | `StringNonEmpty`, `StringBounded`, `StringNonEmptyBounded` |
| `collections` | `VecNonEmpty`, `VecBounded`, `VecNonEmptyBounded`, `OptionSome`, `ResultOk` |
| `tuples` | `Tuple2`, `Tuple3`, `Tuple4` compositional validity |
| `stdlib_collections` | Generic `Option`, `Result`, 2/3/4-tuple composition |
| `durations` | `DurationPositive` |
| `networks` | `IpPrivate`, `IpPublic`, `IpV4`, `IpV6` |
| `paths` | `PathNonEmpty`, `PathAbsolute`, `PathRelative` |
| `ipaddr_bytes` | `Ipv4Bytes`, `Ipv6Bytes`, private/public classification |
| `macaddr` | `MacAddr` unicast/multicast/universal/local bit patterns |
| `socketaddr` | `SocketAddrV4Bytes`, `SocketAddrV6Bytes`, privileged port classification |
| `utf8` | `Utf8Valid`, `Utf8Bounded` length and validity |
| `pathbytes` | `PathBytes`, `PathAbsolute`, `PathRelative` (Unix) |
| `urlbytes` | `UrlBytes`, `SchemeBytes`, `AuthorityBytes` |
| `regexbytes` | `RegexBytes`, `BalancedDelimiters` |
| `uuid_bytes` | `UuidBytes`, `UuidV4`, `UuidNonNil` |
| `external_types` | `Regex`, `Url`, `Uuid`, `DateTime`, `IP`, `PathBuf`, `Duration`, `serde_json::Value` |
| `datetimes` | `DateTimeUtcAfter`, `DateTimeUtcBefore` |
| `urls` | `UrlValid`, `UrlHttps`, `UrlHttp` |
| `uuids` | `UuidV4`, `UuidNonNil` |
| `regexes` | `RegexValid`, `RegexCaseInsensitive` |
| `values` | `ValueObject`, `ValueArray`, `ValueNonNull` |

## Running the proofs

Install Verus following the [official instructions][verus-install], then
verify the crate directly:

```bash
verus crates/elicitation_verus/src/lib.rs
```

Verus writes intermediate files to a `.verus-log/` directory:
- `.vir` / `-sst.vir` — Verus Intermediate Representation
- `.air` / `-final.air` — Anvil IR passed to Z3
- `.smt2` — SMT-LIB2 queries sent to the solver
- `.smt_transcript` — Z3 interaction log

For failed proofs, `.interp` files contain counterexample models from Z3.

## Integration with the elicitation crate

`elicitation_verus` depends on the main `elicitation` crate with the
`verification` feature enabled:

```toml
[dependencies]
elicitation = { version = "0.8", features = ["verification"] }
```

The main crate defines the contract types; this crate proves their
constructors and accessors are correct. Because verification produces no
compiled output, there is zero runtime overhead — the proofs exist solely
as logical witnesses.

The crate is **excluded from the workspace** (`exclude = [...]` in the root
`Cargo.toml`) because it requires Verus's own modified Rust toolchain and
cannot be built with standard `cargo`.

## Verus vs. Kani vs. Creusot

The elicitation workspace maintains three independent formal verification
back-ends. They address different points on the verification spectrum:

| Crate | Tool | Method | Best at |
|---|---|---|---|
| `elicitation_verus` | [Verus] | Unbounded SMT (Z3) | Complex contracts, ghost variables, composition |
| `elicitation_kani` | [Kani] | Bounded model checking | Memory safety, no-panic, bit-level patterns |
| `elicitation_creusot` | [Creusot] | Deductive (Why3 + SMT) | Functional correctness, loop invariants |

Verus is the natural choice when you need to express rich *logical*
properties — implications, conjunctions over compound types, conditional
postconditions — and want a single self-contained toolchain (no Why3
ecosystem required). Kani is faster for concrete numeric/bit properties.
Creusot shines for loop-heavy algorithms.

A property proved by all three tools independently has been checked by three
completely separate verification paths, providing very high confidence.

## License

Licensed under either of [Apache License 2.0](../../LICENSE-APACHE) or
[MIT License](../../LICENSE-MIT) at your option.

[Verus]: https://verus-lang.github.io/verus/
[verus-install]: https://verus-lang.github.io/verus/guide/getting_started.html
[Kani]: https://model-checking.github.io/kani/
[Creusot]: https://github.com/creusot-rs/creusot
[`elicitation`]: https://crates.io/crates/elicitation
