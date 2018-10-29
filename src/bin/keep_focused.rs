extern crate tobii_rust;
extern crate widestring;
extern crate winapi;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw;
use std::os::raw::c_void;
use std::ptr;
use tobii_rust::*;
use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::FALSE;
use winapi::shared::minwindef::LPARAM;
use winapi::shared::minwindef::TRUE;

use tobii_rust::helpers::{self, status_to_result, PtrWrapper, TobiiError};
use winapi::shared::minwindef::LPDWORD;
use winapi::shared::ntdef::LPWSTR;
use winapi::um::winuser;
use winapi::um::winuser::*;
// use winapi::shared::ntdef::LPSTR;
use widestring::U16String;
use winapi::shared::windef::HWND;
use winapi::um::dwmapi::*;
use winapi::um::libloaderapi::GetModuleFileNameW;
// use winapi::um::psapi::*;
use winapi::um::winnt::*;

macro_rules! get_w_string {
    ($fun:ident, $handle:ident) => {{
        let mut buffer: Vec<u16> = vec![0; 256];
        let str_len = $fun(
            $handle,
            buffer.as_ptr() as LPWSTR,
            num::cast(buffer.len()).unwrap(),
        );
        buffer.truncate(str_len as usize);
        buf_to_string(&buffer)
    }};
}

unsafe extern "C" fn custom_log_fn(
    _log_context: *mut ::std::os::raw::c_void,
    level: tobii_log_level_t,
    text: *const raw::c_char,
) {
    if level > tobii_log_level_t_TOBII_LOG_LEVEL_WARN {
        return;
    }
    let s = CStr::from_ptr(text);
    println!("LOG {}: {}", level, s.to_str().unwrap());
}

// fn win32_string( value : &str ) -> Vec<u16> {
//     OsStr::new( value ).encode_wide().chain( once( 0 ) ).collect()
// }

unsafe fn get_title(handle: HWND) -> String {
    get_w_string!(GetWindowTextW, handle)
    //     let mut buffer:Vec<u16> = vec![0;GetWindowTextLengthW(handle) as usize];
    //     let str_len = GetWindowTextW(handle,buffer.as_ptr() as LPWSTR,50);
    //     buffer.truncate(str_len as usize);
    //     buf_to_string(&buffer)
}

fn buf_to_string(buffer: &Vec<u16>) -> String {
    unsafe {
        let u16_str: U16String = U16String::from_ptr(buffer.as_ptr(), buffer.len());
        u16_str.to_string().unwrap()
    }
}

#[derive(Debug)]
struct ParamData {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

const GA_ROOTOWNER: u32 = 3;
fn is_alt_tab_window(hwnd: HWND) -> bool {
    unsafe {
        let mut hwnd_walk = GetAncestor(hwnd, GA_ROOTOWNER);
        // See if we are the last active visible popup
        let mut hwnd_try = GetLastActivePopup(hwnd_walk);
        while GetLastActivePopup(hwnd_walk) != hwnd_try {
            if IsWindowVisible(hwnd_try) != 0 {
                break;
            }
            hwnd_walk = hwnd_try;
            hwnd_try = GetLastActivePopup(hwnd_walk);
        }
        return hwnd_walk == hwnd;
    }
}


unsafe fn is_invisible_win10_background_app_window(handle: HWND) -> bool {
    let mut cloaked_val = 0;
    let res = DwmGetWindowAttribute(
        handle,
        DWMWA_CLOAKED,
        mem::transmute(&mut cloaked_val),
        std::mem::size_of::<u32>() as u32,
    );
    if res != 0 {
        cloaked_val = 0;
    }
    cloaked_val != 0

}
unsafe fn get_super_parent(handle: HWND) -> HWND {
    let mut parent = GetWindow(handle, GW_OWNER);
    loop {
        let next = GetWindow(parent, GW_OWNER);
        if next == ptr::null_mut() {
            break;
        } else {
            parent = next;
        }
    }
    parent
}
unsafe fn is_eligible_for_activation(handle: HWND) -> bool {
    if IsWindow(handle) == 0 {
        return false;
    }
    if handle == GetDesktopWindow() {
        return false;
    }
    if IsIconic(handle) != 0 {
        return false;
    }
    if handle == GetShellWindow() {
        return false;
    }
    if !is_alt_tab_window(handle) {
        return false;
    }
    if GetWindowTextLengthW(handle) == 0 {
        return false;
    }
    if IsWindowVisible(handle) == 0 {
        // could be obscured by other windows and still return true
        return false;
    }

    let owner_hwnd = get_super_parent(handle);
    if !(GetLastActivePopup(owner_hwnd) == handle || owner_hwnd == ptr::null_mut()) {
        return false;
    }

    let es = GetWindowLongPtrW(handle, GWL_EXSTYLE);
    if !(!((es & WS_EX_TOOLWINDOW as isize != 0) && (es & WS_EX_APPWINDOW as isize == 0))
            && !is_invisible_win10_background_app_window(handle))
    {
        return false;
    }
    if es & WS_EX_OVERLAPPEDWINDOW as isize == 0
    {
        return false;
    }

    let class_name = get_w_string!(GetClassNameW, handle);
    if class_name.len() == 0 {
        return false;
    }

    if [
        "Shell_TrayWnd",
        "DV2ControlHost",
        "MsgrIMEWindowClass",
        "SysShadow",
        "Button",
        "Settings",
        "Microsoft Store",
    ]
        .contains(&class_name.as_str())
    {
        println!("abort {:?}", class_name);
        return false;
    }
    if class_name.starts_with("WMP9MediaBarFlyout") {
        //WMP's "now playing" taskbar-toolbar
        return false;
    }

    return true;
}

// unsafe fn get_last_visible_active_pop_up_of_window(handle: HWND) -> HWND {
//     let last_pop_up = GetLastActivePopup(handle);
//     if IsWindowVisible(last_pop_up) != 0 {
//         last_pop_up
//     } else if last_pop_up == handle {
//         ptr::null_mut()
//     } else {
//         get_last_visible_active_pop_up_of_window(last_pop_up)
//     }
// }
fn get_windows() -> Vec<HWND> {
    let mut windows: Vec<HWND> = vec![];
    // let closure_pointer_pointer: *mut c_void = unsafe { mem::transmute(&mut trait_obj) };
    // let lparam = closure_pointer_pointer as usize;
    // EnumWindows(enum_windows_callback, gaze_point);

    enumerate_windows(&mut |handle: HWND| {
        if unsafe{is_eligible_for_activation(handle)} {
            windows.push(handle);
        }
        true
    });
    windows
}

pub fn enumerate_windows<F>(mut callback: F)
where
    F: FnMut(HWND) -> bool,
{
    let mut trait_obj: &mut FnMut(HWND) -> bool = &mut callback;
    let closure_pointer_pointer: *mut c_void = unsafe { mem::transmute(&mut trait_obj) };

    let lparam = closure_pointer_pointer as LPARAM;
    unsafe { EnumWindows(Some(enumerate_callback), lparam) };
}

unsafe extern "system" fn enumerate_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let closure: &mut &mut FnMut(HWND) -> bool = mem::transmute(lparam as *mut c_void);
    if closure(hwnd) {
        TRUE
    } else {
        FALSE
    }
}

use std::time::{SystemTime};

fn key_was_pressed() ->  bool {
    for i in 1..190 {
        if unsafe { GetAsyncKeyState(i) } != 0 { // -32767
            return true;
        }
    }
    false
}

unsafe extern "C" fn gaze_callback(
    gaze_point: *const tobii_gaze_point_t,
    user_data: *mut ::std::os::raw::c_void,
) {
    let pt = &*gaze_point;

    // let mut yo = LASTINPUTINFO{
    //     cbSize: mem::size_of::<LASTINPUTINFO>() as u32,
    //     dwTime: 0,
    // };
    // let _res = GetLastInputInfo(mem::transmute(&mut yo));

    // let delta = winapi::um::sysinfoapi::GetTickCount() - yo.dwTime;
    // println!("{:?}", delta);
    // if delta < 5000 {
    //     return;
    // }

    let data: &mut SubscribeData = mem::transmute(user_data);

    if key_was_pressed() {
        data.last_input_time = SystemTime::now();
        // let start = SystemTime::now();
        // let Duration, = start.duration_since(UNIX_EPOCH)
        //     .expect("Time went backwards");
    }

    let delta = SystemTime::now().duration_since(data.last_input_time).unwrap();
    // println!("{:?}", delta);
    if delta < std::time::Duration::from_millis(100) {
        return;
    }


    let width = winuser::GetSystemMetrics(winuser::SM_CXSCREEN);
    let height = winuser::GetSystemMetrics(winuser::SM_CYSCREEN);
    // let res = Resolution{width, height};

    if pt.validity == tobii_validity_t_TOBII_VALIDITY_VALID {
        let x = (pt.position_xy[0] * width as f32) as i32;
        let y = (pt.position_xy[1] * height as f32) as i32;
        // let param = ParamData{
        //     x, y, width, height
        // };
        // let closure_pointer_pointer: *mut c_void = unsafe { mem::transmute(&mut trait_obj) };
        // let lparam = closure_pointer_pointer as usize;
        // EnumWindows(enum_windows_callback, gaze_point);
        // EnumWindows(Some(enum_windows_callback), mem::transmute(&param));
        

        // let mut rect: winapi::shared::windef::RECT = mem::zeroed();
        // winuser::GetWindowRect(handle, &mut rect as *mut winapi::shared::windef::RECT);

        // let padding = 40; // padding for looking at close X and toolbars
        // // rect
        // rect.top -= padding;
        // if x > rect.left && x < rect.right && y > rect.top && y < rect.bottom {
        //     if handle == winuser::GetForegroundWindow()
        //         || handle == winuser::GetActiveWindow()
        //         || handle == winuser::GetFocus()
        //     {
        //         return;
        //     }

        // }


        for handle in get_windows() {
            let mut rect: winapi::shared::windef::RECT = mem::zeroed();
            winuser::GetWindowRect(handle, &mut rect as *mut winapi::shared::windef::RECT);

            let padding = 40; // padding for looking at close X and toolbars
            // rect
            rect.top -= padding;
            if x > rect.left && x < rect.right && y > rect.top && y < rect.bottom {
                if handle == winuser::GetForegroundWindow()
                    || handle == winuser::GetActiveWindow()
                    || handle == winuser::GetFocus()
                {
                    return;
                }

                if data.looking_at != handle {
                    data.looking_at = handle;
                    data.since = pt.timestamp_us;
                    println!("chenged look");
                }

                if pt.timestamp_us - data.since < 1000 {
                    // println!("{:?}", data.since - pt.timestamp_us);
                    return; //Look longer
                }else{
                    data.since = pt.timestamp_us;
                }

                SetForegroundWindow(data.original);
                let res = SetForegroundWindow(handle);
                // SwitchToThisWindow(handle, 1);
                // let res = 1;
                // if handle == winuser::GetActiveWindow() {
                //     return;
                // }
                // SetActiveWindow(handle);
                // let res = SetFocus(handle);

                // let mut buffer: LPWSTR = ptr::null_mut();
                if res == 0 {
                    //println!("failed {} {}", winapi::um::errhandlingapi::GetLastError(), get_title(handle));
                    // if res == ptr::null_mut() {
                    //     println!("failed {} {}", winapi::um::errhandlingapi::GetLastError(), get_title(handle));
                } else {
                    let mut process_id: u32 = 0;
                    let _thread_id = GetWindowThreadProcessId(handle, &mut process_id as LPDWORD);
                    let mut buffer: Vec<u16> = vec![0; 100];

                    let proc_handle = winapi::um::processthreadsapi::OpenProcess(
                        PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                        1,
                        process_id,
                    );
                    let str_len = GetModuleFileNameW(
                        mem::transmute(proc_handle),
                        buffer.as_ptr() as LPWSTR,
                        100,
                    );
                    buffer.truncate(str_len as usize);
                    println!(
                        "success {}  {} {}",
                        process_id,
                        buf_to_string(&buffer),
                        get_title(handle)
                    );
                    if get_title(handle).len() == 0 {
                        println!(
                            "looking at {:?} {:?} {:?} {:?}",
                            rect.left, rect.right, rect.top, rect.bottom
                        );
                    }
                }
                break;
            }

        }

    }
}

#[derive(Debug, Clone, Copy)]
struct Resolution {
    width: i32,
    height: i32,
}

#[derive(Debug)]
struct SubscribeData {
    pub original: HWND,
    pub looking_at: HWND,
    pub since: i64,
    pub last_input_time: SystemTime,
}

fn main() -> Result<(), TobiiError> {
    unsafe {
        let custom_log = tobii_custom_log_t {
            log_context: ptr::null_mut(),
            log_func: Some(custom_log_fn),
        };

        // println!("{:?}", GetAsyncKeyState(10));
        // println!("{:?}", GetAsyncKeyState(1));

        //SystemParametersInfo(SPI_SETFOREGROUNDLOCKTIMEOUT,0,val,SPIF_SENDWININICHANGE | SPIF_UPDATEINIFILE);
        for handle in get_windows() {
            println!("{:?}", get_title(handle));
        }
        

        // let eligible = vec![];
        let mut handle = winuser::GetForegroundWindow();
        while handle != ptr::null_mut() {
            let mut rect: winapi::shared::windef::RECT = mem::zeroed();
            winuser::GetWindowRect(handle, &mut rect as *mut winapi::shared::windef::RECT);

            if handle == GetDesktopWindow() {
                break; // no
            }

            if is_eligible_for_activation(handle) {
                println!("ELIGIBLE {:?}", get_title(handle));
                // eligible.push(handle);
                // continue;
            }
            handle = GetWindow(handle, GW_HWNDNEXT);
        }

        // std::process::exit(0);

        println!("Initializing API!");
        let mut api_ptr: *mut tobii_api_t = mem::zeroed();
        let status = tobii_api_create(
            &mut api_ptr as *mut *mut tobii_api_t,
            ptr::null_mut(),
            &custom_log as *const _,
        );
        status_to_result(status)?;
        let api = PtrWrapper::new(api_ptr, tobii_api_destroy);

        let devices = helpers::list_devices(api.ptr())?;
        println!("{:?}", devices);

        if devices.len() < 1 {
            println!("No devices");
            return Ok(());
        }

        let url_c_string = CString::new(devices[0].clone()).unwrap();
        let url_c = url_c_string.as_c_str();
        let mut device_ptr: *mut tobii_device_t = mem::zeroed();
        let status = tobii_device_create(
            api.ptr(),
            url_c.as_ptr(),
            &mut device_ptr as *mut *mut tobii_device_t,
        );
        status_to_result(status)?;
        let device = PtrWrapper::new(device_ptr, tobii_device_destroy);

        let mut data = SubscribeData {
            original: winuser::GetForegroundWindow(),
            looking_at: winuser::GetForegroundWindow(),
            since: 0,
            last_input_time: SystemTime::now(),
        };

        // let status = tobii_gaze_point_subscribe(device.ptr(), Some(gaze_callback), ptr::null_mut());
        let status = tobii_gaze_point_subscribe(
            device.ptr(),
            Some(gaze_callback),
            mem::transmute(&mut data),
        );
        let _subscription = PtrWrapper::new(device.ptr(), tobii_gaze_point_unsubscribe);
        status_to_result(status)?;
        loop {
            use std::{thread, time};
            thread::sleep(time::Duration::from_millis(10));

            let status = helpers::wait_for_device_callbacks(device.ptr());
            match status_to_result(status) {
                Err(TobiiError::TimedOut) => continue,
                Err(TobiiError::ConnectionFailed) => {
                    status_to_result(helpers::reconnect(device.ptr()))?;
                    continue;
                }
                Err(e) => return Err(e),
                Ok(()) => (),
            }

            let status = tobii_device_process_callbacks(device.ptr());
            if status == tobii_error_t_TOBII_ERROR_CONNECTION_FAILED {
                status_to_result(helpers::reconnect(device.ptr()))?;
                continue;
            }
            status_to_result(status)?;
        }
    }
    // Ok(())
}
