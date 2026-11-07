use std::ffi::CStr;
use std::os::raw::c_char;

const ONE_SECOND_IN_MICROSECONDS: u64 = 1_000_000;
const ONE_SECOND_IN_MICROSECONDS_F64: f64 = 1_000_000.0;

/// Safe handler invoked on fatal crashes or assertion failures in ITGmania.
/// Logs crash diagnostics with timestamp and process info before aborting.
#[no_mangle]
pub extern "C" fn rust_sm_crash(reason: *const c_char) {
    let reason_str = if reason.is_null() {
        "Unknown crash reason"
    } else {
        unsafe { CStr::from_ptr(reason).to_str().unwrap_or("Invalid UTF-8 in crash reason") }
    };

    eprintln!("========================================================");
    eprintln!("ITGMANIA RUST CRASH HANDLER: Fatal Error Caught");
    eprintln!("Reason: {}", reason_str);
    eprintln!("========================================================");
}

/// Formats a source file location string (`"<file>:<line>"`) for warning and trace logging,
/// stripping any leading directory path up to and including the first occurrence of `"src/"`.
///
/// Returns the number of characters written (excluding the trailing null byte),
/// or -1 if the destination buffer is null or insufficient.
#[no_mangle]
pub extern "C" fn rust_format_log_location(
    file: *const c_char,
    line: i32,
    out_buf: *mut c_char,
    out_buf_len: usize,
) -> i32 {
    if out_buf.is_null() || out_buf_len == 0 {
        return -1;
    }

    let raw_file = if file.is_null() {
        "unknown"
    } else {
        unsafe { CStr::from_ptr(file).to_str().unwrap_or("unknown") }
    };

    // Strip leading path components up to and including the first "src/" or "src\"
    let stripped = if let Some(idx) = raw_file.find("src/") {
        &raw_file[idx + 4..]
    } else if let Some(idx) = raw_file.find("src\\") {
        &raw_file[idx + 4..]
    } else {
        raw_file
    };

    let formatted = format!("{}:{}", stripped, line);
    let bytes = formatted.as_bytes();

    if bytes.len() + 1 > out_buf_len {
        return -1;
    }

    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf as *mut u8, bytes.len());
        *out_buf.add(bytes.len()) = 0;
    }

    bytes.len() as i32
}

/// Calculates the time difference between two timestamps (lhs - rhs) in seconds as a double,
/// preserving sub-microsecond fractional precision.
#[no_mangle]
pub extern "C" fn rust_timer_difference(
    lhs_secs: u64,
    lhs_us: u64,
    rhs_secs: u64,
    rhs_us: u64,
) -> f64 {
    let mut secs = lhs_secs as i64 - rhs_secs as i64;
    let mut us = lhs_us as i64 - rhs_us as i64;

    if us < 0 {
        us += ONE_SECOND_IN_MICROSECONDS as i64;
        secs -= 1;
    }

    (secs as f64) + (us as f64 / ONE_SECOND_IN_MICROSECONDS_F64)
}

/// Adds a floating-point duration `tm` (in seconds) to a timestamp (lhs_secs, lhs_us),
/// writing the resulting seconds and microseconds into `out_secs` and `out_us`.
#[no_mangle]
pub extern "C" fn rust_timer_sum(
    lhs_secs: u64,
    lhs_us: u64,
    tm: f32,
    out_secs: *mut u64,
    out_us: *mut u64,
) {
    if out_secs.is_null() || out_us.is_null() {
        return;
    }

    let seconds = tm.floor() as i64;
    let us = ((tm - tm.floor()) * ONE_SECOND_IN_MICROSECONDS_F64 as f32) as i64;

    let mut res_secs = (lhs_secs as i64) + seconds;
    let mut res_us = (lhs_us as i64) + us;

    if res_us >= ONE_SECOND_IN_MICROSECONDS as i64 {
        res_us -= ONE_SECOND_IN_MICROSECONDS as i64;
        res_secs += 1;
    } else if res_us < 0 {
        res_us += ONE_SECOND_IN_MICROSECONDS as i64;
        res_secs -= 1;
    }

    unsafe {
        *out_secs = if res_secs < 0 { 0 } else { res_secs as u64 };
        *out_us = if res_us < 0 { 0 } else { res_us as u64 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_rust_sm_crash_null() {
        rust_sm_crash(std::ptr::null());
    }

    #[test]
    fn test_rust_sm_crash_valid() {
        let msg = CString::new("Test crash message").unwrap();
        rust_sm_crash(msg.as_ptr());
    }

    #[test]
    fn test_rust_format_log_location_with_src() {
        let file = CString::new("/home/user/itgmania/src/RageLog.cpp").unwrap();
        let mut buf = [0 as c_char; 128];
        let len = rust_format_log_location(file.as_ptr(), 42, buf.as_mut_ptr(), buf.len());
        assert_eq!(len, "RageLog.cpp:42".len() as i32);
        let out_str = unsafe { CStr::from_ptr(buf.as_ptr()).to_str().unwrap() };
        assert_eq!(out_str, "RageLog.cpp:42");
    }

    #[test]
    fn test_rust_format_log_location_without_src() {
        let file = CString::new("RageLog.cpp").unwrap();
        let mut buf = [0 as c_char; 128];
        let len = rust_format_log_location(file.as_ptr(), 100, buf.as_mut_ptr(), buf.len());
        assert_eq!(len, "RageLog.cpp:100".len() as i32);
        let out_str = unsafe { CStr::from_ptr(buf.as_ptr()).to_str().unwrap() };
        assert_eq!(out_str, "RageLog.cpp:100");
    }

    #[test]
    fn test_rust_format_log_location_null_file() {
        let mut buf = [0 as c_char; 128];
        let len = rust_format_log_location(std::ptr::null(), 7, buf.as_mut_ptr(), buf.len());
        assert_eq!(len, "unknown:7".len() as i32);
        let out_str = unsafe { CStr::from_ptr(buf.as_ptr()).to_str().unwrap() };
        assert_eq!(out_str, "unknown:7");
    }

    #[test]
    fn test_rust_format_log_location_buffer_too_small() {
        let file = CString::new("src/main.cpp").unwrap();
        let mut buf = [0 as c_char; 4];
        let len = rust_format_log_location(file.as_ptr(), 1, buf.as_mut_ptr(), buf.len());
        assert_eq!(len, -1);
    }

    #[test]
    fn test_rust_timer_difference() {
        // Simple 1.5 second difference
        let diff = rust_timer_difference(10, 500_000, 9, 0);
        assert!((diff - 1.5).abs() < 1e-6);

        // Difference with microsecond borrow
        let diff_borrow = rust_timer_difference(10, 100_000, 9, 500_000);
        assert!((diff_borrow - 0.6).abs() < 1e-6);

        // Reverse difference (negative)
        let diff_neg = rust_timer_difference(9, 500_000, 10, 100_000);
        assert!((diff_neg - (-0.6)).abs() < 1e-6);
    }

    #[test]
    fn test_rust_timer_sum() {
        let mut out_secs = 0u64;
        let mut out_us = 0u64;

        // Add 2.25s to (5, 500_000) -> (7, 750_000)
        rust_timer_sum(5, 500_000, 2.25, &mut out_secs, &mut out_us);
        assert_eq!(out_secs, 7);
        assert_eq!(out_us, 750_000);

        // Add duration with microsecond overflow
        rust_timer_sum(5, 800_000, 0.5, &mut out_secs, &mut out_us);
        assert_eq!(out_secs, 6);
        assert_eq!(out_us, 300_000);

        // Subtract duration
        rust_timer_sum(5, 200_000, -0.5, &mut out_secs, &mut out_us);
        assert_eq!(out_secs, 4);
        assert_eq!(out_us, 700_000);

        // Null pointer safety
        rust_timer_sum(5, 200_000, 1.0, std::ptr::null_mut(), std::ptr::null_mut());
    }
}
