use std::sync::{Arc, Mutex};

async fn bar() {
    // Simulate some asynchronous work
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}

async fn foo(x: Arc<Mutex<u32>>) {
    {
        let mut guard = x.lock().unwrap();
        *guard += 1;
        println!("{guard}")
    }
    bar().await;
}

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    foo(counter.clone()).await;
}
