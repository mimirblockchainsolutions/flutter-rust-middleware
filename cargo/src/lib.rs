extern crate serde_json;
use std::os::raw::c_char;
use std::ffi::{CString, CStr};

// FLOW:
// app -> request_fuction -> Deserialize
// -> send Dispatcher function with provided arguments
// -> Dispatcher calls function
// -> Serialize return value -> send to app


#[no_mangle]
pub extern "C" fn request_function(payload: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(payload) };
    let rslt = handle_input(&c_str);
    let output = serde_json::to_string(&rslt).expect("always serializes");
    CString::new(output).unwrap().into_raw()
}

fn handle_input(cstr: &CStr) -> Result<String, String> {
    match &cstr.to_str() {
        Err(_) => Ok("Error: No input".to_string()),
        Ok(_string) => Ok("testing".to_string()),
    }
}

#[no_mangle]
pub extern "C" fn request_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

/// Expose the JNI interface for android below
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::jstring;

    #[no_mangle]
    pub unsafe extern "C" fn Java_labs_mimir_middleware_MiddleWare_result(
        env: JNIEnv,
        _: JClass,
        java_pattern: JString,
    ) -> jstring {
        // Our Java companion code might pass-in "world" as a string, hence the name.
        let payload = request_function(
            env.get_string(java_pattern)
                .expect("invalid pattern string")
                .as_ptr(),
        );
        // Retake pointer so that we can use it below and allow memory to be freed when it goes out of scope.
        let payload_ptr = CString::from_raw(payload);
        let output = env.new_string(payload_ptr.to_str().unwrap()).expect(
            "Couldn't create java string!",
        );

        output.into_inner()
    }
}
