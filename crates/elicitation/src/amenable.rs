//! Constitutional proof roles for lawful sidecar exchange.
//!
//! This module defines the dependency-light trait family intended for a future
//! `amenable` crate boundary. It does not depend on `elicitation` proof types.
//! Concrete adapters that bridge this constitutional surface onto a local proof
//! system belong in sibling modules.

use std::marker::PhantomData;

/// Marker trait for a verifier backend.
pub trait Verifier: 'static {
    /// Structured reporting surface for backend-specific provenance and
    /// configuration disclosure.
    type Metadata: Provenance;

    /// Canonical backend name for audit and display purposes.
    fn name() -> &'static str;

    /// Provenance metadata describing the verifier backend and its reporting
    /// surface.
    fn metadata() -> Vec<MetadataEntry> {
        <Self::Metadata as Provenance>::metadata()
    }
}

/// The Kani verifier backend.
pub struct KaniVerifier;

/// Provenance surface for the Kani verifier backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct KaniVerifierMetadata;

impl Provenance for KaniVerifierMetadata {
    fn metadata() -> Vec<MetadataEntry> {
        vec![
            MetadataEntry::new("verifier_family", "kani"),
            MetadataEntry::new("authority", "Kani Rust Verifier"),
            MetadataEntry::new("source_url", "https://model-checking.github.io/kani/"),
            MetadataEntry::new("proof_artifact", "Rust proof harness token stream"),
            MetadataEntry::new(
                "configuration_channel",
                "CLI arguments and KANI_* or PROVE_* environment variables",
            ),
            MetadataEntry::new(
                "configuration_surface",
                "package selection, flags, timeout, and report output",
            ),
        ]
    }
}

impl Verifier for KaniVerifier {
    type Metadata = KaniVerifierMetadata;

    fn name() -> &'static str {
        "kani"
    }
}

/// The Creusot verifier backend.
pub struct CreusotVerifier;

/// Provenance surface for the Creusot verifier backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CreusotVerifierMetadata;

impl Provenance for CreusotVerifierMetadata {
    fn metadata() -> Vec<MetadataEntry> {
        vec![
            MetadataEntry::new("verifier_family", "creusot"),
            MetadataEntry::new("authority", "Creusot project"),
            MetadataEntry::new("source_url", "https://creusot-rs.github.io/creusot/"),
            MetadataEntry::new("proof_artifact", "Why3-oriented proof token stream"),
            MetadataEntry::new(
                "configuration_channel",
                "CLI arguments and CREUSOT_* or PROVE_* environment variables",
            ),
            MetadataEntry::new(
                "configuration_surface",
                "package selection, flags, binary path, timeout, and report output",
            ),
        ]
    }
}

impl Verifier for CreusotVerifier {
    type Metadata = CreusotVerifierMetadata;

    fn name() -> &'static str {
        "creusot"
    }
}

/// The Verus verifier backend.
pub struct VerusVerifier;

/// Provenance surface for the Verus verifier backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VerusVerifierMetadata;

impl Provenance for VerusVerifierMetadata {
    fn metadata() -> Vec<MetadataEntry> {
        vec![
            MetadataEntry::new("verifier_family", "verus"),
            MetadataEntry::new("authority", "Verus project"),
            MetadataEntry::new("source_url", "https://verus-lang.github.io/verus/"),
            MetadataEntry::new("proof_artifact", "Verus proof module token stream"),
            MetadataEntry::new(
                "configuration_channel",
                "CLI arguments and VERUS_* environment variables",
            ),
            MetadataEntry::new(
                "configuration_surface",
                "binary path, source selection, flags, timeout, and report output",
            ),
        ]
    }
}

impl Verifier for VerusVerifier {
    type Metadata = VerusVerifierMetadata;

    fn name() -> &'static str {
        "verus"
    }
}

/// Leaf proof emitter for a verifier backend.
pub trait WitnessSource<V: Verifier> {
    /// Backend-facing proof artifact emitted by this source.
    type ProofArtifact;

    /// Emit the verifier-facing proof artifact for this backend.
    fn proof() -> Self::ProofArtifact;
}

/// Constitutional extraction of verifier-facing proof emission.
///
/// A witness consumes an evidence stack and emits the verifier-facing proof
/// artifact that backend consumes.
pub trait Witness<V: Verifier> {
    /// Evidence stack used to justify this verifier-facing proof surface.
    type SupportingEvidence: Evidence;

    /// Backend-facing proof artifact emitted for this verifier.
    type ProofArtifact;

    /// Emit the verifier-facing proof artifact for this backend.
    fn proof() -> Self::ProofArtifact;

    /// Concise description of the evidence lineage behind this proof.
    fn lineage_summary() -> &'static str {
        <Self::SupportingEvidence as Evidence>::lineage_summary()
    }

    /// Code-level audit surface responsible for upholding this proof.
    fn audit_surface() -> &'static [&'static str] {
        <Self::SupportingEvidence as Evidence>::audit_surface()
    }
}

/// Alias over the builtin verifier trio carried by the constitutional surface.
pub trait Witnessed:
    Witness<KaniVerifier> + Witness<CreusotVerifier> + Witness<VerusVerifier>
{
}

impl<T> Witnessed for T where
    T: Witness<KaniVerifier> + Witness<CreusotVerifier> + Witness<VerusVerifier>
{
}

/// Explicit refinement of a type into a standard-root role.
///
/// This wrapper is purely type-level. It does not carry a value. The point is
/// to make the root-role promotion explicit in the codebase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AsStandard<P> {
    _marker: PhantomData<fn() -> P>,
}

impl<P> AsStandard<P> {
    /// Promote type `P` into the standard-root role.
    pub const fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<P> From<P> for AsStandard<P> {
    fn from(_value: P) -> Self {
        Self::new()
    }
}

/// Explicit refinement of a type into an objective-root role.
///
/// This wrapper is purely type-level. It does not carry a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AsObjective<P> {
    _marker: PhantomData<fn() -> P>,
}

impl<P> AsObjective<P> {
    /// Promote type `P` into the objective-root role.
    pub const fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<P> From<P> for AsObjective<P> {
    fn from(_value: P) -> Self {
        Self::new()
    }
}

/// One provenance metadata fact expressed as a key-value pair.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetadataEntry {
    /// Stable metadata key.
    key: &'static str,
    /// Stable metadata value.
    value: &'static str,
}

impl MetadataEntry {
    /// Create a new provenance metadata entry.
    pub const fn new(key: &'static str, value: &'static str) -> Self {
        Self { key, value }
    }

    /// Return the metadata key.
    pub const fn key(&self) -> &'static str {
        self.key
    }

    /// Return the metadata value.
    pub const fn value(&self) -> &'static str {
        self.value
    }
}

/// Structured provenance describing how a claim is sourced and audited.
pub trait Provenance {
    /// Return the provenance metadata describing this claim's source of trust.
    fn metadata() -> Vec<MetadataEntry>;
}

/// A proposition with explicit audit lineage.
///
/// Root propositions may implement this directly. Derived propositions should
/// use it to disclose the upstream roots and code surface that justify them.
pub trait Evidence {
    /// Concrete witness carrier referenced for downstream proof reporting.
    type WitnessType: WitnessSource<KaniVerifier>
        + WitnessSource<CreusotVerifier>
        + WitnessSource<VerusVerifier>;

    /// Concise description of the evidence lineage behind this claim.
    fn lineage_summary() -> &'static str;

    /// Code-level audit surface responsible for upholding the claim.
    fn audit_surface() -> &'static [&'static str];

    /// Human-readable identifier for the witness carrier.
    fn witness_type_name() -> &'static str {
        std::any::type_name::<Self::WitnessType>()
    }
}

impl<E> Witness<KaniVerifier> for E
where
    E: Evidence,
{
    type SupportingEvidence = E;
    type ProofArtifact = <E::WitnessType as WitnessSource<KaniVerifier>>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <E::WitnessType as WitnessSource<KaniVerifier>>::proof()
    }
}

impl<E> Witness<CreusotVerifier> for E
where
    E: Evidence,
{
    type SupportingEvidence = E;
    type ProofArtifact = <E::WitnessType as WitnessSource<CreusotVerifier>>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <E::WitnessType as WitnessSource<CreusotVerifier>>::proof()
    }
}

impl<E> Witness<VerusVerifier> for E
where
    E: Evidence,
{
    type SupportingEvidence = E;
    type ProofArtifact = <E::WitnessType as WitnessSource<VerusVerifier>>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <E::WitnessType as WitnessSource<VerusVerifier>>::proof()
    }
}

/// A proposition rooted in an external normative authority.
pub trait Standard: Evidence + Provenance {
    /// The authorizing body for the standard.
    fn authorizing_body() -> &'static str;

    /// A canonical link or citation for the authoritative source text.
    fn authoritative_source() -> &'static str;

    /// The clause, section, or scope represented by this proposition.
    fn source_scope() -> &'static str;

    /// Concise summary of the normative language being encoded.
    fn normative_summary() -> &'static str;

    /// Why the code-level proposition faithfully represents the source text.
    fn fidelity_rationale() -> &'static str;
}

/// A proposition rooted in architectural design authority.
pub trait Objective: Evidence + Provenance {
    /// The design authority, owner, or originating author.
    fn design_authority() -> &'static str;

    /// Where this objective fits within the larger architecture.
    fn architectural_context() -> &'static str;

    /// The intended guarantee or invariant being claimed.
    fn intended_invariant() -> &'static str;

    /// Why this objective is necessary in the design.
    fn rationale() -> &'static str;
}

/// Opaque proof token carried through lawful exchanges.
pub trait ProofToken {
    /// Evidence proposition justified by this token.
    type Proposition: Evidence;
}

/// Canonical pairing between a payload and an explicit proof sidecar.
pub trait Sidecar {
    /// The primary payload carried through an exchange.
    type Primary: Evidence;

    /// The proposition carried as the proof sidecar.
    type Proposition: Evidence;

    /// Concrete proof token sidecar.
    type SidecarToken: ProofToken<Proposition = Self::Proposition>;

    /// Borrow the primary payload.
    fn primary(&self) -> &Self::Primary;

    /// Copy out the proof sidecar token.
    fn sidecar(&self) -> Self::SidecarToken;
}

/// Constitutional alias for lawful proof minting from an existing credential.
pub trait Establish<C>: Evidence + Sized {
    /// Concrete proof token minted for this evidence.
    type Token: ProofToken<Proposition = Self>;

    /// Mint a proof token from a lawful credential.
    #[track_caller]
    fn establish(credential: &C) -> Self::Token;
}

/// Lawful proof-bearing exchange from one sidecar state to another.
pub trait Exchange<Input, Output> {
    /// Proposition required before the exchange may proceed.
    type Precondition: Evidence;

    /// Proposition established by a successful exchange.
    type Postcondition: Evidence;

    /// Concrete proof token required for the precondition.
    type PreconditionToken: ProofToken<Proposition = Self::Precondition>;

    /// Concrete proof token minted for the postcondition.
    type PostconditionToken: ProofToken<Proposition = Self::Postcondition>;

    /// Error surface for failed exchanges.
    type Error;

    /// Perform the exchange, consuming the input sidecar and minting a new one.
    fn exchange(
        &self,
        input: Input,
        proof: Self::PreconditionToken,
    ) -> Result<(Output, Self::PostconditionToken), Self::Error>;
}

/// Finite-state-machine surface for a closed proof-bearing system.
pub trait StateMachine {
    /// State carrier for the machine.
    type State;

    /// Invariant preserved across lawful transitions.
    type Invariant;
}

/// A closed world of lawful proof exchanges.
pub trait Amenable: StateMachine {
    /// Verifier-facing proof surface type for this closed proof system.
    type ProofSurface;

    /// Human-readable identifier for the governing invariant type.
    fn invariant_name() -> &'static str {
        std::any::type_name::<Self::Invariant>()
    }

    /// Emit the Kani proof surface for the closed proof system.
    fn kani_surface() -> Self::ProofSurface;

    /// Emit the Creusot proof surface for the closed proof system.
    fn creusot_surface() -> Self::ProofSurface;

    /// Emit the Verus proof surface for the closed proof system.
    fn verus_surface() -> Self::ProofSurface;

    /// Code-level audit surface for the closed proof system.
    fn audit_surface() -> &'static [&'static str];
}

/// Provenance helper for Rust standard-library-backed carriers.
///
/// This trait names the authoritative Rust documentation surface and semantic
/// summary for concrete std or core types used as trusted roots.
pub trait RustStdType: 'static {
    /// The Rust crate that normatively defines the type.
    fn rust_source_crate() -> &'static str;

    /// The Rust module path that normatively defines the type.
    fn rust_source_module() -> &'static str;

    /// The canonical documentation URL for the type.
    fn rust_doc_url() -> &'static str;

    /// Concise summary of the semantic promise made by the standard library.
    fn rust_semantics_summary() -> &'static str;
}
