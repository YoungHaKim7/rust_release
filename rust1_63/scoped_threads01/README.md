# Result

```
$ cargo run
   Compiling scoped_threads01 v0.1.0 (/Users/globalyoung/Documents/test/test/rust/rust_release/rust1_63/scoped_threads01)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/scoped_threads01`
Hello from first scoped thead
Hello from the main thread
Hello from second scoped thead
 x = 4
 a = [1, 2, 3, 4]
 a.len = 4



// dgb!
$ cargo run 
Hello from the main thread
Hello from first scoped thead
[src/main.rs:8] &a = [
Hello from second scoped thead
    1,
    2,
    3,
]
[src/main.rs:13] &a = [
    1,
    2,
    3,
]
 x = 4
 a = [1, 2, 3, 4]
 a.len = 4

```
