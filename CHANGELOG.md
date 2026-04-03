# Changelog

All notable changes to the `elicitation` project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Gate proof emission methods behind opt-in `proofs` feature
- Phase 1 ToolDescriptor + DescriptorPlugin with SecureFetchPlugin canary
- Phase 2 #[elicit_tool] attribute macro
- Phase 3 #[derive(ElicitPlugin)] + inventory registration
- Phase 4 PluginContext injection + shared reqwest::Client
- Phase 6 global emit registry via inventory
- Serde bridge for constrained types + UrlHttps canary
- Propagate UrlValid + F64Positive to all HTTP params
- Add serde boundary consistency proofs (Phase E)
- Serde boundary coverage for Creusot and Verus
- Phase 1 — ToCodeLiteral trait + impls for primitives and constrained types
- Phase 2 — extend #[elicit_tool] to parse emit / emit_ctx attrs
- Phase 3 — emit_rewriter token rewriter
- Phase 5 — canary migration of secure_fetch to #[elicit_tool] emit
- Phase 6 — all 40 smoke tests passing
- Replace emit=false with CustomEmit<P> trait escape hatch
- Replace heuristic dep inference with toml-based cargo metadata
- Add elicit_newtype_traits! for explicit trait forwarding
- Add structural type graph visualization system
- Add graph subcommand for type visualization
- Add TypeGraphPlugin MCP tool (Phase E)
- Add Debug and Elicitation impls for Established<P>
- 51 real SMT proofs via Alt-Ergo — bools and integers de-trusted
- De-trust chars, durations, mechanisms, utf8 — 67 SMT goals proved
- De-trust socketaddr, pathbytes, macaddr — 127 SMT goals proved
- De-trust collections batch 4 — 138 SMT goals proved
- De-trust ipaddr_bytes batch 5 — 181 SMT goals proved
- De-trust networks/http/utf8 batch 6 — 195 SMT goals proved
- De-trust collections/strings batch 7 — 199 SMT goals proved
- De-trust socketaddr accessors batch 8 — 203 SMT goals proved
- De-trust urlbytes/pathbytes/regexbytes batch 9 — 210 SMT goals proved
- De-trust byte literal witnesses batch 10 — 233 SMT goals proved
- Phase A — TypeGraphKey registry + derive emission
- De-trust tuples/strings batch 11 — 240 SMT goals proved
- Track per-goal proof results with timestamps
- Add uuid_bytes extern_specs — 33 new SMT goals proved
- Add Elicitation impls for clap 4 types
- Add newtype wrappers for clap types with MCP reflect methods
- Add clap-types proof harnesses
- Add trusted third-party assumptions for clap builder types
- Add clap-types proof functions
- Add clap-types proof functions
- De-trust clap label_count proofs via extern_spec axioms
- Add ElicitSpec impls for all 11 clap types
- Milestone 1 — DynamicToolRegistry
- Implement #[reflect_trait] proc macro (Milestone 2)
- Add type_map + ElicitProxy for clap trait factories
- Complete clap trait factory coverage
- Add register_convert<T, U> for serde-mediated type conversion
- Emit-only macro tools for std macros
- Fragment tool model with RawFragment + assemble
- Implement shadow crate for sqlx with full MCP tool coverage
- Restore AnyColumn wrapper and direct AnyValueKind decode
- Add sqlx_types Verus proof module
- Add sqlx-types harnesses to Kani registry and fix compile errors
- Register sqlx_types + clap_types in runner; de-trust label_count proofs
- Register clap_types (26) and sqlx_types (32) in VerusProof::all()
- Driver-specific pool/connection/transaction plugins
- ToSqlxArgs factory + verification (Phase B+C)
- Phase 6C — SqlxWorkflowPlugin tests + connect_with
- Phase 6D — proposition combinator verification harnesses
- Refactor workflow to individual #[elicit_tool] methods
- Add RegexWorkflowPlugin with verified contracts and emit wiring
- Phase 1 — TokioTimePlugin with UUID-keyed time registry
- Phase 2 — TokioSyncPlugin with semaphore, notify, barrier
- Phase 3 — TokioRuntimePlugin with inspect_flavor
- Phase 4 — TokioFsPlugin with 14 async fs tools
- Phase 5 — TokioNetPlugin with 15 TCP/UDP tools
- Phase 6 — TokioProcessPlugin with 9 process tools
- Phases 7-10 — task, channels, signal, io plugins
- Phase 11 — TokioUnixPlugin with 15 Unix domain socket tools
- Phase 12 — spawn/spawn_blocking/block_in_place as emit-only tools
- Phase 12b — TokioSpawnPlugin spawn factory
- Add runtime::Builder emit-only tools
- TokioIoCopyPlugin — factory-based io::copy across plugins
- Full proof coverage — Props + kani/creusot/verus axioms
- Proof coverage for SqlxFragPlugin macro emit tools
- Derive Elicit on Props, add proof methods and coverage tests
- Enforce proof completeness via required trait methods
- Add ElicitComplete supertrait and fix proof delegation in generic types
- Add collection proof helpers and wire into wrapper types
- Wire primitive proof delegation through Default wrapper chain
- Add kani_array_all_satisfy helper and wire ArrayAllSatisfy composition
- Replace empty TokenStream::new() with real trivial proofs for style enums and unit structs
- Wire verus/creusot delegation chain and add stdlib primitive proof helpers
- Wire real proofs for all third-party types
- Add proof composition runtime validation
- Add validate_proofs_non_empty and full coverage tests
- Phase 1 ledger workflow smoke test
- Elevate Prop to require proofs, add #[derive(Prop)]
- Add VerifiedWorkflow registration, tests, and guide update
- Add Phase 3 dynamic transfers with parameterized queries
- Add Phase 4 constraint validation with pre-transfer checks
- Move typestate ledger to elicit_server
- Add ElicitPromptTree trait and derive support
- Add AccessKit bridge via prompt-tree-accesskit feature
- Annotate type graph nodes and edges with prompt text
- Initial shadow crate with all accesskit types
- Impl Elicitation for all 17 accesskit enum types
- ElicitSpec + ElicitComplete for all 17 accesskit enum types
- Egui Phase 0 — Select enums + select_trenchcoat! macro
- Egui Phase 0.4 — composite struct wrappers
- Egui Phase 0.7 — Kani, Creusot, and Verus proofs
- Phase 1 scaffold — shadow crate with 23 widget tools
- Complete Phase 1 — 32 widget tools, 31 WidgetJson variants
- Phase 2 — containers, layout, style, and response tools
- Phase 3 — runtime context management and egui 0.34
- Add menu_tools and input_tools modules
- Phases 4-7 — style expansion, menus, input, fragments (148 tools)
- Wire elicit_egui into server emit chain (Phase 8)
- Implement Phase 1 typestate UI verification system
- Phase 2 — constraint proofs and expanded tests
- Phase 3 — egui renderer for WCAG-verified AccessKit trees
- Renderer proofs — Kani, Creusot, Verus harnesses
- LayoutBuilder — ergonomic AccessKit tree construction
- LayoutBuilder verification — Kani/Creusot/Verus
- Add #[derive(ToCodeLiteral)] proc macro
- Scaffold elicit_ratatui crate with Phase 1 tools
- Add layout, text, and advanced widget tools
- Add 52 widget property setter tools and crossterm runtime dep
- Add runtime terminal/events and fragment code gen tools
- Phase 2 — Elicitation traits + ElicitComplete + Kani proofs
- Creusot + Verus proofs for ratatui types
- Non-trusted Creusot proofs for egui + ratatui composites
- Shadow struct Verus proofs for egui + ratatui composites
- Composable constraint system with GeoRust + cssparser + palette + taffy
- Phase 9A — ElicitComplete for geo-types primitives
- Phase 9B — ElicitComplete for palette Srgb
- Phase 9C — CssLength formal verification proofs
- Phase 9D — BoundingBox and LayoutMode verification proofs
- Phase 9E — WCAG contrast and constraint verification proofs
- Phase 9F ConstraintProfile and typestate proofs
- Add RenderBackend trait for dual-frontend rendering
- Bidirectional AccessKit bridges
- Add RatatuiBackend implementing RenderBackend
- Add terminal-specific constraints (Phase 10C)
- Terminal breakpoint verification system (Phase 10D)
- Add Creusot proof artifacts for egui types
- Add Creusot proof artifacts for ratatui types
- Support rich text (TextJson) in WidgetJson::Paragraph
- Remove proofs/verification feature flags, fix elicit_newtype! trait impls
- Auto-derive ToCodeLiteral from #[derive(Elicit)]; add as ElicitComplete supertrait
- Add full ElicitComplete support for all std::sync::atomic types
- Emit impl ElicitComplete from #[derive(Elicit)]
- ElicitComplete universal impls + generic collection/tuple support
- ToCodeLiteral + ElicitComplete for serde newtype wrappers
- ElicitComplete for all third-party elicitation types
- ElicitComplete for all elicit_clap, elicit_reqwest, elicit_sqlx types
- Newtype wrapper proof composition + ElicitComplete for all Elicit types
- Add ToCodeLiteral impl for fixed-size arrays [T; N]

### Documentation

- CONTRACT_PARAMS_PLAN — contract-carrying param types (replaces Phase 7)
- Add EMIT_AUTODERIVE_PLAN.md to planning index
- Add TYPE_GRAPH_GUIDE.md user guide + update planning index
- Add CREUSOT_GUIDE.md for Creusot 0.10.0 proof suite
- Update guide and tracking to reflect 240 SMT goals
- Add THIRD_PARTY_SUPPORT_GUIDE with complete integration checklist
- Add core verification principle to support guide
- Document clap de-trusting opportunities and string literal wall
- Add ElicitSpec (TypeSpec) step to third-party support guide
- Add REFLECT_TRAIT_PLAN.md for #[reflect_trait] macro design
- Replace REFLECT_TRAIT_PLAN with correct architecture
- Add Phase 3B trait factories to THIRD_PARTY_SUPPORT_GUIDE.md
- Rewrite README to reflect trait factory + register_convert design
- Add README covering wrappers, trait factories, type_map
- Add SHADOW_CRATE_MOTIVATION.md
- Fragment tools as third mechanism + macro checklist
- Add elicit_sqlx and PluginContext refactor plans
- Update plan to use StatefulPlugin / SqlxContext design
- Harden third-party support guide with proof execution requirements
- Add README explaining shadow crate motivation and type choices
- Document driver-specific plugin layer in README
- Document parameterized queries + ToSqlxArgsFactory (Phase D)
- Phase 6E — verified workflows README section
- Add Phase 3C/3D to third-party support guide; fix Fragment Tools EmitCode guidance
- Fix markdownlint issues in THIRD_PARTY_SUPPORT_GUIDE.md; configure MD060/MD024
- Fix 537 markdownlint errors across 34 files
- Add shadow-crate plugin inventory table and fix serde_json anchor
- Correct de-trusted section in tokio_types
- Add README in the spirit of elicit_sqlx
- Add PROPOSED_README.md — full architecture rewrite draft
- Improve style system and generators sections
- Update Creusot stats to 20,885 valid goals across 19 modules
- Update Creusot stats to 22,837 valid goals (post-boundary-fix run)
- Add soup-to-nuts Getting Started section
- Fix rmcp version to "1" (>=1.0 required)
- Rewrite Getting Started as single cohesive example
- Rewrite Getting Started using real elicitation API
- Comprehensive README overhaul
- Promote PROPOSED_README to README
- Add Phase 7 — ElicitComplete registration and proof validation
- Write README-style documentation for ElicitComplete
- Write README-style module documentation
- Add typestate ledger design using elicitation patterns
- Add comprehensive README for typestate ledger module
- Write comprehensive module-level documentation
- Add VISUALIZATION_GUIDE.md tying together all visualization layers
- Add README
- Add Phase 0 and guide cross-references to plan
- Add comprehensive README
- Update proof counts and tracking for builder proofs
- Add comprehensive module docs for ToCodeLiteral derive
- Add ToCodeLiteral and Prop to README
- Add GeoRust ecosystem integration plans
- Add foreign type composite proofs section to Creusot guide
- Add VERUS_GUIDE.md for proof quality standards
- Update Formal UI plan and constraint research
- Rewrite README for current architecture
- Add AccessKit bridge + dual-frontend IR section
- Rewrite README with full tool reference + AccessKit bridge
- Update 0.9.2 release notes for post-cutoff work
- Rename and update release notes for 0.10.0

### Fixed

- Move elicited_types binding inside cfg(proofs) scope
- Gate feature-conditional code and exports; rewrite check-features to use cargo-hack powerset
- Remove redundant bare `use toml;`
- Replace approximate PI literal with std::f64::consts::PI
- Use toml::from_str instead of .parse() for toml 1.0 compat; add dep-pipeline tests
- Run cargo creusot instead of cargo check in run_creusot_module
- Achieve 26/26 passing modules in tracked verification
- Use is_none_or instead of map_or(true, ...) in renderers
- Enable serde_json feature so 5 gated harnesses are compiled
- Add extern_specs for sqlx Select label counts; all 3 goals now proved
- Resolve EmitCode conflicts and dead code
- Replace hardcoded path dep with crates.io + patch, fix proof compilation errors
- Fix compilation errors with --all-features
- Fix proof fn mode errors in serde_boundary and sqlx_types
- Allow vec_init_then_push in verify_vec_non_empty_valid
- Wrap ToSqlxArgs proofs in verus! block
- Fix sqlx workflow scope, emit_rewriter Box<dyn Error>, smoke tests
- Gate UnixSignalKind re-export and unix-only imports with #[cfg(unix)]
- Replace live BTreeMap construction with symbolic proof in sqlx_types
- Correct verify-kani recipe package name, flags, and unwind
- Enforce third-party boundary in proof functions
- Remove duplicate proof methods in value_impl.rs
- Generate real proofs from derive(Elicit) macro
- Replace bare impl Prop with #[derive(Prop)] in test harnesses
- Pre-merge check failures — Prop impls, proof stubs, feature gates
- Gate VerifiedWorkflow impls for Unix-only signal types
- Resolve unused variable warnings under feature combinations
- Add missing ElicitPromptTree impls + regression tests
- Style system refactor + ElicitPromptTree feature gates
- Clippy warnings in elicit_ui, elicit_egui, elicit_accesskit, elicit_server
- Feature-gate egui tests for check-all compatibility
- Eliminate dead_code warnings in prompt_tree_test
- Gate prompt_tree datetime/serde_json impls with not(kani)
- Replace builder proofs with heap-free structural proofs
- ElementId::new takes u64, not NodeId
- Remove unsupported f64-to-f32 casts in ui_types proofs
- Eliminate all f32 from ui_types proofs
- Resolve cross-feature re-exports and mark opaque proofs trusted
- Add vstd::prelude import to ui_types.rs
- Restore emit code-gen with surgical shared-params suppression
- Use kani::assume() function not macro syntax
- Add geo-types and palette feature aliases for workspace compat
- Expose render_node and render_widget publicly
- Render ParagraphText::Rich to ratatui Text<'static>
- Fix all 9 failing ui_types Kani proofs
- Resolve all check-all clippy warnings
- Resolve remaining check-all clippy warnings
- Use dedicated kani/verus/creusot_atomic proof helpers
- Replace empty TokenStream proof methods with trusted boundary stubs
- Update Established<P> JsonSchema impl to schemars 1.x API
- Add missing ToCodeLiteral impls + fix DateTimeUtcAfter accessor
- Prove all StringNonEmpty goals in strings module
- Resolve check-all warnings
- Gate prompt-tree impls and fix taffy 0.10 API breakage

### Miscellaneous

- Rustfmt serde_boundary.rs + update Cargo.lock
- Prepare 0.9.1 — changelog + release notes
- Bump workspace to 0.9.1; update toml dep to 1.0
- Remove stale planning documents
- Commit proof artifacts from verify-creusot-prove
- Add .copilot/ to gitignore
- Bump to 0.10.0, update deps, organize Cargo.toml
- Remove obsolete Kani benchmark scripts

### Refactor

- Migrate workflow to #[elicit_tool] + #[derive(ElicitPlugin)]
- Migrate workflow to #[elicit_tool] + #[derive(ElicitPlugin)]
- Migrate workflow to #[elicit_tool] + #[derive(ElicitPlugin)]
- Migrate workflow to #[elicit_tool] + #[derive(ElicitPlugin)]
- Migrate all plugins to #[elicit_tool] + #[derive(ElicitPlugin)]
- Remove graph feature gate — always on
- Replace SerdePlugin with #[reflect_trait] factories
- Replace concrete PluginContext with trait + typed contexts
- Replace manual dispatcher with #[elicit_tool] + CustomEmit
- Migrate fetch_and_parse to #[elicit_tool] + #[derive(ElicitPlugin)]
- Migrate to #[elicit_tool] + add emit=None macro mode
- Replace all glob re-exports with explicit named exports
- Move egui renderer from elicit_ui to elicit_egui
- Consolidate derive attrs and format assert! calls in derive tests
- Reformat proof coverage assert! calls to multi-line style

### Styling

- Rustfmt cleanup (line wraps, import order)
- Rustfmt cleanup
- Cargo fmt
- Fix blanks-around-fences lint in README
- Apply rustfmt formatting
- Apply rustfmt formatting
- Apply rustfmt formatting
- Apply rustfmt formatting

### Testing

- Serde boundary tests for constrained types
- 20 integration tests for all clap trait factories
- Add fragment unit tests and live DB integration tests
- Add Phase 5b typestate integration tests
- Add Phase 6 concurrent transfer tests
- Add 64 integration tests for serde types and conversions
- Proof_coverage_test.rs for all elicit_* crates

### Ci

- Add pull_request trigger and feature-powerset job
- Move check-features to pre-merge only; remove from CI pipe

### Vendor

- Vendor creusot-std, creusot-std-proc, pearlite-syn from creusot 0.10

## [0.9.0] - 2026-03-06

### Added

- Phase 1 — erased-serde dep + serde derives on elicitation types
- Phase 2 — derive(Elicit) emits serde derives on Style enums
- Scaffold SerdePlugin with 4 MCP tools via erased-serde
- Phase 3 — ElicitJson blanket impl for single-shot JSON elicitation
- Phase 4 — serde derives on param structs and newtype wrappers
- Phase 6 — serde_json newtype wrappers via library macros
- Verified JSON workflows with typestate contracts
- EmitCode trait — code recovery foundation
- EmitCode impls for serde_json and reqwest workflow tools
- Add EmitBinaryPlugin for code recovery
- Workspace path override + smoke test
- Support tuple structs in #[derive(Elicit)]
- Support unit structs in #[derive(Elicit)]
- New crate — JsonSchema-compatible Uuid newtype
- Add elicit_time, elicit_regex shadow crates; reqwest type newtypes
- Add elicit_chrono, elicit_url, elicit_jiff shadow crates
- Add workflow plugins to elicit_url, elicit_chrono, elicit_jiff, elicit_time
- Cross-crate workflow plugins, EmitCode for all new crates, smoke test parity
- Replace tautological proof stubs with real TokenStream-returning proof generators
- Wire all existing kani/verus/creusot harnesses into emission methods for all types

### Documentation

- Regenerate CHANGELOG.md for v0.9.0
- Add README for elicit_serde
- Add README for elicit_server
- Rewrite elicit_server README to convey cross-crate composition value

### Fixed

- Add ParsedJson::into_value and pointer_update; fix serde cfg warning
- Impl blocks, imports and functions properly gated for unix only, enabling compilation on Windows.
- Suppress unexpected_cfgs on generated serde cfg_attr
- Remove serde cfg_attr from generated style enums
- Remove unused Generics and Ident imports
- Complete rmcp 1.1.0 migration — clippy clean pre-merge
- Add tokio dev-dep for doctest compilation
- Use forward slashes in path deps for Windows TOML compat
- Silence non_snake_case warning in impl_emit_tuple macro
- Add shadow crates to workspace.dependencies for workspace = true inheritance

### Miscellaneous

- Migrate to rmcp 1.1.0 API
- Increment version for Verus proofs.

### Styling

- Run cargo fmt across workspace

### Testing

- Add derive regression coverage matrix and cross-platform CI
- Add third-party MCP compat coverage matrix

### Ci

- Add workflow_dispatch trigger for manual runs

### Merge

- Dev → main (rmcp 1.1.0 migration, derive coverage, CI workflow)

## [0.8.3] - 2026-03-02

### Added

- Add elicit_newtype! declarative macro for transparent wrappers
- Add method reflection infrastructure for MCP tool generation
- Connect method reflection pipeline with parameter struct generation
- Implement wrapper method generation logic for MCP tools
- Integrate MCP tool wrapper generation into reflect_methods pipeline
- Add elicit_newtype_methods! declarative macro
- Add generic method support
- Add generic support for struct derives
- Add generic support for enum derives
- Add generic support for #[derive(Rand)]
- Use Arc internally in elicit_newtype! for universal Clone support
- Add shadow crate skeleton for integration testing
- Add consuming method support to elicit_newtype_methods!
- Add Phase 2 implementation with declarative macro limitations
- Add consuming method support to #[reflect_methods]
- Add reqwest feature with full Elicitation trait impls
- Add Kani/Verus/Creusot coverage for reqwest types
- Implement ElicitPlugin for elicit_reqwest with HTTP tools
- Expose all reqwest type methods as MCP tools
- Add WorkflowPlugin with 10 phrase-level HTTP tool compositions
- Phase 2 anodized spec layer - complete std + feature-gated specs
- Phase 3 TypeSpecPlugin MCP tools
- Phase 2 gaps - time OffsetDateTime and reqwest StatusCode specs
- Phase 4 composed ElicitSpec via #[derive(Elicit)]
- Anodized TypeSpec explorer — phases 1-4

### Documentation

- Add method reflection implementation plan
- Fix newtype macro documentation with correct syntax examples
- Update METHOD_REFLECTION_PLAN with implementation progress
- Add README
- Add quick start section to README
- Update README with newtype macros and TypeSpec explorer
- Add README and update Cargo.toml metadata
- Add README and update Cargo.toml metadata
- Add README and update Cargo.toml metadata
- Add README and update Cargo.toml metadata
- Regenerate full changelog from git history

### Fixed

- Add generic support for elicit_checked tool generation
- Replace single-arm match with if let (clippy)
- Resolve CI failures in elicit_spec_derive tests
- Remove duplicate readme key in Cargo.toml

### Miscellaneous

- Bump version to 0.8.3 and update derive-new to 0.7
- Set crate-specific readme, description, keywords, categories
- Bump to 0.8.3 and update verus deps to 2026-03-01

### Refactor

- Remove generic method support from declarative macro
- Remove ad-hoc trait impls, document coverage gaps

### Styling

- Fix clippy warnings in method_reflection module
- Apply cargo fmt to struct_impl function calls

### Testing

- Add integration tests for method reflection

### Merge

- Plugin registry + 81-tool reqwest shadow crate
- Doc sprint - crate READMEs, changelog, version 0.8.3
- Bump elicitation_verus to 0.8.3, update verus deps 2026-03-01

## [0.8.2] - 2026-02-23

### Added

- First working proof module - bools
- Add char contract type proofs
- Add i8 integer contract type proofs
- Complete integer contract type proofs
- Add float contract type proofs
- Add string contract type proofs
- Add collection contract type proofs
- Add duration and tuple contract type proofs
- Complete all remaining contract type proofs
- Add base type proofs for stdlib and external crates
- Add verus_proof trait method for compositional verification
- Add char contract proofs
- Complete cloud of assumptions approach
- Add string contract proofs
- Add float contract proofs
- Add duration and tuple contract proofs
- Add collection contract proofs
- Add network address contract proofs
- Complete coverage with 171 proofs across all derivable types
- Add creusot_proof() trait integration for compositional verification
- Create elicitation_prusti workspace crate with 239 proofs
- Add verification trenchcoat coverage (456 total proofs)
- Add verification trenchcoat coverage (27 modules total)
- Add prusti_proof() trait integration
- Add verification trenchcoat coverage (456 total proofs)
- Add verification trenchcoat coverage (27 modules total)
- Add prusti_proof() trait integration
- Update derive macros for compositional verification
- Export all proof functions from lib.rs
- Add creusot_runner module for compilation tracking
- Complete CLI integration and justfile recipes
- Implement branch strategy for edition compatibility
- Add edition detection and full verification workflow

### Documentation

- Document verification status
- Add progress tracking summary
- Add compositional verification example and comprehensive tracking documentation
- Update tracking with trenchcoat coverage (456 proofs)
- Update tracking with trenchcoat coverage (456 proofs)
- Add comprehensive tracking documentation
- Update tracking docs with CLI usage
- Update Prusti branch documentation to reflect frozen status

### Fixed

- Exclude elicitation_verus from cargo builds
- Remove remaining verify-creusot references
- Remove verify-creusot from examples
- Fix syntax error in verification_multi_example
- Restore verification trenchcoat proofs (393 total)
- Restore verification trenchcoat proofs (393 total)
- Restore missing module files from origin/main
- Restore all 19 proof modules from main branch
- Add missing kani_proof() to DateTimeComponents
- Add Elicitation impl for DateTimeComponents
- Remove invalid DateTimeComponents kani_proof calls
- Add kani_proof() to DateTimeComponents for full verification
- Implement Elicitation trait for DateTimeComponents
- Add feature flags to Cargo.toml
- Implement Elicitation trait for DateTimeComponents
- Add verify-kani feature for workspace compatibility
- Scope runner to elicitation package only
- Temporarily exclude Creusot/Prusti to unblock Kani
- Clean workspace build - zero errors
- Remove invalid check-cfg for feature flags
- Complete module imports and exports
- Remove star imports and add cfg guards
- Rewrite runner to use elicitation_verus crate with JSON parsing
- Remove 78 non-functional creusot stub functions
- Add --all-features flag to cargo kani invocation
- Use std::vec! to avoid macro ambiguity
- Add kani to check-cfg allowlist
- Remove verify-kani feature references
- Resolve compilation errors in elicitation_kani crate
- Fix ValidationError import in elicitation_kani crate
- Remove misplaced Prusti code generation
- Eliminate all warnings in Kani verification builds
- Kani registration in cargo
- Use workspace dependencies for elicitation
- Remove publish = false to allow publishing
- Add verification feature to elicitation dependency
- Add elicitation dependency with verification feature
- Update gitignore to exclude all target directories
- Email for author
- Add missing Verus standard library dependencies
- Add Verus stdlib dependencies and update gitignore
- Remove unused star imports and use explicit SpecOrd imports
- Remove path dependency for publishability

### Miscellaneous

- Simplify lib.rs, add cfg checks
- Bump version to 0.8.2 and fix shellexpand workspace dependency
- Update CHANGELOG.md for v0.8.2
- Add package metadata to creusot and verus crates

### Refactor

- Rename crate to elicitation_creusot
- Remove Creusot code from main crate
- Remove verify-creusot feature from main crate
- Remove inline proof directory
- Remove Prusti from dev branch
- Extract Kani to dedicated crate

### Styling

- Reorder imports for consistency

### Wip

- Attempt adding contracts to main crate

## [0.8.1] - 2026-02-17

### Documentation

- Update CHANGELOG for v0.8.1

### Fixed

- Suppress unexpected_cfgs warnings for kani

## [0.8.0] - 2026-02-16

### Added

- Add ElicitationContext for tracking elicitation state
- Add ChoiceSet for dynamic choice elicitation
- Add Filter trait for filtered selection

### Documentation

- Add dynamic_choices example demonstrating ChoiceSet
- Add compositional verification examples and guide

### Fixed

- Update tests and examples for v0.8.0 API changes
- Remove --execute flag from pre-release dry-run

### Miscellaneous

- Update author field with email

### Refactor

- Change Select/Survey traits to return owned Vec types
- Update macros to generate Vec returns for Select/Survey

### Testing

- Add tests for ChoiceSet dynamic choice elicitation

## [0.7.0] - 2026-02-14

### Added

- Update to rmcp 0.15 with SEP-1577 sampling changes

### Fixed

- Server-side elicitation for enum and struct derive macros
- Use send_prompt() for server-side elicitation in all verification types

### Miscellaneous

- Archive outdated planning docs and cleanup
- Apply pre-merge clippy fixes

## [0.6.11] - 2026-02-09

### Added

- Add UUID generator Kani proofs - Phase 1 complete
- Register UUID generator proofs in verification runner
- Add IoError and JsonError generator Kani proofs - Phase 2 complete
- Add Kani proofs for chrono datetime generators (Phase 3)
- Add Kani proofs for time and jiff generators (Phase 4)
- Create elicitation_rand crate with implementation plan
- Implement Phase 1 - RNG elicitation (StdRng, ChaCha8Rng)
- Implement Phase 2 - RandomGenerator for primitives
- Implement Phase 3 - Distribution generators (Uniform, Weighted)
- Phase 5 - Kani verification (bounds logic only)
- Phase 1 MVP - Derive macro for contract-aware random generation
- Contract composition with And/Or support
- Full struct support with per-field contracts
- Enum support with uniform variant selection
- Collection support with VecGenerator
- Rand trait architecture + unit struct support
- Third-party type support (String, chrono, jiff, time)
- Paradigm integration (Select, Survey, Affirm)
- Add default "full" feature bundle
- Add Rand implementations for UUID, URL, PathBuf

### Documentation

- Complete README rewrite - tutorial-driven approach
- Add comprehensive Requirements and Constraints section
- Add comprehensive rmcp tool router integration guide
- Fix generator checklist to reflect 100% coverage
- Update CHANGELOG.md for v0.6.11
- Regenerate complete CHANGELOG.md from scratch

### Fixed

- Migrate elicitation_rand to use workspace dependencies
- Rename gen variables to generator
- Change gen_bool to random_bool for rand 0.10
- Use path for workspace member dependencies
- Regular dependencies use workspace, dev-dependencies use path

### Miscellaneous

- Allow manual_async_fn lint in trait tools test
- Run cargo fmt

### Refactor

- Consolidate derive macros into single crate

## [0.6.10] - 2026-02-08

### Added

- Add async_trait support for trait tools macro

### Miscellaneous

- Bump version to 0.6.10

## [0.6.9] - 2026-02-08

### Added

- Basic trait tools macro with explicit method list

### Documentation

- Add guide for tool_router warnings
- Add comprehensive guide for elicit_trait_tools_router macro

### Miscellaneous

- Bump version to 0.6.9 and format code

### Wip

- Start TDD implementation of elicit_trait_tools_router macro

## [0.6.8] - 2026-02-07

### Fixed

- Add ElicitToolOutput wrapper for MCP enum compatibility
- Remove unused imports from enum test

### Miscellaneous

- Increment patch version

## [0.6.7] - 2026-02-07

### Added

- Add Elicit trait for feature-gated types

### Documentation

- Add tool composition example and update changelog

### Fixed

- Rmcp API integration for elicit_tools proc macro

### Miscellaneous

- Bump version to 0.6.7
- Lockfile checkin

### Refactor

- Simplify server-side elicitation - add method to Elicitation trait

### Testing

- Verify all feature-gated types have elicit_checked()
- Prove tool composition works - regular + elicit tools together

## [0.6.6] - 2026-02-06

### Added

- Implement #[elicit_tools(...)] proc macro attribute

### Documentation

- Update CHANGELOG.md for 0.6.6 release

### Miscellaneous

- Bump version to 0.6.6

## [0.6.5] - 2026-02-06

### Added

- Add elicit_tools! macro for embedding in existing impl blocks

### Documentation

- Document JsonSchema requirement for Elicit derive
- Add integration guide for botticelli

### Fixed

- Add &self to elicit_router methods for rmcp compatibility
- Update canary test for new method signature
- Silence dead_code warnings in test structs
- Update time to 0.3.47 (RUSTSEC-2026-0009)

### Miscellaneous

- Bump version to 0.6.5
- Remove outdated phase3_test example
- Remove outdated phase4/phase5 examples

## [0.6.4] - 2026-02-06

### Fixed

- Replace all client references with communicator in manual implementations
- Remove unused imports (clippy --fix)
- Replace all client references with communicator (0.6.4)

### Miscellaneous

- Bump version to 0.6.4

## [0.6.3] - 2026-02-06

### Added

- Add Agent vs Human style variants to String

### Fixed

- Update derive macro parameter and documentation
- Remove unused imports and add allow(dead_code) for test struct

## [0.6.2] - 2026-02-06

### Added

- Server-side rmcp tool integration architecture
- Add ElicitServer for unified client/server support
- Implement ElicitCommunicator trait for unified client/server support
- Complete derive macro integration with ElicitCommunicator

### Documentation

- Add server-side elicitation implementation plan
- Add unified elicitation trait planning document
- Add comprehensive server-side elicitation documentation
- Add merge summary for server-side elicitation

### Fixed

- Remove .peer() call in enum_impl
- Enum_impl derive macro .peer() call

### Miscellaneous

- Increment version to 0.6.2

## [0.6.1] - 2026-02-05

### Documentation

- Update CHANGELOG for 0.6.1 release

### Fixed

- Remove cfg_attr test guard to enable downstream testing

### Miscellaneous

- Bump version to 0.6.1

## [0.6.0] - 2026-02-05

### Added

- Generate elicit_checked() methods
- Add inventory-based automatic tool discovery

### Documentation

- Update README and CHANGELOG for 0.6.0 API

### Miscellaneous

- Update bytes to 1.11.1 to fix RUSTSEC-2026-0007

### Refactor

- Centralize all dependencies in workspace Cargo.toml

## [0.5.0] - 2026-02-05

### Added

- [**BREAKING**] Make all APIs Arc<Peer> compatible for tool registration

### Documentation

- Add Elicit tool attribute patch documentation
- Update CHANGELOG for 0.5.0 release
- Add elicit_checked() method generation proposal

### Fixed

- Use cfg_attr for rmcp::tool to support test builds
- Clean up Arc<Peer> migration issues

### Miscellaneous

- [**BREAKING**] Bump version to 0.5.0
- Update Cargo.lock

## [0.4.8] - 2026-02-05

### Added

- Add Elicitation implementation for unit type ()
- Add SystemTime Kani proofs with cloud assumptions

### Documentation

- Fix missing documentation warnings

### Fixed

- Remove unused verify-* features causing cfg warnings
- Resolve type complexity and doc warnings

### Miscellaneous

- Update version to 0.4.8

### Testing

- Add Kani proofs for unit type ()

## [0.4.7] - 2026-02-04

### Added

- Add Phase 1.1 - Core contract types
- Add Phase 1.1 - Core contract types
- Add Phase 1.2 - Logical implication
- Add Phase 1.3 - Conjunction algebra
- Add Phase 2.1 - Proof-returning elicitation
- Add Phase 2.2 - Type refinement system
- Add Phase 2.3 - Enum variant proofs
- Add Phase 3.1 - Tool trait with contracts
- Add Phase 3.2 - Tool composition
- Add Phase 4.1 - Basic contract proofs
- Add Phase 4.2 - Tool chain verification proofs

### Documentation

- Add contracts implementation plan
- Add composition primitives vision document
- Add Phase 1.4 - Comprehensive documentation
- Phase 5 documentation and examples
- Update CHANGELOG for v0.4.7 release

### Fixed

- Update type names in Kani proofs

### Miscellaneous

- Bump version to 0.4.7 for contracts release

## [0.4.6] - 2026-02-04

### Fixed

- Remove verification code generation to eliminate warnings

### Miscellaneous

- Update changelog for 0.4.6
- Update Cargo.lock for 0.4.6

## [0.4.5] - 2026-02-03

### Added

- Auto-generate MCP tool functions for #[derive(Elicit)]
- Implement Elicitation for regex contract types
- Add Generator trait and Instant support
- Add Generator support for UUID
- Add Generator support for SystemTime
- Add Generator support for Duration
- Add Generator support for OffsetDateTime
- Add Generator support for datetime libraries
- Add Generator for unit structs - the simplest elicitation!
- Add Generator for std::io::Error and serde_json::Error

### Fixed

- Remove cfg feature gates from generated code
- Generate only Kani verification in user crates
- Restore all verifier backends without cfg gates
- Use cfg gates with allow directives for all verifiers
- Build errors and verification feature gates
- Export datetime generators and fix doctest examples
- Remove duplicate attributes and add missing impls

### Miscellaneous

- Bump version to 0.4.5 and update changelog
- Update Cargo.lock for 0.4.5 release

## [0.4.4] - 2026-02-03

### Added

- Add Creusot deductive verifier support
- Add comprehensive verification tool setup and management
- Implement Phase 1 - primitive contracts with Kani
- Implement Phase 2 - multi-verifier primitive contracts
- Implement Phase 3.1 - verifier backend registry
- Implement with_contract() method (Phase 3.2)
- Compile-time contract selection (Phase 3.3)
- Contract composition operators (Phase 3.4)
- Complete Phase 3 - Testing & documentation
- Add unsigned integer contracts (Phase 4.1)
- Add signed integer contracts (Phase 4.2)
- Add floating point contracts (Phase 4.3)
- Complete Phase 4 - All primitive types (Phase 4.4)
- Add Phase 5 complex type contracts
- Add Phase 6 comprehensive examples
- Add mechanism-level contracts POC
- Complete vision with Trenchcoat Pattern
- Add I8Positive contract type (Phase 7 POC)
- Implement i8 contract types (Phase 7)
- Add i16, u8, u16 contract type families (Phase 7)
- Complete all integer contract types with macros (Phase 7)
- Add float contract types (Phase 8)
- Add char elicitation support
- Add string, bool, char contract types (Phase 9)
- Add UUID and Duration contract types
- Add Network contract types
- Add PathBuf contract types (Phase 10 complete)
- Add DateTime contract types (Phase 11 complete)
- Add Tuple contract types (Phase 12 partial)
- Add Collection contract types (Phase 13 complete)
- Complete all collection and JSON contract types
- Add Kani proof harnesses for contract types
- Add 34 new Kani proof harnesses
- Prove const generic Range types with Kani
- Complete Kani proof coverage for all contract types
- Add mechanism contract proofs (Survey, Affirm, composition)
- Add Verus proof infrastructure (86 proof stubs)
- Implement 30 actual Verus proofs (~35% complete)
- Complete Verus implementation - 86/86 proofs (100%)
- Complete Creusot implementation - 86/86 proofs (100%)
- Complete Prusti implementation - 86/86 proofs (100%)
- Add URL contract types (5 types)
- Add URL verification proofs for all 4 verifiers
- Add Regex contract types (5 types)
- Add Regex verification proofs for all 4 verifiers
- Add proc macro for automatic instrumentation
- Roll out instrumentation macro to all contract types
- Add full instrumentation to client module
- Add instrumentation to Style enums, MCP, errors, contracts
- Instrument business logic helper functions
- Add verification contract generation infrastructure
- Add contract metadata introspection (#[contract_type])
- Complete Phase 2 - metadata query infrastructure
- Phase 3 - Generate real Kani contracts
- Add U8Positive and U16Positive types
- Add InputNonEmpty and export DurationNonZero
- Fix verification test and document K
- Implement Phase 4 - Enum verification support
- Add Creusot backend support (Phase 5.1)
- Add Prusti backend support (Phase 5.2)
- Add Verus backend support (Phase 5.3 COMPLETE)
- Implement all missing contract types (Phase 5 completion)
- Add Verus support via external binary
- Add Utf8Bytes foundation for string contracts
- Add comprehensive Kani UTF-8 symbolic proofs
- Add just recipes for Kani benchmarks and long proofs
- Add UUID byte-level validation foundation
- Add IP address byte-level validation foundation
- Add MAC address byte-level validation foundation
- Add SocketAddr byte-level validation foundation
- Add PathBuf (Unix) byte validation with compositional UTF-8
- Systematic Vec → from_slice() pattern for Kani
- Add regex validation with recursive trait bounds
- Add verification benchmark suite for all proof types
- Add marginal cost benchmarking for Kani proofs
- Add checkpointed chunked proof system
- Add 2-byte chunked proofs
- Make chunked proofs dynamically configurable
- Add 2-byte 16-chunk configuration
- Add 2-byte 16-chunk harnesses
- Add MCP derives to integer types
- Add MCP wrapper types for primitives
- Use MCP wrapper types internally
- Replace manual parsing with serde in MCP wrapper types
- Replace manual implementations with macro-generated Default wrappers
- Add CSV-based Kani verification tracking system
- Apply cfg(kani) pattern to external types
- Apply cfg(kani) pattern to regex/URL types and fix instrumentation
- Apply castle-on-cloud pattern to UTF-8 validation
- Convert UTF-8 validation tests to wrapper tests
- Apply castle-on-cloud to URL parsing and validation
- Add UTF-8 validation proofs for Prusti
- Add PathBytes validation proofs for Prusti
- Add RegexBytes validation proofs for Prusti
- Add UrlBytes validation proofs for Prusti
- Add IpAddrBytes validation proofs for Prusti
- Add network proofs for SocketAddr and MacAddr
- Add UuidBytes validation proofs for Prusti
- Add Prusti verification runner with CSV tracking
- Integrate Prusti runner with CLI and justfile
- Migrate Creusot proofs to 0.9 and plan core refactor

### Documentation

- Add verification implementation plan
- Revise verification plan - simple to complex, Kani to many
- Complete Phase 6 comprehensive documentation
- Add Total Verification master plan
- Add compositional verification implementation plan
- Add missing documentation for U8Positive and U16Positive methods
- Add Kani limitations and workarounds documentation
- Add comprehensive Kani verification patterns guide
- Add MCP verification integration plan
- Update integration plan with Phase 1 progress
- Add MCP integration summary
- Correct verification characterization from 'smoke test' to 'compositional verification'
- Add Prusti verification tracking documentation
- Document Prusti Edition 2024 incompatibility
- Add comprehensive verifier status assessment
- Update CHANGELOG for unreleased changes

### Fixed

- Fix Kani proof compilation errors
- Correct verification plan - Elicit IS Verifiable
- Resolve import errors for kani proofs
- Add type annotations to kani proofs
- Make instrumented_impl no-op under Kani + add missing imports
- Add missing type imports to kani_proofs files
- Replace kani::assert with standard assert! macro
- Replace kani::assert with assert! in verification/kani.rs
- Add I8Positive import and fix type annotations in mechanisms.rs
- Fix proof signatures for Prusti and Creusot
- Feature-gate verification examples
- Add loop unwind bounds to prevent Kani infinite loops
- Replace all to_string() with String::from() in Kani proofs
- Replace UUID parse_str with from_bytes in Kani proofs
- Add --default-unwind 20 to Kani verification
- Increase unwind bounds for URL and Regex proofs
- Update i64 prompt to match test expectations
- Preserve serde error source chain in wrapper deserialization
- Add provably-correct unwinding bounds for UTF-8 Kani proofs
- Add unwind bounds to all UTF-8 Kani proofs
- Add unwind bounds to all Kani proofs and remove unused imports
- Complete unwind bounds for remaining Kani proofs
- Remove invalid feature gate and unused imports
- Add missing NoContent match arm and gate Duration import
- Gate UUID types and unused Duration import with cfg(kani)
- Match UUID types export gate to definition gate
- Enable all optional features in runner
- Fix RegexSetNonEmpty and pathbuf kani proofs
- Remove String payloads from IP version errors
- Apply castle-on-cloud to scheme validation tests
- Apply castle-on-cloud pattern to URL composition tests
- Reduce buffer size in string contract test
- Remove as_str() call in regex literal test
- Apply castle-on-cloud to pathbytes validation
- Apply castle-on-cloud to PathAbsolute and remaining pathbytes tests
- Apply castle-on-cloud to float contract types
- Apply castle-on-cloud to more float and char contract types
- Remove incompatible assertions from symbolic validation tests
- Fix Prusti installation and clean dead code
- Fix Prusti env var (PRUSTI_CHECK_OVERFLOWS)
- Fix Creusot imports and add creusot-std dependency
- Complete Verus setup and update justfile
- Correct URL type names in urlbytes tests
- Use path dependency for elicitation_derive tests

### Miscellaneous

- Update creusot-contracts from 0.2 to 0.8
- Resolve compilation warnings
- Add description to elicitation_macros for crates.io
- Release

### Refactor

- Generic contract framework for multiple verifiers
- Update plan with contract-as-newtype vision
- Modularize types into submodules
- Split kani_proofs.rs into 11 modular files
- Split verus_proofs.rs into 11 modular files
- Split creusot_proofs.rs into 11 modular files
- Split prusti_proofs.rs into 11 modular files
- Remove glob imports, make all exports explicit
- Complete kani_proofs import and API fixes
- Refactor StringNonEmpty to use Utf8Bytes foundation
- Simplify chunked proofs to use library harnesses
- Clean chunked proof syntax to use plain numbers
- Eliminate unwrap calls in parameter builders

### Testing

- Add comprehensive type coverage test

### Wip

- Adding missing integer type exports

## [0.4.3] - 2026-01-19

### Fixed

- Add missing integer type implementations (i128, isize, u128, usize)

### Miscellaneous

- Release

## [0.4.2] - 2026-01-19

### Added

- Add UUID elicitation support

### Miscellaneous

- Release

## [0.4.1] - 2026-01-19

### Documentation

- Fix CHANGELOG.md to reflect actual published version 0.4.0
- Update documentation for 0.4.1 release

### Miscellaneous

- Release

## [0.4.0] - 2026-01-19

### Added

- Add Value elicitation support
- Add chrono datetime elicitation support
- Add time datetime elicitation support
- Add jiff datetime elicitation support
- Add elicitation style system - REVOLUTIONARY
- Add field-level style support with derive integration
- Update derive macro for Style System v2
- Add ElicitationStyle trait for extensible style system
- Complete Phase 3 - Inline elicitation for all primitives
- Complete Phase 4 - Auto-selection for styles
- Complete Phase 5 - Builder pattern for ergonomic style overrides

### Documentation

- Add serde_json::Value elicitation implementation plan
- Add comprehensive datetime elicitation implementation plan
- Add datetime plan to index
- Add comprehensive elicitation style system implementation plan
- Document all 0.2.2 features and bump version
- Plan Style System v2 with associated types
- Add custom_style example demonstrating ElicitationStyle
- Update STYLE_SYSTEM_V2_PLAN with Phase 1-2 completion status
- Mark all phases complete - Style System v2 ready for v0.3.0
- Update CHANGELOG.md for 0.3.0 release

### Fixed

- Update all examples to use ElicitClient
- Fix example imports and styled struct derive
- Add allow(dead_code) to test structs
- Update release.toml to remove deprecated [workspace] section

### Miscellaneous

- Prepare for 0.3.0 release
- Release

### Styling

- Run cargo fmt to fix formatting

### Testing

- Delete worthless style_derive_test.rs

### Wip

- Start Style System v2 - primitives done
- Complete primitives including tuples
- Add Option and Result style implementations
- Complete all containers
- Complete all collections with Style v2
- Complete all datetime types with Style v2
- Complete serde_json::Value with Style v2

## [0.2.1] - 2026-01-18

### Added

- Migrate from pmcp to rmcp 0.12.0 SDK
- Add enum variant field support (v0.2.1)

### Documentation

- Add publication enhancements and fix examples

### Miscellaneous

- Prepare v0.2.0 release with publication metadata
- Update derive_more to v2
- Update Rust edition to 2024
- Enhance release workflow with git-cliff and cargo-release

### Styling

- Fix import ordering per rustfmt

## [0.1.0] - 2025-12-28

### Added

- Implement foundation for elicitation library
- Implement primitive type elicitation
- Implement container type elicitation
- Implement derive macro for enums (Select pattern)
- Implement derive macro for structs (Survey pattern)
- Add standard library collection support (HashMap, BTreeMap, HashSet, BTreeSet)
- Add VecDeque and LinkedList collection support
- Add PathBuf filesystem path support
- Add network type support (IP addresses and socket addresses)
- Add Duration time duration support
- Add Result<T, E> container support for success/failure outcomes
- Add v0.3.0 advanced types (tuples, smart pointers, arrays)

### Documentation

- Add planning documents for elicitation library
- Add implementation plan for v0.1.0
- Add comprehensive documentation and examples (Phase 6)
- Add comprehensive roadmap for stdlib type support
- Add MCP setup instructions and clarify client requirement
- Update ROADMAP to use Elicitation trait name
- Mark HashMap, BTreeMap, HashSet, BTreeSet as completed in ROADMAP
- Add comprehensive validation roadmap and ecosystem analysis
- Update ROADMAP with refined validation strategy (v0.4.0-0.6.0)

### Miscellaneous

- Add markdownlint config and fix linting issues
- Update repository URL and authors in Cargo.toml
- Prepare v0.1.0 release

### Refactor

- Apply arcgis error pattern for boilerplate reduction
- Rename core trait to Elicitation, expose derive as Elicit

<!-- generated by git-cliff -->
