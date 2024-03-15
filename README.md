<p align="center">
    <img width=70px src="https://user-images.githubusercontent.com/67513038/213436632-820a1675-98d9-4626-979d-be63c60cdcb7.png" />
</p>

# link

- [최근-러스트-업데이트-러스트Weekly & etc뉴스](#최근-러스트-업데이트-weeklyetc뉴스)

<hr>

# 공부할게 많은 버젼별 Rust-clippy

https://rust-lang.github.io/rust-clippy/

# rustc --version --verbose

- test한 환경을 남에게 보여줄때 좋다.

```
$ rustc --version --verbose
rustc 1.70.0 (90c541806 2023-05-31)
binary: rustc
commit-hash: 90c541806f23a127002de5b4038be731ba1458ca
commit-date: 2023-05-31
host: aarch64-apple-darwin
release: 1.70.0
LLVM version: 16.0.2
```

<hr>

# Rust 러스트 업데이트 내용 미리 보기

- spoiler book ㅋㅋ 

- Spoiler: there are loads.↩︎

Luca Palmieri. Zero To Production In Rust (Kindle Locations 595-596). Kindle Edition. 

https://doc.rust-lang.org/nightly/unstable-book/the-unstable-book.html

# Debug information is not included in build scripts by default anymore(속도 올리려고 1.69에서 디버그 정보 빠짐 다시 넣는 방법)

- If you want to debug a build script, you can add this snippet to your ```Cargo.toml``` to emit debug information again:
- Cargo.toml https://blog.rust-lang.org/2023/04/20/Rust-1.69.0.html

```
[profile.dev.build-override]
debug = true
[profile.release.build-override]
debug = true
```
<br>

<hr>

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

<br>

<hr>

# Rust Relese 노트 미리 알아보기

https://releases.rs/docs/

<hr>

# 최근 러스트 업데이트 weekly&etc뉴스

- 러스트 Weekly
  - https://this-week-in-rust.org/

- crates.io: Download changes
  - Mar. 11, 2024 · Tobias Bieniek on behalf of the crates.io team
    - https://blog.rust-lang.org/2024/03/11/crates-io-download-changes.html

<hr>

<br>

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

https://github.com/rust-lang/rust/blob/master/RELEASES.md

<br>

https://github.com/rust-lang/rust/blob/1.64.0/RELEASES.md

<br>

# Rust 공식 Blog

https://blog.rust-lang.org/

<hr>

# Rust Code 배울게 많다.
https://github.com/oli-obk

<hr>

<br>

# Asynchronous Programming in Rust \_1.63 부터 이해해야할 Async

https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html

# cargo add 활용법

```
$ cargo add serde -F serde/derive && cargo add tokio -F tokio/full


// short ver.
$ cargo add serde -F derive && cargo add tokio -F full
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

- 다른 cargo add

```
$ cargo add serde tokio -F serde/derive -F tokio/full

    Updating crates.io index
      Adding serde v1.0.159 to dependencies.
             Features:
             + derive
             + serde_derive
             + std
             - alloc
             - rc
             - unstable
      Adding tokio v1.27.0 to dependencies.
             Features:
             + bytes
             + fs
             + full
             + io-std
             + io-util
             + libc
             + macros
             + net
             + num_cpus
             + parking_lot
             + process
```

# ```cargo clean``` 활용법(target을 그냥 지우기 보단 cargo clean을 활용하자)

```
$ cargo clean

     Removed 347 files, 102.8MiB total

```

- target폴더 같은거 찾아서 지워준다 굿 👍Cleans dependencies and build artifacts from your projects. 
  - https://github.com/tbillington/kondo


<br>

<hr>

# 중국 사람이 정리한 Rust eBook 번역해서 볼만함

https://github.com/sunface/rust-course

<br>

- 이거 eBook 장난아니게 버젼별 정리까지 최고

https://course.rs/about-book.html

<br>

- Rust by practice

영문 버전 중국어 보다 보기 편하다. ^^;

https://practice.rs/why-exercise.html

https://github.com/sunface/rust-by-practice

<br>

- Cook Book

https://rusty.rs/about.html

<br>

# Rustup show

```
PS D:\rust_toolchain_toml> rustup show
Default host: x86_64-pc-windows-msvc
rustup home:  C:\Users\user\.rustup

installed toolchains
--------------------

stable-x86_64-pc-windows-msvc (default)
nightly-2023-02-21-x86_64-pc-windows-msvc
1.65.0-x86_64-pc-windows-msvc
1.68.0-x86_64-pc-windows-msvc

active toolchain
----------------

1.68.0-x86_64-pc-windows-msvc (overridden by 'D:\rust-toolchain.toml')
rustc 1.68.0 (2c8cc3432 2023-03-06)
```

# rustup toolchain remove nightly-2023-02-21 1.65.0(필요없는거 지우기)

```
PS D:\rust_toolchain_toml> rustup toolchain remove nightly-2023-02-21 1.65.0

info: uninstalling toolchain 'nightly-2023-02-21-x86_64-pc-windows-msvc'
info: toolchain 'nightly-2023-02-21-x86_64-pc-windows-msvc' uninstalled
info: uninstalling toolchain '1.65.0-x86_64-pc-windows-msvc'
info: toolchain '1.65.0-x86_64-pc-windows-msvc' uninstalled

PS D:\rust_toolchain_toml> rustup show
Default host: x86_64-pc-windows-msvc
rustup home:  C:\Users\user\.rustup

installed toolchains
--------------------

stable-x86_64-pc-windows-msvc (default)
1.68.0-x86_64-pc-windows-msvc

active toolchain
----------------

1.68.0-x86_64-pc-windows-msvc (overridden by 'D:\rust-toolchain.toml')
rustc 1.68.0 (2c8cc3432 2023-03-06)


```

- rustup show
```
rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/gy/.rustup

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu (default)
nightly-x86_64-unknown-linux-gnu

installed targets for active toolchain
--------------------------------------

wasm32-unknown-unknown
x86_64-apple-ios
x86_64-unknown-linux-gnu

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.75.0 (82e1608df 2023-12-21)


```

- rustup target remove

```
rustup target remove x86_64-apple-ios   
info: removing component 'rust-std' for 'x86_64-apple-ios'

```

# crates.io관련 뉴스
- crates.io: API status code changes(240206)
  - https://blog.rust-lang.org/2024/02/06/crates-io-status-codes.html
