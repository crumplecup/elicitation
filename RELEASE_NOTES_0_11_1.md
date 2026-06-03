# Release Notes — elicitation 0.11.1

## The headline: silencing the cfg warning storm

If you've wired up a crate with `#[derive(Elicit)]` alongside Kani or Creusot, you've
probably seen the wall of `unexpected_cfg` warnings that followed. This release eliminates
them.

The root problem is structural: elicitation's derive macro and proof generator emit code
gated on `cfg(kani)`, `cfg(creusot)`, and `cfg(verus)`. When those toolchains aren't active,
Rust rightfully flags the unknown cfg keys. The naive fix — annotate every callsite — doesn't
scale across a generated codebase, and `#[allow]` attributes are a maintenance smell.

The solution required reaching into the expansion itself. The `formal_method` proc-macro
expansion now routes all cfg-bearing tokens through an isolated `allow` module, so the
warnings are suppressed structurally at the point of generation rather than scattered
callsite-by-callsite. The contracted wrapper function is emitted with a clean body, and
tracing instrumentation is correctly gated under `#[cfg(not(kani))]` so it doesn't appear
in Kani's symbolic execution model. A companion `UNEXPECTED_CFGS.md` white paper documents
the full suppression strategy for contributors.

This was the majority of the engineering effort in this cycle. The result is a clean build
with any combination of toolchains active.

---

## Cleaner builds, leaner dependencies

`elicitation` now ships with `default = []`. The upshot for new users: your build only
compiles what you actually use. Previously, a project that only needed JSON schemas would
silently drag in geo, bevy, wgpu, and two dozen other heavy crates. With granular features,
build times are proportional to what you declare.

The `full` feature is still available as an explicit opt-in, and `dev` continues to enable
everything for workspace-level development.

> **Note:** if you're upgrading from 0.11.0, add `features = ["your-feature"]` to your
> elicitation dependency for any domain types you use. The `full` feature restores prior
> behavior exactly.

As a related bonus: the workspace toolchain is now **`stable`**. It had been on
`nightly-2026-04-21` purely as a Creusot side-effect; no workspace crate actually needs
`#![feature(...)]`. `cargo creusot` manages its own nightly override internally and is
unaffected.

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
