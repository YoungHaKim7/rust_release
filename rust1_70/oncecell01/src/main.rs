use std::cell::OnceCell;

fn main() {
    let cell = OnceCell::new();

    let value: &str = cell.get_or_init(|| "Hello, World OnceCell Std".to_string());

    println!("{value}");
    println!("{}", cell.get().is_some());
}
