fn main() {
    let x = 9;

    match x {
        num @ i32::MIN..=9 => println!("The number was between MIN and 9: {num}"),
        num => println!("It was bigger than 9: {num}"),
    }
    println!("Hello, world!");
}
