use std::os::raw::{c_char};
use std::ffi::{CString, CStr};
use rust_core::make_request;

#[no_mangle]
pub extern fn make_app_request(c_char_app_request: *const c_char, c_char_app_context: *const c_char) -> *mut c_char {
    let c_str_app_request = unsafe { CStr::from_ptr(c_char_app_request) };
    let c_str_app_context = unsafe { CStr::from_ptr(c_char_app_context) };

    let request_as_json = c_str_app_request.to_str().unwrap().to_string();
    let app_context_as_json = c_str_app_context.to_str().unwrap().to_string();
    let response_as_json = make_request(request_as_json, app_context_as_json);

    CString::new(response_as_json).unwrap().into_raw()
}

#[no_mangle]
pub extern fn make_app_request_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}