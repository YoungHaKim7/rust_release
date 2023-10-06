use std::rc::Weak;

fn main() {
    let empty: Weak<i64> = Weak::new();
    assert!(empty.upgrade().is_none());
}
