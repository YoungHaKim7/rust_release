# Rust 1.74.0 | Let's Get Rusty
https://youtu.be/E0rPugR8s30?si=PqvJ4Lf-F0lcs-nw

- Stablize ```impl_trait_projections``` #115659
  - https://github.com/rust-lang/rust/pull/115659

<hr>

# Rust 컴파일 속도 올리기 How to use it(Nightly 231109 기준)

https://doc.rust-lang.org/beta/cargo/reference/registry-authentication.html

https://blog.rust-lang.org/2023/11/09/parallel-rustc.html

The nightly compiler is now shipping with the parallel front-end enabled. However, by default it runs in single-threaded mode and won't reduce compile times.

Keen users can opt into multi-threaded mode with the -Z threads option. For example:

```bash
$ RUSTFLAGS="-Z threads=8" cargo build --release
```

Alternatively, to opt in from a config.toml file (for one or more projects), add these lines:

- 경로 맞는지는 테스트 해봐야함
- ```~/.cargo/config.toml```

```toml
[build]
rustflags = ["-Z", "threads=8"]

```

<hr>

# Rust 1.74.0 공식 문서

- https://github.com/rust-lang/rust/blob/1.74.0/RELEASES.md

- https://blog.rust-lang.org/2023/11/16/Rust-1.74.0.html

# Rust’s most wanted feature is coming!

https://youtu.be/EA19neOCeCo


# Stabilizing async fn in traits in 2023

https://blog.rust-lang.org/inside-rust/2023/05/03/stabilizing-async-fn-in-trait.html
