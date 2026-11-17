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

# **[Rust 1.100.0 - never 타입과 사용자 정의 메모리 할당자 안정화](<https://news.hada.io/topic?id=37070&utm_source=discord&utm_medium=bot&utm_campaign=5116>)**
- **never 타입 `!` 안정화**  
  - `Result<T, !>`처럼 실패할 수 없는 연산을 표현할 수 있음  
  - 기존 `Infallible`은 `!`의 타입 별칭이 됨  
  - 2015, 2018, 2021 에디션도 2024 에디션과 같이 never 타입 폴백을 `()`에서 `!`로 바꿈. 일부 코드의 타입 추론에 영향을 줄 수 있음  
- **사용자 정의 메모리 할당자 API 안정화**  
  - `Allocator` 트레이트로 사용자 정의 할당자를 구현할 수 있음  
  - `Vec::new_in`, `Box::new_in`으로 객체를 만들 때 사용할 할당자를 지정할 수 있음  
- **`DropGuard`** 는 스코프…