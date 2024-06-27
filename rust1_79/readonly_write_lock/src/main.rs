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
            println!(
                "get_value : 30이 나오나 확인 :   {}",
                get_value(&lock_clone)
            )
        }
    })
    .join()
    .unwrap();

    let lock2 = RwLock::new(0);

    let lock2_clone = Arc::new(lock2);

    // Perform operations on the RwLock
    std::thread::spawn(move || {
        increment(&lock2_clone, 200);
        {
            println!(
                "get_value : 200이 나오나 확인 :   {}",
                get_value(&lock2_clone)
            )
        }
    })
    .join()
    .unwrap();

    let lock3 = RwLock::new(1000);
    let lock3_clone = Arc::new(lock3);

    std::thread::spawn(move || {
        reset_to_zero(&lock3_clone);
        {
            println!(
                "get_value(reset_to_zero) : 0이 나오나 확인 :   {}",
                get_value(&lock3_clone)
            )
        }
    })
    .join()
    .unwrap();

    let lock4 = RwLock::new(0);
    // Perform the assertion
    assert_is_zero(&lock4);
    println!("assert_is_zero 문제 없음 : 0 으로 초기화 잘됨");
}
