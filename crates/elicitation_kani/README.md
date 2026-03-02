# elicitation_kani

Kani model-checking proofs for [`elicitation`] contract types.

This crate contains **291+ proof harnesses** that formally verify the
correctness of every contract type in the elicitation framework using
[Kani] — a Rust model checker that uses symbolic execution to confirm
properties hold for *all possible inputs*, not just the ones you thought
to test.

## What Kani does

Kani performs *bounded model checking*: it symbolically executes your
Rust code, treating inputs as unknown values that range over every possible
bit pattern. When it encounters an `assert!`, it asks an SMT solver whether
a counterexample exists. If none is found within the unwind bound, the
property is proved for all inputs.

```rust
#[kani::proof]
fn verify_i32_positive_construction() {
    let value: i32 = kani::any(); // symbolic — represents every i32
    match I32Positive::new(value) {
        Ok(wrapped) => assert!(wrapped.get() > 0),  // invariant holds
        Err(_)      => assert!(value <= 0),          // rejection justified
    }
}
```

Kani symbolically explores every branch for every possible `i32`, then
confirms the invariant without a single test value being chosen by hand.

## The "castle on cloud" pattern

The guiding philosophy is: **trust the foundations, verify the walls**.

We trust the Rust stdlib, `chrono`, `time`, `jiff`, `url`, `uuid`, `regex`
and other battle-tested crates for their own correctness. We verify that
*our wrapper logic* — constructors, accessors, generators — uses those
foundations correctly.

```
  ┌─────────────────────────────────────┐
  │  Our wrappers   ← verified by Kani  │
  ├─────────────────────────────────────┤
  │  stdlib / third-party crates        │  ← trusted
  └─────────────────────────────────────┘
```

This keeps harnesses fast and focused. We do not re-prove that `i32` addition
is correct; we prove that `I32Positive::new` stores the value faithfully and
that `get()` returns it.

## What is verified

Every contract type has at least two harnesses:

| Harness kind | Property established |
|---|---|
| `verify_*_valid` | Construction succeeds → invariant holds on the result |
| `verify_*_invalid` | Construction fails → precondition was violated |
| `verify_*_accessor` | `get()` / `into_inner()` returns the construction value |
| `verify_*_generator_*` | Generator produces values that satisfy the invariant |

### Coverage by module

| Module | Types proved |
|---|---|
| `bools` | `BoolTrue`, `BoolFalse` |
| `chars` | `CharAlphabetic`, `CharAlphanumeric`, `CharNumeric` |
| `integers` | `I8`–`I128` / `U8`–`U128` Positive, NonNegative, NonZero, Range |
| `floats` | `F32`/`F64` Positive, NonNegative, Finite |
| `strings` | `StringNonEmpty<MAX_LEN>` |
| `collections` | NonEmpty `Vec`, `HashMap`, `BTreeMap`, `HashSet`; `Box`/`Arc`/`Rc`; `Option`/`Result`; `Tuple2`–`Tuple4` |
| `networks` | `IpPrivate`, `IpPublic`, `IpV4`/`IpV6`, `Ipv4Loopback`, `Ipv6Loopback` |
| `ipaddr_bytes` | `Ipv4Bytes`, `Ipv6Bytes`, private/public classification (RFC 1918, RFC 4193) |
| `macaddr` | `MacAddr` unicast/multicast/universal/local bit patterns |
| `socketaddr` | `SocketAddrV4Bytes`, `SocketAddrV6Bytes`, port range classification |
| `utf8` | `Utf8Bytes` length, emptiness, valid UTF-8 composition |
| `pathbytes` *(Unix)* | `PathBytes`, `PathAbsolute`, `PathRelative`, `PathNonEmpty` |
| `durations` | `DurationPositive` |
| `systemtime` | `SystemTimeGenerator` with offset and reference modes |
| `mechanisms` | `Affirm`, `Survey`, `Select` — mechanism composition proofs |
| `unit` | Unit type validity |
| `errors` | `IoErrorGenerator` harnesses |
| `uuid_bytes` *(feature)* | `UuidBytes`, `UuidV4Bytes`, `UuidV7Bytes`, nil/v4/v7 generators |
| `urlbytes` *(feature)* | `SchemeBytes`, `AuthorityBytes`, `UrlBytes`, RFC 3986 constraints |
| `regexbytes` *(feature)* | Layered: balanced delimiters, valid escapes, quantifiers, char classes |
| `datetimes_chrono` *(feature)* | `DateTimeUtcGenerator`, `NaiveDateTimeGenerator` offset/reference logic |
| `datetimes_time` *(feature)* | `OffsetDateTimeGenerator` offset/reference logic |
| `datetimes_jiff` *(feature)* | `TimestampGenerator` symbolic gate proofs |

## Running the proofs

Install Kani following the [official instructions][kani-install], then:

```bash
# All 291 harnesses
cargo kani -p elicitation_kani --all-features -- --default-unwind 20

# Feature-specific subset
cargo kani -p elicitation_kani --features uuid,chrono -- --default-unwind 20

# Single harness
cargo kani -p elicitation_kani --harness verify_i32_positive_construction
```

The `--default-unwind 20` bound handles loop-containing types (URL parsing,
UTF-8 scanning). Simpler harnesses complete in seconds; the full suite runs
in roughly five to ten minutes.

## Using this crate for your own types

Add it as a dev-dependency. The harnesses here also serve as templates — copy
the pattern for your own contract types:

```toml
[dev-dependencies]
elicitation_kani = { version = "0.8" }
```

```rust
use elicitation::verification::types::I32Positive;

#[kani::proof]
fn verify_my_wrapper_preserves_positivity() {
    let raw: i32 = kani::any();
    kani::assume(raw > 0);

    let pos = I32Positive::new(raw).unwrap();
    let doubled = pos.get() * 2;

    // Verify: doubling a positive i32 within safe range stays positive
    kani::assume(raw < i32::MAX / 2);
    assert!(doubled > 0);
}
```

Kani will symbolically verify this for every `i32` in `(0, MAX/2)`.

## Kani vs. Creusot vs. Verus

The elicitation workspace maintains three independent formal verification
back-ends. They are complementary:

| Crate | Tool | Method | Best at |
|---|---|---|---|
| `elicitation_kani` | [Kani] | Bounded model checking | Memory safety, no-panic, bit-level patterns |
| `elicitation_creusot` | [Creusot] | Deductive (SMT) | Full functional correctness, loop invariants |
| `elicitation_verus` | [Verus] | SMT + linear types | Ownership-aware specs, ghost variables |

Kani is fastest to get started with and excels at concrete bit-level
properties (IP range membership, MAC address bit flags, integer bounds).
When the same property is proved by all three tools independently, confidence
in correctness is very high.

## Feature flags

```toml
[features]
default = []
kani       # Enable kani-verifier dependency
chrono     # DateTimeUtc, NaiveDateTime generator proofs
time       # OffsetDateTime generator proofs
jiff       # Timestamp symbolic gate proofs
uuid       # UUID generator and byte-level proofs
url        # URL scheme/authority/byte-level proofs
regex      # Layered regex validation proofs
```

## License

Licensed under either of [Apache License 2.0](../../LICENSE-APACHE) or
[MIT License](../../LICENSE-MIT) at your option.

[Kani]: https://model-checking.github.io/kani/
[kani-install]: https://model-checking.github.io/kani/install-guide.html
[Creusot]: https://github.com/creusot-rs/creusot
[Verus]: https://verus-lang.github.io/verus/
[`elicitation`]: https://crates.io/crates/elicitation
