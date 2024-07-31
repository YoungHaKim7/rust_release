use std::sync::LazyLock;
use std::time::Instant;

static LAZY_TIME: LazyLock<Instant> = LazyLock::new(Instant::now);

fn main() {
    let start = Instant::now();
    std::thread::scope(|s| {
        s.spawn(|| {
            println!("Thread lazy time is {:?}", LAZY_TIME.duration_since(start));
            for i in 0..100 {
                println!("test {}", i);
            }
        });
        println!("Main lazy time is {:?}", LAZY_TIME.duration_since(start));
    });
}
