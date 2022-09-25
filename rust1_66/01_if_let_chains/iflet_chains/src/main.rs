// Rust 1.66 code

#![feature(let_chains)]
fn main() {
    let is_good = true;
    if is_good && is_good {}
    let some_value = Some(9);
    let second_value = Some(10);

    if let Some(number) = some_value && let Some(other_number) =  second_value {

    }
}
