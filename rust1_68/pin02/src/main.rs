use tokio::pin;

async fn my_async_fn() {
    // async logic here
    println!("my_async_fn()");
}

#[tokio::main]
async fn main() {
    let future = my_async_fn();
    pin!(future);

    (&mut future).await;
}
