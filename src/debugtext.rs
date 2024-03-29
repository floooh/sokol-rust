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

/// Helper function to cast a Rust slice into a sokol Range
pub fn slice_as_range<T>(data: &[T]) -> Range {
    Range { size: std::mem::size_of_val(data), ptr: data.as_ptr() as *const _ }
}
/// Helper function to cast a Rust reference into a sokol Range
pub fn value_as_range<T>(value: &T) -> Range {
    Range { size: std::mem::size_of::<T>(), ptr: value as *const T as *const _ }
}

impl<T> From<&[T]> for Range {
    #[inline]
    fn from(data: &[T]) -> Self {
        slice_as_range(data)
    }
}
impl<T> From<&T> for Range {
    #[inline]
    fn from(value: &T) -> Self {
        value_as_range(value)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum LogItem {
    Ok,
    MallocFailed,
    AddCommitListenerFailed,
    CommandBufferFull,
    ContextPoolExhausted,
    CannotDestroyDefaultContext,
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
pub struct Context {
    pub id: u32,
}
impl Context {
    pub const fn new() -> Self {
        Self { id: 0 }
    }
}
impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Range {
    pub ptr: *const core::ffi::c_void,
    pub size: usize,
}
impl Range {
    pub const fn new() -> Self {
        Self { ptr: core::ptr::null(), size: 0 }
    }
}
impl Default for Range {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FontDesc {
    pub data: Range,
    pub first_char: u8,
    pub last_char: u8,
}
impl FontDesc {
    pub const fn new() -> Self {
        Self { data: Range::new(), first_char: 0, last_char: 0 }
    }
}
impl Default for FontDesc {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ContextDesc {
    pub max_commands: i32,
    pub char_buf_size: i32,
    pub canvas_width: f32,
    pub canvas_height: f32,
    pub tab_width: i32,
    pub color_format: sg::PixelFormat,
    pub depth_format: sg::PixelFormat,
    pub sample_count: i32,
}
impl ContextDesc {
    pub const fn new() -> Self {
        Self {
            max_commands: 0,
            char_buf_size: 0,
            canvas_width: 0.0,
            canvas_height: 0.0,
            tab_width: 0,
            color_format: sg::PixelFormat::new(),
            depth_format: sg::PixelFormat::new(),
            sample_count: 0,
        }
    }
}
impl Default for ContextDesc {
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
    pub context_pool_size: i32,
    pub printf_buf_size: i32,
    pub fonts: [FontDesc; 8],
    pub context: ContextDesc,
    pub allocator: Allocator,
    pub logger: Logger,
}
impl Desc {
    pub const fn new() -> Self {
        Self {
            context_pool_size: 0,
            printf_buf_size: 0,
            fonts: [FontDesc::new(); 8],
            context: ContextDesc::new(),
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
pub mod ffi {
    #![allow(unused_imports)]
    use super::*;
    extern "C" {
        pub fn sdtx_setup(desc: *const Desc);
        pub fn sdtx_shutdown();
        pub fn sdtx_font_kc853() -> FontDesc;
        pub fn sdtx_font_kc854() -> FontDesc;
        pub fn sdtx_font_z1013() -> FontDesc;
        pub fn sdtx_font_cpc() -> FontDesc;
        pub fn sdtx_font_c64() -> FontDesc;
        pub fn sdtx_font_oric() -> FontDesc;
        pub fn sdtx_make_context(desc: *const ContextDesc) -> Context;
        pub fn sdtx_destroy_context(ctx: Context);
        pub fn sdtx_set_context(ctx: Context);
        pub fn sdtx_get_context() -> Context;
        pub fn sdtx_default_context() -> Context;
        pub fn sdtx_draw();
        pub fn sdtx_context_draw(ctx: Context);
        pub fn sdtx_draw_layer(layer_id: i32);
        pub fn sdtx_context_draw_layer(ctx: Context, layer_id: i32);
        pub fn sdtx_layer(layer_id: i32);
        pub fn sdtx_font(font_index: usize);
        pub fn sdtx_canvas(w: f32, h: f32);
        pub fn sdtx_origin(x: f32, y: f32);
        pub fn sdtx_home();
        pub fn sdtx_pos(x: f32, y: f32);
        pub fn sdtx_pos_x(x: f32);
        pub fn sdtx_pos_y(y: f32);
        pub fn sdtx_move(dx: f32, dy: f32);
        pub fn sdtx_move_x(dx: f32);
        pub fn sdtx_move_y(dy: f32);
        pub fn sdtx_crlf();
        pub fn sdtx_color3b(r: u8, g: u8, b: u8);
        pub fn sdtx_color3f(r: f32, g: f32, b: f32);
        pub fn sdtx_color4b(r: u8, g: u8, b: u8, a: u8);
        pub fn sdtx_color4f(r: f32, g: f32, b: f32, a: f32);
        pub fn sdtx_color1i(rgba: u32);
        pub fn sdtx_putc(c: core::ffi::c_char);
        pub fn sdtx_puts(str: *const core::ffi::c_char);
        pub fn sdtx_putr(str: *const core::ffi::c_char, len: i32);
    }
}
#[inline]
pub fn setup(desc: &Desc) {
    unsafe { ffi::sdtx_setup(desc) }
}
#[inline]
pub fn shutdown() {
    unsafe { ffi::sdtx_shutdown() }
}
#[inline]
pub fn font_kc853() -> FontDesc {
    unsafe { ffi::sdtx_font_kc853() }
}
#[inline]
pub fn font_kc854() -> FontDesc {
    unsafe { ffi::sdtx_font_kc854() }
}
#[inline]
pub fn font_z1013() -> FontDesc {
    unsafe { ffi::sdtx_font_z1013() }
}
#[inline]
pub fn font_cpc() -> FontDesc {
    unsafe { ffi::sdtx_font_cpc() }
}
#[inline]
pub fn font_c64() -> FontDesc {
    unsafe { ffi::sdtx_font_c64() }
}
#[inline]
pub fn font_oric() -> FontDesc {
    unsafe { ffi::sdtx_font_oric() }
}
#[inline]
pub fn make_context(desc: &ContextDesc) -> Context {
    unsafe { ffi::sdtx_make_context(desc) }
}
#[inline]
pub fn destroy_context(ctx: Context) {
    unsafe { ffi::sdtx_destroy_context(ctx) }
}
#[inline]
pub fn set_context(ctx: Context) {
    unsafe { ffi::sdtx_set_context(ctx) }
}
#[inline]
pub fn get_context() -> Context {
    unsafe { ffi::sdtx_get_context() }
}
#[inline]
pub fn default_context() -> Context {
    unsafe { ffi::sdtx_default_context() }
}
#[inline]
pub fn draw() {
    unsafe { ffi::sdtx_draw() }
}
#[inline]
pub fn context_draw(ctx: Context) {
    unsafe { ffi::sdtx_context_draw(ctx) }
}
#[inline]
pub fn draw_layer(layer_id: i32) {
    unsafe { ffi::sdtx_draw_layer(layer_id) }
}
#[inline]
pub fn context_draw_layer(ctx: Context, layer_id: i32) {
    unsafe { ffi::sdtx_context_draw_layer(ctx, layer_id) }
}
#[inline]
pub fn layer(layer_id: i32) {
    unsafe { ffi::sdtx_layer(layer_id) }
}
#[inline]
pub fn font(font_index: usize) {
    unsafe { ffi::sdtx_font(font_index) }
}
#[inline]
pub fn canvas(w: f32, h: f32) {
    unsafe { ffi::sdtx_canvas(w, h) }
}
#[inline]
pub fn origin(x: f32, y: f32) {
    unsafe { ffi::sdtx_origin(x, y) }
}
#[inline]
pub fn home() {
    unsafe { ffi::sdtx_home() }
}
#[inline]
pub fn pos(x: f32, y: f32) {
    unsafe { ffi::sdtx_pos(x, y) }
}
#[inline]
pub fn pos_x(x: f32) {
    unsafe { ffi::sdtx_pos_x(x) }
}
#[inline]
pub fn pos_y(y: f32) {
    unsafe { ffi::sdtx_pos_y(y) }
}
#[inline]
pub fn move_cursor(dx: f32, dy: f32) {
    unsafe { ffi::sdtx_move(dx, dy) }
}
#[inline]
pub fn move_cursor_x(dx: f32) {
    unsafe { ffi::sdtx_move_x(dx) }
}
#[inline]
pub fn move_cursor_y(dy: f32) {
    unsafe { ffi::sdtx_move_y(dy) }
}
#[inline]
pub fn crlf() {
    unsafe { ffi::sdtx_crlf() }
}
#[inline]
pub fn color3b(r: u8, g: u8, b: u8) {
    unsafe { ffi::sdtx_color3b(r, g, b) }
}
#[inline]
pub fn color3f(r: f32, g: f32, b: f32) {
    unsafe { ffi::sdtx_color3f(r, g, b) }
}
#[inline]
pub fn color4b(r: u8, g: u8, b: u8, a: u8) {
    unsafe { ffi::sdtx_color4b(r, g, b, a) }
}
#[inline]
pub fn color4f(r: f32, g: f32, b: f32, a: f32) {
    unsafe { ffi::sdtx_color4f(r, g, b, a) }
}
#[inline]
pub fn color1i(rgba: u32) {
    unsafe { ffi::sdtx_color1i(rgba) }
}
#[inline]
pub fn putc(c: core::ffi::c_char) {
    unsafe { ffi::sdtx_putc(c) }
}
#[inline]
pub fn puts(str: &str) {
    let tmp_0 = std::ffi::CString::new(str).unwrap();
    unsafe { ffi::sdtx_puts(tmp_0.as_ptr()) }
}
#[inline]
pub fn putr(str: &str, len: i32) {
    let tmp_0 = std::ffi::CString::new(str).unwrap();
    unsafe { ffi::sdtx_putr(tmp_0.as_ptr(), len) }
}
