# Result

```
$ cargo run


   Compiling async_fn_must_use v0.1.0 (/Users/globalyoung/Documents/test/test/test/rust_release/rust1_67/async_fn_must_use)

warning: unused output of future returned by `get_int` that must be used
  --> src/main.rs:12:5
   |
12 |     get_int().await;
   |     ^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default

warning: `async_fn_must_use` (bin "async_fn_must_use") generated 1 warning

    Finished dev [unoptimized + debuginfo] target(s) in 5.22s
     Running `target/debug/async_fn_must_use`

```
