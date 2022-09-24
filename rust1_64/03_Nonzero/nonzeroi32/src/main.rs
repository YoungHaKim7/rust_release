// https://runebook.dev/ko/docs/rust/std/num/struct.nonzeroi32
use std::num::NonZeroI32;

fn main() {
    // let pos = NonZeroI32::new(1);
    // let neg = NonZeroI32::new(-1);
    let min = NonZeroI32::new(i32::MIN);

    // assert_eq!(Some(pos), neg.expect("REASON").checked_abs());
    assert_eq!(None, min.expect("REASON").checked_abs());
}
