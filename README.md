# soksak-sidecar-terminal-kitty 0.0.38

Kitty terminal-state provider for `soksak-spec-sidecar-terminal` 0.0.2. Kitty's production Screen
and VT parser are delivered through the pinned provider SDK; Rust consumes only the SDK's opaque C
handle and flat snapshot/cell structs. Recovery ordering, alt-screen preservation and restore
serialization, input unit normalization, normal scrollback, and base-theme composition are owned by
`soksak-kit-sidecar-terminal` 0.0.34.

`build-dependencies.json` owns the exact Kitty source, Python tool, target set and SDK tree.
The Make command graph consumes that declaration and writes a canonical tree receipt:

```sh
make lock TARGET=aarch64-apple-darwin
make prepare TARGET=aarch64-apple-darwin
make build TARGET=aarch64-apple-darwin
make verify TARGET=aarch64-apple-darwin
make stage TARGET=aarch64-apple-darwin STAGE=dist
make attest TARGET=aarch64-apple-darwin OUT=/absolute/kitty-release
```

`make lock` is the only owner operation that projects changed Cargo declarations into
`Cargo.lock`. Normal build and verification remain `--locked`.

The SDK tree bundles the exact Python runtime recorded in `python-config.json`. `build.rs` accepts
only the target receipt selected by Make and always links the bundled runtime; a workstation
library path is not a release fallback.
