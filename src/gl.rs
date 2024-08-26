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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum LogItem {
    Ok,
    MallocFailed,
    MakePipelineFailed,
    PipelinePoolExhausted,
    AddCommitListenerFailed,
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
pub struct Pipeline {
    pub id: u32,
}
impl Pipeline {
    pub const fn new() -> Self {
        Self { id: 0 }
    }
}
impl Default for Pipeline {
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
pub struct Error {
    pub any: bool,
    pub vertices_full: bool,
    pub uniforms_full: bool,
    pub commands_full: bool,
    pub stack_overflow: bool,
    pub stack_underflow: bool,
    pub no_context: bool,
}
impl Error {
    pub const fn new() -> Self {
        Self {
            any: false,
            vertices_full: false,
            uniforms_full: false,
            commands_full: false,
            stack_overflow: false,
            stack_underflow: false,
            no_context: false,
        }
    }
}
impl Default for Error {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ContextDesc {
    pub max_vertices: i32,
    pub max_commands: i32,
    pub color_format: sg::PixelFormat,
    pub depth_format: sg::PixelFormat,
    pub sample_count: i32,
}
impl ContextDesc {
    pub const fn new() -> Self {
        Self {
            max_vertices: 0,
            max_commands: 0,
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
    pub max_vertices: i32,
    pub max_commands: i32,
    pub context_pool_size: i32,
    pub pipeline_pool_size: i32,
    pub color_format: sg::PixelFormat,
    pub depth_format: sg::PixelFormat,
    pub sample_count: i32,
    pub face_winding: sg::FaceWinding,
    pub allocator: Allocator,
    pub logger: Logger,
}
impl Desc {
    pub const fn new() -> Self {
        Self {
            max_vertices: 0,
            max_commands: 0,
            context_pool_size: 0,
            pipeline_pool_size: 0,
            color_format: sg::PixelFormat::new(),
            depth_format: sg::PixelFormat::new(),
            sample_count: 0,
            face_winding: sg::FaceWinding::new(),
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
        pub fn sgl_setup(desc: *const Desc);
        pub fn sgl_shutdown();
        pub fn sgl_rad(deg: f32) -> f32;
        pub fn sgl_deg(rad: f32) -> f32;
        pub fn sgl_error() -> Error;
        pub fn sgl_context_error(ctx: Context) -> Error;
        pub fn sgl_make_context(desc: *const ContextDesc) -> Context;
        pub fn sgl_destroy_context(ctx: Context);
        pub fn sgl_set_context(ctx: Context);
        pub fn sgl_get_context() -> Context;
        pub fn sgl_default_context() -> Context;
        pub fn sgl_num_vertices() -> i32;
        pub fn sgl_num_commands() -> i32;
        pub fn sgl_draw();
        pub fn sgl_context_draw(ctx: Context);
        pub fn sgl_draw_layer(layer_id: i32);
        pub fn sgl_context_draw_layer(ctx: Context, layer_id: i32);
        pub fn sgl_make_pipeline(desc: *const sg::PipelineDesc) -> Pipeline;
        pub fn sgl_context_make_pipeline(ctx: Context, desc: *const sg::PipelineDesc) -> Pipeline;
        pub fn sgl_destroy_pipeline(pip: Pipeline);
        pub fn sgl_defaults();
        pub fn sgl_viewport(x: i32, y: i32, w: i32, h: i32, origin_top_left: bool);
        pub fn sgl_viewportf(x: f32, y: f32, w: f32, h: f32, origin_top_left: bool);
        pub fn sgl_scissor_rect(x: i32, y: i32, w: i32, h: i32, origin_top_left: bool);
        pub fn sgl_scissor_rectf(x: f32, y: f32, w: f32, h: f32, origin_top_left: bool);
        pub fn sgl_enable_texture();
        pub fn sgl_disable_texture();
        pub fn sgl_texture(img: sg::Image, smp: sg::Sampler);
        pub fn sgl_layer(layer_id: i32);
        pub fn sgl_load_default_pipeline();
        pub fn sgl_load_pipeline(pip: Pipeline);
        pub fn sgl_push_pipeline();
        pub fn sgl_pop_pipeline();
        pub fn sgl_matrix_mode_modelview();
        pub fn sgl_matrix_mode_projection();
        pub fn sgl_matrix_mode_texture();
        pub fn sgl_load_identity();
        pub fn sgl_load_matrix(m: *const f32);
        pub fn sgl_load_transpose_matrix(m: *const f32);
        pub fn sgl_mult_matrix(m: *const f32);
        pub fn sgl_mult_transpose_matrix(m: *const f32);
        pub fn sgl_rotate(angle_rad: f32, x: f32, y: f32, z: f32);
        pub fn sgl_scale(x: f32, y: f32, z: f32);
        pub fn sgl_translate(x: f32, y: f32, z: f32);
        pub fn sgl_frustum(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32);
        pub fn sgl_ortho(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32);
        pub fn sgl_perspective(fov_y: f32, aspect: f32, z_near: f32, z_far: f32);
        pub fn sgl_lookat(
            eye_x: f32,
            eye_y: f32,
            eye_z: f32,
            center_x: f32,
            center_y: f32,
            center_z: f32,
            up_x: f32,
            up_y: f32,
            up_z: f32,
        );
        pub fn sgl_push_matrix();
        pub fn sgl_pop_matrix();
        pub fn sgl_t2f(u: f32, v: f32);
        pub fn sgl_c3f(r: f32, g: f32, b: f32);
        pub fn sgl_c4f(r: f32, g: f32, b: f32, a: f32);
        pub fn sgl_c3b(r: u8, g: u8, b: u8);
        pub fn sgl_c4b(r: u8, g: u8, b: u8, a: u8);
        pub fn sgl_c1i(rgba: u32);
        pub fn sgl_point_size(s: f32);
        pub fn sgl_begin_points();
        pub fn sgl_begin_lines();
        pub fn sgl_begin_line_strip();
        pub fn sgl_begin_triangles();
        pub fn sgl_begin_triangle_strip();
        pub fn sgl_begin_quads();
        pub fn sgl_v2f(x: f32, y: f32);
        pub fn sgl_v3f(x: f32, y: f32, z: f32);
        pub fn sgl_v2f_t2f(x: f32, y: f32, u: f32, v: f32);
        pub fn sgl_v3f_t2f(x: f32, y: f32, z: f32, u: f32, v: f32);
        pub fn sgl_v2f_c3f(x: f32, y: f32, r: f32, g: f32, b: f32);
        pub fn sgl_v2f_c3b(x: f32, y: f32, r: u8, g: u8, b: u8);
        pub fn sgl_v2f_c4f(x: f32, y: f32, r: f32, g: f32, b: f32, a: f32);
        pub fn sgl_v2f_c4b(x: f32, y: f32, r: u8, g: u8, b: u8, a: u8);
        pub fn sgl_v2f_c1i(x: f32, y: f32, rgba: u32);
        pub fn sgl_v3f_c3f(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32);
        pub fn sgl_v3f_c3b(x: f32, y: f32, z: f32, r: u8, g: u8, b: u8);
        pub fn sgl_v3f_c4f(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32, a: f32);
        pub fn sgl_v3f_c4b(x: f32, y: f32, z: f32, r: u8, g: u8, b: u8, a: u8);
        pub fn sgl_v3f_c1i(x: f32, y: f32, z: f32, rgba: u32);
        pub fn sgl_v2f_t2f_c3f(x: f32, y: f32, u: f32, v: f32, r: f32, g: f32, b: f32);
        pub fn sgl_v2f_t2f_c3b(x: f32, y: f32, u: f32, v: f32, r: u8, g: u8, b: u8);
        pub fn sgl_v2f_t2f_c4f(x: f32, y: f32, u: f32, v: f32, r: f32, g: f32, b: f32, a: f32);
        pub fn sgl_v2f_t2f_c4b(x: f32, y: f32, u: f32, v: f32, r: u8, g: u8, b: u8, a: u8);
        pub fn sgl_v2f_t2f_c1i(x: f32, y: f32, u: f32, v: f32, rgba: u32);
        pub fn sgl_v3f_t2f_c3f(x: f32, y: f32, z: f32, u: f32, v: f32, r: f32, g: f32, b: f32);
        pub fn sgl_v3f_t2f_c3b(x: f32, y: f32, z: f32, u: f32, v: f32, r: u8, g: u8, b: u8);
        pub fn sgl_v3f_t2f_c4f(x: f32, y: f32, z: f32, u: f32, v: f32, r: f32, g: f32, b: f32, a: f32);
        pub fn sgl_v3f_t2f_c4b(x: f32, y: f32, z: f32, u: f32, v: f32, r: u8, g: u8, b: u8, a: u8);
        pub fn sgl_v3f_t2f_c1i(x: f32, y: f32, z: f32, u: f32, v: f32, rgba: u32);
        pub fn sgl_end();
    }
}
#[inline]
pub fn setup(desc: &Desc) {
    unsafe { ffi::sgl_setup(desc) }
}
#[inline]
pub fn shutdown() {
    unsafe { ffi::sgl_shutdown() }
}
#[inline]
pub fn rad(deg: f32) -> f32 {
    unsafe { ffi::sgl_rad(deg) }
}
#[inline]
pub fn deg(rad: f32) -> f32 {
    unsafe { ffi::sgl_deg(rad) }
}
#[inline]
pub fn error() -> Error {
    unsafe { ffi::sgl_error() }
}
#[inline]
pub fn context_error(ctx: Context) -> Error {
    unsafe { ffi::sgl_context_error(ctx) }
}
#[inline]
pub fn make_context(desc: &ContextDesc) -> Context {
    unsafe { ffi::sgl_make_context(desc) }
}
#[inline]
pub fn destroy_context(ctx: Context) {
    unsafe { ffi::sgl_destroy_context(ctx) }
}
#[inline]
pub fn set_context(ctx: Context) {
    unsafe { ffi::sgl_set_context(ctx) }
}
#[inline]
pub fn get_context() -> Context {
    unsafe { ffi::sgl_get_context() }
}
#[inline]
pub fn default_context() -> Context {
    unsafe { ffi::sgl_default_context() }
}
#[inline]
pub fn num_vertices() -> i32 {
    unsafe { ffi::sgl_num_vertices() }
}
#[inline]
pub fn num_commands() -> i32 {
    unsafe { ffi::sgl_num_commands() }
}
#[inline]
pub fn draw() {
    unsafe { ffi::sgl_draw() }
}
#[inline]
pub fn context_draw(ctx: Context) {
    unsafe { ffi::sgl_context_draw(ctx) }
}
#[inline]
pub fn draw_layer(layer_id: i32) {
    unsafe { ffi::sgl_draw_layer(layer_id) }
}
#[inline]
pub fn context_draw_layer(ctx: Context, layer_id: i32) {
    unsafe { ffi::sgl_context_draw_layer(ctx, layer_id) }
}
#[inline]
pub fn make_pipeline(desc: &sg::PipelineDesc) -> Pipeline {
    unsafe { ffi::sgl_make_pipeline(desc) }
}
#[inline]
pub fn context_make_pipeline(ctx: Context, desc: &sg::PipelineDesc) -> Pipeline {
    unsafe { ffi::sgl_context_make_pipeline(ctx, desc) }
}
#[inline]
pub fn destroy_pipeline(pip: Pipeline) {
    unsafe { ffi::sgl_destroy_pipeline(pip) }
}
#[inline]
pub fn defaults() {
    unsafe { ffi::sgl_defaults() }
}
#[inline]
pub fn viewport(x: i32, y: i32, w: i32, h: i32, origin_top_left: bool) {
    unsafe { ffi::sgl_viewport(x, y, w, h, origin_top_left) }
}
#[inline]
pub fn viewportf(x: f32, y: f32, w: f32, h: f32, origin_top_left: bool) {
    unsafe { ffi::sgl_viewportf(x, y, w, h, origin_top_left) }
}
#[inline]
pub fn scissor_rect(x: i32, y: i32, w: i32, h: i32, origin_top_left: bool) {
    unsafe { ffi::sgl_scissor_rect(x, y, w, h, origin_top_left) }
}
#[inline]
pub fn scissor_rectf(x: f32, y: f32, w: f32, h: f32, origin_top_left: bool) {
    unsafe { ffi::sgl_scissor_rectf(x, y, w, h, origin_top_left) }
}
#[inline]
pub fn enable_texture() {
    unsafe { ffi::sgl_enable_texture() }
}
#[inline]
pub fn disable_texture() {
    unsafe { ffi::sgl_disable_texture() }
}
#[inline]
pub fn texture(img: sg::Image, smp: sg::Sampler) {
    unsafe { ffi::sgl_texture(img, smp) }
}
#[inline]
pub fn layer(layer_id: i32) {
    unsafe { ffi::sgl_layer(layer_id) }
}
#[inline]
pub fn load_default_pipeline() {
    unsafe { ffi::sgl_load_default_pipeline() }
}
#[inline]
pub fn load_pipeline(pip: Pipeline) {
    unsafe { ffi::sgl_load_pipeline(pip) }
}
#[inline]
pub fn push_pipeline() {
    unsafe { ffi::sgl_push_pipeline() }
}
#[inline]
pub fn pop_pipeline() {
    unsafe { ffi::sgl_pop_pipeline() }
}
#[inline]
pub fn matrix_mode_modelview() {
    unsafe { ffi::sgl_matrix_mode_modelview() }
}
#[inline]
pub fn matrix_mode_projection() {
    unsafe { ffi::sgl_matrix_mode_projection() }
}
#[inline]
pub fn matrix_mode_texture() {
    unsafe { ffi::sgl_matrix_mode_texture() }
}
#[inline]
pub fn load_identity() {
    unsafe { ffi::sgl_load_identity() }
}
#[inline]
pub fn load_matrix(m: &f32) {
    unsafe { ffi::sgl_load_matrix(m) }
}
#[inline]
pub fn load_transpose_matrix(m: &f32) {
    unsafe { ffi::sgl_load_transpose_matrix(m) }
}
#[inline]
pub fn mult_matrix(m: &f32) {
    unsafe { ffi::sgl_mult_matrix(m) }
}
#[inline]
pub fn mult_transpose_matrix(m: &f32) {
    unsafe { ffi::sgl_mult_transpose_matrix(m) }
}
#[inline]
pub fn rotate(angle_rad: f32, x: f32, y: f32, z: f32) {
    unsafe { ffi::sgl_rotate(angle_rad, x, y, z) }
}
#[inline]
pub fn scale(x: f32, y: f32, z: f32) {
    unsafe { ffi::sgl_scale(x, y, z) }
}
#[inline]
pub fn translate(x: f32, y: f32, z: f32) {
    unsafe { ffi::sgl_translate(x, y, z) }
}
#[inline]
pub fn frustum(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) {
    unsafe { ffi::sgl_frustum(l, r, b, t, n, f) }
}
#[inline]
pub fn ortho(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) {
    unsafe { ffi::sgl_ortho(l, r, b, t, n, f) }
}
#[inline]
pub fn perspective(fov_y: f32, aspect: f32, z_near: f32, z_far: f32) {
    unsafe { ffi::sgl_perspective(fov_y, aspect, z_near, z_far) }
}
#[inline]
pub fn lookat(
    eye_x: f32,
    eye_y: f32,
    eye_z: f32,
    center_x: f32,
    center_y: f32,
    center_z: f32,
    up_x: f32,
    up_y: f32,
    up_z: f32,
) {
    unsafe { ffi::sgl_lookat(eye_x, eye_y, eye_z, center_x, center_y, center_z, up_x, up_y, up_z) }
}
#[inline]
pub fn push_matrix() {
    unsafe { ffi::sgl_push_matrix() }
}
#[inline]
pub fn pop_matrix() {
    unsafe { ffi::sgl_pop_matrix() }
}
#[inline]
pub fn t2f(u: f32, v: f32) {
    unsafe { ffi::sgl_t2f(u, v) }
}
#[inline]
pub fn c3f(r: f32, g: f32, b: f32) {
    unsafe { ffi::sgl_c3f(r, g, b) }
}
#[inline]
pub fn c4f(r: f32, g: f32, b: f32, a: f32) {
    unsafe { ffi::sgl_c4f(r, g, b, a) }
}
#[inline]
pub fn c3b(r: u8, g: u8, b: u8) {
    unsafe { ffi::sgl_c3b(r, g, b) }
}
#[inline]
pub fn c4b(r: u8, g: u8, b: u8, a: u8) {
    unsafe { ffi::sgl_c4b(r, g, b, a) }
}
#[inline]
pub fn c1i(rgba: u32) {
    unsafe { ffi::sgl_c1i(rgba) }
}
#[inline]
pub fn point_size(s: f32) {
    unsafe { ffi::sgl_point_size(s) }
}
#[inline]
pub fn begin_points() {
    unsafe { ffi::sgl_begin_points() }
}
#[inline]
pub fn begin_lines() {
    unsafe { ffi::sgl_begin_lines() }
}
#[inline]
pub fn begin_line_strip() {
    unsafe { ffi::sgl_begin_line_strip() }
}
#[inline]
pub fn begin_triangles() {
    unsafe { ffi::sgl_begin_triangles() }
}
#[inline]
pub fn begin_triangle_strip() {
    unsafe { ffi::sgl_begin_triangle_strip() }
}
#[inline]
pub fn begin_quads() {
    unsafe { ffi::sgl_begin_quads() }
}
#[inline]
pub fn v2f(x: f32, y: f32) {
    unsafe { ffi::sgl_v2f(x, y) }
}
#[inline]
pub fn v3f(x: f32, y: f32, z: f32) {
    unsafe { ffi::sgl_v3f(x, y, z) }
}
#[inline]
pub fn v2f_t2f(x: f32, y: f32, u: f32, v: f32) {
    unsafe { ffi::sgl_v2f_t2f(x, y, u, v) }
}
#[inline]
pub fn v3f_t2f(x: f32, y: f32, z: f32, u: f32, v: f32) {
    unsafe { ffi::sgl_v3f_t2f(x, y, z, u, v) }
}
#[inline]
pub fn v2f_c3f(x: f32, y: f32, r: f32, g: f32, b: f32) {
    unsafe { ffi::sgl_v2f_c3f(x, y, r, g, b) }
}
#[inline]
pub fn v2f_c3b(x: f32, y: f32, r: u8, g: u8, b: u8) {
    unsafe { ffi::sgl_v2f_c3b(x, y, r, g, b) }
}
#[inline]
pub fn v2f_c4f(x: f32, y: f32, r: f32, g: f32, b: f32, a: f32) {
    unsafe { ffi::sgl_v2f_c4f(x, y, r, g, b, a) }
}
#[inline]
pub fn v2f_c4b(x: f32, y: f32, r: u8, g: u8, b: u8, a: u8) {
    unsafe { ffi::sgl_v2f_c4b(x, y, r, g, b, a) }
}
#[inline]
pub fn v2f_c1i(x: f32, y: f32, rgba: u32) {
    unsafe { ffi::sgl_v2f_c1i(x, y, rgba) }
}
#[inline]
pub fn v3f_c3f(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) {
    unsafe { ffi::sgl_v3f_c3f(x, y, z, r, g, b) }
}
#[inline]
pub fn v3f_c3b(x: f32, y: f32, z: f32, r: u8, g: u8, b: u8) {
    unsafe { ffi::sgl_v3f_c3b(x, y, z, r, g, b) }
}
#[inline]
pub fn v3f_c4f(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32, a: f32) {
    unsafe { ffi::sgl_v3f_c4f(x, y, z, r, g, b, a) }
}
#[inline]
pub fn v3f_c4b(x: f32, y: f32, z: f32, r: u8, g: u8, b: u8, a: u8) {
    unsafe { ffi::sgl_v3f_c4b(x, y, z, r, g, b, a) }
}
#[inline]
pub fn v3f_c1i(x: f32, y: f32, z: f32, rgba: u32) {
    unsafe { ffi::sgl_v3f_c1i(x, y, z, rgba) }
}
#[inline]
pub fn v2f_t2f_c3f(x: f32, y: f32, u: f32, v: f32, r: f32, g: f32, b: f32) {
    unsafe { ffi::sgl_v2f_t2f_c3f(x, y, u, v, r, g, b) }
}
#[inline]
pub fn v2f_t2f_c3b(x: f32, y: f32, u: f32, v: f32, r: u8, g: u8, b: u8) {
    unsafe { ffi::sgl_v2f_t2f_c3b(x, y, u, v, r, g, b) }
}
#[inline]
pub fn v2f_t2f_c4f(x: f32, y: f32, u: f32, v: f32, r: f32, g: f32, b: f32, a: f32) {
    unsafe { ffi::sgl_v2f_t2f_c4f(x, y, u, v, r, g, b, a) }
}
#[inline]
pub fn v2f_t2f_c4b(x: f32, y: f32, u: f32, v: f32, r: u8, g: u8, b: u8, a: u8) {
    unsafe { ffi::sgl_v2f_t2f_c4b(x, y, u, v, r, g, b, a) }
}
#[inline]
pub fn v2f_t2f_c1i(x: f32, y: f32, u: f32, v: f32, rgba: u32) {
    unsafe { ffi::sgl_v2f_t2f_c1i(x, y, u, v, rgba) }
}
#[inline]
pub fn v3f_t2f_c3f(x: f32, y: f32, z: f32, u: f32, v: f32, r: f32, g: f32, b: f32) {
    unsafe { ffi::sgl_v3f_t2f_c3f(x, y, z, u, v, r, g, b) }
}
#[inline]
pub fn v3f_t2f_c3b(x: f32, y: f32, z: f32, u: f32, v: f32, r: u8, g: u8, b: u8) {
    unsafe { ffi::sgl_v3f_t2f_c3b(x, y, z, u, v, r, g, b) }
}
#[inline]
pub fn v3f_t2f_c4f(x: f32, y: f32, z: f32, u: f32, v: f32, r: f32, g: f32, b: f32, a: f32) {
    unsafe { ffi::sgl_v3f_t2f_c4f(x, y, z, u, v, r, g, b, a) }
}
#[inline]
pub fn v3f_t2f_c4b(x: f32, y: f32, z: f32, u: f32, v: f32, r: u8, g: u8, b: u8, a: u8) {
    unsafe { ffi::sgl_v3f_t2f_c4b(x, y, z, u, v, r, g, b, a) }
}
#[inline]
pub fn v3f_t2f_c1i(x: f32, y: f32, z: f32, u: f32, v: f32, rgba: u32) {
    unsafe { ffi::sgl_v3f_t2f_c1i(x, y, z, u, v, rgba) }
}
#[inline]
pub fn end() {
    unsafe { ffi::sgl_end() }
}
