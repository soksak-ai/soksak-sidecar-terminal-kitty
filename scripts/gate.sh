#!/bin/sh
set -eu

[ "$#" -eq 2 ] && [ -n "$1" ] && [ -n "$2" ] || { echo 'usage: gate.sh <target> <stage-out>' >&2; exit 2; }
target=$1
case "$2" in /*) stage_out=$2 ;; *) stage_out=$PWD/$2 ;; esac
[ -d "$stage_out" ] || { echo "stage output is missing: $stage_out" >&2; exit 1; }
sdk=${SOKSAK_BUILD_DEPENDENCY_ROOT:?Make supplies SOKSAK_BUILD_DEPENDENCY_ROOT}/targets/$target/kitty-provider
[ -d "$sdk/runtime/lib" ] || { echo "Kitty SDK runtime is missing: $sdk/runtime/lib" >&2; exit 1; }
node --test scripts/*.test.mjs
cargo test --locked --release --target "$target" --no-run
test_sdk=target/$target/release/deps/kitty-provider
if [ ! -e "$test_sdk" ] || ! diff -qr "$sdk" "$test_sdk" >/dev/null; then
  next=$test_sdk.next.$$
  previous=$test_sdk.previous.$$
  trap 'chmod -R u+w "$next" "$previous" 2>/dev/null || true; rm -rf -- "$next" "$previous"' EXIT HUP INT TERM
  mkdir -p "$next"
  (cd "$sdk" && tar -cf - .) | (cd "$next" && tar -xf -)
  find "$next" -type l -exec false {} +
  find "$next" -type f -exec chmod a-w {} +
  find "$next" -type d -exec chmod 0555 {} +
  chmod u+w "$next"
  if [ -e "$test_sdk" ]; then chmod u+w "$test_sdk"; mv "$test_sdk" "$previous"; fi
  mv "$next" "$test_sdk"
  chmod 0555 "$test_sdk"
  if [ -e "$previous" ]; then chmod -R u+w "$previous"; rm -rf -- "$previous"; fi
  trap - EXIT HUP INT TERM
fi
SOKSAK_STAGE_OUT="$stage_out" cargo test --locked --release --target "$target"
