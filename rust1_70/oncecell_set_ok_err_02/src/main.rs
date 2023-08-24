use std::cell::OnceCell;

fn main() {
    let cell = OnceCell::new();
    cell.set(92);
    cell.set(62);

    println!("cell value = {:?}", cell)
}
