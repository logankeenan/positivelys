use std::os::raw::{c_char};
use std::ffi::{CString, CStr};
use rust_core::make_request;

#[no_mangle]
pub extern fn make_app_request(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };

    let request_as_json = c_str.to_str().unwrap().to_string();
    let response_as_json = make_request(request_as_json);

    CString::new(response_as_json).unwrap().into_raw()
}

#[no_mangle]
pub extern fn make_app_request_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}