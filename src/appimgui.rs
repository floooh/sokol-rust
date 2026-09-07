//! To use this module, enable the feature "imgui"
// machine generated, do not edit

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::app as sapp;

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
        pub fn sappimgui_setup();
        pub fn sappimgui_shutdown();
        pub fn sappimgui_track_frame();
        pub fn sappimgui_track_event(ev: *const sapp::Event);
        pub fn sappimgui_draw();
        pub fn sappimgui_draw_menu(title: *const core::ffi::c_char);
        pub fn sappimgui_draw_hud_window_content();
        pub fn sappimgui_draw_publicstate_window_content();
        pub fn sappimgui_draw_event_window_content();
        pub fn sappimgui_draw_hud_window(title: *const core::ffi::c_char);
        pub fn sappimgui_draw_publicstate_window(title: *const core::ffi::c_char);
        pub fn sappimgui_draw_event_window(title: *const core::ffi::c_char);
        pub fn sappimgui_draw_hud_menu_item(label: *const core::ffi::c_char);
        pub fn sappimgui_draw_publicstate_menu_item(label: *const core::ffi::c_char);
        pub fn sappimgui_draw_event_menu_item(label: *const core::ffi::c_char);
    }
}
#[inline]
pub fn setup() {
    unsafe { ffi::sappimgui_setup() }
}
#[inline]
pub fn shutdown() {
    unsafe { ffi::sappimgui_shutdown() }
}
#[inline]
pub fn track_frame() {
    unsafe { ffi::sappimgui_track_frame() }
}
#[inline]
pub fn track_event(ev: &sapp::Event) {
    unsafe { ffi::sappimgui_track_event(ev) }
}
#[inline]
pub fn draw() {
    unsafe { ffi::sappimgui_draw() }
}
#[inline]
pub fn draw_menu(title: &str) {
    let tmp_0 = std::ffi::CString::new(title).unwrap();
    unsafe { ffi::sappimgui_draw_menu(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_hud_window_content() {
    unsafe { ffi::sappimgui_draw_hud_window_content() }
}
#[inline]
pub fn draw_publicstate_window_content() {
    unsafe { ffi::sappimgui_draw_publicstate_window_content() }
}
#[inline]
pub fn draw_event_window_content() {
    unsafe { ffi::sappimgui_draw_event_window_content() }
}
#[inline]
pub fn draw_hud_window(title: &str) {
    let tmp_0 = std::ffi::CString::new(title).unwrap();
    unsafe { ffi::sappimgui_draw_hud_window(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_publicstate_window(title: &str) {
    let tmp_0 = std::ffi::CString::new(title).unwrap();
    unsafe { ffi::sappimgui_draw_publicstate_window(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_event_window(title: &str) {
    let tmp_0 = std::ffi::CString::new(title).unwrap();
    unsafe { ffi::sappimgui_draw_event_window(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_hud_menu_item(label: &str) {
    let tmp_0 = std::ffi::CString::new(label).unwrap();
    unsafe { ffi::sappimgui_draw_hud_menu_item(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_publicstate_menu_item(label: &str) {
    let tmp_0 = std::ffi::CString::new(label).unwrap();
    unsafe { ffi::sappimgui_draw_publicstate_menu_item(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_event_menu_item(label: &str) {
    let tmp_0 = std::ffi::CString::new(label).unwrap();
    unsafe { ffi::sappimgui_draw_event_menu_item(tmp_0.as_ptr()) }
}
