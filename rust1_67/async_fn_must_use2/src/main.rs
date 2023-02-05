#[tokio::main]
async fn main() {
    print_int().await
}

#[must_use]
async fn get_int() -> u32 {
    0
}

async fn print_int() {
    let i = get_int().await;
}
