#!/usr/bin/env bash
# Runs the IPv6 integration tests (tests/ipv6_tests.rs). Expects the container to be
# started on an IPv6-enabled docker network with NET_ADMIN and SYS_ADMIN, see
# .github/workflows/rust.yml and run_isolated.sh.
set -euo pipefail
script_dir=$(cd "$(dirname "$0")" && pwd)
cd "$script_dir"
[[ -f Cargo.toml ]] || cd ..

if [[ ! -e /proc/net/if_inet6 ]] || [[ "$(cat /proc/sys/net/ipv6/conf/all/disable_ipv6)" != "0" ]]; then
    echo "IPv6 is not enabled in this container; start it on a docker network created with --ipv6" >&2
    exit 1
fi

# Only the IPv6 modules: ipv6_tests.rs also pulls in ipv4_tests as a submodule,
# whose ignored tests are already covered by the test-linux-ipv4 job.
"$script_dir/run_isolated.sh" ipv6 ipv6_tests sacn_ipv6_

# IPv6 source socket option tests in src_tests.rs; these skip themselves without IPv6
# unless SACN_REQUIRE_IPV6 is set. They need only ::1, so they run in the container's
# own namespace.
SACN_REQUIRE_IPV6=1 cargo test --locked --test src_tests ipv6 -- --nocapture
