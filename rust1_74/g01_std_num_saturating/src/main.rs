use std::num::Saturating;

fn main() {
    let max = Saturating(u32::MAX);
    let one = Saturating(1u32);

    assert_eq!(u32::MAX, (max + one).0);
}
