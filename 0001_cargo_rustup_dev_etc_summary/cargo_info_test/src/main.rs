use std::sync::{Arc, Mutex};
use tokio::task;

// Define the shared data struct
#[derive(Debug)]
struct Data {
    value: i32,
}

// Define a struct that holds the shared data
#[derive(Debug)]
struct MyStruct {
    shared_data: Arc<Mutex<Data>>,
}

impl MyStruct {
    fn new(data: Data) -> Self {
        let shared_data = Arc::new(Mutex::new(data));
        MyStruct { shared_data }
    }

    fn sync_method(&self) {
        let mut data = self.shared_data.lock().unwrap();
        data.value += 1;
        println!("Sync method: {:?}", *data);
    }

    async fn async_method(shared_data: Arc<Mutex<Data>>) {
        let mut data = shared_data.lock().unwrap();
        data.value += 1;
        println!("Async method: {:?}", *data);
    }
}

#[tokio::main]
async fn main() {
    let my_struct = MyStruct::new(Data { value: 0 });
    my_struct.sync_method();

    let shared_data = my_struct.shared_data.clone();
    task::spawn(async move {
        MyStruct::async_method(shared_data).await;
    })
    .await
    .unwrap();
}
