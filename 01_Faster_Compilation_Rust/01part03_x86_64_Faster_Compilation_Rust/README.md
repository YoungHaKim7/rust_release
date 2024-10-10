# Faster linking times on nightly on Linux using `rust-lld`

- https://blog.rust-lang.org/2024/05/17/enabling-rust-lld-on-linux.html

```toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-Zlinker-features=-lld"]
```
