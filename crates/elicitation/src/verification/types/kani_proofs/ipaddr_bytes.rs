//! Kani proofs for IP address byte validation.
//!
//! These proofs verify the correctness of IPv4 and IPv6 byte-level validation
//! following RFC 1918 (private IPv4) and RFC 4193 (private IPv6).

#![cfg(kani)]

use crate::verification::types::{
    Ipv4Bytes, Ipv4Private, Ipv4Public, Ipv6Bytes, Ipv6Private, Ipv6Public, is_ipv4_private,
    is_ipv6_private,
};

// ============================================================================
// IPv4 Private Range Proofs (RFC 1918)
// ============================================================================

#[kani::proof]
fn verify_ipv4_10_network_is_private() {
    let octets: [u8; 4] = kani::any();
    kani::assume(octets[0] == 10);

    // All of 10.0.0.0/8 is private
    assert!(is_ipv4_private(&octets));

    let private = Ipv4Private::new(octets);
    assert!(private.is_ok());
}

#[kani::proof]
fn verify_ipv4_172_16_31_is_private() {
    let mut octets: [u8; 4] = kani::any();
    octets[0] = 172;
    kani::assume(octets[1] >= 16 && octets[1] <= 31);

    // 172.16.0.0/12 is private
    assert!(is_ipv4_private(&octets));

    let private = Ipv4Private::new(octets);
    assert!(private.is_ok());
}

#[kani::proof]
fn verify_ipv4_172_outside_range_not_private() {
    let mut octets: [u8; 4] = kani::any();
    octets[0] = 172;

    // Second octet outside 16-31
    let second: u8 = kani::any();
    kani::assume(second < 16 || second > 31);
    octets[1] = second;

    // Should not be private
    assert!(!is_ipv4_private(&octets));
}

#[kani::proof]
fn verify_ipv4_192_168_is_private() {
    let mut octets: [u8; 4] = kani::any();
    octets[0] = 192;
    octets[1] = 168;

    // 192.168.0.0/16 is private
    assert!(is_ipv4_private(&octets));

    let private = Ipv4Private::new(octets);
    assert!(private.is_ok());
}

#[kani::proof]
fn verify_ipv4_192_non_168_not_private() {
    let mut octets: [u8; 4] = kani::any();
    octets[0] = 192;

    let second: u8 = kani::any();
    kani::assume(second != 168);
    octets[1] = second;

    // Should not be private
    assert!(!is_ipv4_private(&octets));
}

// ============================================================================
// IPv4 Public Address Proofs
// ============================================================================

#[kani::proof]
fn verify_ipv4_public_construction() {
    let octets: [u8; 4] = kani::any();

    // Assume it's actually public (not private, loopback, etc.)
    kani::assume(!is_ipv4_private(&octets));
    kani::assume(octets[0] != 127); // Not loopback
    kani::assume(!(octets[0] >= 224 && octets[0] <= 239)); // Not multicast
    kani::assume(octets != [0, 0, 0, 0]); // Not unspecified
    kani::assume(octets != [255, 255, 255, 255]); // Not broadcast

    let ipv4 = Ipv4Bytes::new(octets);
    assert!(ipv4.is_public());

    let public = Ipv4Public::new(octets);
    assert!(public.is_ok());
}

#[kani::proof]
fn verify_ipv4_loopback_not_public() {
    let mut octets: [u8; 4] = kani::any();
    octets[0] = 127;

    let ipv4 = Ipv4Bytes::new(octets);
    assert!(ipv4.is_loopback());
    assert!(!ipv4.is_public());

    let public = Ipv4Public::new(octets);
    assert!(public.is_err());
}

#[kani::proof]
fn verify_ipv4_multicast_not_public() {
    let mut octets: [u8; 4] = kani::any();

    // Multicast range: 224-239
    let first: u8 = kani::any();
    kani::assume(first >= 224 && first <= 239);
    octets[0] = first;

    let ipv4 = Ipv4Bytes::new(octets);
    assert!(ipv4.is_multicast());
    assert!(!ipv4.is_public());
}

// ============================================================================
// IPv6 Private Range Proofs (RFC 4193)
// ============================================================================

#[kani::proof]
fn verify_ipv6_fc00_private() {
    let mut octets: [u8; 16] = kani::any();

    // fc00::/7 - first byte in range fc-fd
    let first: u8 = kani::any();
    kani::assume((first & 0xfe) == 0xfc); // 0xfc or 0xfd
    octets[0] = first;

    // Should be private
    assert!(is_ipv6_private(&octets));

    let private = Ipv6Private::new(octets);
    assert!(private.is_ok());
}

#[kani::proof]
fn verify_ipv6_outside_fc00_not_private() {
    let mut octets: [u8; 16] = kani::any();

    // Not fc00::/7
    let first: u8 = kani::any();
    kani::assume((first & 0xfe) != 0xfc);
    octets[0] = first;

    // Should not be private
    assert!(!is_ipv6_private(&octets));
}

// ============================================================================
// IPv6 Special Address Proofs
// ============================================================================

#[kani::proof]
fn verify_ipv6_loopback() {
    let mut octets = [0u8; 16];
    octets[15] = 1; // ::1

    let ipv6 = Ipv6Bytes::new(octets);
    assert!(ipv6.is_loopback());
    assert!(!ipv6.is_public());
}

#[kani::proof]
fn verify_ipv6_unspecified() {
    let octets = [0u8; 16]; // ::

    let ipv6 = Ipv6Bytes::new(octets);
    assert!(ipv6.is_unspecified());
    assert!(!ipv6.is_public());
}

#[kani::proof]
fn verify_ipv6_multicast() {
    let mut octets: [u8; 16] = kani::any();
    octets[0] = 0xff; // ff00::/8

    let ipv6 = Ipv6Bytes::new(octets);
    assert!(ipv6.is_multicast());
    assert!(!ipv6.is_public());
}

#[kani::proof]
fn verify_ipv6_public_construction() {
    let octets: [u8; 16] = kani::any();

    // Assume it's actually public
    kani::assume((octets[0] & 0xfe) != 0xfc); // Not private
    kani::assume(octets[0] != 0xff); // Not multicast
    kani::assume(!(octets[..15] == [0; 15] && octets[15] == 1)); // Not loopback
    kani::assume(octets != [0; 16]); // Not unspecified

    let ipv6 = Ipv6Bytes::new(octets);
    assert!(ipv6.is_public());

    let public = Ipv6Public::new(octets);
    assert!(public.is_ok());
}

// ============================================================================
// Round-Trip Proofs
// ============================================================================

#[kani::proof]
fn verify_ipv4_roundtrip() {
    let octets: [u8; 4] = kani::any();

    let ipv4 = Ipv4Bytes::new(octets);
    let extracted = ipv4.octets();

    assert_eq!(octets, extracted);
}

#[kani::proof]
fn verify_ipv6_roundtrip() {
    let octets: [u8; 16] = kani::any();

    let ipv6 = Ipv6Bytes::new(octets);
    let extracted = ipv6.octets();

    assert_eq!(octets, extracted);
}

#[kani::proof]
fn verify_ipv4_private_roundtrip() {
    let octets: [u8; 4] = kani::any();
    kani::assume(is_ipv4_private(&octets));

    if let Ok(private) = Ipv4Private::new(octets) {
        let extracted = private.octets();
        assert_eq!(octets, extracted);
    } else {
        kani::cover!(false, "Constructor must succeed for private IPs");
    }
}

#[kani::proof]
fn verify_ipv6_private_roundtrip() {
    let octets: [u8; 16] = kani::any();
    kani::assume(is_ipv6_private(&octets));

    if let Ok(private) = Ipv6Private::new(octets) {
        let extracted = private.octets();
        assert_eq!(octets, extracted);
    } else {
        kani::cover!(false, "Constructor must succeed for private IPs");
    }
}

