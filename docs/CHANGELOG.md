# Change log

This file records completed changes. Current rules are defined by the other documents in this
directory and the selected contracts.

## 2026-08-31

- Release 0.0.37 assigns the rewritten source commit a new immutable release identity; 0.0.36
  remains bound to its original source commit and bytes.

## 2026-08-30

- Release 0.0.36 selects the final terminal Kit 0.0.34 revision.
- DEC modes 9 and 1001 remain false because Kitty does not retain either mode; no supported
  tracking state is used as an alias or fallback.
- Wheel and pointer admission now follows the public `TerminalModes` rules, with owner tests for
  unsupported legacy modes and distinct 1000, 1002, and 1003 engine facts.
- Native wheel mouse reports now use Kitty's live provider encoder for X10, UTF-8, SGR, and URXVT
  protocols on both axes with exact repetition and modifiers.
- Alternate-screen DEC mode 1007 wheel input emits application-cursor arrows on both axes.
- Both PTY routes reject stale mode decisions; device-unit normalization and normal scrollback stay
  in the common terminal Kit.

## 2026-08-29

- Pointer events now use the forked Kitty provider's live mouse encoder.
- Selection and wheel remain explicit open owner rows.
- Version 0.0.30 was an unpublished intermediate stage that aligned the pointer-aware Kit and Kitty
  provider SDK. The staging owner retained those bytes and required the completed implementation to
  advance to 0.0.31; no staged directory was deleted or overwritten.

## 2026-08-28

- Terminal theme overrides now come from the versioned Kitty provider color-state ABI.
- OSC color reset clears explicit override presence without changing the host-owned base theme.
- Cursor shape and blink state now come from the versioned Kitty provider snapshot.
- The renderer receives Kitty's 500 ms cursor animation policy.
- Changed native SDK declarations now use validated atomic replacement instead of cache deletion.
- Contract cursor acceptance and the arm64 owner gate passed.
