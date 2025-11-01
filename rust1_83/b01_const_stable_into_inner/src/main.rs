use std::cell::UnsafeCell;
fn main() {
    let uc = UnsafeCell::new(5);

    let five = uc.into_inner();

    println! {"const cell into inner stable : {five}"};
}
