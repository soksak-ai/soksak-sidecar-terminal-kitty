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

Pointer reporting uses the Kitty provider fork's `kitty_provider_pointer` ABI. That ABI invokes
Kitty's live `Screen` mouse encoder, so tracking mode, protocol, button motion, release form, and
modifiers remain engine-owned. The Sidecar passes normalized cell/button/action/modifier facts and
does not copy X10, UTF-8, SGR, or motion encoding. `tests/pointer_input.rs` pins SGR press, held
motion, release, and no-button any-motion. Kitty retains DEC modes 1000, 1002, and 1003 as distinct
Screen facts. It does not retain DEC modes 9 or 1001, so `mouse_x10` and `mouse_highlight` remain
explicitly false rather than being inferred from another tracking mode. Pointer admission follows
the common `TerminalModes::reports_pointer` rule. Selection uses Kitty's live `Screen` selection
methods.

Wheel device-unit accumulation and normal-screen scrollback remain common Kit state. Only the two
PTY routes cross the engine boundary. Mouse-report wheel input passes Kitty button codes 4-7 to the
same live provider encoder, preserving X10, UTF-8, SGR, URXVT, modifier, coordinate, axis, and
repetition rules. Alternate scroll is accepted only while both the alternate screen and DEC mode
1007 are live, and emits application-cursor arrows on both axes. Both routes recheck current engine
modes and reject stale Kit decisions with `WHEEL_MODE_CHANGED`. `tests/wheel_input.rs` pins these
rules, including the boundary that keeps ordinary scrollback Kit-owned. Mouse-route admission uses
the common `TerminalModes::mouse_reporting` rule.
