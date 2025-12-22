# Announcing Rust 1.92.0
- Dec. 11, 2025 · The Rust Release Team 
  - https://blog.rust-lang.org/2025/12/11/Rust-1.92.0/

# Infallible

- https://doc.rust-lang.org/stable/std/convert/enum.Infallible.html

- The error type for errors that can never happen.

Since this enum has no variant, a value of this type can never actually exist. This can be useful for generic APIs that use Result and parameterize the error type, to indicate that the result is always Ok.

- For example, the TryFrom trait (conversion that returns a Result) has a blanket implementation for all types where a reverse Into implementation exists.
  - 절대 발생할 수 없는 오류 유형.
  - 이 열거형에는 변형이 없으므로 이 유형의 값은 실제로 존재할 수 없습니다. 이는 결과를 사용하고 오류 유형을 매개변수화하는 일반 API에 유용할 수 있으며, 결과가 항상 정상임을 나타냅니다.

  - 예를 들어, TryFrom 특성(결과를 반환하는 변환)은 역인투 구현이 존재하는 모든 유형에 대해 포괄적으로 구현됩니다.

```rs

impl<T, U> TryFrom<U> for T where U: Into<T> {
    type Error = Infallible;

    fn try_from(value: U) -> Result<Self, Infallible> {
        Ok(U::into(value))  // Never returns `Err`
    }
}
```

Future compatibility

This enum has the same role as the ! “never” type, which is unstable in this version of Rust. When ! is stabilized, we plan to make Infallible a type alias to it:
ⓘ

```rs
pub type Infallible = !;

// … and eventually deprecate Infallible.

// However there is one case where ! syntax can be used before ! is stabilized as a full-fledged type: in the position of a function’s return type. Specifically, it is possible to have implementations for two different function pointer types:

trait MyTrait {}
impl MyTrait for fn() -> ! {}
impl MyTrait for fn() -> std::convert::Infallible {}
```

With Infallible being an enum, this code is valid. However when Infallible becomes an alias for the never type, the two impls will start to overlap and therefore will be disallowed by the language’s trait coherence rules.

