async fn f(number: u8) {
    println!("async {number}");
}

async fn g() {
    f(8).await;
}

fn require_send(_: impl Send) {}

#[tokio::main]
async fn main() {
    require_send(g().await);
}
