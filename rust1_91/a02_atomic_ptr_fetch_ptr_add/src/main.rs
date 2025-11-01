use std::sync::atomic::{AtomicPtr, Ordering};

fn main() {
    let atom = AtomicPtr::<i64>::new(core::ptr::null_mut());

    (atom.fetch_ptr_add(1, Ordering::Relaxed).addr(), 0);
    (atom.load(Ordering::Relaxed).addr(), 8);

    println!("size_of::<i64> : {} bytes", size_of::<i64>());

    println!("atom : {atom:?}");
}
