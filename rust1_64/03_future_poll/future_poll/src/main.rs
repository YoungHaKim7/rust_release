use std::future::{self, Future};
use std::pin::Pin;
use std::task::{ready, Context, Poll};

pub fn do_poll(cx: &mut Context<'_>) -> Poll<()> {
    let mut fut = future::ready!(42);
    let fut = Pin::new(&mut fut);

    let num = ready!(fut.poll(cx));
    // ... use num

    Poll::Ready(())
}

fn main() {
    let f = future::poll_fn(|cx| {
        let value = ready!(read_future.poll(cx));
        println!("1 task: ");

        std::task::Poll::Ready(value + 1)
    });

    do_poll(2);
}
