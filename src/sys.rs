/* automatically generated by rust-bindgen 0.58.1 */

pub type size_t = ::std::os::raw::c_ulong;
pub const OIDNDeviceType_OIDN_DEVICE_TYPE_DEFAULT: OIDNDeviceType = 0;
pub const OIDNDeviceType_OIDN_DEVICE_TYPE_CPU: OIDNDeviceType = 1;
pub type OIDNDeviceType = ::std::os::raw::c_uint;
pub const OIDNError_OIDN_ERROR_NONE: OIDNError = 0;
pub const OIDNError_OIDN_ERROR_UNKNOWN: OIDNError = 1;
pub const OIDNError_OIDN_ERROR_INVALID_ARGUMENT: OIDNError = 2;
pub const OIDNError_OIDN_ERROR_INVALID_OPERATION: OIDNError = 3;
pub const OIDNError_OIDN_ERROR_OUT_OF_MEMORY: OIDNError = 4;
pub const OIDNError_OIDN_ERROR_UNSUPPORTED_HARDWARE: OIDNError = 5;
pub const OIDNError_OIDN_ERROR_CANCELLED: OIDNError = 6;
pub type OIDNError = ::std::os::raw::c_uint;
pub type OIDNErrorFunction = ::std::option::Option<
    unsafe extern "system" fn(
        userPtr: *mut ::std::os::raw::c_void,
        code: OIDNError,
        message: *const ::std::os::raw::c_char,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OIDNDeviceImpl {
    _unused: [u8; 0],
}
pub type OIDNDevice = *mut OIDNDeviceImpl;
extern "system" {
    pub fn oidnNewDevice(type_: OIDNDeviceType) -> OIDNDevice;
}
extern "system" {
    pub fn oidnRetainDevice(device: OIDNDevice);
}
extern "system" {
    pub fn oidnReleaseDevice(device: OIDNDevice);
}
extern "system" {
    pub fn oidnSetDevice1b(device: OIDNDevice, name: *const ::std::os::raw::c_char, value: bool);
}
extern "system" {
    pub fn oidnSetDevice1i(
        device: OIDNDevice,
        name: *const ::std::os::raw::c_char,
        value: ::std::os::raw::c_int,
    );
}
extern "system" {
    pub fn oidnGetDevice1b(device: OIDNDevice, name: *const ::std::os::raw::c_char) -> bool;
}
extern "system" {
    pub fn oidnGetDevice1i(
        device: OIDNDevice,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "system" {
    pub fn oidnSetDeviceErrorFunction(
        device: OIDNDevice,
        func: OIDNErrorFunction,
        userPtr: *mut ::std::os::raw::c_void,
    );
}
extern "system" {
    pub fn oidnGetDeviceError(
        device: OIDNDevice,
        outMessage: *mut *const ::std::os::raw::c_char,
    ) -> OIDNError;
}
extern "system" {
    pub fn oidnCommitDevice(device: OIDNDevice);
}
pub const OIDNFormat_OIDN_FORMAT_UNDEFINED: OIDNFormat = 0;
pub const OIDNFormat_OIDN_FORMAT_FLOAT: OIDNFormat = 1;
pub const OIDNFormat_OIDN_FORMAT_FLOAT2: OIDNFormat = 2;
pub const OIDNFormat_OIDN_FORMAT_FLOAT3: OIDNFormat = 3;
pub const OIDNFormat_OIDN_FORMAT_FLOAT4: OIDNFormat = 4;
pub type OIDNFormat = ::std::os::raw::c_uint;
pub const OIDNAccess_OIDN_ACCESS_READ: OIDNAccess = 0;
pub const OIDNAccess_OIDN_ACCESS_WRITE: OIDNAccess = 1;
pub const OIDNAccess_OIDN_ACCESS_READ_WRITE: OIDNAccess = 2;
pub const OIDNAccess_OIDN_ACCESS_WRITE_DISCARD: OIDNAccess = 3;
pub type OIDNAccess = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OIDNBufferImpl {
    _unused: [u8; 0],
}
pub type OIDNBuffer = *mut OIDNBufferImpl;
extern "system" {
    pub fn oidnNewBuffer(device: OIDNDevice, byteSize: size_t) -> OIDNBuffer;
}
extern "system" {
    pub fn oidnNewSharedBuffer(
        device: OIDNDevice,
        ptr: *mut ::std::os::raw::c_void,
        byteSize: size_t,
    ) -> OIDNBuffer;
}
extern "system" {
    pub fn oidnMapBuffer(
        buffer: OIDNBuffer,
        access: OIDNAccess,
        byteOffset: size_t,
        byteSize: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "system" {
    pub fn oidnUnmapBuffer(buffer: OIDNBuffer, mappedPtr: *mut ::std::os::raw::c_void);
}
extern "system" {
    pub fn oidnRetainBuffer(buffer: OIDNBuffer);
}
extern "system" {
    pub fn oidnReleaseBuffer(buffer: OIDNBuffer);
}
pub type OIDNProgressMonitorFunction = ::std::option::Option<
    unsafe extern "system" fn(userPtr: *mut ::std::os::raw::c_void, n: f64) -> bool,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OIDNFilterImpl {
    _unused: [u8; 0],
}
pub type OIDNFilter = *mut OIDNFilterImpl;
extern "system" {
    pub fn oidnNewFilter(device: OIDNDevice, type_: *const ::std::os::raw::c_char) -> OIDNFilter;
}
extern "system" {
    pub fn oidnRetainFilter(filter: OIDNFilter);
}
extern "system" {
    pub fn oidnReleaseFilter(filter: OIDNFilter);
}
extern "system" {
    pub fn oidnSetFilterImage(
        filter: OIDNFilter,
        name: *const ::std::os::raw::c_char,
        buffer: OIDNBuffer,
        format: OIDNFormat,
        width: size_t,
        height: size_t,
        byteOffset: size_t,
        bytePixelStride: size_t,
        byteRowStride: size_t,
    );
}
extern "system" {
    pub fn oidnSetSharedFilterImage(
        filter: OIDNFilter,
        name: *const ::std::os::raw::c_char,
        ptr: *mut ::std::os::raw::c_void,
        format: OIDNFormat,
        width: size_t,
        height: size_t,
        byteOffset: size_t,
        bytePixelStride: size_t,
        byteRowStride: size_t,
    );
}
extern "system" {
    pub fn oidnRemoveFilterImage(filter: OIDNFilter, name: *const ::std::os::raw::c_char);
}
extern "system" {
    pub fn oidnSetSharedFilterData(
        filter: OIDNFilter,
        name: *const ::std::os::raw::c_char,
        ptr: *mut ::std::os::raw::c_void,
        byteSize: size_t,
    );
}
extern "system" {
    pub fn oidnUpdateFilterData(filter: OIDNFilter, name: *const ::std::os::raw::c_char);
}
extern "system" {
    pub fn oidnRemoveFilterData(filter: OIDNFilter, name: *const ::std::os::raw::c_char);
}
extern "system" {
    pub fn oidnSetFilter1b(filter: OIDNFilter, name: *const ::std::os::raw::c_char, value: bool);
}
extern "system" {
    pub fn oidnGetFilter1b(filter: OIDNFilter, name: *const ::std::os::raw::c_char) -> bool;
}
extern "system" {
    pub fn oidnSetFilter1i(
        filter: OIDNFilter,
        name: *const ::std::os::raw::c_char,
        value: ::std::os::raw::c_int,
    );
}
extern "system" {
    pub fn oidnGetFilter1i(
        filter: OIDNFilter,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "system" {
    pub fn oidnSetFilter1f(filter: OIDNFilter, name: *const ::std::os::raw::c_char, value: f32);
}
extern "system" {
    pub fn oidnGetFilter1f(filter: OIDNFilter, name: *const ::std::os::raw::c_char) -> f32;
}
extern "system" {
    pub fn oidnSetFilterProgressMonitorFunction(
        filter: OIDNFilter,
        func: OIDNProgressMonitorFunction,
        userPtr: *mut ::std::os::raw::c_void,
    );
}
extern "system" {
    pub fn oidnCommitFilter(filter: OIDNFilter);
}
extern "system" {
    pub fn oidnExecuteFilter(filter: OIDNFilter);
}
