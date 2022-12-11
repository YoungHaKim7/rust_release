fn main() {
    let parsed_result = "kjldkjf".parse::<i32>();

    let Ok(num) = parsed_result else {
        panic!("Dear God");
    };
    println!("{num}");
}
