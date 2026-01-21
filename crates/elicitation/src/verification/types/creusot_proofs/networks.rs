//! Creusot proofs for network contract types.

#![cfg(feature = "verify-creusot")]
#![allow(unused_imports)]

use crate::*;
use creusot_contracts::*;

// Network Contract Proofs
// ============================================================================

/// Prove that IpPrivate construction succeeds for private IPs.
#[cfg(feature = "verification")]
#[requires(value.is_private())]
#[ensures(result.is_ok())]
pub fn verify_ip_private_valid(value: std::net::IpAddr) -> Result<IpPrivate, ValidationError> {
    IpPrivate::new(value)
}

/// Prove that IpPublic construction succeeds for non-private IPs.
#[cfg(feature = "verification")]
#[requires(!value.is_private())]
#[ensures(result.is_ok())]
pub fn verify_ip_public_valid(value: std::net::IpAddr) -> Result<IpPublic, ValidationError> {
    IpPublic::new(value)
}

/// Prove that IpV4 construction succeeds for IPv4 addresses.
#[cfg(feature = "verification")]
#[requires(value.is_ipv4())]
#[ensures(result.is_ok())]
pub fn verify_ip_v4_valid(value: std::net::IpAddr) -> Result<IpV4, ValidationError> {
    IpV4::new(value)
}

/// Prove that IpV6 construction succeeds for IPv6 addresses.
#[cfg(feature = "verification")]
#[requires(value.is_ipv6())]
#[ensures(result.is_ok())]
pub fn verify_ip_v6_valid(value: std::net::IpAddr) -> Result<IpV6, ValidationError> {
    IpV6::new(value)
}

/// Prove that Ipv4Loopback construction succeeds for IPv4 loopback.
#[cfg(feature = "verification")]
#[requires(value.is_loopback())]
#[ensures(result.is_ok())]
pub fn verify_ipv4_loopback_valid(value: std::net::Ipv4Addr) -> Result<Ipv4Loopback, ValidationError> {
    Ipv4Loopback::new(value)
}

/// Prove that Ipv6Loopback construction succeeds for IPv6 loopback.
#[cfg(feature = "verification")]
#[requires(value.is_loopback())]
#[ensures(result.is_ok())]
pub fn verify_ipv6_loopback_valid(value: std::net::Ipv6Addr) -> Result<Ipv6Loopback, ValidationError> {
    Ipv6Loopback::new(value)
}

// ============================================================================
