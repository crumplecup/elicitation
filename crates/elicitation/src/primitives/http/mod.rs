//! Reqwest HTTP type implementations.
//!
//! # Coverage map
//!
//! This module provides elicitation support for all reqwest-family types where
//! it is technically possible to do so. The table below records every type that
//! was assessed and the outcome, so future contributors know where the lines are
//! and why.
//!
//! ## Covered via trenchcoat wrapper
//!
//! These upstream types are opaque (no `Serialize`/`Deserialize`/`JsonSchema`),
//! so we shadow them with an owned trenchcoat that stores a serializable recipe
//! and rebuilds the raw object on demand via `build_raw`.
//!
//! | Upstream type | Trenchcoat |
//! |---|---|
//! | `reqwest::cookie::Jar` | [`ReqwestCookieJar`] |
//! | `reqwest::redirect::Policy` | [`ReqwestRedirectPolicy`] |
//! | `reqwest::retry::Builder` | [`ReqwestRetryBuilder`] |
//! | `reqwest::tls::Certificate` | [`ReqwestCertificate`] |
//! | `reqwest::tls::CertificateRevocationList` | [`ReqwestCertificateRevocationList`] |
//! | `reqwest::tls::Identity` | [`ReqwestIdentity`] |
//! | `reqwest::tls::TlsInfo` | [`ReqwestTlsInfo`] |
//!
//! Two lifetime-bound types (`reqwest::cookie::Cookie<'a>` and
//! `reqwest::redirect::Attempt<'a>`) also fall into this category. They cannot
//! satisfy the `'static` bound required by `Elicitation`, so we cover them with
//! owned snapshots: [`ReqwestCookie`] and [`ReqwestRedirectAttempt`].
//!
//! Note: `reqwest::retry::Builder::classify_fn` accepts a closure over private
//! reqwest types (`classify::ReqRep` / `classify::Action`) that cannot be named
//! outside the reqwest crate. The trenchcoat covers all other builder methods;
//! callers who need a custom classifier should apply `.classify_fn(…)` directly
//! to the `reqwest::retry::Builder` returned by
//! [`ReqwestRetryBuilder::build_raw`].
//!
//! ## Externally blocked
//!
//! Our elicitation traits are fully implemented for these types, but
//! `ElicitComplete` cannot be added because the upstream crate does not provide
//! `Serialize`, `Deserialize`, or `JsonSchema`. No action is possible until
//! those traits are upstreamed.
//!
//! `bytes::Bytes`, `http::error::Error`, `http::HeaderMap`, `http::HeaderName`,
//! `http::HeaderValue`, `http::Method`, `http::StatusCode`, `http::Version`,
//! `reqwest::tls::Version`
//!
//! ## Hard skips
//!
//! | Type | Reason |
//! |---|---|
//! | `http::extensions::Extensions` | Orphan rule: no upstream serde; type is a heterogeneous map with no stable serialization story. |
//! | `reqwest::redirect::Action` | Internal opaque type; no upstream serde and no public constructor. |
//! | `tower::util::boxed_clone_sync::BoxCloneSyncService` | Third-party type re-exported by reqwest; no upstream serde and the type erases its inner service. |

mod body;
mod bytes;
mod client;
mod cookie;
mod error;
mod header_map;
mod header_name;
mod header_value;
mod http_error;
mod method;
mod proxy;
mod redirect;
mod request;
mod retry;
mod request_builder;
mod response;
mod status_code;
mod tls;
mod tls_version;
mod version;

pub use body::BodyStyle;
pub use bytes::BytesStyle;
pub use client::ClientStyle;
pub use cookie::{
    ReqwestCookie, ReqwestCookieAttributes, ReqwestCookieEntry, ReqwestCookieFlags,
    ReqwestCookieJar,
};
pub use error::ErrorStyle;
pub use header_map::HeaderMapStyle;
pub use header_name::HeaderNameStyle;
pub use header_value::HeaderValueStyle;
pub use http_error::HttpErrorStyle;
pub use method::MethodStyle;
pub use proxy::{ReqwestNoProxy, ReqwestProxy};
pub use redirect::{ReqwestRedirectAction, ReqwestRedirectAttempt, ReqwestRedirectPolicy};
pub use retry::ReqwestRetryBuilder;
pub use request::RequestStyle;
pub use request_builder::RequestBuilderStyle;
pub use response::{ResponseStyle, capture_reqwest_response};
pub use status_code::StatusCodeStyle;
pub use tls::{
    ReqwestCertificate, ReqwestCertificateRevocationList, ReqwestIdentity, ReqwestTlsInfo,
};
pub use tls_version::TlsVersionStyle;
pub use version::VersionStyle;
