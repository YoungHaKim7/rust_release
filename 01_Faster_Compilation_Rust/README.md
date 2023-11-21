# 멀티 쓰레드 숫자가 8이상은 되어야 효과가 있는듯 싶다.

- 여러가지 환경에서 test중..
- 글을 잘 읽어보니 크기와 최적화에 따라서 효과 편찬가 큰듯 싶다.


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
