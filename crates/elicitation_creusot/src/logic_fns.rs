//! Trusted logic functions for elicitation types used in Creusot contracts.
//!
//! These `#[trusted] #[logic(opaque)]` functions provide logic-context-callable
//! access to elicitation type methods, which cannot be called directly in
//! Pearlite logic context because they are compiled as program functions.
//!
//! All functions are trusted axioms — they assert the relationship between
//! the logic function and the underlying program behavior without proof.

#[cfg(creusot)]
use crate::*;

#[cfg(creusot)]
use elicitation::{
    I8Positive,
    verification::types::{
        Ipv4Bytes, Ipv6Bytes, MacAddr, PathBytes, SocketAddrV4Bytes, SocketAddrV6Bytes, Utf8Bytes,
    },
};

// ============================================================================
// Ipv4Bytes logic accessors
// ============================================================================

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn ipv4_octets(_addr: Ipv4Bytes) -> [u8; 4] {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn ipv4_is_loopback(_addr: Ipv4Bytes) -> bool {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn ipv4_is_unspecified(_addr: Ipv4Bytes) -> bool {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn ipv4_is_broadcast(_addr: Ipv4Bytes) -> bool {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn ipv4_is_multicast(_addr: Ipv4Bytes) -> bool {
    dead
}

// ============================================================================
// Ipv6Bytes logic accessors
// ============================================================================

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn ipv6_octets(_addr: Ipv6Bytes) -> [u8; 16] {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn ipv6_is_loopback(_addr: Ipv6Bytes) -> bool {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn ipv6_is_unspecified(_addr: Ipv6Bytes) -> bool {
    dead
}

// ============================================================================
// MacAddr logic accessors
// ============================================================================

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn mac_octets(_addr: MacAddr) -> [u8; 6] {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn mac_is_unicast(_addr: MacAddr) -> bool {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn mac_is_multicast(_addr: MacAddr) -> bool {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn mac_is_universal(_addr: MacAddr) -> bool {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn mac_is_local(_addr: MacAddr) -> bool {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn mac_is_broadcast(_addr: MacAddr) -> bool {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn mac_is_null(_addr: MacAddr) -> bool {
    dead
}

// ============================================================================
// SocketAddr logic accessors
// ============================================================================

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn v4_port(_addr: SocketAddrV4Bytes) -> u16 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn v6_port(_addr: SocketAddrV6Bytes) -> u16 {
    dead
}

// ============================================================================
// Utf8Bytes logic accessors
// ============================================================================

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn utf8_len<const N: usize>(_u: &Utf8Bytes<N>) -> usize {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn utf8_is_empty<const N: usize>(_u: &Utf8Bytes<N>) -> bool {
    dead
}

// ============================================================================
// PathBytes logic accessors
// ============================================================================

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn path_len<const N: usize>(_p: &PathBytes<N>) -> usize {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn path_is_empty<const N: usize>(_p: &PathBytes<N>) -> bool {
    dead
}

// ============================================================================
// Integer type logic accessors
// ============================================================================

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn i8pos_inner(_p: I8Positive) -> i8 {
    dead
}

// ============================================================================
// I8Positive::get (ref-based accessor)
// ============================================================================

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn i8pos_get(p: &I8Positive) -> i8 {
    dead
}
