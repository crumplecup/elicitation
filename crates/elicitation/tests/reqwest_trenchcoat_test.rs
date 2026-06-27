#![cfg(feature = "reqwest")]

use elicitation::{
    ReqwestCertificate, ReqwestCertificateRevocationList, ReqwestCookie, ReqwestCookieAttributes,
    ReqwestCookieEntry, ReqwestCookieFlags, ReqwestCookieJar, ReqwestIdentity, ReqwestNoProxy,
    ReqwestProxy, ReqwestRedirectAction, ReqwestRedirectAttempt, ReqwestRedirectPolicy,
    ReqwestRetryBuilder, ReqwestTlsInfo,
};
use url::Url;

// ── ReqwestCertificateRevocationList ─────────────────────────────────────────

#[test]
fn crl_from_pem_rejects_invalid_bytes() {
    let result = ReqwestCertificateRevocationList::from_pem(b"not a real crl");
    assert!(result.is_err(), "invalid PEM should not construct a CRL");
}

#[test]
fn crl_from_pem_serializes_and_deserializes() {
    // Inject via the serde shape: {"recipe":{"Pem":{"pem":[...]}}}
    let json = r#"{"recipe":{"Pem":{"pem":[104,101,108,108,111]}}}"#;
    let restored: ReqwestCertificateRevocationList =
        serde_json::from_str(json).expect("deserializes");
    let json2 = serde_json::to_string(&restored).expect("serializes");
    assert!(
        json2.contains("Pem"),
        "serialized form contains Pem variant: {json2}"
    );
    assert!(
        json2.contains("pem"),
        "serialized form contains pem field: {json2}"
    );
}

#[test]
fn crl_to_code_literal_contains_from_pem() {
    use elicitation::emit_code::ToCodeLiteral;
    let json = r#"{"recipe":{"Pem":{"pem":[104,101,108,108,111]}}}"#;
    let crl: ReqwestCertificateRevocationList = serde_json::from_str(json).expect("deserializes");
    let tokens = crl.to_code_literal().to_string();
    assert!(tokens.contains("from_pem"), "tokens: {tokens}");
    assert!(
        tokens.contains("ReqwestCertificateRevocationList"),
        "tokens: {tokens}"
    );
}

#[test]
fn crl_build_raw_after_deserialization_fails_on_invalid_bytes() {
    let json = r#"{"recipe":{"Pem":{"pem":[104,101,108,108,111]}}}"#;
    let crl: ReqwestCertificateRevocationList = serde_json::from_str(json).expect("deserializes");
    assert!(
        crl.build_raw().is_err(),
        "non-CRL bytes should fail to rebuild"
    );
}

#[test]
fn no_proxy_rebuilds_from_raw_string() {
    let no_proxy = ReqwestNoProxy::from_string("localhost,127.0.0.1");
    assert!(no_proxy.build_raw().is_ok());
}

#[test]
fn custom_proxy_without_live_raw_is_explicitly_blocked() {
    let proxy = ReqwestProxy::custom(
        "|url| if url.domain() == Some(\"example.com\") { None::<&str> } else { Some(\"http://proxy\") }",
    );
    assert!(proxy.build_raw().is_err());
}

#[test]
fn certificate_round_trips_der_snapshot() {
    let certificate = ReqwestCertificate::from_der(b"not der").unwrap();
    assert!(certificate.build_raw().is_ok());
}

#[test]
fn identity_rejects_invalid_pem_identity() {
    let identity = ReqwestIdentity::from_pem(b"not pem");
    assert!(identity.is_err());
}

#[test]
fn tls_info_without_live_raw_is_explicitly_blocked() {
    let tls_info = ReqwestTlsInfo::from_peer_certificate(Some(vec![1, 2, 3]));
    assert!(tls_info.build_raw().is_err());
}

// ── ReqwestCookieJar ──────────────────────────────────────────────────────────

#[test]
fn cookie_jar_replays_entries_on_build_raw() {
    let url = Url::parse("https://example.com").expect("valid URL");
    let jar = ReqwestCookieJar::default()
        .add("session=abc; Path=/; HttpOnly", url.clone())
        .add("csrf=xyz; Path=/api", url.clone());
    let raw = jar.build_raw();
    // reqwest::cookie::Jar has no public inspection API; we verify it builds without panic
    let _ = raw;
    assert_eq!(jar.entries().len(), 2);
}

#[test]
fn cookie_jar_serializes_and_deserializes() {
    let url = Url::parse("https://example.com").expect("valid URL");
    let jar = ReqwestCookieJar::default().add("a=1", url);
    let json = serde_json::to_string(&jar).expect("serializes");
    let restored: ReqwestCookieJar = serde_json::from_str(&json).expect("deserializes");
    assert_eq!(restored.entries().len(), 1);
}

#[test]
fn cookie_entry_to_code_literal_contains_url_and_cookie() {
    use elicitation::emit_code::ToCodeLiteral;
    let url = Url::parse("https://example.com").expect("valid URL");
    let entry = ReqwestCookieEntry::new("session=tok; Path=/", url);
    let tokens = entry.to_code_literal().to_string();
    assert!(tokens.contains("session=tok"), "tokens: {tokens}");
    assert!(tokens.contains("example.com"), "tokens: {tokens}");
}

#[test]
fn cookie_jar_to_code_literal_contains_from_entries() {
    use elicitation::emit_code::ToCodeLiteral;
    let jar = ReqwestCookieJar::default();
    let tokens = jar.to_code_literal().to_string();
    assert!(tokens.contains("from_entries"), "tokens: {tokens}");
}

// ── ReqwestCookieFlags ────────────────────────────────────────────────────────

#[test]
fn cookie_flags_roundtrips() {
    let flags = ReqwestCookieFlags::new(true, false, true, false);
    assert!(flags.http_only());
    assert!(!flags.secure());
    assert!(flags.same_site_lax());
    assert!(!flags.same_site_strict());
    let json = serde_json::to_string(&flags).expect("serializes");
    let restored: ReqwestCookieFlags = serde_json::from_str(&json).expect("deserializes");
    assert_eq!(restored.http_only(), flags.http_only());
    assert_eq!(restored.same_site_strict(), flags.same_site_strict());
}

#[test]
fn cookie_flags_to_code_literal() {
    use elicitation::emit_code::ToCodeLiteral;
    let flags = ReqwestCookieFlags::new(true, true, false, false);
    let tokens = flags.to_code_literal().to_string();
    assert!(tokens.contains("ReqwestCookieFlags"), "tokens: {tokens}");
    assert!(tokens.contains("true"), "tokens: {tokens}");
}

// ── ReqwestCookieAttributes ───────────────────────────────────────────────────

#[test]
fn cookie_attributes_expires_roundtrips_via_unix_secs() {
    use std::time::{Duration, SystemTime};
    let attrs = ReqwestCookieAttributes::new(
        Some("/api".to_string()),
        Some("example.com".to_string()),
        Some(Duration::from_secs(3600)),
        Some(1_700_000_000),
    );
    assert_eq!(attrs.path(), Some("/api"));
    assert_eq!(attrs.domain(), Some("example.com"));
    assert_eq!(attrs.max_age(), Some(Duration::from_secs(3600)));
    let expected = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    assert_eq!(attrs.expires(), Some(expected));
}

#[test]
fn cookie_attributes_empty_roundtrips() {
    let attrs = ReqwestCookieAttributes::new(None, None, None, None);
    let json = serde_json::to_string(&attrs).expect("serializes");
    let restored: ReqwestCookieAttributes = serde_json::from_str(&json).expect("deserializes");
    assert_eq!(restored.path(), None);
    assert_eq!(restored.expires(), None);
}

// ── ReqwestCookie ─────────────────────────────────────────────────────────────

#[test]
fn cookie_snapshot_stores_name_and_value() {
    let flags = ReqwestCookieFlags::new(true, true, false, false);
    let attrs = ReqwestCookieAttributes::new(None, None, None, None);
    let cookie = ReqwestCookie::new("session", "abc123", flags, attrs);
    assert_eq!(cookie.name(), "session");
    assert_eq!(cookie.value(), "abc123");
    assert!(cookie.flags().http_only());
    assert!(cookie.flags().secure());
}

#[test]
fn cookie_snapshot_serializes_and_deserializes() {
    let flags = ReqwestCookieFlags::new(false, true, true, false);
    let attrs = ReqwestCookieAttributes::new(Some("/".to_string()), None, None, None);
    let cookie = ReqwestCookie::new("csrf", "tok", flags, attrs);
    let json = serde_json::to_string(&cookie).expect("serializes");
    let restored: ReqwestCookie = serde_json::from_str(&json).expect("deserializes");
    assert_eq!(restored.name(), "csrf");
    assert_eq!(restored.value(), "tok");
    assert_eq!(restored.attributes().path(), Some("/"));
}

#[test]
fn cookie_snapshot_to_code_literal_contains_name() {
    use elicitation::emit_code::ToCodeLiteral;
    let flags = ReqwestCookieFlags::new(false, false, false, false);
    let attrs = ReqwestCookieAttributes::new(None, None, None, None);
    let cookie = ReqwestCookie::new("my_cookie", "val", flags, attrs);
    let tokens = cookie.to_code_literal().to_string();
    assert!(tokens.contains("my_cookie"), "tokens: {tokens}");
    assert!(tokens.contains("ReqwestCookie"), "tokens: {tokens}");
}

// ── ReqwestRedirectPolicy ─────────────────────────────────────────────────────

#[test]
fn redirect_limited_rebuilds() {
    let policy = ReqwestRedirectPolicy::limited(5);
    assert!(policy.build_raw().is_ok());
}

#[test]
fn redirect_none_rebuilds() {
    let policy = ReqwestRedirectPolicy::none();
    assert!(policy.build_raw().is_ok());
}

#[test]
fn redirect_custom_with_live_handler_rebuilds() {
    let policy = ReqwestRedirectPolicy::custom("follow everything", |_attempt| {
        ReqwestRedirectAction::Follow
    });
    assert!(policy.build_raw().is_ok());
}

#[test]
fn redirect_custom_after_deserialization_blocks_build_raw() {
    let policy = ReqwestRedirectPolicy::custom("follow everything", |_attempt| {
        ReqwestRedirectAction::Follow
    });
    let json = serde_json::to_string(&policy).expect("serializes");
    let restored: ReqwestRedirectPolicy = serde_json::from_str(&json).expect("deserializes");
    assert!(
        restored.build_raw().is_err(),
        "custom policy without live handler must error on build_raw"
    );
}

#[test]
fn redirect_limited_serializes_and_deserializes() {
    let policy = ReqwestRedirectPolicy::limited(3);
    let json = serde_json::to_string(&policy).expect("serializes");
    let restored: ReqwestRedirectPolicy = serde_json::from_str(&json).expect("deserializes");
    assert!(restored.build_raw().is_ok());
}

#[test]
fn redirect_none_serializes_and_deserializes() {
    let policy = ReqwestRedirectPolicy::none();
    let json = serde_json::to_string(&policy).expect("serializes");
    let restored: ReqwestRedirectPolicy = serde_json::from_str(&json).expect("deserializes");
    assert!(restored.build_raw().is_ok());
}

#[test]
fn redirect_policy_to_code_literal_limited() {
    use elicitation::emit_code::ToCodeLiteral;
    let policy = ReqwestRedirectPolicy::limited(7);
    let tokens = policy.to_code_literal().to_string();
    assert!(
        tokens.contains("limited"),
        "should contain 'limited': {tokens}"
    );
    assert!(
        tokens.contains("7"),
        "should contain the max value: {tokens}"
    );
}

#[test]
fn redirect_policy_to_code_literal_none() {
    use elicitation::emit_code::ToCodeLiteral;
    let policy = ReqwestRedirectPolicy::none();
    let tokens = policy.to_code_literal().to_string();
    assert!(tokens.contains("none"), "should contain 'none': {tokens}");
}

#[test]
fn redirect_policy_to_code_literal_custom_without_code_includes_description() {
    use elicitation::emit_code::ToCodeLiteral;
    let policy = ReqwestRedirectPolicy::custom("allow only example.com", |_attempt| {
        ReqwestRedirectAction::Stop
    });
    let tokens = policy.to_code_literal().to_string();
    assert!(
        tokens.contains("allow only example.com"),
        "custom policy without code should still include the description: {tokens}"
    );
    assert!(
        tokens.contains("custom"),
        "output should mention the 'custom' constructor: {tokens}"
    );
}

#[test]
fn redirect_policy_to_code_literal_custom_with_code_uses_handler_tokens() {
    use elicitation::emit_code::ToCodeLiteral;
    let policy =
        ReqwestRedirectPolicy::custom("stop all redirects", |_attempt| ReqwestRedirectAction::Stop)
            .with_code("|_attempt| ::elicitation::ReqwestRedirectAction::Stop");
    let tokens = policy.to_code_literal().to_string();
    assert!(
        !tokens.contains("compile_error"),
        "custom policy with valid code should not emit compile_error: {tokens}"
    );
    assert!(
        tokens.contains("custom"),
        "output should mention the 'custom' constructor: {tokens}"
    );
}

// ── ReqwestRedirectAttempt ────────────────────────────────────────────────────

#[test]
fn redirect_attempt_snapshot_captures_status_as_typed_code() {
    let url = Url::parse("https://example.com/new").expect("valid URL");
    let prev = Url::parse("https://example.com/old").expect("valid URL");
    let attempt = ReqwestRedirectAttempt::new(
        url.clone(),
        vec![prev.clone()],
        reqwest::StatusCode::MOVED_PERMANENTLY,
    );

    assert_eq!(attempt.url(), &url);
    assert_eq!(attempt.previous(), &[prev]);
    assert_eq!(attempt.status(), reqwest::StatusCode::MOVED_PERMANENTLY);
}

#[test]
fn redirect_attempt_to_code_literal_contains_url_and_status() {
    use elicitation::emit_code::ToCodeLiteral;
    let url = Url::parse("https://example.com/target").expect("valid URL");
    let attempt = ReqwestRedirectAttempt::new(url, vec![], reqwest::StatusCode::FOUND);
    let tokens = attempt.to_code_literal().to_string();
    assert!(
        tokens.contains("example.com"),
        "code literal should contain the URL: {tokens}"
    );
    assert!(
        tokens.contains("302") || tokens.contains("FOUND") || tokens.contains("from_u16"),
        "code literal should encode the status code: {tokens}"
    );
}

// ── ReqwestRedirectAction ─────────────────────────────────────────────────────

#[test]
fn redirect_action_to_code_literal_follow() {
    use elicitation::emit_code::ToCodeLiteral;
    let action = ReqwestRedirectAction::Follow;
    let tokens = action.to_code_literal().to_string();
    assert!(tokens.contains("Follow"), "tokens: {tokens}");
}

#[test]
fn redirect_action_to_code_literal_stop() {
    use elicitation::emit_code::ToCodeLiteral;
    let action = ReqwestRedirectAction::Stop;
    let tokens = action.to_code_literal().to_string();
    assert!(tokens.contains("Stop"), "tokens: {tokens}");
}

#[test]
fn redirect_action_to_code_literal_error_includes_message() {
    use elicitation::emit_code::ToCodeLiteral;
    let action = ReqwestRedirectAction::Error {
        message: "too many hops".to_string(),
    };
    let tokens = action.to_code_literal().to_string();
    assert!(tokens.contains("Error"), "tokens: {tokens}");
    assert!(tokens.contains("too many hops"), "tokens: {tokens}");
}

// ── ReqwestRetryBuilder ───────────────────────────────────────────────────────

#[test]
fn retry_never_builds_raw() {
    let builder = ReqwestRetryBuilder::never();
    let _ = builder.build_raw();
}

#[test]
fn retry_for_host_builds_raw() {
    let builder = ReqwestRetryBuilder::for_host("api.example.com");
    let _ = builder.build_raw();
}

#[test]
fn retry_for_host_with_modifiers_builds_raw() {
    let builder = ReqwestRetryBuilder::for_host("api.example.com")
        .with_max_extra_load(0.3)
        .with_max_retries(5);
    let _ = builder.build_raw();
}

#[test]
fn retry_no_budget_builds_raw() {
    let builder = ReqwestRetryBuilder::for_host("api.example.com").no_budget();
    let _ = builder.build_raw();
}

#[test]
fn retry_never_serializes_and_deserializes() {
    let builder = ReqwestRetryBuilder::never();
    let json = serde_json::to_string(&builder).expect("serializes");
    assert!(json.contains("Never"), "json: {json}");
    let restored: ReqwestRetryBuilder = serde_json::from_str(&json).expect("deserializes");
    let _ = restored.build_raw();
}

#[test]
fn retry_for_host_serializes_and_deserializes() {
    let builder = ReqwestRetryBuilder::for_host("api.example.com")
        .with_max_retries(3)
        .no_budget();
    let json = serde_json::to_string(&builder).expect("serializes");
    assert!(json.contains("api.example.com"), "json: {json}");
    let restored: ReqwestRetryBuilder = serde_json::from_str(&json).expect("deserializes");
    let _ = restored.build_raw();
}

#[test]
fn retry_never_to_code_literal_contains_never() {
    use elicitation::emit_code::ToCodeLiteral;
    let builder = ReqwestRetryBuilder::never();
    let tokens = builder.to_code_literal().to_string();
    assert!(tokens.contains("never"), "tokens: {tokens}");
    assert!(tokens.contains("ReqwestRetryBuilder"), "tokens: {tokens}");
}

#[test]
fn retry_for_host_to_code_literal_contains_host() {
    use elicitation::emit_code::ToCodeLiteral;
    let builder = ReqwestRetryBuilder::for_host("api.example.com");
    let tokens = builder.to_code_literal().to_string();
    assert!(tokens.contains("api.example.com"), "tokens: {tokens}");
    assert!(tokens.contains("for_host"), "tokens: {tokens}");
}

#[test]
fn retry_default_max_retries_not_emitted_in_code_literal() {
    use elicitation::emit_code::ToCodeLiteral;
    let builder = ReqwestRetryBuilder::for_host("api.example.com");
    let tokens = builder.to_code_literal().to_string();
    assert!(
        !tokens.contains("with_max_retries"),
        "default retries should be omitted: {tokens}"
    );
}

#[test]
fn retry_non_default_max_retries_emitted_in_code_literal() {
    use elicitation::emit_code::ToCodeLiteral;
    let builder = ReqwestRetryBuilder::for_host("api.example.com").with_max_retries(5);
    let tokens = builder.to_code_literal().to_string();
    assert!(tokens.contains("with_max_retries"), "tokens: {tokens}");
    assert!(tokens.contains("5"), "tokens: {tokens}");
}

#[test]
fn retry_no_budget_emitted_in_code_literal() {
    use elicitation::emit_code::ToCodeLiteral;
    let builder = ReqwestRetryBuilder::for_host("api.example.com").no_budget();
    let tokens = builder.to_code_literal().to_string();
    assert!(tokens.contains("no_budget"), "tokens: {tokens}");
}

#[test]
fn retry_max_extra_load_emitted_in_code_literal() {
    use elicitation::emit_code::ToCodeLiteral;
    let builder = ReqwestRetryBuilder::for_host("api.example.com").with_max_extra_load(0.5);
    let tokens = builder.to_code_literal().to_string();
    assert!(tokens.contains("with_max_extra_load"), "tokens: {tokens}");
}

#[test]
fn retry_modifiers_are_noop_on_never() {
    use elicitation::emit_code::ToCodeLiteral;
    let builder = ReqwestRetryBuilder::never()
        .no_budget()
        .with_max_extra_load(0.5)
        .with_max_retries(10);
    let tokens = builder.to_code_literal().to_string();
    assert!(tokens.contains("never"), "tokens: {tokens}");
    assert!(!tokens.contains("no_budget"), "tokens: {tokens}");
    assert!(!tokens.contains("with_max_retries"), "tokens: {tokens}");
}
