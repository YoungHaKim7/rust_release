# Result

```
$ cargo run
   Compiling panic_hook04_abort v0.1.0 (/Users/globalyoung/Documents/test/test/rust/rust_release/rust1_68/panic_hook04_abort)
warning: unreachable statement
 --> src/main.rs:8:9
  |
7 |         panic!();
  |         -------- any code following this expression is unreachable
8 |         println!("There was a problem");
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unreachable statement
  |
  = note: `#[warn(unreachable_code)]` on by default
  = note: this warning originates in the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: `panic_hook04_abort` (bin "panic_hook04_abort") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/panic_hook04_abort`

thread panicked while processing panic. aborting.

[1]    24078 abort      cargo run
```
