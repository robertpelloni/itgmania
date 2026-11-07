# Changelog

## [Unreleased]
- Added `bash ./fetch-extern-deps.sh` step to GitHub Actions CI workflow to populate external dependencies across all runner platforms.
- Enhanced `rust_sm_crash` handler in `src/rust_core` with structured diagnostics and unit tests.
- Implemented `rust_format_log_location` in Rust core (`src/rust_core`) with comprehensive unit tests and wired into `ShowWarningOrTrace` in `src/RageLog.cpp`.
