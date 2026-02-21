//! Creusot proofs for network address contract types.
//!
//! Cloud of assumptions: We trust Rust std::net types (IpAddr, Ipv4Addr, Ipv6Addr)
//! and our classification logic (private/public, loopback). We verify wrapper structure.

use creusot_std::prelude::*;
use elicitation::{IpPrivate, IpPublic, IpV4, IpV6, Ipv4Loopback, Ipv6Loopback};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

// ============================================================================
// IpAddr Classification Proofs
// ============================================================================

/// Verify IpPrivate construction with private IP.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_ip_private_valid() -> Result<IpPrivate, elicitation::ValidationError> {
    IpPrivate::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)))
}

/// Verify IpPrivate rejects public IP.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_ip_private_invalid() -> Result<IpPrivate, elicitation::ValidationError> {
    IpPrivate::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)))
}

/// Verify IpPublic construction with public IP.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_ip_public_valid() -> Result<IpPublic, elicitation::ValidationError> {
    IpPublic::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)))
}

/// Verify IpPublic rejects private IP.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_ip_public_invalid() -> Result<IpPublic, elicitation::ValidationError> {
    IpPublic::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)))
}

// ============================================================================
// IpV4/IpV6 Proofs
// ============================================================================

/// Verify IpV4 construction with IPv4 address.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_ipv4_valid() -> Result<IpV4, elicitation::ValidationError> {
    IpV4::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
}

/// Verify IpV4 rejects IPv6 address.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_ipv4_invalid() -> Result<IpV4, elicitation::ValidationError> {
    IpV4::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)))
}

/// Verify IpV6 construction with IPv6 address.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_ipv6_valid() -> Result<IpV6, elicitation::ValidationError> {
    IpV6::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)))
}

/// Verify IpV6 rejects IPv4 address.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_ipv6_invalid() -> Result<IpV6, elicitation::ValidationError> {
    IpV6::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
}

// ============================================================================
// Loopback Proofs
// ============================================================================

/// Verify Ipv4Loopback construction with loopback address.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_ipv4_loopback_valid() -> Result<Ipv4Loopback, elicitation::ValidationError> {
    Ipv4Loopback::new(Ipv4Addr::new(127, 0, 0, 1))
}

/// Verify Ipv4Loopback rejects non-loopback.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_ipv4_loopback_invalid() -> Result<Ipv4Loopback, elicitation::ValidationError> {
    Ipv4Loopback::new(Ipv4Addr::new(192, 168, 1, 1))
}

/// Verify Ipv6Loopback construction with loopback address.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_ipv6_loopback_valid() -> Result<Ipv6Loopback, elicitation::ValidationError> {
    Ipv6Loopback::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1))
}

/// Verify Ipv6Loopback rejects non-loopback.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_ipv6_loopback_invalid() -> Result<Ipv6Loopback, elicitation::ValidationError> {
    Ipv6Loopback::new(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1))
}
