# Networking design notes

How the library uses sockets, and where Linux, macOS and Windows differ. This is for contributors; the user-facing summary is in the crate documentation (`src/lib.rs`, "Choosing network interfaces" and "Platform notes").

## Why interfaces have to be chosen for multicast

Unicast follows the routing table: a packet to `10.0.0.20` leaves on the interface with the route to `10.0.0.0/24`. Multicast groups (`239.255.x.y` for IPv4, `ff18::83:00:xx:yy` and similar for IPv6, see E1.31 §9.3) have no such route. Without further instruction the operating system:

- sends multicast on the interface of the default route (or a route for `224.0.0.0/4` if one exists), and only that one;
- joins a group on the interface it picks for the default route.

So on a laptop with Wi-Fi as its default route and a USB Ethernet adapter on the lighting network, sACN goes out over Wi-Fi and a receiver joins on Wi-Fi. `set_multicast_interfaces` exists to change this behavior.

## How an interface is named to the operating system

| | IPv4 | IPv6 |
| --- | --- | --- |
| Outgoing multicast interface | `IP_MULTICAST_IF`, takes one of the interface's addresses | `IPV6_MULTICAST_IF`, takes the interface index |
| Joining a group | `IP_ADD_MEMBERSHIP`, takes one of the interface's addresses | `IPV6_JOIN_GROUP`, takes the interface index |
| "Let the OS choose" | `0.0.0.0` | index `0` |

`interface::resolve` turns a `NetworkInterface` variant into the form above (`MulticastIf`): for IPv4 the address that was given, or the interface's first IPv4 address; for IPv6 the index, after checking the interface has an IPv6 address. Interfaces are listed with the `if-addrs` crate (`getifaddrs()` on Unix, `GetAdaptersAddresses()` on Windows).

On Linux, an IPv4 source socket bound to `0.0.0.0` also uses the `IP_MULTICAST_IF` address as the source address of the packets it sends.

> [!WARNING]
> Interface indexes are assigned by the operating system, differ between hosts, and can change when an adapter is removed and re-added, so names are better for anything saved in a configuration file.

## Receiver sockets

### Binding

| | Linux and macOS | Windows |
| --- | --- | --- |
| Address bound | always the unspecified address (`0.0.0.0` or `::`) | the address given to `with_ip` |
| Port bound | always 5568 (`ACN_SDT_MULTICAST_PORT`) | the port given to `with_ip` |
| Address given to `with_ip` used for | IPv4: the default interface to join groups on; IPv6: only the IP version | the bind, and IPv4: the default interface to join groups on |

The difference comes from how each OS matches an incoming datagram to a socket:

- On Linux and macOS a UDP socket bound to a unicast address only receives datagrams whose destination is that address. Multicast is addressed to the group, so a socket bound to `192.168.0.6` never receives it, even after joining the group. A socket has to be bound to the unspecified address (or to the group address itself) to receive multicast.
- On Windows a socket bound to a unicast address also receives multicast for groups joined on that address's interface, but not for groups joined on other interfaces. A Windows receiver bound to a specific address therefore can't receive on several interfaces.

Binding the group address (for example `239.255.0.1:5568`) also works on Linux and macOS, but receives only that one group and no unicast. sACN uses one group per universe plus the discovery universe, so this would mean one socket per universe; the library doesn't do it.

### What binding the unspecified address exposes

Binding `0.0.0.0:5568` is recommended by the library, and causes the socket to listen for unicast on all interfaces and only receive multicast groups it joined. Sockets created by the library in this way receive:

- Unicast and broadcast: anything sent to port 5568 on any of the host's addresses, on any interface.
- Multicast: only the groups the socket joined, on the interfaces it is configured for. macOS and Windows only deliver the groups the socket joined natively, and on Linux we set `IP_MULTICAST_ALL` (or `IPV6_MULTICAST_ALL` for IPv6) to 0 to keep this behavior consistent across platforms. Linux defaults it to 1, which delivers to the socket every group joined by any socket on the host, on any interface.

Ways to limit unicast to chosen interfaces, none of which the library does today:

- Linux: `SO_BINDTODEVICE` (by name) or `SO_BINDTOIFINDEX` (by index) restricts the socket to one interface, for both unicast and multicast.
- macOS: `IP_BOUND_IF` / `IPV6_BOUND_IF` does the same.
- All three: `IP_PKTINFO` (Linux, Windows) or `IP_RECVIF` (macOS) report the interface each datagram arrived on, which allows filtering in the library for several interfaces.
- The host firewall.

### Port sharing

Several programs on one host may receive sACN, so receiver sockets set `SO_REUSEADDR`, and `SO_REUSEPORT` where it exists (not Windows, and ignored where the OS reports `ENOPROTOOPT`, such as ESP-IDF). Every socket bound to the port receives a copy of each multicast datagram. Unicast is different: Linux spreads unicast datagrams across the sockets in a `SO_REUSEPORT` group, so each one goes to only one of the receivers. This is why sources bind an ephemeral port by default; a source bound to 5568 would join that `SO_REUSEPORT` group and receive unicast it never reads.

### Multicast group membership

- A receiver joins every listened universe, the synchronisation universes and the discovery universe on each chosen interface.
- Joining a group a second time on the same interface fails with `EADDRINUSE`. Two `NetworkInterface`s that resolve to the same host interface (for example two of its addresses) are therefore joined once, and an IPv6 membership on index 0 (OS's choice) is the same as one on the interface the OS picked.
- When a receiver's configured interfaces change, new memberships are joined before old ones are left, so no packets are lost.
- The exception is a membership on the OS's choice (`0.0.0.0` / index 0), which is left first. The OS resolves the choice to a concrete interface when the group is joined, and the membership stays on that interface even if the default route changes later. Leaving with `0.0.0.0` / index 0 doesn't name an interface, so Linux removes the socket's most recently joined membership of that group. If a new membership for the group had already been joined, that is the one removed, and the old one stays. Joining first could also fail with `EADDRINUSE` when the new interface is the one the OS chose.
- On Windows, IPv6 multicast is reported as unsupported (`is_multicast_enabled` is false for an IPv6 receiver).

### Duplicate packets

A receiver joined on several interfaces that are on the same network receives each packet once per interface. E1.31 §6.7.2 already requires discarding a packet whose sequence number isn't newer than the last one, so the copies are dropped. They are dropped silently (not returned as `OutOfSequence`) when the receiver uses more than one interface, or for one second after its interfaces change.

## Source sockets

- Bound to an ephemeral port by default (see [Port sharing](#port-sharing)), and `SO_REUSEADDR` / `SO_REUSEPORT` (Linux only) are set.
- With no chosen interfaces, multicast goes where the OS sends it (see the first section).
- With chosen interfaces, the source sets `IP_MULTICAST_IF` / `IPV6_MULTICAST_IF` before each multicast send and sends the packet once per interface. Unicast is sent once and follows the routing table.
