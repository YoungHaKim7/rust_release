# Rust version Setting

```
rustup update stable
```

- Version control

```
rustup default stable

rustup default beta

rustup default nightly
```

# rust_release

rust release 제일 빨리 알려주는 트위터

[Mara Bos Twitter](https://twitter.com/m_ou_se)

https://twitter.com/m_ou_se

- 이 분의 rust vs cpp concurrency

https://blog.m-ou.se/rust-cpp-concurrency/

<br>

# What Rust is it

https://www.whatrustisit.com/

<br>

<br>
- Releases 노트 지금까지 나온거 쭉 볼 수 있다.

https://github.com/rust-lang/rust/blob/1.64.0/RELEASES.md

<br>

# Rust 공식 Blog

https://blog.rust-lang.org/

<br>

# Asynchronous Programming in Rust \_1.63 부터 이해해야할 Async

https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html

# cargo add 활용법

```
$ cargo add serde -F serde/derive && cargo add tokio -F tokio/full
```

- 알아서 이쁘게 들어간다.

Cargo.toml

```
[dependencies]
serde = { version = "1.0.140", features = ["derive"] }
tokio = { version = "1.20.1", features = ["full"] }
```

cargo add 활용법❤️귀찮은 features넣는 법 -F이게 좋네 ㅋ❤️(Cargo.toml 넣기 귀찮다. ㅋㅋ)

https://economiceco.tistory.com/m/14544
