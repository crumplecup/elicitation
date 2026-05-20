# Elicitation 0.11.0 Release Notes

---

## Overview

The headline story is the maturation of the **Verified State Machine (VSM) toolchain** into a complete, end-to-end developer workflow — from annotating your Rust code to running proofs across three backends with a single command. Alongside that, this release delivers a wave of new shadow crates, a production-quality database explorer application, and a complete formal GIS standards library.

---

## 🛠️ The `elicitation generate` + `elicitation prove` Workflow

The most impactful change for users doing formal verification. You can now go from annotated source to verified proofs without writing any boilerplate:

```sh
# Scan your VSMs and emit Kani/Verus/Creusot proof harnesses
elicitation generate kani --crate-path crates/my_crate
elicitation generate verus --crate-path crates/my_crate
elicitation generate creusot --crate-path crates/my_crate

# Run all backends with per-harness progress + CSV tracking
elicitation prove --kani --csv
elicitation prove --verus --csv
elicitation prove --creusot --csv
```

Key capabilities:
- **syn-based VSM scanner** — reads `#[derive(VerifiedStateMachine)]` and `#[formal_method]` annotations directly from source; no separate manifest required.
- **Precise import resolver** — scans parent modules to emit correct `use` paths in generated files; generated code is immediately `rustfmt`-formatted and warning-free.
- **Per-harness CSV tracking** with configurable `KANI_TIMEOUT` (default 300s) and `--resume` support so interrupted runs pick up where they left off.
- **indicatif progress bar** with per-harness status, elapsed time, and live spinner.
- **Reliable process cleanup** — on timeout, `libc::killpg` + recursive `/proc` tree walk ensures cbmc and all its descendants are fully reaped. No more orphaned processes consuming RAM after a timeout.
- **Verus V13 `assume_specification` pattern** — generated Verus companions use the most powerful and composable spec-attachment mechanism available.
- **Creusot `extern_spec!` architecture** — generated companions use trusted premises rather than `#[trusted]` conclusions, producing real verification conditions.

---

## ✅ Proof Gallery Completions

All three backend galleries are now fully proven:

| Backend | Gallery levels | Harnesses |
|---------|---------------|-----------|
| **Kani** | Gallery L1–L15 + all VSM harnesses | 221/221 passing |
| **Creusot** | C1–C24 | All Why3 goals closed |
| **Verus** | V1–V13 | All verified |

Notable gallery additions:
- **Kani L15** — DFCC + bounded Vec blowup diagnosis; documents the exact CBMC failure modes so you know what patterns to avoid.
- **Creusot C21–C24** — two-step composition proofs for full panel machine lifecycles.
- **Verus V11–V13** — leaf-lemma + composition patterns for multi-step VSM transitions.

---

## 🔒 Verified State Machines

The `#[derive(VerifiedStateMachine)]` and `#[formal_method]` derive macros are now stable:

- `#[formal_method]` gates `#[instrument]` behind `cfg_attr(not(kani), ...)` automatically, preventing tracing prologues from appearing in DFCC verification conditions.
- `KaniCompose` derive macro generates depth-aware compositional harnesses (`d0`/`d1`/`d2`) — leaf proofs compose via `stub_verified` in ~4s vs ~33s for full leaf proofs.
- `Established<P>: KaniCompose` — the `Established` wrapper participates in compositional proofs out of the box.
- The **archive module** is fully covered by four VSMs (`ArchiveNav`, `ArchivePanel`, `ArchiveConnection`, `ArchiveOverlay`) with generated harnesses for Kani, Verus, and Creusot.

---

## 🗄️ `elicit_redb` — Embedded Database Backend

A complete rewrite of `elicit_redb` as a proper shadow crate:

- **`RedbBackend`** implements the full `elicit_db` trait suite — typed key-value operations, table management, and transactional semantics.
- **UUID-handle pattern** — `RedbHandle` wraps a `Uuid` rather than a raw `Arc<Database>`, making handles cheap to clone and `Send + Sync` trivially.
- **`ArchiveKvPlugin`** — 10 MCP tools exposing redb KV operations to AI agents and MCP clients directly from the archive module.

---

## 📊 Archive Application

The archive database explorer is now a full-featured application with three parallel frontends — **egui/wgpu** (native desktop), **Leptos** (browser), and **ratatui** (terminal):

**Phase highlights shipped in this release:**
- Interactive data grid with inline row edit/insert/delete
- SQL editor with syntax highlighting, query history, and saved queries
- Column statistics, EXPLAIN viewer, and query plan comparison
- ERD diagram view with FK enrichment and visual grid layout
- Monitor panel with SSE live polling, WAL stats, role management, and backup controls
- CSV/JSON/TSV/NDJSON export
- SSH tunnel + SSL connection profiles
- Multi-connection management

---

## 💰 GAAP / Financial Domain

`elicit_server` gains a first-class GAAP module with full ASC concordance:

- Four-phase ledger foundation: account types and chart of accounts, journal entries with typestate VSM, ledger storage with balance queries, financial statements with period analysis and ratios.
- `GaapBackend` trait + descriptor types + three-role trait interface.
- Journal→GAAP bridge with proof composition impls for audit-traceable operations.
- PostgreSQL persistence layer.

---

## 🗺️ GIS Formal Standards Library

A comprehensive suite of formal contracts and trait interfaces for GIS standards:

| Standard | Coverage |
|----------|----------|
| ISO 19111 | Full CRS interface + 55 gap props (§21–29) + FV precondition props |
| OGC SFS | Contracts + three-role traits + finiteness guards + aggregate validity |
| ISO 19115 | Contracts + three-role traits + 21 FV gap props |
| RFC 7946 GeoJSON | 210 contracts + three-role trait interface |
| FGDC CSDGM | 160 props (§0–§10) + descriptor types |

All five standards integrate with the `ProvableFrom` credentialed proof-minting system.

---

## 📦 New Shadow Crates

0.11.0 adds MCP shadow crate coverage for:

| Crate | Tools |
|-------|-------|
| `elicit_axum` | 4 plugins, 22 tools — FromRequest, IntoResponse, Router, Handler |
| `elicit_tower` | limit, retry, http, util, builder, balance, steer plugins |
| `elicit_leptos` | 84 MCP tools; full Leptos 0.8 coverage |
| `elicit_wgpu` | Phase 2 primitives + 5 plugins, 17 tools |
| `elicit_winit` | 11 types + 3 plugins; winit 0.30 |
| `elicit_bevy` | 18+ Bevy 0.18 shadow types across render, input, scene, camera domains |
| `elicit_georaster` | Raster support + proof coverage |
| `elicit_rstar` | Core wrappers and tree factories + proof coverage |
| `elicit_proj` | Proj-types Phase 2 support |
| `elicit_wkt` | WKT read/write |
| `elicit_wkb` | WKB read/write |
| `elicit_geojson` | Spatial and GeoJSON transport helpers |
| SurrealDB | 62 tools across 7 plugins; SurrealDB 3.x complete |
| `elicit_csv` | 7 primitive types, full MCP coverage |
| `elicit_toml` | Phase 1 + Phase 2 TOML ecosystem support |
| polars | 4 plugins, 72 tools |
| `elicit_uom` | Multi-param factory pattern for units-of-measure |

---

## 🔒 WCAG 2.2 Proof-Carrying Palette

- Full WCAG 2.2 concordance rewrite — all success criteria modelled as contracts.
- `ProvableFrom` credential system — proof dependencies are tracked as ZST credential types at compile time.
- Chain-of-custody proof tokens flow through all three rendering backends (egui, Leptos, ratatui).
- `proof_credential!` macro for zero-cost ZST credential + `ProvableFrom` binding.

---

## 🔧 Infrastructure

- **CI overhaul** — feature-group matrix jobs, per-platform disk space management, split lint/test workflows, `ci` feature flag for smoke tests.
- **`cargo-semver-checks`** wired into `just pre-release` — API compatibility is checked against crates.io baseline before every release.
- **`elicitation_macros` retired** — helpers moved to `elicitation_kani`; `ELICITATION_MACROS.md` serves as the macro discovery document.
- **`just check-features`** powerset check fixed — no more false warnings from vendored `surrealdb-types`.

---

## Upgrade Notes

- Workspace version bumped `0.10.0 → 0.11.0`. All crates share this version.
- The legacy `elicit_proofs` runner binary has been removed. Use `elicitation prove --kani` / `--verus` / `--creusot` instead.
- `elicitation_macros` is no longer a workspace member. Remove it from any direct dependencies.
- `.env` is required for `elicitation prove` to locate proof packages. Copy `.env.example` to `.env` and set `KANI_PACKAGE`, `CREUSOT_PACKAGE`, and `VERUS_PATH`.
