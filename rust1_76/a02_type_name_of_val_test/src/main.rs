use std::any::type_name_of_val;

fn main() {
    let x = 34;
    let y: i64 = 100_234_212_120;
    let x_float: f32 = 3.1;
    let x_f64 = 3.14;
    let str_string = "string";
    let my_string = "to_string".to_string();

    println!("x : i32 = {} , typeid = {}", x, type_name_of_val(&x));
    println!("y : i64 = {} , typeid = {}", y, type_name_of_val(&y));
    println!(
        "x_float : f32 = {} , typeid = {}",
        x_float,
        type_name_of_val(&x_float)
    );
    println!(
        "x_f64 : f64 = {} , typeid = {}",
        x_f64,
        type_name_of_val(&x_f64)
    );
    println!(
        "str_string : string = {} , typeid = {}",
        str_string,
        type_name_of_val(&str_string)
    );
    println!(
        "my_string : string = {} , typeid = {}",
        my_string,
        type_name_of_val(&my_string)
    );
}
