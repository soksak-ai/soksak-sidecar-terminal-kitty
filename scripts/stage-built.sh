#!/bin/sh
set -eu

[ "$#" -eq 2 ] && [ -n "$1" ] && [ -n "$2" ] || { echo 'usage: stage-built.sh <out> <target>' >&2; exit 2; }
out=$1
target=$2
repository=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
# An absolute candidate output is allowed only outside the source repository.
case "$out" in ''|/|.|*..*|"$repository"|"$repository"/*) echo 'stage output is unsafe or inside the source repository' >&2; exit 2 ;; esac
name=soksak-sidecar-terminal-kitty
binary=target/$target/release/$name
sdk=${SOKSAK_BUILD_DEPENDENCY_ROOT:?Make supplies SOKSAK_BUILD_DEPENDENCY_ROOT}/targets/$target/kitty-provider
[ -f "$binary" ] && [ -d "$sdk" ] || { echo "built Kitty binary or SDK is missing" >&2; exit 1; }
mkdir -p "$out"
[ ! -L "$out" ] || { echo 'stage output must not be a symbolic link' >&2; exit 2; }
if [ -e "$out/$name" ]; then
  cmp -s "$binary" "$out/$name" || { echo "staged Kitty binary conflicts with current build" >&2; exit 1; }
else
  cp "$binary" "$out/.$name.next.$$"
  chmod +x "$out/.$name.next.$$"
  mv "$out/.$name.next.$$" "$out/$name"
fi
if [ -e "$out/kitty-provider" ]; then
  diff -qr "$sdk" "$out/kitty-provider" >/dev/null || { echo "staged Kitty SDK conflicts with current build" >&2; exit 1; }
else
  mkdir -p "$out/kitty-provider.next.$$"
  (cd "$sdk" && tar -cf - .) | (cd "$out/kitty-provider.next.$$" && tar -xf -)
  find "$out/kitty-provider.next.$$" -type l -exec false {} +
  find "$out/kitty-provider.next.$$" -type f -exec chmod a-w {} +
  find "$out/kitty-provider.next.$$" -type d -exec chmod 0555 {} +
  chmod u+w "$out/kitty-provider.next.$$"
  mv "$out/kitty-provider.next.$$" "$out/kitty-provider"
  chmod 0555 "$out/kitty-provider"
fi
generated=$out/.sidecar.json.next.$$
sed "s#\"process\": \"dist/$name\"#\"process\": \"dist/$name\"#" sidecar.json > "$generated"
if [ -e "$out/sidecar.json" ]; then
  cmp -s "$generated" "$out/sidecar.json" || { echo "staged Kitty manifest conflicts with source" >&2; exit 1; }
  find "$generated" -delete
else
  mv "$generated" "$out/sidecar.json"
fi
echo "SIDECAR_STAGED target=$target output=$out/$name"
