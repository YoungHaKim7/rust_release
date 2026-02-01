// https://doc.rust-lang.org/stable/std/mem/fn.replace.html
use std::mem;

fn main() {
    let mut v: Vec<i32> = vec![1, 2];

    let old_v = mem::replace(&mut v, vec![3, 4, 5]);
    assert_eq!(vec![1, 2], old_v);
    assert_eq!(vec![3, 4, 5], v);
}
