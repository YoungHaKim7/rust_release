use std::ffi::CString;
use std::os::raw::c_char;

pub fn get_string() -> *const c_char {
    let cstring = CString::new("Hello, C-world_Rust FFI").unwrap();
    cstring.as_ptr()
}
