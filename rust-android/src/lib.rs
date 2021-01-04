#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use std::ffi::{CString, CStr};
use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::{jstring};
use rust_core::make_request;

#[no_mangle]
pub unsafe extern fn Java_com_cultivatedsoftware_positivelys_services_AppService_makeapprequest(env: JNIEnv, _: JObject, j_app_request: JString, j_app_context: JString) -> jstring {
    let app_request_c_string = CString::from(
        CStr::from_ptr(
            env.get_string(j_app_request).unwrap().as_ptr()
        )
    );
    let app_request = app_request_c_string.to_str().unwrap().to_string();

    let app_context_c_string = CString::from(
        CStr::from_ptr(
            env.get_string(j_app_context).unwrap().as_ptr()
        )
    );
    let app_context = app_context_c_string.to_str().unwrap().to_string();

    let response_as_json = make_request(app_request, app_context);

    let output = env.new_string(response_as_json.to_owned()).unwrap();
    output.into_inner()
}