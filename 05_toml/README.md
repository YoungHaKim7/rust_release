# link

<hr />

# toml v0.9[|🔝|](#link)
- https://epage.github.io/blog/2025/07/toml-09/
  - https://github.com/toml-rs/toml/blob/main/crates/toml/CHANGELOG.md#090---2025-07-08

# Rust TOML Parser[|🔝|](#link)
- https://github.com/toml-rs/toml
- doc
  - https://docs.rs/toml/latest/toml/

# `Cargo.toml` 관련 변경 내용[|🔝|](#link)

- [(251016) If your crate requires the previous default target list, you can explicitly define it in your Cargo.toml:](https://blog.rust-lang.org/2025/10/16/docsrs-changed-default-targets/)
```toml
[package.metadata.docs.rs]
targets = [
    "x86_64-unknown-linux-gnu",
    "x86_64-apple-darwin",
    "x86_64-pc-windows-msvc",
    "i686-unknown-linux-gnu",
    "i686-pc-windows-msvc"
]
```

- (230420)Debug information is not included in build scripts by default anymore(속도 올리려고 1.69에서 디버그 정보 빠짐 다시 넣는 방법)[|🔝|](#link)
  - If you want to debug a build script, you can add this snippet to your ```Cargo.toml``` to emit debug information again:
  - Cargo.toml https://blog.rust-lang.org/2023/04/20/Rust-1.69.0.html
```toml
[profile.dev.build-override]
debug = true
[profile.release.build-override]
debug = true

# Link-Time Optimizations, or LTOs in short, is that while Rust compiles the code file by file,
[profile.release]
lto = true
```
