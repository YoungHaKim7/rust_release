use std::ops::ControlFlow;

fn main() {
    let r = (2..100).try_for_each(|x| {
        if 323 % x == 0 {
            return ControlFlow::Break(x);
        }

        ControlFlow::Continue(())
    });
    assert_eq!(r, ControlFlow::Break(17));
}
