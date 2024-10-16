#![allow(clippy::style)]

#[warn(clippy::double_neg)]
fn main() {
    let x = 1;
    let y = --x;
    //      ^^ warning: double negation
}
