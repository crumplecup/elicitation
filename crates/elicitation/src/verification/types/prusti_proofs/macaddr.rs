//! Prusti proofs for MAC address validation types.
//!
//! Validates MAC addresses (EUI-48):
//! - MacAddr: 6-byte MAC address
//! - Unicast/multicast classification
//! - Universal/local classification
//! - Special addresses (broadcast, null)
//!
//! This is compositional verification: bit_logic_correct → wrapper_correct.

#![cfg(feature = "verify-prusti")]
#![allow(unused_imports)]

use crate::verification::types::{
    MacAddr, ValidationError, is_multicast, is_unicast, is_universal,
};
use prusti_contracts::*;

// MacAddr Validation Proofs
// ============================================================================

/// Verify: MacAddr construction always succeeds
#[cfg(prusti)]
pub fn verify_mac_construction(octets: [u8; 6]) -> MacAddr {
    MacAddr::new(octets)
}

/// Verify: octets() returns the same octets
#[cfg(prusti)]
#[ensures(result.octets() == octets)]
pub fn verify_mac_octets_accessor(octets: [u8; 6]) -> MacAddr {
    MacAddr::new(octets)
}

/// Verify: Unicast universal address (bit 0 = 0, bit 1 = 0)
#[cfg(prusti)]
#[ensures(result.is_unicast())]
#[ensures(result.is_universal())]
pub fn verify_mac_unicast_universal() -> MacAddr {
    MacAddr::new([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E])
}

/// Verify: Multicast universal address (bit 0 = 1, bit 1 = 0)
#[cfg(prusti)]
#[ensures(result.is_multicast())]
#[ensures(result.is_universal())]
pub fn verify_mac_multicast_universal() -> MacAddr {
    MacAddr::new([0x01, 0x00, 0x5E, 0x00, 0x00, 0x01])
}

/// Verify: Unicast local address (bit 0 = 0, bit 1 = 1)
#[cfg(prusti)]
#[ensures(result.is_unicast())]
#[ensures(result.is_local())]
pub fn verify_mac_unicast_local() -> MacAddr {
    MacAddr::new([0x02, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E])
}

/// Verify: Multicast local address (bit 0 = 1, bit 1 = 1)
#[cfg(prusti)]
#[ensures(result.is_multicast())]
#[ensures(result.is_local())]
pub fn verify_mac_multicast_local() -> MacAddr {
    MacAddr::new([0x03, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E])
}

/// Verify: Broadcast address (FF:FF:FF:FF:FF:FF)
#[cfg(prusti)]
#[ensures(result.is_broadcast())]
#[ensures(result.is_multicast())]
pub fn verify_mac_broadcast() -> MacAddr {
    MacAddr::new([0xFF; 6])
}

/// Verify: Null address (00:00:00:00:00:00)
#[cfg(prusti)]
#[ensures(result.is_null())]
#[ensures(result.is_unicast())]
#[ensures(result.is_universal())]
pub fn verify_mac_null() -> MacAddr {
    MacAddr::new([0x00; 6])
}

/// Verify: Common vendor MAC (Intel OUI 00:1B:21)
#[cfg(prusti)]
#[ensures(result.is_unicast())]
#[ensures(result.is_universal())]
pub fn verify_mac_intel_oui() -> MacAddr {
    MacAddr::new([0x00, 0x1B, 0x21, 0x12, 0x34, 0x56])
}

/// Verify: Common vendor MAC (Cisco OUI 00:1E:14)
#[cfg(prusti)]
#[ensures(result.is_unicast())]
#[ensures(result.is_universal())]
pub fn verify_mac_cisco_oui() -> MacAddr {
    MacAddr::new([0x00, 0x1E, 0x14, 0xAB, 0xCD, 0xEF])
}

// Helper Function Proofs
// ============================================================================

/// Verify: is_unicast correctly identifies unicast (bit 0 = 0)
#[cfg(prusti)]
#[ensures(result)]
pub fn verify_is_unicast_00() -> bool {
    is_unicast(&[0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E])
}

/// Verify: is_unicast correctly identifies unicast (bit 0 = 0, bit 1 = 1)
#[cfg(prusti)]
#[ensures(result)]
pub fn verify_is_unicast_02() -> bool {
    is_unicast(&[0x02, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E])
}

/// Verify: is_multicast correctly identifies multicast (bit 0 = 1)
#[cfg(prusti)]
#[ensures(result)]
pub fn verify_is_multicast_01() -> bool {
    is_multicast(&[0x01, 0x00, 0x5E, 0x00, 0x00, 0x01])
}

/// Verify: is_multicast correctly identifies multicast (bit 0 = 1, bit 1 = 1)
#[cfg(prusti)]
#[ensures(result)]
pub fn verify_is_multicast_03() -> bool {
    is_multicast(&[0x03, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E])
}

/// Verify: is_multicast correctly identifies broadcast
#[cfg(prusti)]
#[ensures(result)]
pub fn verify_is_multicast_broadcast() -> bool {
    is_multicast(&[0xFF; 6])
}

/// Verify: is_universal correctly identifies universal (bit 1 = 0)
#[cfg(prusti)]
#[ensures(result)]
pub fn verify_is_universal_00() -> bool {
    is_universal(&[0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E])
}

/// Verify: is_universal correctly identifies universal (bit 0 = 1, bit 1 = 0)
#[cfg(prusti)]
#[ensures(result)]
pub fn verify_is_universal_01() -> bool {
    is_universal(&[0x01, 0x00, 0x5E, 0x00, 0x00, 0x01])
}

// Bit Manipulation Edge Cases
// ============================================================================

/// Verify: Even octets are unicast (bit 0 = 0)
#[cfg(prusti)]
#[ensures(result.is_unicast())]
pub fn verify_mac_even_first_octet() -> MacAddr {
    MacAddr::new([0x04, 0x00, 0x00, 0x00, 0x00, 0x00])
}

/// Verify: Odd octets are multicast (bit 0 = 1)
#[cfg(prusti)]
#[ensures(result.is_multicast())]
pub fn verify_mac_odd_first_octet() -> MacAddr {
    MacAddr::new([0x05, 0x00, 0x00, 0x00, 0x00, 0x00])
}

/// Verify: 0x00 is unicast universal
#[cfg(prusti)]
#[ensures(result.is_unicast())]
#[ensures(result.is_universal())]
pub fn verify_mac_00_unicast_universal() -> MacAddr {
    MacAddr::new([0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
}

/// Verify: 0x01 is multicast universal
#[cfg(prusti)]
#[ensures(result.is_multicast())]
#[ensures(result.is_universal())]
pub fn verify_mac_01_multicast_universal() -> MacAddr {
    MacAddr::new([0x01, 0x00, 0x00, 0x00, 0x00, 0x00])
}

/// Verify: 0x02 is unicast local
#[cfg(prusti)]
#[ensures(result.is_unicast())]
#[ensures(result.is_local())]
pub fn verify_mac_02_unicast_local() -> MacAddr {
    MacAddr::new([0x02, 0x00, 0x00, 0x00, 0x00, 0x00])
}

/// Verify: 0x03 is multicast local
#[cfg(prusti)]
#[ensures(result.is_multicast())]
#[ensures(result.is_local())]
pub fn verify_mac_03_multicast_local() -> MacAddr {
    MacAddr::new([0x03, 0x00, 0x00, 0x00, 0x00, 0x00])
}

// All Max/Min Edge Cases
// ============================================================================

/// Verify: All zeros (null address)
#[cfg(prusti)]
#[ensures(result.is_null())]
pub fn verify_mac_all_zeros() -> MacAddr {
    MacAddr::new([0x00; 6])
}

/// Verify: All ones (broadcast)
#[cfg(prusti)]
#[ensures(result.is_broadcast())]
pub fn verify_mac_all_ones() -> MacAddr {
    MacAddr::new([0xFF; 6])
}

/// Verify: Alternating pattern
#[cfg(prusti)]
#[ensures(result.is_multicast())]
pub fn verify_mac_alternating() -> MacAddr {
    MacAddr::new([0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00])
}

/// Verify: Sequential values
#[cfg(prusti)]
#[ensures(result.is_unicast())]
pub fn verify_mac_sequential() -> MacAddr {
    MacAddr::new([0x00, 0x01, 0x02, 0x03, 0x04, 0x05])
}

// ============================================================================
