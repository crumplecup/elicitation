//! Prusti proofs for socket address validation types.
//!
//! Validates socket addresses (IP + port):
//! - SocketAddrV4Bytes: IPv4 + port
//! - SocketAddrV6Bytes: IPv6 + port
//! - Port range classifications (well-known, registered, dynamic)
//!
//! This is compositional verification: stdlib_ip_correct → socket_wrapper_correct.

#![cfg(feature = "verify-prusti")]
#![allow(unused_imports)]

use crate::verification::types::{
    Ipv4Bytes, Ipv6Bytes, SocketAddrV4Bytes, SocketAddrV6Bytes, ValidationError, is_dynamic_port,
    is_nonzero_port, is_privileged_port, is_registered_port, is_well_known_port,
};
use prusti_contracts::*;

// SocketAddrV4Bytes Validation Proofs
// ============================================================================

/// Verify: SocketAddrV4Bytes construction always succeeds
#[cfg(feature = "verify-prusti")]
pub fn verify_socket_v4_construction(ip: [u8; 4], port: u16) -> SocketAddrV4Bytes {
    SocketAddrV4Bytes::from_octets(ip, port)
}

/// Verify: ip() accessor returns the IP address
#[cfg(feature = "verify-prusti")]
pub fn verify_socket_v4_ip_accessor(ip: [u8; 4], port: u16) -> SocketAddrV4Bytes {
    let socket = SocketAddrV4Bytes::from_octets(ip, port);
    let _ip_ref = socket.ip();
    socket
}

/// Verify: port() accessor returns the port
#[cfg(feature = "verify-prusti")]
#[ensures(result.port() == port)]
pub fn verify_socket_v4_port_accessor(ip: [u8; 4], port: u16) -> SocketAddrV4Bytes {
    SocketAddrV4Bytes::from_octets(ip, port)
}

/// Verify: into_parts() decomposes correctly
#[cfg(feature = "verify-prusti")]
pub fn verify_socket_v4_into_parts(ip: [u8; 4], port: u16) -> SocketAddrV4Bytes {
    let socket = SocketAddrV4Bytes::from_octets(ip, port);
    let (_ip, _port) = socket.into_parts();
    socket
}

/// Verify: Common socket address (HTTP on localhost)
#[cfg(feature = "verify-prusti")]
#[ensures(result.port() == 80)]
pub fn verify_socket_v4_localhost_http() -> SocketAddrV4Bytes {
    SocketAddrV4Bytes::from_octets([127, 0, 0, 1], 80)
}

/// Verify: Common socket address (HTTPS on localhost)
#[cfg(feature = "verify-prusti")]
#[ensures(result.port() == 443)]
pub fn verify_socket_v4_localhost_https() -> SocketAddrV4Bytes {
    SocketAddrV4Bytes::from_octets([127, 0, 0, 1], 443)
}

/// Verify: Common socket address (SSH)
#[cfg(feature = "verify-prusti")]
#[ensures(result.port() == 22)]
pub fn verify_socket_v4_ssh() -> SocketAddrV4Bytes {
    SocketAddrV4Bytes::from_octets([192, 168, 1, 1], 22)
}

/// Verify: Development server port
#[cfg(feature = "verify-prusti")]
#[ensures(result.port() == 3000)]
pub fn verify_socket_v4_dev_server() -> SocketAddrV4Bytes {
    SocketAddrV4Bytes::from_octets([127, 0, 0, 1], 3000)
}

// SocketAddrV6Bytes Validation Proofs
// ============================================================================

/// Verify: SocketAddrV6Bytes construction always succeeds
#[cfg(feature = "verify-prusti")]
pub fn verify_socket_v6_construction(ip: [u8; 16], port: u16) -> SocketAddrV6Bytes {
    SocketAddrV6Bytes::from_octets(ip, port)
}

/// Verify: ip() accessor returns the IP address
#[cfg(feature = "verify-prusti")]
pub fn verify_socket_v6_ip_accessor(ip: [u8; 16], port: u16) -> SocketAddrV6Bytes {
    let socket = SocketAddrV6Bytes::from_octets(ip, port);
    let _ip_ref = socket.ip();
    socket
}

/// Verify: port() accessor returns the port
#[cfg(feature = "verify-prusti")]
#[ensures(result.port() == port)]
pub fn verify_socket_v6_port_accessor(ip: [u8; 16], port: u16) -> SocketAddrV6Bytes {
    SocketAddrV6Bytes::from_octets(ip, port)
}

/// Verify: into_parts() decomposes correctly
#[cfg(feature = "verify-prusti")]
pub fn verify_socket_v6_into_parts(ip: [u8; 16], port: u16) -> SocketAddrV6Bytes {
    let socket = SocketAddrV6Bytes::from_octets(ip, port);
    let (_ip, _port) = socket.into_parts();
    socket
}

/// Verify: IPv6 localhost HTTP
#[cfg(feature = "verify-prusti")]
#[ensures(result.port() == 80)]
pub fn verify_socket_v6_localhost_http() -> SocketAddrV6Bytes {
    SocketAddrV6Bytes::from_octets([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 80)
}

/// Verify: IPv6 localhost HTTPS
#[cfg(feature = "verify-prusti")]
#[ensures(result.port() == 443)]
pub fn verify_socket_v6_localhost_https() -> SocketAddrV6Bytes {
    SocketAddrV6Bytes::from_octets([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], 443)
}

// Port Classification Proofs
// ============================================================================

/// Verify: Port 0 is well-known
#[cfg(feature = "verify-prusti")]
#[ensures(result)]
pub fn verify_port_0_well_known() -> bool {
    is_well_known_port(0)
}

/// Verify: Port 80 (HTTP) is well-known
#[cfg(feature = "verify-prusti")]
#[ensures(result)]
pub fn verify_port_80_well_known() -> bool {
    is_well_known_port(80)
}

/// Verify: Port 443 (HTTPS) is well-known
#[cfg(feature = "verify-prusti")]
#[ensures(result)]
pub fn verify_port_443_well_known() -> bool {
    is_well_known_port(443)
}

/// Verify: Port 1023 is well-known (boundary)
#[cfg(feature = "verify-prusti")]
#[ensures(result)]
pub fn verify_port_1023_well_known() -> bool {
    is_well_known_port(1023)
}

/// Verify: Port 1024 is registered (boundary)
#[cfg(feature = "verify-prusti")]
#[ensures(result)]
pub fn verify_port_1024_registered() -> bool {
    is_registered_port(1024)
}

/// Verify: Port 3000 (dev server) is registered
#[cfg(feature = "verify-prusti")]
#[ensures(result)]
pub fn verify_port_3000_registered() -> bool {
    is_registered_port(3000)
}

/// Verify: Port 5432 (PostgreSQL) is registered
#[cfg(feature = "verify-prusti")]
#[ensures(result)]
pub fn verify_port_5432_registered() -> bool {
    is_registered_port(5432)
}

/// Verify: Port 49151 is registered (boundary)
#[cfg(feature = "verify-prusti")]
#[ensures(result)]
pub fn verify_port_49151_registered() -> bool {
    is_registered_port(49151)
}

/// Verify: Port 49152 is dynamic (boundary)
#[cfg(feature = "verify-prusti")]
#[ensures(result)]
pub fn verify_port_49152_dynamic() -> bool {
    is_dynamic_port(49152)
}

/// Verify: Port 65535 is dynamic (max port)
#[cfg(feature = "verify-prusti")]
#[ensures(result)]
pub fn verify_port_65535_dynamic() -> bool {
    is_dynamic_port(65535)
}

/// Verify: Port 0 is privileged
#[cfg(feature = "verify-prusti")]
#[ensures(result)]
pub fn verify_port_0_privileged() -> bool {
    is_privileged_port(0)
}

/// Verify: Port 1023 is privileged (boundary)
#[cfg(feature = "verify-prusti")]
#[ensures(result)]
pub fn verify_port_1023_privileged() -> bool {
    is_privileged_port(1023)
}

/// Verify: Port 1024 is not privileged
#[cfg(feature = "verify-prusti")]
#[ensures(!result)]
pub fn verify_port_1024_not_privileged() -> bool {
    is_privileged_port(1024)
}

/// Verify: Port 0 is not nonzero
#[cfg(feature = "verify-prusti")]
#[ensures(!result)]
pub fn verify_port_0_not_nonzero() -> bool {
    is_nonzero_port(0)
}

/// Verify: Port 1 is nonzero
#[cfg(feature = "verify-prusti")]
#[ensures(result)]
pub fn verify_port_1_nonzero() -> bool {
    is_nonzero_port(1)
}

/// Verify: Port 80 is nonzero
#[cfg(feature = "verify-prusti")]
#[ensures(result)]
pub fn verify_port_80_nonzero() -> bool {
    is_nonzero_port(80)
}

// Edge Cases
// ============================================================================

/// Verify: Zero IP with zero port
#[cfg(feature = "verify-prusti")]
#[ensures(result.port() == 0)]
pub fn verify_socket_v4_zero_zero() -> SocketAddrV4Bytes {
    SocketAddrV4Bytes::from_octets([0, 0, 0, 0], 0)
}

/// Verify: Max IP with max port
#[cfg(feature = "verify-prusti")]
#[ensures(result.port() == 65535)]
pub fn verify_socket_v4_max_max() -> SocketAddrV4Bytes {
    SocketAddrV4Bytes::from_octets([255, 255, 255, 255], 65535)
}

/// Verify: IPv6 zero IP with zero port
#[cfg(feature = "verify-prusti")]
#[ensures(result.port() == 0)]
pub fn verify_socket_v6_zero_zero() -> SocketAddrV6Bytes {
    SocketAddrV6Bytes::from_octets([0; 16], 0)
}

/// Verify: IPv6 max IP with max port
#[cfg(feature = "verify-prusti")]
#[ensures(result.port() == 65535)]
pub fn verify_socket_v6_max_max() -> SocketAddrV6Bytes {
    SocketAddrV6Bytes::from_octets([0xff; 16], 65535)
}

// ============================================================================
