#!/usr/bin/env bash
# Verify a challenge's starter code is a valid starting point:
#   1. `src/starter.rs` compiles when it stands in for `src/lib.rs`
#   2. the challenge tests FAIL against it (otherwise the starter gives the answer away)
#   3. `src/starter.rs` is rustfmt-clean (`cargo fmt` skips it - it is not a crate target)
# `src/lib.rs` is restored afterwards, including on failure.
#
# Usage: scripts/check-starter.sh <slug> [<slug> ...]
set -uo pipefail

CHALLENGES_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../challenges" && pwd)"

# The cargo commands below need the challenges workspace, not the caller's cwd.
cd "$CHALLENGES_DIR/.." || exit 1

status=0

for slug in "$@"; do
    dir="$CHALLENGES_DIR/$slug"
    if [ ! -d "$dir" ]; then
        echo "FAIL $slug: no such challenge directory"
        status=1
        continue
    fi

    backup="$(mktemp)"
    cp "$dir/src/lib.rs" "$backup"
    cp "$dir/src/starter.rs" "$dir/src/lib.rs"

    # shellcheck disable=SC2064
    trap "cp '$backup' '$dir/src/lib.rs'; rm -f '$backup'" EXIT

    problems=()

    if ! cargo build -p "$slug" >/dev/null 2>&1; then
        problems+=("starter.rs does not compile")
    elif cargo test -p "$slug" >/dev/null 2>&1; then
        problems+=("tests PASS against starter.rs (starter leaks the solution)")
    fi

    # Check the real starter.rs, not the copy standing in as lib.rs.
    if ! rustfmt --edition 2021 --check "$dir/src/starter.rs" >/dev/null 2>&1; then
        problems+=("starter.rs is not rustfmt-clean (rustfmt --edition 2021 src/starter.rs)")
    fi

    if [ ${#problems[@]} -gt 0 ]; then
        printf 'FAIL %s: %s\n' "$slug" "$(
            IFS='; '
            echo "${problems[*]}"
        )"
        status=1
    else
        echo "ok   $slug: starter compiles, tests fail as expected, rustfmt clean"
    fi

    cp "$backup" "$dir/src/lib.rs"
    rm -f "$backup"
    trap - EXIT
done

exit $status
