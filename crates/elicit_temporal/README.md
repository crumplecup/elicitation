# elicit_temporal

Standards-anchored temporal contract interface crate.

## Purpose

`elicit_temporal` is the branch-layer temporal interface for the elicitation
workspace. It defines the shared proposition vocabulary that leaf crates such as
`elicit_time`, `elicit_chrono`, and `elicit_jiff` can establish, and that
consumer layers such as `elicit_db`, `elicit_sqlx`, `elicit_redb`,
`elicit_surrealdb`, and `elicit_polars` can require.

This crate is intentionally contract-first. Its current slice focuses on:

- standards source tracking
- proposition naming
- proof-composition bundles
- neutral temporal descriptors
- object-safe parser, formatter, zone, conversion, interval, and reporter traits
- composite IXDTF timestamp descriptors for suffix-aware interchange
- `ProvableFrom` exchange seams for proof sidecars

## Standards

Current seed standards:

- ISO 8601-1:2019, *Date and time - Representations for information interchange - Part 1: Basic rules*
- ISO 8601-2:2019, *Date and time - Representations for information interchange - Part 2: Extensions*
- RFC 3339, *Date and Time on the Internet: Timestamps*
- RFC 9557, *Date and Time on the Internet: Timestamps with additional information*

See [STANDARDS_LEDGER.md](STANDARDS_LEDGER.md) for the extraction ledger and
coverage plan, and [standards/README.md](standards/README.md) for the local
standards corpus used to tighten exact citations.

## Design

The central pattern is the proof sidecar exchange:

1. Producers establish fine-grained temporal propositions.
2. Evidence bundles compose those propositions into higher-order guarantees.
3. Consumers accept `Established<P>` tokens instead of re-validating syntax or
   semantics ad hoc.

This keeps temporal correctness cumulative across the pipeline and aligns with
the workspace-wide `ProvableFrom` pattern.
