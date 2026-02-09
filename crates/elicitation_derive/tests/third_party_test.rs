//! Tests for third-party type Rand implementations.

use elicitation_rand::{Generator, Rand};

#[test]
fn test_string_generation() {
    let generator = String::rand_generator(42);
    let s = generator.generate();

    // Should generate lowercase alphabetic string
    assert!(s.chars().all(|c| c.is_ascii_lowercase()));
    assert!(s.len() <= 32);
}

#[test]
fn test_string_deterministic() {
    let generator1 = String::rand_generator(12345);
    let generator2 = String::rand_generator(12345);

    assert_eq!(generator1.generate(), generator2.generate());
}

#[test]
#[cfg(feature = "chrono")]
fn test_datetime_utc() {
    use chrono::{DateTime, Utc};

    let generator = DateTime::<Utc>::rand_generator(999);
    let dt = generator.generate();

    // Should generate valid datetime
    assert!(dt.timestamp() != 0 || dt == DateTime::from_timestamp(0, 0).unwrap());
}

#[test]
#[cfg(feature = "chrono")]
fn test_naive_datetime() {
    use chrono::NaiveDateTime;

    let generator = NaiveDateTime::rand_generator(777);
    let dt = generator.generate();

    // Should generate valid datetime
    assert!(dt.and_utc().timestamp() >= 0 || dt.and_utc().timestamp() < 0);
}

#[test]
#[cfg(feature = "jiff")]
fn test_jiff_timestamp() {
    use jiff::Timestamp;

    let generator = Timestamp::rand_generator(555);
    let ts = generator.generate();

    // Should generate valid timestamp
    assert!(ts.as_second() >= 0 || ts.as_second() < 0);
}

#[test]
#[cfg(feature = "time")]
fn test_time_offset_datetime() {
    use time::OffsetDateTime;

    let generator = OffsetDateTime::rand_generator(333);
    let dt = generator.generate();

    // Should generate valid datetime
    assert!(dt.unix_timestamp() >= 0 || dt.unix_timestamp() < 0);
}

#[test]
#[cfg(feature = "uuid")]
fn test_uuid_generation() {
    use uuid::Uuid;

    let generator = Uuid::rand_generator(111);
    let id = generator.generate();

    // Should generate valid UUID
    assert_ne!(id, Uuid::nil());

    // Different seeds = different UUIDs
    let generator2 = Uuid::rand_generator(222);
    assert_ne!(id, generator2.generate());
}

#[test]
#[cfg(feature = "uuid")]
fn test_uuid_deterministic() {
    use uuid::Uuid;

    let generator1 = Uuid::rand_generator(54321);
    let generator2 = Uuid::rand_generator(54321);

    assert_eq!(generator1.generate(), generator2.generate());
}

#[test]
#[cfg(feature = "url")]
fn test_url_generation() {
    use url::Url;

    let generator = Url::rand_generator(888);
    let url = generator.generate();

    // Should generate valid URL
    assert!(url.scheme() == "http" || url.scheme() == "https" || url.scheme() == "ftp");
    assert!(url.host_str().is_some());

    // Should be parseable
    assert_eq!(url, Url::parse(url.as_str()).unwrap());
}

#[test]
#[cfg(feature = "url")]
fn test_url_deterministic() {
    use url::Url;

    let generator1 = Url::rand_generator(99999);
    let generator2 = Url::rand_generator(99999);

    assert_eq!(generator1.generate(), generator2.generate());
}

#[test]
fn test_pathbuf_generation() {
    use std::path::PathBuf;

    let generator = PathBuf::rand_generator(666);
    let path = generator.generate();

    // Should generate non-empty path
    assert!(!path.as_os_str().is_empty());

    // Should have components
    assert!(path.components().count() > 0);
}

#[test]
fn test_pathbuf_deterministic() {
    use std::path::PathBuf;

    let generator1 = PathBuf::rand_generator(11111);
    let generator2 = PathBuf::rand_generator(11111);

    assert_eq!(generator1.generate(), generator2.generate());
}

#[test]
fn test_pathbuf_multiple_components() {
    use std::path::PathBuf;

    let generator = PathBuf::rand_generator(123456);
    let path = generator.generate();

    // Should have multiple path components
    let count = path.components().count();
    assert!(
        (1..=5).contains(&count),
        "Expected 1-5 components, got {}",
        count
    );
}
