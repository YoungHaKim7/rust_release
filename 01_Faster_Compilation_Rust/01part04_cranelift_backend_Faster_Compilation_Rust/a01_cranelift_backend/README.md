# Cranelift based backend for rustc (LLVM이 느려서 대체하려고 나온 프로젝트)

- https://github.com/rust-lang/rustc_codegen_cranelift

# Download using Rustup
- The Cranelift codegen backend is distributed in nightly builds on Linux and x86_64 macOS. If you want to install it using Rustup, you can do that by running:# Result

```bash
$ rustup component add rustc-codegen-cranelift-preview --toolchain nightly
```

- Once it is installed, you can enable it with one of the following approaches:

```bash
CARGO_PROFILE_DEV_CODEGEN_BACKEND=cranelift cargo +nightly build -Zcodegen-backend
```

- Add the following to `.cargo/config.toml`: 

```bash
[unstable]
codegen-backend = true

[profile.dev]
codegen-backend = "cranelift"
```

- Add the following to Cargo.toml: 

```toml
# This line needs to come before anything else in Cargo.toml
cargo-features = ["codegen-backend"]

[profile.dev]
codegen-backend = "cranelift"

```
