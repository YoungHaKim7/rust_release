# Tracking issue for -Z features=dev_dep #7916
- https://github.com/rust-lang/cargo/issues/7916
- Implementation: #7820
Documentation: https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#features

Summary
`-Z features=dev_dep` causes the feature resolver to prevent features enabled on dev dependencies from being enabled for normal dependencies. For example:

```toml
[dependencies]
serde = {version = "1.0", default-features = false}

[dev-dependencies]
serde = {version = "1.0", features = ["std"]}
```

- In this example, the library will normally link against serde without the std feature. However, when built as a test or example, it will include the std feature.

- This mode is ignored if you are building any test, bench, or example. That is, dev dependency features will still be unified if you run commands like cargo test or cargo build --all-targets.

- Unresolved issues

<hr />
