# elicit_proofs

> **Auto-generated** — do not edit by hand.
> Regenerate with `elicitation generate proof-crate`.

Kani, Creusot, and Verus proof harnesses for [`elicit_server`],
produced by [elicitation](https://crates.io/crates/elicitation).

## Verifiers

| Backend  | Feature flag | Run command                       |
|----------|-------------|-----------------------------------|
| Kani     | `kani`      | `cargo kani`                      |
| Creusot  | `creusot`   | `cargo creusot prove`             |
| Verus    | `verus`     | `verus src/lib.rs`                |

## Regenerating

```sh
elicitation generate proof-crate \
    --crate-path <path-to-elicit_server> \
    --crate-name elicit_proofs \
    --out <path-to-this-crate>
```
