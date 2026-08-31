#!/bin/sh
set -eu

[ "$#" -eq 2 ] && [ -n "$1" ] && [ -n "$2" ] || { echo 'usage: prepare-kitty-sdk.sh <target> <build-root>' >&2; exit 2; }
target=$1
build_root=$2
case "$build_root" in /*|*..*) echo 'build root must be repository-relative' >&2; exit 2 ;; esac
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
build_root=$root/$build_root
receipt=$build_root/receipts/$target.json
mkdir -p "$build_root/sources" "$build_root/.transactions" "$build_root/.locks"
transaction=$build_root/.transactions/prepare.$target.$$
source_next=$build_root/sources/.next.$$
lock=$build_root/.locks/$target
remove_tree() {
  candidate=$1
  case "$candidate" in "$build_root"/.transactions/*|"$build_root"/targets/"$target") ;; *) echo "refusing unsafe transaction cleanup: $candidate" >&2; exit 79 ;; esac
  [ ! -e "$candidate" ] || chmod -R u+w "$candidate" 2>/dev/null || true
  rm -rf -- "$candidate"
}
if ! mkdir "$lock" 2>/dev/null; then
  owner=$(cat "$lock/owner" 2>/dev/null || true)
  case "$owner" in ''|*[!0-9]*) ;; *) kill -0 "$owner" 2>/dev/null && { echo "Kitty SDK preparation is already running for $target" >&2; exit 79; } ;; esac
  rmdir "$lock" 2>/dev/null || { echo "Kitty SDK lock has invalid contents: $lock" >&2; exit 79; }
  mkdir "$lock"
fi
printf '%s\n' "$$" > "$lock/owner"
cleanup() {
  for candidate in "$transaction" "$source_next"; do
    case "$candidate" in
      "$build_root"/.transactions/*) remove_tree "$candidate" ;;
      "$build_root"/sources/.next.*) rm -rf -- "$candidate" ;;
    esac
  done
  rm -f "$lock/owner"
  rmdir "$lock" 2>/dev/null || true
}
trap cleanup EXIT HUP INT TERM
mkdir -p "$transaction/targets/$target/kitty-provider"
resolution=$transaction/resolution.json
soksak-sdk validate build-dependencies "$root/build-dependencies.json" --dependency kitty-provider-sdk --target "$target" > "$resolution"
repository=$(node -e 'const v=require(process.argv[1]);process.stdout.write(v.repository)' "$resolution")
commit=$(node -e 'const v=require(process.argv[1]);process.stdout.write(v.commit)' "$resolution")
python_version=$(node -e 'const v=require(process.argv[1]);process.stdout.write(v.tools.python)' "$resolution")
current_target=$build_root/targets/$target
current_source=$current_target/kitty-provider/source-commit.txt
for stale in "$build_root"/.transactions/prepare."$target".*; do
  [ -d "$stale" ] && [ "$stale" != "$transaction" ] || continue
  stale_owner=${stale##*.}
  case "$stale_owner" in ''|*[!0-9]*) echo "Kitty SDK transaction has no process owner: $stale" >&2; exit 79 ;; esac
  kill -0 "$stale_owner" 2>/dev/null && { echo "Kitty SDK transaction is still owned by process $stale_owner" >&2; exit 79; }
  if [ -e "$stale/previous-target" ]; then
    [ -f "$receipt" ] && soksak-sdk validate build-receipt "$receipt" --dependencies "$root/build-dependencies.json" --output-root "$build_root" >/dev/null 2>&1 || {
      echo "interrupted Kitty SDK replacement requires a valid current target: $stale" >&2; exit 79;
    }
  fi
  remove_tree "$stale"
done
if [ -f "$receipt" ]; then
  if soksak-sdk validate build-receipt "$receipt" --dependencies "$root/build-dependencies.json" --output-root "$build_root" > "$transaction/current-validation.log" 2>&1; then
    cat "$transaction/current-validation.log"
    echo "KITTY_SDK_REUSED target=$target"
    exit 0
  fi
  if [ ! -f "$current_source" ] || [ "$(cat "$current_source")" = "$commit" ]; then
    cat "$transaction/current-validation.log" >&2
    echo "current Kitty SDK receipt is invalid for its declared source" >&2
    exit 79
  fi
elif [ -e "$current_target" ]; then
  echo "unverified Kitty SDK output exists for $target" >&2
  exit 79
fi
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

stage=$build_root/builds/$target/$commit
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
node "$root/scripts/normalize-kitty-sdk.mjs" "$sdk"
for required in "$sdk/lib/libkitty_provider.a" "$sdk/python/kitty/fast_data_types.so" "$sdk/python-config.json"; do
  [ -f "$required" ] || { echo "Kitty SDK output is missing: $required" >&2; exit 79; }
done
cp -R "$sdk/." "$transaction/targets/$target/kitty-provider"
find "$transaction/targets/$target/kitty-provider" -type f \( -name '*.pyc' -o -name '*.pyo' \) -delete
find "$transaction/targets/$target/kitty-provider" -name '.DS_Store' -type f -delete
# What the embedded interpreter never reads at run time: the standard library's own test suite, the
# headers and static archive that exist to build against it, the editor, and the installer. A unit
# that shipped them would carry a third of its size for nothing.
runtime_lib=$transaction/targets/$target/kitty-provider/runtime/lib
if [ -d "$runtime_lib" ]; then
  python_lib=$(find "$runtime_lib" -maxdepth 1 -type d -name 'python3.*' -print -quit)
  if [ -n "$python_lib" ]; then
    rm -rf -- "$python_lib/test" "$python_lib/idlelib" "$python_lib/ensurepip" "$python_lib/pydoc_data" \
      "$python_lib/tkinter" "$python_lib/turtledemo" "$python_lib/lib2to3"
    find "$python_lib" -maxdepth 1 -type d -name 'config-*' -exec rm -rf -- {} +
    find "$python_lib" -type d -name 'tests' -prune -exec rm -rf -- {} +
    find "$python_lib" -type f -name '*.a' -delete
  fi
  find "$runtime_lib" -maxdepth 1 -type f -name '*.a' -delete
fi
printf '%s\n' "$commit" > "$transaction/targets/$target/kitty-provider/source-commit.txt"
printf '%s\n' "$python_version" > "$transaction/targets/$target/kitty-provider/python-version.txt"
if find "$transaction/targets/$target/kitty-provider" -type l -print -quit | grep -q .; then
  echo "Kitty SDK output contains a symbolic link" >&2; exit 79
fi
find "$transaction/targets/$target/kitty-provider" -type f -exec chmod a-w {} +
find "$transaction/targets/$target/kitty-provider" -type d -exec chmod 0555 {} +
mkdir -p "$transaction/receipts"
soksak-sdk validate build-receipt-create "$root/build-dependencies.json" --dependency kitty-provider-sdk \
  --target "$target" --output-root "$transaction" --out "$transaction/receipts/$target.json"
soksak-sdk validate build-receipt "$transaction/receipts/$target.json" --dependencies "$root/build-dependencies.json" --output-root "$transaction"
mkdir -p "$build_root/targets" "$build_root/receipts"
previous_target=$transaction/previous-target
previous_receipt=$transaction/previous-receipt.json
if [ -e "$current_target" ]; then
  [ -f "$receipt" ] || { echo "Kitty SDK target has no receipt" >&2; exit 79; }
  mv "$current_target" "$previous_target"
  mv "$receipt" "$previous_receipt"
fi
mv "$transaction/targets/$target" "$build_root/targets/$target"
mv "$transaction/receipts/$target.json" "$receipt"
if ! soksak-sdk validate build-receipt "$receipt" --dependencies "$root/build-dependencies.json" --output-root "$build_root"; then
  remove_tree "$current_target"
  rm -f -- "$receipt"
  if [ -e "$previous_target" ]; then
    mv "$previous_target" "$current_target"
    mv "$previous_receipt" "$receipt"
  fi
  exit 79
fi
remove_tree "$previous_target"
rm -f -- "$previous_receipt"
echo "KITTY_SDK_READY target=$target"
