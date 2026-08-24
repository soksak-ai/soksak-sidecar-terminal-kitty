#!/bin/sh
set -eu

[ "$#" -eq 1 ] && [ -n "$1" ] || { echo 'usage: check-build-environment.sh <target>' >&2; exit 78; }
target=$1
resolution=$(soksak-validate build-dependencies build-dependencies.json --dependency kitty-provider-sdk --target "$target") || exit 78
python_expected=$(printf '%s' "$resolution" | node -e 'let s="";process.stdin.on("data",c=>s+=c).on("end",()=>process.stdout.write(JSON.parse(s).tools.python))')
case "$(uname -s)-$(uname -m)" in
  Darwin-arm64) host_target=aarch64-apple-darwin; node_platform=darwin; node_arch=arm64; python_machine=arm64 ;;
  Darwin-x86_64)
    if [ "$(sysctl -n hw.optional.arm64 2>/dev/null || true)" = 1 ]; then
      host_target=aarch64-apple-darwin; node_platform=darwin; node_arch=arm64; python_machine=arm64
    else
      host_target=x86_64-apple-darwin; node_platform=darwin; node_arch=x64; python_machine=x86_64
    fi
    ;;
  Linux-aarch64|Linux-arm64) host_target=aarch64-unknown-linux-gnu; node_platform=linux; node_arch=arm64; python_machine=aarch64 ;;
  Linux-x86_64) host_target=x86_64-unknown-linux-gnu; node_platform=linux; node_arch=x64; python_machine=x86_64 ;;
  *) echo "TOOLCHAIN_MISMATCH: unsupported Kitty host $(uname -s)-$(uname -m)" >&2; exit 78 ;;
esac
python_actual=$(python3 -c 'import platform,sys;print(sys.version.split()[0]);print(platform.machine())' 2>/dev/null || true)
python_version=$(printf '%s\n' "$python_actual" | sed -n '1p')
python_arch=$(printf '%s\n' "$python_actual" | sed -n '2p')
rust_expected=$(sed -n 's/^channel = "\([^"]*\)"$/\1/p' rust-toolchain.toml)
rust_actual=$(rustc --version 2>/dev/null | awk '{print $2}' || true)
rust_host=$(rustc -vV 2>/dev/null | sed -n 's/^host: //p' || true)
node_actual_platform=$(node -p process.platform 2>/dev/null || true)
node_actual_arch=$(node -p process.arch 2>/dev/null || true)
if [ "$target" != "$host_target" ] || [ "$python_version" != "$python_expected" ] || [ "$python_arch" != "$python_machine" ] || \
   [ -z "$rust_expected" ] || [ "$rust_actual" != "$rust_expected" ] || [ "$rust_host" != "$target" ] || \
   [ "$node_actual_platform" != "$node_platform" ] || [ "$node_actual_arch" != "$node_arch" ]; then
  printf 'TOOLCHAIN_MISMATCH: target=%s hostTarget=%s python=%s/%s rust=%s/%s nodeRuntime=%s/%s; expected python=%s/%s rust=%s/%s nodeRuntime=%s/%s\n' \
    "$target" "$host_target" "${python_version:-missing}" "${python_arch:-unknown}" "${rust_actual:-missing}" "${rust_host:-unknown}" \
    "${node_actual_platform:-unknown}" "${node_actual_arch:-unknown}" "$python_expected" "$python_machine" "$rust_expected" "$target" "$node_platform" "$node_arch" >&2
  exit 78
fi
printf 'BUILD_ENVIRONMENT_READY target=%s python=%s/%s rust=%s/%s nodeRuntime=%s/%s\n' \
  "$target" "$python_version" "$python_arch" "$rust_actual" "$rust_host" "$node_actual_platform" "$node_actual_arch"
