use std::ffi::CStr;

fn main() {
    let cstr = CStr::from_bytes_with_nul(b"foo\0").expect("CStr::from_bytes_with_nul failed");
    assert_eq!(cstr.to_str(), Ok("foo"));
    println!("{cstr:?}");
}
