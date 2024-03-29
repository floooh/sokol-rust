// machine generated, do not edit

#![allow(dead_code)]
#![allow(unused_imports)]

/// Helper function to convert a C string to a Rust string slice
#[inline]
fn c_char_ptr_to_rust_str(c_char_ptr: *const core::ffi::c_char) -> &'static str {
    let c_str = unsafe { core::ffi::CStr::from_ptr(c_char_ptr) };
    c_str.to_str().expect("c_char_ptr contained invalid Utf8 Data")
}

pub mod ffi {
    #![allow(unused_imports)]
    use super::*;
    extern "C" {
        pub fn slog_func(
            tag: *const core::ffi::c_char,
            log_level: u32,
            log_item: u32,
            message: *const core::ffi::c_char,
            line_nr: u32,
            filename: *const core::ffi::c_char,
            user_data: *mut core::ffi::c_void,
        );
    }
}
#[inline]
pub extern "C" fn slog_func(
    tag: *const core::ffi::c_char,
    log_level: u32,
    log_item: u32,
    message: *const core::ffi::c_char,
    line_nr: u32,
    filename: *const core::ffi::c_char,
    user_data: *mut core::ffi::c_void,
) {
    unsafe { ffi::slog_func(tag, log_level, log_item, message, line_nr, filename, user_data) }
}
