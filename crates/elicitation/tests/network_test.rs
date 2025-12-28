//! Tests for network type implementations.

use elicitation::{Elicitation, Prompt};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

#[test]
fn test_ipaddr_has_prompt() {
    assert!(IpAddr::prompt().is_some());
}

#[test]
fn test_ipv4addr_has_prompt() {
    assert!(Ipv4Addr::prompt().is_some());
}

#[test]
fn test_ipv6addr_has_prompt() {
    assert!(Ipv6Addr::prompt().is_some());
}

#[test]
fn test_socketaddr_has_prompt() {
    assert!(SocketAddr::prompt().is_some());
}

#[test]
fn test_socketaddrv4_has_prompt() {
    assert!(SocketAddrV4::prompt().is_some());
}

#[test]
fn test_socketaddrv6_has_prompt() {
    assert!(SocketAddrV6::prompt().is_some());
}

#[test]
fn test_ipaddr_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<IpAddr>();
}

#[test]
fn test_ipv4addr_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<Ipv4Addr>();
}

#[test]
fn test_ipv6addr_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<Ipv6Addr>();
}

#[test]
fn test_socketaddr_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<SocketAddr>();
}

#[test]
fn test_socketaddrv4_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<SocketAddrV4>();
}

#[test]
fn test_socketaddrv6_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<SocketAddrV6>();
}

#[test]
fn test_ipv4_parsing() {
    // Test that standard IPv4 addresses parse correctly
    let addr: Ipv4Addr = "127.0.0.1".parse().expect("Valid IPv4");
    assert_eq!(addr, Ipv4Addr::new(127, 0, 0, 1));

    let addr: Ipv4Addr = "192.168.1.1".parse().expect("Valid IPv4");
    assert_eq!(addr, Ipv4Addr::new(192, 168, 1, 1));
}

#[test]
fn test_ipv6_parsing() {
    // Test that standard IPv6 addresses parse correctly
    let addr: Ipv6Addr = "::1".parse().expect("Valid IPv6");
    assert_eq!(addr, Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    let addr: Ipv6Addr = "2001:db8::1".parse().expect("Valid IPv6");
    assert!(addr.segments()[0] == 0x2001 && addr.segments()[1] == 0xdb8);
}

#[test]
fn test_socketaddr_parsing() {
    // Test IPv4 socket address
    let addr: SocketAddr = "127.0.0.1:8080".parse().expect("Valid socket address");
    assert_eq!(addr.port(), 8080);

    // Test IPv6 socket address (brackets required)
    let addr: SocketAddr = "[::1]:8080".parse().expect("Valid socket address");
    assert_eq!(addr.port(), 8080);
}
