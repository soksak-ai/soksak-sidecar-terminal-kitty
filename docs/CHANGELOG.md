# Change log

This file records completed changes. Current rules are defined by the other documents in this
directory and the selected contracts.

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
