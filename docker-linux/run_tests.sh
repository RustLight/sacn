#!/usr/bin/env bash
# Runs the unit tests and the IPv4 integration tests (tests/ipv4_tests.rs).
# Needs root, or NET_ADMIN and SYS_ADMIN in a container, see run_isolated.sh.
set -euo pipefail
script_dir=$(cd "$(dirname "$0")" && pwd)
cd "$script_dir"
[[ -f Cargo.toml ]] || cd ..

cargo test --locked

"$script_dir/run_isolated.sh" ipv4 ipv4_tests
