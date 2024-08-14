# Rust 1.80.0 is now available! :crab::rainbow:

This release brings you LazyCell, LazyLock, checked cfg names & values, exclusive ranges in patterns, IntoIterator for Box<[T]>, Option::take_if, split_at_checked, and more!

Check out the announcement and release notes:
- https://blog.rust-lang.org/2024/07/25/Rust-1.80.0.html
  - https://releases.rs/ 
- https://github.com/rust-lang/rust/blob/1.80.0/RELEASES.md

<hr>

# Rust 1.80.0: Top 10 Most Interesting Things & Blog Highlights | Nathan Stocks
- https://youtu.be/iwV-T5yHJQw?si=8oQ6SI9gAo2EqhBF

# Option(method. take_if)

- Takes the value out of the option, but only if the predicate evaluates to true on a mutable reference to the value.
  - 값에 대한 변경 가능한 참조에서 술어가 참으로 평가되는 경우에만 옵션에서 값을 가져옵니다.
    - In other words, replaces self with None if the predicate returns true. This method operates similar to Option::take but conditional.
      - 즉, 술어가 true를 반환하면 self를 None으로 대체합니다. 이 방법은 Option::take와 유사하게 작동하지만 조건부입니다.

- https://doc.rust-lang.org/std/option/enum.Option.html#method.take_if

<hr>

# Support C23's variadics without a named parameter(ISO/IEC 9899:2024)
- 명명된 매개변수 없이 C23의 변형을 지원합니다(ISO/IEC 9899:2024)
  - https://github.com/rust-lang/rust/pull/124048/
    -  https://www.open-std.org/jtc1/sc22/wg14/www/docs/n2975.pdf

- C23 is the informal name for ISO/IEC 9899:2024
  - https://en.wikipedia.org/wiki/C23_(C_standard_revision)
  - https://www.open-std.org/jtc1/sc22/wg14/www/docs/n3220.pdf

<hr>

# libc 종속성 제거 - 윈도우쪽 업데이트-이건 속이 시원하다 ㅋ
- Internal Changes
- These changes do not affect any public interfaces of Rust, but they represent significant improvements to the performance or internals of rustc and related tools.
  - Misc improvements to size of generated html by rustdoc e.g. [#124738](https://github.com/rust-lang/rust/pull/124738/) and [#123734](https://github.com/rust-lang/rust/pull/123734/)
  - [MSVC targets no longer depend on libc](https://github.com/rust-lang/rust/pull/124050/)
