# Announcing Rust 1.98.0| Aug. 20, 2026 · The Rust Release Team 
- https://blog.rust-lang.org/2026/08/20/Rust-1.98.0/


# 이제 UB나지 않는다.

```rust
use std::mem::ManuallyDrop;

fn main() {
    let mut x = ManuallyDrop::new(Box::new(1));
    unsafe { ManuallyDrop::drop(&mut x) };
    let x = x; // UB!
    println!("Manuall Drop Working");
}
```
