fn main() {
    static X: [u32; 4] = [1, 2, 3, 4];

    static BYTE_SLICE: &[u8] =
        unsafe { std::slice::from_raw_parts(X.as_ptr().cast(), X.len() * 4) };

    println!("{:?}", BYTE_SLICE);
    // [1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]
}
