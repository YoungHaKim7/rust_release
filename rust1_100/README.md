# Announcing Rust 1.100.0
- Nov. 12, 2026 · The Rust Release Team 

```rs
fn infallible_unwrap<T>(res: Result<T, !>) -> T {
    // ^^ writing `Result<_, !>` was not allowed before
    let Ok(v) = res;
    v
}
```
- https://blog.rust-lang.org/2026/11/12/Rust-1.100.0/

# Rust 1.100: Why Did the Never Type Take 10 Years?
- @m.ebrahimi12
  - https://youtu.be/mvah0HwIRAE?si=ZPwwxWjDQA4IowT8
