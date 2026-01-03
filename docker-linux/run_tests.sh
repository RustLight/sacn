#!/usr/bin/env bash
set -euo
ip addr add 192.168.0.6/24 dev lo || true
ip addr add 192.168.0.7/24 dev lo || true
ip addr add 192.168.0.8/24 dev lo || true
ip route add 224.0.0.0/4 dev lo || true
ip route add 239.255.0.0/16 dev lo || true

ip link set dev lo multicast on

ip a show dev lo

# requires elevated permissions to adjust these configs.
# tests pass on the default debain container without setting these.
# cat << EOF >> /etc/sysctl.conf
# net.ipv4.conf.all.rp_filter = 0
# net.ipv4.conf.default.rp_filter = 0
# net.ipv4.conf.default.rp_filter = 0
# net.ipv4.conf.all.accept_local = 1
# net.ipv4.conf.all.mc_forwarding = 1
# EOF
# sysctl -p

cargo test
cargo test --test ipv4_tests -- --ignored --test-threads=1
