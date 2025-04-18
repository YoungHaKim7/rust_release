fn main() {
    let v = &mut [1, 2, 3];
    if let Ok([a, b]) = v.get_disjoint_mut([0, 2]) {
        *a = 413;
        *b = 612;
    }
    assert_eq!(v, &[413, 2, 612]);
    println!("v : {:?}", &v);

    if let Ok([a, b]) = v.get_disjoint_mut([0..1, 1..3]) {
        a[0] = 8;
        b[0] = 88;
        b[1] = 888;
    }
    assert_eq!(v, &[8, 88, 888]);
    println!("v : {:?}", &v);
}
