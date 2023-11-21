# Result

https://github.com/rust-lang/rust/issues/63063

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
