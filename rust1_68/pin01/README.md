# Result

```
error[E0277]: `[async fn body@src/main.rs:1:24: 4:2]` cannot be unpinned
  --> src/main.rs:10:18
   |
1  | async fn my_async_fn() {
   |                        - within this `impl Future<Output = ()>`
...
10 |     (&mut future).await;
   |                  ^^^^^^
   |                  |
   |                  within `impl Future<Output = ()>`, the trait `Unpin` is not implemented for `[async fn body@src/main.rs:1:24: 4:2]`
   |                  help: remove the `.await`
   |
   = note: consider using `Box::pin`
note: required because it appears within the type `impl Future<Output = ()>`
  --> src/main.rs:1:24
   |
1  | async fn my_async_fn() {
   |                        ^
   = note: required for `&mut impl Future<Output = ()>` to implement `Future`
   = note: required for `&mut impl Future<Output = ()>` to implement `IntoFuture`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `pin01` due to previous error
```
