fn main() -> Result<(), String> {
    let parsed_result = "kjldkjf".parse::<i32>().map_err(|e| e.to_string());

    // let binding
    // pattern match
    // irrefutable
    let x = 9;
    // refutable pattern
    if let Ok(num) = parsed_result {}

    // let binding

    let Ok(num) = parsed_result else {
        return Err("Couldn't make a number".to_string())
    };
    println!("{num}");
}
