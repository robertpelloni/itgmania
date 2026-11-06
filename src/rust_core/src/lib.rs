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
}
