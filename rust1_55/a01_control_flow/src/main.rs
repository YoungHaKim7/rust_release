use std::ops::ControlFlow;

fn main() {
    let iter = 100;
    let mut sum = 0;
    let _ = (0..iter).try_for_each(|x| {
        if x % 2 == 0 {
            return ControlFlow::Continue(());
        }
        sum += x;
        if sum > 100 {
            return ControlFlow::Break(());
        }
        ControlFlow::Continue(())
    });
    println!("sum = {}", sum);
}
