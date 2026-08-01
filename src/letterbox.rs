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
pub struct Border {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}
impl Border {
    pub const fn new() -> Self {
        Self { left: 0, right: 0, top: 0, bottom: 0 }
    }
}
impl Default for Border {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum Anchor {
    Center = 0,
    Top,
    Bottom,
    Left,
    Right,
}
impl Anchor {
    pub const fn new() -> Self {
        Self::Center
    }
}
impl Default for Anchor {
    fn default() -> Self {
        Self::Center
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LetterboxDesc {
    pub content_aspect_ratio: f32,
    pub anchor: Anchor,
    pub border: Border,
}
impl LetterboxDesc {
    pub const fn new() -> Self {
        Self { content_aspect_ratio: 0.0, anchor: Anchor::new(), border: Border::new() }
    }
}
impl Default for LetterboxDesc {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
impl Viewport {
    pub const fn new() -> Self {
        Self { x: 0, y: 0, width: 0, height: 0 }
    }
}
impl Default for Viewport {
    fn default() -> Self {
        Self::new()
    }
}
pub mod ffi {
    #![allow(unused_imports)]
    use super::*;
    extern "C" {
        pub fn slbx_letterbox(width: i32, height: i32, desc: *const LetterboxDesc) -> Viewport;
    }
}
#[inline]
pub fn letterbox(width: i32, height: i32, desc: &LetterboxDesc) -> Viewport {
    unsafe { ffi::slbx_letterbox(width, height, desc) }
}
