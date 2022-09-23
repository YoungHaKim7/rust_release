// https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html
use futures::executor::block_on;

async fn run() {
    use core::future::poll_fn;
    use std::task::{Context, Poll};

    fn read_line(_cx: &mut Context<'_>) -> Poll<String> {
        Poll::Ready("Hello, World!".into())
    }

    let read_future = poll_fn(read_line);
    println!("{}, {}", read_future.await, "Hello, World!".to_owned());
}

fn main() {
    let future = run();
    block_on(future);
}
