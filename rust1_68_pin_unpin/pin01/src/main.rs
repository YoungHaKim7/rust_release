async fn my_async_fn() {
    // async logic here
    println!("my_async_fn()");
}


#[tokio::main]
async fn main() {
    let mut future = my_async_fn();
    (&mut future).await;
}
