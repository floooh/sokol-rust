// machine generated, do not edit

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::gfx as sg;

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
        pub fn sglue_environment() -> sg::Environment;
        pub fn sglue_swapchain() -> sg::Swapchain;
    }
}
#[inline]
pub fn environment() -> sg::Environment {
    unsafe { ffi::sglue_environment() }
}
#[inline]
pub fn swapchain() -> sg::Swapchain {
    unsafe { ffi::sglue_swapchain() }
}
