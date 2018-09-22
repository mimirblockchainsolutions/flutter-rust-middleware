#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive; // 1.0.70
extern crate serde; // 1.0.70
use std::os::raw::c_char;
use std::ffi::{CString, CStr};

use std::fmt::Debug;
use serde::Serialize;

extern crate mimir_crypto;
extern crate mimir_common;
extern crate rand;
extern crate toml;
extern crate rlp;
//extern crate reqwest;
extern crate ring;
extern crate crypto;
mod transact;
mod transaction;
mod store;
mod error;
mod key;

use structopt::StructOpt;
use mimir_common::types::{Bytes, U256};
use mimir_crypto::secp256k1::Address;
// FLOW:
// app -> request_fuction -> Deserialize
// -> send Dispatcher function with provided arguments
// -> Dispatcher calls function
// -> Serialize return value -> send to app


#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
#[serde(rename_all = "kebab-case")]
enum Request {
    ListAccounts,
    CreateWallet,
    HelloJson,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Response {
    ListAccounts(Vec<String>),
    CreateWallet(Address),
    HelloJson(String),
}

fn list_accounts() -> Result<Response, ()> {
    Ok(Response::ListAccounts(vec!["test-acct"]))
}

fn create_wallet() -> Result<Address, Error> {
    let keys = key::keygen();
    store::store_keys(&keys)?;
    Ok(Response::CreateWallet(keys.address))
}

fn hello_json(name: String) -> Result<String, String> {
    Ok(format!("Hello {}", name))
}

fn dispatch(req: Request) -> String {
    match req {
        Request::ListAccounts() => ser_rslt(list_accounts()),
        Request::HelloJson(val) => ser_rslt(hello_json(val)),
        Request::CreateWallet() => ser_rslt(create_wallet().map_err(|e| e.to_string())),
    }
}

fn ser_rslt<T: Serialize + Debug, E: Serialize + Debug>(rslt: Result<Response, E>) -> String {
    match serde_json::to_string(&rslt) {
        Ok(serialized) => serialized,
        Err(_) => {
            let msg = format!("serialization failed for {:?}", rslt);
            let err: Result<String, String> = Err(msg);
            serde_json::to_string(&err).expect("must serialize")
        }
    }
}

#[no_mangle]
pub extern "C" fn request_function(payload: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(payload) };
    let r_string = c_str.to_string_lossy();
    let output = match serde_json::from_str(&r_string) {
        Ok(request) => dispatch(request),
        Err(error) => {
            let msg = format!("deserialization failed: {}", error);
            let err: Result<String, String> = Err(msg);
            serde_json::to_string(&err).expect("always serializes")
        }
    };
    CString::new(output).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn function_free(s: *mut c_char) {
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
