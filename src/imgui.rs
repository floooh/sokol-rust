//! To use this module, enable the feature "imgui"
// machine generated, do not edit

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::{app as sapp, gfx as sg};

/// Helper function to convert a C string to a rust string slice
#[inline]
fn c_char_ptr_to_rust_str(c_char_ptr: *const core::ffi::c_char) -> &'static str {
    let c_str = unsafe { core::ffi::CStr::from_ptr(c_char_ptr) };
    c_str.to_str().expect("c_char_ptr contained invalid Utf8 Data")
}

pub const INVALID_ID: usize = 0;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Image {
    pub id: u32,
}
impl Image {
    pub const fn new() -> Self {
        Self { id: 0 }
    }
}
impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImageDesc {
    pub image: sg::Image,
    pub sampler: sg::Sampler,
}
impl ImageDesc {
    pub const fn new() -> Self {
        Self { image: sg::Image::new(), sampler: sg::Sampler::new() }
    }
}
impl Default for ImageDesc {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum LogItem {
    Ok,
    MallocFailed,
    ImagePoolExhausted,
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
    pub image_pool_size: i32,
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
            image_pool_size: 0,
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
pub mod ffi {
    #![allow(unused_imports)]
    use super::*;
    extern "C" {
        pub fn simgui_setup(desc: *const Desc);
        pub fn simgui_new_frame(desc: *const FrameDesc);
        pub fn simgui_render();
        pub fn simgui_make_image(desc: *const ImageDesc) -> Image;
        pub fn simgui_destroy_image(img: Image);
        pub fn simgui_query_image_desc(img: Image) -> ImageDesc;
        pub fn simgui_imtextureid(img: Image) -> *mut core::ffi::c_void;
        pub fn simgui_image_from_imtextureid(imtextureid: *mut core::ffi::c_void) -> Image;
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
pub fn make_image(desc: &ImageDesc) -> Image {
    unsafe { ffi::simgui_make_image(desc) }
}
#[inline]
pub fn destroy_image(img: Image) {
    unsafe { ffi::simgui_destroy_image(img) }
}
#[inline]
pub fn query_image_desc(img: Image) -> ImageDesc {
    unsafe { ffi::simgui_query_image_desc(img) }
}
#[inline]
pub fn imtextureid(img: Image) -> *mut core::ffi::c_void {
    unsafe { ffi::simgui_imtextureid(img) }
}
#[inline]
pub fn image_from_imtextureid(imtextureid: *mut core::ffi::c_void) -> Image {
    unsafe { ffi::simgui_image_from_imtextureid(imtextureid) }
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
