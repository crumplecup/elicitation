//! Kani proofs for MAC address byte validation.
//!
//! These proofs verify the correctness of MAC address (EUI-48) byte-level
//! validation following IEEE 802 specification.

#![cfg(kani)]

use crate::verification::types::{
    MacAddr, MacLocal, MacMulticast, MacUnicast, MacUniversal, is_local, is_multicast, is_unicast,
    is_universal,
};

// ============================================================================
// Unicast/Multicast Proofs (Bit 0 of Byte 0)
// ============================================================================

#[kani::proof]
#[kani::unwind(1)]
fn verify_unicast_detection() {
    let mut octets: [u8; 6] = kani::any();

    // Force bit 0 of byte 0 to 0 (unicast)
    octets[0] = octets[0] & 0xFE;

    // Should be unicast
    assert!(is_unicast(&octets));
    assert!(!is_multicast(&octets));

    let mac = MacAddr::new(octets);
    assert!(mac.is_unicast());
    assert!(!mac.is_multicast());

    let unicast = MacUnicast::new(octets);
    assert!(unicast.is_ok());
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_multicast_detection() {
    let mut octets: [u8; 6] = kani::any();

    // Force bit 0 of byte 0 to 1 (multicast)
    octets[0] = octets[0] | 0x01;

    // Should be multicast
    assert!(is_multicast(&octets));
    assert!(!is_unicast(&octets));

    let mac = MacAddr::new(octets);
    assert!(mac.is_multicast());
    assert!(!mac.is_unicast());

    let multicast = MacMulticast::new(octets);
    assert!(multicast.is_ok());
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_unicast_rejects_multicast() {
    let mut octets: [u8; 6] = kani::any();

    // Force multicast
    octets[0] = octets[0] | 0x01;

    // MacUnicast should reject
    let unicast = MacUnicast::new(octets);
    assert!(unicast.is_err());
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_multicast_rejects_unicast() {
    let mut octets: [u8; 6] = kani::any();

    // Force unicast
    octets[0] = octets[0] & 0xFE;

    // MacMulticast should reject
    let multicast = MacMulticast::new(octets);
    assert!(multicast.is_err());
}

// ============================================================================
// Universal/Local Proofs (Bit 1 of Byte 0)
// ============================================================================

#[kani::proof]
#[kani::unwind(1)]
fn verify_universal_detection() {
    let mut octets: [u8; 6] = kani::any();

    // Force bit 1 of byte 0 to 0 (universal)
    octets[0] = octets[0] & 0xFD;

    // Should be universal
    assert!(is_universal(&octets));
    assert!(!is_local(&octets));

    let mac = MacAddr::new(octets);
    assert!(mac.is_universal());
    assert!(!mac.is_local());

    let universal = MacUniversal::new(octets);
    assert!(universal.is_ok());
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_local_detection() {
    let mut octets: [u8; 6] = kani::any();

    // Force bit 1 of byte 0 to 1 (local)
    octets[0] = octets[0] | 0x02;

    // Should be local
    assert!(is_local(&octets));
    assert!(!is_universal(&octets));

    let mac = MacAddr::new(octets);
    assert!(mac.is_local());
    assert!(!mac.is_universal());

    let local = MacLocal::new(octets);
    assert!(local.is_ok());
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_universal_rejects_local() {
    let mut octets: [u8; 6] = kani::any();

    // Force local
    octets[0] = octets[0] | 0x02;

    // MacUniversal should reject
    let universal = MacUniversal::new(octets);
    assert!(universal.is_err());
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_local_rejects_universal() {
    let mut octets: [u8; 6] = kani::any();

    // Force universal
    octets[0] = octets[0] & 0xFD;

    // MacLocal should reject
    let local = MacLocal::new(octets);
    assert!(local.is_err());
}

// ============================================================================
// Special Address Proofs
// ============================================================================

#[kani::proof]
#[kani::unwind(1)]
fn verify_broadcast_is_multicast() {
    let octets = [0xFF; 6];

    let mac = MacAddr::new(octets);
    assert!(mac.is_broadcast());
    assert!(mac.is_multicast());
    assert!(mac.is_local());
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_null_is_unicast() {
    let octets = [0x00; 6];

    let mac = MacAddr::new(octets);
    assert!(mac.is_null());
    assert!(mac.is_unicast());
    assert!(mac.is_universal());
}

// ============================================================================
// Combined Bit Patterns
// ============================================================================

#[kani::proof]
#[kani::unwind(1)]
fn verify_unicast_universal() {
    let mut octets: [u8; 6] = kani::any();

    // Bits 0 and 1 both clear
    octets[0] = octets[0] & 0xFC;

    let mac = MacAddr::new(octets);
    assert!(mac.is_unicast());
    assert!(mac.is_universal());
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_unicast_local() {
    let mut octets: [u8; 6] = kani::any();

    // Bit 0 clear, bit 1 set
    octets[0] = (octets[0] & 0xFC) | 0x02;

    let mac = MacAddr::new(octets);
    assert!(mac.is_unicast());
    assert!(mac.is_local());
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_multicast_universal() {
    let mut octets: [u8; 6] = kani::any();

    // Bit 0 set, bit 1 clear
    octets[0] = (octets[0] & 0xFC) | 0x01;

    let mac = MacAddr::new(octets);
    assert!(mac.is_multicast());
    assert!(mac.is_universal());
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_multicast_local() {
    let mut octets: [u8; 6] = kani::any();

    // Both bits set
    octets[0] = octets[0] | 0x03;

    let mac = MacAddr::new(octets);
    assert!(mac.is_multicast());
    assert!(mac.is_local());
}

// ============================================================================
// Round-Trip Proofs
// ============================================================================

#[kani::proof]
#[kani::unwind(1)]
fn verify_macaddr_roundtrip() {
    let octets: [u8; 6] = kani::any();

    let mac = MacAddr::new(octets);
    let extracted = mac.octets();

    assert_eq!(octets, extracted);
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_unicast_roundtrip() {
    let mut octets: [u8; 6] = kani::any();
    octets[0] = octets[0] & 0xFE; // Force unicast

    let unicast = MacUnicast::new(octets).unwrap();
    let extracted = unicast.octets();

    assert_eq!(octets, extracted);
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_multicast_roundtrip() {
    let mut octets: [u8; 6] = kani::any();
    octets[0] = octets[0] | 0x01; // Force multicast

    let multicast = MacMulticast::new(octets).unwrap();
    let extracted = multicast.octets();

    assert_eq!(octets, extracted);
}
