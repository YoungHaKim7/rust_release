fn main() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    std::thread::scope(|s| {
        s.spawn(|| {
            println!("Hello from first scoped thead");
            dbg!(&a);
        });
        s.spawn(|| {
            println!("Hello from second scoped thead");
            x += a[0] + a[2];
            dbg!(&a);
        });
        println!("Hello from the main thread");
    });

    a.push(4);
    println!(" x = {:?}\n a = {:?}\n a.len = {}\n", x, a, a.len());
}
