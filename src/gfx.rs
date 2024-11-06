// machine generated, do not edit

#![allow(dead_code)]
#![allow(unused_imports)]

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
pub struct Buffer {
    pub id: u32,
}
impl Buffer {
    pub const fn new() -> Self {
        Self { id: 0 }
    }
}
impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}
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
pub struct Sampler {
    pub id: u32,
}
impl Sampler {
    pub const fn new() -> Self {
        Self { id: 0 }
    }
}
impl Default for Sampler {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Shader {
    pub id: u32,
}
impl Shader {
    pub const fn new() -> Self {
        Self { id: 0 }
    }
}
impl Default for Shader {
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
pub struct Attachments {
    pub id: u32,
}
impl Attachments {
    pub const fn new() -> Self {
        Self { id: 0 }
    }
}
impl Default for Attachments {
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
pub const INVALID_ID: u32 = 0;
pub const NUM_INFLIGHT_FRAMES: usize = 2;
pub const MAX_COLOR_ATTACHMENTS: usize = 4;
pub const MAX_UNIFORMBLOCK_MEMBERS: usize = 16;
pub const MAX_VERTEX_ATTRIBUTES: usize = 16;
pub const MAX_MIPMAPS: usize = 16;
pub const MAX_TEXTUREARRAY_LAYERS: usize = 128;
pub const MAX_UNIFORMBLOCK_BINDSLOTS: usize = 8;
pub const MAX_VERTEXBUFFER_BINDSLOTS: usize = 8;
pub const MAX_IMAGE_BINDSLOTS: usize = 16;
pub const MAX_SAMPLER_BINDSLOTS: usize = 16;
pub const MAX_STORAGEBUFFER_BINDSLOTS: usize = 8;
pub const MAX_IMAGE_SAMPLER_PAIRS: usize = 16;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl Color {
    pub const fn new() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
    }
}
impl Default for Color {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum Backend {
    Glcore,
    Gles3,
    D3d11,
    MetalIos,
    MetalMacos,
    MetalSimulator,
    Wgpu,
    Dummy,
}
impl Backend {
    pub const fn new() -> Self {
        Self::Glcore
    }
}
impl Default for Backend {
    fn default() -> Self {
        Self::Glcore
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum PixelFormat {
    Default,
    None,
    R8,
    R8sn,
    R8ui,
    R8si,
    R16,
    R16sn,
    R16ui,
    R16si,
    R16f,
    Rg8,
    Rg8sn,
    Rg8ui,
    Rg8si,
    R32ui,
    R32si,
    R32f,
    Rg16,
    Rg16sn,
    Rg16ui,
    Rg16si,
    Rg16f,
    Rgba8,
    Srgb8a8,
    Rgba8sn,
    Rgba8ui,
    Rgba8si,
    Bgra8,
    Rgb10a2,
    Rg11b10f,
    Rgb9e5,
    Rg32ui,
    Rg32si,
    Rg32f,
    Rgba16,
    Rgba16sn,
    Rgba16ui,
    Rgba16si,
    Rgba16f,
    Rgba32ui,
    Rgba32si,
    Rgba32f,
    Depth,
    DepthStencil,
    Bc1Rgba,
    Bc2Rgba,
    Bc3Rgba,
    Bc3Srgba,
    Bc4R,
    Bc4Rsn,
    Bc5Rg,
    Bc5Rgsn,
    Bc6hRgbf,
    Bc6hRgbuf,
    Bc7Rgba,
    Bc7Srgba,
    PvrtcRgb2bpp,
    PvrtcRgb4bpp,
    PvrtcRgba2bpp,
    PvrtcRgba4bpp,
    Etc2Rgb8,
    Etc2Srgb8,
    Etc2Rgb8a1,
    Etc2Rgba8,
    Etc2Srgb8a8,
    EacR11,
    EacR11sn,
    EacRg11,
    EacRg11sn,
    Astc4x4Rgba,
    Astc4x4Srgba,
    Num,
}
impl PixelFormat {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for PixelFormat {
    fn default() -> Self {
        Self::Default
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PixelformatInfo {
    pub sample: bool,
    pub filter: bool,
    pub render: bool,
    pub blend: bool,
    pub msaa: bool,
    pub depth: bool,
    pub compressed: bool,
    pub bytes_per_pixel: i32,
}
impl PixelformatInfo {
    pub const fn new() -> Self {
        Self {
            sample: false,
            filter: false,
            render: false,
            blend: false,
            msaa: false,
            depth: false,
            compressed: false,
            bytes_per_pixel: 0,
        }
    }
}
impl Default for PixelformatInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Features {
    pub origin_top_left: bool,
    pub image_clamp_to_border: bool,
    pub mrt_independent_blend_state: bool,
    pub mrt_independent_write_mask: bool,
    pub storage_buffer: bool,
}
impl Features {
    pub const fn new() -> Self {
        Self {
            origin_top_left: false,
            image_clamp_to_border: false,
            mrt_independent_blend_state: false,
            mrt_independent_write_mask: false,
            storage_buffer: false,
        }
    }
}
impl Default for Features {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Limits {
    pub max_image_size_2d: i32,
    pub max_image_size_cube: i32,
    pub max_image_size_3d: i32,
    pub max_image_size_array: i32,
    pub max_image_array_layers: i32,
    pub max_vertex_attrs: i32,
    pub gl_max_vertex_uniform_components: i32,
    pub gl_max_combined_texture_image_units: i32,
}
impl Limits {
    pub const fn new() -> Self {
        Self {
            max_image_size_2d: 0,
            max_image_size_cube: 0,
            max_image_size_3d: 0,
            max_image_size_array: 0,
            max_image_array_layers: 0,
            max_vertex_attrs: 0,
            gl_max_vertex_uniform_components: 0,
            gl_max_combined_texture_image_units: 0,
        }
    }
}
impl Default for Limits {
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
pub enum Usage {
    Default,
    Immutable,
    Dynamic,
    Stream,
    Num,
}
impl Usage {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for Usage {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum BufferType {
    Default,
    Vertexbuffer,
    Indexbuffer,
    Storagebuffer,
    Num,
}
impl BufferType {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for BufferType {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum IndexType {
    Default,
    None,
    Uint16,
    Uint32,
    Num,
}
impl IndexType {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for IndexType {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum ImageType {
    Default,
    Dim2,
    Cube,
    Dim3,
    Array,
    Num,
}
impl ImageType {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for ImageType {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum ImageSampleType {
    Default,
    Float,
    Depth,
    Sint,
    Uint,
    UnfilterableFloat,
    Num,
}
impl ImageSampleType {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for ImageSampleType {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum SamplerType {
    Default,
    Filtering,
    Nonfiltering,
    Comparison,
    Num,
}
impl SamplerType {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for SamplerType {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum CubeFace {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
    Num,
}
impl CubeFace {
    pub const fn new() -> Self {
        Self::PosX
    }
}
impl Default for CubeFace {
    fn default() -> Self {
        Self::PosX
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum PrimitiveType {
    Default,
    Points,
    Lines,
    LineStrip,
    Triangles,
    TriangleStrip,
    Num,
}
impl PrimitiveType {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for PrimitiveType {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum Filter {
    Default,
    Nearest,
    Linear,
    Num,
}
impl Filter {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for Filter {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum Wrap {
    Default,
    Repeat,
    ClampToEdge,
    ClampToBorder,
    MirroredRepeat,
    Num,
}
impl Wrap {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for Wrap {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum BorderColor {
    Default,
    TransparentBlack,
    OpaqueBlack,
    OpaqueWhite,
    Num,
}
impl BorderColor {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for BorderColor {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum VertexFormat {
    Invalid,
    Float,
    Float2,
    Float3,
    Float4,
    Byte4,
    Byte4n,
    Ubyte4,
    Ubyte4n,
    Short2,
    Short2n,
    Ushort2n,
    Short4,
    Short4n,
    Ushort4n,
    Uint10N2,
    Half2,
    Half4,
    Num,
}
impl VertexFormat {
    pub const fn new() -> Self {
        Self::Invalid
    }
}
impl Default for VertexFormat {
    fn default() -> Self {
        Self::Invalid
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum VertexStep {
    Default,
    PerVertex,
    PerInstance,
    Num,
}
impl VertexStep {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for VertexStep {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum UniformType {
    Invalid,
    Float,
    Float2,
    Float3,
    Float4,
    Int,
    Int2,
    Int3,
    Int4,
    Mat4,
    Num,
}
impl UniformType {
    pub const fn new() -> Self {
        Self::Invalid
    }
}
impl Default for UniformType {
    fn default() -> Self {
        Self::Invalid
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum UniformLayout {
    Default,
    Native,
    Std140,
    Num,
}
impl UniformLayout {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for UniformLayout {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum CullMode {
    Default,
    None,
    Front,
    Back,
    Num,
}
impl CullMode {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for CullMode {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum FaceWinding {
    Default,
    Ccw,
    Cw,
    Num,
}
impl FaceWinding {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for FaceWinding {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum CompareFunc {
    Default,
    Never,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always,
    Num,
}
impl CompareFunc {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for CompareFunc {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum StencilOp {
    Default,
    Keep,
    Zero,
    Replace,
    IncrClamp,
    DecrClamp,
    Invert,
    IncrWrap,
    DecrWrap,
    Num,
}
impl StencilOp {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for StencilOp {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum BlendFactor {
    Default,
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstColor,
    OneMinusDstColor,
    DstAlpha,
    OneMinusDstAlpha,
    SrcAlphaSaturated,
    BlendColor,
    OneMinusBlendColor,
    BlendAlpha,
    OneMinusBlendAlpha,
    Num,
}
impl BlendFactor {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for BlendFactor {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum BlendOp {
    Default,
    Add,
    Subtract,
    ReverseSubtract,
    Num,
}
impl BlendOp {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for BlendOp {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum ColorMask {
    Default = 0,
    None = 16,
    R = 1,
    G = 2,
    Rg = 3,
    B = 4,
    Rb = 5,
    Gb = 6,
    Rgb = 7,
    A = 8,
    Ra = 9,
    Ga = 10,
    Rga = 11,
    Ba = 12,
    Rba = 13,
    Gba = 14,
    Rgba = 15,
}
impl ColorMask {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for ColorMask {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum LoadAction {
    Default,
    Clear,
    Load,
    Dontcare,
}
impl LoadAction {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for LoadAction {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum StoreAction {
    Default,
    Store,
    Dontcare,
}
impl StoreAction {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for StoreAction {
    fn default() -> Self {
        Self::Default
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ColorAttachmentAction {
    pub load_action: LoadAction,
    pub store_action: StoreAction,
    pub clear_value: Color,
}
impl ColorAttachmentAction {
    pub const fn new() -> Self {
        Self {
            load_action: LoadAction::new(),
            store_action: StoreAction::new(),
            clear_value: Color::new(),
        }
    }
}
impl Default for ColorAttachmentAction {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DepthAttachmentAction {
    pub load_action: LoadAction,
    pub store_action: StoreAction,
    pub clear_value: f32,
}
impl DepthAttachmentAction {
    pub const fn new() -> Self {
        Self {
            load_action: LoadAction::new(),
            store_action: StoreAction::new(),
            clear_value: 0.0,
        }
    }
}
impl Default for DepthAttachmentAction {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct StencilAttachmentAction {
    pub load_action: LoadAction,
    pub store_action: StoreAction,
    pub clear_value: u8,
}
impl StencilAttachmentAction {
    pub const fn new() -> Self {
        Self { load_action: LoadAction::new(), store_action: StoreAction::new(), clear_value: 0 }
    }
}
impl Default for StencilAttachmentAction {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PassAction {
    pub colors: [ColorAttachmentAction; 4],
    pub depth: DepthAttachmentAction,
    pub stencil: StencilAttachmentAction,
}
impl PassAction {
    pub const fn new() -> Self {
        Self {
            colors: [ColorAttachmentAction::new(); 4],
            depth: DepthAttachmentAction::new(),
            stencil: StencilAttachmentAction::new(),
        }
    }
}
impl Default for PassAction {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MetalSwapchain {
    pub current_drawable: *const core::ffi::c_void,
    pub depth_stencil_texture: *const core::ffi::c_void,
    pub msaa_color_texture: *const core::ffi::c_void,
}
impl MetalSwapchain {
    pub const fn new() -> Self {
        Self {
            current_drawable: core::ptr::null(),
            depth_stencil_texture: core::ptr::null(),
            msaa_color_texture: core::ptr::null(),
        }
    }
}
impl Default for MetalSwapchain {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3d11Swapchain {
    pub render_view: *const core::ffi::c_void,
    pub resolve_view: *const core::ffi::c_void,
    pub depth_stencil_view: *const core::ffi::c_void,
}
impl D3d11Swapchain {
    pub const fn new() -> Self {
        Self {
            render_view: core::ptr::null(),
            resolve_view: core::ptr::null(),
            depth_stencil_view: core::ptr::null(),
        }
    }
}
impl Default for D3d11Swapchain {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WgpuSwapchain {
    pub render_view: *const core::ffi::c_void,
    pub resolve_view: *const core::ffi::c_void,
    pub depth_stencil_view: *const core::ffi::c_void,
}
impl WgpuSwapchain {
    pub const fn new() -> Self {
        Self {
            render_view: core::ptr::null(),
            resolve_view: core::ptr::null(),
            depth_stencil_view: core::ptr::null(),
        }
    }
}
impl Default for WgpuSwapchain {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GlSwapchain {
    pub framebuffer: u32,
}
impl GlSwapchain {
    pub const fn new() -> Self {
        Self { framebuffer: 0 }
    }
}
impl Default for GlSwapchain {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Swapchain {
    pub width: i32,
    pub height: i32,
    pub sample_count: i32,
    pub color_format: PixelFormat,
    pub depth_format: PixelFormat,
    pub metal: MetalSwapchain,
    pub d3d11: D3d11Swapchain,
    pub wgpu: WgpuSwapchain,
    pub gl: GlSwapchain,
}
impl Swapchain {
    pub const fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            sample_count: 0,
            color_format: PixelFormat::new(),
            depth_format: PixelFormat::new(),
            metal: MetalSwapchain::new(),
            d3d11: D3d11Swapchain::new(),
            wgpu: WgpuSwapchain::new(),
            gl: GlSwapchain::new(),
        }
    }
}
impl Default for Swapchain {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Pass {
    pub _start_canary: u32,
    pub action: PassAction,
    pub attachments: Attachments,
    pub swapchain: Swapchain,
    pub label: *const core::ffi::c_char,
    pub _end_canary: u32,
}
impl Pass {
    pub const fn new() -> Self {
        Self {
            _start_canary: 0,
            action: PassAction::new(),
            attachments: Attachments::new(),
            swapchain: Swapchain::new(),
            label: core::ptr::null(),
            _end_canary: 0,
        }
    }
}
impl Default for Pass {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Bindings {
    pub _start_canary: u32,
    pub vertex_buffers: [Buffer; 8],
    pub vertex_buffer_offsets: [i32; 8],
    pub index_buffer: Buffer,
    pub index_buffer_offset: i32,
    pub images: [Image; 16],
    pub samplers: [Sampler; 16],
    pub storage_buffers: [Buffer; 8],
    pub _end_canary: u32,
}
impl Bindings {
    pub const fn new() -> Self {
        Self {
            _start_canary: 0,
            vertex_buffers: [Buffer::new(); 8],
            vertex_buffer_offsets: [0; 8],
            index_buffer: Buffer::new(),
            index_buffer_offset: 0,
            images: [Image::new(); 16],
            samplers: [Sampler::new(); 16],
            storage_buffers: [Buffer::new(); 8],
            _end_canary: 0,
        }
    }
}
impl Default for Bindings {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BufferDesc {
    pub _start_canary: u32,
    pub size: usize,
    pub _type: BufferType,
    pub usage: Usage,
    pub data: Range,
    pub label: *const core::ffi::c_char,
    pub gl_buffers: [u32; 2],
    pub mtl_buffers: [*const core::ffi::c_void; 2],
    pub d3d11_buffer: *const core::ffi::c_void,
    pub wgpu_buffer: *const core::ffi::c_void,
    pub _end_canary: u32,
}
impl BufferDesc {
    pub const fn new() -> Self {
        Self {
            _start_canary: 0,
            size: 0,
            _type: BufferType::new(),
            usage: Usage::new(),
            data: Range::new(),
            label: core::ptr::null(),
            gl_buffers: [0; 2],
            mtl_buffers: [core::ptr::null(); 2],
            d3d11_buffer: core::ptr::null(),
            wgpu_buffer: core::ptr::null(),
            _end_canary: 0,
        }
    }
}
impl Default for BufferDesc {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImageData {
    pub subimage: [[Range; 16]; 6],
}
impl ImageData {
    pub const fn new() -> Self {
        Self { subimage: [[Range::new(); 16]; 6] }
    }
}
impl Default for ImageData {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImageDesc {
    pub _start_canary: u32,
    pub _type: ImageType,
    pub render_target: bool,
    pub width: i32,
    pub height: i32,
    pub num_slices: i32,
    pub num_mipmaps: i32,
    pub usage: Usage,
    pub pixel_format: PixelFormat,
    pub sample_count: i32,
    pub data: ImageData,
    pub label: *const core::ffi::c_char,
    pub gl_textures: [u32; 2],
    pub gl_texture_target: u32,
    pub mtl_textures: [*const core::ffi::c_void; 2],
    pub d3d11_texture: *const core::ffi::c_void,
    pub d3d11_shader_resource_view: *const core::ffi::c_void,
    pub wgpu_texture: *const core::ffi::c_void,
    pub wgpu_texture_view: *const core::ffi::c_void,
    pub _end_canary: u32,
}
impl ImageDesc {
    pub const fn new() -> Self {
        Self {
            _start_canary: 0,
            _type: ImageType::new(),
            render_target: false,
            width: 0,
            height: 0,
            num_slices: 0,
            num_mipmaps: 0,
            usage: Usage::new(),
            pixel_format: PixelFormat::new(),
            sample_count: 0,
            data: ImageData::new(),
            label: core::ptr::null(),
            gl_textures: [0; 2],
            gl_texture_target: 0,
            mtl_textures: [core::ptr::null(); 2],
            d3d11_texture: core::ptr::null(),
            d3d11_shader_resource_view: core::ptr::null(),
            wgpu_texture: core::ptr::null(),
            wgpu_texture_view: core::ptr::null(),
            _end_canary: 0,
        }
    }
}
impl Default for ImageDesc {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SamplerDesc {
    pub _start_canary: u32,
    pub min_filter: Filter,
    pub mag_filter: Filter,
    pub mipmap_filter: Filter,
    pub wrap_u: Wrap,
    pub wrap_v: Wrap,
    pub wrap_w: Wrap,
    pub min_lod: f32,
    pub max_lod: f32,
    pub border_color: BorderColor,
    pub compare: CompareFunc,
    pub max_anisotropy: u32,
    pub label: *const core::ffi::c_char,
    pub gl_sampler: u32,
    pub mtl_sampler: *const core::ffi::c_void,
    pub d3d11_sampler: *const core::ffi::c_void,
    pub wgpu_sampler: *const core::ffi::c_void,
    pub _end_canary: u32,
}
impl SamplerDesc {
    pub const fn new() -> Self {
        Self {
            _start_canary: 0,
            min_filter: Filter::new(),
            mag_filter: Filter::new(),
            mipmap_filter: Filter::new(),
            wrap_u: Wrap::new(),
            wrap_v: Wrap::new(),
            wrap_w: Wrap::new(),
            min_lod: 0.0,
            max_lod: 0.0,
            border_color: BorderColor::new(),
            compare: CompareFunc::new(),
            max_anisotropy: 0,
            label: core::ptr::null(),
            gl_sampler: 0,
            mtl_sampler: core::ptr::null(),
            d3d11_sampler: core::ptr::null(),
            wgpu_sampler: core::ptr::null(),
            _end_canary: 0,
        }
    }
}
impl Default for SamplerDesc {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum ShaderStage {
    None,
    Vertex,
    Fragment,
}
impl ShaderStage {
    pub const fn new() -> Self {
        Self::None
    }
}
impl Default for ShaderStage {
    fn default() -> Self {
        Self::None
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ShaderFunction {
    pub source: *const core::ffi::c_char,
    pub bytecode: Range,
    pub entry: *const core::ffi::c_char,
    pub d3d11_target: *const core::ffi::c_char,
}
impl ShaderFunction {
    pub const fn new() -> Self {
        Self {
            source: core::ptr::null(),
            bytecode: Range::new(),
            entry: core::ptr::null(),
            d3d11_target: core::ptr::null(),
        }
    }
}
impl Default for ShaderFunction {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ShaderVertexAttr {
    pub glsl_name: *const core::ffi::c_char,
    pub hlsl_sem_name: *const core::ffi::c_char,
    pub hlsl_sem_index: u8,
}
impl ShaderVertexAttr {
    pub const fn new() -> Self {
        Self {
            glsl_name: core::ptr::null(),
            hlsl_sem_name: core::ptr::null(),
            hlsl_sem_index: 0,
        }
    }
}
impl Default for ShaderVertexAttr {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GlslShaderUniform {
    pub _type: UniformType,
    pub array_count: u16,
    pub glsl_name: *const core::ffi::c_char,
}
impl GlslShaderUniform {
    pub const fn new() -> Self {
        Self { _type: UniformType::new(), array_count: 0, glsl_name: core::ptr::null() }
    }
}
impl Default for GlslShaderUniform {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ShaderUniformBlock {
    pub stage: ShaderStage,
    pub size: u32,
    pub hlsl_register_b_n: u8,
    pub msl_buffer_n: u8,
    pub wgsl_group0_binding_n: u8,
    pub layout: UniformLayout,
    pub glsl_uniforms: [GlslShaderUniform; 16],
}
impl ShaderUniformBlock {
    pub const fn new() -> Self {
        Self {
            stage: ShaderStage::new(),
            size: 0,
            hlsl_register_b_n: 0,
            msl_buffer_n: 0,
            wgsl_group0_binding_n: 0,
            layout: UniformLayout::new(),
            glsl_uniforms: [GlslShaderUniform::new(); 16],
        }
    }
}
impl Default for ShaderUniformBlock {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ShaderImage {
    pub stage: ShaderStage,
    pub image_type: ImageType,
    pub sample_type: ImageSampleType,
    pub multisampled: bool,
    pub hlsl_register_t_n: u8,
    pub msl_texture_n: u8,
    pub wgsl_group1_binding_n: u8,
}
impl ShaderImage {
    pub const fn new() -> Self {
        Self {
            stage: ShaderStage::new(),
            image_type: ImageType::new(),
            sample_type: ImageSampleType::new(),
            multisampled: false,
            hlsl_register_t_n: 0,
            msl_texture_n: 0,
            wgsl_group1_binding_n: 0,
        }
    }
}
impl Default for ShaderImage {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ShaderSampler {
    pub stage: ShaderStage,
    pub sampler_type: SamplerType,
    pub hlsl_register_s_n: u8,
    pub msl_sampler_n: u8,
    pub wgsl_group1_binding_n: u8,
}
impl ShaderSampler {
    pub const fn new() -> Self {
        Self {
            stage: ShaderStage::new(),
            sampler_type: SamplerType::new(),
            hlsl_register_s_n: 0,
            msl_sampler_n: 0,
            wgsl_group1_binding_n: 0,
        }
    }
}
impl Default for ShaderSampler {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ShaderStorageBuffer {
    pub stage: ShaderStage,
    pub readonly: bool,
    pub hlsl_register_t_n: u8,
    pub msl_buffer_n: u8,
    pub wgsl_group1_binding_n: u8,
    pub glsl_binding_n: u8,
}
impl ShaderStorageBuffer {
    pub const fn new() -> Self {
        Self {
            stage: ShaderStage::new(),
            readonly: false,
            hlsl_register_t_n: 0,
            msl_buffer_n: 0,
            wgsl_group1_binding_n: 0,
            glsl_binding_n: 0,
        }
    }
}
impl Default for ShaderStorageBuffer {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ShaderImageSamplerPair {
    pub stage: ShaderStage,
    pub image_slot: u8,
    pub sampler_slot: u8,
    pub glsl_name: *const core::ffi::c_char,
}
impl ShaderImageSamplerPair {
    pub const fn new() -> Self {
        Self {
            stage: ShaderStage::new(),
            image_slot: 0,
            sampler_slot: 0,
            glsl_name: core::ptr::null(),
        }
    }
}
impl Default for ShaderImageSamplerPair {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ShaderDesc {
    pub _start_canary: u32,
    pub vertex_func: ShaderFunction,
    pub fragment_func: ShaderFunction,
    pub attrs: [ShaderVertexAttr; 16],
    pub uniform_blocks: [ShaderUniformBlock; 8],
    pub storage_buffers: [ShaderStorageBuffer; 8],
    pub images: [ShaderImage; 16],
    pub samplers: [ShaderSampler; 16],
    pub image_sampler_pairs: [ShaderImageSamplerPair; 16],
    pub label: *const core::ffi::c_char,
    pub _end_canary: u32,
}
impl ShaderDesc {
    pub const fn new() -> Self {
        Self {
            _start_canary: 0,
            vertex_func: ShaderFunction::new(),
            fragment_func: ShaderFunction::new(),
            attrs: [ShaderVertexAttr::new(); 16],
            uniform_blocks: [ShaderUniformBlock::new(); 8],
            storage_buffers: [ShaderStorageBuffer::new(); 8],
            images: [ShaderImage::new(); 16],
            samplers: [ShaderSampler::new(); 16],
            image_sampler_pairs: [ShaderImageSamplerPair::new(); 16],
            label: core::ptr::null(),
            _end_canary: 0,
        }
    }
}
impl Default for ShaderDesc {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VertexBufferLayoutState {
    pub stride: i32,
    pub step_func: VertexStep,
    pub step_rate: i32,
}
impl VertexBufferLayoutState {
    pub const fn new() -> Self {
        Self { stride: 0, step_func: VertexStep::new(), step_rate: 0 }
    }
}
impl Default for VertexBufferLayoutState {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VertexAttrState {
    pub buffer_index: i32,
    pub offset: i32,
    pub format: VertexFormat,
}
impl VertexAttrState {
    pub const fn new() -> Self {
        Self { buffer_index: 0, offset: 0, format: VertexFormat::new() }
    }
}
impl Default for VertexAttrState {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VertexLayoutState {
    pub buffers: [VertexBufferLayoutState; 8],
    pub attrs: [VertexAttrState; 16],
}
impl VertexLayoutState {
    pub const fn new() -> Self {
        Self {
            buffers: [VertexBufferLayoutState::new(); 8],
            attrs: [VertexAttrState::new(); 16],
        }
    }
}
impl Default for VertexLayoutState {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct StencilFaceState {
    pub compare: CompareFunc,
    pub fail_op: StencilOp,
    pub depth_fail_op: StencilOp,
    pub pass_op: StencilOp,
}
impl StencilFaceState {
    pub const fn new() -> Self {
        Self {
            compare: CompareFunc::new(),
            fail_op: StencilOp::new(),
            depth_fail_op: StencilOp::new(),
            pass_op: StencilOp::new(),
        }
    }
}
impl Default for StencilFaceState {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct StencilState {
    pub enabled: bool,
    pub front: StencilFaceState,
    pub back: StencilFaceState,
    pub read_mask: u8,
    pub write_mask: u8,
    pub _ref: u8,
}
impl StencilState {
    pub const fn new() -> Self {
        Self {
            enabled: false,
            front: StencilFaceState::new(),
            back: StencilFaceState::new(),
            read_mask: 0,
            write_mask: 0,
            _ref: 0,
        }
    }
}
impl Default for StencilState {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DepthState {
    pub pixel_format: PixelFormat,
    pub compare: CompareFunc,
    pub write_enabled: bool,
    pub bias: f32,
    pub bias_slope_scale: f32,
    pub bias_clamp: f32,
}
impl DepthState {
    pub const fn new() -> Self {
        Self {
            pixel_format: PixelFormat::new(),
            compare: CompareFunc::new(),
            write_enabled: false,
            bias: 0.0,
            bias_slope_scale: 0.0,
            bias_clamp: 0.0,
        }
    }
}
impl Default for DepthState {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BlendState {
    pub enabled: bool,
    pub src_factor_rgb: BlendFactor,
    pub dst_factor_rgb: BlendFactor,
    pub op_rgb: BlendOp,
    pub src_factor_alpha: BlendFactor,
    pub dst_factor_alpha: BlendFactor,
    pub op_alpha: BlendOp,
}
impl BlendState {
    pub const fn new() -> Self {
        Self {
            enabled: false,
            src_factor_rgb: BlendFactor::new(),
            dst_factor_rgb: BlendFactor::new(),
            op_rgb: BlendOp::new(),
            src_factor_alpha: BlendFactor::new(),
            dst_factor_alpha: BlendFactor::new(),
            op_alpha: BlendOp::new(),
        }
    }
}
impl Default for BlendState {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ColorTargetState {
    pub pixel_format: PixelFormat,
    pub write_mask: ColorMask,
    pub blend: BlendState,
}
impl ColorTargetState {
    pub const fn new() -> Self {
        Self {
            pixel_format: PixelFormat::new(),
            write_mask: ColorMask::new(),
            blend: BlendState::new(),
        }
    }
}
impl Default for ColorTargetState {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineDesc {
    pub _start_canary: u32,
    pub shader: Shader,
    pub layout: VertexLayoutState,
    pub depth: DepthState,
    pub stencil: StencilState,
    pub color_count: i32,
    pub colors: [ColorTargetState; 4],
    pub primitive_type: PrimitiveType,
    pub index_type: IndexType,
    pub cull_mode: CullMode,
    pub face_winding: FaceWinding,
    pub sample_count: i32,
    pub blend_color: Color,
    pub alpha_to_coverage_enabled: bool,
    pub label: *const core::ffi::c_char,
    pub _end_canary: u32,
}
impl PipelineDesc {
    pub const fn new() -> Self {
        Self {
            _start_canary: 0,
            shader: Shader::new(),
            layout: VertexLayoutState::new(),
            depth: DepthState::new(),
            stencil: StencilState::new(),
            color_count: 0,
            colors: [ColorTargetState::new(); 4],
            primitive_type: PrimitiveType::new(),
            index_type: IndexType::new(),
            cull_mode: CullMode::new(),
            face_winding: FaceWinding::new(),
            sample_count: 0,
            blend_color: Color::new(),
            alpha_to_coverage_enabled: false,
            label: core::ptr::null(),
            _end_canary: 0,
        }
    }
}
impl Default for PipelineDesc {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AttachmentDesc {
    pub image: Image,
    pub mip_level: i32,
    pub slice: i32,
}
impl AttachmentDesc {
    pub const fn new() -> Self {
        Self { image: Image::new(), mip_level: 0, slice: 0 }
    }
}
impl Default for AttachmentDesc {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AttachmentsDesc {
    pub _start_canary: u32,
    pub colors: [AttachmentDesc; 4],
    pub resolves: [AttachmentDesc; 4],
    pub depth_stencil: AttachmentDesc,
    pub label: *const core::ffi::c_char,
    pub _end_canary: u32,
}
impl AttachmentsDesc {
    pub const fn new() -> Self {
        Self {
            _start_canary: 0,
            colors: [AttachmentDesc::new(); 4],
            resolves: [AttachmentDesc::new(); 4],
            depth_stencil: AttachmentDesc::new(),
            label: core::ptr::null(),
            _end_canary: 0,
        }
    }
}
impl Default for AttachmentsDesc {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TraceHooks {
    pub user_data: *mut core::ffi::c_void,
    pub reset_state_cache: Option<extern "C" fn(*mut core::ffi::c_void)>,
    pub make_buffer: Option<extern "C" fn(*const BufferDesc, Buffer, *mut core::ffi::c_void)>,
    pub make_image: Option<extern "C" fn(*const ImageDesc, Image, *mut core::ffi::c_void)>,
    pub make_sampler: Option<extern "C" fn(*const SamplerDesc, Sampler, *mut core::ffi::c_void)>,
    pub make_shader: Option<extern "C" fn(*const ShaderDesc, Shader, *mut core::ffi::c_void)>,
    pub make_pipeline: Option<extern "C" fn(*const PipelineDesc, Pipeline, *mut core::ffi::c_void)>,
    pub make_attachments: Option<extern "C" fn(*const AttachmentsDesc, Attachments, *mut core::ffi::c_void)>,
    pub destroy_buffer: Option<extern "C" fn(Buffer, *mut core::ffi::c_void)>,
    pub destroy_image: Option<extern "C" fn(Image, *mut core::ffi::c_void)>,
    pub destroy_sampler: Option<extern "C" fn(Sampler, *mut core::ffi::c_void)>,
    pub destroy_shader: Option<extern "C" fn(Shader, *mut core::ffi::c_void)>,
    pub destroy_pipeline: Option<extern "C" fn(Pipeline, *mut core::ffi::c_void)>,
    pub destroy_attachments: Option<extern "C" fn(Attachments, *mut core::ffi::c_void)>,
    pub update_buffer: Option<extern "C" fn(Buffer, *const Range, *mut core::ffi::c_void)>,
    pub update_image: Option<extern "C" fn(Image, *const ImageData, *mut core::ffi::c_void)>,
    pub append_buffer: Option<extern "C" fn(Buffer, *const Range, i32, *mut core::ffi::c_void)>,
    pub begin_pass: Option<extern "C" fn(*const Pass, *mut core::ffi::c_void)>,
    pub apply_viewport: Option<extern "C" fn(i32, i32, i32, i32, bool, *mut core::ffi::c_void)>,
    pub apply_scissor_rect: Option<extern "C" fn(i32, i32, i32, i32, bool, *mut core::ffi::c_void)>,
    pub apply_pipeline: Option<extern "C" fn(Pipeline, *mut core::ffi::c_void)>,
    pub apply_bindings: Option<extern "C" fn(*const Bindings, *mut core::ffi::c_void)>,
    pub apply_uniforms: Option<extern "C" fn(i32, *const Range, *mut core::ffi::c_void)>,
    pub draw: Option<extern "C" fn(i32, i32, i32, *mut core::ffi::c_void)>,
    pub end_pass: Option<extern "C" fn(*mut core::ffi::c_void)>,
    pub commit: Option<extern "C" fn(*mut core::ffi::c_void)>,
    pub alloc_buffer: Option<extern "C" fn(Buffer, *mut core::ffi::c_void)>,
    pub alloc_image: Option<extern "C" fn(Image, *mut core::ffi::c_void)>,
    pub alloc_sampler: Option<extern "C" fn(Sampler, *mut core::ffi::c_void)>,
    pub alloc_shader: Option<extern "C" fn(Shader, *mut core::ffi::c_void)>,
    pub alloc_pipeline: Option<extern "C" fn(Pipeline, *mut core::ffi::c_void)>,
    pub alloc_attachments: Option<extern "C" fn(Attachments, *mut core::ffi::c_void)>,
    pub dealloc_buffer: Option<extern "C" fn(Buffer, *mut core::ffi::c_void)>,
    pub dealloc_image: Option<extern "C" fn(Image, *mut core::ffi::c_void)>,
    pub dealloc_sampler: Option<extern "C" fn(Sampler, *mut core::ffi::c_void)>,
    pub dealloc_shader: Option<extern "C" fn(Shader, *mut core::ffi::c_void)>,
    pub dealloc_pipeline: Option<extern "C" fn(Pipeline, *mut core::ffi::c_void)>,
    pub dealloc_attachments: Option<extern "C" fn(Attachments, *mut core::ffi::c_void)>,
    pub init_buffer: Option<extern "C" fn(Buffer, *const BufferDesc, *mut core::ffi::c_void)>,
    pub init_image: Option<extern "C" fn(Image, *const ImageDesc, *mut core::ffi::c_void)>,
    pub init_sampler: Option<extern "C" fn(Sampler, *const SamplerDesc, *mut core::ffi::c_void)>,
    pub init_shader: Option<extern "C" fn(Shader, *const ShaderDesc, *mut core::ffi::c_void)>,
    pub init_pipeline: Option<extern "C" fn(Pipeline, *const PipelineDesc, *mut core::ffi::c_void)>,
    pub init_attachments: Option<extern "C" fn(Attachments, *const AttachmentsDesc, *mut core::ffi::c_void)>,
    pub uninit_buffer: Option<extern "C" fn(Buffer, *mut core::ffi::c_void)>,
    pub uninit_image: Option<extern "C" fn(Image, *mut core::ffi::c_void)>,
    pub uninit_sampler: Option<extern "C" fn(Sampler, *mut core::ffi::c_void)>,
    pub uninit_shader: Option<extern "C" fn(Shader, *mut core::ffi::c_void)>,
    pub uninit_pipeline: Option<extern "C" fn(Pipeline, *mut core::ffi::c_void)>,
    pub uninit_attachments: Option<extern "C" fn(Attachments, *mut core::ffi::c_void)>,
    pub fail_buffer: Option<extern "C" fn(Buffer, *mut core::ffi::c_void)>,
    pub fail_image: Option<extern "C" fn(Image, *mut core::ffi::c_void)>,
    pub fail_sampler: Option<extern "C" fn(Sampler, *mut core::ffi::c_void)>,
    pub fail_shader: Option<extern "C" fn(Shader, *mut core::ffi::c_void)>,
    pub fail_pipeline: Option<extern "C" fn(Pipeline, *mut core::ffi::c_void)>,
    pub fail_attachments: Option<extern "C" fn(Attachments, *mut core::ffi::c_void)>,
    pub push_debug_group: Option<extern "C" fn(*const core::ffi::c_char, *mut core::ffi::c_void)>,
    pub pop_debug_group: Option<extern "C" fn(*mut core::ffi::c_void)>,
}
impl TraceHooks {
    pub const fn new() -> Self {
        Self {
            user_data: core::ptr::null_mut(),
            reset_state_cache: None,
            make_buffer: None,
            make_image: None,
            make_sampler: None,
            make_shader: None,
            make_pipeline: None,
            make_attachments: None,
            destroy_buffer: None,
            destroy_image: None,
            destroy_sampler: None,
            destroy_shader: None,
            destroy_pipeline: None,
            destroy_attachments: None,
            update_buffer: None,
            update_image: None,
            append_buffer: None,
            begin_pass: None,
            apply_viewport: None,
            apply_scissor_rect: None,
            apply_pipeline: None,
            apply_bindings: None,
            apply_uniforms: None,
            draw: None,
            end_pass: None,
            commit: None,
            alloc_buffer: None,
            alloc_image: None,
            alloc_sampler: None,
            alloc_shader: None,
            alloc_pipeline: None,
            alloc_attachments: None,
            dealloc_buffer: None,
            dealloc_image: None,
            dealloc_sampler: None,
            dealloc_shader: None,
            dealloc_pipeline: None,
            dealloc_attachments: None,
            init_buffer: None,
            init_image: None,
            init_sampler: None,
            init_shader: None,
            init_pipeline: None,
            init_attachments: None,
            uninit_buffer: None,
            uninit_image: None,
            uninit_sampler: None,
            uninit_shader: None,
            uninit_pipeline: None,
            uninit_attachments: None,
            fail_buffer: None,
            fail_image: None,
            fail_sampler: None,
            fail_shader: None,
            fail_pipeline: None,
            fail_attachments: None,
            push_debug_group: None,
            pop_debug_group: None,
        }
    }
}
impl Default for TraceHooks {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SlotInfo {
    pub state: ResourceState,
    pub res_id: u32,
}
impl SlotInfo {
    pub const fn new() -> Self {
        Self { state: ResourceState::new(), res_id: 0 }
    }
}
impl Default for SlotInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BufferInfo {
    pub slot: SlotInfo,
    pub update_frame_index: u32,
    pub append_frame_index: u32,
    pub append_pos: i32,
    pub append_overflow: bool,
    pub num_slots: i32,
    pub active_slot: i32,
}
impl BufferInfo {
    pub const fn new() -> Self {
        Self {
            slot: SlotInfo::new(),
            update_frame_index: 0,
            append_frame_index: 0,
            append_pos: 0,
            append_overflow: false,
            num_slots: 0,
            active_slot: 0,
        }
    }
}
impl Default for BufferInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImageInfo {
    pub slot: SlotInfo,
    pub upd_frame_index: u32,
    pub num_slots: i32,
    pub active_slot: i32,
}
impl ImageInfo {
    pub const fn new() -> Self {
        Self { slot: SlotInfo::new(), upd_frame_index: 0, num_slots: 0, active_slot: 0 }
    }
}
impl Default for ImageInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SamplerInfo {
    pub slot: SlotInfo,
}
impl SamplerInfo {
    pub const fn new() -> Self {
        Self { slot: SlotInfo::new() }
    }
}
impl Default for SamplerInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ShaderInfo {
    pub slot: SlotInfo,
}
impl ShaderInfo {
    pub const fn new() -> Self {
        Self { slot: SlotInfo::new() }
    }
}
impl Default for ShaderInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineInfo {
    pub slot: SlotInfo,
}
impl PipelineInfo {
    pub const fn new() -> Self {
        Self { slot: SlotInfo::new() }
    }
}
impl Default for PipelineInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AttachmentsInfo {
    pub slot: SlotInfo,
}
impl AttachmentsInfo {
    pub const fn new() -> Self {
        Self { slot: SlotInfo::new() }
    }
}
impl Default for AttachmentsInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FrameStatsGl {
    pub num_bind_buffer: u32,
    pub num_active_texture: u32,
    pub num_bind_texture: u32,
    pub num_bind_sampler: u32,
    pub num_use_program: u32,
    pub num_render_state: u32,
    pub num_vertex_attrib_pointer: u32,
    pub num_vertex_attrib_divisor: u32,
    pub num_enable_vertex_attrib_array: u32,
    pub num_disable_vertex_attrib_array: u32,
    pub num_uniform: u32,
}
impl FrameStatsGl {
    pub const fn new() -> Self {
        Self {
            num_bind_buffer: 0,
            num_active_texture: 0,
            num_bind_texture: 0,
            num_bind_sampler: 0,
            num_use_program: 0,
            num_render_state: 0,
            num_vertex_attrib_pointer: 0,
            num_vertex_attrib_divisor: 0,
            num_enable_vertex_attrib_array: 0,
            num_disable_vertex_attrib_array: 0,
            num_uniform: 0,
        }
    }
}
impl Default for FrameStatsGl {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FrameStatsD3d11Pass {
    pub num_om_set_render_targets: u32,
    pub num_clear_render_target_view: u32,
    pub num_clear_depth_stencil_view: u32,
    pub num_resolve_subresource: u32,
}
impl FrameStatsD3d11Pass {
    pub const fn new() -> Self {
        Self {
            num_om_set_render_targets: 0,
            num_clear_render_target_view: 0,
            num_clear_depth_stencil_view: 0,
            num_resolve_subresource: 0,
        }
    }
}
impl Default for FrameStatsD3d11Pass {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FrameStatsD3d11Pipeline {
    pub num_rs_set_state: u32,
    pub num_om_set_depth_stencil_state: u32,
    pub num_om_set_blend_state: u32,
    pub num_ia_set_primitive_topology: u32,
    pub num_ia_set_input_layout: u32,
    pub num_vs_set_shader: u32,
    pub num_vs_set_constant_buffers: u32,
    pub num_ps_set_shader: u32,
    pub num_ps_set_constant_buffers: u32,
}
impl FrameStatsD3d11Pipeline {
    pub const fn new() -> Self {
        Self {
            num_rs_set_state: 0,
            num_om_set_depth_stencil_state: 0,
            num_om_set_blend_state: 0,
            num_ia_set_primitive_topology: 0,
            num_ia_set_input_layout: 0,
            num_vs_set_shader: 0,
            num_vs_set_constant_buffers: 0,
            num_ps_set_shader: 0,
            num_ps_set_constant_buffers: 0,
        }
    }
}
impl Default for FrameStatsD3d11Pipeline {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FrameStatsD3d11Bindings {
    pub num_ia_set_vertex_buffers: u32,
    pub num_ia_set_index_buffer: u32,
    pub num_vs_set_shader_resources: u32,
    pub num_ps_set_shader_resources: u32,
    pub num_vs_set_samplers: u32,
    pub num_ps_set_samplers: u32,
}
impl FrameStatsD3d11Bindings {
    pub const fn new() -> Self {
        Self {
            num_ia_set_vertex_buffers: 0,
            num_ia_set_index_buffer: 0,
            num_vs_set_shader_resources: 0,
            num_ps_set_shader_resources: 0,
            num_vs_set_samplers: 0,
            num_ps_set_samplers: 0,
        }
    }
}
impl Default for FrameStatsD3d11Bindings {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FrameStatsD3d11Uniforms {
    pub num_update_subresource: u32,
}
impl FrameStatsD3d11Uniforms {
    pub const fn new() -> Self {
        Self { num_update_subresource: 0 }
    }
}
impl Default for FrameStatsD3d11Uniforms {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FrameStatsD3d11Draw {
    pub num_draw_indexed_instanced: u32,
    pub num_draw_indexed: u32,
    pub num_draw_instanced: u32,
    pub num_draw: u32,
}
impl FrameStatsD3d11Draw {
    pub const fn new() -> Self {
        Self {
            num_draw_indexed_instanced: 0,
            num_draw_indexed: 0,
            num_draw_instanced: 0,
            num_draw: 0,
        }
    }
}
impl Default for FrameStatsD3d11Draw {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FrameStatsD3d11 {
    pub pass: FrameStatsD3d11Pass,
    pub pipeline: FrameStatsD3d11Pipeline,
    pub bindings: FrameStatsD3d11Bindings,
    pub uniforms: FrameStatsD3d11Uniforms,
    pub draw: FrameStatsD3d11Draw,
    pub num_map: u32,
    pub num_unmap: u32,
}
impl FrameStatsD3d11 {
    pub const fn new() -> Self {
        Self {
            pass: FrameStatsD3d11Pass::new(),
            pipeline: FrameStatsD3d11Pipeline::new(),
            bindings: FrameStatsD3d11Bindings::new(),
            uniforms: FrameStatsD3d11Uniforms::new(),
            draw: FrameStatsD3d11Draw::new(),
            num_map: 0,
            num_unmap: 0,
        }
    }
}
impl Default for FrameStatsD3d11 {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FrameStatsMetalIdpool {
    pub num_added: u32,
    pub num_released: u32,
    pub num_garbage_collected: u32,
}
impl FrameStatsMetalIdpool {
    pub const fn new() -> Self {
        Self { num_added: 0, num_released: 0, num_garbage_collected: 0 }
    }
}
impl Default for FrameStatsMetalIdpool {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FrameStatsMetalPipeline {
    pub num_set_blend_color: u32,
    pub num_set_cull_mode: u32,
    pub num_set_front_facing_winding: u32,
    pub num_set_stencil_reference_value: u32,
    pub num_set_depth_bias: u32,
    pub num_set_render_pipeline_state: u32,
    pub num_set_depth_stencil_state: u32,
}
impl FrameStatsMetalPipeline {
    pub const fn new() -> Self {
        Self {
            num_set_blend_color: 0,
            num_set_cull_mode: 0,
            num_set_front_facing_winding: 0,
            num_set_stencil_reference_value: 0,
            num_set_depth_bias: 0,
            num_set_render_pipeline_state: 0,
            num_set_depth_stencil_state: 0,
        }
    }
}
impl Default for FrameStatsMetalPipeline {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FrameStatsMetalBindings {
    pub num_set_vertex_buffer: u32,
    pub num_set_vertex_texture: u32,
    pub num_set_vertex_sampler_state: u32,
    pub num_set_fragment_buffer: u32,
    pub num_set_fragment_texture: u32,
    pub num_set_fragment_sampler_state: u32,
}
impl FrameStatsMetalBindings {
    pub const fn new() -> Self {
        Self {
            num_set_vertex_buffer: 0,
            num_set_vertex_texture: 0,
            num_set_vertex_sampler_state: 0,
            num_set_fragment_buffer: 0,
            num_set_fragment_texture: 0,
            num_set_fragment_sampler_state: 0,
        }
    }
}
impl Default for FrameStatsMetalBindings {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FrameStatsMetalUniforms {
    pub num_set_vertex_buffer_offset: u32,
    pub num_set_fragment_buffer_offset: u32,
}
impl FrameStatsMetalUniforms {
    pub const fn new() -> Self {
        Self { num_set_vertex_buffer_offset: 0, num_set_fragment_buffer_offset: 0 }
    }
}
impl Default for FrameStatsMetalUniforms {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FrameStatsMetal {
    pub idpool: FrameStatsMetalIdpool,
    pub pipeline: FrameStatsMetalPipeline,
    pub bindings: FrameStatsMetalBindings,
    pub uniforms: FrameStatsMetalUniforms,
}
impl FrameStatsMetal {
    pub const fn new() -> Self {
        Self {
            idpool: FrameStatsMetalIdpool::new(),
            pipeline: FrameStatsMetalPipeline::new(),
            bindings: FrameStatsMetalBindings::new(),
            uniforms: FrameStatsMetalUniforms::new(),
        }
    }
}
impl Default for FrameStatsMetal {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FrameStatsWgpuUniforms {
    pub num_set_bindgroup: u32,
    pub size_write_buffer: u32,
}
impl FrameStatsWgpuUniforms {
    pub const fn new() -> Self {
        Self { num_set_bindgroup: 0, size_write_buffer: 0 }
    }
}
impl Default for FrameStatsWgpuUniforms {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FrameStatsWgpuBindings {
    pub num_set_vertex_buffer: u32,
    pub num_skip_redundant_vertex_buffer: u32,
    pub num_set_index_buffer: u32,
    pub num_skip_redundant_index_buffer: u32,
    pub num_create_bindgroup: u32,
    pub num_discard_bindgroup: u32,
    pub num_set_bindgroup: u32,
    pub num_skip_redundant_bindgroup: u32,
    pub num_bindgroup_cache_hits: u32,
    pub num_bindgroup_cache_misses: u32,
    pub num_bindgroup_cache_collisions: u32,
    pub num_bindgroup_cache_invalidates: u32,
    pub num_bindgroup_cache_hash_vs_key_mismatch: u32,
}
impl FrameStatsWgpuBindings {
    pub const fn new() -> Self {
        Self {
            num_set_vertex_buffer: 0,
            num_skip_redundant_vertex_buffer: 0,
            num_set_index_buffer: 0,
            num_skip_redundant_index_buffer: 0,
            num_create_bindgroup: 0,
            num_discard_bindgroup: 0,
            num_set_bindgroup: 0,
            num_skip_redundant_bindgroup: 0,
            num_bindgroup_cache_hits: 0,
            num_bindgroup_cache_misses: 0,
            num_bindgroup_cache_collisions: 0,
            num_bindgroup_cache_invalidates: 0,
            num_bindgroup_cache_hash_vs_key_mismatch: 0,
        }
    }
}
impl Default for FrameStatsWgpuBindings {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FrameStatsWgpu {
    pub uniforms: FrameStatsWgpuUniforms,
    pub bindings: FrameStatsWgpuBindings,
}
impl FrameStatsWgpu {
    pub const fn new() -> Self {
        Self { uniforms: FrameStatsWgpuUniforms::new(), bindings: FrameStatsWgpuBindings::new() }
    }
}
impl Default for FrameStatsWgpu {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FrameStats {
    pub frame_index: u32,
    pub num_passes: u32,
    pub num_apply_viewport: u32,
    pub num_apply_scissor_rect: u32,
    pub num_apply_pipeline: u32,
    pub num_apply_bindings: u32,
    pub num_apply_uniforms: u32,
    pub num_draw: u32,
    pub num_update_buffer: u32,
    pub num_append_buffer: u32,
    pub num_update_image: u32,
    pub size_apply_uniforms: u32,
    pub size_update_buffer: u32,
    pub size_append_buffer: u32,
    pub size_update_image: u32,
    pub gl: FrameStatsGl,
    pub d3d11: FrameStatsD3d11,
    pub metal: FrameStatsMetal,
    pub wgpu: FrameStatsWgpu,
}
impl FrameStats {
    pub const fn new() -> Self {
        Self {
            frame_index: 0,
            num_passes: 0,
            num_apply_viewport: 0,
            num_apply_scissor_rect: 0,
            num_apply_pipeline: 0,
            num_apply_bindings: 0,
            num_apply_uniforms: 0,
            num_draw: 0,
            num_update_buffer: 0,
            num_append_buffer: 0,
            num_update_image: 0,
            size_apply_uniforms: 0,
            size_update_buffer: 0,
            size_append_buffer: 0,
            size_update_image: 0,
            gl: FrameStatsGl::new(),
            d3d11: FrameStatsD3d11::new(),
            metal: FrameStatsMetal::new(),
            wgpu: FrameStatsWgpu::new(),
        }
    }
}
impl Default for FrameStats {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum LogItem {
    Ok,
    MallocFailed,
    GlTextureFormatNotSupported,
    Gl3dTexturesNotSupported,
    GlArrayTexturesNotSupported,
    GlShaderCompilationFailed,
    GlShaderLinkingFailed,
    GlVertexAttributeNotFoundInShader,
    GlImageSamplerNameNotFoundInShader,
    GlFramebufferStatusUndefined,
    GlFramebufferStatusIncompleteAttachment,
    GlFramebufferStatusIncompleteMissingAttachment,
    GlFramebufferStatusUnsupported,
    GlFramebufferStatusIncompleteMultisample,
    GlFramebufferStatusUnknown,
    D3d11CreateBufferFailed,
    D3d11CreateBufferSrvFailed,
    D3d11CreateDepthTextureUnsupportedPixelFormat,
    D3d11CreateDepthTextureFailed,
    D3d11Create2dTextureUnsupportedPixelFormat,
    D3d11Create2dTextureFailed,
    D3d11Create2dSrvFailed,
    D3d11Create3dTextureUnsupportedPixelFormat,
    D3d11Create3dTextureFailed,
    D3d11Create3dSrvFailed,
    D3d11CreateMsaaTextureFailed,
    D3d11CreateSamplerStateFailed,
    D3d11LoadD3dcompiler47DllFailed,
    D3d11ShaderCompilationFailed,
    D3d11ShaderCompilationOutput,
    D3d11CreateConstantBufferFailed,
    D3d11CreateInputLayoutFailed,
    D3d11CreateRasterizerStateFailed,
    D3d11CreateDepthStencilStateFailed,
    D3d11CreateBlendStateFailed,
    D3d11CreateRtvFailed,
    D3d11CreateDsvFailed,
    D3d11MapForUpdateBufferFailed,
    D3d11MapForAppendBufferFailed,
    D3d11MapForUpdateImageFailed,
    MetalCreateBufferFailed,
    MetalTextureFormatNotSupported,
    MetalCreateTextureFailed,
    MetalCreateSamplerFailed,
    MetalShaderCompilationFailed,
    MetalShaderCreationFailed,
    MetalShaderCompilationOutput,
    MetalShaderEntryNotFound,
    MetalCreateRpsFailed,
    MetalCreateRpsOutput,
    MetalCreateDssFailed,
    WgpuBindgroupsPoolExhausted,
    WgpuBindgroupscacheSizeGreaterOne,
    WgpuBindgroupscacheSizePow2,
    WgpuCreatebindgroupFailed,
    WgpuCreateBufferFailed,
    WgpuCreateTextureFailed,
    WgpuCreateTextureViewFailed,
    WgpuCreateSamplerFailed,
    WgpuCreateShaderModuleFailed,
    WgpuShaderCreateBindgroupLayoutFailed,
    WgpuCreatePipelineLayoutFailed,
    WgpuCreateRenderPipelineFailed,
    WgpuAttachmentsCreateTextureViewFailed,
    DrawRequiredBindingsOrUniformsMissing,
    IdenticalCommitListener,
    CommitListenerArrayFull,
    TraceHooksNotEnabled,
    DeallocBufferInvalidState,
    DeallocImageInvalidState,
    DeallocSamplerInvalidState,
    DeallocShaderInvalidState,
    DeallocPipelineInvalidState,
    DeallocAttachmentsInvalidState,
    InitBufferInvalidState,
    InitImageInvalidState,
    InitSamplerInvalidState,
    InitShaderInvalidState,
    InitPipelineInvalidState,
    InitAttachmentsInvalidState,
    UninitBufferInvalidState,
    UninitImageInvalidState,
    UninitSamplerInvalidState,
    UninitShaderInvalidState,
    UninitPipelineInvalidState,
    UninitAttachmentsInvalidState,
    FailBufferInvalidState,
    FailImageInvalidState,
    FailSamplerInvalidState,
    FailShaderInvalidState,
    FailPipelineInvalidState,
    FailAttachmentsInvalidState,
    BufferPoolExhausted,
    ImagePoolExhausted,
    SamplerPoolExhausted,
    ShaderPoolExhausted,
    PipelinePoolExhausted,
    PassPoolExhausted,
    BeginpassAttachmentInvalid,
    DrawWithoutBindings,
    ValidateBufferdescCanary,
    ValidateBufferdescSize,
    ValidateBufferdescData,
    ValidateBufferdescDataSize,
    ValidateBufferdescNoData,
    ValidateBufferdescStoragebufferSupported,
    ValidateBufferdescStoragebufferSizeMultiple4,
    ValidateImagedataNodata,
    ValidateImagedataDataSize,
    ValidateImagedescCanary,
    ValidateImagedescWidth,
    ValidateImagedescHeight,
    ValidateImagedescRtPixelformat,
    ValidateImagedescNonrtPixelformat,
    ValidateImagedescMsaaButNoRt,
    ValidateImagedescNoMsaaRtSupport,
    ValidateImagedescMsaaNumMipmaps,
    ValidateImagedescMsaa3dImage,
    ValidateImagedescDepth3dImage,
    ValidateImagedescRtImmutable,
    ValidateImagedescRtNoData,
    ValidateImagedescInjectedNoData,
    ValidateImagedescDynamicNoData,
    ValidateImagedescCompressedImmutable,
    ValidateSamplerdescCanary,
    ValidateSamplerdescAnistropicRequiresLinearFiltering,
    ValidateShaderdescCanary,
    ValidateShaderdescSource,
    ValidateShaderdescBytecode,
    ValidateShaderdescSourceOrBytecode,
    ValidateShaderdescNoBytecodeSize,
    ValidateShaderdescNoContUbMembers,
    ValidateShaderdescUbSizeIsZero,
    ValidateShaderdescUbMetalBufferSlotOutOfRange,
    ValidateShaderdescUbMetalBufferSlotCollision,
    ValidateShaderdescUbHlslRegisterBOutOfRange,
    ValidateShaderdescUbHlslRegisterBCollision,
    ValidateShaderdescUbWgslGroup0BindingOutOfRange,
    ValidateShaderdescUbWgslGroup0BindingCollision,
    ValidateShaderdescNoUbMembers,
    ValidateShaderdescUbUniformGlslName,
    ValidateShaderdescUbSizeMismatch,
    ValidateShaderdescUbArrayCount,
    ValidateShaderdescUbStd140ArrayType,
    ValidateShaderdescStoragebufferMetalBufferSlotOutOfRange,
    ValidateShaderdescStoragebufferMetalBufferSlotCollision,
    ValidateShaderdescStoragebufferHlslRegisterTOutOfRange,
    ValidateShaderdescStoragebufferHlslRegisterTCollision,
    ValidateShaderdescStoragebufferGlslBindingOutOfRange,
    ValidateShaderdescStoragebufferGlslBindingCollision,
    ValidateShaderdescStoragebufferWgslGroup1BindingOutOfRange,
    ValidateShaderdescStoragebufferWgslGroup1BindingCollision,
    ValidateShaderdescStoragebufferReadonly,
    ValidateShaderdescImageMetalTextureSlotOutOfRange,
    ValidateShaderdescImageMetalTextureSlotCollision,
    ValidateShaderdescImageHlslRegisterTOutOfRange,
    ValidateShaderdescImageHlslRegisterTCollision,
    ValidateShaderdescImageWgslGroup1BindingOutOfRange,
    ValidateShaderdescImageWgslGroup1BindingCollision,
    ValidateShaderdescSamplerMetalSamplerSlotOutOfRange,
    ValidateShaderdescSamplerMetalSamplerSlotCollision,
    ValidateShaderdescSamplerHlslRegisterSOutOfRange,
    ValidateShaderdescSamplerHlslRegisterSCollision,
    ValidateShaderdescSamplerWgslGroup1BindingOutOfRange,
    ValidateShaderdescSamplerWgslGroup1BindingCollision,
    ValidateShaderdescImageSamplerPairImageSlotOutOfRange,
    ValidateShaderdescImageSamplerPairSamplerSlotOutOfRange,
    ValidateShaderdescImageSamplerPairImageStageMismatch,
    ValidateShaderdescImageSamplerPairSamplerStageMismatch,
    ValidateShaderdescImageSamplerPairGlslName,
    ValidateShaderdescNonfilteringSamplerRequired,
    ValidateShaderdescComparisonSamplerRequired,
    ValidateShaderdescImageNotReferencedByImageSamplerPairs,
    ValidateShaderdescSamplerNotReferencedByImageSamplerPairs,
    ValidateShaderdescAttrStringTooLong,
    ValidatePipelinedescCanary,
    ValidatePipelinedescShader,
    ValidatePipelinedescNoContAttrs,
    ValidatePipelinedescLayoutStride4,
    ValidatePipelinedescAttrSemantics,
    ValidateAttachmentsdescCanary,
    ValidateAttachmentsdescNoAttachments,
    ValidateAttachmentsdescNoContColorAtts,
    ValidateAttachmentsdescImage,
    ValidateAttachmentsdescMiplevel,
    ValidateAttachmentsdescFace,
    ValidateAttachmentsdescLayer,
    ValidateAttachmentsdescSlice,
    ValidateAttachmentsdescImageNoRt,
    ValidateAttachmentsdescColorInvPixelformat,
    ValidateAttachmentsdescDepthInvPixelformat,
    ValidateAttachmentsdescImageSizes,
    ValidateAttachmentsdescImageSampleCounts,
    ValidateAttachmentsdescResolveColorImageMsaa,
    ValidateAttachmentsdescResolveImage,
    ValidateAttachmentsdescResolveSampleCount,
    ValidateAttachmentsdescResolveMiplevel,
    ValidateAttachmentsdescResolveFace,
    ValidateAttachmentsdescResolveLayer,
    ValidateAttachmentsdescResolveSlice,
    ValidateAttachmentsdescResolveImageNoRt,
    ValidateAttachmentsdescResolveImageSizes,
    ValidateAttachmentsdescResolveImageFormat,
    ValidateAttachmentsdescDepthImage,
    ValidateAttachmentsdescDepthMiplevel,
    ValidateAttachmentsdescDepthFace,
    ValidateAttachmentsdescDepthLayer,
    ValidateAttachmentsdescDepthSlice,
    ValidateAttachmentsdescDepthImageNoRt,
    ValidateAttachmentsdescDepthImageSizes,
    ValidateAttachmentsdescDepthImageSampleCount,
    ValidateBeginpassCanary,
    ValidateBeginpassAttachmentsExists,
    ValidateBeginpassAttachmentsValid,
    ValidateBeginpassColorAttachmentImage,
    ValidateBeginpassResolveAttachmentImage,
    ValidateBeginpassDepthstencilAttachmentImage,
    ValidateBeginpassSwapchainExpectWidth,
    ValidateBeginpassSwapchainExpectWidthNotset,
    ValidateBeginpassSwapchainExpectHeight,
    ValidateBeginpassSwapchainExpectHeightNotset,
    ValidateBeginpassSwapchainExpectSamplecount,
    ValidateBeginpassSwapchainExpectSamplecountNotset,
    ValidateBeginpassSwapchainExpectColorformat,
    ValidateBeginpassSwapchainExpectColorformatNotset,
    ValidateBeginpassSwapchainExpectDepthformatNotset,
    ValidateBeginpassSwapchainMetalExpectCurrentdrawable,
    ValidateBeginpassSwapchainMetalExpectCurrentdrawableNotset,
    ValidateBeginpassSwapchainMetalExpectDepthstenciltexture,
    ValidateBeginpassSwapchainMetalExpectDepthstenciltextureNotset,
    ValidateBeginpassSwapchainMetalExpectMsaacolortexture,
    ValidateBeginpassSwapchainMetalExpectMsaacolortextureNotset,
    ValidateBeginpassSwapchainD3d11ExpectRenderview,
    ValidateBeginpassSwapchainD3d11ExpectRenderviewNotset,
    ValidateBeginpassSwapchainD3d11ExpectResolveview,
    ValidateBeginpassSwapchainD3d11ExpectResolveviewNotset,
    ValidateBeginpassSwapchainD3d11ExpectDepthstencilview,
    ValidateBeginpassSwapchainD3d11ExpectDepthstencilviewNotset,
    ValidateBeginpassSwapchainWgpuExpectRenderview,
    ValidateBeginpassSwapchainWgpuExpectRenderviewNotset,
    ValidateBeginpassSwapchainWgpuExpectResolveview,
    ValidateBeginpassSwapchainWgpuExpectResolveviewNotset,
    ValidateBeginpassSwapchainWgpuExpectDepthstencilview,
    ValidateBeginpassSwapchainWgpuExpectDepthstencilviewNotset,
    ValidateBeginpassSwapchainGlExpectFramebufferNotset,
    ValidateApipPipelineValidId,
    ValidateApipPipelineExists,
    ValidateApipPipelineValid,
    ValidateApipShaderExists,
    ValidateApipShaderValid,
    ValidateApipCurpassAttachmentsExists,
    ValidateApipCurpassAttachmentsValid,
    ValidateApipAttCount,
    ValidateApipColorFormat,
    ValidateApipDepthFormat,
    ValidateApipSampleCount,
    ValidateAbndPipeline,
    ValidateAbndPipelineExists,
    ValidateAbndPipelineValid,
    ValidateAbndExpectedVb,
    ValidateAbndVbExists,
    ValidateAbndVbType,
    ValidateAbndVbOverflow,
    ValidateAbndNoIb,
    ValidateAbndIb,
    ValidateAbndIbExists,
    ValidateAbndIbType,
    ValidateAbndIbOverflow,
    ValidateAbndExpectedImageBinding,
    ValidateAbndImgExists,
    ValidateAbndImageTypeMismatch,
    ValidateAbndImageMsaa,
    ValidateAbndExpectedFilterableImage,
    ValidateAbndExpectedDepthImage,
    ValidateAbndExpectedSamplerBinding,
    ValidateAbndUnexpectedSamplerCompareNever,
    ValidateAbndExpectedSamplerCompareNever,
    ValidateAbndExpectedNonfilteringSampler,
    ValidateAbndSmpExists,
    ValidateAbndExpectedStoragebufferBinding,
    ValidateAbndStoragebufferExists,
    ValidateAbndStoragebufferBindingBuffertype,
    ValidateAubNoPipeline,
    ValidateAubNoUbAtSlot,
    ValidateAubSize,
    ValidateUpdatebufUsage,
    ValidateUpdatebufSize,
    ValidateUpdatebufOnce,
    ValidateUpdatebufAppend,
    ValidateAppendbufUsage,
    ValidateAppendbufSize,
    ValidateAppendbufUpdate,
    ValidateUpdimgUsage,
    ValidateUpdimgOnce,
    ValidationFailed,
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
pub struct EnvironmentDefaults {
    pub color_format: PixelFormat,
    pub depth_format: PixelFormat,
    pub sample_count: i32,
}
impl EnvironmentDefaults {
    pub const fn new() -> Self {
        Self {
            color_format: PixelFormat::new(),
            depth_format: PixelFormat::new(),
            sample_count: 0,
        }
    }
}
impl Default for EnvironmentDefaults {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MetalEnvironment {
    pub device: *const core::ffi::c_void,
}
impl MetalEnvironment {
    pub const fn new() -> Self {
        Self { device: core::ptr::null() }
    }
}
impl Default for MetalEnvironment {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3d11Environment {
    pub device: *const core::ffi::c_void,
    pub device_context: *const core::ffi::c_void,
}
impl D3d11Environment {
    pub const fn new() -> Self {
        Self { device: core::ptr::null(), device_context: core::ptr::null() }
    }
}
impl Default for D3d11Environment {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WgpuEnvironment {
    pub device: *const core::ffi::c_void,
}
impl WgpuEnvironment {
    pub const fn new() -> Self {
        Self { device: core::ptr::null() }
    }
}
impl Default for WgpuEnvironment {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Environment {
    pub defaults: EnvironmentDefaults,
    pub metal: MetalEnvironment,
    pub d3d11: D3d11Environment,
    pub wgpu: WgpuEnvironment,
}
impl Environment {
    pub const fn new() -> Self {
        Self {
            defaults: EnvironmentDefaults::new(),
            metal: MetalEnvironment::new(),
            d3d11: D3d11Environment::new(),
            wgpu: WgpuEnvironment::new(),
        }
    }
}
impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CommitListener {
    pub func: Option<extern "C" fn(*mut core::ffi::c_void)>,
    pub user_data: *mut core::ffi::c_void,
}
impl CommitListener {
    pub const fn new() -> Self {
        Self { func: None, user_data: core::ptr::null_mut() }
    }
}
impl Default for CommitListener {
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
    pub _start_canary: u32,
    pub buffer_pool_size: i32,
    pub image_pool_size: i32,
    pub sampler_pool_size: i32,
    pub shader_pool_size: i32,
    pub pipeline_pool_size: i32,
    pub attachments_pool_size: i32,
    pub uniform_buffer_size: i32,
    pub max_commit_listeners: i32,
    pub disable_validation: bool,
    pub d3d11_shader_debugging: bool,
    pub mtl_force_managed_storage_mode: bool,
    pub mtl_use_command_buffer_with_retained_references: bool,
    pub wgpu_disable_bindgroups_cache: bool,
    pub wgpu_bindgroups_cache_size: i32,
    pub allocator: Allocator,
    pub logger: Logger,
    pub environment: Environment,
    pub _end_canary: u32,
}
impl Desc {
    pub const fn new() -> Self {
        Self {
            _start_canary: 0,
            buffer_pool_size: 0,
            image_pool_size: 0,
            sampler_pool_size: 0,
            shader_pool_size: 0,
            pipeline_pool_size: 0,
            attachments_pool_size: 0,
            uniform_buffer_size: 0,
            max_commit_listeners: 0,
            disable_validation: false,
            d3d11_shader_debugging: false,
            mtl_force_managed_storage_mode: false,
            mtl_use_command_buffer_with_retained_references: false,
            wgpu_disable_bindgroups_cache: false,
            wgpu_bindgroups_cache_size: 0,
            allocator: Allocator::new(),
            logger: Logger::new(),
            environment: Environment::new(),
            _end_canary: 0,
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
pub struct D3d11BufferInfo {
    pub buf: *const core::ffi::c_void,
}
impl D3d11BufferInfo {
    pub const fn new() -> Self {
        Self { buf: core::ptr::null() }
    }
}
impl Default for D3d11BufferInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3d11ImageInfo {
    pub tex2d: *const core::ffi::c_void,
    pub tex3d: *const core::ffi::c_void,
    pub res: *const core::ffi::c_void,
    pub srv: *const core::ffi::c_void,
}
impl D3d11ImageInfo {
    pub const fn new() -> Self {
        Self {
            tex2d: core::ptr::null(),
            tex3d: core::ptr::null(),
            res: core::ptr::null(),
            srv: core::ptr::null(),
        }
    }
}
impl Default for D3d11ImageInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3d11SamplerInfo {
    pub smp: *const core::ffi::c_void,
}
impl D3d11SamplerInfo {
    pub const fn new() -> Self {
        Self { smp: core::ptr::null() }
    }
}
impl Default for D3d11SamplerInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3d11ShaderInfo {
    pub cbufs: [*const core::ffi::c_void; 8],
    pub vs: *const core::ffi::c_void,
    pub fs: *const core::ffi::c_void,
}
impl D3d11ShaderInfo {
    pub const fn new() -> Self {
        Self { cbufs: [core::ptr::null(); 8], vs: core::ptr::null(), fs: core::ptr::null() }
    }
}
impl Default for D3d11ShaderInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3d11PipelineInfo {
    pub il: *const core::ffi::c_void,
    pub rs: *const core::ffi::c_void,
    pub dss: *const core::ffi::c_void,
    pub bs: *const core::ffi::c_void,
}
impl D3d11PipelineInfo {
    pub const fn new() -> Self {
        Self {
            il: core::ptr::null(),
            rs: core::ptr::null(),
            dss: core::ptr::null(),
            bs: core::ptr::null(),
        }
    }
}
impl Default for D3d11PipelineInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3d11AttachmentsInfo {
    pub color_rtv: [*const core::ffi::c_void; 4],
    pub resolve_rtv: [*const core::ffi::c_void; 4],
    pub dsv: *const core::ffi::c_void,
}
impl D3d11AttachmentsInfo {
    pub const fn new() -> Self {
        Self {
            color_rtv: [core::ptr::null(); 4],
            resolve_rtv: [core::ptr::null(); 4],
            dsv: core::ptr::null(),
        }
    }
}
impl Default for D3d11AttachmentsInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MtlBufferInfo {
    pub buf: [*const core::ffi::c_void; 2],
    pub active_slot: i32,
}
impl MtlBufferInfo {
    pub const fn new() -> Self {
        Self { buf: [core::ptr::null(); 2], active_slot: 0 }
    }
}
impl Default for MtlBufferInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MtlImageInfo {
    pub tex: [*const core::ffi::c_void; 2],
    pub active_slot: i32,
}
impl MtlImageInfo {
    pub const fn new() -> Self {
        Self { tex: [core::ptr::null(); 2], active_slot: 0 }
    }
}
impl Default for MtlImageInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MtlSamplerInfo {
    pub smp: *const core::ffi::c_void,
}
impl MtlSamplerInfo {
    pub const fn new() -> Self {
        Self { smp: core::ptr::null() }
    }
}
impl Default for MtlSamplerInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MtlShaderInfo {
    pub vertex_lib: *const core::ffi::c_void,
    pub fragment_lib: *const core::ffi::c_void,
    pub vertex_func: *const core::ffi::c_void,
    pub fragment_func: *const core::ffi::c_void,
}
impl MtlShaderInfo {
    pub const fn new() -> Self {
        Self {
            vertex_lib: core::ptr::null(),
            fragment_lib: core::ptr::null(),
            vertex_func: core::ptr::null(),
            fragment_func: core::ptr::null(),
        }
    }
}
impl Default for MtlShaderInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MtlPipelineInfo {
    pub rps: *const core::ffi::c_void,
    pub dss: *const core::ffi::c_void,
}
impl MtlPipelineInfo {
    pub const fn new() -> Self {
        Self { rps: core::ptr::null(), dss: core::ptr::null() }
    }
}
impl Default for MtlPipelineInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WgpuBufferInfo {
    pub buf: *const core::ffi::c_void,
}
impl WgpuBufferInfo {
    pub const fn new() -> Self {
        Self { buf: core::ptr::null() }
    }
}
impl Default for WgpuBufferInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WgpuImageInfo {
    pub tex: *const core::ffi::c_void,
    pub view: *const core::ffi::c_void,
}
impl WgpuImageInfo {
    pub const fn new() -> Self {
        Self { tex: core::ptr::null(), view: core::ptr::null() }
    }
}
impl Default for WgpuImageInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WgpuSamplerInfo {
    pub smp: *const core::ffi::c_void,
}
impl WgpuSamplerInfo {
    pub const fn new() -> Self {
        Self { smp: core::ptr::null() }
    }
}
impl Default for WgpuSamplerInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WgpuShaderInfo {
    pub vs_mod: *const core::ffi::c_void,
    pub fs_mod: *const core::ffi::c_void,
    pub bgl: *const core::ffi::c_void,
}
impl WgpuShaderInfo {
    pub const fn new() -> Self {
        Self { vs_mod: core::ptr::null(), fs_mod: core::ptr::null(), bgl: core::ptr::null() }
    }
}
impl Default for WgpuShaderInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WgpuPipelineInfo {
    pub pip: *const core::ffi::c_void,
}
impl WgpuPipelineInfo {
    pub const fn new() -> Self {
        Self { pip: core::ptr::null() }
    }
}
impl Default for WgpuPipelineInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WgpuAttachmentsInfo {
    pub color_view: [*const core::ffi::c_void; 4],
    pub resolve_view: [*const core::ffi::c_void; 4],
    pub ds_view: *const core::ffi::c_void,
}
impl WgpuAttachmentsInfo {
    pub const fn new() -> Self {
        Self {
            color_view: [core::ptr::null(); 4],
            resolve_view: [core::ptr::null(); 4],
            ds_view: core::ptr::null(),
        }
    }
}
impl Default for WgpuAttachmentsInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GlBufferInfo {
    pub buf: [u32; 2],
    pub active_slot: i32,
}
impl GlBufferInfo {
    pub const fn new() -> Self {
        Self { buf: [0; 2], active_slot: 0 }
    }
}
impl Default for GlBufferInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GlImageInfo {
    pub tex: [u32; 2],
    pub tex_target: u32,
    pub msaa_render_buffer: u32,
    pub active_slot: i32,
}
impl GlImageInfo {
    pub const fn new() -> Self {
        Self { tex: [0; 2], tex_target: 0, msaa_render_buffer: 0, active_slot: 0 }
    }
}
impl Default for GlImageInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GlSamplerInfo {
    pub smp: u32,
}
impl GlSamplerInfo {
    pub const fn new() -> Self {
        Self { smp: 0 }
    }
}
impl Default for GlSamplerInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GlShaderInfo {
    pub prog: u32,
}
impl GlShaderInfo {
    pub const fn new() -> Self {
        Self { prog: 0 }
    }
}
impl Default for GlShaderInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GlAttachmentsInfo {
    pub framebuffer: u32,
    pub msaa_resolve_framebuffer: [u32; 4],
}
impl GlAttachmentsInfo {
    pub const fn new() -> Self {
        Self { framebuffer: 0, msaa_resolve_framebuffer: [0; 4] }
    }
}
impl Default for GlAttachmentsInfo {
    fn default() -> Self {
        Self::new()
    }
}
pub mod ffi {
    #![allow(unused_imports)]
    use super::*;
    extern "C" {
        pub fn sg_setup(desc: *const Desc);
        pub fn sg_shutdown();
        pub fn sg_isvalid() -> bool;
        pub fn sg_reset_state_cache();
        pub fn sg_install_trace_hooks(trace_hooks: *const TraceHooks) -> TraceHooks;
        pub fn sg_push_debug_group(name: *const core::ffi::c_char);
        pub fn sg_pop_debug_group();
        pub fn sg_add_commit_listener(listener: CommitListener) -> bool;
        pub fn sg_remove_commit_listener(listener: CommitListener) -> bool;
        pub fn sg_make_buffer(desc: *const BufferDesc) -> Buffer;
        pub fn sg_make_image(desc: *const ImageDesc) -> Image;
        pub fn sg_make_sampler(desc: *const SamplerDesc) -> Sampler;
        pub fn sg_make_shader(desc: *const ShaderDesc) -> Shader;
        pub fn sg_make_pipeline(desc: *const PipelineDesc) -> Pipeline;
        pub fn sg_make_attachments(desc: *const AttachmentsDesc) -> Attachments;
        pub fn sg_destroy_buffer(buf: Buffer);
        pub fn sg_destroy_image(img: Image);
        pub fn sg_destroy_sampler(smp: Sampler);
        pub fn sg_destroy_shader(shd: Shader);
        pub fn sg_destroy_pipeline(pip: Pipeline);
        pub fn sg_destroy_attachments(atts: Attachments);
        pub fn sg_update_buffer(buf: Buffer, data: *const Range);
        pub fn sg_update_image(img: Image, data: *const ImageData);
        pub fn sg_append_buffer(buf: Buffer, data: *const Range) -> i32;
        pub fn sg_query_buffer_overflow(buf: Buffer) -> bool;
        pub fn sg_query_buffer_will_overflow(buf: Buffer, size: usize) -> bool;
        pub fn sg_begin_pass(pass: *const Pass);
        pub fn sg_apply_viewport(x: i32, y: i32, width: i32, height: i32, origin_top_left: bool);
        pub fn sg_apply_viewportf(x: f32, y: f32, width: f32, height: f32, origin_top_left: bool);
        pub fn sg_apply_scissor_rect(x: i32, y: i32, width: i32, height: i32, origin_top_left: bool);
        pub fn sg_apply_scissor_rectf(x: f32, y: f32, width: f32, height: f32, origin_top_left: bool);
        pub fn sg_apply_pipeline(pip: Pipeline);
        pub fn sg_apply_bindings(bindings: *const Bindings);
        pub fn sg_apply_uniforms(ub_slot: usize, data: *const Range);
        pub fn sg_draw(base_element: usize, num_elements: usize, num_instances: usize);
        pub fn sg_end_pass();
        pub fn sg_commit();
        pub fn sg_query_desc() -> Desc;
        pub fn sg_query_backend() -> Backend;
        pub fn sg_query_features() -> Features;
        pub fn sg_query_limits() -> Limits;
        pub fn sg_query_pixelformat(fmt: PixelFormat) -> PixelformatInfo;
        pub fn sg_query_row_pitch(fmt: PixelFormat, width: i32, row_align_bytes: i32) -> i32;
        pub fn sg_query_surface_pitch(fmt: PixelFormat, width: i32, height: i32, row_align_bytes: i32)
            -> i32;
        pub fn sg_query_buffer_state(buf: Buffer) -> ResourceState;
        pub fn sg_query_image_state(img: Image) -> ResourceState;
        pub fn sg_query_sampler_state(smp: Sampler) -> ResourceState;
        pub fn sg_query_shader_state(shd: Shader) -> ResourceState;
        pub fn sg_query_pipeline_state(pip: Pipeline) -> ResourceState;
        pub fn sg_query_attachments_state(atts: Attachments) -> ResourceState;
        pub fn sg_query_buffer_info(buf: Buffer) -> BufferInfo;
        pub fn sg_query_image_info(img: Image) -> ImageInfo;
        pub fn sg_query_sampler_info(smp: Sampler) -> SamplerInfo;
        pub fn sg_query_shader_info(shd: Shader) -> ShaderInfo;
        pub fn sg_query_pipeline_info(pip: Pipeline) -> PipelineInfo;
        pub fn sg_query_attachments_info(atts: Attachments) -> AttachmentsInfo;
        pub fn sg_query_buffer_desc(buf: Buffer) -> BufferDesc;
        pub fn sg_query_image_desc(img: Image) -> ImageDesc;
        pub fn sg_query_sampler_desc(smp: Sampler) -> SamplerDesc;
        pub fn sg_query_shader_desc(shd: Shader) -> ShaderDesc;
        pub fn sg_query_pipeline_desc(pip: Pipeline) -> PipelineDesc;
        pub fn sg_query_attachments_desc(atts: Attachments) -> AttachmentsDesc;
        pub fn sg_query_buffer_defaults(desc: *const BufferDesc) -> BufferDesc;
        pub fn sg_query_image_defaults(desc: *const ImageDesc) -> ImageDesc;
        pub fn sg_query_sampler_defaults(desc: *const SamplerDesc) -> SamplerDesc;
        pub fn sg_query_shader_defaults(desc: *const ShaderDesc) -> ShaderDesc;
        pub fn sg_query_pipeline_defaults(desc: *const PipelineDesc) -> PipelineDesc;
        pub fn sg_query_attachments_defaults(desc: *const AttachmentsDesc) -> AttachmentsDesc;
        pub fn sg_alloc_buffer() -> Buffer;
        pub fn sg_alloc_image() -> Image;
        pub fn sg_alloc_sampler() -> Sampler;
        pub fn sg_alloc_shader() -> Shader;
        pub fn sg_alloc_pipeline() -> Pipeline;
        pub fn sg_alloc_attachments() -> Attachments;
        pub fn sg_dealloc_buffer(buf: Buffer);
        pub fn sg_dealloc_image(img: Image);
        pub fn sg_dealloc_sampler(smp: Sampler);
        pub fn sg_dealloc_shader(shd: Shader);
        pub fn sg_dealloc_pipeline(pip: Pipeline);
        pub fn sg_dealloc_attachments(attachments: Attachments);
        pub fn sg_init_buffer(buf: Buffer, desc: *const BufferDesc);
        pub fn sg_init_image(img: Image, desc: *const ImageDesc);
        pub fn sg_init_sampler(smg: Sampler, desc: *const SamplerDesc);
        pub fn sg_init_shader(shd: Shader, desc: *const ShaderDesc);
        pub fn sg_init_pipeline(pip: Pipeline, desc: *const PipelineDesc);
        pub fn sg_init_attachments(attachments: Attachments, desc: *const AttachmentsDesc);
        pub fn sg_uninit_buffer(buf: Buffer);
        pub fn sg_uninit_image(img: Image);
        pub fn sg_uninit_sampler(smp: Sampler);
        pub fn sg_uninit_shader(shd: Shader);
        pub fn sg_uninit_pipeline(pip: Pipeline);
        pub fn sg_uninit_attachments(atts: Attachments);
        pub fn sg_fail_buffer(buf: Buffer);
        pub fn sg_fail_image(img: Image);
        pub fn sg_fail_sampler(smp: Sampler);
        pub fn sg_fail_shader(shd: Shader);
        pub fn sg_fail_pipeline(pip: Pipeline);
        pub fn sg_fail_attachments(atts: Attachments);
        pub fn sg_enable_frame_stats();
        pub fn sg_disable_frame_stats();
        pub fn sg_frame_stats_enabled() -> bool;
        pub fn sg_query_frame_stats() -> FrameStats;
        pub fn sg_d3d11_device() -> *const core::ffi::c_void;
        pub fn sg_d3d11_device_context() -> *const core::ffi::c_void;
        pub fn sg_d3d11_query_buffer_info(buf: Buffer) -> D3d11BufferInfo;
        pub fn sg_d3d11_query_image_info(img: Image) -> D3d11ImageInfo;
        pub fn sg_d3d11_query_sampler_info(smp: Sampler) -> D3d11SamplerInfo;
        pub fn sg_d3d11_query_shader_info(shd: Shader) -> D3d11ShaderInfo;
        pub fn sg_d3d11_query_pipeline_info(pip: Pipeline) -> D3d11PipelineInfo;
        pub fn sg_d3d11_query_attachments_info(atts: Attachments) -> D3d11AttachmentsInfo;
        pub fn sg_mtl_device() -> *const core::ffi::c_void;
        pub fn sg_mtl_render_command_encoder() -> *const core::ffi::c_void;
        pub fn sg_mtl_query_buffer_info(buf: Buffer) -> MtlBufferInfo;
        pub fn sg_mtl_query_image_info(img: Image) -> MtlImageInfo;
        pub fn sg_mtl_query_sampler_info(smp: Sampler) -> MtlSamplerInfo;
        pub fn sg_mtl_query_shader_info(shd: Shader) -> MtlShaderInfo;
        pub fn sg_mtl_query_pipeline_info(pip: Pipeline) -> MtlPipelineInfo;
        pub fn sg_wgpu_device() -> *const core::ffi::c_void;
        pub fn sg_wgpu_queue() -> *const core::ffi::c_void;
        pub fn sg_wgpu_command_encoder() -> *const core::ffi::c_void;
        pub fn sg_wgpu_render_pass_encoder() -> *const core::ffi::c_void;
        pub fn sg_wgpu_query_buffer_info(buf: Buffer) -> WgpuBufferInfo;
        pub fn sg_wgpu_query_image_info(img: Image) -> WgpuImageInfo;
        pub fn sg_wgpu_query_sampler_info(smp: Sampler) -> WgpuSamplerInfo;
        pub fn sg_wgpu_query_shader_info(shd: Shader) -> WgpuShaderInfo;
        pub fn sg_wgpu_query_pipeline_info(pip: Pipeline) -> WgpuPipelineInfo;
        pub fn sg_wgpu_query_attachments_info(atts: Attachments) -> WgpuAttachmentsInfo;
        pub fn sg_gl_query_buffer_info(buf: Buffer) -> GlBufferInfo;
        pub fn sg_gl_query_image_info(img: Image) -> GlImageInfo;
        pub fn sg_gl_query_sampler_info(smp: Sampler) -> GlSamplerInfo;
        pub fn sg_gl_query_shader_info(shd: Shader) -> GlShaderInfo;
        pub fn sg_gl_query_attachments_info(atts: Attachments) -> GlAttachmentsInfo;
    }
}
#[inline]
pub fn setup(desc: &Desc) {
    unsafe { ffi::sg_setup(desc) }
}
#[inline]
pub fn shutdown() {
    unsafe { ffi::sg_shutdown() }
}
#[inline]
pub fn isvalid() -> bool {
    unsafe { ffi::sg_isvalid() }
}
#[inline]
pub fn reset_state_cache() {
    unsafe { ffi::sg_reset_state_cache() }
}
#[inline]
pub fn install_trace_hooks(trace_hooks: &TraceHooks) -> TraceHooks {
    unsafe { ffi::sg_install_trace_hooks(trace_hooks) }
}
#[inline]
pub fn push_debug_group(name: &str) {
    let tmp_0 = std::ffi::CString::new(name).unwrap();
    unsafe { ffi::sg_push_debug_group(tmp_0.as_ptr()) }
}
#[inline]
pub fn pop_debug_group() {
    unsafe { ffi::sg_pop_debug_group() }
}
#[inline]
pub fn add_commit_listener(listener: CommitListener) -> bool {
    unsafe { ffi::sg_add_commit_listener(listener) }
}
#[inline]
pub fn remove_commit_listener(listener: CommitListener) -> bool {
    unsafe { ffi::sg_remove_commit_listener(listener) }
}
#[inline]
pub fn make_buffer(desc: &BufferDesc) -> Buffer {
    unsafe { ffi::sg_make_buffer(desc) }
}
#[inline]
pub fn make_image(desc: &ImageDesc) -> Image {
    unsafe { ffi::sg_make_image(desc) }
}
#[inline]
pub fn make_sampler(desc: &SamplerDesc) -> Sampler {
    unsafe { ffi::sg_make_sampler(desc) }
}
#[inline]
pub fn make_shader(desc: &ShaderDesc) -> Shader {
    unsafe { ffi::sg_make_shader(desc) }
}
#[inline]
pub fn make_pipeline(desc: &PipelineDesc) -> Pipeline {
    unsafe { ffi::sg_make_pipeline(desc) }
}
#[inline]
pub fn make_attachments(desc: &AttachmentsDesc) -> Attachments {
    unsafe { ffi::sg_make_attachments(desc) }
}
#[inline]
pub fn destroy_buffer(buf: Buffer) {
    unsafe { ffi::sg_destroy_buffer(buf) }
}
#[inline]
pub fn destroy_image(img: Image) {
    unsafe { ffi::sg_destroy_image(img) }
}
#[inline]
pub fn destroy_sampler(smp: Sampler) {
    unsafe { ffi::sg_destroy_sampler(smp) }
}
#[inline]
pub fn destroy_shader(shd: Shader) {
    unsafe { ffi::sg_destroy_shader(shd) }
}
#[inline]
pub fn destroy_pipeline(pip: Pipeline) {
    unsafe { ffi::sg_destroy_pipeline(pip) }
}
#[inline]
pub fn destroy_attachments(atts: Attachments) {
    unsafe { ffi::sg_destroy_attachments(atts) }
}
#[inline]
pub fn update_buffer(buf: Buffer, data: &Range) {
    unsafe { ffi::sg_update_buffer(buf, data) }
}
#[inline]
pub fn update_image(img: Image, data: &ImageData) {
    unsafe { ffi::sg_update_image(img, data) }
}
#[inline]
pub fn append_buffer(buf: Buffer, data: &Range) -> i32 {
    unsafe { ffi::sg_append_buffer(buf, data) }
}
#[inline]
pub fn query_buffer_overflow(buf: Buffer) -> bool {
    unsafe { ffi::sg_query_buffer_overflow(buf) }
}
#[inline]
pub fn query_buffer_will_overflow(buf: Buffer, size: usize) -> bool {
    unsafe { ffi::sg_query_buffer_will_overflow(buf, size) }
}
#[inline]
pub fn begin_pass(pass: &Pass) {
    unsafe { ffi::sg_begin_pass(pass) }
}
#[inline]
pub fn apply_viewport(x: i32, y: i32, width: i32, height: i32, origin_top_left: bool) {
    unsafe { ffi::sg_apply_viewport(x, y, width, height, origin_top_left) }
}
#[inline]
pub fn apply_viewportf(x: f32, y: f32, width: f32, height: f32, origin_top_left: bool) {
    unsafe { ffi::sg_apply_viewportf(x, y, width, height, origin_top_left) }
}
#[inline]
pub fn apply_scissor_rect(x: i32, y: i32, width: i32, height: i32, origin_top_left: bool) {
    unsafe { ffi::sg_apply_scissor_rect(x, y, width, height, origin_top_left) }
}
#[inline]
pub fn apply_scissor_rectf(x: f32, y: f32, width: f32, height: f32, origin_top_left: bool) {
    unsafe { ffi::sg_apply_scissor_rectf(x, y, width, height, origin_top_left) }
}
#[inline]
pub fn apply_pipeline(pip: Pipeline) {
    unsafe { ffi::sg_apply_pipeline(pip) }
}
#[inline]
pub fn apply_bindings(bindings: &Bindings) {
    unsafe { ffi::sg_apply_bindings(bindings) }
}
#[inline]
pub fn apply_uniforms(ub_slot: usize, data: &Range) {
    unsafe { ffi::sg_apply_uniforms(ub_slot, data) }
}
#[inline]
pub fn draw(base_element: usize, num_elements: usize, num_instances: usize) {
    unsafe { ffi::sg_draw(base_element, num_elements, num_instances) }
}
#[inline]
pub fn end_pass() {
    unsafe { ffi::sg_end_pass() }
}
#[inline]
pub fn commit() {
    unsafe { ffi::sg_commit() }
}
#[inline]
pub fn query_desc() -> Desc {
    unsafe { ffi::sg_query_desc() }
}
#[inline]
pub fn query_backend() -> Backend {
    unsafe { ffi::sg_query_backend() }
}
#[inline]
pub fn query_features() -> Features {
    unsafe { ffi::sg_query_features() }
}
#[inline]
pub fn query_limits() -> Limits {
    unsafe { ffi::sg_query_limits() }
}
#[inline]
pub fn query_pixelformat(fmt: PixelFormat) -> PixelformatInfo {
    unsafe { ffi::sg_query_pixelformat(fmt) }
}
#[inline]
pub fn query_row_pitch(fmt: PixelFormat, width: i32, row_align_bytes: i32) -> i32 {
    unsafe { ffi::sg_query_row_pitch(fmt, width, row_align_bytes) }
}
#[inline]
pub fn query_surface_pitch(fmt: PixelFormat, width: i32, height: i32, row_align_bytes: i32) -> i32 {
    unsafe { ffi::sg_query_surface_pitch(fmt, width, height, row_align_bytes) }
}
#[inline]
pub fn query_buffer_state(buf: Buffer) -> ResourceState {
    unsafe { ffi::sg_query_buffer_state(buf) }
}
#[inline]
pub fn query_image_state(img: Image) -> ResourceState {
    unsafe { ffi::sg_query_image_state(img) }
}
#[inline]
pub fn query_sampler_state(smp: Sampler) -> ResourceState {
    unsafe { ffi::sg_query_sampler_state(smp) }
}
#[inline]
pub fn query_shader_state(shd: Shader) -> ResourceState {
    unsafe { ffi::sg_query_shader_state(shd) }
}
#[inline]
pub fn query_pipeline_state(pip: Pipeline) -> ResourceState {
    unsafe { ffi::sg_query_pipeline_state(pip) }
}
#[inline]
pub fn query_attachments_state(atts: Attachments) -> ResourceState {
    unsafe { ffi::sg_query_attachments_state(atts) }
}
#[inline]
pub fn query_buffer_info(buf: Buffer) -> BufferInfo {
    unsafe { ffi::sg_query_buffer_info(buf) }
}
#[inline]
pub fn query_image_info(img: Image) -> ImageInfo {
    unsafe { ffi::sg_query_image_info(img) }
}
#[inline]
pub fn query_sampler_info(smp: Sampler) -> SamplerInfo {
    unsafe { ffi::sg_query_sampler_info(smp) }
}
#[inline]
pub fn query_shader_info(shd: Shader) -> ShaderInfo {
    unsafe { ffi::sg_query_shader_info(shd) }
}
#[inline]
pub fn query_pipeline_info(pip: Pipeline) -> PipelineInfo {
    unsafe { ffi::sg_query_pipeline_info(pip) }
}
#[inline]
pub fn query_attachments_info(atts: Attachments) -> AttachmentsInfo {
    unsafe { ffi::sg_query_attachments_info(atts) }
}
#[inline]
pub fn query_buffer_desc(buf: Buffer) -> BufferDesc {
    unsafe { ffi::sg_query_buffer_desc(buf) }
}
#[inline]
pub fn query_image_desc(img: Image) -> ImageDesc {
    unsafe { ffi::sg_query_image_desc(img) }
}
#[inline]
pub fn query_sampler_desc(smp: Sampler) -> SamplerDesc {
    unsafe { ffi::sg_query_sampler_desc(smp) }
}
#[inline]
pub fn query_shader_desc(shd: Shader) -> ShaderDesc {
    unsafe { ffi::sg_query_shader_desc(shd) }
}
#[inline]
pub fn query_pipeline_desc(pip: Pipeline) -> PipelineDesc {
    unsafe { ffi::sg_query_pipeline_desc(pip) }
}
#[inline]
pub fn query_attachments_desc(atts: Attachments) -> AttachmentsDesc {
    unsafe { ffi::sg_query_attachments_desc(atts) }
}
#[inline]
pub fn query_buffer_defaults(desc: &BufferDesc) -> BufferDesc {
    unsafe { ffi::sg_query_buffer_defaults(desc) }
}
#[inline]
pub fn query_image_defaults(desc: &ImageDesc) -> ImageDesc {
    unsafe { ffi::sg_query_image_defaults(desc) }
}
#[inline]
pub fn query_sampler_defaults(desc: &SamplerDesc) -> SamplerDesc {
    unsafe { ffi::sg_query_sampler_defaults(desc) }
}
#[inline]
pub fn query_shader_defaults(desc: &ShaderDesc) -> ShaderDesc {
    unsafe { ffi::sg_query_shader_defaults(desc) }
}
#[inline]
pub fn query_pipeline_defaults(desc: &PipelineDesc) -> PipelineDesc {
    unsafe { ffi::sg_query_pipeline_defaults(desc) }
}
#[inline]
pub fn query_attachments_defaults(desc: &AttachmentsDesc) -> AttachmentsDesc {
    unsafe { ffi::sg_query_attachments_defaults(desc) }
}
#[inline]
pub fn alloc_buffer() -> Buffer {
    unsafe { ffi::sg_alloc_buffer() }
}
#[inline]
pub fn alloc_image() -> Image {
    unsafe { ffi::sg_alloc_image() }
}
#[inline]
pub fn alloc_sampler() -> Sampler {
    unsafe { ffi::sg_alloc_sampler() }
}
#[inline]
pub fn alloc_shader() -> Shader {
    unsafe { ffi::sg_alloc_shader() }
}
#[inline]
pub fn alloc_pipeline() -> Pipeline {
    unsafe { ffi::sg_alloc_pipeline() }
}
#[inline]
pub fn alloc_attachments() -> Attachments {
    unsafe { ffi::sg_alloc_attachments() }
}
#[inline]
pub fn dealloc_buffer(buf: Buffer) {
    unsafe { ffi::sg_dealloc_buffer(buf) }
}
#[inline]
pub fn dealloc_image(img: Image) {
    unsafe { ffi::sg_dealloc_image(img) }
}
#[inline]
pub fn dealloc_sampler(smp: Sampler) {
    unsafe { ffi::sg_dealloc_sampler(smp) }
}
#[inline]
pub fn dealloc_shader(shd: Shader) {
    unsafe { ffi::sg_dealloc_shader(shd) }
}
#[inline]
pub fn dealloc_pipeline(pip: Pipeline) {
    unsafe { ffi::sg_dealloc_pipeline(pip) }
}
#[inline]
pub fn dealloc_attachments(attachments: Attachments) {
    unsafe { ffi::sg_dealloc_attachments(attachments) }
}
#[inline]
pub fn init_buffer(buf: Buffer, desc: &BufferDesc) {
    unsafe { ffi::sg_init_buffer(buf, desc) }
}
#[inline]
pub fn init_image(img: Image, desc: &ImageDesc) {
    unsafe { ffi::sg_init_image(img, desc) }
}
#[inline]
pub fn init_sampler(smg: Sampler, desc: &SamplerDesc) {
    unsafe { ffi::sg_init_sampler(smg, desc) }
}
#[inline]
pub fn init_shader(shd: Shader, desc: &ShaderDesc) {
    unsafe { ffi::sg_init_shader(shd, desc) }
}
#[inline]
pub fn init_pipeline(pip: Pipeline, desc: &PipelineDesc) {
    unsafe { ffi::sg_init_pipeline(pip, desc) }
}
#[inline]
pub fn init_attachments(attachments: Attachments, desc: &AttachmentsDesc) {
    unsafe { ffi::sg_init_attachments(attachments, desc) }
}
#[inline]
pub fn uninit_buffer(buf: Buffer) {
    unsafe { ffi::sg_uninit_buffer(buf) }
}
#[inline]
pub fn uninit_image(img: Image) {
    unsafe { ffi::sg_uninit_image(img) }
}
#[inline]
pub fn uninit_sampler(smp: Sampler) {
    unsafe { ffi::sg_uninit_sampler(smp) }
}
#[inline]
pub fn uninit_shader(shd: Shader) {
    unsafe { ffi::sg_uninit_shader(shd) }
}
#[inline]
pub fn uninit_pipeline(pip: Pipeline) {
    unsafe { ffi::sg_uninit_pipeline(pip) }
}
#[inline]
pub fn uninit_attachments(atts: Attachments) {
    unsafe { ffi::sg_uninit_attachments(atts) }
}
#[inline]
pub fn fail_buffer(buf: Buffer) {
    unsafe { ffi::sg_fail_buffer(buf) }
}
#[inline]
pub fn fail_image(img: Image) {
    unsafe { ffi::sg_fail_image(img) }
}
#[inline]
pub fn fail_sampler(smp: Sampler) {
    unsafe { ffi::sg_fail_sampler(smp) }
}
#[inline]
pub fn fail_shader(shd: Shader) {
    unsafe { ffi::sg_fail_shader(shd) }
}
#[inline]
pub fn fail_pipeline(pip: Pipeline) {
    unsafe { ffi::sg_fail_pipeline(pip) }
}
#[inline]
pub fn fail_attachments(atts: Attachments) {
    unsafe { ffi::sg_fail_attachments(atts) }
}
#[inline]
pub fn enable_frame_stats() {
    unsafe { ffi::sg_enable_frame_stats() }
}
#[inline]
pub fn disable_frame_stats() {
    unsafe { ffi::sg_disable_frame_stats() }
}
#[inline]
pub fn frame_stats_enabled() -> bool {
    unsafe { ffi::sg_frame_stats_enabled() }
}
#[inline]
pub fn query_frame_stats() -> FrameStats {
    unsafe { ffi::sg_query_frame_stats() }
}
#[inline]
pub fn d3d11_device() -> *const core::ffi::c_void {
    unsafe { ffi::sg_d3d11_device() }
}
#[inline]
pub fn d3d11_device_context() -> *const core::ffi::c_void {
    unsafe { ffi::sg_d3d11_device_context() }
}
#[inline]
pub fn d3d11_query_buffer_info(buf: Buffer) -> D3d11BufferInfo {
    unsafe { ffi::sg_d3d11_query_buffer_info(buf) }
}
#[inline]
pub fn d3d11_query_image_info(img: Image) -> D3d11ImageInfo {
    unsafe { ffi::sg_d3d11_query_image_info(img) }
}
#[inline]
pub fn d3d11_query_sampler_info(smp: Sampler) -> D3d11SamplerInfo {
    unsafe { ffi::sg_d3d11_query_sampler_info(smp) }
}
#[inline]
pub fn d3d11_query_shader_info(shd: Shader) -> D3d11ShaderInfo {
    unsafe { ffi::sg_d3d11_query_shader_info(shd) }
}
#[inline]
pub fn d3d11_query_pipeline_info(pip: Pipeline) -> D3d11PipelineInfo {
    unsafe { ffi::sg_d3d11_query_pipeline_info(pip) }
}
#[inline]
pub fn d3d11_query_attachments_info(atts: Attachments) -> D3d11AttachmentsInfo {
    unsafe { ffi::sg_d3d11_query_attachments_info(atts) }
}
#[inline]
pub fn mtl_device() -> *const core::ffi::c_void {
    unsafe { ffi::sg_mtl_device() }
}
#[inline]
pub fn mtl_render_command_encoder() -> *const core::ffi::c_void {
    unsafe { ffi::sg_mtl_render_command_encoder() }
}
#[inline]
pub fn mtl_query_buffer_info(buf: Buffer) -> MtlBufferInfo {
    unsafe { ffi::sg_mtl_query_buffer_info(buf) }
}
#[inline]
pub fn mtl_query_image_info(img: Image) -> MtlImageInfo {
    unsafe { ffi::sg_mtl_query_image_info(img) }
}
#[inline]
pub fn mtl_query_sampler_info(smp: Sampler) -> MtlSamplerInfo {
    unsafe { ffi::sg_mtl_query_sampler_info(smp) }
}
#[inline]
pub fn mtl_query_shader_info(shd: Shader) -> MtlShaderInfo {
    unsafe { ffi::sg_mtl_query_shader_info(shd) }
}
#[inline]
pub fn mtl_query_pipeline_info(pip: Pipeline) -> MtlPipelineInfo {
    unsafe { ffi::sg_mtl_query_pipeline_info(pip) }
}
#[inline]
pub fn wgpu_device() -> *const core::ffi::c_void {
    unsafe { ffi::sg_wgpu_device() }
}
#[inline]
pub fn wgpu_queue() -> *const core::ffi::c_void {
    unsafe { ffi::sg_wgpu_queue() }
}
#[inline]
pub fn wgpu_command_encoder() -> *const core::ffi::c_void {
    unsafe { ffi::sg_wgpu_command_encoder() }
}
#[inline]
pub fn wgpu_render_pass_encoder() -> *const core::ffi::c_void {
    unsafe { ffi::sg_wgpu_render_pass_encoder() }
}
#[inline]
pub fn wgpu_query_buffer_info(buf: Buffer) -> WgpuBufferInfo {
    unsafe { ffi::sg_wgpu_query_buffer_info(buf) }
}
#[inline]
pub fn wgpu_query_image_info(img: Image) -> WgpuImageInfo {
    unsafe { ffi::sg_wgpu_query_image_info(img) }
}
#[inline]
pub fn wgpu_query_sampler_info(smp: Sampler) -> WgpuSamplerInfo {
    unsafe { ffi::sg_wgpu_query_sampler_info(smp) }
}
#[inline]
pub fn wgpu_query_shader_info(shd: Shader) -> WgpuShaderInfo {
    unsafe { ffi::sg_wgpu_query_shader_info(shd) }
}
#[inline]
pub fn wgpu_query_pipeline_info(pip: Pipeline) -> WgpuPipelineInfo {
    unsafe { ffi::sg_wgpu_query_pipeline_info(pip) }
}
#[inline]
pub fn wgpu_query_attachments_info(atts: Attachments) -> WgpuAttachmentsInfo {
    unsafe { ffi::sg_wgpu_query_attachments_info(atts) }
}
#[inline]
pub fn gl_query_buffer_info(buf: Buffer) -> GlBufferInfo {
    unsafe { ffi::sg_gl_query_buffer_info(buf) }
}
#[inline]
pub fn gl_query_image_info(img: Image) -> GlImageInfo {
    unsafe { ffi::sg_gl_query_image_info(img) }
}
#[inline]
pub fn gl_query_sampler_info(smp: Sampler) -> GlSamplerInfo {
    unsafe { ffi::sg_gl_query_sampler_info(smp) }
}
#[inline]
pub fn gl_query_shader_info(shd: Shader) -> GlShaderInfo {
    unsafe { ffi::sg_gl_query_shader_info(shd) }
}
#[inline]
pub fn gl_query_attachments_info(atts: Attachments) -> GlAttachmentsInfo {
    unsafe { ffi::sg_gl_query_attachments_info(atts) }
}
