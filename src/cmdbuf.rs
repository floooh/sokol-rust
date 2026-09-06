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

pub const INVALID_ID: usize = 0;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Cmdbuf {
    pub id: u32,
}
impl Cmdbuf {
    pub const fn new() -> Self {
        Self { id: 0 }
    }
}
impl Default for Cmdbuf {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum ResourceState {
    Initial,
    Alloc,
    Valid,
    Failed,
    Invalid,
}
impl ResourceState {
    pub const fn new() -> Self {
        Self::Initial
    }
}
impl Default for ResourceState {
    fn default() -> Self {
        Self::Initial
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CmdbufDesc {
    pub size: usize,
    pub label: *const core::ffi::c_char,
}
impl CmdbufDesc {
    pub const fn new() -> Self {
        Self { size: 0, label: core::ptr::null() }
    }
}
impl Default for CmdbufDesc {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CmdbufInfo {
    pub size: usize,
    pub remaining: usize,
    pub overflown: bool,
}
impl CmdbufInfo {
    pub const fn new() -> Self {
        Self { size: 0, remaining: 0, overflown: false }
    }
}
impl Default for CmdbufInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum LogItem {
    Ok,
    MallocFailed,
    CmdbufPoolExhausted,
    CmdbufOverflow,
    CmdbufNotValid,
    SubmitCmdbufOverflown,
    SubmitInvalidCommand,
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
    pub cmdbuf_pool_size: i32,
    pub allocator: Allocator,
    pub logger: Logger,
}
impl Desc {
    pub const fn new() -> Self {
        Self { cmdbuf_pool_size: 0, allocator: Allocator::new(), logger: Logger::new() }
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
        pub fn scb_setup(desc: *const Desc);
        pub fn scb_shutdown();
        pub fn scb_make_cmdbuf(desc: *const CmdbufDesc) -> Cmdbuf;
        pub fn scb_destroy_cmdbuf(cb: Cmdbuf);
        pub fn scb_submit(cb: Cmdbuf);
        pub fn scb_reset(cb: Cmdbuf);
        pub fn scb_apply_viewport(cb: Cmdbuf, x: i32, y: i32, width: i32, height: i32, origin_top_left: bool);
        pub fn scb_apply_viewportf(
            cb: Cmdbuf,
            x: f32,
            y: f32,
            width: f32,
            height: f32,
            origin_top_left: bool,
        );
        pub fn scb_apply_scissor_rect(
            cb: Cmdbuf,
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            origin_top_left: bool,
        );
        pub fn scb_apply_scissor_rectf(
            cb: Cmdbuf,
            x: f32,
            y: f32,
            width: f32,
            height: f32,
            origin_top_left: bool,
        );
        pub fn scb_apply_pipeline(cb: Cmdbuf, pip: sg::Pipeline);
        pub fn scb_apply_bindings(cb: Cmdbuf, bindings: *const sg::Bindings);
        pub fn scb_apply_uniforms(cb: Cmdbuf, ub_slot: i32, data: *const sg::Range);
        pub fn scb_draw(cb: Cmdbuf, base_element: i32, num_elements: i32, num_instances: i32);
        pub fn scb_draw_ex(
            cb: Cmdbuf,
            base_element: i32,
            num_elements: i32,
            num_instances: i32,
            base_vertex: i32,
            base_instance: i32,
        );
        pub fn scb_dispatch(cb: Cmdbuf, num_groups_x: i32, num_groups_y: i32, num_groups_z: i32);
        pub fn scb_query_cmdbuf_state(cb: Cmdbuf) -> ResourceState;
        pub fn scb_query_cmdbuf_info(cb: Cmdbuf) -> CmdbufInfo;
    }
}
#[inline]
pub fn setup(desc: &Desc) {
    unsafe { ffi::scb_setup(desc) }
}
#[inline]
pub fn shutdown() {
    unsafe { ffi::scb_shutdown() }
}
#[inline]
pub fn make_cmdbuf(desc: &CmdbufDesc) -> Cmdbuf {
    unsafe { ffi::scb_make_cmdbuf(desc) }
}
#[inline]
pub fn destroy_cmdbuf(cb: Cmdbuf) {
    unsafe { ffi::scb_destroy_cmdbuf(cb) }
}
#[inline]
pub fn submit(cb: Cmdbuf) {
    unsafe { ffi::scb_submit(cb) }
}
#[inline]
pub fn reset(cb: Cmdbuf) {
    unsafe { ffi::scb_reset(cb) }
}
#[inline]
pub fn apply_viewport(cb: Cmdbuf, x: i32, y: i32, width: i32, height: i32, origin_top_left: bool) {
    unsafe { ffi::scb_apply_viewport(cb, x, y, width, height, origin_top_left) }
}
#[inline]
pub fn apply_viewportf(cb: Cmdbuf, x: f32, y: f32, width: f32, height: f32, origin_top_left: bool) {
    unsafe { ffi::scb_apply_viewportf(cb, x, y, width, height, origin_top_left) }
}
#[inline]
pub fn apply_scissor_rect(cb: Cmdbuf, x: i32, y: i32, width: i32, height: i32, origin_top_left: bool) {
    unsafe { ffi::scb_apply_scissor_rect(cb, x, y, width, height, origin_top_left) }
}
#[inline]
pub fn apply_scissor_rectf(cb: Cmdbuf, x: f32, y: f32, width: f32, height: f32, origin_top_left: bool) {
    unsafe { ffi::scb_apply_scissor_rectf(cb, x, y, width, height, origin_top_left) }
}
#[inline]
pub fn apply_pipeline(cb: Cmdbuf, pip: sg::Pipeline) {
    unsafe { ffi::scb_apply_pipeline(cb, pip) }
}
#[inline]
pub fn apply_bindings(cb: Cmdbuf, bindings: &sg::Bindings) {
    unsafe { ffi::scb_apply_bindings(cb, bindings) }
}
#[inline]
pub fn apply_uniforms(cb: Cmdbuf, ub_slot: i32, data: &sg::Range) {
    unsafe { ffi::scb_apply_uniforms(cb, ub_slot, data) }
}
#[inline]
pub fn draw(cb: Cmdbuf, base_element: i32, num_elements: i32, num_instances: i32) {
    unsafe { ffi::scb_draw(cb, base_element, num_elements, num_instances) }
}
#[inline]
pub fn draw_ex(
    cb: Cmdbuf,
    base_element: i32,
    num_elements: i32,
    num_instances: i32,
    base_vertex: i32,
    base_instance: i32,
) {
    unsafe { ffi::scb_draw_ex(cb, base_element, num_elements, num_instances, base_vertex, base_instance) }
}
#[inline]
pub fn dispatch(cb: Cmdbuf, num_groups_x: i32, num_groups_y: i32, num_groups_z: i32) {
    unsafe { ffi::scb_dispatch(cb, num_groups_x, num_groups_y, num_groups_z) }
}
#[inline]
pub fn query_cmdbuf_state(cb: Cmdbuf) -> ResourceState {
    unsafe { ffi::scb_query_cmdbuf_state(cb) }
}
#[inline]
pub fn query_cmdbuf_info(cb: Cmdbuf) -> CmdbufInfo {
    unsafe { ffi::scb_query_cmdbuf_info(cb) }
}
