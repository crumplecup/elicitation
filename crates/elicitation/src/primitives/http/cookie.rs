//! Trenchcoats for `reqwest::cookie::{Cookie, Jar}`.

use std::time::{Duration, SystemTime};

use crate::{ElicitSpec, type_spec::TypeSpecInventoryKey};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

// ── ReqwestCookieEntry ────────────────────────────────────────────────────────

/// A single cookie-string + URL pair stored in a [`ReqwestCookieJar`] recipe.
///
/// Each entry corresponds to one [`reqwest::cookie::Jar::add_cookie_str`] call.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, crate::Elicit)]
#[prompt("Describe a cookie entry (raw cookie-header string and associated URL):")]
#[to_code_literal(path = "::elicitation::ReqwestCookieEntry::new", tuple)]
pub struct ReqwestCookieEntry {
    /// Raw cookie header value (e.g. `"session=abc; Path=/; HttpOnly"`).
    #[prompt("Raw cookie header string:")]
    cookie_str: String,
    /// URL the cookie is scoped to.
    #[prompt("URL this cookie is associated with:")]
    url: Url,
}

impl ReqwestCookieEntry {
    /// Construct a cookie entry from its parts.
    #[tracing::instrument(skip(cookie_str, url), level = "debug")]
    pub fn new(cookie_str: impl Into<String>, url: Url) -> Self {
        Self {
            cookie_str: cookie_str.into(),
            url,
        }
    }

    /// Borrow the raw cookie-header string.
    pub fn cookie_str(&self) -> &str {
        &self.cookie_str
    }

    /// Borrow the associated URL.
    pub fn url(&self) -> &Url {
        &self.url
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "elicitation::ReqwestCookieEntry",
    <ReqwestCookieEntry as ElicitSpec>::type_spec,
    std::any::TypeId::of::<ReqwestCookieEntry>
));

// ── ReqwestCookieJar ──────────────────────────────────────────────────────────

/// Owned recipe-tracking trenchcoat for `reqwest::cookie::Jar`.
///
/// Records each `add_cookie_str` call as a [`ReqwestCookieEntry`] so the jar
/// can be serialized, elicited, and rebuilt via `build_raw`. The raw
/// `reqwest::cookie::Jar` exposes no inspection API after construction, so the
/// recipe is the authoritative representation.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, crate::Elicit)]
#[prompt("Describe a cookie jar (list of cookie entries to add):")]
#[to_code_literal(path = "::elicitation::ReqwestCookieJar::from_entries", tuple)]
pub struct ReqwestCookieJar {
    #[prompt("Cookie entries to add to this jar:")]
    entries: Vec<ReqwestCookieEntry>,
}

impl ReqwestCookieJar {
    /// Construct a jar from a pre-built list of entries.
    #[tracing::instrument(skip(entries), level = "debug")]
    pub fn from_entries(entries: Vec<ReqwestCookieEntry>) -> Self {
        Self { entries }
    }

    /// Record a cookie entry, returning the updated jar (builder-style).
    #[tracing::instrument(skip(self, cookie_str, url), level = "debug")]
    pub fn add(self, cookie_str: impl Into<String>, url: Url) -> Self {
        let mut entries = self.entries;
        entries.push(ReqwestCookieEntry::new(cookie_str, url));
        Self { entries }
    }

    /// Borrow the recorded cookie entries.
    pub fn entries(&self) -> &[ReqwestCookieEntry] {
        &self.entries
    }

    /// Rebuild a raw `reqwest::cookie::Jar` by replaying every recorded entry.
    #[tracing::instrument(skip(self), level = "debug")]
    pub fn build_raw(&self) -> reqwest::cookie::Jar {
        let jar = reqwest::cookie::Jar::default();
        for entry in &self.entries {
            jar.add_cookie_str(entry.cookie_str(), entry.url());
        }
        jar
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "elicitation::ReqwestCookieJar",
    <ReqwestCookieJar as ElicitSpec>::type_spec,
    std::any::TypeId::of::<ReqwestCookieJar>
));

// ── ReqwestCookieFlags ────────────────────────────────────────────────────────

/// Security and same-site flag snapshot for a [`ReqwestCookie`].
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, crate::Elicit)]
#[prompt("Describe the security flags for this cookie:")]
#[to_code_literal(path = "::elicitation::ReqwestCookieFlags::new", tuple)]
pub struct ReqwestCookieFlags {
    /// Whether the `HttpOnly` flag is set.
    #[prompt("Is the HttpOnly flag set?")]
    http_only: bool,
    /// Whether the `Secure` flag is set.
    #[prompt("Is the Secure flag set?")]
    secure: bool,
    /// Whether `SameSite=Lax` is set.
    #[prompt("Is SameSite=Lax set?")]
    same_site_lax: bool,
    /// Whether `SameSite=Strict` is set.
    #[prompt("Is SameSite=Strict set?")]
    same_site_strict: bool,
}

impl ReqwestCookieFlags {
    /// Construct cookie flags from their parts.
    #[tracing::instrument(level = "debug")]
    pub fn new(http_only: bool, secure: bool, same_site_lax: bool, same_site_strict: bool) -> Self {
        Self {
            http_only,
            secure,
            same_site_lax,
            same_site_strict,
        }
    }

    /// Whether the `HttpOnly` flag is set.
    pub fn http_only(&self) -> bool {
        self.http_only
    }

    /// Whether the `Secure` flag is set.
    pub fn secure(&self) -> bool {
        self.secure
    }

    /// Whether `SameSite=Lax` is set.
    pub fn same_site_lax(&self) -> bool {
        self.same_site_lax
    }

    /// Whether `SameSite=Strict` is set.
    pub fn same_site_strict(&self) -> bool {
        self.same_site_strict
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "elicitation::ReqwestCookieFlags",
    <ReqwestCookieFlags as ElicitSpec>::type_spec,
    std::any::TypeId::of::<ReqwestCookieFlags>
));

// ── ReqwestCookieAttributes ───────────────────────────────────────────────────

/// Optional attribute snapshot for a [`ReqwestCookie`].
///
/// The `expires_unix_secs` field stores the cookie expiry as seconds since the
/// Unix epoch rather than `SystemTime` because the orphan rule prevents
/// `impl Serialize for SystemTime` in this crate. Cookies set before the Unix
/// epoch (effectively impossible in practice) are represented as `None`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, crate::Elicit)]
#[prompt("Describe the optional attributes for this cookie:")]
#[to_code_literal(path = "::elicitation::ReqwestCookieAttributes::new", tuple)]
pub struct ReqwestCookieAttributes {
    /// Optional cookie `Path` attribute.
    #[prompt("Cookie Path attribute, if present:")]
    path: Option<String>,
    /// Optional cookie `Domain` attribute.
    #[prompt("Cookie Domain attribute, if present:")]
    domain: Option<String>,
    /// Optional `Max-Age` expressed as a duration.
    #[prompt("Cookie Max-Age duration, if present:")]
    max_age: Option<Duration>,
    /// Optional expiry as seconds since the Unix epoch.
    #[prompt("Cookie expiry as seconds since Unix epoch, if present:")]
    expires_unix_secs: Option<u64>,
}

impl ReqwestCookieAttributes {
    /// Construct cookie attributes from their parts.
    #[tracing::instrument(level = "debug")]
    pub fn new(
        path: Option<String>,
        domain: Option<String>,
        max_age: Option<Duration>,
        expires_unix_secs: Option<u64>,
    ) -> Self {
        Self {
            path,
            domain,
            max_age,
            expires_unix_secs,
        }
    }

    /// Optional cookie `Path` attribute.
    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    /// Optional cookie `Domain` attribute.
    pub fn domain(&self) -> Option<&str> {
        self.domain.as_deref()
    }

    /// Optional `Max-Age` attribute.
    pub fn max_age(&self) -> Option<Duration> {
        self.max_age
    }

    /// Optional expiry as a `SystemTime`, converted from the stored Unix timestamp.
    pub fn expires(&self) -> Option<SystemTime> {
        self.expires_unix_secs
            .map(|secs| SystemTime::UNIX_EPOCH + Duration::from_secs(secs))
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "elicitation::ReqwestCookieAttributes",
    <ReqwestCookieAttributes as ElicitSpec>::type_spec,
    std::any::TypeId::of::<ReqwestCookieAttributes>
));

// ── ReqwestCookie ─────────────────────────────────────────────────────────────

/// Owned shadow of `reqwest::cookie::Cookie<'_>`.
///
/// Captures all observable state from the lifetime-bound `Cookie<'a>` into a
/// fully owned, serializable form. `Cookie` has no public constructor in
/// reqwest (it is only produced by `Response::cookies()`), so this type is a
/// pure snapshot — `build_raw` is not available.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, crate::Elicit)]
#[prompt("Describe an observed HTTP cookie snapshot:")]
#[to_code_literal(path = "::elicitation::ReqwestCookie::new", tuple)]
pub struct ReqwestCookie {
    /// Cookie name.
    #[prompt("Cookie name:")]
    name: String,
    /// Cookie value.
    #[prompt("Cookie value:")]
    value: String,
    /// Security and same-site flags.
    #[prompt("Security flags for this cookie:")]
    flags: ReqwestCookieFlags,
    /// Optional attributes (path, domain, max-age, expiry).
    #[prompt("Optional attributes for this cookie:")]
    attributes: ReqwestCookieAttributes,
}

impl ReqwestCookie {
    /// Construct a cookie snapshot from its parts.
    #[tracing::instrument(skip(name, value), level = "debug")]
    pub fn new(
        name: impl Into<String>,
        value: impl Into<String>,
        flags: ReqwestCookieFlags,
        attributes: ReqwestCookieAttributes,
    ) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            flags,
            attributes,
        }
    }

    /// Cookie name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Cookie value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Security and same-site flags.
    pub fn flags(&self) -> &ReqwestCookieFlags {
        &self.flags
    }

    /// Optional attributes (path, domain, max-age, expiry).
    pub fn attributes(&self) -> &ReqwestCookieAttributes {
        &self.attributes
    }
}

impl<'a> From<&reqwest::cookie::Cookie<'a>> for ReqwestCookie {
    #[tracing::instrument(skip(cookie), level = "debug")]
    fn from(cookie: &reqwest::cookie::Cookie<'a>) -> Self {
        let flags = ReqwestCookieFlags::new(
            cookie.http_only(),
            cookie.secure(),
            cookie.same_site_lax(),
            cookie.same_site_strict(),
        );
        let attributes = ReqwestCookieAttributes::new(
            cookie.path().map(str::to_string),
            cookie.domain().map(str::to_string),
            cookie.max_age(),
            cookie
                .expires()
                .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                .map(|d| d.as_secs()),
        );
        Self::new(cookie.name(), cookie.value(), flags, attributes)
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "elicitation::ReqwestCookie",
    <ReqwestCookie as ElicitSpec>::type_spec,
    std::any::TypeId::of::<ReqwestCookie>
));
