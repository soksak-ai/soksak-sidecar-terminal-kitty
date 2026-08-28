# Change log

This file records completed changes. Current rules are defined by the other documents in this
directory and the selected contracts.

## 2026-08-28

- Terminal theme overrides now come from the versioned Kitty provider color-state ABI.
- OSC color reset clears explicit override presence without changing the host-owned base theme.
- Cursor shape and blink state now come from the versioned Kitty provider snapshot.
- The renderer receives Kitty's 500 ms cursor animation policy.
- Changed native SDK declarations now use validated atomic replacement instead of cache deletion.
- Contract cursor acceptance and the arm64 owner gate passed.
