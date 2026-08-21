#!/bin/sh
set -eu
dist=${1:-dist}
sdk=${SOKSAK_KITTY_PROVIDER_SDK:?SOKSAK_KITTY_PROVIDER_SDK is required}
SOKSAK_KITTY_BUNDLE_BUILD=1 cargo build --release --bin soksak-sidecar-terminal-kitty
mkdir -p "$dist"
cp target/release/soksak-sidecar-terminal-kitty "$dist/.soksak-sidecar-terminal-kitty.tmp"
chmod +x "$dist/.soksak-sidecar-terminal-kitty.tmp"
mv -f "$dist/.soksak-sidecar-terminal-kitty.tmp" "$dist/soksak-sidecar-terminal-kitty"
rm -rf "$dist/kitty-provider.next"
cp -R "$sdk" "$dist/kitty-provider.next"
rm -rf "$dist/kitty-provider"
mv "$dist/kitty-provider.next" "$dist/kitty-provider"
find "$dist/kitty-provider" -type l -exec false {} +
if command -v otool >/dev/null 2>&1; then
  if otool -l "$dist/soksak-sidecar-terminal-kitty" | grep -F "$sdk" >/dev/null; then
    echo "staged Kitty binary retains its build SDK path" >&2
    exit 1
  fi
  otool -l "$dist/soksak-sidecar-terminal-kitty" | grep -F '@loader_path/kitty-provider/runtime/lib' >/dev/null
fi
