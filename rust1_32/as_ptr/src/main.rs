fn main() {
    let s = "Hello";
    let ptr = s.as_ptr();
    println!("s.as_ptr : {ptr:?}");
}
