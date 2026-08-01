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
pub struct Framebuffer {
    pub id: u32,
}
impl Framebuffer {
    pub const fn new() -> Self {
        Self { id: 0 }
    }
}
impl Default for Framebuffer {
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
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum Format {
    Default = 0,
    Rgba8,
    Palette8,
}
impl Format {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for Format {
    fn default() -> Self {
        Self::Default
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
impl Rect {
    pub const fn new() -> Self {
        Self { x: 0, y: 0, width: 0, height: 0 }
    }
}
impl Default for Rect {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RenderPassDesc {
    pub color_format: sg::PixelFormat,
    pub depth_format: sg::PixelFormat,
    pub sample_count: i32,
}
impl RenderPassDesc {
    pub const fn new() -> Self {
        Self {
            color_format: sg::PixelFormat::new(),
            depth_format: sg::PixelFormat::new(),
            sample_count: 0,
        }
    }
}
impl Default for RenderPassDesc {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FramebufferDesc {
    pub width: i32,
    pub height: i32,
    pub prescale: i32,
    pub format: Format,
    pub cliprect: Rect,
    pub rotate90: bool,
    pub render_pass: RenderPassDesc,
}
impl FramebufferDesc {
    pub const fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            prescale: 0,
            format: Format::new(),
            cliprect: Rect::new(),
            rotate90: false,
            render_pass: RenderPassDesc::new(),
        }
    }
}
impl Default for FramebufferDesc {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ResizeDesc {
    pub width: i32,
    pub height: i32,
    pub prescale: i32,
    pub cliprect: Rect,
}
impl ResizeDesc {
    pub const fn new() -> Self {
        Self { width: 0, height: 0, prescale: 0, cliprect: Rect::new() }
    }
}
impl Default for ResizeDesc {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UpdateDesc {
    pub pixels: sg::Range,
    pub palette: sg::Range,
}
impl UpdateDesc {
    pub const fn new() -> Self {
        Self { pixels: sg::Range::new(), palette: sg::Range::new() }
    }
}
impl Default for UpdateDesc {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RenderDesc {
    pub use_nearest_filter: bool,
    pub pip: sg::Pipeline,
    pub views: [sg::View; 32],
    pub samplers: [sg::Sampler; 12],
    pub uniforms: [sg::Range; 8],
}
impl RenderDesc {
    pub const fn new() -> Self {
        Self {
            use_nearest_filter: false,
            pip: sg::Pipeline::new(),
            views: [sg::View::new(); 32],
            samplers: [sg::Sampler::new(); 12],
            uniforms: [sg::Range::new(); 8],
        }
    }
}
impl Default for RenderDesc {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TextureInfo {
    pub width: i32,
    pub height: i32,
    pub pixel_format: sg::PixelFormat,
    pub image: sg::Image,
    pub tex_view: sg::View,
}
impl TextureInfo {
    pub const fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            pixel_format: sg::PixelFormat::new(),
            image: sg::Image::new(),
            tex_view: sg::View::new(),
        }
    }
}
impl Default for TextureInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FramebufferInfo {
    pub update: TextureInfo,
    pub offscreen: TextureInfo,
    pub palette: TextureInfo,
    pub nearest_sampler: sg::Sampler,
    pub linear_sampler: sg::Sampler,
}
impl FramebufferInfo {
    pub const fn new() -> Self {
        Self {
            update: TextureInfo::new(),
            offscreen: TextureInfo::new(),
            palette: TextureInfo::new(),
            nearest_sampler: sg::Sampler::new(),
            linear_sampler: sg::Sampler::new(),
        }
    }
}
impl Default for FramebufferInfo {
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
    pub framebuffer_pool_size: i32,
    pub allocator: Allocator,
    pub logger: Logger,
}
impl Desc {
    pub const fn new() -> Self {
        Self { framebuffer_pool_size: 0, allocator: Allocator::new(), logger: Logger::new() }
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
        pub fn sfb_setup(desc: *const Desc);
        pub fn sfb_shutdown();
        pub fn sfb_make_framebuffer(desc: *const FramebufferDesc) -> Framebuffer;
        pub fn sfb_destroy_framebuffer(fb: Framebuffer);
        pub fn sfb_resize(fb: Framebuffer, desc: *const ResizeDesc) -> bool;
        pub fn sfb_update(fb: Framebuffer, desc: *const UpdateDesc);
        pub fn sfb_render(fb: Framebuffer);
        pub fn sfb_render_ex(fb: Framebuffer, desc: *const RenderDesc);
        pub fn sfb_query_framebuffer_state(fb: Framebuffer) -> ResourceState;
        pub fn sfb_query_framebuffer_info(fb: Framebuffer) -> FramebufferInfo;
        pub fn sfb_query_framebuffer_desc(fb: Framebuffer) -> FramebufferDesc;
    }
}
#[inline]
pub fn setup(desc: &Desc) {
    unsafe { ffi::sfb_setup(desc) }
}
#[inline]
pub fn shutdown() {
    unsafe { ffi::sfb_shutdown() }
}
#[inline]
pub fn make_framebuffer(desc: &FramebufferDesc) -> Framebuffer {
    unsafe { ffi::sfb_make_framebuffer(desc) }
}
#[inline]
pub fn destroy_framebuffer(fb: Framebuffer) {
    unsafe { ffi::sfb_destroy_framebuffer(fb) }
}
#[inline]
pub fn resize(fb: Framebuffer, desc: &ResizeDesc) -> bool {
    unsafe { ffi::sfb_resize(fb, desc) }
}
#[inline]
pub fn update(fb: Framebuffer, desc: &UpdateDesc) {
    unsafe { ffi::sfb_update(fb, desc) }
}
#[inline]
pub fn render(fb: Framebuffer) {
    unsafe { ffi::sfb_render(fb) }
}
#[inline]
pub fn render_ex(fb: Framebuffer, desc: &RenderDesc) {
    unsafe { ffi::sfb_render_ex(fb, desc) }
}
#[inline]
pub fn query_framebuffer_state(fb: Framebuffer) -> ResourceState {
    unsafe { ffi::sfb_query_framebuffer_state(fb) }
}
#[inline]
pub fn query_framebuffer_info(fb: Framebuffer) -> FramebufferInfo {
    unsafe { ffi::sfb_query_framebuffer_info(fb) }
}
#[inline]
pub fn query_framebuffer_desc(fb: Framebuffer) -> FramebufferDesc {
    unsafe { ffi::sfb_query_framebuffer_desc(fb) }
}
