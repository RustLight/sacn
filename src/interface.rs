// Copyright 2026 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Selection of the network interfaces used for multicast.
//!
//! By default a [`SacnSource`](crate::source::SacnSource) sends multicast on the interface chosen by the operating
//! system's routing table, and a [`SacnReceiver`](crate::receive::SacnReceiver) joins multicast groups on the default
//! interface. Both can be told to use one or more specific interfaces instead with `set_multicast_interfaces`,
//! identifying each interface by its name or by one of its IP addresses.
//!
//! [`network_interfaces`] lists the interfaces of this host, for example to let a user pick one.

use crate::error::errors::{Result, SacnError};
use std::net::{IpAddr, Ipv4Addr};

/// A network interface to use for multicast, identified by its name or by one of its IP addresses.
///
/// The interface is looked up when it is passed to `set_multicast_interfaces`. If the interface later disappears and
/// comes back (for example a USB adapter being re-plugged), call `set_multicast_interfaces` again.
///
/// # Examples
/// ```
/// use sacn::interface::NetworkInterface;
///
/// let by_name = NetworkInterface::Name("eth0".to_string());
/// let by_v4_addr = NetworkInterface::Addr("192.168.1.10".parse().unwrap());
/// let by_v6_addr = NetworkInterface::Addr("fe80::1".parse().unwrap());
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum NetworkInterface {
    /// The interface with this name, as the operating system reports it: for example `eth0` or `en0`, or the
    /// adapter's friendly name such as `Ethernet 2` on Windows. See [`network_interfaces`].
    Name(String),

    /// The interface that has this IP address assigned.
    Addr(IpAddr),
}

impl From<IpAddr> for NetworkInterface {
    fn from(addr: IpAddr) -> Self {
        NetworkInterface::Addr(addr)
    }
}

impl From<&str> for NetworkInterface {
    fn from(name: &str) -> Self {
        NetworkInterface::Name(name.to_string())
    }
}

impl From<String> for NetworkInterface {
    fn from(name: String) -> Self {
        NetworkInterface::Name(name)
    }
}

/// A network interface of this host, as returned by [`network_interfaces`].
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct InterfaceInfo {
    /// The interface's name, usable with [`NetworkInterface::Name`].
    pub name: String,

    /// The IP addresses assigned to the interface, including link-local addresses.
    pub addrs: Vec<IpAddr>,

    /// The operating system's index for the interface, used for IPv6 multicast.
    index: Option<u32>,
}

/// Returns the network interfaces of this host that have at least one IP address, with their names and addresses.
///
/// # Errors
/// Io: Returned if the operating system's list of interfaces can't be read.
pub fn network_interfaces() -> Result<Vec<InterfaceInfo>> {
    let mut infos: Vec<InterfaceInfo> = Vec::new();
    for iface in if_addrs::get_if_addrs()? {
        let addr = iface.ip();
        match infos.iter_mut().find(|i| i.name == iface.name) {
            Some(info) => {
                if !info.addrs.contains(&addr) {
                    info.addrs.push(addr);
                }
                if info.index.is_none() {
                    info.index = iface.index;
                }
            }
            None => infos.push(InterfaceInfo {
                name: iface.name,
                addrs: vec![addr],
                index: iface.index,
            }),
        }
    }
    Ok(infos)
}

/// An interface resolved to the form the socket options take.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MulticastIf {
    /// An IPv4 interface, given by one of its addresses. `0.0.0.0` lets the operating system choose.
    V4(Ipv4Addr),

    /// An IPv6 interface, given by its index. `0` lets the operating system choose.
    V6(u32),
}

/// Resolves the given interfaces for a socket of the given IP version, looking them up among this host's interfaces.
///
/// # Errors
/// Io (`NotFound`): Returned if an interface doesn't exist, or has no address of the socket's IP version.
///
/// `IpVersionError`: Returned if an interface is given by an address of the other IP version.
pub(crate) fn resolve(interfaces: &[NetworkInterface], ipv6: bool) -> Result<Vec<MulticastIf>> {
    if interfaces.is_empty() {
        return Ok(Vec::new());
    }
    resolve_among(interfaces, ipv6, &network_interfaces()?)
}

/// Resolves the given interfaces against the given list of host interfaces. Split from `resolve` for testing.
fn resolve_among(
    interfaces: &[NetworkInterface],
    ipv6: bool,
    host: &[InterfaceInfo],
) -> Result<Vec<MulticastIf>> {
    let mut resolved = Vec::with_capacity(interfaces.len());
    for interface in interfaces {
        let info = match interface {
            NetworkInterface::Name(name) => host.iter().find(|i| &i.name == name),
            NetworkInterface::Addr(addr) => {
                if addr.is_ipv6() != ipv6 {
                    return Err(SacnError::IpVersionError());
                }
                host.iter().find(|i| i.addrs.contains(addr))
            }
        }
        .ok_or_else(|| not_found(format!("network interface {interface:?} not found")))?;

        let mcast_if = if ipv6 {
            if !info.addrs.iter().any(IpAddr::is_ipv6) {
                return Err(not_found(format!(
                    "network interface {} has no IPv6 address",
                    info.name
                )));
            }
            MulticastIf::V6(info.index.ok_or_else(|| {
                not_found(format!(
                    "network interface {} has no interface index",
                    info.name
                ))
            })?)
        } else {
            let v4 = match interface {
                NetworkInterface::Addr(IpAddr::V4(a)) => Some(*a),
                _ => info.addrs.iter().find_map(|a| match a {
                    IpAddr::V4(a) => Some(*a),
                    IpAddr::V6(_) => None,
                }),
            };
            MulticastIf::V4(v4.ok_or_else(|| {
                not_found(format!(
                    "network interface {} has no IPv4 address",
                    info.name
                ))
            })?)
        };

        if !resolved.contains(&mcast_if) {
            resolved.push(mcast_if);
        }
    }
    Ok(resolved)
}

fn not_found(msg: String) -> SacnError {
    SacnError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, msg))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn host() -> Vec<InterfaceInfo> {
        vec![
            InterfaceInfo {
                name: "lo".to_string(),
                addrs: vec!["127.0.0.1".parse().unwrap(), "::1".parse().unwrap()],
                index: Some(1),
            },
            InterfaceInfo {
                name: "eth0".to_string(),
                addrs: vec![
                    "10.0.0.5".parse().unwrap(),
                    "10.0.0.6".parse().unwrap(),
                    "fe80::5".parse().unwrap(),
                ],
                index: Some(2),
            },
            InterfaceInfo {
                name: "v4only".to_string(),
                addrs: vec!["192.168.1.2".parse().unwrap()],
                index: Some(3),
            },
            InterfaceInfo {
                name: "v6only".to_string(),
                addrs: vec!["fd00::3".parse().unwrap()],
                index: Some(4),
            },
        ]
    }

    fn is_not_found(r: Result<Vec<MulticastIf>>) -> bool {
        matches!(r, Err(SacnError::Io(e)) if e.kind() == std::io::ErrorKind::NotFound)
    }

    #[test]
    fn name_resolves_to_first_ipv4_address() {
        let r = resolve_among(&["eth0".into()], false, &host()).unwrap();
        assert_eq!(r, vec![MulticastIf::V4("10.0.0.5".parse().unwrap())]);
    }

    #[test]
    fn ipv4_address_resolves_to_itself() {
        let r = resolve_among(
            &[NetworkInterface::Addr("10.0.0.6".parse().unwrap())],
            false,
            &host(),
        )
        .unwrap();
        assert_eq!(r, vec![MulticastIf::V4("10.0.0.6".parse().unwrap())]);
    }

    #[test]
    fn name_and_ipv6_address_resolve_to_index() {
        let r = resolve_among(
            &[
                "eth0".into(),
                NetworkInterface::Addr("fd00::3".parse().unwrap()),
            ],
            true,
            &host(),
        )
        .unwrap();
        assert_eq!(r, vec![MulticastIf::V6(2), MulticastIf::V6(4)]);
    }

    #[test]
    fn duplicates_are_removed() {
        let r = resolve_among(
            &[
                "eth0".into(),
                NetworkInterface::Addr("fe80::5".parse().unwrap()),
            ],
            true,
            &host(),
        )
        .unwrap();
        assert_eq!(r, vec![MulticastIf::V6(2)]);
    }

    #[test]
    fn unknown_name_or_address_is_not_found() {
        assert!(is_not_found(resolve_among(
            &["wlan9".into()],
            false,
            &host()
        )));
        assert!(is_not_found(resolve_among(
            &[NetworkInterface::Addr("10.9.9.9".parse().unwrap())],
            false,
            &host()
        )));
    }

    #[test]
    fn interface_without_address_of_socket_version_is_not_found() {
        assert!(is_not_found(resolve_among(
            &["v4only".into()],
            true,
            &host()
        )));
        assert!(is_not_found(resolve_among(
            &["v6only".into()],
            false,
            &host()
        )));
    }

    #[test]
    fn address_of_other_ip_version_is_ip_version_error() {
        assert!(matches!(
            resolve_among(
                &[NetworkInterface::Addr("fd00::3".parse().unwrap())],
                false,
                &host()
            ),
            Err(SacnError::IpVersionError())
        ));
        assert!(matches!(
            resolve_among(
                &[NetworkInterface::Addr("10.0.0.5".parse().unwrap())],
                true,
                &host()
            ),
            Err(SacnError::IpVersionError())
        ));
    }

    #[test]
    fn empty_list_resolves_to_empty() {
        assert_eq!(resolve(&[], false).unwrap(), Vec::new());
    }

    #[test]
    fn host_has_loopback() {
        let ifaces = network_interfaces().unwrap();
        assert!(
            ifaces
                .iter()
                .any(|i| i.addrs.contains(&IpAddr::V4(Ipv4Addr::LOCALHOST)))
        );
    }
}
