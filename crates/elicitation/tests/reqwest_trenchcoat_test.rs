#![cfg(feature = "reqwest")]

use elicitation::{
    ReqwestCertificate, ReqwestIdentity, ReqwestNoProxy, ReqwestProxy, ReqwestTlsInfo,
};

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
