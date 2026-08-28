#!/bin/sh
set -eu

[ "$#" -eq 2 ] && [ -n "$1" ] && [ -n "$2" ] || { echo 'usage: stage-built.sh <out> <target>' >&2; exit 2; }
out=$1
target=$2
repository=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
case "$out" in ''|/|.|*..*|"$repository"|"$repository"/*) echo 'stage output is unsafe or inside the source repository' >&2; exit 2 ;; esac

name=soksak-sidecar-terminal-kitty
binary=target/$target/release/$name
sdk=${SOKSAK_BUILD_DEPENDENCY_ROOT:?Make supplies SOKSAK_BUILD_DEPENDENCY_ROOT}/targets/$target/kitty-provider
[ -f "$binary" ] && [ -d "$sdk" ] || { echo 'built Kitty binary or SDK is missing' >&2; exit 1; }

mkdir -p "$out"
[ ! -L "$out" ] || { echo 'STAGED_STATE_INVALID: output is a symbolic link' >&2; exit 1; }
staged_binary=$out/$name
staged_sdk=$out/kitty-provider
staged_manifest=$out/sidecar.json
next_binary=$out/.$name.next.$$
next_sdk=$out/.kitty-provider.next.$$
next_manifest=$out/.sidecar.json.next.$$
previous_sdk=$out/.kitty-provider.previous.$$
cleanup() {
  chmod -R u+w "$next_sdk" "$previous_sdk" 2>/dev/null || true
  rm -f "$next_binary" "$next_manifest"
  rm -rf "$next_sdk" "$previous_sdk"
}
trap cleanup EXIT HUP INT TERM

cp "$binary" "$next_binary"
chmod +x "$next_binary"
mkdir "$next_sdk"
(cd "$sdk" && tar -cf - .) | (cd "$next_sdk" && tar -xf -)
find "$next_sdk" -type l -exec false {} +
find "$next_sdk" -type f -exec chmod a-w {} +
find "$next_sdk" -type d -exec chmod 0555 {} +
chmod u+w "$next_sdk"
cp sidecar.json "$next_manifest"

for path in "$staged_binary" "$staged_sdk" "$staged_manifest"; do
  [ ! -L "$path" ] || { echo "STAGED_STATE_INVALID: symbolic link: $path" >&2; exit 1; }
done
identity() {
  node -e 'const {readFileSync}=require("node:fs");const v=JSON.parse(readFileSync(process.argv[1],"utf8"));if(typeof v.id!=="string"||typeof v.version!=="string")process.exit(1);process.stdout.write(v.id+"\n"+v.version)' "$1"
}
next_identity=$(identity "$next_manifest") || { echo 'STAGED_STATE_INVALID: source manifest identity' >&2; exit 1; }
next_id=$(printf '%s\n' "$next_identity" | sed -n '1p')
next_version=$(printf '%s\n' "$next_identity" | sed -n '2p')

if [ -f "$staged_manifest" ]; then
  current_identity=$(identity "$staged_manifest") || { echo 'STAGED_STATE_INVALID: staged manifest identity' >&2; exit 1; }
  current_id=$(printf '%s\n' "$current_identity" | sed -n '1p')
  current_version=$(printf '%s\n' "$current_identity" | sed -n '2p')
  [ "$current_id" = "$next_id" ] || { echo 'STAGED_STATE_INVALID: component identity changed' >&2; exit 1; }
  if cmp -s "$next_manifest" "$staged_manifest"; then
    [ ! -e "$staged_binary" ] || cmp -s "$next_binary" "$staged_binary" || { echo "STAGED_BUILD_NOT_DETERMINISTIC: $next_id@$next_version" >&2; exit 1; }
    [ ! -e "$staged_sdk" ] || diff -qr "$next_sdk" "$staged_sdk" >/dev/null || { echo "STAGED_BUILD_NOT_DETERMINISTIC: $next_id@$next_version SDK" >&2; exit 1; }
    if [ -f "$staged_binary" ] && [ -d "$staged_sdk" ]; then
      echo "KITTY_STAGED_UNCHANGED target=$target output=$staged_binary"
      exit 0
    fi
  else
    [ "$current_version" != "$next_version" ] || { echo "STAGED_MANIFEST_CONFLICT: $next_id@$next_version" >&2; exit 1; }
  fi
elif [ -e "$staged_manifest" ]; then
  echo "STAGED_STATE_INVALID: manifest is not a regular file: $staged_manifest" >&2
  exit 1
elif [ -e "$staged_binary" ] && ! cmp -s "$next_binary" "$staged_binary"; then
  echo "STAGED_STATE_INVALID: binary has no matching manifest: $staged_binary" >&2
  exit 1
elif [ -e "$staged_sdk" ] && { [ ! -d "$staged_sdk" ] || ! diff -qr "$next_sdk" "$staged_sdk" >/dev/null; }; then
  echo "STAGED_STATE_INVALID: SDK has no matching manifest: $staged_sdk" >&2
  exit 1
fi

mv "$next_binary" "$staged_binary"
if [ -e "$staged_sdk" ]; then
  chmod u+w "$staged_sdk"
  mv "$staged_sdk" "$previous_sdk"
fi
mv "$next_sdk" "$staged_sdk"
chmod 0555 "$staged_sdk"
mv "$next_manifest" "$staged_manifest"
if [ -e "$previous_sdk" ]; then chmod -R u+w "$previous_sdk"; rm -rf "$previous_sdk"; fi
echo "KITTY_STAGED target=$target output=$staged_binary"
