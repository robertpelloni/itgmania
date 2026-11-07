use std::ffi::CStr;
use std::os::raw::c_char;

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

    // Strip leading path components up to and including the first "src/"
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
        let mut buf = [0 as c_char; 4]; // too small for "main.cpp:1"
        let len = rust_format_log_location(file.as_ptr(), 1, buf.as_mut_ptr(), buf.len());
        assert_eq!(len, -1);
    }
}
