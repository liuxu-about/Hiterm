#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
source "$SCRIPT_DIR/common.sh"

fail() {
  echo "cmd_backspace_binding: $*" >&2
  exit 1
}

tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/hiterm-cmd-backspace-binding.XXXXXX")"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

HOME="$tmp_dir/home"
ZDOTDIR="$HOME"
mkdir -p "$HOME"

vendor_dir="$tmp_dir/vendor"
create_stub_vendor_dir "$vendor_dir"
mkdir -p "$vendor_dir/zsh-syntax-highlighting"

setup_out=""
setup_status=0
setup_out="$(
  HOME="$HOME" \
  ZDOTDIR="$ZDOTDIR" \
  HITERM_INIT_INTERNAL=1 \
  HITERM_SKIP_TOOL_BOOTSTRAP=1 \
  HITERM_SKIP_TERMINFO_BOOTSTRAP=1 \
  HITERM_VENDOR_DIR="$vendor_dir" \
  bash "$REPO_ROOT/assets/shell-integration/setup_zsh.sh" --update-only 2>&1
)" || setup_status=$?
if [[ "$setup_status" -ne 0 ]]; then
  echo "$setup_out" >&2
  fail "setup_zsh.sh failed with exit $setup_status"
fi

hiterm_zsh="$HOME/.config/hiterm/zsh/hiterm.zsh"
[[ -f "$hiterm_zsh" ]] || fail "managed init file not created at $hiterm_zsh"

binding_out=""
if ! binding_out="$(
  TERM=xterm-256color \
  HOME="$HOME" \
  ZDOTDIR="$ZDOTDIR" \
  zsh -f -c '
source "$HOME/.config/hiterm/zsh/hiterm.zsh"
bindkey "^U"
' 2>&1
)"; then
  echo "$binding_out" >&2
  fail "sourcing generated hiterm.zsh failed"
fi

case "$binding_out" in
  *'"^U" backward-kill-line'* ) ;;
  * ) echo "$binding_out" >&2; fail "Ctrl+U is not bound to backward-kill-line" ;;
esac

case "$binding_out" in
  *kill-whole-line* ) echo "$binding_out" >&2; fail "Ctrl+U still kills the whole line" ;;
esac

echo "cmd_backspace_binding smoke test passed"
