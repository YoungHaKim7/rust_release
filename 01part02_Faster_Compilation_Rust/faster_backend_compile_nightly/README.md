# Result

```bash
CARGO_PROFILE_DEV_CODEGEN_BACKEND=cranelift cargo +nightly build -Zcodegen-backend
```
  
<hr>

```bash
RUSTFLAGS="-Zcodegen-backend=cranelift" cargo +nightly build
```

<hr>

```bash

RUSTFLAGS="-Zcodegen-backend=cranelift" cargo +nightly build


   Compiling faster_backend_compile_nightly v0.1.0 (/home/y/my_project/rust_release/01part02_Faster_Compilation_Rust/faster_backend_compile_nightly)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.65s
```


```bash

CARGO_PROFILE_DEV_CODEGEN_BACKEND=cranelift cargo +nightly build -Zcodegen-backend

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s

```



