- [echo로 빠르게 넣기 rust-toolchain.toml & .cargo](#echo-%EB%A1%9C-%EB%B9%A0%EB%A5%B4%EA%B2%8C-%EB%84%A3%EA%B8%B0)

<hr>

# 디버그 정보 빼서 컴파일 빠르게 만들기

- Cargo.toml
  - https://kobzol.github.io/rust/rustc/2025/05/20/disable-debuginfo-to-improve-rust-compile-times.html
  
```toml
[profile.dev]
debug = false

```

# 멀티 쓰레드 숫자가 8이상은 되어야 효과가 있는듯 싶다.

- 여러가지 환경에서 test중..
- 글을 잘 읽어보니 크기와 최적화에 따라서 효과 편찬가 큰듯 싶다.

- Nitghtly에서 하는거라 여러 보안점을 추가한 후에 추후에 정식 버젼으로 들어올듯 싶다.
  - https://github.com/rust-lang/compiler-team/issues/681
    - 비교한 결과 자료 https://perf.rust-lang.org/compare.html?start=cf34adb0dd408d7b2f27a64484df63c36c1cc514&end=9148ca2269d942a02167b2ba918ed9e25467abe8&stat=wall-time&showRawData=true&secondary=false&nonRelevant=true&incrUnchanged=false&incrPatched=false


# How to use it(Nightly 231109 기준)

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

# Available Parallelism : Cpu 갯수 체크하기

```rs
fn main() {
    println!(
        "You can use {:?} threads(available_parallelism) now.  ",
        std::thread::available_parallelism().unwrap()
    );
}
```

# SysInfo
https://docs.rs/sysinfo/latest/sysinfo/

# echo 로 빠르게 넣기
- https://github.com/YoungHaKim7/rust_vim_setting/blob/main/01_Vim_NeoVim_etc_Rust_settings/snippets_Lunar_NeoVim/rust/rust.json
- https://github.com/YoungHaKim7/rust_vim_setting/tree/main/01_Vim_NeoVim_etc_Rust_settings/snippets_Lunar_NeoVim

```bash
rm -rf .cargo rust-toolchain.toml &&
mkdir .cargo &&
echo "[toolchain]" >> rust-toolchain.toml &&
echo "channel ="\"nightly"\"" >> rust-toolchain.toml &&
echo "components = ["\"rustfmt\"", "\"rust-src"\"]" >> rust-toolchain.toml &&

echo "[build]" >> .cargo/config.toml &&
echo "rustflags = ["\"-Z\"", "\"threads=8"\"]" >> .cargo/config.toml
```
