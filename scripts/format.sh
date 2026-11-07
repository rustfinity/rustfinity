#!/bin/bash
# Format this repo with its own settings (not the parent monorepo's):
#
#   - Markdown, JSON, YAML  Prettier (.prettierrc / .prettierignore)
#   - Cargo.toml files      Taplo
#   - Rust crate sources    cargo fmt (challenge src/starter.rs files are not
#                           crate targets, so they are left alone on purpose)
#
#   scripts/format.sh          rewrite files
#   scripts/format.sh --check  report unformatted files, exit non-zero if any
#
# Prettier and Taplo run through bunx when bun is installed, npx otherwise.
set -uo pipefail

cd "$(dirname "$0")/.."

PRETTIER="prettier@3.7.4"
TAPLO="@taplo/cli@0.7.0"

check=false
[ "${1:-}" = "--check" ] && check=true

run_js() {
  if command -v bunx >/dev/null 2>&1; then
    bunx "$@"
  else
    npx --yes "$@"
  fi
}

prettier() {
  if $check; then
    run_js "$PRETTIER" --check "**/*.{md,json,yaml,yml}"
  else
    run_js "$PRETTIER" --write --log-level warn "**/*.{md,json,yaml,yml}"
  fi
}

taplo() {
  if $check; then
    run_js "$TAPLO" fmt --check
  else
    run_js "$TAPLO" fmt
  fi
}

cargo_fmt() {
  if $check; then
    cargo fmt --all --check
  else
    cargo fmt --all
  fi
}

failed=0
for step in prettier taplo cargo_fmt; do
  if output=$("$step" 2>&1); then
    echo "ok   $step"
  else
    echo "FAIL $step"
    echo "$output" | sed 's/^/     /'
    failed=1
  fi
done

exit $failed
