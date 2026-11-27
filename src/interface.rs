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
//! identifying each interface by its name or by one of its IP addresses, or choosing every interface with
//! [`NetworkInterface::All`].
//!
//! [`network_interfaces`] lists the interfaces of this host, for example to let a user pick one.

use crate::error::errors::{Result, SacnError};
use std::net::{IpAddr, Ipv4Addr};

/// A network interface to use for multicast, identified by its name or by one of its IP addresses, or every interface.
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
/// let every_interface = NetworkInterface::All;
/// let loopback = NetworkInterface::Loopback;
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum NetworkInterface {
    /// The interface with this name, as the operating system reports it: for example `eth0` or `en0`, or the
    /// adapter's friendly name such as `Ethernet 2` on Windows. See [`network_interfaces`].
    Name(String),

    /// The interface that has this IP address assigned.
    Addr(IpAddr),

    /// Every interface that is up, isn't a loopback interface, and has an address of the socket's IP version.
    ///
    /// The interfaces are listed when `set_multicast_interfaces` is called, so an interface that appears later isn't
    /// used until `set_multicast_interfaces` is called again. Combine it with another variant to add a loopback
    /// interface, for example `[NetworkInterface::All, NetworkInterface::Loopback]`.
    All,

    /// The loopback interface (`lo` on Linux, `lo0` on macOS, `Loopback Pseudo-Interface 1` on Windows), for
    /// sending to and receiving from other programs on the same host.
    Loopback,
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

    /// True if the interface has a loopback address.
    loopback: bool,

    /// True if the operating system reports the interface as down.
    down: bool,
}

/// Returns the network interfaces of this host that have at least one IP address, with their names and addresses.
///
/// # Errors
/// Io: Returned if the operating system's list of interfaces can't be read.
pub fn network_interfaces() -> Result<Vec<InterfaceInfo>> {
    let mut infos: Vec<InterfaceInfo> = Vec::new();
    for iface in if_addrs::get_if_addrs()? {
        let addr = iface.ip();
        let down = iface.oper_status == if_addrs::IfOperStatus::Down;
        match infos.iter_mut().find(|i| i.name == iface.name) {
            Some(info) => {
                if !info.addrs.contains(&addr) {
                    info.addrs.push(addr);
                }
                if info.index.is_none() {
                    info.index = iface.index;
                }
                info.loopback |= addr.is_loopback();
                info.down &= down;
            }
            None => infos.push(InterfaceInfo {
                name: iface.name,
                addrs: vec![addr],
                index: iface.index,
                loopback: addr.is_loopback(),
                down,
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
pub(crate) fn resolve(interfaces: &[NetworkInterface], ipv6: bool) -> Result<Vec<ResolvedIf>> {
    if interfaces.is_empty() {
        return Ok(Vec::new());
    }
    resolve_among_named(interfaces, ipv6, &network_interfaces()?)
}

/// An interface resolved for a socket, with the name of the host interface it belongs to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ResolvedIf {
    /// The interface in the form the socket options take.
    pub(crate) mcast: MulticastIf,

    /// The name of the host interface. Two `ResolvedIf`s with the same name are the same interface, even if their
    /// `mcast` differ (for example two IPv4 addresses of one interface).
    pub(crate) name: String,
}

/// Returns the name of the host interface that has the given address, if any.
pub(crate) fn name_of_addr(addr: IpAddr) -> Option<String> {
    network_interfaces()
        .ok()?
        .into_iter()
        .find(|i| i.addrs.contains(&addr))
        .map(|i| i.name)
}

/// Resolves the given interfaces against the given list of host interfaces, without their names.
#[cfg(test)]
fn resolve_among(
    interfaces: &[NetworkInterface],
    ipv6: bool,
    host: &[InterfaceInfo],
) -> Result<Vec<MulticastIf>> {
    Ok(resolve_among_named(interfaces, ipv6, host)?
        .into_iter()
        .map(|r| r.mcast)
        .collect())
}

/// Resolves the given interfaces against the given list of host interfaces. Split from `resolve` for testing. Each
/// host interface appears at most once, the first time it is named.
fn resolve_among_named(
    interfaces: &[NetworkInterface],
    ipv6: bool,
    host: &[InterfaceInfo],
) -> Result<Vec<ResolvedIf>> {
    let mut resolved: Vec<ResolvedIf> = Vec::with_capacity(interfaces.len());
    let mut push = |mcast: MulticastIf, info: &InterfaceInfo| {
        if !resolved.iter().any(|r| r.name == info.name) {
            resolved.push(ResolvedIf {
                mcast,
                name: info.name.clone(),
            });
        }
    };

    for interface in interfaces {
        let info = match interface {
            NetworkInterface::Name(name) => host.iter().find(|i| &i.name == name),
            NetworkInterface::Addr(addr) => {
                if addr.is_ipv6() != ipv6 {
                    return Err(SacnError::IpVersionError());
                }
                host.iter().find(|i| i.addrs.contains(addr))
            }
            NetworkInterface::Loopback => {
                let loopback_addr = |a: &IpAddr| a.is_loopback() && a.is_ipv6() == ipv6;
                let info = host
                    .iter()
                    .find(|i| i.addrs.iter().any(loopback_addr))
                    .ok_or_else(|| {
                        not_found(format!(
                            "no loopback interface with an {} address",
                            if ipv6 { "IPv6" } else { "IPv4" }
                        ))
                    })?;
                let v4_hint = info.addrs.iter().find_map(|a| match a {
                    IpAddr::V4(v4) if v4.is_loopback() => Some(*v4),
                    _ => None,
                });
                push(resolve_info(info, v4_hint, ipv6)?, info);
                continue;
            }
            NetworkInterface::All => {
                let mut found = false;
                for info in host.iter().filter(|i| !i.loopback && !i.down) {
                    if let Ok(mcast_if) = resolve_info(info, None, ipv6) {
                        push(mcast_if, info);
                        found = true;
                    }
                }
                if !found {
                    return Err(not_found(format!(
                        "no network interface other than loopback has an {} address",
                        if ipv6 { "IPv6" } else { "IPv4" }
                    )));
                }
                continue;
            }
        }
        .ok_or_else(|| not_found(format!("network interface {interface:?} not found")))?;

        let v4_hint = match interface {
            NetworkInterface::Addr(IpAddr::V4(a)) => Some(*a),
            _ => None,
        };
        push(resolve_info(info, v4_hint, ipv6)?, info);
    }
    Ok(resolved)
}

/// Resolves one host interface for a socket of the given IP version. An IPv4 interface is used through `v4_addr` if
/// given, otherwise through its first IPv4 address.
fn resolve_info(
    info: &InterfaceInfo,
    v4_addr: Option<Ipv4Addr>,
    ipv6: bool,
) -> Result<MulticastIf> {
    if ipv6 {
        if !info.addrs.iter().any(IpAddr::is_ipv6) {
            return Err(not_found(format!(
                "network interface {} has no IPv6 address",
                info.name
            )));
        }
        Ok(MulticastIf::V6(info.index.ok_or_else(|| {
            not_found(format!(
                "network interface {} has no interface index",
                info.name
            ))
        })?))
    } else {
        let v4 = v4_addr.or_else(|| {
            info.addrs.iter().find_map(|a| match a {
                IpAddr::V4(a) => Some(*a),
                IpAddr::V6(_) => None,
            })
        });
        Ok(MulticastIf::V4(v4.ok_or_else(|| {
            not_found(format!(
                "network interface {} has no IPv4 address",
                info.name
            ))
        })?))
    }
}

fn not_found(msg: String) -> SacnError {
    SacnError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, msg))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn iface(name: &str, addrs: &[&str], index: u32) -> InterfaceInfo {
        let addrs: Vec<IpAddr> = addrs.iter().map(|a| a.parse().unwrap()).collect();
        InterfaceInfo {
            name: name.to_string(),
            loopback: addrs.iter().any(IpAddr::is_loopback),
            addrs,
            index: Some(index),
            down: false,
        }
    }

    fn host() -> Vec<InterfaceInfo> {
        vec![
            iface("lo", &["127.0.0.1", "::1"], 1),
            iface("eth0", &["10.0.0.5", "10.0.0.6", "fe80::5"], 2),
            iface("v4only", &["192.168.1.2"], 3),
            iface("v6only", &["fd00::3"], 4),
            InterfaceInfo {
                down: true,
                ..iface("down0", &["10.9.0.1", "fd00::9"], 5)
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
    fn all_resolves_to_interfaces_that_are_up_and_not_loopback() {
        let r = resolve_among(&[NetworkInterface::All], false, &host()).unwrap();
        assert_eq!(
            r,
            vec![
                MulticastIf::V4("10.0.0.5".parse().unwrap()),
                MulticastIf::V4("192.168.1.2".parse().unwrap())
            ]
        );
        let r = resolve_among(&[NetworkInterface::All], true, &host()).unwrap();
        assert_eq!(r, vec![MulticastIf::V6(2), MulticastIf::V6(4)]);
    }

    #[test]
    fn all_combines_with_other_interfaces() {
        let r = resolve_among(
            &[NetworkInterface::All, "lo".into(), "eth0".into()],
            false,
            &host(),
        )
        .unwrap();
        assert_eq!(
            r,
            vec![
                MulticastIf::V4("10.0.0.5".parse().unwrap()),
                MulticastIf::V4("192.168.1.2".parse().unwrap()),
                MulticastIf::V4("127.0.0.1".parse().unwrap())
            ]
        );
    }

    #[test]
    fn all_without_usable_interfaces_is_not_found() {
        let only_loopback = vec![iface("lo", &["127.0.0.1"], 1)];
        assert!(is_not_found(resolve_among(
            &[NetworkInterface::All],
            false,
            &only_loopback
        )));
    }

    #[test]
    fn loopback_resolves_to_loopback_address_or_index() {
        let host = vec![
            iface("eth0", &["10.0.0.5"], 2),
            iface("lo", &["192.168.0.6", "127.0.0.1", "::1"], 1),
        ];
        let r = resolve_among(&[NetworkInterface::Loopback], false, &host).unwrap();
        assert_eq!(r, vec![MulticastIf::V4("127.0.0.1".parse().unwrap())]);
        let r = resolve_among(&[NetworkInterface::Loopback], true, &host).unwrap();
        assert_eq!(r, vec![MulticastIf::V6(1)]);
    }

    #[test]
    fn missing_loopback_is_not_found() {
        let host = vec![iface("lo", &["127.0.0.1"], 1)];
        assert!(is_not_found(resolve_among(
            &[NetworkInterface::Loopback],
            true,
            &host
        )));
    }

    #[test]
    fn two_addresses_of_one_interface_resolve_once() {
        let r = resolve_among(
            &[
                NetworkInterface::Addr("10.0.0.6".parse().unwrap()),
                NetworkInterface::Addr("10.0.0.5".parse().unwrap()),
            ],
            false,
            &host(),
        )
        .unwrap();
        assert_eq!(r, vec![MulticastIf::V4("10.0.0.6".parse().unwrap())]);
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
