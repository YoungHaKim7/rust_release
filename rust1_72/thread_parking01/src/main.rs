use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

fn main() {
    println!("Hello, world!");
    let flag = Arc::new(AtomicBool::new(false));

    let t2 = thread::spawn(move || {
        while !flag.load(Ordering::Acquire) {
            thread::park();
        }
    });

    flag.store(true, Ordering::Release);
    t2.thread().unpark();

    // t1: thread.unpark()
    // t1: flag.store(true)
    // t2: flag.load() == false

    // t2 now parks, is immediately unblocked but never
    // acquires the flag, and thus spins forever
}
