// a constant function
const fn foo(n: usize) -> usize {
    n + 1
}

fn main() {
    const BAR: usize = foo(5);
    let array = [0_u8; foo(7)];
    dbg!(array);
}
