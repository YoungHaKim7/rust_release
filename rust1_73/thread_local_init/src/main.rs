use std::cell::Cell;

thread_local! {
    static THINGS: Cell<Vec<i32>> = Cell::new(Vec::new());
}

fn f() {
    // before:
    THINGS.with(|i| i.set(vec![1, 2, 3]));
    println!("{THINGS:?}");
    // now:
    THINGS.set(vec![1, 2, 3]);

    // ...

    // before:
    let v = THINGS.with(|i| i.take());
    // now:
    let v: Vec<i32> = THINGS.take();
    println!("{v:?}");
}

fn main() {
    f();
}
