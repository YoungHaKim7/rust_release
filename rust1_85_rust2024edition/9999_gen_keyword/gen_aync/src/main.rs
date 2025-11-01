fn main() {
    let odd_dup = |xs| {
        Gen::new(async move |mut y| {
            for x in xs {
                if x % 2 == 1 {
                    y.r#yield(x * 2).await;
                }
            }
        })
    };

    let odd_dup: Pin<&mut _> = pin!(odd_dup(1u8..20));
    let odd_dup = odd_dup.init();

    for (i, x) in odd_dup.enumerate() {
        assert_eq!((i as u8 * 2 + 1) * 2, x);
    }
    println!("Hello, world!");
}
