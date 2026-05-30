# Kani 0.67.0 bundles rustc 1.93.0-nightly, blocking sqlx 0.9 adoption

**Repository:** model-checking/kani

## Summary

`sqlx 0.9.0` declares `rust-version = "1.94"` in its `Cargo.toml`. Kani 0.67.0 bundles `rustc 1.93.0-nightly`, so any workspace that includes `sqlx 0.9` as a dependency fails at the `cargo kani list` step with:

```
error: rustc 1.93.0-nightly is not supported by the following packages:
  sqlx@0.9.0 requires rustc 1.94.0
  sqlx-core@0.9.0 requires rustc 1.94.0
  ...
Either upgrade rustc or select compatible dependency versions
```

## Steps to reproduce

1. Create a workspace with `sqlx = "0.9"` as a dependency.
2. Run `cargo kani list` (or `cargo kani`).
3. Observe the MSRV error above.

## Expected behavior

Kani's bundled toolchain should be recent enough to compile crates that declare `rust-version = "1.94"`. sqlx 0.9 was released in early 2026 and is the current stable release.

## Current behavior

Kani 0.67.0 ships with `rustc 1.93.0-nightly` (toolchain `nightly-2025-11-21`), which is one minor version behind sqlx 0.9's MSRV.

## Workaround

Downgrade to `sqlx 0.8` until a new Kani release ships with a nightly ≥ 1.94.

## Environment

- `cargo-kani 0.67.0`
- Kani-bundled rustc: `1.93.0-nightly`
- `sqlx 0.9.0` MSRV: `1.94`
- Host toolchain: `nightly-2026-04-21` (rustc 1.97.0-nightly) — unaffected
