# Result

```bash

$ cargo r

warning: unexpected `cfg` condition value: `crayon`

 --> src/main.rs:4:11
  |
4 |     #[cfg(feature = "crayon")]
  |           ^^^^^^^^^^^^^^^^^^ help: remove the condition
  |
  = note: no expected values for `feature`
  = help: consider adding `crayon` as a feature in `Cargo.toml`
  = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration
  = note: `#[warn(unexpected_cfgs)]` on by default

warning: `a02_checked_cfg` (bin "a02_checked_cfg") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/a02_checked_cfg`
Hello, world!
```

