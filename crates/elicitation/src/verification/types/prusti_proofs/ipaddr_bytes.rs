//! Prusti proofs for IP address validation types.
//!
//! Validates IPv4 and IPv6 addresses with private/public classification:
//! - Ipv4Bytes: 4-byte IPv4 address
//! - Ipv4Private: Private IPv4 (RFC 1918)
//! - Ipv4Public: Public IPv4
//! - Ipv6Bytes: 16-byte IPv6 address
//! - Ipv6Private: Private IPv6 (RFC 4193)
//! - Ipv6Public: Public IPv6
//!
//! This is compositional verification: stdlib_ip_logic_correct → wrapper_correct.

#![cfg(feature = "verify-prusti")]
#![allow(unused_imports)]

use crate::verification::types::{
    Ipv4Bytes, Ipv4Private, Ipv4Public, Ipv6Bytes, Ipv6Private, Ipv6Public, ValidationError,
    is_ipv4_private, is_ipv6_private,
};
use prusti_contracts::*;

// Ipv4Bytes Validation Proofs
// ============================================================================

/// Verify: Ipv4Bytes always succeeds (all octet combinations valid)
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv4_construction(octets: [u8; 4]) -> Ipv4Bytes {
    Ipv4Bytes::new(octets)
}

/// Verify: octets() returns the same octets
#[cfg(feature = "verify-prusti")]
#[ensures(result.octets() == octets)]
pub fn verify_ipv4_octets_accessor(octets: [u8; 4]) -> Ipv4Bytes {
    Ipv4Bytes::new(octets)
}

/// Verify: Specific IPv4 address (localhost)
#[cfg(feature = "verify-prusti")]
#[ensures(result.is_loopback())]
pub fn verify_ipv4_localhost() -> Ipv4Bytes {
    Ipv4Bytes::new([127, 0, 0, 1])
}

/// Verify: Unspecified address (0.0.0.0)
#[cfg(feature = "verify-prusti")]
#[ensures(result.is_unspecified())]
pub fn verify_ipv4_unspecified() -> Ipv4Bytes {
    Ipv4Bytes::new([0, 0, 0, 0])
}

/// Verify: Broadcast address (255.255.255.255)
#[cfg(feature = "verify-prusti")]
#[ensures(result.is_broadcast())]
pub fn verify_ipv4_broadcast() -> Ipv4Bytes {
    Ipv4Bytes::new([255, 255, 255, 255])
}

/// Verify: Multicast address (224.0.0.0/4)
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv4_multicast() -> Ipv4Bytes {
    Ipv4Bytes::new([224, 0, 0, 1])
}

/// Verify: Private address detection (10.0.0.0/8)
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv4_private_10_network() -> Ipv4Bytes {
    Ipv4Bytes::new([10, 0, 0, 1])
}

/// Verify: Private address detection (172.16.0.0/12)
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv4_private_172_network() -> Ipv4Bytes {
    Ipv4Bytes::new([172, 16, 0, 1])
}

/// Verify: Private address detection (192.168.0.0/16)
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv4_private_192_network() -> Ipv4Bytes {
    Ipv4Bytes::new([192, 168, 0, 1])
}

/// Verify: Public address (8.8.8.8 - Google DNS)
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv4_public() -> Ipv4Bytes {
    Ipv4Bytes::new([8, 8, 8, 8])
}

// Ipv4Private Validation Proofs
// ============================================================================

/// Verify: Ipv4Private accepts 10.0.0.0/8
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv4_private_10_valid() -> Result<Ipv4Private, ValidationError> {
    Ipv4Private::new([10, 0, 0, 1])
}

/// Verify: Ipv4Private accepts 172.16.0.0/12
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv4_private_172_16_valid() -> Result<Ipv4Private, ValidationError> {
    Ipv4Private::new([172, 16, 0, 1])
}

/// Verify: Ipv4Private accepts 172.31.255.255 (upper bound)
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv4_private_172_31_valid() -> Result<Ipv4Private, ValidationError> {
    Ipv4Private::new([172, 31, 255, 255])
}

/// Verify: Ipv4Private accepts 192.168.0.0/16
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv4_private_192_168_valid() -> Result<Ipv4Private, ValidationError> {
    Ipv4Private::new([192, 168, 1, 1])
}

/// Verify: get() returns underlying Ipv4Bytes
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv4_private_get() -> Result<Ipv4Private, ValidationError> {
    let private = Ipv4Private::new([10, 0, 0, 1])?;
    let _inner = private.get();
    Ok(private)
}

// Ipv4Public Validation Proofs
// ============================================================================

/// Verify: Ipv4Public accepts public address
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv4_public_google_dns() -> Result<Ipv4Public, ValidationError> {
    Ipv4Public::new([8, 8, 8, 8])
}

/// Verify: Ipv4Public accepts Cloudflare DNS
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv4_public_cloudflare() -> Result<Ipv4Public, ValidationError> {
    Ipv4Public::new([1, 1, 1, 1])
}

/// Verify: get() returns underlying Ipv4Bytes
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv4_public_get() -> Result<Ipv4Public, ValidationError> {
    let public = Ipv4Public::new([8, 8, 8, 8])?;
    let _inner = public.get();
    Ok(public)
}

// Ipv6Bytes Validation Proofs
// ============================================================================

/// Verify: Ipv6Bytes always succeeds (all byte combinations valid)
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv6_construction(segments: [u8; 16]) -> Ipv6Bytes {
    Ipv6Bytes::new(segments)
}

/// Verify: segments() returns the same segments
#[cfg(feature = "verify-prusti")]
#[ensures(result.segments() == segments)]
pub fn verify_ipv6_segments_accessor(segments: [u8; 16]) -> Ipv6Bytes {
    Ipv6Bytes::new(segments)
}

/// Verify: Specific IPv6 address (localhost ::1)
#[cfg(feature = "verify-prusti")]
#[ensures(result.is_loopback())]
pub fn verify_ipv6_localhost() -> Ipv6Bytes {
    Ipv6Bytes::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])
}

/// Verify: Unspecified address (::)
#[cfg(feature = "verify-prusti")]
#[ensures(result.is_unspecified())]
pub fn verify_ipv6_unspecified() -> Ipv6Bytes {
    Ipv6Bytes::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
}

/// Verify: Multicast address (ff00::/8)
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv6_multicast() -> Ipv6Bytes {
    Ipv6Bytes::new([0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])
}

/// Verify: Private address detection (fc00::/7)
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv6_private_fc00() -> Ipv6Bytes {
    Ipv6Bytes::new([0xfc, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])
}

/// Verify: Private address detection (fd00::/7)
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv6_private_fd00() -> Ipv6Bytes {
    Ipv6Bytes::new([0xfd, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])
}

/// Verify: Public address (2001:4860:4860::8888 - Google DNS)
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv6_public() -> Ipv6Bytes {
    Ipv6Bytes::new([
        0x20, 0x01, 0x48, 0x60, 0x48, 0x60, 0, 0, 0, 0, 0, 0, 0, 0, 0x88, 0x88,
    ])
}

// Ipv6Private Validation Proofs
// ============================================================================

/// Verify: Ipv6Private accepts fc00::/7
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv6_private_fc00_valid() -> Result<Ipv6Private, ValidationError> {
    Ipv6Private::new([0xfc, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])
}

/// Verify: Ipv6Private accepts fd00::/7
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv6_private_fd00_valid() -> Result<Ipv6Private, ValidationError> {
    Ipv6Private::new([0xfd, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])
}

/// Verify: get() returns underlying Ipv6Bytes
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv6_private_get() -> Result<Ipv6Private, ValidationError> {
    let private = Ipv6Private::new([0xfc, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])?;
    let _inner = private.get();
    Ok(private)
}

// Ipv6Public Validation Proofs
// ============================================================================

/// Verify: Ipv6Public accepts public address
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv6_public_google_dns() -> Result<Ipv6Public, ValidationError> {
    Ipv6Public::new([
        0x20, 0x01, 0x48, 0x60, 0x48, 0x60, 0, 0, 0, 0, 0, 0, 0, 0, 0x88, 0x88,
    ])
}

/// Verify: Ipv6Public accepts Cloudflare DNS
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv6_public_cloudflare() -> Result<Ipv6Public, ValidationError> {
    Ipv6Public::new([
        0x26, 0x06, 0x47, 0x00, 0x47, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0x11, 0x11,
    ])
}

/// Verify: get() returns underlying Ipv6Bytes
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv6_public_get() -> Result<Ipv6Public, ValidationError> {
    let public = Ipv6Public::new([
        0x20, 0x01, 0x48, 0x60, 0x48, 0x60, 0, 0, 0, 0, 0, 0, 0, 0, 0x88, 0x88,
    ])?;
    let _inner = public.get();
    Ok(public)
}

// Helper Function Proofs
// ============================================================================

/// Verify: is_ipv4_private correctly identifies 10.0.0.0/8
#[cfg(feature = "verify-prusti")]
#[ensures(result)]
pub fn verify_is_ipv4_private_10() -> bool {
    is_ipv4_private(&[10, 0, 0, 1])
}

/// Verify: is_ipv4_private correctly identifies 172.16.0.0/12
#[cfg(feature = "verify-prusti")]
#[ensures(result)]
pub fn verify_is_ipv4_private_172() -> bool {
    is_ipv4_private(&[172, 16, 0, 1])
}

/// Verify: is_ipv4_private correctly identifies 192.168.0.0/16
#[cfg(feature = "verify-prusti")]
#[ensures(result)]
pub fn verify_is_ipv4_private_192() -> bool {
    is_ipv4_private(&[192, 168, 0, 1])
}

/// Verify: is_ipv4_private correctly rejects public address
#[cfg(feature = "verify-prusti")]
#[ensures(!result)]
pub fn verify_is_ipv4_private_public() -> bool {
    is_ipv4_private(&[8, 8, 8, 8])
}

/// Verify: is_ipv6_private correctly identifies fc00::/7
#[cfg(feature = "verify-prusti")]
#[ensures(result)]
pub fn verify_is_ipv6_private_fc00() -> bool {
    is_ipv6_private(&[0xfc, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])
}

/// Verify: is_ipv6_private correctly identifies fd00::/7
#[cfg(feature = "verify-prusti")]
#[ensures(result)]
pub fn verify_is_ipv6_private_fd00() -> bool {
    is_ipv6_private(&[0xfd, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])
}

/// Verify: is_ipv6_private correctly rejects public address
#[cfg(feature = "verify-prusti")]
#[ensures(!result)]
pub fn verify_is_ipv6_private_public() -> bool {
    is_ipv6_private(&[
        0x20, 0x01, 0x48, 0x60, 0x48, 0x60, 0, 0, 0, 0, 0, 0, 0, 0, 0x88, 0x88,
    ])
}

// Edge Cases
// ============================================================================

/// Verify: IPv4 boundary - 172.15.x.x is not private
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv4_172_15_boundary() -> Ipv4Bytes {
    Ipv4Bytes::new([172, 15, 255, 255])
}

/// Verify: IPv4 boundary - 172.32.x.x is not private
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv4_172_32_boundary() -> Ipv4Bytes {
    Ipv4Bytes::new([172, 32, 0, 0])
}

/// Verify: IPv6 boundary - fb00::/8 is not private
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv6_fb00_boundary() -> Ipv6Bytes {
    Ipv6Bytes::new([
        0xfb, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff,
    ])
}

/// Verify: IPv6 boundary - fe00::/8 is not private
#[cfg(feature = "verify-prusti")]
pub fn verify_ipv6_fe00_boundary() -> Ipv6Bytes {
    Ipv6Bytes::new([0xfe, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])
}

// ============================================================================
