# rust 1.61(```as_mut_ptr```)
- https://riptutorial.com/rust/example/24222/displaying-raw-pointers
- Create some data, a raw pointer pointing to it and a null pointer
  - https://doc.rust-lang.org/stable/std/primitive.slice.html#method.as_mut_ptr
    - rust 1.61_as_mut_ptr


# Custom exit codes from `main`
- https://blog.rust-lang.org/2022/05/19/Rust-1.61.0/#custom-exit-codes-from-main

```rs
use std::process::{ExitCode, Termination};

#[repr(u8)]
pub enum GitBisectResult {
    Good = 0,
    Bad = 1,
    Skip = 125,
    Abort = 255,
}

impl Termination for GitBisectResult {
    fn report(self) -> ExitCode {
        // Maybe print a message here
        ExitCode::from(self as u8)
    }
}

fn main() -> GitBisectResult {
    std::panic::catch_unwind(|| {
        todo!("test the commit")
    }).unwrap_or(GitBisectResult::Abort)
}
```
