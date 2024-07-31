fn main() {
    let mut x = Some(42);
    let prev = x.take_if(|v| *v == 42);
    assert_eq!(x, None);
    assert_eq!(prev, Some(42));
}
