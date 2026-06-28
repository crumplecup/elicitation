//! Systematic API surface coverage for `elicit_reqwest`.
//!
//! Each test calls one or more methods from the corresponding reqwest type,
//! verifying that the method exists with the right signature and produces the
//! expected value. Compile errors here mean the shadow is missing a method.
//! Assertion failures mean the method exists but behaves incorrectly.

use elicit_reqwest::{
    Body, Certificate, CertificateRevocationList, Client, ClientBuilder, HeaderMap, HttpClient,
    Method, NoProxy, Policy, Proxy, Request, Response, StatusCode, Url, Version,
};
use futures::StreamExt;

// ── Helpers ───────────────────────────────────────────────────────────────────

fn test_cert() -> Certificate {
    let certified = rcgen::generate_simple_self_signed(vec!["localhost".to_string()])
        .expect("rcgen cert");
    Certificate::from_pem(certified.cert.pem().as_bytes()).expect("parse pem")
}

fn test_crl() -> CertificateRevocationList {
    let key_pair = rcgen::KeyPair::generate().expect("key pair");
    let cert_params =
        rcgen::CertificateParams::new(vec!["localhost".to_string()]).expect("cert params");
    let issuer = rcgen::CertifiedIssuer::self_signed(cert_params, key_pair).expect("issuer");
    let crl_params = rcgen::CertificateRevocationListParams {
        this_update: rcgen::date_time_ymd(2024, 1, 1),
        next_update: rcgen::date_time_ymd(2030, 1, 1),
        crl_number: rcgen::SerialNumber::from_slice(&[1]),
        issuing_distribution_point: None,
        revoked_certs: vec![],
        key_identifier_method: rcgen::KeyIdMethod::Sha256,
    };
    // CertifiedIssuer derefs to Issuer, which is what signed_by expects
    let crl = crl_params.signed_by(&issuer).expect("crl");
    CertificateRevocationList::from_pem(crl.pem().expect("crl pem").as_bytes())
        .expect("parse crl pem")
}

fn url(s: &str) -> Url {
    s.parse().expect("valid URL in test")
}

fn get_url() -> Url {
    url("https://example.com/api/v1/users")
}

fn client() -> Client {
    Client::new()
}

// ── Client ────────────────────────────────────────────────────────────────────

#[test]
fn client_new_and_builder_exist() {
    let _: Client = Client::new();
    let _: ClientBuilder = Client::builder();
}

#[test]
fn client_verb_methods_return_builder() {
    let c = client();
    let u = get_url();
    let _ = c.get_url(u.clone());
    let _ = c.post_url(u.clone());
    let _ = c.put_url(u.clone());
    let _ = c.delete_url(u.clone());
    let _ = c.patch_url(u.clone());
    let _ = c.head_url(u.clone());
}

#[test]
fn client_request_url_accepts_custom_method() {
    let c = client();
    let method = Method::from(reqwest::Method::OPTIONS);
    let rb = c.request_url(method, get_url());
    let req = rb.build().expect("valid request");
    assert_eq!(req.method().as_str(), "OPTIONS");
}

#[test]
fn client_execute_signature_is_present() {
    // Verify the method compiles by constructing the arguments.
    // drop() discards the unawaited future without running it.
    let c = client();
    let req = Request::new(Method::from(reqwest::Method::GET), get_url());
    drop(async move {
        let _: Result<Response, _> = c.execute(req).await;
    });
}

// ── ClientBuilder ─────────────────────────────────────────────────────────────

#[test]
fn client_builder_basic_options_chain() {
    let cb = ClientBuilder::new()
        .user_agent("elicit-test/1.0")
        .https_only(true)
        .referer(false)
        .connection_verbose(false)
        .cookie_store(true);
    cb.build().expect("builds without error");
}

#[test]
fn client_builder_timeout_options() {
    let cb = ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(30))
        .connect_timeout(std::time::Duration::from_secs(10))
        .read_timeout(std::time::Duration::from_secs(60))
        .tcp_keepalive(std::time::Duration::from_secs(5))
        .pool_idle_timeout(std::time::Duration::from_secs(90))
        .pool_max_idle_per_host(4)
        .tcp_nodelay(true);
    cb.build().expect("builds without error");
}

#[test]
fn client_builder_compression_flags() {
    let cb = ClientBuilder::new()
        .gzip(true)
        .brotli(true)
        .deflate(true)
        .zstd(true);
    cb.build().expect("builds without error");
}

#[test]
fn client_builder_redirect_policy() {
    let cb = ClientBuilder::new().redirect(Policy::limited(5));
    cb.build().expect("builds without error");
}

#[test]
fn client_builder_no_proxy() {
    let cb = ClientBuilder::new().no_proxy();
    cb.build().expect("builds without error");
}

#[test]
fn client_builder_proxy_and_no_proxy() {
    let proxy_url = url("http://proxy.internal:3128/");
    let proxy = Proxy::http(proxy_url).expect("valid proxy");
    let cb = ClientBuilder::new().proxy(proxy);
    cb.build().expect("builds without error");
}

#[test]
fn client_builder_http_version_options() {
    // http2_prior_knowledge and http1_only are mutually exclusive in practice;
    // test each independently.
    ClientBuilder::new()
        .http2_prior_knowledge()
        .build()
        .expect("http2 builds");

    ClientBuilder::new()
        .http1_only()
        .build()
        .expect("http1 builds");
}

#[test]
fn client_builder_http2_tuning() {
    let cb = ClientBuilder::new()
        .http2_initial_stream_window_size(1024 * 1024u32)
        .http2_initial_connection_window_size(4 * 1024 * 1024u32)
        .http2_adaptive_window(true)
        .http2_max_frame_size(16_384u32);
    cb.build().expect("builds without error");
}

#[test]
fn client_builder_tls_options() {
    let cb = ClientBuilder::new()
        .tls_sni(true)
        .min_tls_version(reqwest::tls::Version::TLS_1_2.into())
        .max_tls_version(reqwest::tls::Version::TLS_1_3.into())
        .tls_info(false);
    cb.build().expect("builds without error");
}

#[test]
fn client_builder_danger_options_do_not_panic() {
    let cb = ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true);
    cb.build().expect("builds without error");
}

#[test]
fn client_builder_local_address_none() {
    let cb = ClientBuilder::new().local_address(None);
    cb.build().expect("builds without error");
}

#[test]
fn client_builder_local_address_ipv4() {
    let addr: std::net::IpAddr = "127.0.0.1".parse().expect("valid IP");
    let cb = ClientBuilder::new().local_address(addr);
    cb.build().expect("builds without error");
}

#[test]
fn client_builder_default_headers() {
    let mut raw = http::HeaderMap::new();
    raw.insert(
        http::header::ACCEPT,
        http::HeaderValue::from_static("application/json"),
    );
    let headers = HeaderMap::from(raw);
    let cb = ClientBuilder::new().default_headers(headers);
    cb.build().expect("builds without error");
}

// ── ClientBuilder — TLS material ─────────────────────────────────────────────

#[test]
fn client_builder_add_root_certificate() {
    ClientBuilder::new()
        .add_root_certificate(test_cert())
        .build()
        .expect("builds with extra root cert");
}

#[test]
fn client_builder_add_crl() {
    ClientBuilder::new()
        .add_crl(test_crl())
        .build()
        .expect("builds with crl");
}

#[test]
fn client_builder_tls_certs_only() {
    ClientBuilder::new()
        .tls_certs_only(vec![test_cert()])
        .build()
        .expect("builds with tls_certs_only");
}

// ── RequestBuilder ────────────────────────────────────────────────────────────

#[test]
fn request_builder_method_and_url_preserved() {
    let req = client().get_url(get_url()).build().expect("valid request");
    assert_eq!(req.method().as_str(), "GET");
    assert_eq!(req.url().as_str(), "https://example.com/api/v1/users");
}

#[test]
fn request_builder_single_header() {
    let req = client()
        .get_url(get_url())
        .header("x-request-id".to_string(), "abc-123".to_string())
        .build()
        .expect("valid request");
    // Verify the request was built; header presence is in the raw request.
    assert_eq!(req.method().as_str(), "GET");
}

#[test]
fn request_builder_bulk_headers() {
    let mut raw = http::HeaderMap::new();
    raw.insert(
        http::header::ACCEPT,
        http::HeaderValue::from_static("application/json"),
    );
    raw.insert(
        http::header::CACHE_CONTROL,
        http::HeaderValue::from_static("no-cache"),
    );
    let headers = HeaderMap::from(raw);

    let req = client()
        .get_url(get_url())
        .headers(headers)
        .build()
        .expect("valid request");
    assert_eq!(req.method().as_str(), "GET");
}

#[test]
fn request_builder_version() {
    let v = Version::from(reqwest::Version::HTTP_11);
    let req = client()
        .get_url(get_url())
        .version(v)
        .build()
        .expect("valid request");
    assert_eq!(req.method().as_str(), "GET");
}

#[test]
fn request_builder_bearer_auth() {
    let req = client()
        .get_url(get_url())
        .bearer_auth("my-secret-token".to_string())
        .build()
        .expect("valid request");
    assert_eq!(req.method().as_str(), "GET");
}

#[test]
fn request_builder_basic_auth() {
    let req = client()
        .get_url(get_url())
        .basic_auth("user".to_string(), Some("pass".to_string()))
        .build()
        .expect("valid request");
    assert_eq!(req.method().as_str(), "GET");
}

#[test]
fn request_builder_timeout() {
    let req = client()
        .get_url(get_url())
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .expect("valid request");
    assert_eq!(req.method().as_str(), "GET");
}

#[test]
fn request_builder_query_appends_to_url() {
    let req = client()
        .get_url(url("https://api.example.com/search"))
        .query(vec![
            ("q".to_string(), "rust lang".to_string()),
            ("page".to_string(), "2".to_string()),
        ])
        .build()
        .expect("valid request");
    let url_str = req.url().as_str();
    assert!(
        url_str.contains("q=rust+lang") || url_str.contains("q=rust%20lang"),
        "expected query param in URL, got: {url_str}"
    );
    assert!(
        url_str.contains("page=2"),
        "expected page param, got: {url_str}"
    );
}

#[test]
fn request_builder_form_body() {
    let req = client()
        .post_url(url("https://api.example.com/login"))
        .form(vec![
            ("username".to_string(), "alice".to_string()),
            ("password".to_string(), "s3cr3t".to_string()),
        ])
        .build()
        .expect("valid request");
    assert_eq!(req.method().as_str(), "POST");
    let body = req.body().map(|b| b.as_bytes()).unwrap_or_default();
    let body_str = std::str::from_utf8(body).expect("valid UTF-8 body");
    assert!(
        body_str.contains("username=alice"),
        "expected username in body, got: {body_str}"
    );
    assert!(
        body_str.contains("password=s3cr3t"),
        "expected password in body, got: {body_str}"
    );
}

#[test]
fn request_builder_body_bytes() {
    let req = client()
        .post_url(url("https://api.example.com/upload"))
        .body_bytes(b"hello world".to_vec())
        .build()
        .expect("valid request");
    assert_eq!(req.method().as_str(), "POST");
    let body = req.body().map(|b| b.as_bytes()).unwrap_or_default();
    assert_eq!(body, b"hello world");
}

#[test]
fn request_builder_body_shadow_type() {
    let body = Body::from_bytes(b"shadow body".to_vec());
    let req = client()
        .post_url(url("https://api.example.com/upload"))
        .body(body)
        .build()
        .expect("valid request");
    assert_eq!(req.body().map(|b| b.as_bytes()), Some(b"shadow body".as_ref()));
}

#[test]
fn request_builder_json_bytes() {
    let req = client()
        .post_url(url("https://api.example.com/items"))
        .json_bytes(br#"{"name":"widget"}"#.to_vec())
        .build()
        .expect("valid request");
    assert_eq!(req.method().as_str(), "POST");
}

#[test]
fn request_builder_build_split() {
    let (returned_client, result) = client()
        .get_url(get_url())
        .bearer_auth("tok".to_string())
        .build_split();
    let req = result.expect("valid request");
    assert_eq!(req.method().as_str(), "GET");
    // The returned client can be used to make further requests.
    let _ = returned_client.get_url(get_url());
}

#[test]
fn request_builder_full_chain() {
    let req = client()
        .post_url(url("https://api.example.com/data"))
        .bearer_auth("token-abc".to_string())
        .header("x-trace-id".to_string(), "trace-123".to_string())
        .timeout(std::time::Duration::from_secs(10))
        .body_bytes(b"payload".to_vec())
        .build()
        .expect("valid request");
    assert_eq!(req.method().as_str(), "POST");
    assert_eq!(req.url().as_str(), "https://api.example.com/data");
}

// ── Request ───────────────────────────────────────────────────────────────────

#[test]
fn request_new_and_accessors() {
    let method = Method::from(reqwest::Method::DELETE);
    let u = url("https://api.example.com/items/42");
    let req = Request::new(method, u.clone());
    assert_eq!(req.method().as_str(), "DELETE");
    assert_eq!(req.url().as_str(), "https://api.example.com/items/42");
    assert!(req.body().is_none());
}

#[test]
fn request_headers_getter() {
    let req = client()
        .get_url(get_url())
        .header("x-foo".to_string(), "bar".to_string())
        .build()
        .expect("valid request");
    let _: &HeaderMap = req.headers();
}

#[test]
fn request_version_getter() {
    let req = client()
        .get_url(get_url())
        .version(Version::from(reqwest::Version::HTTP_2))
        .build()
        .expect("valid request");
    let _: &Version = req.version();
}

#[test]
fn request_timeout_getter() {
    let req = client()
        .get_url(get_url())
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .expect("valid request");
    assert_eq!(req.timeout(), Some(std::time::Duration::from_secs(5)));
}

// ── Response ──────────────────────────────────────────────────────────────────

fn make_response() -> Response {
    let mut raw_headers = http::HeaderMap::new();
    raw_headers.insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("application/json"),
    );
    raw_headers.insert(
        http::header::CONTENT_LENGTH,
        http::HeaderValue::from_static("13"),
    );
    Response::from_parts(
        StatusCode::from(reqwest::StatusCode::OK),
        Version::from(reqwest::Version::HTTP_11),
        url("https://api.example.com/items"),
        HeaderMap::from(raw_headers),
        br#"{"ok": true}"#.to_vec(),
    )
}

#[test]
fn response_status_returns_status_code() {
    let resp = make_response();
    let status: StatusCode = resp.status();
    assert_eq!(status.as_u16(), 200);
}

#[test]
fn response_url_returns_url_ref() {
    let resp = make_response();
    let u: &Url = resp.url();
    assert_eq!(u.as_str(), "https://api.example.com/items");
}

#[test]
fn response_version_returns_version() {
    let resp = make_response();
    let _: Version = resp.version();
}

#[test]
fn response_headers_returns_header_map_ref() {
    let resp = make_response();
    let _: &HeaderMap = resp.headers();
}

#[test]
fn response_content_length_from_headers() {
    let resp = make_response();
    assert_eq!(resp.content_length(), Some(13));
}

#[tokio::test]
async fn response_bytes_consumes_body() {
    let resp = make_response();
    let bytes: Vec<u8> = resp.bytes().await.expect("bytes");
    assert_eq!(bytes, br#"{"ok": true}"#);
}

#[tokio::test]
async fn response_text_decodes_body() {
    let resp = make_response();
    let text = resp.text().await.expect("text");
    assert_eq!(text, r#"{"ok": true}"#);
}

#[test]
fn response_error_for_status_ok_passes_through() {
    let resp = make_response();
    assert!(resp.error_for_status().is_ok());
}

#[test]
fn response_error_for_status_4xx_returns_err() {
    let resp = Response::from_parts(
        StatusCode::from(reqwest::StatusCode::NOT_FOUND),
        Version::from(reqwest::Version::HTTP_11),
        url("https://api.example.com/items/99"),
        HeaderMap::from(http::HeaderMap::new()),
        vec![],
    );
    assert!(resp.error_for_status().is_err());
}

#[test]
fn response_error_for_status_ref_does_not_consume() {
    let resp = make_response();
    let result = resp.error_for_status_ref();
    assert!(result.is_ok());
    // resp is still usable after error_for_status_ref
    assert_eq!(resp.status().as_u16(), 200);
}

#[tokio::test]
async fn response_text_with_charset_returns_body_as_string() {
    let resp = make_response();
    let text = resp.text_with_charset("utf-8").await.expect("text");
    assert_eq!(text, r#"{"ok": true}"#);
}

#[tokio::test]
async fn response_chunk_returns_bytes_then_none() {
    let mut resp = make_response();
    let chunk = resp.chunk().await.expect("chunk").expect("Some");
    assert_eq!(chunk.as_ref(), br#"{"ok": true}"#);
    let done = resp.chunk().await.expect("no error");
    assert!(done.is_none());
}

#[tokio::test]
async fn response_bytes_stream_yields_all_body_bytes() {
    let resp = make_response();
    let collected: Vec<_> = resp
        .bytes_stream()
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .map(|r| r.expect("chunk ok"))
        .collect();
    let all: Vec<u8> = collected.into_iter().flat_map(|b| b.to_vec()).collect();
    assert_eq!(all, br#"{"ok": true}"#);
}

// ── Proxy ─────────────────────────────────────────────────────────────────────

#[test]
fn proxy_http_construction() {
    let proxy = Proxy::http(url("http://proxy.corp.example:3128/")).expect("valid proxy");
    let _ = proxy.build_raw().expect("builds raw proxy");
}

#[test]
fn proxy_https_construction() {
    let proxy = Proxy::https(url("http://proxy.corp.example:3128/")).expect("valid proxy");
    let _ = proxy.build_raw().expect("builds raw proxy");
}

#[test]
fn proxy_all_construction() {
    let proxy = Proxy::all(url("http://proxy.corp.example:3128/")).expect("valid proxy");
    let _ = proxy.build_raw().expect("builds raw proxy");
}

#[test]
fn proxy_with_basic_auth() {
    let proxy = Proxy::http(url("http://proxy.corp.example:3128/"))
        .expect("valid proxy")
        .basic_auth("user".to_string(), "pass".to_string());
    let _ = proxy.build_raw().expect("builds raw proxy with auth");
}

#[test]
fn proxy_with_no_proxy_exclusions() {
    let proxy = Proxy::http(url("http://proxy.corp.example:3128/"))
        .expect("valid proxy")
        .no_proxy(Some(NoProxy::from_string("localhost,127.0.0.1")));
    let _ = proxy.build_raw().expect("builds with no-proxy list");
}

#[test]
fn proxy_with_custom_http_auth() {
    let proxy = Proxy::http(url("http://proxy.corp.example:3128/"))
        .expect("valid proxy")
        .custom_http_auth("Basic dXNlcjpwYXNz".to_string());
    let _ = proxy.build_raw().expect("builds raw proxy with custom auth");
}

#[test]
fn proxy_with_headers() {
    let mut raw = http::HeaderMap::new();
    raw.insert(
        http::header::PROXY_AUTHORIZATION,
        http::HeaderValue::from_static("Bearer tok"),
    );
    let proxy = Proxy::http(url("http://proxy.corp.example:3128/"))
        .expect("valid proxy")
        .headers(HeaderMap::from(raw));
    let _ = proxy.build_raw().expect("builds raw proxy with headers");
}

// ── Body ──────────────────────────────────────────────────────────────────────

#[test]
fn body_from_bytes_and_accessor() {
    let b = Body::from_bytes(vec![1u8, 2, 3]);
    assert_eq!(b.as_bytes(), &[1u8, 2, 3]);
}

#[test]
fn body_empty() {
    let b = Body::from_bytes(vec![]);
    assert!(b.as_bytes().is_empty());
}
