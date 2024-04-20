use std::sync::OnceLock;

#[derive(Debug)]
struct A<'a>(&'a str);

impl<'a> Drop for A<'a> {
    fn drop(&mut self) {}
}

fn main() {
    let cell = OnceLock::new();
    {
        let s = String::new();
        let _ = cell.set(A(&s));
    }
}
