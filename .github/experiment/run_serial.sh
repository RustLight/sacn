#!/usr/bin/env bash
# Experiment: run a test target's ignored tests one at a time with a timeout each.
# Usage: run_serial.sh <target> [filter]
set -uo pipefail
target=$1; filter=${2:-}
cargo test --locked --test "$target" --no-run 2>&1 | tee build.log
bin=$(sed -n "s/.*Executable tests[\\/]$target\.rs (\(.*\))/\1/p" build.log | tail -n1)
echo "binary: $bin"
tests=()
while IFS= read -r line; do tests+=("$line"); done < <("$bin" --ignored --list --format terse ${filter:+"$filter"} | sed -n 's/: test$//p' | tr -d '\r')
if [[ $RUNNER_OS == macOS ]]; then to() { perl -e 'alarm shift; exec @ARGV or die' "$@"; }; else to() { timeout "$@"; }; fi
pass=0; fail=0; failed=()
for t in "${tests[@]}"; do
  start=$SECONDS
  if to 120 "$bin" --ignored --exact "$t" --test-threads=1 > out.log 2>&1; then
    echo "ok     $((SECONDS-start))s $t"; pass=$((pass+1))
  else
    rc=$?
    echo "FAILED $((SECONDS-start))s (rc $rc) $t"; fail=$((fail+1)); failed+=("$t")
    echo "::group::$t output"; tail -n 40 out.log; echo "::endgroup::"
  fi
done
echo "SUMMARY $target: ${#tests[@]} tests, $pass passed, $fail failed"
(( fail )) && printf 'FAILED: %s\n' "${failed[@]}"
echo "### $RUNNER_OS $SETUP $target: $pass/${#tests[@]} passed" >> "$GITHUB_STEP_SUMMARY"
for t in ${failed[@]+"${failed[@]}"}; do echo "- failed: \`$t\`" >> "$GITHUB_STEP_SUMMARY"; done
exit 0
