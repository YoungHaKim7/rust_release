// // https://doc.rust-lang.org/std/future/trait.IntoFuture.html
// use std::future::{Future, IntoFuture};
// use std::pin::Pin;
// use std::process::Output;

// pub struct Error {}
// pub struct StorageResponse {}
// pub struct StorageRequest(bool);

// impl StorageRequest {
//     // Create a new instance of `StorageRequest`.
//     pub fn new() -> Self {}
//     // Decide whether debug mode should be enabled.
//     pub fn set_debug(self, b: bool) -> Self {}
//     // Send the request and receive a response.
//     pub async fn send(self) -> Result<StorageResponse, Error> {}
// }

// // The new implementations:
// // 1. create a new named future type
// // 2. implement `IntoFuture` for `StorageRequest`
// // pub type StorageRequestFuture =
// //      Pin<Box<dyn Future<Output = Result<StorageResponse, Error> + Send + 'static>>>

// impl IntoFuture for StorageRequest {
//     type IntoFuture = StorageRequestFuture;
//     type Output = <StorageRequestFuture as Future>::Output;
//     fn into_future(self) -> Self::IntoFuture {
//         Box::pin(self.send())
//     }
// }

// pub trait IntoFuture {
//     type Output;
//     type IntoFuture: Future
//     where
//         <Self::IntoFuture as Future>::Output = Self::Output;

//     fn into_future(self) -> Self::IntoFuture;
// }

use std::future::IntoFuture;

async fn foo() {
    let v = async { "meow" };
    let mut fut = v.into_future();
    assert_eq!("meow", fut.await);
}

fn main() {
    foo();
}
