#!/usr/bin/env bash
# Runs test suites with coverage instrumentation, then writes cargo-llvm-cov's
# reports to target/llvm-cov/: lcov.info, summary.json and html/.
#
# Usage: run_coverage.sh <suite>...
#   unit: cargo test (without the doctests)
#   ipv4: run_tests.sh, the IPv4 network tests
#   ipv6: run_tests_ipv6.sh, the IPv6 network tests
# e.g. run_coverage.sh unit ipv4 ipv6 for one report covering all of them. CI
# records each suite in its own job and merges their lcov.info files.
#
# Needs cargo-llvm-cov, which the Docker image includes. For ipv4 and ipv6, start the
# container as for the suites' own scripts: with NET_ADMIN and SYS_ADMIN, and on
# an IPv6-enabled docker network for ipv6. The IPv4 tests run in their own
# network namespaces, so they don't mind the IPv6 network. To get the reports
# out of the container, mount the repository and run the script from there, see
# CONTRIBUTING.md.
set -euo pipefail
script_dir=$(cd "$(dirname "$0")" && pwd)
cd "$script_dir"
[[ -f Cargo.toml ]] || cd ..

if (( $# == 0 )); then
    echo "usage: run_coverage.sh <unit|ipv4|ipv6>..." >&2
    exit 2
fi
for suite in "$@"; do
    case "$suite" in
        unit|ipv4|ipv6) ;;
        *) echo "unknown suite: $suite" >&2; exit 2 ;;
    esac
done

# Instrument every cargo build below, and write the profile data to target/.
cargo llvm-cov clean --workspace
# shellcheck source=/dev/null
source <(cargo llvm-cov show-env --sh)

for suite in "$@"; do
    case "$suite" in
        unit) cargo test --locked --lib --bins --tests ;;
        ipv4) "$script_dir/run_tests.sh" ;;
        ipv6) "$script_dir/run_tests_ipv6.sh" ;;
    esac
done

cargo llvm-cov report --html
cargo llvm-cov report --lcov --output-path target/llvm-cov/lcov.info
cargo llvm-cov report --json --summary-only --output-path target/llvm-cov/summary.json
cargo llvm-cov report --summary-only
