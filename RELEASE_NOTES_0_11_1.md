# Release Notes — elicitation 0.11.1

## The headline: leaner by default

The most consequential change in this release is one that users will feel immediately at
build time: **`elicitation` now ships with `default = []`**. Previously the crate defaulted
to `full`, silently pulling in chrono, geo, polars-types, bevy-types, egui, wgpu, and two
dozen other heavy dependencies whether you asked for them or not. That era is over.

Each consumer crate now declares exactly the features it needs. The result is faster
incremental builds, smaller dependency trees, and no more surprises when `cargo build`
fetches half of crates.io for a project that only needs JSON schemas. The `full` feature is
still available as an explicit opt-in, and `dev` continues to enable everything for
workspace-level development.

> **Breaking change:** any downstream crate relying on implicit transitive features must now
> declare them explicitly. The fix is always a one-liner — add `features = ["your-feature"]`
> to your elicitation dependency.

The same experiment that landed `default = []` also brought **`elicit_polars` into the
workspace** as a full member. The crate had previously been excluded due to a version conflict
between `surrealdb-types`' geo 0.32 dependency and the workspace's geo 0.33. With elicitation
no longer dragging geo into the polars feature tree, the conflict evaporates. `elicit_polars`
now participates in CI alongside its 35 siblings.

As a bonus discovery: the workspace had been on `nightly-2026-04-21` purely as a side effect
of Creusot development. No workspace crate actually requires `#![feature(...)]`. The
**toolchain is now `stable`** — `cargo creusot` manages its own nightly override internally
and is unaffected. Everyday builds are now on stable 1.96.

---

## Text IR and rich rendering pipeline

A new intermediate representation for rich text landed across three crates this cycle.
`elicit_ui` gains `RenderContext` and `RenderVerifiable` traits backed by `peniko` and
`parley`, establishing a verifiable rendering contract at the GUI layer. `elicit_accesskit`
picks up a `rich_text` sidecar on `NodeJson`, threading structured text into the
accessibility tree. `elicit_ratatui` completes the pipeline: `RenderContext` is now
implemented for `ratatui::Buffer`, and `bridge_paragraph` renders per-span styled text,
preserving leading whitespace correctly via `Wrap { trim: false }`.

---

## Middleware and observability

A new middleware module in `elicitation` provides context propagation and observability hooks
for tool invocations. `ObservableCommunicator` is now generic over message type, removing the
previous monomorphization constraint. `elicit_ratatui` gains `TuiCommunicator`, a
terminal-flavored communicator for interactive elicitation sessions.

---

## Proof infrastructure

The proof-crate generator received several rounds of hardening. Multi-source `--crate-path`
support landed, allowing a single generator invocation to cover multi-crate workspaces.
`why3find.json` generation, dependency scanning, and import priority resolution are all
handled automatically. A `kani_skip_features` option was added for crates whose feature flags
are incompatible with the Kani toolchain.

The Kani reexport pipeline was tightened: `_kani_contracted` functions now carry doc
comments, invariant fn re-exports are correctly gated under `#[cfg(kani)]`, and the
`formal_method` expansion routes cfg-bearing tokens through an isolated `allow` module to
suppress spurious `unexpected_cfgs` warnings. An `UNEXPECTED_CFGS.md` white paper documents
the full suppression strategy for maintainers and contributors.

`creusot-std` upgraded to 0.11.0 and the vendored copies were removed entirely. The shadow
workspace sanitizer in the Creusot CLI was fixed, and `--no-check-version` is now passed to
`cargo creusot prove` for smoother CI runs.

---

## Dependency updates

| Crate | From | To | Notes |
|---|---|---|---|
| `surrealdb` / `surrealdb-types` | 3.0.5 | 3.1.2 | `elicit_surrealdb` remains workspace-excluded pending upstream geo 0.33 adoption |
| `rstar` | 0.12 | 0.13 | |
| `anodized` | 0.4 | 0.5 | |
| `creusot-std` | 0.10.0 | 0.11.0 | vendored copies removed |
| `sqlx` | 0.9 attempted | 0.8 (reverted) | holding pending upstream stabilization |
