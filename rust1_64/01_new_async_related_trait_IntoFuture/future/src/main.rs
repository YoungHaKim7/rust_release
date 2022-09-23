use std::future::{ready, IntoFuture, Ready};

struct Example;

impl IntoFuture for Example {
    type Output = i32;
    type IntoFuture = Ready<i32>;
    fn into_future(self) -> Ready<i32> {
        ready(123)
    }
}

async fn example() {
    println!("Hello, {} ! ", Example.await);
}

fn main() {}
