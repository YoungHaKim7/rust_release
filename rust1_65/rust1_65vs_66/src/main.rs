use std::time::{Duration, Instant};

fn push_cap(v: &mut Vec<i32>) {
    for i in 0..4 {
        v.push(i);
    }
}

pub fn bench_push() -> Duration {
    let mut v = Vec::with_capacity(4);
    let now = Instant::now();
    push_cap(&mut v);
    now.elapsed()
}

fn main() {
    println!("Hello, world!");
}
