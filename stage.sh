#!/bin/sh
set -eu
dist=${1:-dist}
target=${2:-}
sdk=${SOKSAK_KITTY_PROVIDER_SDK:?SOKSAK_KITTY_PROVIDER_SDK is required}
target_args=""
release_dir=release
if [ -n "$target" ]; then
  target_args="--target $target"
  release_dir="$target/release"
fi
SOKSAK_KITTY_BUNDLE_BUILD=1 cargo build --release $target_args --bin soksak-sidecar-terminal-kitty
mkdir -p "$dist"
binary="${CARGO_TARGET_DIR:-target}/$release_dir/soksak-sidecar-terminal-kitty"
cp "$binary" "$dist/.soksak-sidecar-terminal-kitty.tmp"
chmod +x "$dist/.soksak-sidecar-terminal-kitty.tmp"
mv -f "$dist/.soksak-sidecar-terminal-kitty.tmp" "$dist/soksak-sidecar-terminal-kitty"
rm -rf "$dist/kitty-provider.next"
cp -R "$sdk" "$dist/kitty-provider.next"
find "$dist/kitty-provider.next" -type f \( -name '*.pyc' -o -name '*.pyo' \) -delete
rm -rf "$dist/kitty-provider"
mv "$dist/kitty-provider.next" "$dist/kitty-provider"
find "$dist/kitty-provider" -type l -exec false {} +
find "$dist/kitty-provider" -type f \( -name '*.pyc' -o -name '*.pyo' \) -exec false {} +
if command -v otool >/dev/null 2>&1; then
  if otool -l "$dist/soksak-sidecar-terminal-kitty" | grep -F "$sdk" >/dev/null; then
    echo "staged Kitty binary retains its build SDK path" >&2
    exit 1
  fi
  otool -l "$dist/soksak-sidecar-terminal-kitty" | grep -F '@loader_path/kitty-provider/runtime/lib' >/dev/null
fi
