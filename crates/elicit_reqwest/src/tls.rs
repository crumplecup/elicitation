//! Shadows for reqwest TLS helper types.

use elicitation::Elicit;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe how a certificate snapshot was encoded:")]
enum CertificateEncoding {
    Der,
    Pem,
}

/// Owned server-certificate snapshot.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe a TLS certificate snapshot:")]
pub struct Certificate {
    #[prompt("Certificate encoding:")]
    encoding: CertificateEncoding,
    #[prompt("Certificate bytes:")]
    bytes: Vec<u8>,
}

impl Certificate {
    /// Construct a certificate snapshot from DER bytes.
    pub fn from_der_bytes(bytes: Vec<u8>) -> Self {
        Self {
            encoding: CertificateEncoding::Der,
            bytes,
        }
    }

    /// Construct a certificate snapshot from PEM bytes.
    pub fn from_pem_bytes(bytes: Vec<u8>) -> Self {
        Self {
            encoding: CertificateEncoding::Pem,
            bytes,
        }
    }

    /// Rebuild a raw `reqwest::Certificate`.
    pub fn build_raw(&self) -> Result<reqwest::Certificate, Error> {
        match self.encoding {
            CertificateEncoding::Der => reqwest::Certificate::from_der(&self.bytes),
            CertificateEncoding::Pem => reqwest::Certificate::from_pem(&self.bytes),
        }
        .map_err(Error::from)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe the TLS identity encoding:")]
enum IdentityEncoding {
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
    Unsupported {
        #[prompt("Human-readable description of the unsupported identity encoding:")]
        description: String,
    },
}

/// Owned client-identity snapshot.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe a TLS client identity snapshot:")]
pub struct Identity {
    #[prompt("Identity encoding and payload:")]
    encoding: IdentityEncoding,
}

impl Identity {
    /// Construct a PKCS#12 identity snapshot.
    pub fn from_pkcs12_der(archive: Vec<u8>, password: impl Into<String>) -> Self {
        Self {
            encoding: IdentityEncoding::Pkcs12 {
                archive,
                password: password.into(),
            },
        }
    }

    /// Construct a PKCS#8 identity snapshot.
    pub fn from_pkcs8_pem(cert_pem: Vec<u8>, key_pem: Vec<u8>) -> Self {
        Self {
            encoding: IdentityEncoding::Pkcs8 { cert_pem, key_pem },
        }
    }

    /// Record an unsupported but documented identity recipe.
    pub fn unsupported(description: impl Into<String>) -> Self {
        Self {
            encoding: IdentityEncoding::Unsupported {
                description: description.into(),
            },
        }
    }

    /// Rebuild a raw `reqwest::Identity` when expressible with the enabled reqwest TLS backend.
    pub fn build_raw(&self) -> Result<reqwest::Identity, Error> {
        match &self.encoding {
            IdentityEncoding::Pkcs12 { archive, password } => {
                reqwest::Identity::from_pkcs12_der(archive, password).map_err(Error::from)
            }
            IdentityEncoding::Pkcs8 { cert_pem, key_pem } => {
                reqwest::Identity::from_pkcs8_pem(cert_pem, key_pem).map_err(Error::from)
            }
            IdentityEncoding::Unsupported { description } => Err(Error::builder(format!(
                "unsupported TLS identity recipe cannot be rebuilt: {description}"
            ))),
        }
    }
}

/// Owned TLS connection metadata snapshot.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe TLS connection metadata:")]
#[to_code_literal(
    path = "::elicit_reqwest::TlsInfo::from_peer_certificate",
    tuple
)]
pub struct TlsInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Peer certificate DER bytes, when present:")]
    peer_certificate: Option<Vec<u8>>,
}

impl TlsInfo {
    /// Construct a TLS-info snapshot from optional peer-certificate bytes.
    pub fn from_peer_certificate(peer_certificate: Option<Vec<u8>>) -> Self {
        Self { peer_certificate }
    }

    /// Borrow the peer certificate, if available.
    pub fn peer_certificate(&self) -> Option<&[u8]> {
        self.peer_certificate.as_deref()
    }
}

impl From<reqwest::tls::TlsInfo> for TlsInfo {
    fn from(value: reqwest::tls::TlsInfo) -> Self {
        Self::from_peer_certificate(value.peer_certificate().map(<[u8]>::to_vec))
    }
}
