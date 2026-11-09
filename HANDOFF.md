# Handoff
Session goals:
1. Bobcoin / Lattice Arcade: Consolidate unified dashboard UI, system tray and server process control manager, resolve build conflicts and bump version to v8.115.0.
2. ITGmania: Continued autonomous development and core Rust crash handling integration (`src/rust_core`).

Current status:
- `bobcoin` subproject updated to v8.115.0 with full test passes and unified command center.
- `itgmania` subproject has Rust core integration in `src/rust_core` with `rust_sm_crash`, `rust_format_log_location`, `rust_timer_difference`, `rust_timer_sum`, `rust_clamp_*`, `rust_url_encode`, `rust_strip_crnl`, `rust_crc32`, `rust_begins_with`, `rust_ends_with`, `rust_trim_bounds`, `rust_binary_to_hex`, `rust_is_hex_val`, `rust_hex_to_binary`, `rust_get_extension`, `rust_dirname`, `rust_make_upper_ascii`, `rust_make_lower_ascii`, `rust_basename`, `rust_splitpath`, `rust_is_an_int`, `rust_power_of_two`, `rust_fapproach`, `rust_fmodfp`, `rust_commify`, `rust_pretty_percent`, `rust_seconds_to_hhmmss`, `rust_seconds_to_mmss`, `rust_seconds_to_mss`, `rust_seconds_to_mmss_msms`, `rust_seconds_to_mss_msms`, `rust_seconds_to_mmss_msmsms`, and `rust_microseconds_to_mmss_msms` in `src/RageLog.cpp`, `src/RageTimer.cpp`, `src/RageUtil.cpp`, and `src/global.h`.
- External CMake scripts (`extern/CMakeLists.txt`, `extern/CMakeProject-*.cmake`) and CI workflow `fetch-extern-deps.sh` configured.
