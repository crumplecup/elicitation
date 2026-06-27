//! Goal 1: Round-trip fidelity tests for `ClientBuilder`.
//!
//! Serializes a recipe, deserializes it, calls `build()`, and verifies the
//! result is a functionally usable client.

use elicit_reqwest::{ClientBuilder, HttpClient, Url};

fn example_url() -> Url {
    "https://example.com/api".parse().expect("valid URL")
}

#[test]
fn default_builder_round_trips() {
    let original = ClientBuilder::new();
    let json = serde_json::to_string(&original).expect("serializes");
    let restored: ClientBuilder = serde_json::from_str(&json).expect("deserializes");
    let json2 = serde_json::to_string(&restored).expect("re-serializes");
    assert_eq!(json, json2);
}

#[test]
fn builder_with_user_agent_round_trips() {
    let original = ClientBuilder::new().user_agent("elicit-test/1.0");
    let json = serde_json::to_string(&original).expect("serializes");
    let restored: ClientBuilder = serde_json::from_str(&json).expect("deserializes");
    let json2 = serde_json::to_string(&restored).expect("re-serializes");
    assert_eq!(json, json2);
}

#[test]
fn builder_with_timeout_round_trips() {
    let original = ClientBuilder::new().timeout(std::time::Duration::from_secs(30));
    let json = serde_json::to_string(&original).expect("serializes");
    let restored: ClientBuilder = serde_json::from_str(&json).expect("deserializes");
    let json2 = serde_json::to_string(&restored).expect("re-serializes");
    assert_eq!(json, json2);
}

#[test]
fn builder_with_all_timeouts_round_trips() {
    let original = ClientBuilder::new()
        .user_agent("elicit-test/1.0")
        .timeout(std::time::Duration::from_secs(30))
        .connect_timeout(std::time::Duration::from_secs(10))
        .pool_max_idle_per_host(4);
    let json = serde_json::to_string(&original).expect("serializes");
    let restored: ClientBuilder = serde_json::from_str(&json).expect("deserializes");
    let json2 = serde_json::to_string(&restored).expect("re-serializes");
    assert_eq!(json, json2);
}

#[test]
fn build_after_round_trip_produces_usable_client() {
    let original = ClientBuilder::new()
        .user_agent("elicit-test/1.0")
        .timeout(std::time::Duration::from_secs(30));
    let json = serde_json::to_string(&original).expect("serializes");
    let restored: ClientBuilder = serde_json::from_str(&json).expect("deserializes");
    let client = restored.build().expect("builds without error");

    let url = example_url();
    let request = client.get_url(url.clone()).build().expect("valid request");
    assert_eq!(request.method().as_str(), "GET");
    assert_eq!(request.url(), &url);
}

#[test]
fn build_with_compression_flags_round_trips() {
    let original = ClientBuilder::new().gzip(true).brotli(false);
    let json = serde_json::to_string(&original).expect("serializes");
    let restored: ClientBuilder = serde_json::from_str(&json).expect("deserializes");
    let json2 = serde_json::to_string(&restored).expect("re-serializes");
    assert_eq!(json, json2);
    restored.build().expect("builds without error");
}

#[test]
fn build_with_https_only_round_trips() {
    let original = ClientBuilder::new().https_only(true).tcp_nodelay(true);
    let json = serde_json::to_string(&original).expect("serializes");
    let restored: ClientBuilder = serde_json::from_str(&json).expect("deserializes");
    let json2 = serde_json::to_string(&restored).expect("re-serializes");
    assert_eq!(json, json2);
    restored.build().expect("builds without error");
}
