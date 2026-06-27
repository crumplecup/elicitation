//! Shadows for reqwest TLS helper types.

use std::borrow::Cow;

use elicitation::{
    Elicit, ElicitCommunicator, ElicitComplete, ElicitIntrospect, ElicitPromptTree, ElicitResult,
    ElicitSpec, Elicitation, Prompt, PromptTree, TypeMetadata, TypeSpec, emit_code::ToCodeLiteral,
    proc_macro2::TokenStream,
};
use schemars::{JsonSchema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tracing::instrument;

use crate::Error;

// ── Certificate ───────────────────────────────────────────────────────────────

/// Owned server-certificate shadow; delegates to [`elicitation::ReqwestCertificate`].
#[derive(Clone)]
pub struct Certificate(elicitation::ReqwestCertificate);

impl std::fmt::Debug for Certificate {
    #[instrument(skip(self, f), level = "trace")]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Certificate").finish_non_exhaustive()
    }
}

impl Serialize for Certificate {
    #[instrument(skip(self, serializer), level = "trace")]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Certificate {
    #[instrument(skip(deserializer), level = "trace")]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self(elicitation::ReqwestCertificate::deserialize(
            deserializer,
        )?))
    }
}

impl JsonSchema for Certificate {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("Certificate")
    }

    fn json_schema(generator: &mut SchemaGenerator) -> schemars::Schema {
        elicitation::ReqwestCertificate::json_schema(generator)
    }
}

impl Prompt for Certificate {
    fn prompt() -> Option<&'static str> {
        Some("Describe a TLS server certificate snapshot:")
    }
}

impl Elicitation for Certificate {
    type Style = ();

    #[instrument(skip(communicator), level = "debug")]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        elicitation::ReqwestCertificate::elicit(communicator)
            .await
            .map(Self)
    }

    fn kani_proof() -> TokenStream {
        elicitation::ReqwestCertificate::kani_proof()
    }

    fn verus_proof() -> TokenStream {
        elicitation::ReqwestCertificate::verus_proof()
    }

    fn creusot_proof() -> TokenStream {
        elicitation::ReqwestCertificate::creusot_proof()
    }
}

impl ElicitIntrospect for Certificate {
    fn pattern() -> elicitation::ElicitationPattern {
        elicitation::ReqwestCertificate::pattern()
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "Certificate",
            description: Self::prompt(),
            details: elicitation::ReqwestCertificate::metadata().details,
        }
    }
}

impl ElicitPromptTree for Certificate {
    fn prompt_tree() -> PromptTree {
        match elicitation::ReqwestCertificate::prompt_tree() {
            PromptTree::Survey { fields, .. } => PromptTree::Survey {
                prompt: Self::prompt().map(str::to_string),
                type_name: "Certificate".to_string(),
                fields,
            },
            tree => tree.with_prompt(Self::prompt().map(str::to_string)),
        }
    }
}

impl ElicitSpec for Certificate {
    fn type_spec() -> TypeSpec {
        let base = elicitation::ReqwestCertificate::type_spec();
        TypeSpec::new(
            "Certificate",
            "TLS server certificate snapshot.",
            base.categories().clone(),
        )
    }
}

impl ElicitComplete for Certificate {}

impl ToCodeLiteral for Certificate {
    #[instrument(skip(self), level = "trace")]
    fn to_code_literal(&self) -> TokenStream {
        let inner = self.0.to_code_literal().to_string();
        let fixed = inner.replace(
            "::elicitation::ReqwestCertificate",
            "::elicit_reqwest::Certificate",
        );
        TokenStream::from_str(&fixed).unwrap_or_else(|_| self.0.to_code_literal())
    }
}

impl Certificate {
    /// Construct a certificate from DER-encoded bytes.
    #[instrument(skip(buf), level = "debug")]
    pub fn from_der(buf: &[u8]) -> Result<Self, Error> {
        Ok(Self(elicitation::ReqwestCertificate::from_der(buf)?))
    }

    /// Construct a certificate from PEM-encoded bytes.
    #[instrument(skip(buf), level = "debug")]
    pub fn from_pem(buf: &[u8]) -> Result<Self, Error> {
        Ok(Self(elicitation::ReqwestCertificate::from_pem(buf)?))
    }

    /// Rebuild a raw `reqwest::Certificate`.
    #[instrument(skip(self), level = "debug")]
    pub fn build_raw(&self) -> Result<reqwest::Certificate, Error> {
        Ok(self.0.build_raw()?)
    }
}

// ── Identity ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe the TLS identity encoding:")]
enum IdentityRecipe {
    Pem {
        #[prompt("PEM-encoded certificate/key bytes:")]
        pem: Vec<u8>,
    },
    Pkcs12 {
        #[prompt("PKCS#12 archive bytes:")]
        archive: Vec<u8>,
        #[prompt("Archive password:")]
        password: String,
    },
    Pkcs8 {
        #[prompt("PEM-encoded certificate chain bytes:")]
        cert_pem: Vec<u8>,
        #[prompt("PEM-encoded private key bytes:")]
        key_pem: Vec<u8>,
    },
}

/// Owned client-identity shadow.
#[derive(Clone)]
pub struct Identity {
    raw: Option<reqwest::Identity>,
    recipe: IdentityRecipe,
}

impl std::fmt::Debug for Identity {
    #[instrument(skip(self, f), level = "trace")]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Identity")
            .field("recipe", &self.recipe)
            .finish_non_exhaustive()
    }
}

impl Serialize for Identity {
    #[instrument(skip(self, serializer), level = "trace")]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.recipe.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Identity {
    #[instrument(skip(deserializer), level = "trace")]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            raw: None,
            recipe: IdentityRecipe::deserialize(deserializer)?,
        })
    }
}

impl JsonSchema for Identity {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("Identity")
    }

    fn json_schema(generator: &mut SchemaGenerator) -> schemars::Schema {
        IdentityRecipe::json_schema(generator)
    }
}

impl Prompt for Identity {
    fn prompt() -> Option<&'static str> {
        Some("Describe a TLS client identity snapshot:")
    }
}

impl Elicitation for Identity {
    type Style = ();

    #[instrument(skip(communicator), level = "debug")]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let recipe = IdentityRecipe::elicit(communicator).await?;
        Ok(Self { raw: None, recipe })
    }

    fn kani_proof() -> TokenStream {
        IdentityRecipe::kani_proof()
    }

    fn verus_proof() -> TokenStream {
        IdentityRecipe::verus_proof()
    }

    fn creusot_proof() -> TokenStream {
        IdentityRecipe::creusot_proof()
    }
}

impl ElicitIntrospect for Identity {
    fn pattern() -> elicitation::ElicitationPattern {
        IdentityRecipe::pattern()
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "Identity",
            description: Self::prompt(),
            details: IdentityRecipe::metadata().details,
        }
    }
}

impl ElicitPromptTree for Identity {
    fn prompt_tree() -> PromptTree {
        match IdentityRecipe::prompt_tree() {
            PromptTree::Survey { fields, .. } => PromptTree::Survey {
                prompt: Self::prompt().map(str::to_string),
                type_name: "Identity".to_string(),
                fields,
            },
            tree => tree.with_prompt(Self::prompt().map(str::to_string)),
        }
    }
}

impl ElicitSpec for Identity {
    fn type_spec() -> TypeSpec {
        let base = IdentityRecipe::type_spec();
        TypeSpec::new(
            "Identity",
            "TLS client identity snapshot.",
            base.categories().clone(),
        )
    }
}

impl ElicitComplete for Identity {}

impl ToCodeLiteral for Identity {
    #[instrument(skip(self), level = "trace")]
    fn to_code_literal(&self) -> TokenStream {
        match &self.recipe {
            IdentityRecipe::Pem { pem } => {
                let bytes: Vec<_> = pem.to_vec();
                quote::quote! {
                    match ::elicit_reqwest::Identity::from_pem(&[#(#bytes),*]) {
                        ::std::result::Result::Ok(v) => v,
                        ::std::result::Result::Err(error) => {
                            return ::std::result::Result::Err(error.into());
                        }
                    }
                }
            }
            IdentityRecipe::Pkcs12 { archive, password } => {
                let bytes: Vec<_> = archive.to_vec();
                quote::quote! {
                    match ::elicit_reqwest::Identity::from_pkcs12_der(&[#(#bytes),*], #password) {
                        ::std::result::Result::Ok(v) => v,
                        ::std::result::Result::Err(error) => {
                            return ::std::result::Result::Err(error.into());
                        }
                    }
                }
            }
            IdentityRecipe::Pkcs8 { cert_pem, key_pem } => {
                let cert_bytes: Vec<_> = cert_pem.to_vec();
                let key_bytes: Vec<_> = key_pem.to_vec();
                quote::quote! {
                    match ::elicit_reqwest::Identity::from_pkcs8_pem(
                        &[#(#cert_bytes),*],
                        &[#(#key_bytes),*],
                    ) {
                        ::std::result::Result::Ok(v) => v,
                        ::std::result::Result::Err(error) => {
                            return ::std::result::Result::Err(error.into());
                        }
                    }
                }
            }
        }
    }
}

impl Identity {
    #[instrument(skip(raw), level = "trace")]
    fn from_raw(raw: reqwest::Identity, recipe: IdentityRecipe) -> Self {
        Self {
            raw: Some(raw),
            recipe,
        }
    }

    /// Construct an identity from PEM-encoded certificate and key bytes.
    #[instrument(skip(pem), level = "debug")]
    pub fn from_pem(pem: &[u8]) -> Result<Self, Error> {
        let raw = reqwest::Identity::from_pem(pem)?;
        Ok(Self::from_raw(
            raw,
            IdentityRecipe::Pem { pem: pem.to_vec() },
        ))
    }

    /// Construct an identity from a PKCS#12 archive (native-TLS backend).
    #[instrument(skip(der, password), level = "debug")]
    pub fn from_pkcs12_der(der: &[u8], password: &str) -> Result<Self, Error> {
        let raw = reqwest::Identity::from_pkcs12_der(der, password)?;
        Ok(Self::from_raw(
            raw,
            IdentityRecipe::Pkcs12 {
                archive: der.to_vec(),
                password: password.to_string(),
            },
        ))
    }

    /// Construct an identity from PEM certificate chain and key (native-TLS backend).
    #[instrument(skip(cert_pem, key_pem), level = "debug")]
    pub fn from_pkcs8_pem(cert_pem: &[u8], key_pem: &[u8]) -> Result<Self, Error> {
        let raw = reqwest::Identity::from_pkcs8_pem(cert_pem, key_pem)?;
        Ok(Self::from_raw(
            raw,
            IdentityRecipe::Pkcs8 {
                cert_pem: cert_pem.to_vec(),
                key_pem: key_pem.to_vec(),
            },
        ))
    }

    /// Rebuild a raw `reqwest::Identity`.
    #[instrument(skip(self), level = "debug")]
    pub fn build_raw(&self) -> Result<reqwest::Identity, Error> {
        if let Some(raw) = &self.raw {
            return Ok(raw.clone());
        }
        Ok(match &self.recipe {
            IdentityRecipe::Pem { pem } => reqwest::Identity::from_pem(pem)?,
            IdentityRecipe::Pkcs12 { archive, password } => {
                reqwest::Identity::from_pkcs12_der(archive, password)?
            }
            IdentityRecipe::Pkcs8 { cert_pem, key_pem } => {
                reqwest::Identity::from_pkcs8_pem(cert_pem, key_pem)?
            }
        })
    }
}

// ── TlsInfo ───────────────────────────────────────────────────────────────────

/// Owned TLS connection metadata shadow; delegates to [`elicitation::ReqwestTlsInfo`].
#[derive(Clone)]
pub struct TlsInfo(elicitation::ReqwestTlsInfo);

impl std::fmt::Debug for TlsInfo {
    #[instrument(skip(self, f), level = "trace")]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TlsInfo").finish_non_exhaustive()
    }
}

impl Serialize for TlsInfo {
    #[instrument(skip(self, serializer), level = "trace")]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TlsInfo {
    #[instrument(skip(deserializer), level = "trace")]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self(elicitation::ReqwestTlsInfo::deserialize(
            deserializer,
        )?))
    }
}

impl JsonSchema for TlsInfo {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("TlsInfo")
    }

    fn json_schema(generator: &mut SchemaGenerator) -> schemars::Schema {
        elicitation::ReqwestTlsInfo::json_schema(generator)
    }
}

impl Prompt for TlsInfo {
    fn prompt() -> Option<&'static str> {
        Some("Describe TLS connection metadata:")
    }
}

impl Elicitation for TlsInfo {
    type Style = ();

    #[instrument(skip(communicator), level = "debug")]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        elicitation::ReqwestTlsInfo::elicit(communicator)
            .await
            .map(Self)
    }

    fn kani_proof() -> TokenStream {
        elicitation::ReqwestTlsInfo::kani_proof()
    }

    fn verus_proof() -> TokenStream {
        elicitation::ReqwestTlsInfo::verus_proof()
    }

    fn creusot_proof() -> TokenStream {
        elicitation::ReqwestTlsInfo::creusot_proof()
    }
}

impl ElicitIntrospect for TlsInfo {
    fn pattern() -> elicitation::ElicitationPattern {
        elicitation::ReqwestTlsInfo::pattern()
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "TlsInfo",
            description: Self::prompt(),
            details: elicitation::ReqwestTlsInfo::metadata().details,
        }
    }
}

impl ElicitPromptTree for TlsInfo {
    fn prompt_tree() -> PromptTree {
        match elicitation::ReqwestTlsInfo::prompt_tree() {
            PromptTree::Survey { fields, .. } => PromptTree::Survey {
                prompt: Self::prompt().map(str::to_string),
                type_name: "TlsInfo".to_string(),
                fields,
            },
            tree => tree.with_prompt(Self::prompt().map(str::to_string)),
        }
    }
}

impl ElicitSpec for TlsInfo {
    fn type_spec() -> TypeSpec {
        let base = elicitation::ReqwestTlsInfo::type_spec();
        TypeSpec::new(
            "TlsInfo",
            "TLS connection metadata snapshot.",
            base.categories().clone(),
        )
    }
}

impl ElicitComplete for TlsInfo {}

impl ToCodeLiteral for TlsInfo {
    #[instrument(skip(self), level = "trace")]
    fn to_code_literal(&self) -> TokenStream {
        let inner = self.0.to_code_literal().to_string();
        let fixed = inner.replace("::elicitation::ReqwestTlsInfo", "::elicit_reqwest::TlsInfo");
        TokenStream::from_str(&fixed).unwrap_or_else(|_| self.0.to_code_literal())
    }
}

impl TlsInfo {
    /// Wrap a live `reqwest::tls::TlsInfo`.
    #[instrument(skip(raw), level = "debug")]
    pub fn from_raw(raw: reqwest::tls::TlsInfo) -> Self {
        Self(elicitation::ReqwestTlsInfo::from_raw(raw))
    }

    /// Construct a metadata-only snapshot from optional peer-certificate bytes.
    #[instrument(skip(peer_certificate), level = "debug")]
    pub fn from_peer_certificate(peer_certificate: Option<Vec<u8>>) -> Self {
        Self(elicitation::ReqwestTlsInfo::from_peer_certificate(
            peer_certificate,
        ))
    }

    /// Borrow the captured peer-certificate bytes, if present.
    #[instrument(skip(self), level = "trace")]
    pub fn peer_certificate(&self) -> Option<&[u8]> {
        self.0.peer_certificate()
    }
}

impl From<reqwest::tls::TlsInfo> for TlsInfo {
    #[instrument(skip(value), level = "debug")]
    fn from(value: reqwest::tls::TlsInfo) -> Self {
        Self::from_raw(value)
    }
}

// ── TlsVersion ────────────────────────────────────────────────────────────────

/// TLS protocol version shadow for `reqwest::tls::Version`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("TLS protocol version:")]
pub enum TlsVersion {
    /// TLS 1.0 (legacy; avoid in new configurations).
    #[serde(rename = "TLS_1_0")]
    Tls10,
    /// TLS 1.1 (legacy; avoid in new configurations).
    #[serde(rename = "TLS_1_1")]
    Tls11,
    /// TLS 1.2.
    #[serde(rename = "TLS_1_2")]
    Tls12,
    /// TLS 1.3.
    #[serde(rename = "TLS_1_3")]
    Tls13,
}

impl TlsVersion {
    /// Rebuild a raw `reqwest::tls::Version`.
    #[instrument(skip(self), level = "trace")]
    pub fn build_raw(&self) -> reqwest::tls::Version {
        match self {
            Self::Tls10 => reqwest::tls::Version::TLS_1_0,
            Self::Tls11 => reqwest::tls::Version::TLS_1_1,
            Self::Tls12 => reqwest::tls::Version::TLS_1_2,
            Self::Tls13 => reqwest::tls::Version::TLS_1_3,
        }
    }
}

impl From<reqwest::tls::Version> for TlsVersion {
    fn from(v: reqwest::tls::Version) -> Self {
        match v {
            reqwest::tls::Version::TLS_1_0 => Self::Tls10,
            reqwest::tls::Version::TLS_1_1 => Self::Tls11,
            reqwest::tls::Version::TLS_1_2 => Self::Tls12,
            reqwest::tls::Version::TLS_1_3 => Self::Tls13,
            _ => Self::Tls12,
        }
    }
}

impl From<TlsVersion> for reqwest::tls::Version {
    fn from(v: TlsVersion) -> Self {
        v.build_raw()
    }
}

// ── CertificateRevocationList ─────────────────────────────────────────────────

/// CRL shadow; delegates to [`elicitation::ReqwestCertificateRevocationList`].
#[derive(Clone)]
pub struct CertificateRevocationList(elicitation::ReqwestCertificateRevocationList);

impl std::fmt::Debug for CertificateRevocationList {
    #[instrument(skip(self, f), level = "trace")]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CertificateRevocationList")
            .finish_non_exhaustive()
    }
}

impl Serialize for CertificateRevocationList {
    #[instrument(skip(self, serializer), level = "trace")]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CertificateRevocationList {
    #[instrument(skip(deserializer), level = "trace")]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self(
            elicitation::ReqwestCertificateRevocationList::deserialize(deserializer)?,
        ))
    }
}

impl JsonSchema for CertificateRevocationList {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("CertificateRevocationList")
    }

    fn json_schema(generator: &mut SchemaGenerator) -> schemars::Schema {
        elicitation::ReqwestCertificateRevocationList::json_schema(generator)
    }
}

impl Prompt for CertificateRevocationList {
    fn prompt() -> Option<&'static str> {
        Some("Describe a certificate revocation list (CRL) snapshot:")
    }
}

impl Elicitation for CertificateRevocationList {
    type Style = ();

    #[instrument(skip(communicator), level = "debug")]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        elicitation::ReqwestCertificateRevocationList::elicit(communicator)
            .await
            .map(Self)
    }

    fn kani_proof() -> TokenStream {
        elicitation::ReqwestCertificateRevocationList::kani_proof()
    }

    fn verus_proof() -> TokenStream {
        elicitation::ReqwestCertificateRevocationList::verus_proof()
    }

    fn creusot_proof() -> TokenStream {
        elicitation::ReqwestCertificateRevocationList::creusot_proof()
    }
}

impl ElicitIntrospect for CertificateRevocationList {
    fn pattern() -> elicitation::ElicitationPattern {
        elicitation::ReqwestCertificateRevocationList::pattern()
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "CertificateRevocationList",
            description: Self::prompt(),
            details: elicitation::ReqwestCertificateRevocationList::metadata().details,
        }
    }
}

impl ElicitPromptTree for CertificateRevocationList {
    fn prompt_tree() -> PromptTree {
        match elicitation::ReqwestCertificateRevocationList::prompt_tree() {
            PromptTree::Survey { fields, .. } => PromptTree::Survey {
                prompt: Self::prompt().map(str::to_string),
                type_name: "CertificateRevocationList".to_string(),
                fields,
            },
            tree => tree.with_prompt(Self::prompt().map(str::to_string)),
        }
    }
}

impl ElicitSpec for CertificateRevocationList {
    fn type_spec() -> TypeSpec {
        let base = elicitation::ReqwestCertificateRevocationList::type_spec();
        TypeSpec::new(
            "CertificateRevocationList",
            "Certificate revocation list snapshot.",
            base.categories().clone(),
        )
    }
}

impl ElicitComplete for CertificateRevocationList {}

impl ToCodeLiteral for CertificateRevocationList {
    #[instrument(skip(self), level = "trace")]
    fn to_code_literal(&self) -> TokenStream {
        let inner = self.0.to_code_literal().to_string();
        let fixed = inner.replace(
            "::elicitation::ReqwestCertificateRevocationList",
            "::elicit_reqwest::CertificateRevocationList",
        );
        TokenStream::from_str(&fixed).unwrap_or_else(|_| self.0.to_code_literal())
    }
}

impl CertificateRevocationList {
    /// Construct a CRL from a single PEM-encoded CRL.
    #[instrument(skip(pem), level = "debug")]
    pub fn from_pem(pem: &[u8]) -> Result<Self, Error> {
        Ok(Self(
            elicitation::ReqwestCertificateRevocationList::from_pem(pem)?,
        ))
    }

    /// Parse a PEM bundle and return one CRL per entry.
    #[instrument(skip(pem_bundle), level = "debug")]
    pub fn from_pem_bundle(pem_bundle: &[u8]) -> Result<Vec<Self>, Error> {
        Ok(
            elicitation::ReqwestCertificateRevocationList::from_pem_bundle(pem_bundle)?
                .into_iter()
                .map(Self)
                .collect(),
        )
    }

    /// Rebuild a raw `reqwest::tls::CertificateRevocationList`.
    #[instrument(skip(self), level = "debug")]
    pub fn build_raw(&self) -> Result<reqwest::tls::CertificateRevocationList, Error> {
        Ok(self.0.build_raw()?)
    }
}
