//! Shadows for reqwest TLS helper types.

use elicitation::{ElicitComplete, Elicit, Prompt, elicit_newtype, emit_code::ToCodeLiteral,
    proc_macro2::TokenStream};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::Error;

// ── Certificate ───────────────────────────────────────────────────────────────

elicit_newtype!(elicitation::ReqwestCertificate, as Certificate, serde);

impl ElicitComplete for Certificate {}

impl ToCodeLiteral for Certificate {
    #[tracing::instrument(skip(self), level = "trace")]
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
    #[tracing::instrument(skip(buf), level = "debug")]
    pub fn from_der(buf: &[u8]) -> Result<Self, Error> {
        Ok(Self::from(elicitation::ReqwestCertificate::from_der(buf)?))
    }

    /// Construct a certificate from PEM-encoded bytes.
    #[tracing::instrument(skip(buf), level = "debug")]
    pub fn from_pem(buf: &[u8]) -> Result<Self, Error> {
        Ok(Self::from(elicitation::ReqwestCertificate::from_pem(buf)?))
    }

    /// Rebuild a raw `reqwest::Certificate`.
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn build_raw(&self) -> Result<reqwest::Certificate, Error> {
        Ok(self.0.build_raw()?)
    }
}

// ── Identity ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema, Elicit)]
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
    #[tracing::instrument(skip(self, f), level = "trace")]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Identity")
            .field("recipe", &self.recipe)
            .finish_non_exhaustive()
    }
}

impl Serialize for Identity {
    #[tracing::instrument(skip(self, serializer), level = "trace")]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.recipe.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Identity {
    #[tracing::instrument(skip(deserializer), level = "trace")]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            raw: None,
            recipe: IdentityRecipe::deserialize(deserializer)?,
        })
    }
}

impl schemars::JsonSchema for Identity {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("Identity")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        IdentityRecipe::json_schema(generator)
    }
}

impl elicitation::Prompt for Identity {
    fn prompt() -> Option<&'static str> {
        Some("Describe a TLS client identity snapshot:")
    }
}

impl elicitation::Elicitation for Identity {
    type Style = ();

    #[tracing::instrument(skip(communicator), level = "debug")]
    async fn elicit<C: elicitation::ElicitCommunicator>(
        communicator: &C,
    ) -> elicitation::ElicitResult<Self> {
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

impl elicitation::ElicitIntrospect for Identity {
    fn pattern() -> elicitation::ElicitationPattern {
        IdentityRecipe::pattern()
    }

    fn metadata() -> elicitation::TypeMetadata {
        elicitation::TypeMetadata {
            type_name: "Identity",
            description: Self::prompt(),
            details: IdentityRecipe::metadata().details,
        }
    }
}

impl elicitation::ElicitPromptTree for Identity {
    fn prompt_tree() -> elicitation::PromptTree {
        match IdentityRecipe::prompt_tree() {
            elicitation::PromptTree::Survey { fields, .. } => elicitation::PromptTree::Survey {
                prompt: Self::prompt().map(str::to_string),
                type_name: "Identity".to_string(),
                fields,
            },
            tree => tree.with_prompt(Self::prompt().map(str::to_string)),
        }
    }
}

impl elicitation::ElicitSpec for Identity {
    fn type_spec() -> elicitation::TypeSpec {
        let base = IdentityRecipe::type_spec();
        elicitation::TypeSpec::new(
            "Identity",
            "TLS client identity snapshot.",
            base.categories().clone(),
        )
    }
}

impl ElicitComplete for Identity {}

impl ToCodeLiteral for Identity {
    #[tracing::instrument(skip(self), level = "trace")]
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
    #[tracing::instrument(skip(raw), level = "trace")]
    fn from_raw(raw: reqwest::Identity, recipe: IdentityRecipe) -> Self {
        Self {
            raw: Some(raw),
            recipe,
        }
    }

    /// Construct an identity from PEM-encoded certificate and key bytes.
    #[tracing::instrument(skip(pem), level = "debug")]
    pub fn from_pem(pem: &[u8]) -> Result<Self, Error> {
        let raw = reqwest::Identity::from_pem(pem)?;
        Ok(Self::from_raw(
            raw,
            IdentityRecipe::Pem { pem: pem.to_vec() },
        ))
    }

    /// Construct an identity from a PKCS#12 archive (native-TLS backend).
    #[tracing::instrument(skip(der, password), level = "debug")]
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
    #[tracing::instrument(skip(cert_pem, key_pem), level = "debug")]
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
    #[tracing::instrument(skip(self), level = "debug")]
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

elicit_newtype!(elicitation::ReqwestTlsInfo, as TlsInfo, serde);

impl ElicitComplete for TlsInfo {}

impl ToCodeLiteral for TlsInfo {
    #[tracing::instrument(skip(self), level = "trace")]
    fn to_code_literal(&self) -> TokenStream {
        let inner = self.0.to_code_literal().to_string();
        let fixed =
            inner.replace("::elicitation::ReqwestTlsInfo", "::elicit_reqwest::TlsInfo");
        TokenStream::from_str(&fixed).unwrap_or_else(|_| self.0.to_code_literal())
    }
}

impl TlsInfo {
    /// Wrap a live `reqwest::tls::TlsInfo`.
    #[tracing::instrument(skip(raw), level = "debug")]
    pub fn from_raw(raw: reqwest::tls::TlsInfo) -> Self {
        Self::from(elicitation::ReqwestTlsInfo::from_raw(raw))
    }

    /// Construct a metadata-only snapshot from optional peer-certificate bytes.
    #[tracing::instrument(skip(peer_certificate), level = "debug")]
    pub fn from_peer_certificate(peer_certificate: Option<Vec<u8>>) -> Self {
        Self::from(elicitation::ReqwestTlsInfo::from_peer_certificate(
            peer_certificate,
        ))
    }

    /// Borrow the captured peer-certificate bytes, if present.
    #[tracing::instrument(skip(self), level = "trace")]
    pub fn peer_certificate(&self) -> Option<&[u8]> {
        self.0.peer_certificate()
    }
}

impl From<reqwest::tls::TlsInfo> for TlsInfo {
    #[tracing::instrument(skip(value), level = "debug")]
    fn from(value: reqwest::tls::TlsInfo) -> Self {
        Self::from_raw(value)
    }
}

// ── TlsVersion ────────────────────────────────────────────────────────────────

/// TLS protocol version shadow for `reqwest::tls::Version`.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema, Elicit)]
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
    #[tracing::instrument(skip(self), level = "trace")]
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

elicit_newtype!(
    elicitation::ReqwestCertificateRevocationList,
    as CertificateRevocationList,
    serde
);

impl ElicitComplete for CertificateRevocationList {}

impl ToCodeLiteral for CertificateRevocationList {
    #[tracing::instrument(skip(self), level = "trace")]
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
    #[tracing::instrument(skip(pem), level = "debug")]
    pub fn from_pem(pem: &[u8]) -> Result<Self, Error> {
        Ok(Self::from(
            elicitation::ReqwestCertificateRevocationList::from_pem(pem)?,
        ))
    }

    /// Parse a PEM bundle and return one CRL per entry.
    #[tracing::instrument(skip(pem_bundle), level = "debug")]
    pub fn from_pem_bundle(pem_bundle: &[u8]) -> Result<Vec<Self>, Error> {
        Ok(
            elicitation::ReqwestCertificateRevocationList::from_pem_bundle(pem_bundle)?
                .into_iter()
                .map(Self::from)
                .collect(),
        )
    }

    /// Rebuild a raw `reqwest::tls::CertificateRevocationList`.
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn build_raw(&self) -> Result<reqwest::tls::CertificateRevocationList, Error> {
        Ok(self.0.build_raw()?)
    }
}
