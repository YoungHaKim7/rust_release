# Result

```bash
error[E0700]: hidden type for `impl Sized` captures lifetime that does not appear in bounds
 --> src/main.rs:2:5
  |
1 | fn f(x: &()) -> impl Sized {
  |         ---     ---------- opaque type defined here
  |         |
  |         hidden type `&()` captures the anonymous lifetime defined here
2 |     x
  |     ^
  |
help: add a `use<...>` bound to explicitly capture `'_`
  |
1 | fn f(x: &()) -> impl Sized + use<'_> {
  |                            +++++++++

For more information about this error, try `rustc --explain E0700`.
error: could not compile `a01_use_error_msg` (bin "a01_use_error_msg") due to 1 previous error

```

