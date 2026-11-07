# Handoff
Session goals:
1. Bobcoin / Lattice Arcade: Consolidate unified dashboard UI, system tray and server process control manager, resolve build conflicts and bump version to v8.115.0.
2. ITGmania: Continued autonomous development and core Rust crash handling integration (`src/rust_core`).

Current status:
- `bobcoin` subproject updated to v8.115.0 with full test passes and unified command center.
- `itgmania` subproject has Rust core integration in `src/rust_core` with `rust_sm_crash`, `rust_format_log_location`, `rust_timer_difference`, `rust_timer_sum`, and `rust_clamp_*` in `src/RageLog.cpp`, `src/RageTimer.cpp`, and `src/global.h` (preserving inline C++ templates in `RageUtil.h` for hot-path speed).
- External CMake scripts (`extern/CMakeLists.txt`, `extern/CMakeProject-*.cmake`) and CI workflow `fetch-extern-deps.sh` configured.
