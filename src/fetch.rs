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
    FilePathUtf8DecodingFailed,
    SendQueueFull,
    RequestChannelIndexTooBig,
    RequestPathIsNull,
    RequestPathTooLong,
    RequestCallbackMissing,
    RequestChunkSizeGreaterBufferSize,
    RequestUserdataPtrIsSetButUserdataSizeIsNull,
    RequestUserdataPtrIsNullButUserdataSizeIsNot,
    RequestUserdataSizeTooBig,
    ClampingNumChannelsToMaxChannels,
    RequestPoolExhausted,
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
    pub max_requests: u32,
    pub num_channels: u32,
    pub num_lanes: u32,
    pub allocator: Allocator,
    pub logger: Logger,
}
impl Desc {
    pub const fn new() -> Self {
        Self {
            max_requests: 0,
            num_channels: 0,
            num_lanes: 0,
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
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Handle {
    pub id: u32,
}
impl Handle {
    pub const fn new() -> Self {
        Self { id: 0 }
    }
}
impl Default for Handle {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum Error {
    NoError,
    FileNotFound,
    NoBuffer,
    BufferTooSmall,
    UnexpectedEof,
    InvalidHttpStatus,
    Cancelled,
    JsOther,
}
impl Error {
    pub const fn new() -> Self {
        Self::NoError
    }
}
impl Default for Error {
    fn default() -> Self {
        Self::NoError
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Response {
    pub handle: Handle,
    pub dispatched: bool,
    pub fetched: bool,
    pub paused: bool,
    pub finished: bool,
    pub failed: bool,
    pub cancelled: bool,
    pub error_code: Error,
    pub channel: u32,
    pub lane: u32,
    pub path: *const core::ffi::c_char,
    pub user_data: *mut core::ffi::c_void,
    pub data_offset: u32,
    pub data: Range,
    pub buffer: Range,
}
impl Response {
    pub const fn new() -> Self {
        Self {
            handle: Handle::new(),
            dispatched: false,
            fetched: false,
            paused: false,
            finished: false,
            failed: false,
            cancelled: false,
            error_code: Error::new(),
            channel: 0,
            lane: 0,
            path: core::ptr::null(),
            user_data: core::ptr::null_mut(),
            data_offset: 0,
            data: Range::new(),
            buffer: Range::new(),
        }
    }
}
impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Request {
    pub channel: u32,
    pub path: *const core::ffi::c_char,
    pub callback: Option<extern "C" fn(*const Response)>,
    pub chunk_size: u32,
    pub buffer: Range,
    pub user_data: Range,
}
impl Request {
    pub const fn new() -> Self {
        Self {
            channel: 0,
            path: core::ptr::null(),
            callback: None,
            chunk_size: 0,
            buffer: Range::new(),
            user_data: Range::new(),
        }
    }
}
impl Default for Request {
    fn default() -> Self {
        Self::new()
    }
}
pub mod ffi {
    #![allow(unused_imports)]
    use super::*;
    extern "C" {
        pub fn sfetch_setup(desc: *const Desc);
        pub fn sfetch_shutdown();
        pub fn sfetch_valid() -> bool;
        pub fn sfetch_desc() -> Desc;
        pub fn sfetch_max_userdata_bytes() -> i32;
        pub fn sfetch_max_path() -> i32;
        pub fn sfetch_send(request: *const Request) -> Handle;
        pub fn sfetch_handle_valid(h: Handle) -> bool;
        pub fn sfetch_dowork();
        pub fn sfetch_bind_buffer(h: Handle, buffer: Range);
        pub fn sfetch_unbind_buffer(h: Handle) -> *mut core::ffi::c_void;
        pub fn sfetch_cancel(h: Handle);
        pub fn sfetch_pause(h: Handle);
        pub fn sfetch_continue(h: Handle);
    }
}
#[inline]
pub fn setup(desc: &Desc) {
    unsafe { ffi::sfetch_setup(desc) }
}
#[inline]
pub fn shutdown() {
    unsafe { ffi::sfetch_shutdown() }
}
#[inline]
pub fn valid() -> bool {
    unsafe { ffi::sfetch_valid() }
}
#[inline]
pub fn desc() -> Desc {
    unsafe { ffi::sfetch_desc() }
}
#[inline]
pub fn max_userdata_bytes() -> i32 {
    unsafe { ffi::sfetch_max_userdata_bytes() }
}
#[inline]
pub fn max_path() -> i32 {
    unsafe { ffi::sfetch_max_path() }
}
#[inline]
pub fn send(request: &Request) -> Handle {
    unsafe { ffi::sfetch_send(request) }
}
#[inline]
pub fn handle_valid(h: Handle) -> bool {
    unsafe { ffi::sfetch_handle_valid(h) }
}
#[inline]
pub fn dowork() {
    unsafe { ffi::sfetch_dowork() }
}
#[inline]
pub fn bind_buffer(h: Handle, buffer: Range) {
    unsafe { ffi::sfetch_bind_buffer(h, buffer) }
}
#[inline]
pub fn unbind_buffer(h: Handle) -> *mut core::ffi::c_void {
    unsafe { ffi::sfetch_unbind_buffer(h) }
}
#[inline]
pub fn cancel(h: Handle) {
    unsafe { ffi::sfetch_cancel(h) }
}
#[inline]
pub fn pause(h: Handle) {
    unsafe { ffi::sfetch_pause(h) }
}
#[inline]
pub fn continue_fetching(h: Handle) {
    unsafe { ffi::sfetch_continue(h) }
}
