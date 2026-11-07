# TODO
- [x] Configure CI workflow to run `fetch-extern-deps.sh`.
- [x] Enhance Rust core crash handler `rust_sm_crash` and add unit tests.
- [x] Port location formatting in `ShowWarningOrTrace` (`rust_format_log_location`) to Rust core.
- [x] Port timer delta and arithmetic calculations in `RageTimer` (`rust_timer_difference`, `rust_timer_sum`) to Rust core.
- [x] Port numeric clamping utilities (`rust_clamp_f32`, `rust_clamp_f64`, `rust_clamp_i32`) to Rust core and wire into `rage_clamp` in `src/RageUtil.h`.
- [ ] Port additional utility and logging functions to Rust.
