fn main() {
    let v = String::from("🗻∈🌏");
    let v2 = String::from("테스트🗻∈🌏");
    let v3 = String::from("스");
    assert_eq!(Some("🗻"), v.get(0..4));

    // indices not on UTF-8 sequence boundaries
    assert!(v.get(1..).is_none());
    assert!(v.get(..8).is_none());
    // out of bounds
    assert!(v.get(..42).is_none());
    println!("v2 : {:?}", v2.as_bytes());
    println!("v3 : {:?}", v3.as_bytes());
}
