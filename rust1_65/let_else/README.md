# 프로그래밍 언어 러스트를 배웁시다! 182 let else

https://youtu.be/mR9Knqo8XMg

<br>

<hr>

# Primitive Type never

https://doc.rust-lang.org/stable/std/primitive.never.html

<br>

# let else 새로 나온 Error

`else` clause of `let...else` does not diverge
expected type `!`
found unit type `()`
try adding a diverging expression, such as `return` or `panic!(..)`
...or use `match` instead of `let...else`rustcClick for full compiler diagnostic

<br>

<hr>

# Result :

```
9
```

<br>

# 숫자가 아니면 panic!

```
$ cargo run

   Compiling let_else v0.1.0 (/Users/globalyoung/Documents/test/test/rust_release/rust1_65/let_else)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/let_else`
thread 'main' panicked at 'Dear God', src/main.rs:5:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


```
