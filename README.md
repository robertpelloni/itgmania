# ITGmania (robertpelloni fork)

ITGmania is a fork of [StepMania 5.1](https://github.com/stepmania/stepmania/tree/5_1-new),
an advanced cross-platform rhythm game for home and arcade use. This
repository is Robert Pelloni's fork of upstream
[itgmania/itgmania](https://github.com/itgmania/itgmania); the game
itself is the ITGmania team's work (GPLv3), and most of this README
describes their project. The changes in this fork are listed below.

## Changes in this fork

- **Rust core** (`src/rust_core/src/lib.rs` — 54 KB, 1,735 lines, 37
  exported functions) — a Rust integration in progress, wired into
  `src/RageLog.cpp`, `src/RageTimer.cpp`, `src/RageUtil.cpp`,
  `src/RageUtil.h`, and `src/global.h`:
  - Crash handling (`rust_sm_crash`) and log-location formatting
    (`rust_format_log_location`)
  - Timer arithmetic (`rust_timer_difference`, `rust_timer_sum`),
    numeric clamping (`rust_clamp_f32`/`f64`/`i32`), and math helpers
    (`rust_fapproach`, `rust_fmodfp`, `rust_power_of_two`)
  - String utilities ported from `RageUtil`: `rust_url_encode`,
    `rust_strip_crnl`, `rust_crc32`, `rust_begins_with`,
    `rust_ends_with`, `rust_trim_bounds`, `rust_binary_to_hex`,
    `rust_is_hex_val`, `rust_hex_to_binary`, `rust_get_extension`,
    `rust_dirname`, `rust_basename`, `rust_splitpath`,
    `rust_make_upper_ascii`, `rust_make_lower_ascii`, `rust_is_an_int`,
    `rust_utf8_remove_bom`
  - Number/time formatting: `rust_commify`, `rust_pretty_percent`,
    and the `rust_seconds_to_*` / `rust_microseconds_to_*` formatters

  Early-stage: utility functions only, not a rewritten engine.
- **FFmpeg movie texture fix**
  (`src/arch/MovieTexture/MovieTexture_FFMpeg.cpp`) — context
  lifecycle / SWS_BICUBIC fixes. Landed via the `release` branch
  (Protocol #284); the original Jules PR #2 was closed as superseded
  once `main` was synced to `release`.
- **Lua daemon scripts** (`Scripts/00_DaemonInit.lua`,
  `Scripts/DaemonWorkflow.lua`).
- **`bobcoin/` submodule** — a related Lattice Arcade / bobcoin
  subproject (per `HANDOFF.md`, v8.115.0 with a unified dashboard).
- External CMake configuration restored and `fetch-extern-deps.sh`
  wired for CI.

No releases are published from this fork; for a working game, use
upstream ITGmania releases at https://www.itgmania.com/.

## Changes to StepMania 5.1 (upstream ITGmania)

- Built-in network functionality
- Fully 64-bit, optimized for modern OSes
- Reload new songs from within the song select screen
- The mine fix applied (courtesy of
  [DinsFire64](https://gist.github.com/DinsFire64/4a3f763cd3033afd55a176980b32a3b5))
- Held misses tracked in the engine for pad debugging
- Fixed overlapping hold bug
- Per-player visual delay
- Per-player disabling of timing windows
- New preference to control note render ordering
- Increased the Stats.xml file size limit to 100MB
- Changed the default binding for P2/back from hyphen to backslash

## Installation

Use upstream ITGmania releases (installer recommended). Windows 7+
minimum; macOS 11+ (run `xattr -c /Applications/ITGmania.app` after
installing; user data lives in `~/Library/Application Support/ITGmania`).
Linux users install the documented runtime libraries from their package
manager (see upstream docs for the per-distro lists).

### Build from source

ITGmania compiles with [CMake](http://www.cmake.org/). See the `Build`
directory and CMake's documentation. This fork adds the Rust core under
`src/rust_core` (Rust toolchain required for that part).

## Resources

- [ITGmania Website](https://www.itgmania.com/)
- [StepMania 5.1 to ITGmania Migration Guide](Docs/Userdocs/sm5_migration.md)
- [Lua for ITGmania](https://quietly-turning.github.io/Lua-For-SM5/LuaAPI?engine=ITGmania)
- Lua API documentation in the `Docs` folder.

## Status

Fork under active experiment. On 2026-11-19, `main` was synced to
the `release` branch (PR #4): before that sync `main` lagged
`release` by 60 commits and showed only the first 7 Rust functions.
The content of `main` is now the 2026-11-15 `release` head
(851f41ee, Protocol #303 bobcoin submodule sync) plus that sync
merge. The upstream game is mature; the fork's own additions are
experimental and unreleased.

## Licensing

ITGmania and the Simply Love theme are GPLv3 (or any later version).
Songs in the 'StepMania 5' folder are CC-NC; 'Club Fantastic' songs
remain copyright of their authors (free for personal/public use,
including broadcasting and tournaments). See `Docs/Licenses.txt` for
library licenses.

## Credits (upstream)

ITGmania team: Martin Natano (natano), teejusb. Contributors include
Club Fantastic, DinsFire64 (mine fix), EvocaitArt, jenx, LightningXCE,
MegaSphere, and the StepMania 5 / old StepMania teams (see
`Docs/credits_SM5.txt`).
