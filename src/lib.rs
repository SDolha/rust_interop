use std::ffi::{c_char, CStr, CString};
use wasm_bindgen::prelude::*;

pub struct Object {
    pub flag: bool,
    pub data: String,
}

pub fn update(object: Object) -> Object {
    Object {
        flag: true,
        data: object.data + "\nRust Update=\"👏\"",
    }
}

// Rust interop
#[no_mangle]
pub fn rust_update(object: Object) -> Object {
    update(object)
}

// C interop
#[repr(C)]
pub struct CObject {
    pub flag: bool,
    pub data: *const c_char,
}

fn from_c_object(object: CObject) -> Object {
    Object {
        flag: object.flag,
        data: from_c_string(object.data),
    }
}
fn to_c_object(object: Object) -> CObject {
    CObject {
        flag: object.flag,
        data: to_c_string(object.data),
    }
}

fn from_c_string(value: *const c_char) -> String {
    let value = unsafe { CStr::from_ptr(value) };
    let value = value.to_bytes();
    let value = String::from_utf8_lossy(value);
    value.to_string()
}
fn to_c_string(value: String) -> *const c_char {
    let value = CString::new(value);
    let value = value.unwrap();
    value.into_raw()
}

#[no_mangle]
pub extern "C" fn c_update(object: CObject) -> CObject {
    let object = from_c_object(object);
    let object = update(object);
    to_c_object(object)
}

// JavaScript interop
#[wasm_bindgen]
pub struct WebObject {
    flag: bool,
    data: String,
}

#[wasm_bindgen]
impl WebObject {
    #[wasm_bindgen(constructor)]
    pub fn new(flag: bool, data: String) -> WebObject {
        WebObject { flag, data }
    }

    #[wasm_bindgen(getter)]
    pub fn flag(&self) -> bool {
        self.flag
    }

    #[wasm_bindgen(setter)]
    pub fn set_flag(&mut self, flag: bool) {
        self.flag = flag;
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> String {
        self.data.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_data(&mut self, data: String) {
        self.data = data;
    }
}

fn from_web_object(object: WebObject) -> Object {
    Object {
        flag: object.flag,
        data: object.data,
    }
}
fn to_web_object(object: Object) -> WebObject {
    WebObject {
        flag: object.flag,
        data: object.data,
    }
}

#[wasm_bindgen]
pub fn web_update(object: WebObject) -> WebObject {
    let object = from_web_object(object);
    let object = update(object);
    to_web_object(object)
}
