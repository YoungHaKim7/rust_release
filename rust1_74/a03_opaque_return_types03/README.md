# Result

- Stablize ```impl_trait_projections``` #115659

https://github.com/rust-lang/rust/pull/115659

```
$ cargo r
info: syncing channel updates for 'nightly-aarch64-apple-darwin'
info: latest update on 2023-11-21, rust version 1.76.0-nightly (3a85a5cfe 2023-11-20)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
info: downloading component 'rust-src'
info: downloading component 'rust-std'
info: downloading component 'rustc'
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
info: installing component 'rust-src'
info: installing component 'rust-std'
info: installing component 'rustc'
info: installing component 'rustfmt'
   Compiling a03_opaque_return_types03 v0.1.0 (/Users/globalyoung/Documents/test/test/rust/rust_release/rust1_74/a03_opaque_return_types03)
warning: the feature `return_position_impl_trait_in_trait` has been stable since 1.75.0 and no longer requires an attribute to enable
 --> src/main.rs:1:12
  |
1 | #![feature(return_position_impl_trait_in_trait, async_fn_in_trait)]
  |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(stable_features)]` on by default

warning: the feature `async_fn_in_trait` has been stable since 1.75.0 and no longer requires an attribute to enable
 --> src/main.rs:1:49
  |
1 | #![feature(return_position_impl_trait_in_trait, async_fn_in_trait)]
  |                                                 ^^^^^^^^^^^^^^^^^

warning: struct `Wrapper` is never constructed
 --> src/main.rs:6:8
  |
6 | struct Wrapper<'a, T>(&'a T);
  |        ^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: associated functions `async_fn` and `impl_trait` are never used
  --> src/main.rs:9:14
   |
8  | impl Wrapper<'_, ()> {
   | -------------------- associated functions in this implementation
9  |     async fn async_fn() -> Self {
   |              ^^^^^^^^
...
14 |     fn impl_trait() -> impl Iterator<Item = Self> {
   |        ^^^^^^^^^^

warning: associated functions `mk_assoc` and `a_few_assocs` are never used
  --> src/main.rs:32:14
   |
31 | impl<'a, T: Trait<'a>> Wrapper<'a, T> {
   | ------------------------------------- associated functions in this implementation
32 |     async fn mk_assoc() -> T::Assoc {
   |              ^^^^^^^^
...
39 |     fn a_few_assocs() -> impl Iterator<Item = T::Assoc> {
   |        ^^^^^^^^^^^^

warning: opaque type `impl Future<Output = Self>` does not satisfy its associated type bounds
  --> src/main.rs:48:5
   |
48 |     async fn async_fn() -> Self;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
  ::: /Users/globalyoung/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/future/future.rs:41:5
   |
41 |     type Output;
   |     ------------ this associated type bound is unsatisfied for `Self`
   |
   = note: `#[warn(opaque_hidden_inferred_bound)]` on by default

warning: opaque type `impl Iterator<Item = Self>` does not satisfy its associated type bounds
  --> src/main.rs:50:38
   |
50 |     fn impl_trait() -> impl Iterator<Item = Self>;
   |                                      ^^^^^^^^^^^
   |
  ::: /Users/globalyoung/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:79:5
   |
79 |     type Item;
   |     ---------- this associated type bound is unsatisfied for `Self`

warning: `a03_opaque_return_types03` (bin "a03_opaque_return_types03") generated 7 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 3.90s
     Running `target/debug/a03_opaque_return_types03`

Hello, world!

```
