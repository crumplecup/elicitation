//! Kani proofs for socket address validation.
//!
//! These proofs verify the correctness of socket address (IP + port)
//! byte-level validation.

#![cfg(kani)]

use crate::verification::types::{
    Ipv4Bytes, Ipv6Bytes, SocketAddrV4Bytes, SocketAddrV4NonZero, SocketAddrV4Privileged,
    SocketAddrV4Unprivileged, SocketAddrV6Bytes, SocketAddrV6NonZero, SocketAddrV6Privileged,
    SocketAddrV6Unprivileged, is_dynamic_port, is_privileged_port, is_registered_port,
    is_well_known_port,
};

// ============================================================================
// Port Range Proofs
// ============================================================================

#[kani::proof]
fn verify_well_known_port_range() {
    let port: u16 = kani::any();
    kani::assume(port <= 1023);

    assert!(is_well_known_port(port));
    assert!(is_privileged_port(port));
}

#[kani::proof]
fn verify_registered_port_range() {
    let port: u16 = kani::any();
    kani::assume(port >= 1024 && port <= 49151);

    assert!(is_registered_port(port));
    assert!(!is_well_known_port(port));
    assert!(!is_dynamic_port(port));
    assert!(!is_privileged_port(port));
}

#[kani::proof]
fn verify_dynamic_port_range() {
    let port: u16 = kani::any();
    kani::assume(port >= 49152);

    assert!(is_dynamic_port(port));
    assert!(!is_well_known_port(port));
    assert!(!is_registered_port(port));
    assert!(!is_privileged_port(port));
}

// ============================================================================
// SocketAddrV4 Proofs
// ============================================================================

#[kani::proof]
fn verify_socketaddrv4_construction() {
    let ip_octets: [u8; 4] = kani::any();
    let port: u16 = kani::any();

    let ip = Ipv4Bytes::new(ip_octets);
    let addr = SocketAddrV4Bytes::new(ip, port);

    assert_eq!(addr.ip().octets(), ip_octets);
    assert_eq!(addr.port(), port);
}

#[kani::proof]
fn verify_socketaddrv4_nonzero_accepts_nonzero() {
    let ip_octets: [u8; 4] = kani::any();
    let port: u16 = kani::any();
    kani::assume(port != 0);

    let ip = Ipv4Bytes::new(ip_octets);
    let addr = SocketAddrV4NonZero::new(ip, port);

    assert!(addr.is_ok());
    assert_eq!(addr.unwrap().port(), port);
}

#[kani::proof]
fn verify_socketaddrv4_nonzero_rejects_zero() {
    let ip_octets: [u8; 4] = kani::any();
    let ip = Ipv4Bytes::new(ip_octets);

    let addr = SocketAddrV4NonZero::new(ip, 0);
    assert!(addr.is_err());
}

#[kani::proof]
fn verify_socketaddrv4_privileged_accepts_lt1024() {
    let ip_octets: [u8; 4] = kani::any();
    let port: u16 = kani::any();
    kani::assume(port < 1024);

    let ip = Ipv4Bytes::new(ip_octets);
    let addr = SocketAddrV4Privileged::new(ip, port);

    assert!(addr.is_ok());
    assert_eq!(addr.unwrap().port(), port);
}

#[kani::proof]
fn verify_socketaddrv4_privileged_rejects_ge1024() {
    let ip_octets: [u8; 4] = kani::any();
    let port: u16 = kani::any();
    kani::assume(port >= 1024);

    let ip = Ipv4Bytes::new(ip_octets);
    let addr = SocketAddrV4Privileged::new(ip, port);

    assert!(addr.is_err());
}

#[kani::proof]
fn verify_socketaddrv4_unprivileged_accepts_ge1024() {
    let ip_octets: [u8; 4] = kani::any();
    let port: u16 = kani::any();
    kani::assume(port >= 1024);

    let ip = Ipv4Bytes::new(ip_octets);
    let addr = SocketAddrV4Unprivileged::new(ip, port);

    assert!(addr.is_ok());
    assert_eq!(addr.unwrap().port(), port);
}

#[kani::proof]
fn verify_socketaddrv4_unprivileged_rejects_lt1024() {
    let ip_octets: [u8; 4] = kani::any();
    let port: u16 = kani::any();
    kani::assume(port < 1024);

    let ip = Ipv4Bytes::new(ip_octets);
    let addr = SocketAddrV4Unprivileged::new(ip, port);

    assert!(addr.is_err());
}

// ============================================================================
// SocketAddrV6 Proofs
// ============================================================================

#[kani::proof]
fn verify_socketaddrv6_construction() {
    let ip_octets: [u8; 16] = kani::any();
    let port: u16 = kani::any();

    let ip = Ipv6Bytes::new(ip_octets);
    let addr = SocketAddrV6Bytes::new(ip, port);

    assert_eq!(addr.ip().octets(), ip_octets);
    assert_eq!(addr.port(), port);
}

#[kani::proof]
fn verify_socketaddrv6_nonzero_accepts_nonzero() {
    let ip_octets: [u8; 16] = kani::any();
    let port: u16 = kani::any();
    kani::assume(port != 0);

    let ip = Ipv6Bytes::new(ip_octets);
    let addr = SocketAddrV6NonZero::new(ip, port);

    assert!(addr.is_ok());
    assert_eq!(addr.unwrap().port(), port);
}

#[kani::proof]
fn verify_socketaddrv6_nonzero_rejects_zero() {
    let ip_octets: [u8; 16] = kani::any();
    let ip = Ipv6Bytes::new(ip_octets);

    let addr = SocketAddrV6NonZero::new(ip, 0);
    assert!(addr.is_err());
}

#[kani::proof]
fn verify_socketaddrv6_privileged_accepts_lt1024() {
    let ip_octets: [u8; 16] = kani::any();
    let port: u16 = kani::any();
    kani::assume(port < 1024);

    let ip = Ipv6Bytes::new(ip_octets);
    let addr = SocketAddrV6Privileged::new(ip, port);

    assert!(addr.is_ok());
    assert_eq!(addr.unwrap().port(), port);
}

#[kani::proof]
fn verify_socketaddrv6_privileged_rejects_ge1024() {
    let ip_octets: [u8; 16] = kani::any();
    let port: u16 = kani::any();
    kani::assume(port >= 1024);

    let ip = Ipv6Bytes::new(ip_octets);
    let addr = SocketAddrV6Privileged::new(ip, port);

    assert!(addr.is_err());
}

#[kani::proof]
fn verify_socketaddrv6_unprivileged_accepts_ge1024() {
    let ip_octets: [u8; 16] = kani::any();
    let port: u16 = kani::any();
    kani::assume(port >= 1024);

    let ip = Ipv6Bytes::new(ip_octets);
    let addr = SocketAddrV6Unprivileged::new(ip, port);

    assert!(addr.is_ok());
    assert_eq!(addr.unwrap().port(), port);
}

#[kani::proof]
fn verify_socketaddrv6_unprivileged_rejects_lt1024() {
    let ip_octets: [u8; 16] = kani::any();
    let port: u16 = kani::any();
    kani::assume(port < 1024);

    let ip = Ipv6Bytes::new(ip_octets);
    let addr = SocketAddrV6Unprivileged::new(ip, port);

    assert!(addr.is_err());
}

// ============================================================================
// Round-Trip Proofs
// ============================================================================

#[kani::proof]
fn verify_socketaddrv4_roundtrip() {
    let ip_octets: [u8; 4] = kani::any();
    let port: u16 = kani::any();

    let ip = Ipv4Bytes::new(ip_octets);
    let addr = SocketAddrV4Bytes::new(ip, port);
    let (extracted_ip, extracted_port) = addr.into_parts();

    assert_eq!(extracted_ip.octets(), ip_octets);
    assert_eq!(extracted_port, port);
}

#[kani::proof]
fn verify_socketaddrv6_roundtrip() {
    let ip_octets: [u8; 16] = kani::any();
    let port: u16 = kani::any();

    let ip = Ipv6Bytes::new(ip_octets);
    let addr = SocketAddrV6Bytes::new(ip, port);
    let (extracted_ip, extracted_port) = addr.into_parts();

    assert_eq!(extracted_ip.octets(), ip_octets);
    assert_eq!(extracted_port, port);
}
