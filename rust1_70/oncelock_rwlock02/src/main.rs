use std::sync::{OnceLock, RwLock};

static COUNT: OnceLock<RwLock<i32>> = OnceLock::new();

fn function_with_static_variable() {
    let count = COUNT.get_or_init(|| RwLock::new(0));
    let mut count = count.write().unwrap();
    *count += 1;
    println!("{}", *count);
}
fn main() {
    function_with_static_variable();
    function_with_static_variable();
    function_with_static_variable();
    function_with_static_variable();
}
