#!/usr/bin/env bash
# Runs the IPv6 integration tests (tests/ipv6_tests.rs). Expects the container to be
# started on an IPv6-enabled docker network with NET_ADMIN, see .github/workflows/rust.yml.
set -euo pipefail

if [[ ! -e /proc/net/if_inet6 ]] || [[ "$(cat /proc/sys/net/ipv6/conf/all/disable_ipv6)" != "0" ]]; then
    echo "IPv6 is not enabled in this container; start it on a docker network created with --ipv6" >&2
    exit 1
fi

# Addresses the tests bind to (TEST_NETWORK_INTERFACE_IPV6 in tests/ipv6_tests.rs).
# nodad: skip duplicate address detection so they are usable immediately.
for addr in \
    2a02:c7f:d20a:c600:a502:2dae:7716:601b \
    2a02:c7f:d20a:c600:a502:2dae:7716:601c \
    2a02:c7f:d20a:c600:a502:2dae:7716:601d; do
    ip -6 addr add "$addr/128" dev lo nodad || true
done

ip link set dev lo multicast on

ip -6 addr show
ip -6 route show table all

# Only the IPv6 modules: ipv6_tests.rs also pulls in ipv4_tests as a submodule,
# whose ignored tests are already covered by the test-linux-ipv4 job.
cargo test --locked --test ipv6_tests -- --ignored --test-threads=1 sacn_ipv6_

# IPv6 source socket option tests in src_tests.rs; these skip themselves without IPv6
# unless SACN_REQUIRE_IPV6 is set.
SACN_REQUIRE_IPV6=1 cargo test --locked --test src_tests ipv6 -- --nocapture
