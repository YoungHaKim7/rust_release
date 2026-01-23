use std::cell::Cell;

thread_local! {
    static X: Cell<i32> = const { Cell::new(1) };
}
fn main() {
    assert_eq!(X.get(), 1);
}
