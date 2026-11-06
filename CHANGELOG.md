# Changelog

## [Unreleased]
- Initialized `src/rust_core` staticlib crate integrated into CMake build pipeline.
- Implemented `rust_sm_crash` handler in Rust with UTF-8 safety checks and unit tests.
- Wired `sm_crash` in `src/global.cpp` to `rust_sm_crash`.
