use std::sync::{Arc, RwLock};

// A function that asserts the value in the RwLock is zero
fn assert_is_zero(lock: &RwLock<i32>) {
    let num = lock.read().unwrap();
    assert_eq!(*num, 0);
}

// A function that increments the value in the RwLock by a given amount
fn increment(lock: &RwLock<i32>, amount: i32) {
    let mut num = lock.write().unwrap();
    *num += amount;
}

// A function that resets the value in the RwLock to zero
fn reset_to_zero(lock: &RwLock<i32>) {
    let mut num = lock.write().unwrap();
    *num = 0;
}

// A function that returns the current value in the RwLock
fn get_value(lock: &RwLock<i32>) -> i32 {
    let num = lock.read().unwrap();
    *num
}

// A function that demonstrates using RwLock with multiple threads
fn main() {
    let lock = RwLock::new(10);

    // Clone the RwLock for use in the thread
    let lock_clone = Arc::new(lock);

    std::thread::spawn(move || {
        increment(&lock_clone, 20);
        {
            println!("get_value : 15 {}", get_value(&lock_clone))
        }
    })
    .join()
    .unwrap();

    // Read the value and assert it is 15
    // println!("get_value : 15 {}", get_value(&lock_clone02));

    // Reset the value to zero
    // reset_to_zero(&lock_clone);

    // // Assert the value is zero
    // assert_is_zero(&lock);

    let lock2 = RwLock::new(0);

    // Perform operations on the RwLock
    increment(&lock2, 10);
    assert_eq!(get_value(&lock2), 10);

    reset_to_zero(&lock2);
    assert_is_zero(&lock2);
}
