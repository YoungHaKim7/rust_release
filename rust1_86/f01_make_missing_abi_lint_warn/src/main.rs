// I don't recommend it.
// extern fn foo() {
//     println!("Called foo from Rust with C ABI");
// };
//
// ~~~~~~~~~~~~~~~
// recommand
// Define a C-compatible function
extern "C" fn foo() {
    println!("Called foo from Rust with C ABI");
}

fn main() {
    println!("Hello, world!");

    // Call the extern "C" function like any other function
    foo();
}
