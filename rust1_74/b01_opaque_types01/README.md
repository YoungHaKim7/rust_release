# Source

- https://github.com/rust-lang/rfcs/pull/2071

# Result

https://github.com/rust-lang/rust/issues/63063

```
$ cargo r
   Compiling b01_opaque_types01 v0.1.0 (/Users/globalyoung/Documents/test/test/rust/rust_release/rust1_74/b01_opaque_types01)
warning: type alias `Foo` is never used
 --> src/main.rs:4:6
  |
4 | type Foo<T> = impl MyTrait;
  |      ^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: fields `a`, `b`, and `c` are never read
 --> src/main.rs:7:5
  |
6 | struct MyStruct<A, B, C> {
  |        -------- fields in this struct
7 |     a: A,
  |     ^
8 |     b: B,
  |     ^
9 |     c: C,
  |     ^

warning: function `foo` is never used
  --> src/main.rs:13:4
   |
13 | fn foo<T>(t: T) -> Foo<T> {
   |    ^^^

warning: `b01_opaque_types01` (bin "b01_opaque_types01") generated 3 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/b01_opaque_types01`
Hello, world!
```


- ```#![feature(type_alias_impl_trait)]``` 이거 안 쓰면 나오는 에러 

```
`impl Trait` in type aliases is unstable
 --> src/main.rs:2:15
  |
2 | type Foo<T> = impl MyTrait;
  |               ^^^^^^^^^^^^
  |
  = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
  = help: add `#![feature(type_alias_impl_trait)]` to the crate attributes to enable

For more information about this error, try `rustc --explain E0658`.
error: could not compile `b01_opaque_types01` (bin "b01_opaque_types01") due to previous error

```
