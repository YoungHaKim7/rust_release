# Result(일부러 error나는 코드임 )

- https://stdrs.dev/nightly/x86_64-unknown-linux-gnu/std/sync/once_lock/struct.OnceLock.html


```bash

error[E0597]: `s` does not live long enough
  --> src/main.rs:14:28
   |
13 |         let s = String::new();
   |             - binding `s` declared here
14 |         let _ = cell.set(A(&s));
   |                            ^^ borrowed value does not live long enough
15 |     }
   |     - `s` dropped here while still borrowed
16 | }
   | - borrow might be used here, when `cell` is dropped and runs the `Drop` code for type `OnceLock`
   |
   = note: values in a scope are dropped in the opposite order they are defined

For more information about this error, try `rustc --explain E0597`.
error: could not compile `oncelock_02` (bin "oncelock_02") due to 1 previous error

```

