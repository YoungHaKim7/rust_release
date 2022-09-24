extern "C" {
    fn puts(s: *const core::ffi::c_char);
}
fn main() {
    unsafe {
        puts(b"Hello, world\n0".as_ptr().cast());
    }
}
