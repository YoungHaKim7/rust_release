fn main() {
    let mut v = vec![44, 55, 4, 33, 2, 443, 1].into_iter();

    while let Some(item) = v.next()
        && item < 100
        && item > 5
    {
        println!("{:?}", item);
    }
}
