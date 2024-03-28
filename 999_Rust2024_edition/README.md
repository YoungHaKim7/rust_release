# Rust Version
- 🚀 Cutting-edge Features with 'Editions'
- Rust introduces significant changes or new features through ‘Editions’, which are released every 3 years.
  - Rust Edition 2015
    - https://doc.rust-lang.org/edition-guide/rust-2015/index.html
  - Rust Edition 2018
    - (181206) https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
      - https://doc.rust-lang.org/edition-guide/rust-2018/index.html
  - Rust Edition 2021
    - https://doc.rust-lang.org/edition-guide/rust-2021/index.html
    - (211021) https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html
      - https://news.hada.io/topic?id=5240
  - Rust2024 Edition (2024년 하반기 출시 예정)
      - (231215) https://blog.rust-lang.org/2023/12/15/2024-Edition-CFP.html

<hr>


# Rust 2024맛보기

- https://nikomatsakis.github.io/rustnation-24/

  - https://www.youtube.com/live/RQSZ3wLsjNM?si=GZ9UoAfQ7b3s-JOg

# 올해 하반기에 나올 Rust 2024에 대해
- Rust 2024 style edition

- This style guide describes the Rust 2024 style edition. The Rust 2024 style edition is currently nightly-only and may change before the release of Rust 2024.

- For a full history of changes in the Rust 2024 style edition, see the git history of the style guide. Notable changes in the Rust 2024 style edition include:

- 114764 As the last member of a delimited expression, delimited expressions are generally combinable, regardless of the number of members. Previously only applied with exactly one member (except for closures with explicit blocks).
Miscellaneous rustfmt bugfixes.
Use version-sort (sort x8, x16, x32, x64, x128 in that order).
Change "ASCIIbetical" sort to Unicode-aware "non-lowercase before lowercase".

- https://github.com/rust-lang/rust/pull/114764

# Hack without fear

- We had a lot of ideas to improve performance, but we were hesitant to introduce them into critical systems given the risk of subtle bugs.
  - 성능 향상을 위한 아이디어는 많았지만, 미묘한 버그의 위험을 고려하여 중요한 시스템에 도입하는 것을 주저했습니다.
- With Rust, we achieved up to double-digit percentage performance improvements. Rust’s type system provides a structure which we used to safely optimize our code.
  - Rust를 통해 최대 두 자릿수의 성능 향상을 달성했습니다. Rust의 유형 시스템은 코드를 안전하게 최적화하는 데 사용했던 구조를 제공합니다.

# Image(Rust crates.io)

https://crates.io/crates/image


