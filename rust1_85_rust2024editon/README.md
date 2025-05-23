# compiler explorer 에서 러스트 nightly 적용하는 방법
- https://godbolt.org/z/v8EfrqGfG

- rustc nightly선택 후 뒤에 옵션에 추가

```bash
-Z unstable-options --edition=2024
```

<hr />

# Rust 2027로 밀릴 가능성 높은 프로젝트
- Tracking Issue for RFC 3550: New range types #123741(https://github.com/rust-lang/rust/issues/123741)

<hr />

# (241130기준)현재는 nightly로 사용가능한듯 찾아보자
- [베타에서 테스트 해보자https://blog.rust-lang.org/2025/01/22/rust-2024-beta.html](https://blog.rust-lang.org/2025/01/22/rust-2024-beta.html)
- https://www.reddit.com/r/rust/comments/1gxyhkx/the_2024_edition_was_just_stabilized/?rdt=46903
- https://doc.rust-lang.org/nightly/edition-guide/rust-2024/index.html

- https://blog.rust-lang.org/2024/11/27/Rust-2024-public-testing.html

- https://github.com/rust-lang/rust/pull/133349
  - https://github.com/ehuss/rust/commit/31c922263922cc3e74f9c732677946521299ea99

- https://releases.rs/docs/1.85.0/

<hr />

# Rust2024 Edition (2025년 beta버젼에 출시 예정250109 beta version에서 사용가능)
  - [241127뉴스_rust2024 beta version에 250109 적용예정.](https://blog.rust-lang.org/2024/11/27/Rust-2024-public-testing.html)
    - Stable버젼은  Rust 1.85 on 2025-02-20
      - Rust 2024 will enter the beta channel on 2025-01-09, and will be released to stable Rust with Rust 1.85 on 2025-02-20.
  - (231215) https://blog.rust-lang.org/2023/12/15/2024-Edition-CFP.html
  - (240812) [(러스트 Blog)Rust Project goals for 2024](https://blog.rust-lang.org/2024/08/12/Project-goals.html)
    - Rust 2024 Edition 
      - https://rust-lang.github.io/rust-project-goals/2024h2/Rust-2024-Edition.html
    - Async
      - https://rust-lang.github.io/rust-project-goals/2024h2/async.html
    - Rust for Linux
      - https://rust-lang.github.io/rust-project-goals/2024h2/rfl_stable.html
        - Rust(2024) 23가지 목표
          - https://rust-lang.github.io/rust-project-goals/2024h2/index.html#project-goals
          - github
            - https://github.com/rust-lang/rust-project-goals

# The gen auto-trait problem (— 2025-01-13)
- https://blog.yoshuawuyts.com/gen-auto-trait-problem/

# This Development-cycle in Cargo: 1.85
Jan. 17, 2025 · Ed Page on behalf of The Cargo Team
- https://blog.rust-lang.org/inside-rust/2025/01/17/this-development-cycle-in-cargo-1.85.html
