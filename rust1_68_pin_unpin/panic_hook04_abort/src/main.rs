use std::panic;

// unwinding the stack
// panic -> abort
fn main() {
    panic::set_hook(Box::new(|_| {
        panic!();
        println!("There was a problem");
    }));
    panic!();
}
