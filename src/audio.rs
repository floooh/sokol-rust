// machine generated, do not edit

#![allow(dead_code)]
#![allow(unused_imports)]

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
    AlsaSndPcmOpenFailed,
    AlsaFloatSamplesNotSupported,
    AlsaRequestedBufferSizeNotSupported,
    AlsaRequestedChannelCountNotSupported,
    AlsaSndPcmHwParamsSetRateNearFailed,
    AlsaSndPcmHwParamsFailed,
    AlsaPthreadCreateFailed,
    WasapiCreateEventFailed,
    WasapiCreateDeviceEnumeratorFailed,
    WasapiGetDefaultAudioEndpointFailed,
    WasapiDeviceActivateFailed,
    WasapiAudioClientInitializeFailed,
    WasapiAudioClientGetBufferSizeFailed,
    WasapiAudioClientGetServiceFailed,
    WasapiAudioClientSetEventHandleFailed,
    WasapiCreateThreadFailed,
    AaudioStreambuilderOpenStreamFailed,
    AaudioPthreadCreateFailed,
    AaudioRestartingStreamAfterError,
    UsingAaudioBackend,
    AaudioCreateStreambuilderFailed,
    UsingSlesBackend,
    SlesCreateEngineFailed,
    SlesEngineGetEngineInterfaceFailed,
    SlesCreateOutputMixFailed,
    SlesMixerGetVolumeInterfaceFailed,
    SlesEngineCreateAudioPlayerFailed,
    SlesPlayerGetPlayInterfaceFailed,
    SlesPlayerGetVolumeInterfaceFailed,
    SlesPlayerGetBufferqueueInterfaceFailed,
    CoreaudioNewOutputFailed,
    CoreaudioAllocateBufferFailed,
    CoreaudioStartFailed,
    BackendBufferSizeIsntMultipleOfPacketSize,
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
    pub sample_rate: i32,
    pub num_channels: i32,
    pub buffer_frames: i32,
    pub packet_frames: i32,
    pub num_packets: i32,
    pub stream_cb: Option<extern "C" fn(*mut f32, i32, i32)>,
    pub stream_userdata_cb: Option<extern "C" fn(*mut f32, i32, i32, *mut core::ffi::c_void)>,
    pub user_data: *mut core::ffi::c_void,
    pub allocator: Allocator,
    pub logger: Logger,
}
impl Desc {
    pub const fn new() -> Self {
        Self {
            sample_rate: 0,
            num_channels: 0,
            buffer_frames: 0,
            packet_frames: 0,
            num_packets: 0,
            stream_cb: None,
            stream_userdata_cb: None,
            user_data: core::ptr::null_mut(),
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
        pub fn saudio_setup(desc: *const Desc);
        pub fn saudio_shutdown();
        pub fn saudio_isvalid() -> bool;
        pub fn saudio_userdata() -> *mut core::ffi::c_void;
        pub fn saudio_query_desc() -> Desc;
        pub fn saudio_sample_rate() -> i32;
        pub fn saudio_buffer_frames() -> i32;
        pub fn saudio_channels() -> i32;
        pub fn saudio_suspended() -> bool;
        pub fn saudio_expect() -> i32;
        pub fn saudio_push(frames: *const f32, num_frames: i32) -> i32;
    }
}
#[inline]
pub fn setup(desc: &Desc) {
    unsafe { ffi::saudio_setup(desc) }
}
#[inline]
pub fn shutdown() {
    unsafe { ffi::saudio_shutdown() }
}
#[inline]
pub fn isvalid() -> bool {
    unsafe { ffi::saudio_isvalid() }
}
#[inline]
pub fn userdata() -> *mut core::ffi::c_void {
    unsafe { ffi::saudio_userdata() }
}
#[inline]
pub fn query_desc() -> Desc {
    unsafe { ffi::saudio_query_desc() }
}
#[inline]
pub fn sample_rate() -> i32 {
    unsafe { ffi::saudio_sample_rate() }
}
#[inline]
pub fn buffer_frames() -> i32 {
    unsafe { ffi::saudio_buffer_frames() }
}
#[inline]
pub fn channels() -> i32 {
    unsafe { ffi::saudio_channels() }
}
#[inline]
pub fn suspended() -> bool {
    unsafe { ffi::saudio_suspended() }
}
#[inline]
pub fn expect() -> i32 {
    unsafe { ffi::saudio_expect() }
}
#[inline]
pub fn push(frames: &f32, num_frames: i32) -> i32 {
    unsafe { ffi::saudio_push(frames, num_frames) }
}
