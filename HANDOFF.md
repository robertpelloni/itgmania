# Handoff
Session goals:
1. Bobcoin / Lattice Arcade: Consolidate unified dashboard UI, system tray and server process control manager, resolve build conflicts and bump version to v8.115.0.
2. ITGmania: Continued autonomous development and core Rust crash handling integration (`src/rust_core`).

Current status:
- `bobcoin` subproject updated to v8.115.0 with full test passes and unified command center.
- `itgmania` subproject has Rust core integration in `src/rust_core` with `rust_sm_crash`, `rust_format_log_location`, `rust_timer_difference`, `rust_timer_sum`, `rust_clamp_*`, `rust_url_encode`, `rust_strip_crnl`, `rust_crc32`, `rust_begins_with`, and `rust_ends_with` in `src/RageLog.cpp`, `src/RageTimer.cpp`, `src/RageUtil.cpp`, and `src/global.h`.
- External CMake scripts (`extern/CMakeLists.txt`, `extern/CMakeProject-*.cmake`) and CI workflow `fetch-extern-deps.sh` configured.
