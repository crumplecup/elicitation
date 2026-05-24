# elicit_proofs

Formal verification proof harnesses for the [elicitation] MCP server ecosystem.
Contains generated and manual proof harnesses for [Kani], [Verus], and [Creusot],
verifying the state-machine transitions in [`elicit_server`].

## What's in this crate

| Module | Contents |
|---|---|
| `kani/generated/` | Auto-generated `proof_for_contract` harnesses — one per VSM transition |
| `kani/composition.rs` | Manual two-step composition proofs (leaf × stub) |
| `kani/diag.rs` | Diagnostic harnesses for troubleshooting individual transitions |
| `kani/gallery/` | Numbered gallery of verification patterns and known results |
| `kani/suite.rs` | Framework correctness tests for `KaniCompose` |
| `creusot/generated/` | Auto-generated Creusot companion proofs |
| `verus/generated/` | Auto-generated Verus companion proofs |

## Generating proofs

After changing VSM source files in `elicit_server`, regenerate all three backends at once:

```sh
elicitation generate all \
    --crate-path crates/elicit_server/src/archive/vsm \
    --out crates/elicit_proofs/src
```

Or individually:

```sh
elicitation generate kani    --crate-path crates/elicit_server/src/archive/vsm \
                              --out crates/elicit_proofs/src/kani/generated
elicitation generate creusot --crate-path crates/elicit_server/src/archive/vsm \
                              --out crates/elicit_proofs/src/creusot/generated
elicitation generate verus   --crate-path crates/elicit_server/src/archive/vsm \
                              --out crates/elicit_proofs/src/verus/generated
```

## Running proofs

Configure backends in `.env` at the workspace root (see `.env` for all options),
then run with `elicitation prove`:

```sh
# All backends
elicitation prove --kani --verus --creusot

# Single backend
elicitation prove --kani
elicitation prove --verus
elicitation prove --creusot

# Target a specific Kani harness
elicitation prove --kani --kani-harness my_harness

# Record results to CSV and resume interrupted runs
elicitation prove --kani --csv --resume
```

Key `.env` settings:

```sh
KANI_PACKAGE=elicit_proofs
KANI_FLAGS="--lib --features kani -Z function-contracts -Z stubbing"
CREUSOT_PACKAGE=elicit_proofs
VERUS_PATH=~/repos/verus/...      # Path to Verus binary
VERUS_FILE=crates/elicitation_verus/src/lib.rs
```

`elicit_proofs` is the target for the generated Kani and Creusot proof code.
Verus does not run cleanly from the combined proofs crate, so `elicitation prove
--verus` should point at the dedicated `elicitation_verus` crate instead.

## Induction strategy for String

Proofs involving `String` fields use depth-based induction via [`KaniCompose`]:
`kani_depth1()` returns a one-character symbolic string (`kani::any::<char>().to_string()`),
ensuring non-emptiness invariants are genuinely checked rather than vacuously skipped.

## Dependencies

```toml
[dependencies]
elicit_proofs = "0.11"
```

[elicitation]: https://crates.io/crates/elicitation
[`elicit_server`]: https://crates.io/crates/elicit_server
[`KaniCompose`]: https://docs.rs/elicitation/latest/elicitation/trait.KaniCompose.html
[Kani]: https://model-checking.github.io/kani/
[Verus]: https://verus-lang.github.io/verus/
[Creusot]: https://github.com/creusot-rs/creusot
