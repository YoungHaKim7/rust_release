// https://github.com/rust-lang/rust/issues/98608

fn hi() -> impl Sized {hi} as FnOnce<()>::Output == Box<u8>

fn main() {
    let b: Box<dyn Fn() -> Box<u8>> = Box::new(hi);
    let boxed = b();
    let null = *boxed;
    println!("{null:?}");
}

// type AsyncFnPtr = Box<
//     dyn Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = ()>>>,
// >;

// async fn test() {}

// #[allow(unused_must_use)]
// fn main() {
//     Box::new(test) as AsyncFnPtr;
// }