fn main() {
    println!("Hello, world!");

    #[cfg(feature = "crayon")]
    rayon::join(
        || println!("Hello, Thing One!"),
        || println!("Hello, Thing Two!"),
    );
}
