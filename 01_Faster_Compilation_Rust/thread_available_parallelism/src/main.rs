fn main() {
    println!(
        "You can use {:?} threads(available_parallelism) now.  ",
        std::thread::available_parallelism().unwrap()
    );
}
