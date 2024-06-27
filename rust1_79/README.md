# Announcing Rust 1.79.0
- https://blog.rust-lang.org/2024/06/13/Rust-1.79.0.html
  - https://releases.rs/docs/1.79.0/

# Rust 1.79.0: Top 10 Most Interesting Things | Nathan Stocks
- https://youtu.be/u5WD5Ta09vs?si=W2Y5vsTsmKaR2m2R


<hr>

# readonly_write_lock (rust1.79)
- https://rust-lang.github.io/rust-clippy/master/

- Looks for calls to RwLock::write where the lock is only used for reading.
Why is this bad?

- The write portion of RwLock is exclusive, meaning that no other thread can access the lock while this writer is active.
- Example

```rs
use std::sync::RwLock;
fn assert_is_zero(lock: &RwLock<i32>) {
    let num = lock.write().unwrap();
    assert_eq!(*num, 0);
}
```

- Use instead:

```rs
use std::sync::RwLock;
fn assert_is_zero(lock: &RwLock<i32>) {
    let num = lock.read().unwrap();
    assert_eq!(*num, 0);
}
```
