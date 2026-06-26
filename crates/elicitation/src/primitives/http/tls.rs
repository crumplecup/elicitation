//! Trenchcoats for `reqwest` TLS helper types.

use std::borrow::Cow;

use crate::{
    ElicitCommunicator, ElicitComplete, ElicitIntrospect, ElicitPromptTree, ElicitResult,
    ElicitSpec, Elicitation, Prompt, PromptTree, SpecCategory, SpecEntry, TypeMetadata, TypeSpec,
    emit_code::ToCodeLiteral, type_spec::TypeSpecInventoryKey,
};
use proc_macro2::TokenStream;
use schemars::{JsonSchema, SchemaGenerator};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, crate::Elicit)]
#[prompt("Describe how a reqwest TLS certificate was encoded:")]
enum ReqwestCertificateEncoding {
    Der,
    Pem,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, crate::Elicit)]
#[prompt("Describe a reqwest TLS certificate snapshot:")]
struct ReqwestCertificateSnapshot {
    #[prompt("Certificate encoding:")]
    encoding: ReqwestCertificateEncoding,
    #[prompt("Certificate bytes:")]
    bytes: Vec<u8>,
}

/// Elicitation-aware trenchcoat for `reqwest::Certificate`.
#[derive(Clone)]
pub struct ReqwestCertificate {
    raw: Option<reqwest::Certificate>,
    snapshot: ReqwestCertificateSnapshot,
}

impl std::fmt::Debug for ReqwestCertificate {
    #[tracing::instrument(skip(self, f), level = "trace")]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReqwestCertificate")
            .field("snapshot", &self.snapshot)
            .finish_non_exhaustive()
    }
}

impl Serialize for ReqwestCertificate {
    #[tracing::instrument(skip(self, serializer), level = "trace")]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.snapshot.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ReqwestCertificate {
    #[tracing::instrument(skip(deserializer), level = "trace")]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            raw: None,
            snapshot: ReqwestCertificateSnapshot::deserialize(deserializer)?,
        })
    }
}

impl JsonSchema for ReqwestCertificate {
    #[tracing::instrument(level = "trace")]
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("ReqwestCertificate")
    }

    #[tracing::instrument(skip(generator), level = "trace")]
    fn json_schema(generator: &mut SchemaGenerator) -> schemars::Schema {
        <ReqwestCertificateSnapshot as JsonSchema>::json_schema(generator)
    }
}

impl Prompt for ReqwestCertificate {
    fn prompt() -> Option<&'static str> {
        Some("Describe a reqwest TLS certificate snapshot:")
    }
}

impl Elicitation for ReqwestCertificate {
    type Style = ();

    #[tracing::instrument(skip(communicator), level = "debug")]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let snapshot = ReqwestCertificateSnapshot::elicit(communicator).await?;
        Ok(Self {
            raw: None,
            snapshot,
        })
    }

    fn kani_proof() -> TokenStream {
        ReqwestCertificateSnapshot::kani_proof()
    }

    fn verus_proof() -> TokenStream {
        ReqwestCertificateSnapshot::verus_proof()
    }

    fn creusot_proof() -> TokenStream {
        ReqwestCertificateSnapshot::creusot_proof()
    }
}

impl ElicitIntrospect for ReqwestCertificate {
    fn pattern() -> crate::ElicitationPattern {
        ReqwestCertificateSnapshot::pattern()
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::ReqwestCertificate",
            description: Self::prompt(),
            details: ReqwestCertificateSnapshot::metadata().details,
        }
    }
}

impl ElicitPromptTree for ReqwestCertificate {
    fn prompt_tree() -> PromptTree {
        match ReqwestCertificateSnapshot::prompt_tree() {
            PromptTree::Survey { fields, .. } => PromptTree::Survey {
                prompt: Self::prompt().map(str::to_string),
                type_name: "elicitation::ReqwestCertificate".to_string(),
                fields,
            },
            tree => tree.with_prompt(Self::prompt().map(str::to_string)),
        }
    }
}

impl ElicitSpec for ReqwestCertificate {
    fn type_spec() -> TypeSpec {
        TypeSpec::new(
            "elicitation::ReqwestCertificate",
            "Owned trenchcoat for reqwest::Certificate that preserves the upstream constructor recipe.",
            vec![
                SpecCategory::new(
                    "construction",
                    vec![
                        SpecEntry::new("from_der", "Constructs a certificate from DER bytes.")
                            .with_expression(Some(
                                "::elicitation::ReqwestCertificate::from_der(bytes)".to_string(),
                            )),
                        SpecEntry::new("from_pem", "Constructs a certificate from PEM bytes.")
                            .with_expression(Some(
                                "::elicitation::ReqwestCertificate::from_pem(bytes)".to_string(),
                            )),
                    ],
                ),
                SpecCategory::new(
                    "recovery",
                    vec![SpecEntry::new(
                        "build_raw",
                        "Rebuilds a raw reqwest::Certificate from the recorded constructor recipe.",
                    )
                    .with_expression(Some("certificate.build_raw()".to_string()))],
                ),
            ],
        )
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "elicitation::ReqwestCertificate",
    <ReqwestCertificate as ElicitSpec>::type_spec,
    std::any::TypeId::of::<ReqwestCertificate>
));

impl ToCodeLiteral for ReqwestCertificate {
    fn to_code_literal(&self) -> TokenStream {
        let bytes = self.snapshot.bytes.iter();
        match self.snapshot.encoding {
            ReqwestCertificateEncoding::Der => quote::quote! {
                ::elicitation::ReqwestCertificate::from_der(&[#(#bytes),*])?
            },
            ReqwestCertificateEncoding::Pem => quote::quote! {
                ::elicitation::ReqwestCertificate::from_pem(&[#(#bytes),*])?
            },
        }
    }
}

impl ElicitComplete for ReqwestCertificate {}

impl ReqwestCertificate {
    #[tracing::instrument(skip(raw), level = "debug")]
    fn from_raw(raw: reqwest::Certificate, snapshot: ReqwestCertificateSnapshot) -> Self {
        Self {
            raw: Some(raw),
            snapshot,
        }
    }

    /// Construct a certificate trenchcoat from DER bytes.
    #[tracing::instrument(skip(der), level = "debug")]
    pub fn from_der(der: &[u8]) -> ElicitResult<Self> {
        let raw = reqwest::Certificate::from_der(der)?;
        Ok(Self::from_raw(
            raw,
            ReqwestCertificateSnapshot {
                encoding: ReqwestCertificateEncoding::Der,
                bytes: der.to_vec(),
            },
        ))
    }

    /// Construct a certificate trenchcoat from PEM bytes.
    #[tracing::instrument(skip(pem), level = "debug")]
    pub fn from_pem(pem: &[u8]) -> ElicitResult<Self> {
        let raw = reqwest::Certificate::from_pem(pem)?;
        Ok(Self::from_raw(
            raw,
            ReqwestCertificateSnapshot {
                encoding: ReqwestCertificateEncoding::Pem,
                bytes: pem.to_vec(),
            },
        ))
    }

    /// Rebuild a raw `reqwest::Certificate`.
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn build_raw(&self) -> ElicitResult<reqwest::Certificate> {
        if let Some(raw) = &self.raw {
            return Ok(raw.clone());
        }

        match self.snapshot.encoding {
            ReqwestCertificateEncoding::Der => reqwest::Certificate::from_der(&self.snapshot.bytes)
                .map_err(crate::ElicitError::from),
            ReqwestCertificateEncoding::Pem => reqwest::Certificate::from_pem(&self.snapshot.bytes)
                .map_err(crate::ElicitError::from),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, crate::Elicit)]
#[prompt("Describe a reqwest TLS identity snapshot:")]
struct ReqwestIdentitySnapshot {
    #[prompt("PEM identity bytes:")]
    pem: Vec<u8>,
}

/// Elicitation-aware trenchcoat for `reqwest::Identity`.
#[derive(Clone)]
pub struct ReqwestIdentity {
    raw: Option<reqwest::Identity>,
    snapshot: ReqwestIdentitySnapshot,
}

impl std::fmt::Debug for ReqwestIdentity {
    #[tracing::instrument(skip(self, f), level = "trace")]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReqwestIdentity")
            .field("snapshot", &self.snapshot)
            .finish_non_exhaustive()
    }
}

impl Serialize for ReqwestIdentity {
    #[tracing::instrument(skip(self, serializer), level = "trace")]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.snapshot.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ReqwestIdentity {
    #[tracing::instrument(skip(deserializer), level = "trace")]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            raw: None,
            snapshot: ReqwestIdentitySnapshot::deserialize(deserializer)?,
        })
    }
}

impl JsonSchema for ReqwestIdentity {
    #[tracing::instrument(level = "trace")]
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("ReqwestIdentity")
    }

    #[tracing::instrument(skip(generator), level = "trace")]
    fn json_schema(generator: &mut SchemaGenerator) -> schemars::Schema {
        <ReqwestIdentitySnapshot as JsonSchema>::json_schema(generator)
    }
}

impl Prompt for ReqwestIdentity {
    fn prompt() -> Option<&'static str> {
        Some("Describe a reqwest TLS identity snapshot:")
    }
}

impl Elicitation for ReqwestIdentity {
    type Style = ();

    #[tracing::instrument(skip(communicator), level = "debug")]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let snapshot = ReqwestIdentitySnapshot::elicit(communicator).await?;
        Ok(Self {
            raw: None,
            snapshot,
        })
    }

    fn kani_proof() -> TokenStream {
        ReqwestIdentitySnapshot::kani_proof()
    }

    fn verus_proof() -> TokenStream {
        ReqwestIdentitySnapshot::verus_proof()
    }

    fn creusot_proof() -> TokenStream {
        ReqwestIdentitySnapshot::creusot_proof()
    }
}

impl ElicitIntrospect for ReqwestIdentity {
    fn pattern() -> crate::ElicitationPattern {
        ReqwestIdentitySnapshot::pattern()
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::ReqwestIdentity",
            description: Self::prompt(),
            details: ReqwestIdentitySnapshot::metadata().details,
        }
    }
}

impl ElicitPromptTree for ReqwestIdentity {
    fn prompt_tree() -> PromptTree {
        match ReqwestIdentitySnapshot::prompt_tree() {
            PromptTree::Survey { fields, .. } => PromptTree::Survey {
                prompt: Self::prompt().map(str::to_string),
                type_name: "elicitation::ReqwestIdentity".to_string(),
                fields,
            },
            tree => tree.with_prompt(Self::prompt().map(str::to_string)),
        }
    }
}

impl ElicitSpec for ReqwestIdentity {
    fn type_spec() -> TypeSpec {
        TypeSpec::new(
            "elicitation::ReqwestIdentity",
            "Owned trenchcoat for reqwest::Identity that preserves the upstream constructor recipe.",
            vec![
                SpecCategory::new(
                    "construction",
                    vec![SpecEntry::new(
                        "from_pem",
                        "Constructs a client identity from PEM bytes using the enabled reqwest TLS backend.",
                    )
                    .with_expression(Some(
                        "::elicitation::ReqwestIdentity::from_pem(bytes)".to_string(),
                    ))],
                ),
                SpecCategory::new(
                    "recovery",
                    vec![SpecEntry::new(
                        "build_raw",
                        "Rebuilds a raw reqwest::Identity from the recorded constructor recipe.",
                    )
                    .with_expression(Some("identity.build_raw()".to_string()))],
                ),
            ],
        )
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "elicitation::ReqwestIdentity",
    <ReqwestIdentity as ElicitSpec>::type_spec,
    std::any::TypeId::of::<ReqwestIdentity>
));

impl ToCodeLiteral for ReqwestIdentity {
    fn to_code_literal(&self) -> TokenStream {
        let pem = self.snapshot.pem.iter();
        quote::quote! {
            ::elicitation::ReqwestIdentity::from_pem(&[#(#pem),*])?
        }
    }
}

impl ElicitComplete for ReqwestIdentity {}

impl ReqwestIdentity {
    #[tracing::instrument(skip(raw, snapshot), level = "debug")]
    fn from_raw(raw: reqwest::Identity, snapshot: ReqwestIdentitySnapshot) -> Self {
        Self {
            raw: Some(raw),
            snapshot,
        }
    }

    /// Construct an identity trenchcoat from PEM bytes.
    #[tracing::instrument(skip(pem), level = "debug")]
    pub fn from_pem(pem: &[u8]) -> ElicitResult<Self> {
        let raw = reqwest::Identity::from_pem(pem)?;
        Ok(Self::from_raw(
            raw,
            ReqwestIdentitySnapshot { pem: pem.to_vec() },
        ))
    }

    /// Rebuild a raw `reqwest::Identity`.
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn build_raw(&self) -> ElicitResult<reqwest::Identity> {
        if let Some(raw) = &self.raw {
            return Ok(raw.clone());
        }

        reqwest::Identity::from_pem(&self.snapshot.pem).map_err(crate::ElicitError::from)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, crate::Elicit)]
#[prompt("Describe reqwest TLS connection metadata:")]
struct ReqwestTlsInfoSnapshot {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Peer certificate DER bytes, when present:")]
    peer_certificate: Option<Vec<u8>>,
}

/// Elicitation-aware trenchcoat for `reqwest::tls::TlsInfo`.
#[derive(Clone)]
pub struct ReqwestTlsInfo {
    raw: Option<reqwest::tls::TlsInfo>,
    snapshot: ReqwestTlsInfoSnapshot,
}

impl std::fmt::Debug for ReqwestTlsInfo {
    #[tracing::instrument(skip(self, f), level = "trace")]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReqwestTlsInfo")
            .field("snapshot", &self.snapshot)
            .finish_non_exhaustive()
    }
}

impl Serialize for ReqwestTlsInfo {
    #[tracing::instrument(skip(self, serializer), level = "trace")]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.snapshot.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ReqwestTlsInfo {
    #[tracing::instrument(skip(deserializer), level = "trace")]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            raw: None,
            snapshot: ReqwestTlsInfoSnapshot::deserialize(deserializer)?,
        })
    }
}

impl JsonSchema for ReqwestTlsInfo {
    #[tracing::instrument(level = "trace")]
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("ReqwestTlsInfo")
    }

    #[tracing::instrument(skip(generator), level = "trace")]
    fn json_schema(generator: &mut SchemaGenerator) -> schemars::Schema {
        <ReqwestTlsInfoSnapshot as JsonSchema>::json_schema(generator)
    }
}

impl Prompt for ReqwestTlsInfo {
    fn prompt() -> Option<&'static str> {
        Some("Describe reqwest TLS connection metadata:")
    }
}

impl Elicitation for ReqwestTlsInfo {
    type Style = ();

    #[tracing::instrument(skip(communicator), level = "debug")]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let snapshot = ReqwestTlsInfoSnapshot::elicit(communicator).await?;
        Ok(Self {
            raw: None,
            snapshot,
        })
    }

    fn kani_proof() -> TokenStream {
        ReqwestTlsInfoSnapshot::kani_proof()
    }

    fn verus_proof() -> TokenStream {
        ReqwestTlsInfoSnapshot::verus_proof()
    }

    fn creusot_proof() -> TokenStream {
        ReqwestTlsInfoSnapshot::creusot_proof()
    }
}

impl ElicitIntrospect for ReqwestTlsInfo {
    fn pattern() -> crate::ElicitationPattern {
        ReqwestTlsInfoSnapshot::pattern()
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::ReqwestTlsInfo",
            description: Self::prompt(),
            details: ReqwestTlsInfoSnapshot::metadata().details,
        }
    }
}

impl ElicitPromptTree for ReqwestTlsInfo {
    fn prompt_tree() -> PromptTree {
        match ReqwestTlsInfoSnapshot::prompt_tree() {
            PromptTree::Survey { fields, .. } => PromptTree::Survey {
                prompt: Self::prompt().map(str::to_string),
                type_name: "elicitation::ReqwestTlsInfo".to_string(),
                fields,
            },
            tree => tree.with_prompt(Self::prompt().map(str::to_string)),
        }
    }
}

impl ElicitSpec for ReqwestTlsInfo {
    fn type_spec() -> TypeSpec {
        TypeSpec::new(
            "elicitation::ReqwestTlsInfo",
            "Owned trenchcoat for reqwest::tls::TlsInfo that preserves observable connection metadata.",
            vec![
                SpecCategory::new(
                    "construction",
                    vec![
                        SpecEntry::new(
                            "from_raw",
                            "Wraps a live reqwest::tls::TlsInfo together with a captured peer-certificate snapshot.",
                        )
                        .with_expression(Some(
                            "::elicitation::ReqwestTlsInfo::from_raw(raw)".to_string(),
                        )),
                        SpecEntry::new(
                            "from_peer_certificate",
                            "Constructs a metadata-only snapshot from optional peer-certificate bytes.",
                        )
                        .with_expression(Some(
                            "::elicitation::ReqwestTlsInfo::from_peer_certificate(peer_certificate)"
                                .to_string(),
                        )),
                    ],
                ),
                SpecCategory::new(
                    "recovery",
                    vec![SpecEntry::new(
                        "build_raw",
                        "Returns the live wrapped reqwest::tls::TlsInfo when present. Metadata-only snapshots cannot synthesize a fresh runtime handle.",
                    )
                    .with_expression(Some("tls_info.build_raw()".to_string()))],
                ),
            ],
        )
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "elicitation::ReqwestTlsInfo",
    <ReqwestTlsInfo as ElicitSpec>::type_spec,
    std::any::TypeId::of::<ReqwestTlsInfo>
));

impl ToCodeLiteral for ReqwestTlsInfo {
    fn to_code_literal(&self) -> TokenStream {
        let peer_certificate = self
            .snapshot
            .peer_certificate
            .as_ref()
            .map(|peer_certificate| {
                let bytes = peer_certificate.iter();
                quote::quote! { ::std::option::Option::Some(::std::vec![#(#bytes),*]) }
            })
            .unwrap_or_else(|| quote::quote! { ::std::option::Option::None });

        quote::quote! {
            ::elicitation::ReqwestTlsInfo::from_peer_certificate(#peer_certificate)
        }
    }
}

impl ElicitComplete for ReqwestTlsInfo {}

impl ReqwestTlsInfo {
    /// Wrap a live `reqwest::tls::TlsInfo`.
    #[tracing::instrument(skip(raw), level = "debug")]
    pub fn from_raw(raw: reqwest::tls::TlsInfo) -> Self {
        Self {
            snapshot: ReqwestTlsInfoSnapshot {
                peer_certificate: raw.peer_certificate().map(<[u8]>::to_vec),
            },
            raw: Some(raw),
        }
    }

    /// Construct a metadata-only TLS-info trenchcoat from optional peer-certificate bytes.
    #[tracing::instrument(skip(peer_certificate), level = "debug")]
    pub fn from_peer_certificate(peer_certificate: Option<Vec<u8>>) -> Self {
        Self {
            raw: None,
            snapshot: ReqwestTlsInfoSnapshot { peer_certificate },
        }
    }

    /// Borrow the captured peer-certificate bytes, if present.
    #[tracing::instrument(skip(self), level = "trace")]
    pub fn peer_certificate(&self) -> Option<&[u8]> {
        self.snapshot.peer_certificate.as_deref()
    }

    /// Return the live wrapped `reqwest::tls::TlsInfo` when present.
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn build_raw(&self) -> ElicitResult<reqwest::tls::TlsInfo> {
        self.raw.clone().ok_or_else(|| {
            crate::ElicitError::new(crate::ElicitErrorKind::ParseError(
                "reqwest::tls::TlsInfo cannot be reconstructed from metadata alone; wrap a live runtime handle with ReqwestTlsInfo::from_raw".to_string(),
            ))
        })
    }
}
