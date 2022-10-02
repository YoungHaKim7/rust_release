use std::ffi::{CString, NulError};

fn main() {
    let null: NulError = CString::new(b"f\0oo".to_vec()).unwrap_err();
    println!("{null}");
    // result:
    //  nul byte found in provided data at position: 1
}
