use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

fn main() {
    println!("Hello, world!");

    let a = Arc::new(AtomicBool::new(false));
    let b = Arc::new(AtomicBool::new(false));

    let t2 = thread::spawn(move || {
        while !a.load(Ordering::Acquire) || !b.load(Ordering::Acquire) {
            thread::park();
        }
    });

    thread::spawn(move || {
        a.store(true, Ordering::Release);
        t2.thread().unpark();
    });

    b.store(true, Ordering::Release);
    t2.thread().unpark();

    // t1: a.store(true)
    // t1: t2.unpark()
    // t3: b.store(true)
    // t3: t2.unpark()

    // t2 now parks, is immediately unblocked but never
    // acquires the store of `a`, only the store of `b` which
    // was released by the most recent unpark, and thus spins forever
}
