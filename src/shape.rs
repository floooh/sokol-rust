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
pub struct Mat4 {
    pub m: [[f32; 4]; 4],
}
impl Mat4 {
    pub const fn new() -> Self {
        Self { m: [[0.0; 4]; 4] }
    }
}
impl Default for Mat4 {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub normal: u32,
    pub u: u16,
    pub v: u16,
    pub color: u32,
}
impl Vertex {
    pub const fn new() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, normal: 0, u: 0, v: 0, color: 0 }
    }
}
impl Default for Vertex {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ElementRange {
    pub base_element: usize,
    pub num_elements: usize,
}
impl ElementRange {
    pub const fn new() -> Self {
        Self { base_element: 0, num_elements: 0 }
    }
}
impl Default for ElementRange {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SizesItem {
    pub num: u32,
    pub size: u32,
}
impl SizesItem {
    pub const fn new() -> Self {
        Self { num: 0, size: 0 }
    }
}
impl Default for SizesItem {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Sizes {
    pub vertices: SizesItem,
    pub indices: SizesItem,
}
impl Sizes {
    pub const fn new() -> Self {
        Self { vertices: SizesItem::new(), indices: SizesItem::new() }
    }
}
impl Default for Sizes {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BufferItem {
    pub buffer: Range,
    pub data_size: usize,
    pub shape_offset: usize,
}
impl BufferItem {
    pub const fn new() -> Self {
        Self { buffer: Range::new(), data_size: 0, shape_offset: 0 }
    }
}
impl Default for BufferItem {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Buffer {
    pub valid: bool,
    pub vertices: BufferItem,
    pub indices: BufferItem,
}
impl Buffer {
    pub const fn new() -> Self {
        Self { valid: false, vertices: BufferItem::new(), indices: BufferItem::new() }
    }
}
impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Plane {
    pub width: f32,
    pub depth: f32,
    pub tiles: u16,
    pub color: u32,
    pub random_colors: bool,
    pub merge: bool,
    pub transform: Mat4,
}
impl Plane {
    pub const fn new() -> Self {
        Self {
            width: 0.0,
            depth: 0.0,
            tiles: 0,
            color: 0,
            random_colors: false,
            merge: false,
            transform: Mat4::new(),
        }
    }
}
impl Default for Plane {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Box {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub tiles: u16,
    pub color: u32,
    pub random_colors: bool,
    pub merge: bool,
    pub transform: Mat4,
}
impl Box {
    pub const fn new() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
            depth: 0.0,
            tiles: 0,
            color: 0,
            random_colors: false,
            merge: false,
            transform: Mat4::new(),
        }
    }
}
impl Default for Box {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub radius: f32,
    pub slices: u16,
    pub stacks: u16,
    pub color: u32,
    pub random_colors: bool,
    pub merge: bool,
    pub transform: Mat4,
}
impl Sphere {
    pub const fn new() -> Self {
        Self {
            radius: 0.0,
            slices: 0,
            stacks: 0,
            color: 0,
            random_colors: false,
            merge: false,
            transform: Mat4::new(),
        }
    }
}
impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Cylinder {
    pub radius: f32,
    pub height: f32,
    pub slices: u16,
    pub stacks: u16,
    pub color: u32,
    pub random_colors: bool,
    pub merge: bool,
    pub transform: Mat4,
}
impl Cylinder {
    pub const fn new() -> Self {
        Self {
            radius: 0.0,
            height: 0.0,
            slices: 0,
            stacks: 0,
            color: 0,
            random_colors: false,
            merge: false,
            transform: Mat4::new(),
        }
    }
}
impl Default for Cylinder {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Torus {
    pub radius: f32,
    pub ring_radius: f32,
    pub sides: u16,
    pub rings: u16,
    pub color: u32,
    pub random_colors: bool,
    pub merge: bool,
    pub transform: Mat4,
}
impl Torus {
    pub const fn new() -> Self {
        Self {
            radius: 0.0,
            ring_radius: 0.0,
            sides: 0,
            rings: 0,
            color: 0,
            random_colors: false,
            merge: false,
            transform: Mat4::new(),
        }
    }
}
impl Default for Torus {
    fn default() -> Self {
        Self::new()
    }
}
pub mod ffi {
    #![allow(unused_imports)]
    use super::*;
    extern "C" {
        pub fn sshape_build_plane(buf: *const Buffer, params: *const Plane) -> Buffer;
        pub fn sshape_build_box(buf: *const Buffer, params: *const Box) -> Buffer;
        pub fn sshape_build_sphere(buf: *const Buffer, params: *const Sphere) -> Buffer;
        pub fn sshape_build_cylinder(buf: *const Buffer, params: *const Cylinder) -> Buffer;
        pub fn sshape_build_torus(buf: *const Buffer, params: *const Torus) -> Buffer;
        pub fn sshape_plane_sizes(tiles: u32) -> Sizes;
        pub fn sshape_box_sizes(tiles: u32) -> Sizes;
        pub fn sshape_sphere_sizes(slices: u32, stacks: u32) -> Sizes;
        pub fn sshape_cylinder_sizes(slices: u32, stacks: u32) -> Sizes;
        pub fn sshape_torus_sizes(sides: u32, rings: u32) -> Sizes;
        pub fn sshape_element_range(buf: *const Buffer) -> ElementRange;
        pub fn sshape_vertex_buffer_desc(buf: *const Buffer) -> sg::BufferDesc;
        pub fn sshape_index_buffer_desc(buf: *const Buffer) -> sg::BufferDesc;
        pub fn sshape_vertex_buffer_layout_state() -> sg::VertexBufferLayoutState;
        pub fn sshape_position_vertex_attr_state() -> sg::VertexAttrState;
        pub fn sshape_normal_vertex_attr_state() -> sg::VertexAttrState;
        pub fn sshape_texcoord_vertex_attr_state() -> sg::VertexAttrState;
        pub fn sshape_color_vertex_attr_state() -> sg::VertexAttrState;
        pub fn sshape_color_4f(r: f32, g: f32, b: f32, a: f32) -> u32;
        pub fn sshape_color_3f(r: f32, g: f32, b: f32) -> u32;
        pub fn sshape_color_4b(r: u8, g: u8, b: u8, a: u8) -> u32;
        pub fn sshape_color_3b(r: u8, g: u8, b: u8) -> u32;
        pub fn sshape_mat4(m: *const f32) -> Mat4;
        pub fn sshape_mat4_transpose(m: *const f32) -> Mat4;
    }
}
#[inline]
pub fn build_plane(buf: &Buffer, params: &Plane) -> Buffer {
    unsafe { ffi::sshape_build_plane(buf, params) }
}
#[inline]
pub fn build_box(buf: &Buffer, params: &Box) -> Buffer {
    unsafe { ffi::sshape_build_box(buf, params) }
}
#[inline]
pub fn build_sphere(buf: &Buffer, params: &Sphere) -> Buffer {
    unsafe { ffi::sshape_build_sphere(buf, params) }
}
#[inline]
pub fn build_cylinder(buf: &Buffer, params: &Cylinder) -> Buffer {
    unsafe { ffi::sshape_build_cylinder(buf, params) }
}
#[inline]
pub fn build_torus(buf: &Buffer, params: &Torus) -> Buffer {
    unsafe { ffi::sshape_build_torus(buf, params) }
}
#[inline]
pub fn plane_sizes(tiles: u32) -> Sizes {
    unsafe { ffi::sshape_plane_sizes(tiles) }
}
#[inline]
pub fn box_sizes(tiles: u32) -> Sizes {
    unsafe { ffi::sshape_box_sizes(tiles) }
}
#[inline]
pub fn sphere_sizes(slices: u32, stacks: u32) -> Sizes {
    unsafe { ffi::sshape_sphere_sizes(slices, stacks) }
}
#[inline]
pub fn cylinder_sizes(slices: u32, stacks: u32) -> Sizes {
    unsafe { ffi::sshape_cylinder_sizes(slices, stacks) }
}
#[inline]
pub fn torus_sizes(sides: u32, rings: u32) -> Sizes {
    unsafe { ffi::sshape_torus_sizes(sides, rings) }
}
#[inline]
pub fn element_range(buf: &Buffer) -> ElementRange {
    unsafe { ffi::sshape_element_range(buf) }
}
#[inline]
pub fn vertex_buffer_desc(buf: &Buffer) -> sg::BufferDesc {
    unsafe { ffi::sshape_vertex_buffer_desc(buf) }
}
#[inline]
pub fn index_buffer_desc(buf: &Buffer) -> sg::BufferDesc {
    unsafe { ffi::sshape_index_buffer_desc(buf) }
}
#[inline]
pub fn vertex_buffer_layout_state() -> sg::VertexBufferLayoutState {
    unsafe { ffi::sshape_vertex_buffer_layout_state() }
}
#[inline]
pub fn position_vertex_attr_state() -> sg::VertexAttrState {
    unsafe { ffi::sshape_position_vertex_attr_state() }
}
#[inline]
pub fn normal_vertex_attr_state() -> sg::VertexAttrState {
    unsafe { ffi::sshape_normal_vertex_attr_state() }
}
#[inline]
pub fn texcoord_vertex_attr_state() -> sg::VertexAttrState {
    unsafe { ffi::sshape_texcoord_vertex_attr_state() }
}
#[inline]
pub fn color_vertex_attr_state() -> sg::VertexAttrState {
    unsafe { ffi::sshape_color_vertex_attr_state() }
}
#[inline]
pub fn color_4f(r: f32, g: f32, b: f32, a: f32) -> u32 {
    unsafe { ffi::sshape_color_4f(r, g, b, a) }
}
#[inline]
pub fn color_3f(r: f32, g: f32, b: f32) -> u32 {
    unsafe { ffi::sshape_color_3f(r, g, b) }
}
#[inline]
pub fn color_4b(r: u8, g: u8, b: u8, a: u8) -> u32 {
    unsafe { ffi::sshape_color_4b(r, g, b, a) }
}
#[inline]
pub fn color_3b(r: u8, g: u8, b: u8) -> u32 {
    unsafe { ffi::sshape_color_3b(r, g, b) }
}
#[inline]
pub fn mat4(m: &f32) -> Mat4 {
    unsafe { ffi::sshape_mat4(m) }
}
#[inline]
pub fn mat4_transpose(m: &f32) -> Mat4 {
    unsafe { ffi::sshape_mat4_transpose(m) }
}
