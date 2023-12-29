#[derive(Debug)]
struct Widget(i32);

fn main() {
    let array = [Widget(1), Widget(2)];
    match array {
        [ref x, _] => {}
    }
    match array {
        [_, ref y] => {}
    }

    println!("array : {:?}", array);
}
