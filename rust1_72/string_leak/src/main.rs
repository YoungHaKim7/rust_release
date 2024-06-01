fn main() {
    let x = String::from("bucket");
    let static_ref: &'static mut str = x.leak();
    println!("static_ref : {}", static_ref);
    assert_eq!(static_ref, "bucket");
}
