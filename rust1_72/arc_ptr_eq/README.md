# Result

https://github.com/rust-lang/rust/issues/103763

```

$ cargo run
   Compiling arc_ptr_eq v0.1.0 (/Users/globalyoung/Documents/test/test/rust/rust_release/rust1_72/arc_ptr_eq)
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/arc_ptr_eq`

Found one!
[src/main.rs:23] a_data_addr = 0x000000016ee2a6a0
[src/main.rs:24] b_data_addr = 0x000000016ee2a6a0
[src/main.rs:25] a = 0x000000016ee2a6a0
[src/main.rs:26] b = 0x000000016ee2a6a0
[src/main.rs:27] a_msg = "Zst"
[src/main.rs:28] b_msg = "Msg(good gravy)"
[src/main.rs:37] (*zm.0).type_id() = TypeId {
    t: 177224103700599114721518439526924153296,
}
[src/main.rs:38] (*zm.1).type_id() = TypeId {
    t: 18653999097659511391617033891817030262,
}

```
