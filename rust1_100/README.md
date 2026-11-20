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

# Rust Trends Issue 87 - Rust 1.100 Ships the Never Type
- Edition 87 · 2026-11-14
  - https://rust-trends.com/newsletter/rust-1-100-ships-the-never-type/

# **[Rust 1.100.0 - never 타입과 사용자 정의 메모리 할당자 안정화](<https://news.hada.io/topic?id=37070&utm_source=discord&utm_medium=bot&utm_campaign=5116>)**
- **never 타입 `!` 안정화**  
  - `Result<T, !>`처럼 실패할 수 없는 연산을 표현할 수 있음  
  - 기존 `Infallible`은 `!`의 타입 별칭이 됨  
  - 2015, 2018, 2021 에디션도 2024 에디션과 같이 never 타입 폴백을 `()`에서 `!`로 바꿈. 일부 코드의 타입 추론에 영향을 줄 수 있음  
- **사용자 정의 메모리 할당자 API 안정화**  
  - `Allocator` 트레이트로 사용자 정의 할당자를 구현할 수 있음  
  - `Vec::new_in`, `Box::new_in`으로 객체를 만들 때 사용할 할당자를 지정할 수 있음  
- **`DropGuard`** 는 스코프…

# 코드 연습

```rust
use std::process;

fn handle_error(message: &str) -> ! {
    eprintln!("ERROR: {}", message);
    process::exit(1);
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn execute<F>(callback: F) -> i32
where
    F: FnOnce() -> Result<i32, String>,
{
    match callback() {
        Ok(value) => {
            println!("Success: {}", value);
            value
        }

        Err(error) => {
            handle_error(&error);
        }
    }
}

fn main() {
    let result = execute(|| divide(100, 10));

    println!("Result: {}", result);
}
```

The error occurs because `!` represents an uninhabited type: **there is no value of type&#x20;**`!`**&#x20;that you can construct.**

Therefore, this is invalid:

```
Err()
```

`Err` requires a value of type `!`, but no such value exists.

## **1. Correct implementation**

If your function cannot fail, return `Ok(42)`:

```
fn successful_operation() -> Result<i32, !> {
    Ok(42)
}

fn main() {
    let result = successful_operation();

    match result {
        Ok(value) => println!("Value: {}", value),
    }
}
```

Output:

```
Value: 42
```

Notice that you don't need an `Err` match arm because `!` has no possible values.

## **2. What if you want an error?**

Use a real error type:

```
fn operation(success: bool) -> Result<i32, String> {
    if success {
        Ok(42)
    } else {
        Err("Operation failed".to_string())
    }
}

fn main() {
    match operation(false) {
        Ok(value) => println!("Value: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}
```

## **3. Understanding the difference**

| Type                  | Meaning                                      |
| --------------------- | -------------------------------------------- |
| `Result<i32, !>`      | Can return `i32`, but cannot return an error |
| `Result<i32, String>` | Can return `i32` or a `String` error         |
| `fn() -> !`           | Never returns normally                       |

**Key takeaway:** You cannot construct `Err(!)`. The never type is useful precisely because it represents an impossible value, not an empty error object.
