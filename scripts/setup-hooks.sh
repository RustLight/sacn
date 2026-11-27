#!/usr/bin/env bash
# Installs prek and enables the Git hooks from .pre-commit-config.yaml.
#
# Coding agents run this at the start of every session (see AGENTS.md), and
# people can run it once instead of following the manual steps in
# CONTRIBUTING.md. It is safe to run repeatedly: each step is skipped when it
# is already done.
#
# An existing prek is used as is. Otherwise the script installs the
# `prek-version` the CI lint job uses.

set -uo pipefail

cd "$(dirname "$0")/.." || exit 1

warn() { echo "setup-hooks: warning: $*"; }
die() { echo "setup-hooks: error: $*"; exit 1; }

version=$(sed -n "s/^ *prek-version: *['\"]\{0,1\}\([0-9.]*\)['\"]\{0,1\} *$/\1/p" .github/workflows/rust.yml | head -n 1)
[ -n "$version" ] || die "could not read prek-version from .github/workflows/rust.yml"

# Tools installed with --user, uv or pipx land here.
case ":$PATH:" in
  *":$HOME/.local/bin:"*) ;;
  *) export PATH="$HOME/.local/bin:$PATH" ;;
esac

installed() { command -v prek >/dev/null 2>&1; }

if ! installed; then
  echo "setup-hooks: installing prek $version"
  if command -v uv >/dev/null 2>&1; then
    uv tool install --quiet --force "prek==$version" >&2
  fi
  if ! installed && command -v pipx >/dev/null 2>&1; then
    pipx install --force "prek==$version" >&2
  fi
  if ! installed && command -v python3 >/dev/null 2>&1; then
    python3 -m pip install --quiet --user "prek==$version" >&2 ||
      python3 -m pip install --quiet --user --break-system-packages "prek==$version" >&2
  fi
  if ! installed && command -v curl >/dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 -LsSf \
      "https://github.com/j178/prek/releases/download/v$version/prek-installer.sh" | sh >&2
  fi
  installed || die "could not install prek $version; see https://prek.j178.dev/installation/"
fi

# Make prek available to the rest of a Claude Code session.
if [ -n "${CLAUDE_ENV_FILE:-}" ]; then
  echo "export PATH=\"$HOME/.local/bin:\$PATH\"" >>"$CLAUDE_ENV_FILE"
fi

prek install >&2 || die "prek install failed"

# Download each hook's tools now, so the first commit doesn't, and so a hook
# that can't be installed shows up at the start of the session. Sandboxes
# that block go.dev can't install actionlint, for example; the other hooks
# still work there.
failed=()
for hook in $(prek list 2>/dev/null | sed 's/^.*://'); do
  if ! out=$(prek prepare-hooks "$hook" 2>&1); then
    failed+=("$hook")
    warn "could not install the $hook hook:"
    echo "$out" | sed 's/^/  /'
  fi
done
if [ ${#failed[@]} -gt 0 ]; then
  skip=$(IFS=,; echo "${failed[*]}")
  warn "commit with SKIP=$skip, check what those hooks cover another way," \
    "and say in the pull request which hooks did not run (see AGENTS.md)."
fi

echo "setup-hooks: prek and the Git hooks are installed"
