use super::*;
use std::ffi::CStr;
use std::os::raw;
use std::ptr;
use std::thread;
use std::time;

pub struct PtrWrapper<T> {
    ptr: *mut T,
    destroy_fn: unsafe extern "C" fn(ptr: *mut T) -> tobii_error_t,
}

impl<T> PtrWrapper<T> {
    pub unsafe fn new(
        ptr: *mut T,
        destroy_fn: unsafe extern "C" fn(ptr: *mut T) -> tobii_error_t,
    ) -> PtrWrapper<T> {
        PtrWrapper { ptr, destroy_fn }
    }

    pub fn ptr(&self) -> *mut T {
        self.ptr
    }
}

impl<T> Drop for PtrWrapper<T> {
    fn drop(&mut self) {
        let destroy_fn = self.destroy_fn;
        let status = unsafe { destroy_fn(self.ptr) };
        assert_eq!(status, tobii_error_t_TOBII_ERROR_NO_ERROR);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TobiiError {
    Internal,
    InsufficientLicense,
    NotSupported,
    NotAvailable,
    ConnectionFailed,
    TimedOut,
    AllocationFailed,
    InvalidParameter,
    CalibrationAlreadyStarted,
    CalibrationNotStarted,
    AlreadySubscribed,
    NotSubscribed,
    OperationFailed,
    ConflictingApiInstances,
    CalibrationBusy,
    CallbackInProgress,
    Unknown(tobii_error_t),
}

pub fn status_to_result(status: tobii_error_t) -> Result<(), TobiiError> {
    match status {
        tobii_error_t_TOBII_ERROR_NO_ERROR => Ok(()),
        tobii_error_t_TOBII_ERROR_INTERNAL => Err(TobiiError::Internal),
        tobii_error_t_TOBII_ERROR_INSUFFICIENT_LICENSE => Err(TobiiError::InsufficientLicense),
        tobii_error_t_TOBII_ERROR_NOT_SUPPORTED => Err(TobiiError::NotSupported),
        tobii_error_t_TOBII_ERROR_NOT_AVAILABLE => Err(TobiiError::NotAvailable),
        tobii_error_t_TOBII_ERROR_CONNECTION_FAILED => Err(TobiiError::ConnectionFailed),
        tobii_error_t_TOBII_ERROR_TIMED_OUT => Err(TobiiError::TimedOut),
        tobii_error_t_TOBII_ERROR_ALLOCATION_FAILED => Err(TobiiError::AllocationFailed),
        tobii_error_t_TOBII_ERROR_INVALID_PARAMETER => Err(TobiiError::InvalidParameter),
        tobii_error_t_TOBII_ERROR_CALIBRATION_ALREADY_STARTED => {
            Err(TobiiError::CalibrationAlreadyStarted)
        }
        tobii_error_t_TOBII_ERROR_CALIBRATION_NOT_STARTED => Err(TobiiError::CalibrationNotStarted),
        tobii_error_t_TOBII_ERROR_ALREADY_SUBSCRIBED => Err(TobiiError::AlreadySubscribed),
        tobii_error_t_TOBII_ERROR_NOT_SUBSCRIBED => Err(TobiiError::NotSubscribed),
        tobii_error_t_TOBII_ERROR_OPERATION_FAILED => Err(TobiiError::OperationFailed),
        tobii_error_t_TOBII_ERROR_CONFLICTING_API_INSTANCES => {
            Err(TobiiError::ConflictingApiInstances)
        }
        tobii_error_t_TOBII_ERROR_CALIBRATION_BUSY => Err(TobiiError::CalibrationBusy),
        tobii_error_t_TOBII_ERROR_CALLBACK_IN_PROGRESS => Err(TobiiError::CallbackInProgress),
        _ => Err(TobiiError::Unknown(status)),
    }
}

pub unsafe fn list_devices(api: *mut tobii_api_t) -> Result<Vec<String>, TobiiError> {
    unsafe extern "C" fn callback(url: *const raw::c_char, user_data: *mut raw::c_void) {
        let list = &mut *(user_data as *mut Vec<String>);
        let s = CStr::from_ptr(url);
        list.push(s.to_str().unwrap().to_string());
    }

    let mut list: Vec<String> = Vec::new();
    let list_ptr = &mut list as *mut Vec<String>;
    let status =
        tobii_enumerate_local_device_urls(api, Some(callback), list_ptr as *mut raw::c_void);
    status_to_result(status)?;
    Ok(list)
}

pub unsafe fn reconnect(device: *mut tobii_device_t) -> tobii_error_t {
    for _i in 0..40 {
        let status = tobii_device_reconnect(device);
        if status != tobii_error_t_TOBII_ERROR_CONNECTION_FAILED {
            return status;
        }
        thread::sleep(time::Duration::from_millis(250));
    }
    return tobii_error_t_TOBII_ERROR_CONNECTION_FAILED;
}

pub unsafe fn wait_for_device_callbacks(device: *mut tobii_device_t) -> tobii_error_t {
    let ptr_ptr_dev: *const *mut tobii_device_t = (&device) as *const *mut tobii_device_t;
    tobii_wait_for_callbacks(ptr::null_mut(), 1, ptr_ptr_dev)
}

// public static WindowsList Load()
//         {
//             var lShellWindow = GetShellWindow();
//             var windows = new List<IWindowEntry>();
//             var currentProcessId = Process.GetCurrentProcess().Id;

//             EnumWindows((hWnd, lParam) =>
//             {
//                 if (!EligibleForActivation(hWnd, lShellWindow))
//                     return true;

//                 var window = WindowEntryFactory.Create(hWnd);

//                 if (window == null || window.ProcessId == currentProcessId || window.Title == null)
//                     return true;

//                 window.ProcessName = WmiProcessWatcher.GetProcessName(window.ProcessId, () => window.ProcessName);

//                 if (IsKnownException(window))
//                     return true;

//                 windows.Add(window);

//                 return true;
//             }, 0);

//             return new WindowsList(windows);
// }
