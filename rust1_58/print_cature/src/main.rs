// rust 1.57 code
fn get_person() -> String {
    String::from("sunface")
}

fn main() {
    let p = get_person();
    println!("Hello, {}! ", p); // implicit position
    println!("Hello, {0}! ", p); // explicit index
    println!("Hello, {person}! ", person = p);
}
