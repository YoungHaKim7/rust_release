use std::{sync::RwLock, thread};

fn f(value: &RwLock<Option<bool>>) {
    match *value.read().unwrap() {
        Some(x) => {
            println!("value is {x}");
        }
        _ => {
            let mut s = value.write().unwrap();
            if s.is_none() {
                *s = Some(true);
            }
        }
    }
    // <--- Read lock is dropped here in both 2021 and 2024
}

fn main() {
    thread::spawn(move || {
        let my_val: &RwLock<Option<bool>> = &RwLock::new(Some(false));
        f(my_val);
    })
    .join()
    .unwrap();
    thread::spawn(move || {
        let my_val: &RwLock<Option<bool>> = &RwLock::new(Some(true));
        f(my_val);
    })
    .join()
    .unwrap();

    // match도 해결 안됨.
    // thread::spawn(move || {
    //     let my_val: &RwLock<Option<bool>> = &RwLock::new(None);
    //     f(my_val);
    // })
    // .join()
    // .unwrap();
}
