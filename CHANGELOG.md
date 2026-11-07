# Changelog

## [Unreleased]
- Added `bash ./fetch-extern-deps.sh` step to GitHub Actions CI workflow to populate external dependencies across all runner platforms.
- Enhanced `rust_sm_crash` handler in `src/rust_core` with structured diagnostics and unit tests.
- Implemented `rust_format_log_location` in Rust core (`src/rust_core`) with comprehensive unit tests and wired into `ShowWarningOrTrace` in `src/RageLog.cpp`.
- Implemented `rust_timer_difference` and `rust_timer_sum` in Rust core (`src/rust_core`) with comprehensive unit tests and wired into `RageTimer::Difference` and `RageTimer::Sum` in `src/RageTimer.cpp`.
- Implemented `rust_clamp_f32`, `rust_clamp_f64`, and `rust_clamp_i32` in Rust core (`src/rust_core`) with unit tests, preserving C++ inline templates for performance.
- Implemented `rust_url_encode` in Rust core (`src/rust_core`) with unit tests and wired into `URLEncode` in `src/RageUtil.cpp`.
- Implemented `rust_strip_crnl` in Rust core (`src/rust_core`) with unit tests and wired into `StripCrnl` in `src/RageUtil.cpp`.
- Implemented `rust_crc32` in Rust core (`src/rust_core`) with precomputed lookup table and unit tests, and wired into `CRC32` in `src/RageUtil.cpp`.
