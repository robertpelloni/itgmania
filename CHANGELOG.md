# Changelog

## [1.0.1] - 2026-06-22
### Fixed
- Reverted invalid namespace `avcodec::SWS_BICUBIC` to the raw macro `SWS_BICUBIC` to fix hard compilation errors in `MovieTexture_FFMpeg.h`.
- Removed raw `PlayerNumber` mapping to `Player.cpp`, `AttackDisplay.cpp` and `NoteField.cpp` utilizing `m_pPlayerState->m_PlayerNumber` when necessary instead.

## [1.0.0] - Initial state
