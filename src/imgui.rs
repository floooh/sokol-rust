//! To use this module, enable the feature "imgui"
// machine generated, do not edit

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::app as sapp;
use crate::gfx as sg;

/// Helper function to convert a C string to a Rust string slice
#[inline]
fn c_char_ptr_to_rust_str(c_char_ptr: *const core::ffi::c_char) -> &'static str {
    let c_str = unsafe { core::ffi::CStr::from_ptr(c_char_ptr) };
    c_str.to_str().expect("c_char_ptr contained invalid Utf8 Data")
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum LogItem {
    Ok,
    MallocFailed,
    BufferOverflow,
}
impl LogItem {
    pub const fn new() -> Self {
        Self::Ok
    }
}
impl Default for LogItem {
    fn default() -> Self {
        Self::Ok
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Allocator {
    pub alloc_fn: Option<extern "C" fn(usize, *mut core::ffi::c_void) -> *mut core::ffi::c_void>,
    pub free_fn: Option<extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void)>,
    pub user_data: *mut core::ffi::c_void,
}
impl Allocator {
    pub const fn new() -> Self {
        Self { alloc_fn: None, free_fn: None, user_data: core::ptr::null_mut() }
    }
}
impl Default for Allocator {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Logger {
    pub func: Option<
        extern "C" fn(
            *const core::ffi::c_char,
            u32,
            u32,
            *const core::ffi::c_char,
            u32,
            *const core::ffi::c_char,
            *mut core::ffi::c_void,
        ),
    >,
    pub user_data: *mut core::ffi::c_void,
}
impl Logger {
    pub const fn new() -> Self {
        Self { func: None, user_data: core::ptr::null_mut() }
    }
}
impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Desc {
    pub max_vertices: i32,
    pub color_format: sg::PixelFormat,
    pub depth_format: sg::PixelFormat,
    pub sample_count: i32,
    pub ini_filename: *const core::ffi::c_char,
    pub no_default_font: bool,
    pub disable_paste_override: bool,
    pub disable_set_mouse_cursor: bool,
    pub disable_windows_resize_from_edges: bool,
    pub write_alpha_channel: bool,
    pub allocator: Allocator,
    pub logger: Logger,
}
impl Desc {
    pub const fn new() -> Self {
        Self {
            max_vertices: 0,
            color_format: sg::PixelFormat::new(),
            depth_format: sg::PixelFormat::new(),
            sample_count: 0,
            ini_filename: core::ptr::null(),
            no_default_font: false,
            disable_paste_override: false,
            disable_set_mouse_cursor: false,
            disable_windows_resize_from_edges: false,
            write_alpha_channel: false,
            allocator: Allocator::new(),
            logger: Logger::new(),
        }
    }
}
impl Default for Desc {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FrameDesc {
    pub width: i32,
    pub height: i32,
    pub delta_time: f64,
    pub dpi_scale: f32,
}
impl FrameDesc {
    pub const fn new() -> Self {
        Self { width: 0, height: 0, delta_time: 0.0, dpi_scale: 0.0 }
    }
}
impl Default for FrameDesc {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FontTexDesc {
    pub min_filter: sg::Filter,
    pub mag_filter: sg::Filter,
}
impl FontTexDesc {
    pub const fn new() -> Self {
        Self { min_filter: sg::Filter::new(), mag_filter: sg::Filter::new() }
    }
}
impl Default for FontTexDesc {
    fn default() -> Self {
        Self::new()
    }
}
pub mod ffi {
    #![allow(unused_imports)]
    use super::*;
    extern "C" {
        pub fn simgui_setup(desc: *const Desc);
        pub fn simgui_new_frame(desc: *const FrameDesc);
        pub fn simgui_render();
        pub fn simgui_imtextureid(tex_view: sg::View) -> u64;
        pub fn simgui_imtextureid_with_sampler(tex_view: sg::View, smp: sg::Sampler) -> u64;
        pub fn simgui_texture_view_from_imtextureid(imtex_id: u64) -> sg::View;
        pub fn simgui_sampler_from_imtextureid(imtex_id: u64) -> sg::Sampler;
        pub fn simgui_add_focus_event(focus: bool);
        pub fn simgui_add_mouse_pos_event(x: f32, y: f32);
        pub fn simgui_add_touch_pos_event(x: f32, y: f32);
        pub fn simgui_add_mouse_button_event(mouse_button: i32, down: bool);
        pub fn simgui_add_mouse_wheel_event(wheel_x: f32, wheel_y: f32);
        pub fn simgui_add_input_character(c: u32);
        pub fn simgui_add_input_characters_utf8(c: *const core::ffi::c_char);
        pub fn simgui_add_touch_button_event(mouse_button: i32, down: bool);
        pub fn simgui_handle_event(ev: *const sapp::Event) -> bool;
        pub fn simgui_map_keycode(keycode: sapp::Keycode) -> i32;
        pub fn simgui_shutdown();
    }
}
#[inline]
pub fn setup(desc: &Desc) {
    unsafe { ffi::simgui_setup(desc) }
}
#[inline]
pub fn new_frame(desc: &FrameDesc) {
    unsafe { ffi::simgui_new_frame(desc) }
}
#[inline]
pub fn render() {
    unsafe { ffi::simgui_render() }
}
#[inline]
pub fn imtextureid(tex_view: sg::View) -> u64 {
    unsafe { ffi::simgui_imtextureid(tex_view) }
}
#[inline]
pub fn imtextureid_with_sampler(tex_view: sg::View, smp: sg::Sampler) -> u64 {
    unsafe { ffi::simgui_imtextureid_with_sampler(tex_view, smp) }
}
#[inline]
pub fn texture_view_from_imtextureid(imtex_id: u64) -> sg::View {
    unsafe { ffi::simgui_texture_view_from_imtextureid(imtex_id) }
}
#[inline]
pub fn sampler_from_imtextureid(imtex_id: u64) -> sg::Sampler {
    unsafe { ffi::simgui_sampler_from_imtextureid(imtex_id) }
}
#[inline]
pub fn add_focus_event(focus: bool) {
    unsafe { ffi::simgui_add_focus_event(focus) }
}
#[inline]
pub fn add_mouse_pos_event(x: f32, y: f32) {
    unsafe { ffi::simgui_add_mouse_pos_event(x, y) }
}
#[inline]
pub fn add_touch_pos_event(x: f32, y: f32) {
    unsafe { ffi::simgui_add_touch_pos_event(x, y) }
}
#[inline]
pub fn add_mouse_button_event(mouse_button: i32, down: bool) {
    unsafe { ffi::simgui_add_mouse_button_event(mouse_button, down) }
}
#[inline]
pub fn add_mouse_wheel_event(wheel_x: f32, wheel_y: f32) {
    unsafe { ffi::simgui_add_mouse_wheel_event(wheel_x, wheel_y) }
}
#[inline]
pub fn add_input_character(c: u32) {
    unsafe { ffi::simgui_add_input_character(c) }
}
#[inline]
pub fn add_input_characters_utf8(c: &str) {
    let tmp_0 = std::ffi::CString::new(c).unwrap();
    unsafe { ffi::simgui_add_input_characters_utf8(tmp_0.as_ptr()) }
}
#[inline]
pub fn add_touch_button_event(mouse_button: i32, down: bool) {
    unsafe { ffi::simgui_add_touch_button_event(mouse_button, down) }
}
#[inline]
pub fn handle_event(ev: &sapp::Event) -> bool {
    unsafe { ffi::simgui_handle_event(ev) }
}
#[inline]
pub fn map_keycode(keycode: sapp::Keycode) -> i32 {
    unsafe { ffi::simgui_map_keycode(keycode) }
}
#[inline]
pub fn shutdown() {
    unsafe { ffi::simgui_shutdown() }
}
