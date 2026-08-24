#!/bin/sh
set -eu

[ "$#" -eq 1 ] && [ -n "$1" ] || { echo 'usage: gate.sh <target>' >&2; exit 2; }
target=$1
sdk=${SOKSAK_BUILD_DEPENDENCY_ROOT:?Make supplies SOKSAK_BUILD_DEPENDENCY_ROOT}/targets/$target/kitty-provider
[ -d "$sdk/runtime/lib" ] || { echo "Kitty SDK runtime is missing: $sdk/runtime/lib" >&2; exit 1; }
cargo test --locked --release --target "$target" --no-run
test_sdk=target/$target/release/deps/kitty-provider
if [ -e "$test_sdk" ]; then
  diff -qr "$sdk" "$test_sdk" >/dev/null || { echo "Kitty test SDK conflicts with receipt output" >&2; exit 1; }
else
  mkdir -p "$test_sdk.next.$$"
  (cd "$sdk" && tar -cf - .) | (cd "$test_sdk.next.$$" && tar -xf -)
  find "$test_sdk.next.$$" -type l -exec false {} +
  find "$test_sdk.next.$$" -type f -exec chmod a-w {} +
  find "$test_sdk.next.$$" -type d -exec chmod 0555 {} +
  chmod u+w "$test_sdk.next.$$"
  mv "$test_sdk.next.$$" "$test_sdk"
  chmod 0555 "$test_sdk"
fi
cargo test --locked --release --target "$target"
