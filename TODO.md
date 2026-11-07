# TODO
- [x] Configure CI workflow to run `fetch-extern-deps.sh`.
- [x] Enhance Rust core crash handler `rust_sm_crash` and add unit tests.
- [x] Port location formatting in `ShowWarningOrTrace` (`rust_format_log_location`) to Rust core.
- [x] Port timer delta and arithmetic calculations in `RageTimer` (`rust_timer_difference`, `rust_timer_sum`) to Rust core.
- [x] Implement numeric clamping utilities (`rust_clamp_f32`, `rust_clamp_f64`, `rust_clamp_i32`) in Rust core while preserving C++ inline templates in `RageUtil.h` for hot-path speed.
- [x] Port URL encoding logic (`rust_url_encode`) to Rust core and wire into `URLEncode` in `src/RageUtil.cpp`.
- [ ] Port additional utility and logging functions to Rust.
