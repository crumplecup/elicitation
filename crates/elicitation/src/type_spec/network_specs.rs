//! [`ElicitSpec`](crate::ElicitSpec) implementations for network contract types.

use crate::verification::types::{IpPrivate, IpPublic, IpV4, IpV6, Ipv4Loopback, Ipv6Loopback};
use crate::{
    ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
    TypeSpecInventoryKey,
};

macro_rules! impl_network_spec {
    (
        type    = $ty:ty,
        name    = $name:literal,
        summary = $summary:literal,
        requires = [($req_label:literal, $req_desc:literal, $req_expr:literal)] $(,)?
    ) => {
        impl ElicitSpec for $ty {
            fn type_spec() -> TypeSpec {
                let requires = SpecCategoryBuilder::default()
                    .name("requires".to_string())
                    .entries(vec![
                        SpecEntryBuilder::default()
                            .label($req_label.to_string())
                            .description($req_desc.to_string())
                            .expression(Some($req_expr.to_string()))
                            .build()
                            .expect("valid SpecEntry"),
                    ])
                    .build()
                    .expect("valid SpecCategory");
                TypeSpecBuilder::default()
                    .type_name($name.to_string())
                    .summary($summary.to_string())
                    .categories(vec![requires])
                    .build()
                    .expect("valid TypeSpec")
            }
        }

        inventory::submit!(TypeSpecInventoryKey::new(
            $name,
            <$ty as ElicitSpec>::type_spec
        ));
    };
}

impl_network_spec!(
    type    = IpPrivate,
    name    = "IpPrivate",
    summary = "An IP address guaranteed to be in a private range (RFC 1918 / fc00::/7).",
    requires = [("private", "Address must be private: IPv4 RFC 1918 (10/8, 172.16/12, 192.168/16) or IPv6 fc00::/7.", "ip.is_ipv4() || ip.is_ipv6()")],
);

impl_network_spec!(
    type    = IpPublic,
    name    = "IpPublic",
    summary = "An IP address guaranteed to be publicly routable (not private or loopback).",
    requires = [("public", "Address must not be a private RFC 1918 / fc00::/7 or loopback address.", "!ip.is_loopback()")],
);

impl_network_spec!(
    type    = IpV4,
    name    = "IpV4",
    summary = "An IP address guaranteed to be IPv4.",
    requires = [("ipv4", "Address must be an IPv4 address.", "ip.is_ipv4()")],
);

impl_network_spec!(
    type    = IpV6,
    name    = "IpV6",
    summary = "An IP address guaranteed to be IPv6.",
    requires = [("ipv6", "Address must be an IPv6 address.", "ip.is_ipv6()")],
);

impl_network_spec!(
    type    = Ipv4Loopback,
    name    = "Ipv4Loopback",
    summary = "An IPv4 address guaranteed to be the loopback address (127.0.0.1).",
    requires = [("loopback", "Address must be the IPv4 loopback (127.x.x.x).", "ip.is_loopback()")],
);

impl_network_spec!(
    type    = Ipv6Loopback,
    name    = "Ipv6Loopback",
    summary = "An IPv6 address guaranteed to be the loopback address (::1).",
    requires = [("loopback", "Address must be the IPv6 loopback (::1).", "ip.is_loopback()")],
);
