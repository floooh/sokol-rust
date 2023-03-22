//! To use this module, enable the feature "imgui"
// machine generated, do not edit

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::gfx as sg;

/// Helper function to convert a C string to a rust string slice
#[inline]
fn c_char_ptr_to_rust_str(c_char_ptr: *const core::ffi::c_char) -> &'static str {
    let c_str = unsafe { core::ffi::CStr::from_ptr(c_char_ptr) };
    c_str.to_str().expect("c_char_ptr contained invalid Utf8 Data")
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiStr {
    pub buf: [core::ffi::c_char; 96],
}
impl ImguiStr {
    pub const fn new() -> Self {
        Self { buf: [0; 96] }
    }
}
impl Default for ImguiStr {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiBuffer {
    pub res_id: sg::Buffer,
    pub label: ImguiStr,
    pub desc: sg::BufferDesc,
}
impl ImguiBuffer {
    pub const fn new() -> Self {
        Self { res_id: sg::Buffer::new(), label: ImguiStr::new(), desc: sg::BufferDesc::new() }
    }
}
impl Default for ImguiBuffer {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiImage {
    pub res_id: sg::Image,
    pub ui_scale: f32,
    pub label: ImguiStr,
    pub desc: sg::ImageDesc,
}
impl ImguiImage {
    pub const fn new() -> Self {
        Self {
            res_id: sg::Image::new(),
            ui_scale: 0.0,
            label: ImguiStr::new(),
            desc: sg::ImageDesc::new(),
        }
    }
}
impl Default for ImguiImage {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiShader {
    pub res_id: sg::Shader,
    pub label: ImguiStr,
    pub vs_entry: ImguiStr,
    pub vs_d3d11_target: ImguiStr,
    pub vs_image_name: [ImguiStr; 12],
    pub vs_uniform_name: [[ImguiStr; 16]; 4],
    pub fs_entry: ImguiStr,
    pub fs_d3d11_target: ImguiStr,
    pub fs_image_name: [ImguiStr; 12],
    pub fs_uniform_name: [[ImguiStr; 16]; 4],
    pub attr_name: [ImguiStr; 16],
    pub attr_sem_name: [ImguiStr; 16],
    pub desc: sg::ShaderDesc,
}
impl ImguiShader {
    pub const fn new() -> Self {
        Self {
            res_id: sg::Shader::new(),
            label: ImguiStr::new(),
            vs_entry: ImguiStr::new(),
            vs_d3d11_target: ImguiStr::new(),
            vs_image_name: [ImguiStr::new(); 12],
            vs_uniform_name: [[ImguiStr::new(); 16]; 4],
            fs_entry: ImguiStr::new(),
            fs_d3d11_target: ImguiStr::new(),
            fs_image_name: [ImguiStr::new(); 12],
            fs_uniform_name: [[ImguiStr::new(); 16]; 4],
            attr_name: [ImguiStr::new(); 16],
            attr_sem_name: [ImguiStr::new(); 16],
            desc: sg::ShaderDesc::new(),
        }
    }
}
impl Default for ImguiShader {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiPipeline {
    pub res_id: sg::Pipeline,
    pub label: ImguiStr,
    pub desc: sg::PipelineDesc,
}
impl ImguiPipeline {
    pub const fn new() -> Self {
        Self {
            res_id: sg::Pipeline::new(),
            label: ImguiStr::new(),
            desc: sg::PipelineDesc::new(),
        }
    }
}
impl Default for ImguiPipeline {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiPass {
    pub res_id: sg::Pass,
    pub label: ImguiStr,
    pub color_image_scale: [f32; 4],
    pub ds_image_scale: f32,
    pub desc: sg::PassDesc,
}
impl ImguiPass {
    pub const fn new() -> Self {
        Self {
            res_id: sg::Pass::new(),
            label: ImguiStr::new(),
            color_image_scale: [0.0; 4],
            ds_image_scale: 0.0,
            desc: sg::PassDesc::new(),
        }
    }
}
impl Default for ImguiPass {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiBuffers {
    pub open: bool,
    pub num_slots: i32,
    pub sel_buf: sg::Buffer,
    pub slots: *mut ImguiBuffer,
}
impl ImguiBuffers {
    pub const fn new() -> Self {
        Self {
            open: false,
            num_slots: 0,
            sel_buf: sg::Buffer::new(),
            slots: core::ptr::null_mut(),
        }
    }
}
impl Default for ImguiBuffers {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiImages {
    pub open: bool,
    pub num_slots: i32,
    pub sel_img: sg::Image,
    pub slots: *mut ImguiImage,
}
impl ImguiImages {
    pub const fn new() -> Self {
        Self {
            open: false,
            num_slots: 0,
            sel_img: sg::Image::new(),
            slots: core::ptr::null_mut(),
        }
    }
}
impl Default for ImguiImages {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiShaders {
    pub open: bool,
    pub num_slots: i32,
    pub sel_shd: sg::Shader,
    pub slots: *mut ImguiShader,
}
impl ImguiShaders {
    pub const fn new() -> Self {
        Self {
            open: false,
            num_slots: 0,
            sel_shd: sg::Shader::new(),
            slots: core::ptr::null_mut(),
        }
    }
}
impl Default for ImguiShaders {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiPipelines {
    pub open: bool,
    pub num_slots: i32,
    pub sel_pip: sg::Pipeline,
    pub slots: *mut ImguiPipeline,
}
impl ImguiPipelines {
    pub const fn new() -> Self {
        Self {
            open: false,
            num_slots: 0,
            sel_pip: sg::Pipeline::new(),
            slots: core::ptr::null_mut(),
        }
    }
}
impl Default for ImguiPipelines {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiPasses {
    pub open: bool,
    pub num_slots: i32,
    pub sel_pass: sg::Pass,
    pub slots: *mut ImguiPass,
}
impl ImguiPasses {
    pub const fn new() -> Self {
        Self {
            open: false,
            num_slots: 0,
            sel_pass: sg::Pass::new(),
            slots: core::ptr::null_mut(),
        }
    }
}
impl Default for ImguiPasses {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum ImguiCmd {
    CmdInvalid,
    CmdResetStateCache,
    CmdMakeBuffer,
    CmdMakeImage,
    CmdMakeShader,
    CmdMakePipeline,
    CmdMakePass,
    CmdDestroyBuffer,
    CmdDestroyImage,
    CmdDestroyShader,
    CmdDestroyPipeline,
    CmdDestroyPass,
    CmdUpdateBuffer,
    CmdUpdateImage,
    CmdAppendBuffer,
    CmdBeginDefaultPass,
    CmdBeginPass,
    CmdApplyViewport,
    CmdApplyScissorRect,
    CmdApplyPipeline,
    CmdApplyBindings,
    CmdApplyUniforms,
    CmdDraw,
    CmdEndPass,
    CmdCommit,
    CmdAllocBuffer,
    CmdAllocImage,
    CmdAllocShader,
    CmdAllocPipeline,
    CmdAllocPass,
    CmdDeallocBuffer,
    CmdDeallocImage,
    CmdDeallocShader,
    CmdDeallocPipeline,
    CmdDeallocPass,
    CmdInitBuffer,
    CmdInitImage,
    CmdInitShader,
    CmdInitPipeline,
    CmdInitPass,
    CmdUninitBuffer,
    CmdUninitImage,
    CmdUninitShader,
    CmdUninitPipeline,
    CmdUninitPass,
    CmdFailBuffer,
    CmdFailImage,
    CmdFailShader,
    CmdFailPipeline,
    CmdFailPass,
    CmdPushDebugGroup,
    CmdPopDebugGroup,
    CmdErrBufferPoolExhausted,
    CmdErrImagePoolExhausted,
    CmdErrShaderPoolExhausted,
    CmdErrPipelinePoolExhausted,
    CmdErrPassPoolExhausted,
    CmdErrContextMismatch,
    CmdErrPassInvalid,
    CmdErrDrawInvalid,
    CmdErrBindingsInvalid,
}
impl ImguiCmd {
    pub const fn new() -> Self {
        Self::CmdInvalid
    }
}
impl Default for ImguiCmd {
    fn default() -> Self {
        Self::CmdInvalid
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsMakeBuffer {
    pub result: sg::Buffer,
}
impl ImguiArgsMakeBuffer {
    pub const fn new() -> Self {
        Self { result: sg::Buffer::new() }
    }
}
impl Default for ImguiArgsMakeBuffer {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsMakeImage {
    pub result: sg::Image,
}
impl ImguiArgsMakeImage {
    pub const fn new() -> Self {
        Self { result: sg::Image::new() }
    }
}
impl Default for ImguiArgsMakeImage {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsMakeShader {
    pub result: sg::Shader,
}
impl ImguiArgsMakeShader {
    pub const fn new() -> Self {
        Self { result: sg::Shader::new() }
    }
}
impl Default for ImguiArgsMakeShader {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsMakePipeline {
    pub result: sg::Pipeline,
}
impl ImguiArgsMakePipeline {
    pub const fn new() -> Self {
        Self { result: sg::Pipeline::new() }
    }
}
impl Default for ImguiArgsMakePipeline {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsMakePass {
    pub result: sg::Pass,
}
impl ImguiArgsMakePass {
    pub const fn new() -> Self {
        Self { result: sg::Pass::new() }
    }
}
impl Default for ImguiArgsMakePass {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsDestroyBuffer {
    pub buffer: sg::Buffer,
}
impl ImguiArgsDestroyBuffer {
    pub const fn new() -> Self {
        Self { buffer: sg::Buffer::new() }
    }
}
impl Default for ImguiArgsDestroyBuffer {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsDestroyImage {
    pub image: sg::Image,
}
impl ImguiArgsDestroyImage {
    pub const fn new() -> Self {
        Self { image: sg::Image::new() }
    }
}
impl Default for ImguiArgsDestroyImage {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsDestroyShader {
    pub shader: sg::Shader,
}
impl ImguiArgsDestroyShader {
    pub const fn new() -> Self {
        Self { shader: sg::Shader::new() }
    }
}
impl Default for ImguiArgsDestroyShader {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsDestroyPipeline {
    pub pipeline: sg::Pipeline,
}
impl ImguiArgsDestroyPipeline {
    pub const fn new() -> Self {
        Self { pipeline: sg::Pipeline::new() }
    }
}
impl Default for ImguiArgsDestroyPipeline {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsDestroyPass {
    pub pass: sg::Pass,
}
impl ImguiArgsDestroyPass {
    pub const fn new() -> Self {
        Self { pass: sg::Pass::new() }
    }
}
impl Default for ImguiArgsDestroyPass {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsUpdateBuffer {
    pub buffer: sg::Buffer,
    pub data_size: usize,
}
impl ImguiArgsUpdateBuffer {
    pub const fn new() -> Self {
        Self { buffer: sg::Buffer::new(), data_size: 0 }
    }
}
impl Default for ImguiArgsUpdateBuffer {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsUpdateImage {
    pub image: sg::Image,
}
impl ImguiArgsUpdateImage {
    pub const fn new() -> Self {
        Self { image: sg::Image::new() }
    }
}
impl Default for ImguiArgsUpdateImage {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsAppendBuffer {
    pub buffer: sg::Buffer,
    pub data_size: usize,
    pub result: i32,
}
impl ImguiArgsAppendBuffer {
    pub const fn new() -> Self {
        Self { buffer: sg::Buffer::new(), data_size: 0, result: 0 }
    }
}
impl Default for ImguiArgsAppendBuffer {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsBeginDefaultPass {
    pub action: sg::PassAction,
    pub width: i32,
    pub height: i32,
}
impl ImguiArgsBeginDefaultPass {
    pub const fn new() -> Self {
        Self { action: sg::PassAction::new(), width: 0, height: 0 }
    }
}
impl Default for ImguiArgsBeginDefaultPass {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsBeginPass {
    pub pass: sg::Pass,
    pub action: sg::PassAction,
}
impl ImguiArgsBeginPass {
    pub const fn new() -> Self {
        Self { pass: sg::Pass::new(), action: sg::PassAction::new() }
    }
}
impl Default for ImguiArgsBeginPass {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsApplyViewport {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub origin_top_left: bool,
}
impl ImguiArgsApplyViewport {
    pub const fn new() -> Self {
        Self { x: 0, y: 0, width: 0, height: 0, origin_top_left: false }
    }
}
impl Default for ImguiArgsApplyViewport {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsApplyScissorRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub origin_top_left: bool,
}
impl ImguiArgsApplyScissorRect {
    pub const fn new() -> Self {
        Self { x: 0, y: 0, width: 0, height: 0, origin_top_left: false }
    }
}
impl Default for ImguiArgsApplyScissorRect {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsApplyPipeline {
    pub pipeline: sg::Pipeline,
}
impl ImguiArgsApplyPipeline {
    pub const fn new() -> Self {
        Self { pipeline: sg::Pipeline::new() }
    }
}
impl Default for ImguiArgsApplyPipeline {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsApplyBindings {
    pub bindings: sg::Bindings,
}
impl ImguiArgsApplyBindings {
    pub const fn new() -> Self {
        Self { bindings: sg::Bindings::new() }
    }
}
impl Default for ImguiArgsApplyBindings {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsApplyUniforms {
    pub stage: sg::ShaderStage,
    pub ub_index: i32,
    pub data_size: usize,
    pub pipeline: sg::Pipeline,
    pub ubuf_pos: usize,
}
impl ImguiArgsApplyUniforms {
    pub const fn new() -> Self {
        Self {
            stage: sg::ShaderStage::new(),
            ub_index: 0,
            data_size: 0,
            pipeline: sg::Pipeline::new(),
            ubuf_pos: 0,
        }
    }
}
impl Default for ImguiArgsApplyUniforms {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsDraw {
    pub base_element: i32,
    pub num_elements: i32,
    pub num_instances: i32,
}
impl ImguiArgsDraw {
    pub const fn new() -> Self {
        Self { base_element: 0, num_elements: 0, num_instances: 0 }
    }
}
impl Default for ImguiArgsDraw {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsAllocBuffer {
    pub result: sg::Buffer,
}
impl ImguiArgsAllocBuffer {
    pub const fn new() -> Self {
        Self { result: sg::Buffer::new() }
    }
}
impl Default for ImguiArgsAllocBuffer {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsAllocImage {
    pub result: sg::Image,
}
impl ImguiArgsAllocImage {
    pub const fn new() -> Self {
        Self { result: sg::Image::new() }
    }
}
impl Default for ImguiArgsAllocImage {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsAllocShader {
    pub result: sg::Shader,
}
impl ImguiArgsAllocShader {
    pub const fn new() -> Self {
        Self { result: sg::Shader::new() }
    }
}
impl Default for ImguiArgsAllocShader {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsAllocPipeline {
    pub result: sg::Pipeline,
}
impl ImguiArgsAllocPipeline {
    pub const fn new() -> Self {
        Self { result: sg::Pipeline::new() }
    }
}
impl Default for ImguiArgsAllocPipeline {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsAllocPass {
    pub result: sg::Pass,
}
impl ImguiArgsAllocPass {
    pub const fn new() -> Self {
        Self { result: sg::Pass::new() }
    }
}
impl Default for ImguiArgsAllocPass {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsDeallocBuffer {
    pub buffer: sg::Buffer,
}
impl ImguiArgsDeallocBuffer {
    pub const fn new() -> Self {
        Self { buffer: sg::Buffer::new() }
    }
}
impl Default for ImguiArgsDeallocBuffer {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsDeallocImage {
    pub image: sg::Image,
}
impl ImguiArgsDeallocImage {
    pub const fn new() -> Self {
        Self { image: sg::Image::new() }
    }
}
impl Default for ImguiArgsDeallocImage {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsDeallocShader {
    pub shader: sg::Shader,
}
impl ImguiArgsDeallocShader {
    pub const fn new() -> Self {
        Self { shader: sg::Shader::new() }
    }
}
impl Default for ImguiArgsDeallocShader {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsDeallocPipeline {
    pub pipeline: sg::Pipeline,
}
impl ImguiArgsDeallocPipeline {
    pub const fn new() -> Self {
        Self { pipeline: sg::Pipeline::new() }
    }
}
impl Default for ImguiArgsDeallocPipeline {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsDeallocPass {
    pub pass: sg::Pass,
}
impl ImguiArgsDeallocPass {
    pub const fn new() -> Self {
        Self { pass: sg::Pass::new() }
    }
}
impl Default for ImguiArgsDeallocPass {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsInitBuffer {
    pub buffer: sg::Buffer,
}
impl ImguiArgsInitBuffer {
    pub const fn new() -> Self {
        Self { buffer: sg::Buffer::new() }
    }
}
impl Default for ImguiArgsInitBuffer {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsInitImage {
    pub image: sg::Image,
}
impl ImguiArgsInitImage {
    pub const fn new() -> Self {
        Self { image: sg::Image::new() }
    }
}
impl Default for ImguiArgsInitImage {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsInitShader {
    pub shader: sg::Shader,
}
impl ImguiArgsInitShader {
    pub const fn new() -> Self {
        Self { shader: sg::Shader::new() }
    }
}
impl Default for ImguiArgsInitShader {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsInitPipeline {
    pub pipeline: sg::Pipeline,
}
impl ImguiArgsInitPipeline {
    pub const fn new() -> Self {
        Self { pipeline: sg::Pipeline::new() }
    }
}
impl Default for ImguiArgsInitPipeline {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsInitPass {
    pub pass: sg::Pass,
}
impl ImguiArgsInitPass {
    pub const fn new() -> Self {
        Self { pass: sg::Pass::new() }
    }
}
impl Default for ImguiArgsInitPass {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsUninitBuffer {
    pub buffer: sg::Buffer,
}
impl ImguiArgsUninitBuffer {
    pub const fn new() -> Self {
        Self { buffer: sg::Buffer::new() }
    }
}
impl Default for ImguiArgsUninitBuffer {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsUninitImage {
    pub image: sg::Image,
}
impl ImguiArgsUninitImage {
    pub const fn new() -> Self {
        Self { image: sg::Image::new() }
    }
}
impl Default for ImguiArgsUninitImage {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsUninitShader {
    pub shader: sg::Shader,
}
impl ImguiArgsUninitShader {
    pub const fn new() -> Self {
        Self { shader: sg::Shader::new() }
    }
}
impl Default for ImguiArgsUninitShader {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsUninitPipeline {
    pub pipeline: sg::Pipeline,
}
impl ImguiArgsUninitPipeline {
    pub const fn new() -> Self {
        Self { pipeline: sg::Pipeline::new() }
    }
}
impl Default for ImguiArgsUninitPipeline {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsUninitPass {
    pub pass: sg::Pass,
}
impl ImguiArgsUninitPass {
    pub const fn new() -> Self {
        Self { pass: sg::Pass::new() }
    }
}
impl Default for ImguiArgsUninitPass {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsFailBuffer {
    pub buffer: sg::Buffer,
}
impl ImguiArgsFailBuffer {
    pub const fn new() -> Self {
        Self { buffer: sg::Buffer::new() }
    }
}
impl Default for ImguiArgsFailBuffer {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsFailImage {
    pub image: sg::Image,
}
impl ImguiArgsFailImage {
    pub const fn new() -> Self {
        Self { image: sg::Image::new() }
    }
}
impl Default for ImguiArgsFailImage {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsFailShader {
    pub shader: sg::Shader,
}
impl ImguiArgsFailShader {
    pub const fn new() -> Self {
        Self { shader: sg::Shader::new() }
    }
}
impl Default for ImguiArgsFailShader {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsFailPipeline {
    pub pipeline: sg::Pipeline,
}
impl ImguiArgsFailPipeline {
    pub const fn new() -> Self {
        Self { pipeline: sg::Pipeline::new() }
    }
}
impl Default for ImguiArgsFailPipeline {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsFailPass {
    pub pass: sg::Pass,
}
impl ImguiArgsFailPass {
    pub const fn new() -> Self {
        Self { pass: sg::Pass::new() }
    }
}
impl Default for ImguiArgsFailPass {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgsPushDebugGroup {
    pub name: ImguiStr,
}
impl ImguiArgsPushDebugGroup {
    pub const fn new() -> Self {
        Self { name: ImguiStr::new() }
    }
}
impl Default for ImguiArgsPushDebugGroup {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiArgs {
    pub make_buffer: ImguiArgsMakeBuffer,
    pub make_image: ImguiArgsMakeImage,
    pub make_shader: ImguiArgsMakeShader,
    pub make_pipeline: ImguiArgsMakePipeline,
    pub make_pass: ImguiArgsMakePass,
    pub destroy_buffer: ImguiArgsDestroyBuffer,
    pub destroy_image: ImguiArgsDestroyImage,
    pub destroy_shader: ImguiArgsDestroyShader,
    pub destroy_pipeline: ImguiArgsDestroyPipeline,
    pub destroy_pass: ImguiArgsDestroyPass,
    pub update_buffer: ImguiArgsUpdateBuffer,
    pub update_image: ImguiArgsUpdateImage,
    pub append_buffer: ImguiArgsAppendBuffer,
    pub begin_default_pass: ImguiArgsBeginDefaultPass,
    pub begin_pass: ImguiArgsBeginPass,
    pub apply_viewport: ImguiArgsApplyViewport,
    pub apply_scissor_rect: ImguiArgsApplyScissorRect,
    pub apply_pipeline: ImguiArgsApplyPipeline,
    pub apply_bindings: ImguiArgsApplyBindings,
    pub apply_uniforms: ImguiArgsApplyUniforms,
    pub draw: ImguiArgsDraw,
    pub alloc_buffer: ImguiArgsAllocBuffer,
    pub alloc_image: ImguiArgsAllocImage,
    pub alloc_shader: ImguiArgsAllocShader,
    pub alloc_pipeline: ImguiArgsAllocPipeline,
    pub alloc_pass: ImguiArgsAllocPass,
    pub dealloc_buffer: ImguiArgsDeallocBuffer,
    pub dealloc_image: ImguiArgsDeallocImage,
    pub dealloc_shader: ImguiArgsDeallocShader,
    pub dealloc_pipeline: ImguiArgsDeallocPipeline,
    pub dealloc_pass: ImguiArgsDeallocPass,
    pub init_buffer: ImguiArgsInitBuffer,
    pub init_image: ImguiArgsInitImage,
    pub init_shader: ImguiArgsInitShader,
    pub init_pipeline: ImguiArgsInitPipeline,
    pub init_pass: ImguiArgsInitPass,
    pub uninit_buffer: ImguiArgsUninitBuffer,
    pub uninit_image: ImguiArgsUninitImage,
    pub uninit_shader: ImguiArgsUninitShader,
    pub uninit_pipeline: ImguiArgsUninitPipeline,
    pub uninit_pass: ImguiArgsUninitPass,
    pub fail_buffer: ImguiArgsFailBuffer,
    pub fail_image: ImguiArgsFailImage,
    pub fail_shader: ImguiArgsFailShader,
    pub fail_pipeline: ImguiArgsFailPipeline,
    pub fail_pass: ImguiArgsFailPass,
    pub push_debug_group: ImguiArgsPushDebugGroup,
}
impl ImguiArgs {
    pub const fn new() -> Self {
        Self {
            make_buffer: ImguiArgsMakeBuffer::new(),
            make_image: ImguiArgsMakeImage::new(),
            make_shader: ImguiArgsMakeShader::new(),
            make_pipeline: ImguiArgsMakePipeline::new(),
            make_pass: ImguiArgsMakePass::new(),
            destroy_buffer: ImguiArgsDestroyBuffer::new(),
            destroy_image: ImguiArgsDestroyImage::new(),
            destroy_shader: ImguiArgsDestroyShader::new(),
            destroy_pipeline: ImguiArgsDestroyPipeline::new(),
            destroy_pass: ImguiArgsDestroyPass::new(),
            update_buffer: ImguiArgsUpdateBuffer::new(),
            update_image: ImguiArgsUpdateImage::new(),
            append_buffer: ImguiArgsAppendBuffer::new(),
            begin_default_pass: ImguiArgsBeginDefaultPass::new(),
            begin_pass: ImguiArgsBeginPass::new(),
            apply_viewport: ImguiArgsApplyViewport::new(),
            apply_scissor_rect: ImguiArgsApplyScissorRect::new(),
            apply_pipeline: ImguiArgsApplyPipeline::new(),
            apply_bindings: ImguiArgsApplyBindings::new(),
            apply_uniforms: ImguiArgsApplyUniforms::new(),
            draw: ImguiArgsDraw::new(),
            alloc_buffer: ImguiArgsAllocBuffer::new(),
            alloc_image: ImguiArgsAllocImage::new(),
            alloc_shader: ImguiArgsAllocShader::new(),
            alloc_pipeline: ImguiArgsAllocPipeline::new(),
            alloc_pass: ImguiArgsAllocPass::new(),
            dealloc_buffer: ImguiArgsDeallocBuffer::new(),
            dealloc_image: ImguiArgsDeallocImage::new(),
            dealloc_shader: ImguiArgsDeallocShader::new(),
            dealloc_pipeline: ImguiArgsDeallocPipeline::new(),
            dealloc_pass: ImguiArgsDeallocPass::new(),
            init_buffer: ImguiArgsInitBuffer::new(),
            init_image: ImguiArgsInitImage::new(),
            init_shader: ImguiArgsInitShader::new(),
            init_pipeline: ImguiArgsInitPipeline::new(),
            init_pass: ImguiArgsInitPass::new(),
            uninit_buffer: ImguiArgsUninitBuffer::new(),
            uninit_image: ImguiArgsUninitImage::new(),
            uninit_shader: ImguiArgsUninitShader::new(),
            uninit_pipeline: ImguiArgsUninitPipeline::new(),
            uninit_pass: ImguiArgsUninitPass::new(),
            fail_buffer: ImguiArgsFailBuffer::new(),
            fail_image: ImguiArgsFailImage::new(),
            fail_shader: ImguiArgsFailShader::new(),
            fail_pipeline: ImguiArgsFailPipeline::new(),
            fail_pass: ImguiArgsFailPass::new(),
            push_debug_group: ImguiArgsPushDebugGroup::new(),
        }
    }
}
impl Default for ImguiArgs {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiCaptureItem {
    pub cmd: ImguiCmd,
    pub color: u32,
    pub args: ImguiArgs,
}
impl ImguiCaptureItem {
    pub const fn new() -> Self {
        Self { cmd: ImguiCmd::new(), color: 0, args: ImguiArgs::new() }
    }
}
impl Default for ImguiCaptureItem {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiCaptureBucket {
    pub ubuf_size: usize,
    pub ubuf_pos: usize,
    pub ubuf: *mut u8,
    pub num_items: i32,
    pub items: [ImguiCaptureItem; 4096],
}
impl ImguiCaptureBucket {
    pub const fn new() -> Self {
        Self {
            ubuf_size: 0,
            ubuf_pos: 0,
            ubuf: core::ptr::null_mut(),
            num_items: 0,
            items: [ImguiCaptureItem::new(); 4096],
        }
    }
}
impl Default for ImguiCaptureBucket {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiCapture {
    pub open: bool,
    pub bucket_index: i32,
    pub sel_item: i32,
    pub bucket: [ImguiCaptureBucket; 2],
}
impl ImguiCapture {
    pub const fn new() -> Self {
        Self {
            open: false,
            bucket_index: 0,
            sel_item: 0,
            bucket: [ImguiCaptureBucket::new(); 2],
        }
    }
}
impl Default for ImguiCapture {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiCaps {
    pub open: bool,
}
impl ImguiCaps {
    pub const fn new() -> Self {
        Self { open: false }
    }
}
impl Default for ImguiCaps {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiAllocator {
    pub alloc: Option<extern "C" fn(usize, *mut core::ffi::c_void) -> *mut core::ffi::c_void>,
    pub free: Option<extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void)>,
    pub user_data: *mut core::ffi::c_void,
}
impl ImguiAllocator {
    pub const fn new() -> Self {
        Self { alloc: None, free: None, user_data: core::ptr::null_mut() }
    }
}
impl Default for ImguiAllocator {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImguiDesc {
    pub allocator: ImguiAllocator,
}
impl ImguiDesc {
    pub const fn new() -> Self {
        Self { allocator: ImguiAllocator::new() }
    }
}
impl Default for ImguiDesc {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Imgui {
    pub init_tag: u32,
    pub desc: ImguiDesc,
    pub buffers: ImguiBuffers,
    pub images: ImguiImages,
    pub shaders: ImguiShaders,
    pub pipelines: ImguiPipelines,
    pub passes: ImguiPasses,
    pub capture: ImguiCapture,
    pub caps: ImguiCaps,
    pub cur_pipeline: sg::Pipeline,
    pub hooks: sg::TraceHooks,
}
impl Imgui {
    pub const fn new() -> Self {
        Self {
            init_tag: 0,
            desc: ImguiDesc::new(),
            buffers: ImguiBuffers::new(),
            images: ImguiImages::new(),
            shaders: ImguiShaders::new(),
            pipelines: ImguiPipelines::new(),
            passes: ImguiPasses::new(),
            capture: ImguiCapture::new(),
            caps: ImguiCaps::new(),
            cur_pipeline: sg::Pipeline::new(),
            hooks: sg::TraceHooks::new(),
        }
    }
}
impl Default for Imgui {
    fn default() -> Self {
        Self::new()
    }
}
pub mod ffi {
    #![allow(unused_imports)]
    use super::*;
    extern "C" {
        pub fn sg_imgui_init(ctx: *mut Imgui, desc: *const ImguiDesc);
        pub fn sg_imgui_discard(ctx: *mut Imgui);
        pub fn sg_imgui_draw(ctx: *mut Imgui);
        pub fn sg_imgui_draw_buffers_content(ctx: *mut Imgui);
        pub fn sg_imgui_draw_images_content(ctx: *mut Imgui);
        pub fn sg_imgui_draw_shaders_content(ctx: *mut Imgui);
        pub fn sg_imgui_draw_pipelines_content(ctx: *mut Imgui);
        pub fn sg_imgui_draw_passes_content(ctx: *mut Imgui);
        pub fn sg_imgui_draw_capture_content(ctx: *mut Imgui);
        pub fn sg_imgui_draw_capabilities_content(ctx: *mut Imgui);
        pub fn sg_imgui_draw_buffers_window(ctx: *mut Imgui);
        pub fn sg_imgui_draw_images_window(ctx: *mut Imgui);
        pub fn sg_imgui_draw_shaders_window(ctx: *mut Imgui);
        pub fn sg_imgui_draw_pipelines_window(ctx: *mut Imgui);
        pub fn sg_imgui_draw_passes_window(ctx: *mut Imgui);
        pub fn sg_imgui_draw_capture_window(ctx: *mut Imgui);
        pub fn sg_imgui_draw_capabilities_window(ctx: *mut Imgui);
    }
}
#[inline]
pub fn init(ctx: &mut Imgui, desc: &ImguiDesc) {
    unsafe { ffi::sg_imgui_init(ctx, desc) }
}
#[inline]
pub fn discard(ctx: &mut Imgui) {
    unsafe { ffi::sg_imgui_discard(ctx) }
}
#[inline]
pub fn draw(ctx: &mut Imgui) {
    unsafe { ffi::sg_imgui_draw(ctx) }
}
#[inline]
pub fn draw_buffers_content(ctx: &mut Imgui) {
    unsafe { ffi::sg_imgui_draw_buffers_content(ctx) }
}
#[inline]
pub fn draw_images_content(ctx: &mut Imgui) {
    unsafe { ffi::sg_imgui_draw_images_content(ctx) }
}
#[inline]
pub fn draw_shaders_content(ctx: &mut Imgui) {
    unsafe { ffi::sg_imgui_draw_shaders_content(ctx) }
}
#[inline]
pub fn draw_pipelines_content(ctx: &mut Imgui) {
    unsafe { ffi::sg_imgui_draw_pipelines_content(ctx) }
}
#[inline]
pub fn draw_passes_content(ctx: &mut Imgui) {
    unsafe { ffi::sg_imgui_draw_passes_content(ctx) }
}
#[inline]
pub fn draw_capture_content(ctx: &mut Imgui) {
    unsafe { ffi::sg_imgui_draw_capture_content(ctx) }
}
#[inline]
pub fn draw_capabilities_content(ctx: &mut Imgui) {
    unsafe { ffi::sg_imgui_draw_capabilities_content(ctx) }
}
#[inline]
pub fn draw_buffers_window(ctx: &mut Imgui) {
    unsafe { ffi::sg_imgui_draw_buffers_window(ctx) }
}
#[inline]
pub fn draw_images_window(ctx: &mut Imgui) {
    unsafe { ffi::sg_imgui_draw_images_window(ctx) }
}
#[inline]
pub fn draw_shaders_window(ctx: &mut Imgui) {
    unsafe { ffi::sg_imgui_draw_shaders_window(ctx) }
}
#[inline]
pub fn draw_pipelines_window(ctx: &mut Imgui) {
    unsafe { ffi::sg_imgui_draw_pipelines_window(ctx) }
}
#[inline]
pub fn draw_passes_window(ctx: &mut Imgui) {
    unsafe { ffi::sg_imgui_draw_passes_window(ctx) }
}
#[inline]
pub fn draw_capture_window(ctx: &mut Imgui) {
    unsafe { ffi::sg_imgui_draw_capture_window(ctx) }
}
#[inline]
pub fn draw_capabilities_window(ctx: &mut Imgui) {
    unsafe { ffi::sg_imgui_draw_capabilities_window(ctx) }
}
