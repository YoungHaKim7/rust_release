# rust 1.61(```as_mut_ptr```)
- https://riptutorial.com/rust/example/24222/displaying-raw-pointers
- Create some data, a raw pointer pointing to it and a null pointer
  - https://doc.rust-lang.org/stable/std/primitive.slice.html#method.as_mut_ptr
    - rust 1.61_as_mut_ptr


# (220704) Constant fun (const fn) with Rust - Rainer Stropek - Rust Linz June 2022 | Rust Programming Language]
- https://youtu.be/Vw8BFScm0K0?si=BrHG3DZ8fyWDHAGB
  - Everybody knows constants from other programming languages. But as so often, Rust is different and knows a lot of tricks regarding constants that other languages don't. Rust's const fn recently got new features (v1.61), so we decided to do an overview session about consts in Rust. This is a beginner-friendly session for people who are starting with Rust or haven't taken a deeper look into constants and const fn in Rust.
  - 누구나 다른 프로그래밍 언어의 상수를 알고 있습니다. 하지만 Rust는 다른 언어에서는 모르는 상수에 대한 트릭을 많이 알고 있는 경우가 많습니다. Rust의 const fn은 최근에 새로운 기능(v1.61)을 도입했기 때문에 Rust의 const에 대한 개요 세션을 진행하기로 결정했습니다. Rust부터 시작하거나 Rust의 const와 const fn에 대해 자세히 살펴보지 않은 분들을 위한 초보자 친화적인 세션입니다.

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
