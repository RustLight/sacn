#!/usr/bin/env bash
# Runs the ignored network tests of one integration test target in parallel, each test
# in its own network namespace. A namespace has its own loopback interface, addresses,
# ports and multicast groups, so tests that bind the same sACN port or join the same
# universe can't receive each other's packets.
#
# Usage: run_isolated.sh <ipv4|ipv6> <test target> [test name filter]
#   e.g. run_isolated.sh ipv4 ipv4_tests
#        run_isolated.sh ipv6 ipv6_tests sacn_ipv6_
#
# Needs root, or CAP_SYS_ADMIN and CAP_NET_ADMIN in a container
# (docker run --cap-add=NET_ADMIN --cap-add=SYS_ADMIN).
#
# SACN_TEST_JOBS sets how many tests run at once (default: the CPU count, or 4 if it
# can't be read).
set -euo pipefail

family=${1:?usage: run_isolated.sh <ipv4|ipv6> <test target> [filter]}
target=${2:?usage: run_isolated.sh <ipv4|ipv6> <test target> [filter]}
filter=${3:-}

case "$family" in
    ipv4|ipv6) ;;
    *) echo "unknown address family: $family" >&2; exit 2 ;;
esac

jobs=${SACN_TEST_JOBS:-$(nproc 2>/dev/null || echo 4)}

# Build the test binary and find its path in cargo's output.
bin=$(cargo test --locked --test "$target" --no-run 2>&1 \
    | tee /dev/stderr \
    | sed -n "s/.*Executable tests\/$target\.rs (\(.*\))/\1/p" \
    | tail -n 1)
if [[ -z "$bin" ]]; then
    echo "could not find the test binary for $target" >&2
    exit 1
fi

# Start the longest tests first so none of them starts late and holds up the end of the
# run: the sync runthrough (the longest), then the other tests that wait on the 10 s
# universe discovery interval, then the rest.
mapfile -t tests < <(
    "$bin" --ignored --list --format terse ${filter:+"$filter"} \
        | sed -n 's/: test$//p' \
        | awk '/discover_recv_sync_runthrough/ { print; next }
               /discover/ { discovery[++d] = $0; next }
               { rest[++n] = $0 }
               END { for (i = 1; i <= d; i++) print discovery[i]; for (i = 1; i <= n; i++) print rest[i] }'
)
if (( ${#tests[@]} == 0 )); then
    echo "no ignored tests in $target match '$filter'" >&2
    exit 1
fi

out_dir=$(mktemp -d)
trap 'rm -rf "$out_dir"' EXIT

# Runs one test inside a new network namespace; called as: bash -c run_one <family> <bin> <test> <out dir>.
run_one='
set -u
family=$1 bin=$2 test=$3 out=$4/$3

ip link set dev lo up
ip link set dev lo multicast on
if [[ $family == ipv4 ]]; then
    # TEST_NETWORK_INTERFACE_IPV4 in tests/ipv4_tests.rs.
    for addr in 192.168.0.6 192.168.0.7 192.168.0.8; do
        ip addr add "$addr/24" dev lo
    done
    ip route add 224.0.0.0/4 dev lo
else
    # TEST_NETWORK_INTERFACE_IPV6 in tests/ipv6_tests.rs.
    # nodad: skip duplicate address detection so they are usable immediately.
    for addr in \
        2a02:c7f:d20a:c600:a502:2dae:7716:601b \
        2a02:c7f:d20a:c600:a502:2dae:7716:601c \
        2a02:c7f:d20a:c600:a502:2dae:7716:601d; do
        ip -6 addr add "$addr/128" dev lo nodad
    done
    ip -6 route add ff00::/8 dev lo table local 2>/dev/null || true
fi

start=$SECONDS
if "$bin" --ignored --exact "$test" --test-threads=1 >"$out.log" 2>&1; then
    echo "ok     $(( SECONDS - start ))s  $test"
else
    echo "FAILED $(( SECONDS - start ))s  $test"
    touch "$out.failed"
    exit 1
fi
'

echo "Running ${#tests[@]} tests from $target, $jobs at a time, each in its own network namespace"
start=$SECONDS
printf '%s\n' "${tests[@]}" \
    | xargs -P "$jobs" -I{} unshare --net bash -c "$run_one" run_one "$family" "$bin" {} "$out_dir" \
    || true

shopt -s nullglob
failed=("$out_dir"/*.failed)
for f in "${failed[@]}"; do
    name=$(basename "$f" .failed)
    echo
    echo "---- $name output ----"
    cat "$out_dir/$name.log"
done

echo
echo "${#tests[@]} tests, ${#failed[@]} failed, in $(( SECONDS - start ))s"
(( ${#failed[@]} == 0 ))
