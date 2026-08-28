# Terminal presentation

Kitty owns parsed cursor state. The versioned provider fork exports `Screen.cursor.shape` and
`Screen.cursor.non_blinking` in `KittyProviderSnapshot`; the sidecar does not parse CSI again.

The provider maps Kitty block, beam, underline, and hollow variants to the public block, bar, and
underline terminal shapes. DECTCEM visibility remains separate in `TerminalModes.show_cursor`.
Blink scheduling uses Kitty's 500 ms default renderer interval.

`tests/conformance.rs::cursor_style` runs the contract-owned DECSCUSR, DEC mode 12, DECTCEM, and
warm rehydrate cases. `make verify TARGET=aarch64-apple-darwin` verifies the exact declared provider
SDK and this sidecar.
