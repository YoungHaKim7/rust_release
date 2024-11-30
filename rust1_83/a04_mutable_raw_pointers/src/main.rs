use std::cell::UnsafeCell;

const C: i32 = {
    let c = UnsafeCell::new(41);
    unsafe { *c.get() += 1 };
    c.into_inner()
};

fn main() {
    // error[E0764]: mutable references are not allowed in the final value of constants
    const C: &mut i32 = &mut 4;
    println!("Hello, world!");
}
