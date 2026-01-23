use std::cell::{Cell, RefCell};

thread_local! {
    pub static FOO: Cell<u32> = const { Cell::new(1) };

    static BAR: RefCell<Vec<f32>> = RefCell::new(vec![1.0, 2.0]);
}

fn main() {
    assert_eq!(FOO.get(), 1);

    BAR.with_borrow(|v| assert_eq!(v[1], 2.0));
}
