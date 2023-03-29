use std::panic;

fn main() {
    panic::set_hook(Box::new(|_| {
        println!("There was a problem");
    }));
    panic!();
}
