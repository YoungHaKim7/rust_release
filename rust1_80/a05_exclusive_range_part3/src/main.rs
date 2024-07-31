const TOP: u32 = 10;
// const TOP_MINUS_ONE: u32 = TOP - 1;

fn is_digit(x: u32) -> bool {
    match x {
        0..=TOP => true,
        TOP.. => false,
    }
}
fn main() {
    println!("3 is_digit : {}", is_digit(3));
    println!("1,000 is_digit : {}", is_digit(1_000));
    println!("1,000,000 is_digit : {}", is_digit(1_000_000));
}
