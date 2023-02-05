// https://rust-lang.github.io/rust-clippy/

fn main() {
    let v = if let Some(v) = w { v } else { return };
    // let Some(v) = w else {return};
}
