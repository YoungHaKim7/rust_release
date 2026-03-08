fn main() {
    println!("Hello, world! from_raw_parts rust 1.93!");

    let s = String::from("hello");

    let (ptr, len, cap) = s.into_raw_parts();

    let rebuilt = unsafe { String::from_raw_parts(ptr, len, cap) };

    assert_eq!(rebuilt, "hello");
}
