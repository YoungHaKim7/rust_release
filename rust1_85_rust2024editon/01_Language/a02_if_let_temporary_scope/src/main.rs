use std::{sync::RwLock, thread};
// Starting with 2024

fn f(value: &RwLock<Option<bool>>) {
    if let Some(x) = *value.read().unwrap() {
        println!("value is {x}");
    }
    // <--- Read lock is dropped here in 2024
    else {
        let mut s = value.write().unwrap();
        if s.is_none() {
            *s = Some(true);
        }
    }
}

fn main() {
    thread::spawn(move || {
        let my_val: &RwLock<Option<bool>> = &RwLock::new(Some(false));
        f(my_val);
    })
    .join()
    .unwrap();
}
