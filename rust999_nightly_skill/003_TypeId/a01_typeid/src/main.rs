#![feature(rustc_attrs)]
#![rustc_intrinsic_const_stable_indirect]
fn main() {
    // Basic types
    println!("i32:      {:?}", std::mem::type_info::<i32>());
    println!("&i32:     {:?}", std::mem::type_info::<&i32>());
    println!("&mut i32: {:?}", std::mem::type_info::<&mut i32>());
    println!();

    // Slice types
    println!("[i32]:    {:?}", std::mem::type_info::<[i32]>());
    println!("&[i32]:   {:?}", std::mem::type_info::<&[i32]>());
    println!();

    // Tuple types
    println!("():       {:?}", std::mem::type_info::<()>());
    println!("(i32,):   {:?}", std::mem::type_info::<(i32,)>());
    println!("(i32, i32, i32): {:?}", std::mem::type_info::<(i32, i32, i32)>());
    println!();

    // Array types
    println!("[i32; 0]: {:?}", std::mem::type_info::<[i32; 0]>());
    println!("[i32; 3]: {:?}", std::mem::type_info::<[i32; 3]>());
    println!();

    // Function pointer types
    println!("fn():     {:?}", std::mem::type_info::<fn()>());
    println!("fn(i32) -> i32: {:?}", std::mem::type_info::<fn(i32) -> i32>());
    println!();

    // Custom types
    println!("Point:    {:?}", std::mem::type_info::<Point>());
    println!("&Point:   {:?}", std::mem::type_info::<&Point>());
    println!();

    // Generic types
    println!("Option<i32>: {:?}", std::mem::type_info::<Option<i32>>());
    println!("Result<i32, &str>: {:?}", std::mem::type_info::<Result<i32, &str>>());
    println!();

    // Const pointer types
    println!("*const i32: {:?}", std::mem::type_info::<*const i32>());
    println!("*mut i32:   {:?}", std::mem::type_info::<*mut i32>());
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
