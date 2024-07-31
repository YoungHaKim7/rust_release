fn main() {
    let mut x = Some(42);
    let prev = x.take_if(|v| *v == 42);
    assert_eq!(x, None);
    assert_eq!(prev, Some(42));
    let mut test_x = Some(102);
    println!("test_x : {:?}", test_x);
    let prev_test_x = test_x.take_if(|vx| *vx == 102);
    println!("prev_testx : {:?}", prev_test_x);

    let mut x2 = Some(42);

    let prev02 = x2.take_if(|v| {
        if *v == 42 {
            *v += 1;
            false
        } else {
            false
        }
    });
    assert_eq!(x2, Some(43));
    assert_eq!(prev02, None);

    let prev02 = x2.take_if(|v| *v == 43);
    assert_eq!(x2, None);
    assert_eq!(prev02, Some(43));
}
