#[tokio::main]
async fn main() {
    print_int().await
}

async fn get_int() -> () {
    println!("get_int & print ");
}

async fn print_int() {
    get_int().await;
}
