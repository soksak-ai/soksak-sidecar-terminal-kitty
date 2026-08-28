# Terminal presentation

Kitty owns parsed cursor state. The versioned provider fork exports `Screen.cursor.shape` and
`Screen.cursor.non_blinking` in `KittyProviderSnapshot`; the sidecar does not parse CSI again.

The provider maps Kitty block, beam, underline, and hollow variants to the public block, bar, and
underline terminal shapes. DECTCEM visibility remains separate in `TerminalModes.show_cursor`.
Blink scheduling uses Kitty's 500 ms default renderer interval.

Kitty's provider feed applies OSC 4/10/11/12 and reset sequences to Kitty's own `ColorProfile`.
The provider ABI reports explicit foreground, background, cursor and indexed-palette presence and
RGB values. This sidecar maps that state to `TerminalThemeOverrides`; it does not parse terminal
input again and does not infer presence by comparing an effective color with the host base theme.

`tests/conformance.rs::cursor_style` runs the contract-owned DECSCUSR, DEC mode 12, DECTCEM, and
warm rehydrate cases. `make verify TARGET=aarch64-apple-darwin` verifies the exact declared provider
SDK and this sidecar.
