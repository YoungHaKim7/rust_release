# Result

```
$ cargo run

   Compiling std_array v0.1.0 (/Users/globalyoung/Documents/test/test/test/rust_release/rust1_63/std_array)
warning: unused variable: `i`
 --> src/main.rs:2:51
  |
2 |     let my_array: [i32; 5] = std::array::from_fn(|i| 5);
  |                                                   ^ help: if this is intentional, prefix it with an underscore: `_i`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `std_array` (bin "std_array") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 2.44s
     Running `target/debug/std_array`

[5, 5, 5, 5, 5]
```
