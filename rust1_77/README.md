# Announcing Rust 1.77.0
- https://releases.rs/docs/1.77.0/
  - https://blog.rust-lang.org/2024/03/21/Rust-1.77.0.html

- Changes to `u128`/`i128` layout in 1.77 and 1.78
  - https://blog.rust-lang.org/2024/03/30/i128-layout-update.html

# With WASI 0.2 finally stable, it's an exciting time for WebAssembly development.
https://blog.rust-lang.org/2024/04/09/updates-to-rusts-wasi-targets.html

- WASM 젤 기대되는 기술 이번에 새로운 넘들 추가됨
`wasm32-wasip1`(tier 2)

`wasm32-wasip2` (tier 3)

티어 2하고 3이라 안전성은 좀 떨어지지만 나름 좋네 ㅎ

```

date    Rust Stable    Rust Beta    Rust Nightly    Notes
2024-02-08    1.76    1.77    1.78    wasm32-wasip1 available on nightly
2024-03-21    1.77    1.78    1.79    wasm32-wasip1 available on beta
2024-05-02    1.78    1.79    1.80    wasm32-wasip1 available on stable
2024-06-13    1.79    1.80    1.81    warn if wasm32-wasi is used on nightly
2024-07-25    1.80    1.81    1.82    warn if wasm32-wasi is used on beta
2024-09-05    1.81    1.82    1.83    warn if wasm32-wasi is used on stable
2024-10-17    1.82    1.83    1.84    wasm32-wasi unavailable on nightly
2024-11-28    1.83    1.84    1.85    wasm32-wasi unavailable on beta
2025-01-09    1.84    1.85    1.86    wasm32-wasi unavailable on stable
```