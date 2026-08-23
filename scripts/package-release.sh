#!/bin/sh
set -eu
dist=${1:?dist directory is required}
target=${2:?target is required}
out=${3:?archive path is required}
package=$(mktemp -d "${TMPDIR:-/tmp}/soksak-kitty-release.XXXXXX")
trap 'rm -rf -- "$package"' EXIT HUP INT TERM
mkdir -p "$package/dist"
cp sidecar.json "$package/sidecar.json"
cp LICENSE THIRD-PARTY-NOTICES "$package/"
cp "$dist/soksak-sidecar-terminal-kitty" "$package/dist/"
cp -R "$dist/kitty-provider" "$package/dist/kitty-provider"
find "$package" -type l -exec false {} +
find "$package" -type f \( -name '*.pyc' -o -name '*.pyo' \) -exec false {} +
test -f "$package/dist/kitty-provider/lib/libkitty_provider.a"
test -f "$package/dist/kitty-provider/python/kitty/fast_data_types.so"
test -d "$package/dist/kitty-provider/runtime/lib"
tar -czf "$out" -C "$package" LICENSE THIRD-PARTY-NOTICES sidecar.json dist
tar -tzf "$out" | grep -Fx 'sidecar.json' >/dev/null
tar -tzf "$out" | grep -Fx 'LICENSE' >/dev/null
tar -tzf "$out" | grep -Fx 'THIRD-PARTY-NOTICES' >/dev/null
tar -tzf "$out" | grep -Fx 'dist/soksak-sidecar-terminal-kitty' >/dev/null
printf 'packaged %s for %s\n' "$out" "$target"
