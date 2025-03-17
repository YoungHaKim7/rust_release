# ```cargo rustc```

```
$ cargo rustc -- --emit asm=asssembly.s
  
```

# ```cargo rustc```

```

$ cargo rustc -- --print=cfg
   Compiling cargo_rustc_test v0.1.0 (/home/gyoung/my_project/Rust_lang/rust_release/07_cargo_rustc/cargo_rustc_test)
debug_assertions
panic="unwind"
target_arch="x86_64"
target_endian="little"
target_env="gnu"
target_family="unix"
target_feature="fxsr"
target_feature="sse"
target_feature="sse2"
target_has_atomic="16"
target_has_atomic="32"
target_has_atomic="64"
target_has_atomic="8"
target_has_atomic="ptr"
target_os="linux"
target_pointer_width="64"
target_vendor="unknown"
unix
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s

```
