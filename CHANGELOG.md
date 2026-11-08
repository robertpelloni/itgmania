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
- Implemented `rust_begins_with` and `rust_ends_with` in Rust core (`src/rust_core`) with unit tests and wired into `BeginsWith` and `EndsWith` in `src/RageUtil.cpp`.
- Implemented `rust_trim_bounds` in Rust core (`src/rust_core`) with unit tests and wired into `TrimLeft`, `TrimRight`, and `Trim` in `src/RageUtil.cpp`.
- Implemented `rust_binary_to_hex` in Rust core (`src/rust_core`) with unit tests and wired into `BinaryToHex` in `src/RageUtil.cpp`.
- Implemented `rust_is_hex_val` and `rust_hex_to_binary` in Rust core (`src/rust_core`) with unit tests and wired into `IsHexVal` and `HexToBinary` in `src/RageUtil.cpp`.
- Implemented `rust_get_extension` in Rust core (`src/rust_core`) with unit tests and wired into `GetExtension` in `src/RageUtil.cpp`.
- Implemented `rust_dirname` in Rust core (`src/rust_core`) with unit tests and wired into `Dirname` in `src/RageUtil.cpp`.
- Implemented `rust_make_upper_ascii` and `rust_make_lower_ascii` in Rust core (`src/rust_core`) with unit tests and wired into `MakeUpper` and `MakeLower` in `src/RageUtil.cpp`.
- Implemented `rust_basename` in Rust core (`src/rust_core`) with unit tests and wired into `Basename` in `src/RageUtil.cpp`.
- Implemented `rust_splitpath` in Rust core (`src/rust_core`) with unit tests and wired into `splitpath` in `src/RageUtil.cpp`.
- Implemented `rust_is_an_int` and `rust_power_of_two` in Rust core (`src/rust_core`) with unit tests and wired into `IsAnInt` and `power_of_two` in `src/RageUtil.cpp`.
- Implemented `rust_fapproach` and `rust_fmodfp` in Rust core (`src/rust_core`) with unit tests and wired into `fapproach` and `fmodfp` in `src/RageUtil.cpp`.
- Implemented `rust_commify` in Rust core (`src/rust_core`) with unit tests and wired into `Commify` in `src/RageUtil.cpp`.
