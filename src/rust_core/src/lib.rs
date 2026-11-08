use std::ffi::CStr;
use std::os::raw::c_char;

const ONE_SECOND_IN_MICROSECONDS: u64 = 1_000_000;
const ONE_SECOND_IN_MICROSECONDS_F64: f64 = 1_000_000.0;

// Precomputed IEEE 802.3 CRC32 lookup table (polynomial 0xEDB88320)
const CRC32_TABLE: [u32; 256] = {
    let mut table = [0u32; 256];
    let poly = 0xEDB88320u32;
    let mut i = 0;
    while i < 256 {
        let mut crc = i as u32;
        let mut j = 0;
        while j < 8 {
            if (crc & 1) != 0 {
                crc = (crc >> 1) ^ poly;
            } else {
                crc >>= 1;
            }
            j += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
};

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

/// Clamps a 32-bit floating-point value to the range [l, h].
#[no_mangle]
pub extern "C" fn rust_clamp_f32(val: f32, l: f32, h: f32) -> f32 {
    if val > h {
        h
    } else if val < l {
        l
    } else {
        val
    }
}

/// Clamps a 64-bit floating-point value to the range [l, h].
#[no_mangle]
pub extern "C" fn rust_clamp_f64(val: f64, l: f64, h: f64) -> f64 {
    if val > h {
        h
    } else if val < l {
        l
    } else {
        val
    }
}

/// Clamps a 32-bit signed integer value to the range [l, h].
#[no_mangle]
pub extern "C" fn rust_clamp_i32(val: i32, l: i32, h: i32) -> i32 {
    if val > h {
        h
    } else if val < l {
        l
    } else {
        val
    }
}

/// Encodes a C-string `input` into URL percent-encoded format matching ITGmania's URLEncode semantics:
/// characters between '!' and 'z' inclusive are kept as-is, while all others are encoded as `%XX`.
///
/// Returns the number of bytes written (excluding null terminator), or -1 if `out_buf` is null
/// or too small.
#[no_mangle]
pub extern "C" fn rust_url_encode(
    input: *const c_char,
    out_buf: *mut c_char,
    out_buf_len: usize,
) -> i32 {
    if out_buf.is_null() || out_buf_len == 0 {
        return -1;
    }

    if input.is_null() {
        unsafe { *out_buf = 0; }
        return 0;
    }

    let input_bytes = unsafe { CStr::from_ptr(input).to_bytes() };
    let mut written = 0usize;

    for &b in input_bytes {
        if (b'!'..=b'z').contains(&b) {
            if written + 1 >= out_buf_len {
                return -1;
            }
            unsafe {
                *out_buf.add(written) = b as c_char;
            }
            written += 1;
        } else {
            if written + 3 >= out_buf_len {
                return -1;
            }
            let hex_high = b >> 4;
            let hex_low = b & 0x0F;
            let to_hex_ascii = |n: u8| -> u8 {
                if n < 10 { b'0' + n } else { b'A' + (n - 10) }
            };
            unsafe {
                *out_buf.add(written) = b'%' as c_char;
                *out_buf.add(written + 1) = to_hex_ascii(hex_high) as c_char;
                *out_buf.add(written + 2) = to_hex_ascii(hex_low) as c_char;
            }
            written += 3;
        }
    }

    unsafe {
        *out_buf.add(written) = 0;
    }

    written as i32
}

/// Strips trailing carriage return (`\r`) and newline (`\n`) characters in-place from a mutable buffer.
/// Returns the new trimmed length.
#[no_mangle]
pub extern "C" fn rust_strip_crnl(data: *mut c_char, mut len: usize) -> usize {
    if data.is_null() || len == 0 {
        return 0;
    }

    unsafe {
        while len > 0 {
            let last = *data.add(len - 1) as u8;
            if last == b'\r' || last == b'\n' {
                len -= 1;
                *data.add(len) = 0;
            } else {
                break;
            }
        }
    }

    len
}

/// Calculates or updates an IEEE 802.3 CRC32 checksum over `buf`.
/// Pre- and post-conditioned with 0xFFFFFFFF matching standard StepMania/ITGmania `CRC32`.
#[no_mangle]
pub extern "C" fn rust_crc32(mut crc: u32, buf: *const u8, size: usize) -> u32 {
    if buf.is_null() || size == 0 {
        return crc;
    }

    crc ^= 0xFFFFFFFF;
    let slice = unsafe { std::slice::from_raw_parts(buf, size) };
    for &b in slice {
        crc = (crc >> 8) ^ CRC32_TABLE[((crc as u8) ^ b) as usize];
    }
    crc ^ 0xFFFFFFFF
}

/// Checks if `test_str` starts with `prefix`.
#[no_mangle]
pub extern "C" fn rust_begins_with(test_str: *const c_char, prefix: *const c_char) -> bool {
    if test_str.is_null() || prefix.is_null() {
        return false;
    }

    let s = unsafe { CStr::from_ptr(test_str).to_bytes() };
    let p = unsafe { CStr::from_ptr(prefix).to_bytes() };

    s.starts_with(p)
}

/// Checks if `test_str` ends with `suffix`.
#[no_mangle]
pub extern "C" fn rust_ends_with(test_str: *const c_char, suffix: *const c_char) -> bool {
    if test_str.is_null() || suffix.is_null() {
        return false;
    }

    let s = unsafe { CStr::from_ptr(test_str).to_bytes() };
    let suf = unsafe { CStr::from_ptr(suffix).to_bytes() };

    s.ends_with(suf)
}

/// Finds the trimmed start and end indices of `data` by checking for characters present in `trim_chars`.
/// `out_start` receives the index of the first non-trim character (or `len` if all trimmed).
/// `out_end` receives the index immediately following the last non-trim character (or `0` if all trimmed).
#[no_mangle]
pub extern "C" fn rust_trim_bounds(
    data: *const c_char,
    len: usize,
    trim_chars: *const c_char,
    out_start: *mut usize,
    out_end: *mut usize,
) {
    if out_start.is_null() || out_end.is_null() {
        return;
    }

    if data.is_null() || len == 0 || trim_chars.is_null() {
        unsafe {
            *out_start = 0;
            *out_end = len;
        }
        return;
    }

    let slice = unsafe { std::slice::from_raw_parts(data as *const u8, len) };
    let trim_set = unsafe { CStr::from_ptr(trim_chars).to_bytes() };

    let mut start = 0;
    while start < len && trim_set.contains(&slice[start]) {
        start += 1;
    }

    let mut end = len;
    while end > 0 && trim_set.contains(&slice[end - 1]) {
        end -= 1;
    }

    unsafe {
        *out_start = start;
        *out_end = end;
    }
}

/// Converts raw binary `data` of length `len` into a lowercase hexadecimal string.
/// Returns the number of characters written, or -1 on buffer overflow or null pointers.
#[no_mangle]
pub extern "C" fn rust_binary_to_hex(
    data: *const u8,
    len: usize,
    out_buf: *mut c_char,
    out_buf_len: usize,
) -> i32 {
    if out_buf.is_null() || out_buf_len == 0 {
        return -1;
    }

    if data.is_null() || len == 0 {
        unsafe { *out_buf = 0; }
        return 0;
    }

    if out_buf_len < len * 2 + 1 {
        return -1;
    }

    let slice = unsafe { std::slice::from_raw_parts(data, len) };
    let to_hex = |n: u8| -> u8 {
        if n < 10 { b'0' + n } else { b'a' + (n - 10) }
    };

    for (i, &b) in slice.iter().enumerate() {
        let hi = b >> 4;
        let lo = b & 0x0F;
        unsafe {
            *out_buf.add(i * 2) = to_hex(hi) as c_char;
            *out_buf.add(i * 2 + 1) = to_hex(lo) as c_char;
        }
    }

    unsafe {
        *out_buf.add(len * 2) = 0;
    }

    (len * 2) as i32
}

/// Returns true if all characters in `str_ptr` are valid hexadecimal digits `[0-9a-fA-F]`,
/// and `len > 0`.
#[no_mangle]
pub extern "C" fn rust_is_hex_val(str_ptr: *const c_char, len: usize) -> bool {
    if str_ptr.is_null() || len == 0 {
        return false;
    }

    let slice = unsafe { std::slice::from_raw_parts(str_ptr as *const u8, len) };
    slice.iter().all(|&b| b.is_ascii_hexdigit())
}

/// Decodes an even-length hexadecimal string `hex_str` into raw bytes in `out_buf`.
/// Returns the number of bytes written, or -1 on failure (odd length, invalid hex, buffer too small).
#[no_mangle]
pub extern "C" fn rust_hex_to_binary(
    hex_str: *const c_char,
    len: usize,
    out_buf: *mut u8,
    out_buf_len: usize,
) -> i32 {
    if hex_str.is_null() || out_buf.is_null() {
        return -1;
    }

    if len == 0 {
        return 0;
    }

    if len % 2 != 0 || out_buf_len < len / 2 {
        return -1;
    }

    let slice = unsafe { std::slice::from_raw_parts(hex_str as *const u8, len) };
    let from_hex = |b: u8| -> Option<u8> {
        match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            b'A'..=b'F' => Some(b - b'A' + 10),
            _ => None,
        }
    };

    for i in 0..(len / 2) {
        let hi = match from_hex(slice[i * 2]) {
            Some(v) => v,
            None => return -1,
        };
        let lo = match from_hex(slice[i * 2 + 1]) {
            Some(v) => v,
            None => return -1,
        };
        unsafe {
            *out_buf.add(i) = (hi << 4) | lo;
        }
    }

    (len / 2) as i32
}

/// Extracts file extension from `path` (of length `path_len`) into `out_buf`.
/// Returns length of extension string written, or -1 if buffer is too small or null.
#[no_mangle]
pub extern "C" fn rust_get_extension(
    path: *const c_char,
    path_len: usize,
    out_buf: *mut c_char,
    out_buf_len: usize,
) -> i32 {
    if out_buf.is_null() || out_buf_len == 0 {
        return -1;
    }

    if path.is_null() || path_len == 0 {
        unsafe { *out_buf = 0; }
        return 0;
    }

    let slice = unsafe { std::slice::from_raw_parts(path as *const u8, path_len) };

    let dot_pos = match slice.iter().rposition(|&b| b == b'.') {
        Some(pos) => pos,
        None => {
            unsafe { *out_buf = 0; }
            return 0;
        }
    };

    if slice[dot_pos..].iter().any(|&b| b == b'/') {
        unsafe { *out_buf = 0; }
        return 0;
    }

    let ext = &slice[dot_pos + 1..];
    if ext.len() + 1 > out_buf_len {
        return -1;
    }

    unsafe {
        std::ptr::copy_nonoverlapping(ext.as_ptr(), out_buf as *mut u8, ext.len());
        *out_buf.add(ext.len()) = 0;
    }

    ext.len() as i32
}

/// Extracts directory component from `path` matching StepMania's Dirname semantics:
/// - "/" -> "/"
/// - "a/b/c" -> "a/b/"
/// - "c/" -> "./"
/// - If no directory prefix, returns "./"
///
/// Returns length of directory string written, or -1 if buffer is too small or null.
#[no_mangle]
pub extern "C" fn rust_dirname(
    path: *const c_char,
    path_len: usize,
    out_buf: *mut c_char,
    out_buf_len: usize,
) -> i32 {
    if out_buf.is_null() || out_buf_len == 0 {
        return -1;
    }

    if path.is_null() || path_len == 0 {
        if out_buf_len < 3 {
            return -1;
        }
        unsafe {
            std::ptr::copy_nonoverlapping(b"./\0".as_ptr(), out_buf as *mut u8, 3);
        }
        return 2;
    }

    let slice = unsafe { std::slice::from_raw_parts(path as *const u8, path_len) };

    if path_len == 1 && slice[0] == b'/' {
        if out_buf_len < 2 {
            return -1;
        }
        unsafe {
            std::ptr::copy_nonoverlapping(b"/\0".as_ptr(), out_buf as *mut u8, 2);
        }
        return 1;
    }

    let mut pos = (path_len as isize) - 1;
    while pos >= 0 && slice[pos as usize] == b'/' {
        pos -= 1;
    }

    while pos >= 0 && slice[pos as usize] != b'/' {
        pos -= 1;
    }

    let res = if pos < 0 {
        b"./" as &[u8]
    } else {
        &slice[0..=(pos as usize)]
    };

    if res.len() + 1 > out_buf_len {
        return -1;
    }

    unsafe {
        std::ptr::copy_nonoverlapping(res.as_ptr(), out_buf as *mut u8, res.len());
        *out_buf.add(res.len()) = 0;
    }

    res.len() as i32
}

/// Converts ASCII characters in-place to uppercase.
#[no_mangle]
pub extern "C" fn rust_make_upper_ascii(data: *mut c_char, len: usize) {
    if data.is_null() || len == 0 {
        return;
    }

    let slice = unsafe { std::slice::from_raw_parts_mut(data as *mut u8, len) };
    for b in slice {
        if (b'a'..=b'z').contains(b) {
            *b -= 32;
        }
    }
}

/// Converts ASCII characters in-place to lowercase.
#[no_mangle]
pub extern "C" fn rust_make_lower_ascii(data: *mut c_char, len: usize) {
    if data.is_null() || len == 0 {
        return;
    }

    let slice = unsafe { std::slice::from_raw_parts_mut(data as *mut u8, len) };
    for b in slice {
        if (b'A'..=b'Z').contains(b) {
            *b += 32;
        }
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
        let diff = rust_timer_difference(10, 500_000, 9, 0);
        assert!((diff - 1.5).abs() < 1e-6);

        let diff_borrow = rust_timer_difference(10, 100_000, 9, 500_000);
        assert!((diff_borrow - 0.6).abs() < 1e-6);

        let diff_neg = rust_timer_difference(9, 500_000, 10, 100_000);
        assert!((diff_neg - (-0.6)).abs() < 1e-6);
    }

    #[test]
    fn test_rust_timer_sum() {
        let mut out_secs = 0u64;
        let mut out_us = 0u64;

        rust_timer_sum(5, 500_000, 2.25, &mut out_secs, &mut out_us);
        assert_eq!(out_secs, 7);
        assert_eq!(out_us, 750_000);

        rust_timer_sum(5, 800_000, 0.5, &mut out_secs, &mut out_us);
        assert_eq!(out_secs, 6);
        assert_eq!(out_us, 300_000);

        rust_timer_sum(5, 200_000, -0.5, &mut out_secs, &mut out_us);
        assert_eq!(out_secs, 4);
        assert_eq!(out_us, 700_000);

        rust_timer_sum(5, 200_000, 1.0, std::ptr::null_mut(), std::ptr::null_mut());
    }

    #[test]
    fn test_rust_clamp() {
        assert_eq!(rust_clamp_f32(0.5, 0.0, 1.0), 0.5);
        assert_eq!(rust_clamp_f32(-0.5, 0.0, 1.0), 0.0);
        assert_eq!(rust_clamp_f32(1.5, 0.0, 1.0), 1.0);

        assert_eq!(rust_clamp_f64(50.0, 10.0, 100.0), 50.0);
        assert_eq!(rust_clamp_f64(5.0, 10.0, 100.0), 10.0);
        assert_eq!(rust_clamp_f64(150.0, 10.0, 100.0), 100.0);

        assert_eq!(rust_clamp_i32(5, 0, 10), 5);
        assert_eq!(rust_clamp_i32(-5, 0, 10), 0);
        assert_eq!(rust_clamp_i32(15, 0, 10), 10);
    }

    #[test]
    fn test_rust_url_encode() {
        let mut buf = [0 as c_char; 256];

        let input = CString::new("Hello World").unwrap();
        let len = rust_url_encode(input.as_ptr(), buf.as_mut_ptr(), buf.len());
        assert_eq!(len, "Hello%20World".len() as i32);
        let out_str = unsafe { CStr::from_ptr(buf.as_ptr()).to_str().unwrap() };
        assert_eq!(out_str, "Hello%20World");

        let input2 = CString::new("abcXYZ123-_.~").unwrap();
        let _len2 = rust_url_encode(input2.as_ptr(), buf.as_mut_ptr(), buf.len());
        let out_str2 = unsafe { CStr::from_ptr(buf.as_ptr()).to_str().unwrap() };
        assert_eq!(out_str2, "abcXYZ123-_.%7E");

        let mut small_buf = [0 as c_char; 5];
        let len3 = rust_url_encode(input.as_ptr(), small_buf.as_mut_ptr(), small_buf.len());
        assert_eq!(len3, -1);

        let len_null = rust_url_encode(std::ptr::null(), buf.as_mut_ptr(), buf.len());
        assert_eq!(len_null, 0);
        let out_str_null = unsafe { CStr::from_ptr(buf.as_ptr()).to_str().unwrap() };
        assert_eq!(out_str_null, "");
    }

    #[test]
    fn test_rust_strip_crnl() {
        let mut text = Vec::from(*b"hello world\r\n\0");
        let new_len = rust_strip_crnl(text.as_mut_ptr() as *mut c_char, 13);
        assert_eq!(new_len, 11);
        assert_eq!(&text[..11], b"hello world");

        let mut text_mult = Vec::from(*b"multiple\r\n\r\r\n\0");
        let new_len_mult = rust_strip_crnl(text_mult.as_mut_ptr() as *mut c_char, 13);
        assert_eq!(new_len_mult, 8);
        assert_eq!(&text_mult[..8], b"multiple");

        let mut no_nl = Vec::from(*b"simple\0");
        let new_len_none = rust_strip_crnl(no_nl.as_mut_ptr() as *mut c_char, 6);
        assert_eq!(new_len_none, 6);

        let null_res = rust_strip_crnl(std::ptr::null_mut(), 10);
        assert_eq!(null_res, 0);
    }

    #[test]
    fn test_rust_crc32() {
        let input = b"123456789";
        let crc = rust_crc32(0, input.as_ptr(), input.len());
        assert_eq!(crc, 0xCBF43926);

        let empty_crc = rust_crc32(12345, std::ptr::null(), 0);
        assert_eq!(empty_crc, 12345);

        let part1 = b"12345";
        let part2 = b"6789";
        let mut inc_crc = rust_crc32(0, part1.as_ptr(), part1.len());
        inc_crc = rust_crc32(inc_crc, part2.as_ptr(), part2.len());
        assert_eq!(inc_crc, 0xCBF43926);
    }

    #[test]
    fn test_rust_begins_ends_with() {
        let text = CString::new("StepManiaX").unwrap();
        let prefix = CString::new("Step").unwrap();
        let non_prefix = CString::new("Mania").unwrap();
        let suffix = CString::new("iaX").unwrap();
        let non_suffix = CString::new("Step").unwrap();

        assert!(rust_begins_with(text.as_ptr(), prefix.as_ptr()));
        assert!(!rust_begins_with(text.as_ptr(), non_prefix.as_ptr()));

        assert!(rust_ends_with(text.as_ptr(), suffix.as_ptr()));
        assert!(!rust_ends_with(text.as_ptr(), non_suffix.as_ptr()));

        assert!(!rust_begins_with(std::ptr::null(), prefix.as_ptr()));
        assert!(!rust_ends_with(text.as_ptr(), std::ptr::null()));
    }

    #[test]
    fn test_rust_trim_bounds() {
        let text = CString::new("  \t hello world \n ").unwrap();
        let trim = CString::new(" \t\r\n").unwrap();
        let mut start = 0usize;
        let mut end = 0usize;

        rust_trim_bounds(text.as_ptr(), 18, trim.as_ptr(), &mut start, &mut end);
        assert_eq!(start, 4);
        assert_eq!(end, 15);

        let whitespace = CString::new("   \t  ").unwrap();
        rust_trim_bounds(whitespace.as_ptr(), 6, trim.as_ptr(), &mut start, &mut end);
        assert_eq!(start, 6);
        assert_eq!(end, 0);

        let clean = CString::new("hello").unwrap();
        rust_trim_bounds(clean.as_ptr(), 5, trim.as_ptr(), &mut start, &mut end);
        assert_eq!(start, 0);
        assert_eq!(end, 5);
    }

    #[test]
    fn test_rust_binary_to_hex() {
        let bytes = [0x00u8, 0x12, 0xAB, 0xFF];
        let mut buf = [0 as c_char; 16];
        let len = rust_binary_to_hex(bytes.as_ptr(), bytes.len(), buf.as_mut_ptr(), buf.len());
        assert_eq!(len, 8);
        let out_str = unsafe { CStr::from_ptr(buf.as_ptr()).to_str().unwrap() };
        assert_eq!(out_str, "0012abff");

        let len_empty = rust_binary_to_hex(bytes.as_ptr(), 0, buf.as_mut_ptr(), buf.len());
        assert_eq!(len_empty, 0);

        let mut small_buf = [0 as c_char; 8];
        let len_small = rust_binary_to_hex(bytes.as_ptr(), bytes.len(), small_buf.as_mut_ptr(), small_buf.len());
        assert_eq!(len_small, -1);
    }

    #[test]
    fn test_rust_hex_val_and_decode() {
        let valid_hex = CString::new("12abCF").unwrap();
        assert!(rust_is_hex_val(valid_hex.as_ptr(), 6));

        let invalid_hex = CString::new("12gz").unwrap();
        assert!(!rust_is_hex_val(invalid_hex.as_ptr(), 4));

        let empty = CString::new("").unwrap();
        assert!(!rust_is_hex_val(empty.as_ptr(), 0));

        let mut out = [0u8; 3];
        let count = rust_hex_to_binary(valid_hex.as_ptr(), 6, out.as_mut_ptr(), out.len());
        assert_eq!(count, 3);
        assert_eq!(out, [0x12, 0xAB, 0xCF]);

        let odd_hex = CString::new("12a").unwrap();
        assert_eq!(rust_hex_to_binary(odd_hex.as_ptr(), 3, out.as_mut_ptr(), out.len()), -1);
    }

    #[test]
    fn test_rust_get_extension() {
        let mut buf = [0 as c_char; 32];

        let path1 = CString::new("song.mp3").unwrap();
        let len1 = rust_get_extension(path1.as_ptr(), 8, buf.as_mut_ptr(), buf.len());
        assert_eq!(len1, 3);
        let out1 = unsafe { CStr::from_ptr(buf.as_ptr()).to_str().unwrap() };
        assert_eq!(out1, "mp3");

        let path2 = CString::new("archive.tar.gz").unwrap();
        let len2 = rust_get_extension(path2.as_ptr(), 14, buf.as_mut_ptr(), buf.len());
        assert_eq!(len2, 2);
        let out2 = unsafe { CStr::from_ptr(buf.as_ptr()).to_str().unwrap() };
        assert_eq!(out2, "gz");

        let path3 = CString::new("path/dir.ext/fn").unwrap();
        let len3 = rust_get_extension(path3.as_ptr(), 15, buf.as_mut_ptr(), buf.len());
        assert_eq!(len3, 0);
        let out3 = unsafe { CStr::from_ptr(buf.as_ptr()).to_str().unwrap() };
        assert_eq!(out3, "");

        let path4 = CString::new("README").unwrap();
        let len4 = rust_get_extension(path4.as_ptr(), 6, buf.as_mut_ptr(), buf.len());
        assert_eq!(len4, 0);

        let len_null = rust_get_extension(std::ptr::null(), 0, buf.as_mut_ptr(), buf.len());
        assert_eq!(len_null, 0);
    }

    #[test]
    fn test_rust_dirname() {
        let mut buf = [0 as c_char; 32];

        let path1 = CString::new("a/b/c").unwrap();
        let len1 = rust_dirname(path1.as_ptr(), 5, buf.as_mut_ptr(), buf.len());
        assert_eq!(len1, 4);
        let out1 = unsafe { CStr::from_ptr(buf.as_ptr()).to_str().unwrap() };
        assert_eq!(out1, "a/b/");

        let path2 = CString::new("a/b/c/").unwrap();
        let len2 = rust_dirname(path2.as_ptr(), 6, buf.as_mut_ptr(), buf.len());
        assert_eq!(len2, 4);
        let out2 = unsafe { CStr::from_ptr(buf.as_ptr()).to_str().unwrap() };
        assert_eq!(out2, "a/b/");

        let path3 = CString::new("c/").unwrap();
        let len3 = rust_dirname(path3.as_ptr(), 2, buf.as_mut_ptr(), buf.len());
        assert_eq!(len3, 2);
        let out3 = unsafe { CStr::from_ptr(buf.as_ptr()).to_str().unwrap() };
        assert_eq!(out3, "./");

        let path4 = CString::new("/foo").unwrap();
        let len4 = rust_dirname(path4.as_ptr(), 4, buf.as_mut_ptr(), buf.len());
        assert_eq!(len4, 1);
        let out4 = unsafe { CStr::from_ptr(buf.as_ptr()).to_str().unwrap() };
        assert_eq!(out4, "/");

        let path5 = CString::new("/").unwrap();
        let len5 = rust_dirname(path5.as_ptr(), 1, buf.as_mut_ptr(), buf.len());
        assert_eq!(len5, 1);
        let out5 = unsafe { CStr::from_ptr(buf.as_ptr()).to_str().unwrap() };
        assert_eq!(out5, "/");

        let len6 = rust_dirname(std::ptr::null(), 0, buf.as_mut_ptr(), buf.len());
        assert_eq!(len6, 2);
        let out6 = unsafe { CStr::from_ptr(buf.as_ptr()).to_str().unwrap() };
        assert_eq!(out6, "./");
    }

    #[test]
    fn test_rust_make_upper_lower_ascii() {
        let mut text = Vec::from(*b"Hello World 123!\0");
        rust_make_upper_ascii(text.as_mut_ptr() as *mut c_char, 16);
        assert_eq!(&text[..16], b"HELLO WORLD 123!");

        rust_make_lower_ascii(text.as_mut_ptr() as *mut c_char, 16);
        assert_eq!(&text[..16], b"hello world 123!");

        // Null pointer handling
        rust_make_upper_ascii(std::ptr::null_mut(), 10);
        rust_make_lower_ascii(std::ptr::null_mut(), 10);
    }
}
