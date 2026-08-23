# soksak-sidecar-terminal-kitty 0.0.8

Kitty terminal-state provider for `soksak-spec-sidecar-terminal` 0.0.1. Kitty's production Screen
and VT parser are delivered through the pinned provider SDK; Rust consumes only the SDK's opaque C
handle and flat snapshot/cell structs. Recovery ordering, alt-screen preservation and restore
serialization are owned by `soksak-kit-sidecar-terminal` 0.0.2.

Build and declare the SDK explicitly:

```sh
cd /path/to/kitty
CC=/usr/bin/clang CXX=/usr/bin/clang++ python3 setup.py kitty-provider-sdk
SOKSAK_KITTY_PROVIDER_SDK=/path/to/kitty/build/kitty-provider cargo test
```

The development SDK links the exact Python runtime recorded in `python-config.json`. Release
packaging must bundle that runtime and rewrite native install names; a Homebrew path is not an
acceptable release dependency.
