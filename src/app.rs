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

pub const MAX_TOUCHPOINTS: usize = 8;
pub const MAX_MOUSEBUTTONS: usize = 3;
pub const MAX_KEYCODES: usize = 512;
pub const MAX_ICONIMAGES: usize = 8;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum EventType {
    Invalid,
    KeyDown,
    KeyUp,
    Char,
    MouseDown,
    MouseUp,
    MouseScroll,
    MouseMove,
    MouseEnter,
    MouseLeave,
    TouchesBegan,
    TouchesMoved,
    TouchesEnded,
    TouchesCancelled,
    Resized,
    Iconified,
    Restored,
    Focused,
    Unfocused,
    Suspended,
    Resumed,
    QuitRequested,
    ClipboardPasted,
    FilesDropped,
    Num,
}
impl EventType {
    pub const fn new() -> Self {
        Self::Invalid
    }
}
impl Default for EventType {
    fn default() -> Self {
        Self::Invalid
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum Keycode {
    Invalid = 0,
    Space = 32,
    Apostrophe = 39,
    Comma = 44,
    Minus = 45,
    Period = 46,
    Slash = 47,
    Num0 = 48,
    Num1 = 49,
    Num2 = 50,
    Num3 = 51,
    Num4 = 52,
    Num5 = 53,
    Num6 = 54,
    Num7 = 55,
    Num8 = 56,
    Num9 = 57,
    Semicolon = 59,
    Equal = 61,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    LeftBracket = 91,
    Backslash = 92,
    RightBracket = 93,
    GraveAccent = 96,
    World1 = 161,
    World2 = 162,
    Escape = 256,
    Enter = 257,
    Tab = 258,
    Backspace = 259,
    Insert = 260,
    Delete = 261,
    Right = 262,
    Left = 263,
    Down = 264,
    Up = 265,
    PageUp = 266,
    PageDown = 267,
    Home = 268,
    End = 269,
    CapsLock = 280,
    ScrollLock = 281,
    NumLock = 282,
    PrintScreen = 283,
    Pause = 284,
    F1 = 290,
    F2 = 291,
    F3 = 292,
    F4 = 293,
    F5 = 294,
    F6 = 295,
    F7 = 296,
    F8 = 297,
    F9 = 298,
    F10 = 299,
    F11 = 300,
    F12 = 301,
    F13 = 302,
    F14 = 303,
    F15 = 304,
    F16 = 305,
    F17 = 306,
    F18 = 307,
    F19 = 308,
    F20 = 309,
    F21 = 310,
    F22 = 311,
    F23 = 312,
    F24 = 313,
    F25 = 314,
    Kp0 = 320,
    Kp1 = 321,
    Kp2 = 322,
    Kp3 = 323,
    Kp4 = 324,
    Kp5 = 325,
    Kp6 = 326,
    Kp7 = 327,
    Kp8 = 328,
    Kp9 = 329,
    KpDecimal = 330,
    KpDivide = 331,
    KpMultiply = 332,
    KpSubtract = 333,
    KpAdd = 334,
    KpEnter = 335,
    KpEqual = 336,
    LeftShift = 340,
    LeftControl = 341,
    LeftAlt = 342,
    LeftSuper = 343,
    RightShift = 344,
    RightControl = 345,
    RightAlt = 346,
    RightSuper = 347,
    Menu = 348,
}
impl Keycode {
    pub const fn new() -> Self {
        Self::Invalid
    }
}
impl Default for Keycode {
    fn default() -> Self {
        Self::Invalid
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum AndroidTooltype {
    Unknown = 0,
    Finger = 1,
    Stylus = 2,
    Mouse = 3,
}
impl AndroidTooltype {
    pub const fn new() -> Self {
        Self::Unknown
    }
}
impl Default for AndroidTooltype {
    fn default() -> Self {
        Self::Unknown
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Touchpoint {
    pub identifier: usize,
    pub pos_x: f32,
    pub pos_y: f32,
    pub android_tooltype: AndroidTooltype,
    pub changed: bool,
}
impl Touchpoint {
    pub const fn new() -> Self {
        Self {
            identifier: 0,
            pos_x: 0.0,
            pos_y: 0.0,
            android_tooltype: AndroidTooltype::new(),
            changed: false,
        }
    }
}
impl Default for Touchpoint {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum Mousebutton {
    Left = 0,
    Right = 1,
    Middle = 2,
    Invalid = 256,
}
impl Mousebutton {
    pub const fn new() -> Self {
        Self::Left
    }
}
impl Default for Mousebutton {
    fn default() -> Self {
        Self::Left
    }
}
pub const MODIFIER_SHIFT: u32 = 1;
pub const MODIFIER_CTRL: u32 = 2;
pub const MODIFIER_ALT: u32 = 4;
pub const MODIFIER_SUPER: u32 = 8;
pub const MODIFIER_LMB: u32 = 256;
pub const MODIFIER_RMB: u32 = 512;
pub const MODIFIER_MMB: u32 = 1024;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Event {
    pub frame_count: u64,
    pub _type: EventType,
    pub key_code: Keycode,
    pub char_code: u32,
    pub key_repeat: bool,
    pub modifiers: u32,
    pub mouse_button: Mousebutton,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_dx: f32,
    pub mouse_dy: f32,
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub num_touches: i32,
    pub touches: [Touchpoint; 8],
    pub window_width: i32,
    pub window_height: i32,
    pub framebuffer_width: i32,
    pub framebuffer_height: i32,
}
impl Event {
    pub const fn new() -> Self {
        Self {
            frame_count: 0,
            _type: EventType::new(),
            key_code: Keycode::new(),
            char_code: 0,
            key_repeat: false,
            modifiers: 0,
            mouse_button: Mousebutton::new(),
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_dx: 0.0,
            mouse_dy: 0.0,
            scroll_x: 0.0,
            scroll_y: 0.0,
            num_touches: 0,
            touches: [Touchpoint::new(); 8],
            window_width: 0,
            window_height: 0,
            framebuffer_width: 0,
            framebuffer_height: 0,
        }
    }
}
impl Default for Event {
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
pub struct ImageDesc {
    pub width: i32,
    pub height: i32,
    pub cursor_hotspot_x: i32,
    pub cursor_hotspot_y: i32,
    pub pixels: Range,
}
impl ImageDesc {
    pub const fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            cursor_hotspot_x: 0,
            cursor_hotspot_y: 0,
            pixels: Range::new(),
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
pub struct IconDesc {
    pub sokol_default: bool,
    pub images: [ImageDesc; 8],
}
impl IconDesc {
    pub const fn new() -> Self {
        Self { sokol_default: false, images: [ImageDesc::new(); 8] }
    }
}
impl Default for IconDesc {
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
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum LogItem {
    Ok,
    MallocFailed,
    MacosInvalidNsopenglProfile,
    Win32LoadOpengl32DllFailed,
    Win32CreateHelperWindowFailed,
    Win32HelperWindowGetdcFailed,
    Win32DummyContextSetPixelformatFailed,
    Win32CreateDummyContextFailed,
    Win32DummyContextMakeCurrentFailed,
    Win32GetPixelformatAttribFailed,
    Win32WglFindPixelformatFailed,
    Win32WglDescribePixelformatFailed,
    Win32WglSetPixelformatFailed,
    Win32WglArbCreateContextRequired,
    Win32WglArbCreateContextProfileRequired,
    Win32WglOpenglVersionNotSupported,
    Win32WglOpenglProfileNotSupported,
    Win32WglIncompatibleDeviceContext,
    Win32WglCreateContextAttribsFailedOther,
    Win32D3d11CreateDeviceAndSwapchainWithDebugFailed,
    Win32D3d11GetIdxgifactoryFailed,
    Win32D3d11GetIdxgiadapterFailed,
    Win32D3d11QueryInterfaceIdxgidevice1Failed,
    Win32RegisterRawInputDevicesFailedMouseLock,
    Win32RegisterRawInputDevicesFailedMouseUnlock,
    Win32GetRawInputDataFailed,
    Win32DestroyiconForCursorFailed,
    LinuxGlxLoadLibglFailed,
    LinuxGlxLoadEntryPointsFailed,
    LinuxGlxExtensionNotFound,
    LinuxGlxQueryVersionFailed,
    LinuxGlxVersionTooLow,
    LinuxGlxNoGlxfbconfigs,
    LinuxGlxNoSuitableGlxfbconfig,
    LinuxGlxGetVisualFromFbconfigFailed,
    LinuxGlxRequiredExtensionsMissing,
    LinuxGlxCreateContextFailed,
    LinuxGlxCreateWindowFailed,
    LinuxX11CreateWindowFailed,
    LinuxEglBindOpenglApiFailed,
    LinuxEglBindOpenglEsApiFailed,
    LinuxEglGetDisplayFailed,
    LinuxEglInitializeFailed,
    LinuxEglNoConfigs,
    LinuxEglNoNativeVisual,
    LinuxEglGetVisualInfoFailed,
    LinuxEglCreateWindowSurfaceFailed,
    LinuxEglCreateContextFailed,
    LinuxEglMakeCurrentFailed,
    LinuxX11OpenDisplayFailed,
    LinuxX11QuerySystemDpiFailed,
    LinuxX11DroppedFileUriWrongScheme,
    LinuxX11FailedToBecomeOwnerOfClipboard,
    AndroidUnsupportedInputEventInputCb,
    AndroidUnsupportedInputEventMainCb,
    AndroidReadMsgFailed,
    AndroidWriteMsgFailed,
    AndroidMsgCreate,
    AndroidMsgResume,
    AndroidMsgPause,
    AndroidMsgFocus,
    AndroidMsgNoFocus,
    AndroidMsgSetNativeWindow,
    AndroidMsgSetInputQueue,
    AndroidMsgDestroy,
    AndroidUnknownMsg,
    AndroidLoopThreadStarted,
    AndroidLoopThreadDone,
    AndroidNativeActivityOnstart,
    AndroidNativeActivityOnresume,
    AndroidNativeActivityOnsaveinstancestate,
    AndroidNativeActivityOnwindowfocuschanged,
    AndroidNativeActivityOnpause,
    AndroidNativeActivityOnstop,
    AndroidNativeActivityOnnativewindowcreated,
    AndroidNativeActivityOnnativewindowdestroyed,
    AndroidNativeActivityOninputqueuecreated,
    AndroidNativeActivityOninputqueuedestroyed,
    AndroidNativeActivityOnconfigurationchanged,
    AndroidNativeActivityOnlowmemory,
    AndroidNativeActivityOndestroy,
    AndroidNativeActivityDone,
    AndroidNativeActivityOncreate,
    AndroidCreateThreadPipeFailed,
    AndroidNativeActivityCreateSuccess,
    WgpuDeviceLost,
    WgpuDeviceLog,
    WgpuDeviceUncapturedError,
    WgpuSwapchainCreateSurfaceFailed,
    WgpuSwapchainSurfaceGetCapabilitiesFailed,
    WgpuSwapchainCreateDepthStencilTextureFailed,
    WgpuSwapchainCreateDepthStencilViewFailed,
    WgpuSwapchainCreateMsaaTextureFailed,
    WgpuSwapchainCreateMsaaViewFailed,
    WgpuSwapchainGetcurrenttextureFailed,
    WgpuRequestDeviceStatusError,
    WgpuRequestDeviceStatusUnknown,
    WgpuRequestAdapterStatusUnavailable,
    WgpuRequestAdapterStatusError,
    WgpuRequestAdapterStatusUnknown,
    WgpuCreateInstanceFailed,
    VulkanAllocDeviceMemoryNoSuitableMemoryType,
    VulkanAllocateMemoryFailed,
    VulkanCreateInstanceFailed,
    VulkanEnumeratePhysicalDevicesFailed,
    VulkanNoPhysicalDevicesFound,
    VulkanNoSuitablePhysicalDeviceFound,
    VulkanCreateDeviceFailedExtensionNotPresent,
    VulkanCreateDeviceFailedFeatureNotPresent,
    VulkanCreateDeviceFailedInitializationFailed,
    VulkanCreateDeviceFailedOther,
    VulkanCreateSurfaceFailed,
    VulkanCreateSwapchainFailed,
    VulkanSwapchainCreateImageViewFailed,
    VulkanSwapchainCreateImageFailed,
    VulkanSwapchainAllocImageDeviceMemoryFailed,
    VulkanSwapchainBindImageMemoryFailed,
    VulkanAcquireNextImageFailed,
    VulkanQueuePresentFailed,
    ImageDataSizeMismatch,
    DroppedFilePathTooLong,
    ClipboardStringTooBig,
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
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum PixelFormat {
    Default,
    None,
    Rgba8,
    Srgb8a8,
    Bgra8,
    Sbgra8,
    Depth,
    DepthStencil,
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
pub struct VulkanEnvironment {
    pub physical_device: *const core::ffi::c_void,
    pub device: *const core::ffi::c_void,
    pub queue: *const core::ffi::c_void,
    pub queue_family_index: u32,
}
impl VulkanEnvironment {
    pub const fn new() -> Self {
        Self {
            physical_device: core::ptr::null(),
            device: core::ptr::null(),
            queue: core::ptr::null(),
            queue_family_index: 0,
        }
    }
}
impl Default for VulkanEnvironment {
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
    pub vulkan: VulkanEnvironment,
}
impl Environment {
    pub const fn new() -> Self {
        Self {
            defaults: EnvironmentDefaults::new(),
            metal: MetalEnvironment::new(),
            d3d11: D3d11Environment::new(),
            wgpu: WgpuEnvironment::new(),
            vulkan: VulkanEnvironment::new(),
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
pub struct VulkanSwapchain {
    pub render_image: *const core::ffi::c_void,
    pub render_view: *const core::ffi::c_void,
    pub resolve_image: *const core::ffi::c_void,
    pub resolve_view: *const core::ffi::c_void,
    pub depth_stencil_image: *const core::ffi::c_void,
    pub depth_stencil_view: *const core::ffi::c_void,
    pub render_finished_semaphore: *const core::ffi::c_void,
    pub present_complete_semaphore: *const core::ffi::c_void,
}
impl VulkanSwapchain {
    pub const fn new() -> Self {
        Self {
            render_image: core::ptr::null(),
            render_view: core::ptr::null(),
            resolve_image: core::ptr::null(),
            resolve_view: core::ptr::null(),
            depth_stencil_image: core::ptr::null(),
            depth_stencil_view: core::ptr::null(),
            render_finished_semaphore: core::ptr::null(),
            present_complete_semaphore: core::ptr::null(),
        }
    }
}
impl Default for VulkanSwapchain {
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
    pub vulkan: VulkanSwapchain,
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
            vulkan: VulkanSwapchain::new(),
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
pub struct GlDesc {
    pub major_version: i32,
    pub minor_version: i32,
}
impl GlDesc {
    pub const fn new() -> Self {
        Self { major_version: 0, minor_version: 0 }
    }
}
impl Default for GlDesc {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Win32Desc {
    pub console_utf8: bool,
    pub console_create: bool,
    pub console_attach: bool,
}
impl Win32Desc {
    pub const fn new() -> Self {
        Self { console_utf8: false, console_create: false, console_attach: false }
    }
}
impl Default for Win32Desc {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Html5Desc {
    pub canvas_selector: *const core::ffi::c_char,
    pub canvas_resize: bool,
    pub preserve_drawing_buffer: bool,
    pub premultiplied_alpha: bool,
    pub ask_leave_site: bool,
    pub update_document_title: bool,
    pub bubble_mouse_events: bool,
    pub bubble_touch_events: bool,
    pub bubble_wheel_events: bool,
    pub bubble_key_events: bool,
    pub bubble_char_events: bool,
    pub use_emsc_set_main_loop: bool,
    pub emsc_set_main_loop_simulate_infinite_loop: bool,
}
impl Html5Desc {
    pub const fn new() -> Self {
        Self {
            canvas_selector: core::ptr::null(),
            canvas_resize: false,
            preserve_drawing_buffer: false,
            premultiplied_alpha: false,
            ask_leave_site: false,
            update_document_title: false,
            bubble_mouse_events: false,
            bubble_touch_events: false,
            bubble_wheel_events: false,
            bubble_key_events: false,
            bubble_char_events: false,
            use_emsc_set_main_loop: false,
            emsc_set_main_loop_simulate_infinite_loop: false,
        }
    }
}
impl Default for Html5Desc {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct IosDesc {
    pub keyboard_resizes_canvas: bool,
}
impl IosDesc {
    pub const fn new() -> Self {
        Self { keyboard_resizes_canvas: false }
    }
}
impl Default for IosDesc {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Desc {
    pub init_cb: Option<extern "C" fn()>,
    pub frame_cb: Option<extern "C" fn()>,
    pub cleanup_cb: Option<extern "C" fn()>,
    pub event_cb: Option<extern "C" fn(*const Event)>,
    pub user_data: *mut core::ffi::c_void,
    pub init_userdata_cb: Option<extern "C" fn(*mut core::ffi::c_void)>,
    pub frame_userdata_cb: Option<extern "C" fn(*mut core::ffi::c_void)>,
    pub cleanup_userdata_cb: Option<extern "C" fn(*mut core::ffi::c_void)>,
    pub event_userdata_cb: Option<extern "C" fn(*const Event, *mut core::ffi::c_void)>,
    pub width: i32,
    pub height: i32,
    pub sample_count: i32,
    pub swap_interval: i32,
    pub high_dpi: bool,
    pub fullscreen: bool,
    pub alpha: bool,
    pub window_title: *const core::ffi::c_char,
    pub enable_clipboard: bool,
    pub clipboard_size: i32,
    pub enable_dragndrop: bool,
    pub max_dropped_files: i32,
    pub max_dropped_file_path_length: i32,
    pub icon: IconDesc,
    pub allocator: Allocator,
    pub logger: Logger,
    pub gl: GlDesc,
    pub win32: Win32Desc,
    pub html5: Html5Desc,
    pub ios: IosDesc,
}
impl Desc {
    pub const fn new() -> Self {
        Self {
            init_cb: None,
            frame_cb: None,
            cleanup_cb: None,
            event_cb: None,
            user_data: core::ptr::null_mut(),
            init_userdata_cb: None,
            frame_userdata_cb: None,
            cleanup_userdata_cb: None,
            event_userdata_cb: None,
            width: 0,
            height: 0,
            sample_count: 0,
            swap_interval: 0,
            high_dpi: false,
            fullscreen: false,
            alpha: false,
            window_title: core::ptr::null(),
            enable_clipboard: false,
            clipboard_size: 0,
            enable_dragndrop: false,
            max_dropped_files: 0,
            max_dropped_file_path_length: 0,
            icon: IconDesc::new(),
            allocator: Allocator::new(),
            logger: Logger::new(),
            gl: GlDesc::new(),
            win32: Win32Desc::new(),
            html5: Html5Desc::new(),
            ios: IosDesc::new(),
        }
    }
}
impl Default for Desc {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum Html5FetchError {
    FetchErrorNoError,
    FetchErrorBufferTooSmall,
    FetchErrorOther,
}
impl Html5FetchError {
    pub const fn new() -> Self {
        Self::FetchErrorNoError
    }
}
impl Default for Html5FetchError {
    fn default() -> Self {
        Self::FetchErrorNoError
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Html5FetchResponse {
    pub succeeded: bool,
    pub error_code: Html5FetchError,
    pub file_index: i32,
    pub data: Range,
    pub buffer: Range,
    pub user_data: *mut core::ffi::c_void,
}
impl Html5FetchResponse {
    pub const fn new() -> Self {
        Self {
            succeeded: false,
            error_code: Html5FetchError::new(),
            file_index: 0,
            data: Range::new(),
            buffer: Range::new(),
            user_data: core::ptr::null_mut(),
        }
    }
}
impl Default for Html5FetchResponse {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Html5FetchRequest {
    pub dropped_file_index: i32,
    pub callback: Option<extern "C" fn(*const Html5FetchResponse)>,
    pub buffer: Range,
    pub user_data: *mut core::ffi::c_void,
}
impl Html5FetchRequest {
    pub const fn new() -> Self {
        Self {
            dropped_file_index: 0,
            callback: None,
            buffer: Range::new(),
            user_data: core::ptr::null_mut(),
        }
    }
}
impl Default for Html5FetchRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum MouseCursor {
    Default = 0,
    Arrow,
    Ibeam,
    Crosshair,
    PointingHand,
    ResizeEw,
    ResizeNs,
    ResizeNwse,
    ResizeNesw,
    ResizeAll,
    NotAllowed,
    Custom0,
    Custom1,
    Custom2,
    Custom3,
    Custom4,
    Custom5,
    Custom6,
    Custom7,
    Custom8,
    Custom9,
    Custom10,
    Custom11,
    Custom12,
    Custom13,
    Custom14,
    Custom15,
    Num,
}
impl MouseCursor {
    pub const fn new() -> Self {
        Self::Default
    }
}
impl Default for MouseCursor {
    fn default() -> Self {
        Self::Default
    }
}
pub mod ffi {
    #![allow(unused_imports)]
    use super::*;
    extern "C" {
        pub fn sapp_isvalid() -> bool;
        pub fn sapp_width() -> i32;
        pub fn sapp_widthf() -> f32;
        pub fn sapp_height() -> i32;
        pub fn sapp_heightf() -> f32;
        pub fn sapp_color_format() -> PixelFormat;
        pub fn sapp_depth_format() -> PixelFormat;
        pub fn sapp_sample_count() -> i32;
        pub fn sapp_high_dpi() -> bool;
        pub fn sapp_dpi_scale() -> f32;
        pub fn sapp_show_keyboard(show: bool);
        pub fn sapp_keyboard_shown() -> bool;
        pub fn sapp_is_fullscreen() -> bool;
        pub fn sapp_toggle_fullscreen();
        pub fn sapp_show_mouse(show: bool);
        pub fn sapp_mouse_shown() -> bool;
        pub fn sapp_lock_mouse(lock: bool);
        pub fn sapp_mouse_locked() -> bool;
        pub fn sapp_set_mouse_cursor(cursor: MouseCursor);
        pub fn sapp_get_mouse_cursor() -> MouseCursor;
        pub fn sapp_bind_mouse_cursor_image(cursor: MouseCursor, desc: *const ImageDesc) -> MouseCursor;
        pub fn sapp_unbind_mouse_cursor_image(cursor: MouseCursor);
        pub fn sapp_userdata() -> *mut core::ffi::c_void;
        pub fn sapp_query_desc() -> Desc;
        pub fn sapp_request_quit();
        pub fn sapp_cancel_quit();
        pub fn sapp_quit();
        pub fn sapp_consume_event();
        pub fn sapp_frame_count() -> u64;
        pub fn sapp_frame_duration() -> f64;
        pub fn sapp_set_clipboard_string(str: *const core::ffi::c_char);
        pub fn sapp_get_clipboard_string() -> *const core::ffi::c_char;
        pub fn sapp_set_window_title(str: *const core::ffi::c_char);
        pub fn sapp_set_icon(icon_desc: *const IconDesc);
        pub fn sapp_get_num_dropped_files() -> i32;
        pub fn sapp_get_dropped_file_path(index: i32) -> *const core::ffi::c_char;
        pub fn sapp_run(desc: *const Desc);
        pub fn sapp_get_environment() -> Environment;
        pub fn sapp_get_swapchain() -> Swapchain;
        pub fn sapp_egl_get_display() -> *const core::ffi::c_void;
        pub fn sapp_egl_get_context() -> *const core::ffi::c_void;
        pub fn sapp_html5_ask_leave_site(ask: bool);
        pub fn sapp_html5_get_dropped_file_size(index: i32) -> u32;
        pub fn sapp_html5_fetch_dropped_file(request: *const Html5FetchRequest);
        pub fn sapp_macos_get_window() -> *const core::ffi::c_void;
        pub fn sapp_ios_get_window() -> *const core::ffi::c_void;
        pub fn sapp_d3d11_get_swap_chain() -> *const core::ffi::c_void;
        pub fn sapp_win32_get_hwnd() -> *const core::ffi::c_void;
        pub fn sapp_gl_get_major_version() -> i32;
        pub fn sapp_gl_get_minor_version() -> i32;
        pub fn sapp_gl_is_gles() -> bool;
        pub fn sapp_x11_get_window() -> *const core::ffi::c_void;
        pub fn sapp_x11_get_display() -> *const core::ffi::c_void;
        pub fn sapp_android_get_native_activity() -> *const core::ffi::c_void;
    }
}
#[inline]
pub fn isvalid() -> bool {
    unsafe { ffi::sapp_isvalid() }
}
#[inline]
pub fn width() -> i32 {
    unsafe { ffi::sapp_width() }
}
#[inline]
pub fn widthf() -> f32 {
    unsafe { ffi::sapp_widthf() }
}
#[inline]
pub fn height() -> i32 {
    unsafe { ffi::sapp_height() }
}
#[inline]
pub fn heightf() -> f32 {
    unsafe { ffi::sapp_heightf() }
}
#[inline]
pub fn color_format() -> PixelFormat {
    unsafe { ffi::sapp_color_format() }
}
#[inline]
pub fn depth_format() -> PixelFormat {
    unsafe { ffi::sapp_depth_format() }
}
#[inline]
pub fn sample_count() -> i32 {
    unsafe { ffi::sapp_sample_count() }
}
#[inline]
pub fn high_dpi() -> bool {
    unsafe { ffi::sapp_high_dpi() }
}
#[inline]
pub fn dpi_scale() -> f32 {
    unsafe { ffi::sapp_dpi_scale() }
}
#[inline]
pub fn show_keyboard(show: bool) {
    unsafe { ffi::sapp_show_keyboard(show) }
}
#[inline]
pub fn keyboard_shown() -> bool {
    unsafe { ffi::sapp_keyboard_shown() }
}
#[inline]
pub fn is_fullscreen() -> bool {
    unsafe { ffi::sapp_is_fullscreen() }
}
#[inline]
pub fn toggle_fullscreen() {
    unsafe { ffi::sapp_toggle_fullscreen() }
}
#[inline]
pub fn show_mouse(show: bool) {
    unsafe { ffi::sapp_show_mouse(show) }
}
#[inline]
pub fn mouse_shown() -> bool {
    unsafe { ffi::sapp_mouse_shown() }
}
#[inline]
pub fn lock_mouse(lock: bool) {
    unsafe { ffi::sapp_lock_mouse(lock) }
}
#[inline]
pub fn mouse_locked() -> bool {
    unsafe { ffi::sapp_mouse_locked() }
}
#[inline]
pub fn set_mouse_cursor(cursor: MouseCursor) {
    unsafe { ffi::sapp_set_mouse_cursor(cursor) }
}
#[inline]
pub fn get_mouse_cursor() -> MouseCursor {
    unsafe { ffi::sapp_get_mouse_cursor() }
}
#[inline]
pub fn bind_mouse_cursor_image(cursor: MouseCursor, desc: &ImageDesc) -> MouseCursor {
    unsafe { ffi::sapp_bind_mouse_cursor_image(cursor, desc) }
}
#[inline]
pub fn unbind_mouse_cursor_image(cursor: MouseCursor) {
    unsafe { ffi::sapp_unbind_mouse_cursor_image(cursor) }
}
#[inline]
pub fn userdata() -> *mut core::ffi::c_void {
    unsafe { ffi::sapp_userdata() }
}
#[inline]
pub fn query_desc() -> Desc {
    unsafe { ffi::sapp_query_desc() }
}
#[inline]
pub fn request_quit() {
    unsafe { ffi::sapp_request_quit() }
}
#[inline]
pub fn cancel_quit() {
    unsafe { ffi::sapp_cancel_quit() }
}
#[inline]
pub fn quit() {
    unsafe { ffi::sapp_quit() }
}
#[inline]
pub fn consume_event() {
    unsafe { ffi::sapp_consume_event() }
}
#[inline]
pub fn frame_count() -> u64 {
    unsafe { ffi::sapp_frame_count() }
}
#[inline]
pub fn frame_duration() -> f64 {
    unsafe { ffi::sapp_frame_duration() }
}
#[inline]
pub fn set_clipboard_string(str: &str) {
    let tmp_0 = std::ffi::CString::new(str).unwrap();
    unsafe { ffi::sapp_set_clipboard_string(tmp_0.as_ptr()) }
}
#[inline]
pub fn get_clipboard_string() -> &'static str {
    unsafe { c_char_ptr_to_rust_str(ffi::sapp_get_clipboard_string()) }
}
#[inline]
pub fn set_window_title(str: &str) {
    let tmp_0 = std::ffi::CString::new(str).unwrap();
    unsafe { ffi::sapp_set_window_title(tmp_0.as_ptr()) }
}
#[inline]
pub fn set_icon(icon_desc: &IconDesc) {
    unsafe { ffi::sapp_set_icon(icon_desc) }
}
#[inline]
pub fn get_num_dropped_files() -> i32 {
    unsafe { ffi::sapp_get_num_dropped_files() }
}
#[inline]
pub fn get_dropped_file_path(index: i32) -> &'static str {
    unsafe { c_char_ptr_to_rust_str(ffi::sapp_get_dropped_file_path(index)) }
}
#[inline]
pub fn run(desc: &Desc) {
    unsafe { ffi::sapp_run(desc) }
}
#[inline]
pub fn get_environment() -> Environment {
    unsafe { ffi::sapp_get_environment() }
}
#[inline]
pub fn get_swapchain() -> Swapchain {
    unsafe { ffi::sapp_get_swapchain() }
}
#[inline]
pub fn egl_get_display() -> *const core::ffi::c_void {
    unsafe { ffi::sapp_egl_get_display() }
}
#[inline]
pub fn egl_get_context() -> *const core::ffi::c_void {
    unsafe { ffi::sapp_egl_get_context() }
}
#[inline]
pub fn html5_ask_leave_site(ask: bool) {
    unsafe { ffi::sapp_html5_ask_leave_site(ask) }
}
#[inline]
pub fn html5_get_dropped_file_size(index: i32) -> u32 {
    unsafe { ffi::sapp_html5_get_dropped_file_size(index) }
}
#[inline]
pub fn html5_fetch_dropped_file(request: &Html5FetchRequest) {
    unsafe { ffi::sapp_html5_fetch_dropped_file(request) }
}
#[inline]
pub fn macos_get_window() -> *const core::ffi::c_void {
    unsafe { ffi::sapp_macos_get_window() }
}
#[inline]
pub fn ios_get_window() -> *const core::ffi::c_void {
    unsafe { ffi::sapp_ios_get_window() }
}
#[inline]
pub fn d3d11_get_swap_chain() -> *const core::ffi::c_void {
    unsafe { ffi::sapp_d3d11_get_swap_chain() }
}
#[inline]
pub fn win32_get_hwnd() -> *const core::ffi::c_void {
    unsafe { ffi::sapp_win32_get_hwnd() }
}
#[inline]
pub fn gl_get_major_version() -> i32 {
    unsafe { ffi::sapp_gl_get_major_version() }
}
#[inline]
pub fn gl_get_minor_version() -> i32 {
    unsafe { ffi::sapp_gl_get_minor_version() }
}
#[inline]
pub fn gl_is_gles() -> bool {
    unsafe { ffi::sapp_gl_is_gles() }
}
#[inline]
pub fn x11_get_window() -> *const core::ffi::c_void {
    unsafe { ffi::sapp_x11_get_window() }
}
#[inline]
pub fn x11_get_display() -> *const core::ffi::c_void {
    unsafe { ffi::sapp_x11_get_display() }
}
#[inline]
pub fn android_get_native_activity() -> *const core::ffi::c_void {
    unsafe { ffi::sapp_android_get_native_activity() }
}
