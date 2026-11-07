#!/bin/bash
# Format the Markdown in this repo with its own Prettier settings
# (.prettierrc / .prettierignore here, not the parent monorepo's).
#
#   scripts/format.sh          rewrite files
#   scripts/format.sh --check  list unformatted files, exit non-zero if any
set -euo pipefail

cd "$(dirname "$0")/.."

PRETTIER="prettier@3.7.4"
mode="--write"
[ "${1:-}" = "--check" ] && mode="--check"

if command -v bunx >/dev/null 2>&1; then
  bunx "$PRETTIER" "$mode" "**/*.md"
else
  npx --yes "$PRETTIER" "$mode" "**/*.md"
fi
