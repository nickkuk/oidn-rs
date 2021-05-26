use std::{convert::TryInto, ffi::CStr, os::raw::c_char, ptr};

use crate::sys::*;
use crate::FilterError;

/// An Open Image Denoise device (e.g. a CPU).
///
/// Open Image Denoise supports a device concept, which allows different
/// components of the application to use the API without interfering with each
/// other.
pub struct Device(pub(crate) OIDNDevice);

impl Device {
    /// Create a device using the fastest device available to run denoising
    pub fn new() -> Self {
        let handle = unsafe { oidnNewDevice(OIDNDeviceType_OIDN_DEVICE_TYPE_DEFAULT) };
        unsafe {
            oidnCommitDevice(handle);
        }
        Self(handle)
    }

    /// Create a device to run denoising on the CPU; corresponds to Device::cpu_with_params(0, true)
    pub fn cpu() -> Self {
        let handle = unsafe { oidnNewDevice(OIDNDeviceType_OIDN_DEVICE_TYPE_CPU) };
        unsafe {
            oidnCommitDevice(handle);
        }
        Self(handle)
    }

    /// Create a device to run denoising on the CPU with bounded number of threads and affinity;
    /// max_threads = 0 will set it automatically to get the best performance;
    /// affinity = true binds software threads to hardware threads which improves performance;
    /// otherwise this binding is disabled
    pub fn cpu_with_params(max_threads: u32, affinity: bool) -> Self {
        let handle = unsafe { oidnNewDevice(OIDNDeviceType_OIDN_DEVICE_TYPE_CPU) };
        unsafe {
            oidnSetDevice1i(handle, b"numThreads\0" as *const _ as _, max_threads as i32);
            oidnSetDevice1b(handle, b"setAffinity\0" as *const _ as _, affinity);
            oidnCommitDevice(handle);
        }
        Self(handle)
    }

    pub fn get_error(&self) -> Result<(), (FilterError, String)> {
        let mut err_msg = ptr::null();
        let err = unsafe { oidnGetDeviceError(self.0, &mut err_msg as *mut *const c_char) };
        if OIDNError_OIDN_ERROR_NONE == err {
            Ok(())
        } else {
            let msg = unsafe { CStr::from_ptr(err_msg).to_string_lossy().to_string() };
            Err((err.try_into().unwrap(), msg))
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            oidnReleaseDevice(self.0);
        }
    }
}

impl Default for Device {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<'a, 'b> Send for Device {}
