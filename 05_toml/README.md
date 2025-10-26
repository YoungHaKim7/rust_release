# toml v0.9
- https://epage.github.io/blog/2025/07/toml-09/
  - https://github.com/toml-rs/toml/blob/main/crates/toml/CHANGELOG.md#090---2025-07-08

# Rust TOML Parser
- https://github.com/toml-rs/toml
- doc
  - https://docs.rs/toml/latest/toml/

# `Cargo.toml` 관련 변경 내용

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
