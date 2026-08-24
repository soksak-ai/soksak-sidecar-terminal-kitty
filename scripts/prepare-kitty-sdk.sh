#!/bin/sh
set -eu

[ "$#" -eq 2 ] && [ -n "$1" ] && [ -n "$2" ] || { echo 'usage: prepare-kitty-sdk.sh <target> <build-root>' >&2; exit 2; }
target=$1
build_root=$2
case "$build_root" in /*|*..*) echo 'build root must be repository-relative' >&2; exit 2 ;; esac
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
build_root=$root/$build_root
receipt=$build_root/receipts/$target.json
if [ -f "$receipt" ]; then
  soksak-validate build-receipt "$receipt" --dependencies "$root/build-dependencies.json" --output-root "$build_root"
  echo "KITTY_SDK_REUSED target=$target"
  exit 0
fi
[ ! -e "$build_root/targets/$target" ] || { echo "unverified Kitty SDK output exists for $target" >&2; exit 79; }

mkdir -p "$build_root/sources" "$build_root/.transactions"
transaction=$build_root/.transactions/prepare.$target.$$
source_next=$build_root/sources/.next.$$
stage=$build_root/builds/$target
cleanup() {
  for candidate in "$transaction" "$source_next"; do
    case "$candidate" in "$build_root"/.transactions/*|"$build_root"/sources/.next.*) rm -rf -- "$candidate" ;; esac
  done
}
trap cleanup EXIT HUP INT TERM
mkdir -p "$transaction/targets/$target/kitty-provider"
resolution=$transaction/resolution.json
soksak-validate build-dependencies "$root/build-dependencies.json" --dependency kitty-provider-sdk --target "$target" > "$resolution"
repository=$(node -e 'const v=require(process.argv[1]);process.stdout.write(v.repository)' "$resolution")
commit=$(node -e 'const v=require(process.argv[1]);process.stdout.write(v.commit)' "$resolution")
python_version=$(node -e 'const v=require(process.argv[1]);process.stdout.write(v.tools.python)' "$resolution")
source=$build_root/sources/$commit
if [ -e "$source" ]; then
  [ -d "$source/.git" ] && [ "$(git -C "$source" remote get-url origin)" = "$repository" ] && \
    [ "$(git -C "$source" rev-parse HEAD)" = "$commit" ] && [ -z "$(git -C "$source" status --porcelain)" ] || {
      echo "cached Kitty source differs from build-dependencies.json" >&2; exit 79;
    }
else
  git init -q "$source_next"
  git -C "$source_next" remote add origin "$repository"
  git -C "$source_next" fetch -q --depth 1 origin "$commit"
  git -C "$source_next" -c advice.detachedHead=false checkout -q FETCH_HEAD
  [ "$(git -C "$source_next" rev-parse HEAD)" = "$commit" ] && [ -z "$(git -C "$source_next" status --porcelain)" ] || {
    echo "Kitty source checkout did not materialize the declared commit" >&2; exit 79;
  }
  mv "$source_next" "$source"
fi

if [ -e "$stage" ]; then
  [ -d "$stage" ] && [ -f "$stage/.soksak-build-resolution.json" ] && \
    cmp -s "$resolution" "$stage/.soksak-build-resolution.json" || { echo "Kitty build cache differs from declared inputs" >&2; exit 79; }
else
  mkdir -p "$stage"
  git -C "$source" archive --format=tar "$commit" | tar -xf - -C "$stage"
  cp "$resolution" "$stage/.soksak-build-resolution.json"
fi
(cd "$stage" && python3 setup.py kitty-provider-sdk)
sdk=$stage/build/kitty-provider
for required in "$sdk/lib/libkitty_provider.a" "$sdk/python/kitty/fast_data_types.so" "$sdk/python-config.json"; do
  [ -f "$required" ] || { echo "Kitty SDK output is missing: $required" >&2; exit 79; }
done
cp -R "$sdk/." "$transaction/targets/$target/kitty-provider"
find "$transaction/targets/$target/kitty-provider" -type f \( -name '*.pyc' -o -name '*.pyo' \) -delete
find "$transaction/targets/$target/kitty-provider" -name '.DS_Store' -type f -delete
printf '%s\n' "$commit" > "$transaction/targets/$target/kitty-provider/source-commit.txt"
printf '%s\n' "$python_version" > "$transaction/targets/$target/kitty-provider/python-version.txt"
if find "$transaction/targets/$target/kitty-provider" -type l -print -quit | grep -q .; then
  echo "Kitty SDK output contains a symbolic link" >&2; exit 79
fi
find "$transaction/targets/$target/kitty-provider" -type f -exec chmod a-w {} +
find "$transaction/targets/$target/kitty-provider" -type d -exec chmod 0555 {} +
mkdir -p "$transaction/receipts"
soksak-validate build-receipt-create "$root/build-dependencies.json" --dependency kitty-provider-sdk \
  --target "$target" --output-root "$transaction" --out "$transaction/receipts/$target.json"
soksak-validate build-receipt "$transaction/receipts/$target.json" --dependencies "$root/build-dependencies.json" --output-root "$transaction"
mkdir -p "$build_root/targets" "$build_root/receipts"
[ ! -e "$build_root/targets/$target" ] && [ ! -e "$receipt" ] || { echo "Kitty SDK output appeared concurrently" >&2; exit 79; }
mv "$transaction/targets/$target" "$build_root/targets/$target"
mv "$transaction/receipts/$target.json" "$receipt"
soksak-validate build-receipt "$receipt" --dependencies "$root/build-dependencies.json" --output-root "$build_root"
echo "KITTY_SDK_READY target=$target"
