//! To use this module, enable the feature "imgui"
// machine generated, do not edit

#![allow(dead_code)]
#![allow(unused_imports)]

/// Helper function to convert a C string to a Rust string slice
#[inline]
fn c_char_ptr_to_rust_str(c_char_ptr: *const core::ffi::c_char) -> &'static str {
    let c_str = unsafe { core::ffi::CStr::from_ptr(c_char_ptr) };
    c_str.to_str().expect("c_char_ptr contained invalid Utf8 Data")
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
pub struct Desc {
    pub allocator: Allocator,
}
impl Desc {
    pub const fn new() -> Self {
        Self { allocator: Allocator::new() }
    }
}
impl Default for Desc {
    fn default() -> Self {
        Self::new()
    }
}
pub mod ffi {
    #![allow(unused_imports)]
    use super::*;
    extern "C" {
        pub fn sgimgui_setup(desc: *const Desc);
        pub fn sgimgui_shutdown();
        pub fn sgimgui_draw();
        pub fn sgimgui_draw_menu(title: *const core::ffi::c_char);
        pub fn sgimgui_draw_buffer_window_content();
        pub fn sgimgui_draw_image_window_content();
        pub fn sgimgui_draw_sampler_window_content();
        pub fn sgimgui_draw_shader_window_content();
        pub fn sgimgui_draw_pipeline_window_content();
        pub fn sgimgui_draw_view_window_content();
        pub fn sgimgui_draw_capture_window_content();
        pub fn sgimgui_draw_capabilities_window_content();
        pub fn sgimgui_draw_frame_stats_window_content();
        pub fn sgimgui_draw_buffer_window(title: *const core::ffi::c_char);
        pub fn sgimgui_draw_image_window(title: *const core::ffi::c_char);
        pub fn sgimgui_draw_sampler_window(title: *const core::ffi::c_char);
        pub fn sgimgui_draw_shader_window(title: *const core::ffi::c_char);
        pub fn sgimgui_draw_pipeline_window(title: *const core::ffi::c_char);
        pub fn sgimgui_draw_view_window(title: *const core::ffi::c_char);
        pub fn sgimgui_draw_capture_window(title: *const core::ffi::c_char);
        pub fn sgimgui_draw_capabilities_window(title: *const core::ffi::c_char);
        pub fn sgimgui_draw_frame_stats_window(title: *const core::ffi::c_char);
        pub fn sgimgui_draw_buffer_menu_item(label: *const core::ffi::c_char);
        pub fn sgimgui_draw_image_menu_item(label: *const core::ffi::c_char);
        pub fn sgimgui_draw_sampler_menu_item(label: *const core::ffi::c_char);
        pub fn sgimgui_draw_shader_menu_item(label: *const core::ffi::c_char);
        pub fn sgimgui_draw_pipeline_menu_item(label: *const core::ffi::c_char);
        pub fn sgimgui_draw_view_menu_item(label: *const core::ffi::c_char);
        pub fn sgimgui_draw_capture_menu_item(label: *const core::ffi::c_char);
        pub fn sgimgui_draw_capabilities_menu_item(label: *const core::ffi::c_char);
        pub fn sgimgui_draw_frame_stats_menu_item(label: *const core::ffi::c_char);
    }
}
#[inline]
pub fn setup(desc: &Desc) {
    unsafe { ffi::sgimgui_setup(desc) }
}
#[inline]
pub fn shutdown() {
    unsafe { ffi::sgimgui_shutdown() }
}
#[inline]
pub fn draw() {
    unsafe { ffi::sgimgui_draw() }
}
#[inline]
pub fn draw_menu(title: &str) {
    let tmp_0 = std::ffi::CString::new(title).unwrap();
    unsafe { ffi::sgimgui_draw_menu(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_buffer_window_content() {
    unsafe { ffi::sgimgui_draw_buffer_window_content() }
}
#[inline]
pub fn draw_image_window_content() {
    unsafe { ffi::sgimgui_draw_image_window_content() }
}
#[inline]
pub fn draw_sampler_window_content() {
    unsafe { ffi::sgimgui_draw_sampler_window_content() }
}
#[inline]
pub fn draw_shader_window_content() {
    unsafe { ffi::sgimgui_draw_shader_window_content() }
}
#[inline]
pub fn draw_pipeline_window_content() {
    unsafe { ffi::sgimgui_draw_pipeline_window_content() }
}
#[inline]
pub fn draw_view_window_content() {
    unsafe { ffi::sgimgui_draw_view_window_content() }
}
#[inline]
pub fn draw_capture_window_content() {
    unsafe { ffi::sgimgui_draw_capture_window_content() }
}
#[inline]
pub fn draw_capabilities_window_content() {
    unsafe { ffi::sgimgui_draw_capabilities_window_content() }
}
#[inline]
pub fn draw_frame_stats_window_content() {
    unsafe { ffi::sgimgui_draw_frame_stats_window_content() }
}
#[inline]
pub fn draw_buffer_window(title: &str) {
    let tmp_0 = std::ffi::CString::new(title).unwrap();
    unsafe { ffi::sgimgui_draw_buffer_window(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_image_window(title: &str) {
    let tmp_0 = std::ffi::CString::new(title).unwrap();
    unsafe { ffi::sgimgui_draw_image_window(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_sampler_window(title: &str) {
    let tmp_0 = std::ffi::CString::new(title).unwrap();
    unsafe { ffi::sgimgui_draw_sampler_window(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_shader_window(title: &str) {
    let tmp_0 = std::ffi::CString::new(title).unwrap();
    unsafe { ffi::sgimgui_draw_shader_window(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_pipeline_window(title: &str) {
    let tmp_0 = std::ffi::CString::new(title).unwrap();
    unsafe { ffi::sgimgui_draw_pipeline_window(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_view_window(title: &str) {
    let tmp_0 = std::ffi::CString::new(title).unwrap();
    unsafe { ffi::sgimgui_draw_view_window(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_capture_window(title: &str) {
    let tmp_0 = std::ffi::CString::new(title).unwrap();
    unsafe { ffi::sgimgui_draw_capture_window(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_capabilities_window(title: &str) {
    let tmp_0 = std::ffi::CString::new(title).unwrap();
    unsafe { ffi::sgimgui_draw_capabilities_window(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_frame_stats_window(title: &str) {
    let tmp_0 = std::ffi::CString::new(title).unwrap();
    unsafe { ffi::sgimgui_draw_frame_stats_window(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_buffer_menu_item(label: &str) {
    let tmp_0 = std::ffi::CString::new(label).unwrap();
    unsafe { ffi::sgimgui_draw_buffer_menu_item(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_image_menu_item(label: &str) {
    let tmp_0 = std::ffi::CString::new(label).unwrap();
    unsafe { ffi::sgimgui_draw_image_menu_item(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_sampler_menu_item(label: &str) {
    let tmp_0 = std::ffi::CString::new(label).unwrap();
    unsafe { ffi::sgimgui_draw_sampler_menu_item(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_shader_menu_item(label: &str) {
    let tmp_0 = std::ffi::CString::new(label).unwrap();
    unsafe { ffi::sgimgui_draw_shader_menu_item(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_pipeline_menu_item(label: &str) {
    let tmp_0 = std::ffi::CString::new(label).unwrap();
    unsafe { ffi::sgimgui_draw_pipeline_menu_item(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_view_menu_item(label: &str) {
    let tmp_0 = std::ffi::CString::new(label).unwrap();
    unsafe { ffi::sgimgui_draw_view_menu_item(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_capture_menu_item(label: &str) {
    let tmp_0 = std::ffi::CString::new(label).unwrap();
    unsafe { ffi::sgimgui_draw_capture_menu_item(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_capabilities_menu_item(label: &str) {
    let tmp_0 = std::ffi::CString::new(label).unwrap();
    unsafe { ffi::sgimgui_draw_capabilities_menu_item(tmp_0.as_ptr()) }
}
#[inline]
pub fn draw_frame_stats_menu_item(label: &str) {
    let tmp_0 = std::ffi::CString::new(label).unwrap();
    unsafe { ffi::sgimgui_draw_frame_stats_menu_item(tmp_0.as_ptr()) }
}
