"""Probe: does multicast sent on each interface loop back to a local receiver?"""
import socket, struct, sys, time, json, subprocess

def ifaces():
    out = []
    try:
        import psutil  # noqa
    except ImportError:
        pass
    for idx, name in socket.if_nameindex():
        out.append((idx, name))
    return out

def probe_v4(src_ip, group="239.255.0.1", port=5568):
    r = socket.socket(socket.AF_INET, socket.SOCK_DGRAM, socket.IPPROTO_UDP)
    r.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    r.bind(("0.0.0.0", port))
    r.setsockopt(socket.IPPROTO_IP, socket.IP_ADD_MEMBERSHIP, socket.inet_aton(group) + socket.inet_aton(src_ip))
    r.settimeout(1.0)
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM, socket.IPPROTO_UDP)
    s.setsockopt(socket.IPPROTO_IP, socket.IP_MULTICAST_IF, socket.inet_aton(src_ip))
    s.setsockopt(socket.IPPROTO_IP, socket.IP_MULTICAST_LOOP, 1)
    try:
        s.sendto(b"hello", (group, port))
        data, addr = r.recvfrom(100)
        return f"ok from {addr}"
    except Exception as e:
        return f"FAIL {e!r}"
    finally:
        r.close(); s.close()

def probe_v6(idx, group="ff18::83:0:0:1", port=5568):
    try:
        r = socket.socket(socket.AF_INET6, socket.SOCK_DGRAM, socket.IPPROTO_UDP)
        r.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        r.bind(("::", port))
        mreq = socket.inet_pton(socket.AF_INET6, group) + struct.pack("@I", idx)
        r.setsockopt(socket.IPPROTO_IPV6, socket.IPV6_JOIN_GROUP, mreq)
        r.settimeout(1.0)
        s = socket.socket(socket.AF_INET6, socket.SOCK_DGRAM, socket.IPPROTO_UDP)
        s.setsockopt(socket.IPPROTO_IPV6, socket.IPV6_MULTICAST_IF, idx)
        s.setsockopt(socket.IPPROTO_IPV6, socket.IPV6_MULTICAST_LOOP, 1)
        s.sendto(b"hello", (group, port, 0, idx))
        data, addr = r.recvfrom(100)
        return f"ok from {addr}"
    except Exception as e:
        return f"FAIL {e!r}"

v4 = sys.argv[1:] or []
print("interfaces:", ifaces())
print("v6 loopback ::1 index 0/1 probes follow")
for ip in v4:
    try:
        print(f"v4 via {ip}: {probe_v4(ip)}")
    except Exception as e:
        print(f"v4 via {ip}: FAIL setup {e!r}")
for idx, name in ifaces():
    if name.startswith(("lo", "en0", "feth0", "loopback", "ethernet_4")):
        for g in ("ff18::83:0:0:1", "ff02::1:5ac0"):
            print(f"v6 {g} via {name} ({idx}): {probe_v6(idx, g)}")
