// https://docs.rs/sp-std/2.0.0/sp_std/sync/atomic/struct.AtomicBool.html#method.compare_exchange
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    let some_bool = AtomicBool::new(true);

    assert_eq!(
        some_bool.compare_exchange(true, false, Ordering::Acquire, Ordering::Relaxed),
        Ok(true)
    );
    assert_eq!(some_bool.load(Ordering::Relaxed), false);

    assert_eq!(
        some_bool.compare_exchange(true, true, Ordering::SeqCst, Ordering::Acquire),
        Err(false)
    );
    assert_eq!(some_bool.load(Ordering::Relaxed), false);
}

// use std::{cmp::Ordering, ptr, sync::atomic::AtomicPtr};

// fn main() {
//     static PTR: AtomicPtr<Data> = AtomicPtr::new(ptr::null_mut());
//     let mut p = PTR.load(Ordering::Acquire);

//     if p.is_null() {
//         p = Box::into_raw(Box::new(f()));
//         if let Err(e) =
//             PTR.compare_exchange(ptr::null_mut(), p, Ordering::Release, Ordering::Acquire)
//         {
//             drop(unsafe { Box::from_raw(p) });
//             p = e;
//         }
//     }
//     println!("Hello, world!");
// }
