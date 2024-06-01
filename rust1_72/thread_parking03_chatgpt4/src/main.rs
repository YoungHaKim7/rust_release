use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
};

fn main() {
    println!("Hello, world!");

    let a = Arc::new((Mutex::new(false), Condvar::new()));
    let b = Arc::new((Mutex::new(false), Condvar::new()));

    let a_clone = Arc::clone(&a);
    let b_clone = Arc::clone(&b);

    let t2 = thread::spawn(move || {
        let (a_lock, a_cvar) = &*a_clone;
        let (b_lock, b_cvar) = &*b_clone;

        let mut a_ready = a_lock.lock().unwrap();
        while !*a_ready {
            a_ready = a_cvar.wait(a_ready).unwrap();
        }

        let mut b_ready = b_lock.lock().unwrap();
        while !*b_ready {
            b_ready = b_cvar.wait(b_ready).unwrap();
        }

        println!("Both a and b are true");
    });

    let a_clone = Arc::clone(&a);
    thread::spawn(move || {
        let (lock, cvar) = &*a_clone;
        let mut a_ready = lock.lock().unwrap();
        *a_ready = true;
        cvar.notify_one();
    });

    let (b_lock, b_cvar) = &*b;
    let mut b_ready = b_lock.lock().unwrap();
    *b_ready = true;
    b_cvar.notify_one();

    t2.join().unwrap();
}
