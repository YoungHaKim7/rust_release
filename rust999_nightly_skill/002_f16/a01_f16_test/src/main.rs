#![feature(f16)]
use std::f16;

fn main() {
    let x = 2.0_f16;
    let abs_difference = (x.powf(2.0) - (x * x)).abs();
    assert!(abs_difference <= f16::EPSILON);

    assert_eq!(f16::powf(1.0, f16::NAN), 1.0);
    assert_eq!(f16::powf(f16::NAN, 0.0), 1.0);
    println!("f16 - MIN : {}", f16::MIN);
    println!("f16 - Epsilon : {}", f16::EPSILON);
}
