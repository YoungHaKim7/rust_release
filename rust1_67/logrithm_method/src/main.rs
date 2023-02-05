fn main() {
    assert_eq!(1000_i32.ilog10(), 3);
    assert_eq!(10000_i32.ilog10(), 4);
    assert_eq!(100000_i32.ilog10(), 5);

    assert_eq!(0b1000_i32.ilog2(), 3);
    assert_eq!(0b10000_i32.ilog2(), 4);
    assert_eq!(0b100000_i32.ilog2(), 5);

    assert_eq!(0x1000_i32.ilog(16), 3);
    assert_eq!(0x10000_i32.ilog(16), 4);
    assert_eq!(0x100000_i32.ilog(16), 5);
}
