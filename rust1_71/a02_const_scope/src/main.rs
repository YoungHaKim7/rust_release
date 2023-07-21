use std::cell::Cell;

thread_local! {
    pub static FOO: Cell<u32> = const { Cell::new(1)};
}

fn main() {
    println!("Hello, world!");
}
