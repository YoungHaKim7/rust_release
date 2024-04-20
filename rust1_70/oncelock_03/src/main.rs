use std::sync::OnceLock;

#[derive(Debug)]
struct A<'a>(&'a String);

// impl<'a> Drop for A<'a> {
//     fn drop(&mut self) {}
// }

fn main() {
    let cell = OnceLock::new();
    let s = String::from("test OnceLock");
    let _ = cell.set(A(&s));
    dbg!(s);
}
