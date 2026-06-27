//! Shadow for `reqwest::ClientBuilder`.

use elicitation::Elicit;
use elicitation_derive::reflect_methods;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    Certificate, CertificateRevocationList, Error, HeaderMap, Identity, Policy, Proxy, TlsVersion,
};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe a duration in seconds and nanoseconds:")]
struct DurationSpec {
    #[prompt("Whole seconds component:")]
    secs: u64,
    #[prompt("Nanoseconds component:")]
    nanos: u32,
}

impl DurationSpec {
    #[instrument(skip(timeout), level = "trace")]
    fn from_duration(timeout: std::time::Duration) -> Self {
        Self {
            secs: timeout.as_secs(),
            nanos: timeout.subsec_nanos(),
        }
    }

    #[instrument(skip(self), level = "trace")]
    fn to_duration(&self) -> std::time::Duration {
        std::time::Duration::new(self.secs, self.nanos)
    }
}

/// Owned `reqwest::ClientBuilder` recipe.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe an HTTP client builder recipe:")]
pub struct ClientBuilder {
    // ── Basic ───────────────────────────────────────────────────────────────
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Optional User-Agent header value:")]
    user_agent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Optional default header map:")]
    default_headers: Option<HeaderMap>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Optional redirect policy:")]
    redirect: Option<Policy>,
    #[serde(default)]
    #[prompt("Zero or more configured proxies:")]
    proxies: Vec<Proxy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("When true, only HTTPS connections are allowed:")]
    https_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("When true, follow the Referer header on redirects:")]
    referer: Option<bool>,

    // ── Timeouts / connection pool ───────────────────────────────────────────
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Optional overall request timeout:")]
    timeout: Option<DurationSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Optional connect timeout:")]
    connect_timeout: Option<DurationSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Optional TCP keep-alive interval:")]
    tcp_keepalive: Option<DurationSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Optional pool idle connection timeout:")]
    pool_idle_timeout: Option<DurationSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Maximum idle connections per host:")]
    pool_max_idle_per_host: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("When true, disable Nagle's algorithm (TCP_NODELAY):")]
    tcp_nodelay: Option<bool>,

    // ── Compression ─────────────────────────────────────────────────────────
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Enable gzip decompression:")]
    gzip: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Enable brotli decompression:")]
    brotli: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Enable deflate decompression:")]
    deflate: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Enable zstd decompression:")]
    zstd: Option<bool>,

    // ── TLS ─────────────────────────────────────────────────────────────────
    #[serde(default)]
    #[prompt("Additional root certificates to trust:")]
    root_certificates: Vec<Certificate>,
    #[serde(default)]
    #[prompt("Certificate revocation lists:")]
    certificate_revocation_lists: Vec<CertificateRevocationList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Optional client identity for mutual TLS:")]
    identity: Option<Identity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Minimum TLS version to accept:")]
    min_tls_version: Option<TlsVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Maximum TLS version to accept:")]
    max_tls_version: Option<TlsVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("When false, disable TLS SNI:")]
    tls_sni: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("DANGER: accept invalid TLS certificates:")]
    danger_accept_invalid_certs: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("DANGER: accept invalid TLS hostnames:")]
    danger_accept_invalid_hostnames: Option<bool>,

    // ── HTTP version ────────────────────────────────────────────────────────
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("When true, force HTTP/2 without prior-knowledge negotiation:")]
    http2_prior_knowledge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("When true, force HTTP/1 only:")]
    http1_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Initial HTTP/2 stream window size in bytes:")]
    http2_initial_stream_window_size: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Initial HTTP/2 connection window size in bytes:")]
    http2_initial_connection_window_size: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("When true, enable HTTP/2 adaptive flow control:")]
    http2_adaptive_window: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Maximum HTTP/2 frame size in bytes:")]
    http2_max_frame_size: Option<u32>,

    // ── Cookies ─────────────────────────────────────────────────────────────
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("When true, enable the built-in cookie store:")]
    cookie_store: Option<bool>,

    // ── Connection ──────────────────────────────────────────────────────────
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Optional read timeout:")]
    read_timeout: Option<DurationSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("When true, log verbose connection information:")]
    connection_verbose: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Optional local IP address to bind outgoing connections to:")]
    local_address: Option<String>,
    #[serde(default)]
    #[prompt("When true, no_proxy() was called to disable all proxies:")]
    no_proxy_set: bool,

    // ── TLS extras ──────────────────────────────────────────────────────────
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[prompt("Certificates that form an exclusive trust store (disables system certs):")]
    tls_certs_only: Vec<Certificate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("When true, attach TLS handshake info to responses:")]
    tls_info: Option<bool>,
}

impl ClientBuilder {
    /// Construct a new client-builder recipe with reqwest defaults.
    #[instrument(level = "debug")]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the default User-Agent value.
    #[instrument(skip(self, user_agent), level = "debug")]
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Set default request headers.
    #[instrument(skip(self, headers), level = "debug")]
    pub fn default_headers(mut self, headers: HeaderMap) -> Self {
        self.default_headers = Some(headers);
        self
    }

    /// Set the redirect policy.
    #[instrument(skip(self, policy), level = "debug")]
    pub fn redirect(mut self, policy: Policy) -> Self {
        self.redirect = Some(policy);
        self
    }

    /// Append a configured proxy.
    #[instrument(skip(self, proxy), level = "debug")]
    pub fn proxy(mut self, proxy: Proxy) -> Self {
        self.proxies.push(proxy);
        self
    }

    /// Restrict connections to HTTPS only.
    #[instrument(skip(self), level = "debug")]
    pub fn https_only(mut self, enabled: bool) -> Self {
        self.https_only = Some(enabled);
        self
    }

    /// Control whether the Referer header is followed on redirects.
    #[instrument(skip(self), level = "debug")]
    pub fn referer(mut self, enable: bool) -> Self {
        self.referer = Some(enable);
        self
    }

    /// Set the overall request timeout.
    #[instrument(skip(self, timeout), level = "debug")]
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(DurationSpec::from_duration(timeout));
        self
    }

    /// Set the connect timeout.
    #[instrument(skip(self, timeout), level = "debug")]
    pub fn connect_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.connect_timeout = Some(DurationSpec::from_duration(timeout));
        self
    }

    /// Set the TCP keep-alive interval.
    #[instrument(skip(self, interval), level = "debug")]
    pub fn tcp_keepalive(mut self, interval: impl Into<Option<std::time::Duration>>) -> Self {
        self.tcp_keepalive = interval.into().map(DurationSpec::from_duration);
        self
    }

    /// Set the pool idle connection timeout.
    #[instrument(skip(self, val), level = "debug")]
    pub fn pool_idle_timeout(mut self, val: impl Into<Option<std::time::Duration>>) -> Self {
        self.pool_idle_timeout = val.into().map(DurationSpec::from_duration);
        self
    }

    /// Set the maximum idle connections per host.
    #[instrument(skip(self), level = "debug")]
    pub fn pool_max_idle_per_host(mut self, max: usize) -> Self {
        self.pool_max_idle_per_host = Some(max);
        self
    }

    /// Enable or disable TCP_NODELAY.
    #[instrument(skip(self), level = "debug")]
    pub fn tcp_nodelay(mut self, enabled: bool) -> Self {
        self.tcp_nodelay = Some(enabled);
        self
    }

    /// Enable or disable gzip decompression.
    #[instrument(skip(self), level = "debug")]
    pub fn gzip(mut self, enable: bool) -> Self {
        self.gzip = Some(enable);
        self
    }

    /// Enable or disable brotli decompression.
    #[instrument(skip(self), level = "debug")]
    pub fn brotli(mut self, enable: bool) -> Self {
        self.brotli = Some(enable);
        self
    }

    /// Enable or disable deflate decompression.
    #[instrument(skip(self), level = "debug")]
    pub fn deflate(mut self, enable: bool) -> Self {
        self.deflate = Some(enable);
        self
    }

    /// Enable or disable zstd decompression.
    #[instrument(skip(self), level = "debug")]
    pub fn zstd(mut self, enable: bool) -> Self {
        self.zstd = Some(enable);
        self
    }

    /// Add a root certificate to the trust store.
    #[instrument(skip(self, cert), level = "debug")]
    pub fn add_root_certificate(mut self, cert: Certificate) -> Self {
        self.root_certificates.push(cert);
        self
    }

    /// Add a certificate revocation list.
    #[instrument(skip(self, crl), level = "debug")]
    pub fn add_crl(mut self, crl: CertificateRevocationList) -> Self {
        self.certificate_revocation_lists.push(crl);
        self
    }

    /// Set the client identity for mutual TLS.
    #[instrument(skip(self, identity), level = "debug")]
    pub fn identity(mut self, identity: Identity) -> Self {
        self.identity = Some(identity);
        self
    }

    /// Set the minimum TLS version.
    #[instrument(skip(self, version), level = "debug")]
    pub fn min_tls_version(mut self, version: TlsVersion) -> Self {
        self.min_tls_version = Some(version);
        self
    }

    /// Set the maximum TLS version.
    #[instrument(skip(self, version), level = "debug")]
    pub fn max_tls_version(mut self, version: TlsVersion) -> Self {
        self.max_tls_version = Some(version);
        self
    }

    /// Enable or disable TLS SNI.
    #[instrument(skip(self), level = "debug")]
    pub fn tls_sni(mut self, tls_sni: bool) -> Self {
        self.tls_sni = Some(tls_sni);
        self
    }

    /// Disable TLS certificate verification (DANGER: use only in tests).
    #[instrument(skip(self), level = "debug")]
    pub fn danger_accept_invalid_certs(mut self, accept: bool) -> Self {
        self.danger_accept_invalid_certs = Some(accept);
        self
    }

    /// Disable TLS hostname verification (DANGER: use only in tests).
    #[instrument(skip(self), level = "debug")]
    pub fn danger_accept_invalid_hostnames(mut self, accept: bool) -> Self {
        self.danger_accept_invalid_hostnames = Some(accept);
        self
    }

    /// Force HTTP/2 without prior-knowledge negotiation.
    #[instrument(skip(self), level = "debug")]
    pub fn http2_prior_knowledge(mut self) -> Self {
        self.http2_prior_knowledge = Some(true);
        self
    }

    /// Force HTTP/1 only, disabling HTTP/2 negotiation.
    #[instrument(skip(self), level = "debug")]
    pub fn http1_only(mut self) -> Self {
        self.http1_only = Some(true);
        self
    }

    /// Set the initial HTTP/2 stream window size.
    #[instrument(skip(self, size), level = "debug")]
    pub fn http2_initial_stream_window_size(mut self, size: impl Into<Option<u32>>) -> Self {
        self.http2_initial_stream_window_size = size.into();
        self
    }

    /// Set the initial HTTP/2 connection window size.
    #[instrument(skip(self, size), level = "debug")]
    pub fn http2_initial_connection_window_size(mut self, size: impl Into<Option<u32>>) -> Self {
        self.http2_initial_connection_window_size = size.into();
        self
    }

    /// Enable or disable HTTP/2 adaptive flow control.
    #[instrument(skip(self), level = "debug")]
    pub fn http2_adaptive_window(mut self, enabled: bool) -> Self {
        self.http2_adaptive_window = Some(enabled);
        self
    }

    /// Set the maximum HTTP/2 frame size.
    #[instrument(skip(self, size), level = "debug")]
    pub fn http2_max_frame_size(mut self, size: impl Into<Option<u32>>) -> Self {
        self.http2_max_frame_size = size.into();
        self
    }

    /// Enable or disable the built-in cookie store.
    #[instrument(skip(self), level = "debug")]
    pub fn cookie_store(mut self, enable: bool) -> Self {
        self.cookie_store = Some(enable);
        self
    }

    /// Set the per-read timeout.
    #[instrument(skip(self, timeout), level = "debug")]
    pub fn read_timeout(mut self, timeout: impl Into<Option<std::time::Duration>>) -> Self {
        self.read_timeout = timeout.into().map(DurationSpec::from_duration);
        self
    }

    /// Enable or disable verbose connection logging.
    #[instrument(skip(self), level = "debug")]
    pub fn connection_verbose(mut self, verbose: bool) -> Self {
        self.connection_verbose = Some(verbose);
        self
    }

    /// Bind outgoing connections to the given local IP address.
    #[instrument(skip(self, addr), level = "debug")]
    pub fn local_address(mut self, addr: impl Into<Option<std::net::IpAddr>>) -> Self {
        self.local_address = addr.into().map(|a| a.to_string());
        self
    }

    /// Disable all proxy use (overrides any configured proxies).
    #[instrument(skip(self), level = "debug")]
    pub fn no_proxy(mut self) -> Self {
        self.no_proxy_set = true;
        self
    }

    /// Trust only the given certificates, disabling the system trust store.
    #[instrument(skip(self, certs), level = "debug")]
    pub fn tls_certs_only(mut self, certs: impl IntoIterator<Item = Certificate>) -> Self {
        self.tls_certs_only.extend(certs);
        self
    }

    /// Attach TLS handshake info to responses.
    #[instrument(skip(self), level = "debug")]
    pub fn tls_info(mut self, enable: bool) -> Self {
        self.tls_info = Some(enable);
        self
    }

    #[instrument(skip(self), level = "debug")]
    fn build_reqwest_client(&self) -> Result<reqwest::Client, Error> {
        let mut builder = reqwest::Client::builder();

        if let Some(ua) = &self.user_agent {
            builder = builder.user_agent(ua.clone());
        }
        if let Some(headers) = &self.default_headers {
            builder = builder.default_headers(http::HeaderMap::from(headers.clone()));
        }
        if let Some(policy) = &self.redirect {
            builder = builder.redirect(policy.build_raw()?);
        }
        for proxy in &self.proxies {
            builder = builder.proxy(proxy.build_raw()?);
        }
        if let Some(enabled) = self.https_only {
            builder = builder.https_only(enabled);
        }
        if let Some(enable) = self.referer {
            builder = builder.referer(enable);
        }
        if let Some(t) = &self.timeout {
            builder = builder.timeout(t.to_duration());
        }
        if let Some(t) = &self.connect_timeout {
            builder = builder.connect_timeout(t.to_duration());
        }
        if let Some(t) = &self.tcp_keepalive {
            builder = builder.tcp_keepalive(t.to_duration());
        }
        if let Some(t) = &self.pool_idle_timeout {
            builder = builder.pool_idle_timeout(t.to_duration());
        }
        if let Some(max) = self.pool_max_idle_per_host {
            builder = builder.pool_max_idle_per_host(max);
        }
        if let Some(enabled) = self.tcp_nodelay {
            builder = builder.tcp_nodelay(enabled);
        }
        if let Some(enable) = self.gzip {
            builder = builder.gzip(enable);
        }
        if let Some(enable) = self.brotli {
            builder = builder.brotli(enable);
        }
        if let Some(enable) = self.deflate {
            builder = builder.deflate(enable);
        }
        if let Some(enable) = self.zstd {
            builder = builder.zstd(enable);
        }
        for cert in &self.root_certificates {
            builder = builder.add_root_certificate(cert.build_raw()?);
        }
        for crl in &self.certificate_revocation_lists {
            builder = builder.add_crl(crl.build_raw()?);
        }
        if let Some(identity) = &self.identity {
            builder = builder.identity(identity.build_raw()?);
        }
        if let Some(version) = &self.min_tls_version {
            builder = builder.min_tls_version(reqwest::tls::Version::from(version.clone()));
        }
        if let Some(version) = &self.max_tls_version {
            builder = builder.max_tls_version(reqwest::tls::Version::from(version.clone()));
        }
        if let Some(sni) = self.tls_sni {
            builder = builder.tls_sni(sni);
        }
        if let Some(accept) = self.danger_accept_invalid_certs {
            builder = builder.tls_danger_accept_invalid_certs(accept);
        }
        if let Some(accept) = self.danger_accept_invalid_hostnames {
            builder = builder.tls_danger_accept_invalid_hostnames(accept);
        }
        if self.http2_prior_knowledge == Some(true) {
            builder = builder.http2_prior_knowledge();
        }
        if self.http1_only == Some(true) {
            builder = builder.http1_only();
        }
        if let Some(size) = self.http2_initial_stream_window_size {
            builder = builder.http2_initial_stream_window_size(size);
        }
        if let Some(size) = self.http2_initial_connection_window_size {
            builder = builder.http2_initial_connection_window_size(size);
        }
        if let Some(enabled) = self.http2_adaptive_window {
            builder = builder.http2_adaptive_window(enabled);
        }
        if let Some(size) = self.http2_max_frame_size {
            builder = builder.http2_max_frame_size(size);
        }
        if let Some(enable) = self.cookie_store {
            builder = builder.cookie_store(enable);
        }
        if let Some(t) = &self.read_timeout {
            builder = builder.read_timeout(t.to_duration());
        }
        if let Some(verbose) = self.connection_verbose {
            builder = builder.connection_verbose(verbose);
        }
        if let Some(ref addr_str) = self.local_address
            && let Ok(addr) = addr_str.parse::<std::net::IpAddr>()
        {
            builder = builder.local_address(addr);
        }
        if self.no_proxy_set {
            builder = builder.no_proxy();
        }
        if !self.tls_certs_only.is_empty() {
            let certs: Result<Vec<_>, _> = self
                .tls_certs_only
                .iter()
                .map(Certificate::build_raw)
                .collect();
            builder = builder.tls_certs_only(certs?);
        }
        if let Some(enable) = self.tls_info {
            builder = builder.tls_info(enable);
        }

        Ok(builder.build()?)
    }

    /// Build a `Client` from this recipe.
    #[instrument(skip(self), level = "debug")]
    pub fn build(self) -> Result<crate::Client, Error> {
        let raw = self.build_reqwest_client()?;
        Ok(crate::Client::from_raw(raw))
    }
}

#[reflect_methods]
impl ClientBuilder {
    /// Borrow the configured proxies.
    pub fn proxies(&self) -> &[Proxy] {
        &self.proxies
    }

    /// Borrow the additional root certificates.
    pub fn root_certificates(&self) -> &[Certificate] {
        &self.root_certificates
    }
}
