#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
source "$SCRIPT_DIR/common.sh"

fail() {
  echo "fish_ai_query_clear: $*" >&2
  exit 1
}

tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/hiterm-fish-ai-query.XXXXXX")"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

HOME="$tmp_dir/home"
mkdir -p "$HOME"

vendor_dir="$tmp_dir/vendor"
create_stub_vendor_dir "$vendor_dir"

setup_out=""
setup_status=0
setup_out="$(
  HOME="$HOME" \
  HITERM_INIT_INTERNAL=1 \
  HITERM_SKIP_TOOL_BOOTSTRAP=1 \
  HITERM_SKIP_TERMINFO_BOOTSTRAP=1 \
  HITERM_VENDOR_DIR="$vendor_dir" \
  bash "$REPO_ROOT/assets/shell-integration/setup_fish.sh" --update-only 2>&1
)" || setup_status=$?
if [[ "$setup_status" -ne 0 ]]; then
  echo "$setup_out" >&2
  fail "setup_fish.sh failed with exit $setup_status"
fi

hiterm_fish="$HOME/.config/hiterm/fish/hiterm.fish"
[[ -f "$hiterm_fish" ]] || fail "managed init file not created at $hiterm_fish"

function_body="$(
  awk '
    /^function __hiterm_ai_query_execute$/ { in_fn = 1 }
    in_fn { print }
    in_fn && /^end$/ { exit }
  ' "$hiterm_fish"
)"

[[ "$function_body" == *'__hiterm_set_user_var kaku_ai_query "[mode:$mode] $query"'* ]] \
  || fail "kaku_ai_query user var is missing or not mode-tagged"
[[ "$function_body" == *'commandline -r ""'* ]] \
  || fail "submitted # query buffer is not cleared"

sequence_ok="$(
  awk '
    /^function __hiterm_ai_query_execute$/ { in_fn = 1 }
    in_fn && /__hiterm_set_user_var kaku_ai_query "\[mode:\$mode\] \$query"/ { saw_user_var = 1 }
    in_fn && saw_user_var && /commandline -r ""/ { saw_clear = 1 }
    in_fn && saw_clear && /commandline -f repaint/ { print "ok"; exit }
    in_fn && /^end$/ { exit }
  ' "$hiterm_fish"
)"

[[ "$sequence_ok" == "ok" ]] \
  || fail "expected query send -> commandline clear -> repaint order"

echo "fish_ai_query_clear smoke test passed"
