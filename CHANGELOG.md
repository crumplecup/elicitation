# Changelog

All notable changes to the `elicitation` project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

### Fixed

- Add generic support for elicit_checked tool generation
- Replace single-arm match with if let (clippy)
- Resolve CI failures in elicit_spec_derive tests

### Miscellaneous

- Bump version to 0.8.3 and update derive-new to 0.7
- Set crate-specific readme, description, keywords, categories

### Refactor

- Remove generic method support from declarative macro
- Remove ad-hoc trait impls, document coverage gaps

### Styling

- Fix clippy warnings in method_reflection module
- Apply cargo fmt to struct_impl function calls

### Testing

- Add integration tests for method reflection

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
