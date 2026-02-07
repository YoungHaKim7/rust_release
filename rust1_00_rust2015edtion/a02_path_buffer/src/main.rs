use std::path::PathBuf;
fn main() {
    let mut path = PathBuf::with_capacity(10);
    let capacity = path.capacity();

    // This push is done without reallocating
    path.push(r"C:\");

    assert_eq!(capacity, path.capacity());

    println!("path : {path:?}");
}
