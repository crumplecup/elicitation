# elicitation_creusot

Creusot formal verification proofs for [`elicitation`] contract types.

This crate contains **100+ proof functions** that formally verify the
correctness of every contract type in the elicitation framework using
[Creusot] — a deductive verifier for Rust that translates code to WhyML
and dispatches proofs to automated theorem provers (Z3, CVC4, Alt-Ergo).

## What Creusot does

Creusot performs *deductive program verification*: given a function annotated
with a precondition (`#[requires]`) and a postcondition (`#[ensures]`), it
symbolically executes the function and asks a theorem prover to confirm that,
*for all possible inputs satisfying the precondition*, the output satisfies the
postcondition. This is stronger than testing — it holds for every value, not
just the ones you thought to test.

```rust
/// Prove I32Positive construction succeeds for positive values.
#[requires(value@ > 0)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_i32_positive_valid(value: i32) -> Result<I32Positive, ValidationError> {
    I32Positive::new(value)
}
```

When Creusot processes this function it discharges the obligation:
> *"For all `i32` values greater than zero, `I32Positive::new` returns `Ok`."*

## What this crate verifies

Every contract type in `elicitation` has a corresponding proof module here.
Each module covers:

| Proof kind | What it establishes |
|---|---|
| `verify_*_valid` | Constructor succeeds when the precondition holds |
| `verify_*_invalid` | Constructor fails (returns `Err`) when the precondition is violated |
| `verify_*_boundary` | Correct behaviour at range boundaries |
| `verify_*_accessor` | Accessor methods return the value passed at construction |
| `verify_*_composition` | Compound types are correct when their components are correct |

### Coverage

| Module | Types proved |
|---|---|
| `bools` | `BoolTrue`, `BoolFalse` |
| `chars` | `CharAlphabetic`, `CharAlphanumeric`, `CharNumeric` |
| `integers` | `I8`–`I128` / `U8`–`U128` Positive, NonNegative, Range variants |
| `floats` | `F32`/`F64` Positive, NonNegative, Finite |
| `strings` | `StringNonEmpty<MAX_LEN>` |
| `collections` | NonEmpty `Vec`, `HashMap`, `BTreeMap`, `HashSet`, `BTreeSet`, `VecDeque`, `LinkedList`; `Option`/`Result` wrappers |
| `tuples` | `Tuple2`, `Tuple3`, `Tuple4` compositional wrappers |
| `durations` | `DurationPositive` |
| `networks` | `IpPrivate`, `IpPublic`, `IpV4`, `IpV6`, `Ipv4Loopback`, `Ipv6Loopback` |
| `paths` | `PathBufExists`, `PathBufIsFile`, `PathBufIsDir`, `PathBufReadable` |
| `mechanisms` | `Affirm`, `Survey`, `Select` — and the trenchcoat pattern itself |
| `utf8` | `Utf8Bytes` length and emptiness predicates |
| `ipaddr_bytes` | `Ipv4Bytes`, `Ipv6Bytes`, private/public classification |
| `socketaddr` | `SocketAddrV4Bytes`, `SocketAddrV6Bytes`, port classification |
| `macaddr` | `MacAddr` unicast/multicast/universal/local classification |
| `pathbytes` | (Unix) `PathBytes`, `PathAbsolute`, `PathRelative`, `PathNonEmpty` |
| `uuids` *(feature)* | `UuidNonNil`, `UuidV4`, `UuidBytes`, `UuidV4Bytes`, `UuidV7Bytes` |
| `urls` *(feature)* | `UrlValid`, `UrlHttp`, `UrlHttps`, `UrlWithHost`, RFC 3986 byte-level types |
| `regexes` *(feature)* | `RegexValid`, case-insensitive/multiline variants, layered byte-level validation |
| `values` *(feature)* | `ValueArray`, `ValueObject`, `ValueNonNull` |
| `datetimes_chrono` *(feature)* | `DateTimeUtcAfter`, `DateTimeUtcBefore`, `NaiveDateTimeAfter` |
| `datetimes_time` *(feature)* | `OffsetDateTimeAfter`, `OffsetDateTimeBefore` |
| `datetimes_jiff` *(feature)* | `TimestampAfter`, `TimestampBefore` |
| `http` *(feature)* | `StatusCodeValid` |

## The "cloud of assumptions" pattern

All proofs use `#[trusted]` — the verification obligation is *assumed* rather
than checked by Creusot itself. The point is not to have Creusot verify the
implementation of `I32Positive::new`; that code is simple Rust. The point is
to **document the contract precisely** so that:

1. Downstream user code can be verified *against* those contracts.
2. Any future change to the contract type will break the proofs, making the
   breakage visible before it reaches production.
3. The proof corpus serves as machine-checkable specification documentation.

The "cloud" is the set of trusted axioms. We trust the Rust stdlib, `serde`,
and our own validation logic. Everything above that cloud is verified.

## How users benefit

### You use `elicitation` contract types — no change needed

If you use `I32Positive`, `StringNonEmpty`, or any other contract type from
`elicitation`, you already benefit. The proofs in this crate are part of the
verification baseline your own Creusot verification is built on.

### You write Creusot proofs for your own code

Add this crate as a dev-dependency so Creusot can see the contracts:

```toml
[dev-dependencies]
elicitation_creusot = { version = "0.8", features = ["uuid", "url"] }
```

Then annotate your own functions and run Creusot:

```rust
use elicitation::verification::types::I32Positive;

/// Returns the sum of two positive integers.
#[requires(a@ > 0 && b@ > 0)]
#[ensures(result@ == a@ + b@)]
pub fn add_positive(a: I32Positive, b: I32Positive) -> i32 {
    a.get() + b.get()
}
```

```bash
cargo creusot -- --features dev
```

### You verify composition of elicitation types

Because every field type has a proved contract, Creusot can verify properties
of structs that compose them:

```rust
#[requires(start@ < end@)]
#[ensures(result.start@ < result.end@)]
pub fn make_range(start: I32Positive, end: I32Positive) -> DateRange {
    DateRange { start, end }
}
```

Creusot will use the `I32Positive` postconditions from this crate as lemmas
when discharging the obligation above.

## Running the proofs yourself

Install Creusot following the [official instructions][creusot-install], then:

```bash
# Verify core proofs
cargo creusot

# Verify with all optional features
cargo creusot -- --features all
```

Individual modules can be targeted by path if the full proof run is slow.

## Relationship to other verification crates

The elicitation workspace includes three formal verification back-ends. They
are complementary, not competing:

| Crate | Tool | Method | Strength |
|---|---|---|---|
| `elicitation_creusot` | [Creusot] | Deductive (SMT) | Full functional correctness, loop invariants |
| `elicitation_kani` | [Kani] | Bounded model checking | Memory safety, absence of panics, bounded loops |
| `elicitation_verus` | [Verus] | SMT + linear types | Ownership-aware specs, ghost code |

Each crate proves the same contract types using a different tool, giving
independent confidence in correctness. A property that holds in all three
back-ends has been checked by three completely independent verification paths.

## Feature flags

```toml
[features]
default = []
all     = ["uuid", "serde_json", "url", "regex", "chrono", "time", "jiff"]

uuid       # UuidNonNil, UuidV4, byte-level UUID types
serde_json # ValueArray, ValueObject, ValueNonNull
url        # UrlValid, UrlHttp/Https, RFC 3986 byte proofs
regex      # RegexValid, layered byte-level regex validation
chrono     # DateTimeUtc, NaiveDateTime proofs
time       # OffsetDateTime proofs
jiff       # Timestamp proofs
reqwest    # StatusCodeValid proofs
```

## License

Licensed under either of [Apache License 2.0](../../LICENSE-APACHE) or
[MIT License](../../LICENSE-MIT) at your option.

[Creusot]: https://github.com/creusot-rs/creusot
[creusot-install]: https://github.com/creusot-rs/creusot#installation
[Kani]: https://model-checking.github.io/kani/
[Verus]: https://verus-lang.github.io/verus/
[`elicitation`]: https://crates.io/crates/elicitation
