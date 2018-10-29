#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// include!("bindings.rs"));
pub mod helpers;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub const TOBII_DEVICE_GENERATION_G5: u32 = 2;
pub const TOBII_DEVICE_GENERATION_IS3: u32 = 4;
pub const TOBII_DEVICE_GENERATION_IS4: u32 = 8;
pub type int_least64_t = i64;
pub type uint_least64_t = u64;
pub type int_fast64_t = i64;
pub type uint_fast64_t = u64;
pub type int_least32_t = i32;
pub type uint_least32_t = u32;
pub type int_fast32_t = i32;
pub type uint_fast32_t = u32;
pub type int_least16_t = i16;
pub type uint_least16_t = u16;
pub type int_fast16_t = i16;
pub type uint_fast16_t = u16;
pub type int_least8_t = i8;
pub type uint_least8_t = u8;
pub type int_fast8_t = i8;
pub type uint_fast8_t = u8;
pub type intmax_t = ::std::os::raw::c_longlong;
pub type uintmax_t = ::std::os::raw::c_ulonglong;
pub type wchar_t = ::std::os::raw::c_ushort;
pub type max_align_t = f64;
pub const tobii_error_t_TOBII_ERROR_NO_ERROR: tobii_error_t = 0;
pub const tobii_error_t_TOBII_ERROR_INTERNAL: tobii_error_t = 1;
pub const tobii_error_t_TOBII_ERROR_INSUFFICIENT_LICENSE: tobii_error_t = 2;
pub const tobii_error_t_TOBII_ERROR_NOT_SUPPORTED: tobii_error_t = 3;
pub const tobii_error_t_TOBII_ERROR_NOT_AVAILABLE: tobii_error_t = 4;
pub const tobii_error_t_TOBII_ERROR_CONNECTION_FAILED: tobii_error_t = 5;
pub const tobii_error_t_TOBII_ERROR_TIMED_OUT: tobii_error_t = 6;
pub const tobii_error_t_TOBII_ERROR_ALLOCATION_FAILED: tobii_error_t = 7;
pub const tobii_error_t_TOBII_ERROR_INVALID_PARAMETER: tobii_error_t = 8;
pub const tobii_error_t_TOBII_ERROR_CALIBRATION_ALREADY_STARTED: tobii_error_t = 9;
pub const tobii_error_t_TOBII_ERROR_CALIBRATION_NOT_STARTED: tobii_error_t = 10;
pub const tobii_error_t_TOBII_ERROR_ALREADY_SUBSCRIBED: tobii_error_t = 11;
pub const tobii_error_t_TOBII_ERROR_NOT_SUBSCRIBED: tobii_error_t = 12;
pub const tobii_error_t_TOBII_ERROR_OPERATION_FAILED: tobii_error_t = 13;
pub const tobii_error_t_TOBII_ERROR_CONFLICTING_API_INSTANCES: tobii_error_t = 14;
pub const tobii_error_t_TOBII_ERROR_CALIBRATION_BUSY: tobii_error_t = 15;
pub const tobii_error_t_TOBII_ERROR_CALLBACK_IN_PROGRESS: tobii_error_t = 16;
pub const tobii_error_t_TOBII_ERROR_TOO_MANY_SUBSCRIBERS: tobii_error_t = 17;
pub type tobii_error_t = i32;
extern "C" {
    pub fn tobii_error_message(error: tobii_error_t) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tobii_version_t {
    pub major: ::std::os::raw::c_int,
    pub minor: ::std::os::raw::c_int,
    pub revision: ::std::os::raw::c_int,
    pub build: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_tobii_version_t() {
    assert_eq!(
        ::std::mem::size_of::<tobii_version_t>(),
        16usize,
        concat!("Size of: ", stringify!(tobii_version_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tobii_version_t>(),
        4usize,
        concat!("Alignment of ", stringify!(tobii_version_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_version_t>())).major as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_version_t),
            "::",
            stringify!(major)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_version_t>())).minor as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_version_t),
            "::",
            stringify!(minor)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_version_t>())).revision as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_version_t),
            "::",
            stringify!(revision)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_version_t>())).build as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_version_t),
            "::",
            stringify!(build)
        )
    );
}
extern "C" {
    pub fn tobii_get_api_version(version: *mut tobii_version_t) -> tobii_error_t;
}
pub const tobii_log_level_t_TOBII_LOG_LEVEL_ERROR: tobii_log_level_t = 0;
pub const tobii_log_level_t_TOBII_LOG_LEVEL_WARN: tobii_log_level_t = 1;
pub const tobii_log_level_t_TOBII_LOG_LEVEL_INFO: tobii_log_level_t = 2;
pub const tobii_log_level_t_TOBII_LOG_LEVEL_DEBUG: tobii_log_level_t = 3;
pub const tobii_log_level_t_TOBII_LOG_LEVEL_TRACE: tobii_log_level_t = 4;
pub type tobii_log_level_t = i32;
pub type tobii_log_func_t = ::std::option::Option<
    unsafe extern "C" fn(
        log_context: *mut ::std::os::raw::c_void,
        level: tobii_log_level_t,
        text: *const ::std::os::raw::c_char,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tobii_custom_log_t {
    pub log_context: *mut ::std::os::raw::c_void,
    pub log_func: tobii_log_func_t,
}
#[test]
fn bindgen_test_layout_tobii_custom_log_t() {
    assert_eq!(
        ::std::mem::size_of::<tobii_custom_log_t>(),
        16usize,
        concat!("Size of: ", stringify!(tobii_custom_log_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tobii_custom_log_t>(),
        8usize,
        concat!("Alignment of ", stringify!(tobii_custom_log_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_custom_log_t>())).log_context as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_custom_log_t),
            "::",
            stringify!(log_context)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_custom_log_t>())).log_func as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_custom_log_t),
            "::",
            stringify!(log_func)
        )
    );
}
pub type tobii_malloc_func_t = ::std::option::Option<
    unsafe extern "C" fn(mem_context: *mut ::std::os::raw::c_void, size: usize)
        -> *mut ::std::os::raw::c_void,
>;
pub type tobii_free_func_t = ::std::option::Option<
    unsafe extern "C" fn(
        mem_context: *mut ::std::os::raw::c_void,
        ptr: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tobii_custom_alloc_t {
    pub mem_context: *mut ::std::os::raw::c_void,
    pub malloc_func: tobii_malloc_func_t,
    pub free_func: tobii_free_func_t,
}
#[test]
fn bindgen_test_layout_tobii_custom_alloc_t() {
    assert_eq!(
        ::std::mem::size_of::<tobii_custom_alloc_t>(),
        24usize,
        concat!("Size of: ", stringify!(tobii_custom_alloc_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tobii_custom_alloc_t>(),
        8usize,
        concat!("Alignment of ", stringify!(tobii_custom_alloc_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_custom_alloc_t>())).mem_context as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_custom_alloc_t),
            "::",
            stringify!(mem_context)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_custom_alloc_t>())).malloc_func as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_custom_alloc_t),
            "::",
            stringify!(malloc_func)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_custom_alloc_t>())).free_func as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_custom_alloc_t),
            "::",
            stringify!(free_func)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tobii_api_t {
    _unused: [u8; 0],
}
extern "C" {
    pub fn tobii_api_create(
        api: *mut *mut tobii_api_t,
        custom_alloc: *const tobii_custom_alloc_t,
        custom_log: *const tobii_custom_log_t,
    ) -> tobii_error_t;
}
extern "C" {
    pub fn tobii_api_destroy(api: *mut tobii_api_t) -> tobii_error_t;
}
extern "C" {
    pub fn tobii_system_clock(api: *mut tobii_api_t, timestamp_us: *mut i64) -> tobii_error_t;
}
pub type tobii_device_url_receiver_t = ::std::option::Option<
    unsafe extern "C" fn(
        url: *const ::std::os::raw::c_char,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    pub fn tobii_enumerate_local_device_urls(
        api: *mut tobii_api_t,
        receiver: tobii_device_url_receiver_t,
        user_data: *mut ::std::os::raw::c_void,
    ) -> tobii_error_t;
}
extern "C" {
    pub fn tobii_enumerate_local_device_urls_ex(
        api: *mut tobii_api_t,
        receiver: tobii_device_url_receiver_t,
        user_data: *mut ::std::os::raw::c_void,
        device_generations: u32,
    ) -> tobii_error_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tobii_engine_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tobii_device_t {
    _unused: [u8; 0],
}
extern "C" {
    pub fn tobii_wait_for_callbacks(
        engine: *mut tobii_engine_t,
        device_count: ::std::os::raw::c_int,
        devices: *const *mut tobii_device_t,
    ) -> tobii_error_t;
}
extern "C" {
    pub fn tobii_device_create(
        api: *mut tobii_api_t,
        url: *const ::std::os::raw::c_char,
        device: *mut *mut tobii_device_t,
    ) -> tobii_error_t;
}
extern "C" {
    pub fn tobii_device_destroy(device: *mut tobii_device_t) -> tobii_error_t;
}
extern "C" {
    pub fn tobii_device_reconnect(device: *mut tobii_device_t) -> tobii_error_t;
}
extern "C" {
    pub fn tobii_device_process_callbacks(device: *mut tobii_device_t) -> tobii_error_t;
}
extern "C" {
    pub fn tobii_device_clear_callback_buffers(device: *mut tobii_device_t) -> tobii_error_t;
}
extern "C" {
    pub fn tobii_update_timesync(device: *mut tobii_device_t) -> tobii_error_t;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tobii_device_info_t {
    pub serial_number: [::std::os::raw::c_char; 128usize],
    pub model: [::std::os::raw::c_char; 64usize],
    pub generation: [::std::os::raw::c_char; 64usize],
    pub firmware_version: [::std::os::raw::c_char; 128usize],
}
#[test]
fn bindgen_test_layout_tobii_device_info_t() {
    assert_eq!(
        ::std::mem::size_of::<tobii_device_info_t>(),
        384usize,
        concat!("Size of: ", stringify!(tobii_device_info_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tobii_device_info_t>(),
        1usize,
        concat!("Alignment of ", stringify!(tobii_device_info_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_device_info_t>())).serial_number as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_device_info_t),
            "::",
            stringify!(serial_number)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_device_info_t>())).model as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_device_info_t),
            "::",
            stringify!(model)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_device_info_t>())).generation as *const _ as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_device_info_t),
            "::",
            stringify!(generation)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_device_info_t>())).firmware_version as *const _ as usize
        },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_device_info_t),
            "::",
            stringify!(firmware_version)
        )
    );
}
extern "C" {
    pub fn tobii_get_device_info(
        device: *mut tobii_device_t,
        device_info: *mut tobii_device_info_t,
    ) -> tobii_error_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tobii_track_box_t {
    pub front_upper_right_xyz: [f32; 3usize],
    pub front_upper_left_xyz: [f32; 3usize],
    pub front_lower_left_xyz: [f32; 3usize],
    pub front_lower_right_xyz: [f32; 3usize],
    pub back_upper_right_xyz: [f32; 3usize],
    pub back_upper_left_xyz: [f32; 3usize],
    pub back_lower_left_xyz: [f32; 3usize],
    pub back_lower_right_xyz: [f32; 3usize],
}
#[test]
fn bindgen_test_layout_tobii_track_box_t() {
    assert_eq!(
        ::std::mem::size_of::<tobii_track_box_t>(),
        96usize,
        concat!("Size of: ", stringify!(tobii_track_box_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tobii_track_box_t>(),
        4usize,
        concat!("Alignment of ", stringify!(tobii_track_box_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_track_box_t>())).front_upper_right_xyz as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_track_box_t),
            "::",
            stringify!(front_upper_right_xyz)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_track_box_t>())).front_upper_left_xyz as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_track_box_t),
            "::",
            stringify!(front_upper_left_xyz)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_track_box_t>())).front_lower_left_xyz as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_track_box_t),
            "::",
            stringify!(front_lower_left_xyz)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_track_box_t>())).front_lower_right_xyz as *const _ as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_track_box_t),
            "::",
            stringify!(front_lower_right_xyz)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_track_box_t>())).back_upper_right_xyz as *const _ as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_track_box_t),
            "::",
            stringify!(back_upper_right_xyz)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_track_box_t>())).back_upper_left_xyz as *const _ as usize
        },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_track_box_t),
            "::",
            stringify!(back_upper_left_xyz)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_track_box_t>())).back_lower_left_xyz as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_track_box_t),
            "::",
            stringify!(back_lower_left_xyz)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_track_box_t>())).back_lower_right_xyz as *const _ as usize
        },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_track_box_t),
            "::",
            stringify!(back_lower_right_xyz)
        )
    );
}
extern "C" {
    pub fn tobii_get_track_box(
        device: *mut tobii_device_t,
        track_box: *mut tobii_track_box_t,
    ) -> tobii_error_t;
}
pub const tobii_state_t_TOBII_STATE_POWER_SAVE_ACTIVE: tobii_state_t = 0;
pub const tobii_state_t_TOBII_STATE_REMOTE_WAKE_ACTIVE: tobii_state_t = 1;
pub const tobii_state_t_TOBII_STATE_DEVICE_PAUSED: tobii_state_t = 2;
pub const tobii_state_t_TOBII_STATE_EXCLUSIVE_MODE: tobii_state_t = 3;
pub const tobii_state_t_TOBII_STATE_FAULT: tobii_state_t = 4;
pub const tobii_state_t_TOBII_STATE_WARNING: tobii_state_t = 5;
pub const tobii_state_t_TOBII_STATE_CALIBRATION_ID: tobii_state_t = 6;
pub const tobii_state_t_TOBII_STATE_CALIBRATION_ACTIVE: tobii_state_t = 7;
pub type tobii_state_t = i32;
pub const tobii_state_bool_t_TOBII_STATE_BOOL_FALSE: tobii_state_bool_t = 0;
pub const tobii_state_bool_t_TOBII_STATE_BOOL_TRUE: tobii_state_bool_t = 1;
pub type tobii_state_bool_t = i32;
extern "C" {
    pub fn tobii_get_state_bool(
        device: *mut tobii_device_t,
        state: tobii_state_t,
        value: *mut tobii_state_bool_t,
    ) -> tobii_error_t;
}
extern "C" {
    pub fn tobii_get_state_uint32(
        device: *mut tobii_device_t,
        state: tobii_state_t,
        value: *mut u32,
    ) -> tobii_error_t;
}
pub type tobii_state_string_t = [::std::os::raw::c_char; 512usize];
extern "C" {
    pub fn tobii_get_state_string(
        device: *mut tobii_device_t,
        state: tobii_state_t,
        value: *mut ::std::os::raw::c_char,
    ) -> tobii_error_t;
}
pub const tobii_supported_t_TOBII_NOT_SUPPORTED: tobii_supported_t = 0;
pub const tobii_supported_t_TOBII_SUPPORTED: tobii_supported_t = 1;
pub type tobii_supported_t = i32;
pub const tobii_capability_t_TOBII_CAPABILITY_DISPLAY_AREA_WRITABLE: tobii_capability_t = 0;
pub const tobii_capability_t_TOBII_CAPABILITY_CALIBRATION_2D: tobii_capability_t = 1;
pub const tobii_capability_t_TOBII_CAPABILITY_CALIBRATION_3D: tobii_capability_t = 2;
pub const tobii_capability_t_TOBII_CAPABILITY_PERSISTENT_STORAGE: tobii_capability_t = 3;
pub const tobii_capability_t_TOBII_CAPABILITY_CALIBRATION_PER_EYE: tobii_capability_t = 4;
pub const tobii_capability_t_TOBII_CAPABILITY_COMBINED_GAZE_VR: tobii_capability_t = 5;
pub const tobii_capability_t_TOBII_CAPABILITY_FACE_TYPE: tobii_capability_t = 6;
pub type tobii_capability_t = i32;
extern "C" {
    pub fn tobii_capability_supported(
        device: *mut tobii_device_t,
        capability: tobii_capability_t,
        supported: *mut tobii_supported_t,
    ) -> tobii_error_t;
}
pub const tobii_stream_t_TOBII_STREAM_GAZE_POINT: tobii_stream_t = 0;
pub const tobii_stream_t_TOBII_STREAM_GAZE_ORIGIN: tobii_stream_t = 1;
pub const tobii_stream_t_TOBII_STREAM_EYE_POSITION_NORMALIZED: tobii_stream_t = 2;
pub const tobii_stream_t_TOBII_STREAM_USER_PRESENCE: tobii_stream_t = 3;
pub const tobii_stream_t_TOBII_STREAM_HEAD_POSE: tobii_stream_t = 4;
pub const tobii_stream_t_TOBII_STREAM_WEARABLE: tobii_stream_t = 5;
pub const tobii_stream_t_TOBII_STREAM_GAZE_DATA: tobii_stream_t = 6;
pub const tobii_stream_t_TOBII_STREAM_DIGITAL_SYNCPORT: tobii_stream_t = 7;
pub const tobii_stream_t_TOBII_STREAM_DIAGNOSTICS_IMAGE: tobii_stream_t = 8;
pub type tobii_stream_t = i32;
extern "C" {
    pub fn tobii_stream_supported(
        device: *mut tobii_device_t,
        stream: tobii_stream_t,
        supported: *mut tobii_supported_t,
    ) -> tobii_error_t;
}
pub type tobii_data_receiver_t = ::std::option::Option<
    unsafe extern "C" fn(
        data: *const ::std::os::raw::c_void,
        size: usize,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;
pub const tobii_validity_t_TOBII_VALIDITY_INVALID: tobii_validity_t = 0;
pub const tobii_validity_t_TOBII_VALIDITY_VALID: tobii_validity_t = 1;
pub type tobii_validity_t = i32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tobii_display_area_t {
    pub top_left_mm_xyz: [f32; 3usize],
    pub top_right_mm_xyz: [f32; 3usize],
    pub bottom_left_mm_xyz: [f32; 3usize],
}
#[test]
fn bindgen_test_layout_tobii_display_area_t() {
    assert_eq!(
        ::std::mem::size_of::<tobii_display_area_t>(),
        36usize,
        concat!("Size of: ", stringify!(tobii_display_area_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tobii_display_area_t>(),
        4usize,
        concat!("Alignment of ", stringify!(tobii_display_area_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_display_area_t>())).top_left_mm_xyz as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_display_area_t),
            "::",
            stringify!(top_left_mm_xyz)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_display_area_t>())).top_right_mm_xyz as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_display_area_t),
            "::",
            stringify!(top_right_mm_xyz)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_display_area_t>())).bottom_left_mm_xyz as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_display_area_t),
            "::",
            stringify!(bottom_left_mm_xyz)
        )
    );
}
pub const tobii_enabled_eye_t_TOBII_ENABLED_EYE_LEFT: tobii_enabled_eye_t = 0;
pub const tobii_enabled_eye_t_TOBII_ENABLED_EYE_RIGHT: tobii_enabled_eye_t = 1;
pub const tobii_enabled_eye_t_TOBII_ENABLED_EYE_BOTH: tobii_enabled_eye_t = 2;
pub type tobii_enabled_eye_t = i32;
extern "C" {
    pub fn tobii_engine_create(
        api: *mut tobii_api_t,
        engine: *mut *mut tobii_engine_t,
    ) -> tobii_error_t;
}
extern "C" {
    pub fn tobii_engine_destroy(engine: *mut tobii_engine_t) -> tobii_error_t;
}
extern "C" {
    pub fn tobii_engine_reconnect(engine: *mut tobii_engine_t) -> tobii_error_t;
}
extern "C" {
    pub fn tobii_engine_process_callbacks(engine: *mut tobii_engine_t) -> tobii_error_t;
}
extern "C" {
    pub fn tobii_engine_clear_callback_buffers(engine: *mut tobii_engine_t) -> tobii_error_t;
}
pub const tobii_device_readiness_t_TOBII_DEVICE_READINESS_WAITING_FOR_FIRMWARE_UPGRADE:
    tobii_device_readiness_t = 0;
pub const tobii_device_readiness_t_TOBII_DEVICE_READINESS_UPGRADING_FIRMWARE:
    tobii_device_readiness_t = 1;
pub const tobii_device_readiness_t_TOBII_DEVICE_READINESS_WAITING_FOR_DISPLAY_AREA:
    tobii_device_readiness_t = 2;
pub const tobii_device_readiness_t_TOBII_DEVICE_READINESS_WAITING_FOR_CALIBRATION:
    tobii_device_readiness_t = 3;
pub const tobii_device_readiness_t_TOBII_DEVICE_READINESS_CALIBRATING: tobii_device_readiness_t = 4;
pub const tobii_device_readiness_t_TOBII_DEVICE_READINESS_READY: tobii_device_readiness_t = 5;
pub const tobii_device_readiness_t_TOBII_DEVICE_READINESS_PAUSED: tobii_device_readiness_t = 6;
pub const tobii_device_readiness_t_TOBII_DEVICE_READINESS_MALFUNCTIONING: tobii_device_readiness_t =
    7;
pub type tobii_device_readiness_t = i32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tobii_enumerated_device_t {
    pub url: [::std::os::raw::c_char; 256usize],
    pub serial_number: [::std::os::raw::c_char; 128usize],
    pub model: [::std::os::raw::c_char; 64usize],
    pub generation: [::std::os::raw::c_char; 64usize],
    pub firmware_version: [::std::os::raw::c_char; 128usize],
    pub integration: [::std::os::raw::c_char; 120usize],
    pub readiness: tobii_device_readiness_t,
}
#[test]
fn bindgen_test_layout_tobii_enumerated_device_t() {
    assert_eq!(
        ::std::mem::size_of::<tobii_enumerated_device_t>(),
        764usize,
        concat!("Size of: ", stringify!(tobii_enumerated_device_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tobii_enumerated_device_t>(),
        4usize,
        concat!("Alignment of ", stringify!(tobii_enumerated_device_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_enumerated_device_t>())).url as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_enumerated_device_t),
            "::",
            stringify!(url)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_enumerated_device_t>())).serial_number as *const _ as usize
        },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_enumerated_device_t),
            "::",
            stringify!(serial_number)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_enumerated_device_t>())).model as *const _ as usize },
        384usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_enumerated_device_t),
            "::",
            stringify!(model)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_enumerated_device_t>())).generation as *const _ as usize
        },
        448usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_enumerated_device_t),
            "::",
            stringify!(generation)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_enumerated_device_t>())).firmware_version as *const _
                as usize
        },
        512usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_enumerated_device_t),
            "::",
            stringify!(firmware_version)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_enumerated_device_t>())).integration as *const _ as usize
        },
        640usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_enumerated_device_t),
            "::",
            stringify!(integration)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_enumerated_device_t>())).readiness as *const _ as usize
        },
        760usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_enumerated_device_t),
            "::",
            stringify!(readiness)
        )
    );
}
pub type tobii_enumerated_device_receiver_t = ::std::option::Option<
    unsafe extern "C" fn(
        enumerated_device: *const tobii_enumerated_device_t,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    pub fn tobii_enumerate_devices(
        engine: *mut tobii_engine_t,
        receiver: tobii_enumerated_device_receiver_t,
        user_data: *mut ::std::os::raw::c_void,
    ) -> tobii_error_t;
}
pub const tobii_device_list_change_type_t_TOBII_DEVICE_LIST_CHANGE_TYPE_ADDED:
    tobii_device_list_change_type_t = 0;
pub const tobii_device_list_change_type_t_TOBII_DEVICE_LIST_CHANGE_TYPE_REMOVED:
    tobii_device_list_change_type_t = 1;
pub const tobii_device_list_change_type_t_TOBII_DEVICE_LIST_CHANGE_TYPE_CHANGED:
    tobii_device_list_change_type_t = 2;
pub type tobii_device_list_change_type_t = i32;
pub type tobii_device_list_change_callback_t = ::std::option::Option<
    unsafe extern "C" fn(
        url: *const ::std::os::raw::c_char,
        type_: tobii_device_list_change_type_t,
        readiness: tobii_device_readiness_t,
        timestamp_us: i64,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    pub fn tobii_device_list_change_subscribe(
        engine: *mut tobii_engine_t,
        callback: tobii_device_list_change_callback_t,
        user_data: *mut ::std::os::raw::c_void,
    ) -> tobii_error_t;
}
extern "C" {
    pub fn tobii_device_list_change_unsubscribe(engine: *mut tobii_engine_t) -> tobii_error_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tobii_gaze_point_t {
    pub timestamp_us: i64,
    pub validity: tobii_validity_t,
    pub position_xy: [f32; 2usize],
}
#[test]
fn bindgen_test_layout_tobii_gaze_point_t() {
    assert_eq!(
        ::std::mem::size_of::<tobii_gaze_point_t>(),
        24usize,
        concat!("Size of: ", stringify!(tobii_gaze_point_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tobii_gaze_point_t>(),
        8usize,
        concat!("Alignment of ", stringify!(tobii_gaze_point_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_gaze_point_t>())).timestamp_us as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_gaze_point_t),
            "::",
            stringify!(timestamp_us)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_gaze_point_t>())).validity as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_gaze_point_t),
            "::",
            stringify!(validity)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_gaze_point_t>())).position_xy as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_gaze_point_t),
            "::",
            stringify!(position_xy)
        )
    );
}
pub type tobii_gaze_point_callback_t = ::std::option::Option<
    unsafe extern "C" fn(
        gaze_point: *const tobii_gaze_point_t,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    pub fn tobii_gaze_point_subscribe(
        device: *mut tobii_device_t,
        callback: tobii_gaze_point_callback_t,
        user_data: *mut ::std::os::raw::c_void,
    ) -> tobii_error_t;
}
extern "C" {
    pub fn tobii_gaze_point_unsubscribe(device: *mut tobii_device_t) -> tobii_error_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tobii_gaze_origin_t {
    pub timestamp_us: i64,
    pub left_validity: tobii_validity_t,
    pub left_xyz: [f32; 3usize],
    pub right_validity: tobii_validity_t,
    pub right_xyz: [f32; 3usize],
}
#[test]
fn bindgen_test_layout_tobii_gaze_origin_t() {
    assert_eq!(
        ::std::mem::size_of::<tobii_gaze_origin_t>(),
        40usize,
        concat!("Size of: ", stringify!(tobii_gaze_origin_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tobii_gaze_origin_t>(),
        8usize,
        concat!("Alignment of ", stringify!(tobii_gaze_origin_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_gaze_origin_t>())).timestamp_us as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_gaze_origin_t),
            "::",
            stringify!(timestamp_us)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_gaze_origin_t>())).left_validity as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_gaze_origin_t),
            "::",
            stringify!(left_validity)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_gaze_origin_t>())).left_xyz as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_gaze_origin_t),
            "::",
            stringify!(left_xyz)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_gaze_origin_t>())).right_validity as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_gaze_origin_t),
            "::",
            stringify!(right_validity)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_gaze_origin_t>())).right_xyz as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_gaze_origin_t),
            "::",
            stringify!(right_xyz)
        )
    );
}
pub type tobii_gaze_origin_callback_t = ::std::option::Option<
    unsafe extern "C" fn(
        gaze_origin: *const tobii_gaze_origin_t,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    pub fn tobii_gaze_origin_subscribe(
        device: *mut tobii_device_t,
        callback: tobii_gaze_origin_callback_t,
        user_data: *mut ::std::os::raw::c_void,
    ) -> tobii_error_t;
}
extern "C" {
    pub fn tobii_gaze_origin_unsubscribe(device: *mut tobii_device_t) -> tobii_error_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tobii_eye_position_normalized_t {
    pub timestamp_us: i64,
    pub left_validity: tobii_validity_t,
    pub left_xyz: [f32; 3usize],
    pub right_validity: tobii_validity_t,
    pub right_xyz: [f32; 3usize],
}
#[test]
fn bindgen_test_layout_tobii_eye_position_normalized_t() {
    assert_eq!(
        ::std::mem::size_of::<tobii_eye_position_normalized_t>(),
        40usize,
        concat!("Size of: ", stringify!(tobii_eye_position_normalized_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tobii_eye_position_normalized_t>(),
        8usize,
        concat!("Alignment of ", stringify!(tobii_eye_position_normalized_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_eye_position_normalized_t>())).timestamp_us as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_eye_position_normalized_t),
            "::",
            stringify!(timestamp_us)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_eye_position_normalized_t>())).left_validity as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_eye_position_normalized_t),
            "::",
            stringify!(left_validity)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_eye_position_normalized_t>())).left_xyz as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_eye_position_normalized_t),
            "::",
            stringify!(left_xyz)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_eye_position_normalized_t>())).right_validity as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_eye_position_normalized_t),
            "::",
            stringify!(right_validity)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_eye_position_normalized_t>())).right_xyz as *const _
                as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_eye_position_normalized_t),
            "::",
            stringify!(right_xyz)
        )
    );
}
pub type tobii_eye_position_normalized_callback_t = ::std::option::Option<
    unsafe extern "C" fn(
        eye_position: *const tobii_eye_position_normalized_t,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    pub fn tobii_eye_position_normalized_subscribe(
        device: *mut tobii_device_t,
        callback: tobii_eye_position_normalized_callback_t,
        user_data: *mut ::std::os::raw::c_void,
    ) -> tobii_error_t;
}
extern "C" {
    pub fn tobii_eye_position_normalized_unsubscribe(device: *mut tobii_device_t) -> tobii_error_t;
}
pub const tobii_user_presence_status_t_TOBII_USER_PRESENCE_STATUS_UNKNOWN:
    tobii_user_presence_status_t = 0;
pub const tobii_user_presence_status_t_TOBII_USER_PRESENCE_STATUS_AWAY:
    tobii_user_presence_status_t = 1;
pub const tobii_user_presence_status_t_TOBII_USER_PRESENCE_STATUS_PRESENT:
    tobii_user_presence_status_t = 2;
pub type tobii_user_presence_status_t = i32;
pub type tobii_user_presence_callback_t = ::std::option::Option<
    unsafe extern "C" fn(
        status: tobii_user_presence_status_t,
        timestamp_us: i64,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    pub fn tobii_user_presence_subscribe(
        device: *mut tobii_device_t,
        callback: tobii_user_presence_callback_t,
        user_data: *mut ::std::os::raw::c_void,
    ) -> tobii_error_t;
}
extern "C" {
    pub fn tobii_user_presence_unsubscribe(device: *mut tobii_device_t) -> tobii_error_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tobii_head_pose_t {
    pub timestamp_us: i64,
    pub position_validity: tobii_validity_t,
    pub position_xyz: [f32; 3usize],
    pub rotation_validity_xyz: [tobii_validity_t; 3usize],
    pub rotation_xyz: [f32; 3usize],
}
#[test]
fn bindgen_test_layout_tobii_head_pose_t() {
    assert_eq!(
        ::std::mem::size_of::<tobii_head_pose_t>(),
        48usize,
        concat!("Size of: ", stringify!(tobii_head_pose_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tobii_head_pose_t>(),
        8usize,
        concat!("Alignment of ", stringify!(tobii_head_pose_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_head_pose_t>())).timestamp_us as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_head_pose_t),
            "::",
            stringify!(timestamp_us)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_head_pose_t>())).position_validity as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_head_pose_t),
            "::",
            stringify!(position_validity)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_head_pose_t>())).position_xyz as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_head_pose_t),
            "::",
            stringify!(position_xyz)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_head_pose_t>())).rotation_validity_xyz as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_head_pose_t),
            "::",
            stringify!(rotation_validity_xyz)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_head_pose_t>())).rotation_xyz as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_head_pose_t),
            "::",
            stringify!(rotation_xyz)
        )
    );
}
pub type tobii_head_pose_callback_t = ::std::option::Option<
    unsafe extern "C" fn(
        head_pose: *const tobii_head_pose_t,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    pub fn tobii_head_pose_subscribe(
        device: *mut tobii_device_t,
        callback: tobii_head_pose_callback_t,
        user_data: *mut ::std::os::raw::c_void,
    ) -> tobii_error_t;
}
extern "C" {
    pub fn tobii_head_pose_unsubscribe(device: *mut tobii_device_t) -> tobii_error_t;
}
pub const tobii_notification_type_t_TOBII_NOTIFICATION_TYPE_CALIBRATION_STATE_CHANGED:
    tobii_notification_type_t = 0;
pub const tobii_notification_type_t_TOBII_NOTIFICATION_TYPE_EXCLUSIVE_MODE_STATE_CHANGED:
    tobii_notification_type_t = 1;
pub const tobii_notification_type_t_TOBII_NOTIFICATION_TYPE_TRACK_BOX_CHANGED:
    tobii_notification_type_t = 2;
pub const tobii_notification_type_t_TOBII_NOTIFICATION_TYPE_DISPLAY_AREA_CHANGED:
    tobii_notification_type_t = 3;
pub const tobii_notification_type_t_TOBII_NOTIFICATION_TYPE_FRAMERATE_CHANGED:
    tobii_notification_type_t = 4;
pub const tobii_notification_type_t_TOBII_NOTIFICATION_TYPE_POWER_SAVE_STATE_CHANGED:
    tobii_notification_type_t = 5;
pub const tobii_notification_type_t_TOBII_NOTIFICATION_TYPE_DEVICE_PAUSED_STATE_CHANGED:
    tobii_notification_type_t = 6;
pub const tobii_notification_type_t_TOBII_NOTIFICATION_TYPE_CALIBRATION_ENABLED_EYE_CHANGED:
    tobii_notification_type_t = 7;
pub const tobii_notification_type_t_TOBII_NOTIFICATION_TYPE_CALIBRATION_ID_CHANGED:
    tobii_notification_type_t = 8;
pub const tobii_notification_type_t_TOBII_NOTIFICATION_TYPE_COMBINED_GAZE_FACTOR_CHANGED:
    tobii_notification_type_t = 9;
pub const tobii_notification_type_t_TOBII_NOTIFICATION_TYPE_FAULTS_CHANGED:
    tobii_notification_type_t = 10;
pub const tobii_notification_type_t_TOBII_NOTIFICATION_TYPE_WARNINGS_CHANGED:
    tobii_notification_type_t = 11;
pub const tobii_notification_type_t_TOBII_NOTIFICATION_TYPE_FACE_TYPE_CHANGED:
    tobii_notification_type_t = 12;
pub type tobii_notification_type_t = i32;
pub const tobii_notification_value_type_t_TOBII_NOTIFICATION_VALUE_TYPE_NONE:
    tobii_notification_value_type_t = 0;
pub const tobii_notification_value_type_t_TOBII_NOTIFICATION_VALUE_TYPE_FLOAT:
    tobii_notification_value_type_t = 1;
pub const tobii_notification_value_type_t_TOBII_NOTIFICATION_VALUE_TYPE_STATE:
    tobii_notification_value_type_t = 2;
pub const tobii_notification_value_type_t_TOBII_NOTIFICATION_VALUE_TYPE_DISPLAY_AREA:
    tobii_notification_value_type_t = 3;
pub const tobii_notification_value_type_t_TOBII_NOTIFICATION_VALUE_TYPE_UINT:
    tobii_notification_value_type_t = 4;
pub const tobii_notification_value_type_t_TOBII_NOTIFICATION_VALUE_TYPE_ENABLED_EYE:
    tobii_notification_value_type_t = 5;
pub const tobii_notification_value_type_t_TOBII_NOTIFICATION_VALUE_TYPE_STRING:
    tobii_notification_value_type_t = 6;
pub type tobii_notification_value_type_t = i32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tobii_notification_t {
    pub type_: tobii_notification_type_t,
    pub value_type: tobii_notification_value_type_t,
    pub value: tobii_notification_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union tobii_notification_t__bindgen_ty_1 {
    pub float_: f32,
    pub state: tobii_state_bool_t,
    pub display_area: tobii_display_area_t,
    pub uint_: u32,
    pub enabled_eye: tobii_enabled_eye_t,
    pub string_: tobii_state_string_t,
    _bindgen_union_align: [u32; 128usize],
}
#[test]
fn bindgen_test_layout_tobii_notification_t__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<tobii_notification_t__bindgen_ty_1>(),
        512usize,
        concat!("Size of: ", stringify!(tobii_notification_t__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<tobii_notification_t__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(tobii_notification_t__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_notification_t__bindgen_ty_1>())).float_ as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_notification_t__bindgen_ty_1),
            "::",
            stringify!(float_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_notification_t__bindgen_ty_1>())).state as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_notification_t__bindgen_ty_1),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_notification_t__bindgen_ty_1>())).display_area as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_notification_t__bindgen_ty_1),
            "::",
            stringify!(display_area)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_notification_t__bindgen_ty_1>())).uint_ as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_notification_t__bindgen_ty_1),
            "::",
            stringify!(uint_)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_notification_t__bindgen_ty_1>())).enabled_eye as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_notification_t__bindgen_ty_1),
            "::",
            stringify!(enabled_eye)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tobii_notification_t__bindgen_ty_1>())).string_ as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_notification_t__bindgen_ty_1),
            "::",
            stringify!(string_)
        )
    );
}
#[test]
fn bindgen_test_layout_tobii_notification_t() {
    assert_eq!(
        ::std::mem::size_of::<tobii_notification_t>(),
        520usize,
        concat!("Size of: ", stringify!(tobii_notification_t))
    );
    assert_eq!(
        ::std::mem::align_of::<tobii_notification_t>(),
        4usize,
        concat!("Alignment of ", stringify!(tobii_notification_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_notification_t>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_notification_t),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_notification_t>())).value_type as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_notification_t),
            "::",
            stringify!(value_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tobii_notification_t>())).value as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tobii_notification_t),
            "::",
            stringify!(value)
        )
    );
}
pub type tobii_notifications_callback_t = ::std::option::Option<
    unsafe extern "C" fn(
        notification: *const tobii_notification_t,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    pub fn tobii_notifications_subscribe(
        device: *mut tobii_device_t,
        callback: tobii_notifications_callback_t,
        user_data: *mut ::std::os::raw::c_void,
    ) -> tobii_error_t;
}
extern "C" {
    pub fn tobii_notifications_unsubscribe(device: *mut tobii_device_t) -> tobii_error_t;
}
