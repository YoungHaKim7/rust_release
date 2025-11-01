fn f_implicit(_: &()) -> impl Sized {}
// In Rust 2021 and earlier, the above is equivalent to:
fn f_2021(_: &()) -> impl Sized + use<> {}
// In Rust 2024 and later, it's equivalent to:
fn f_2024(_: &()) -> impl Sized + use<'_> {}

// ~~~~~~

fn capture<'a>(_: &'a ()) -> impl Sized + use<> {}

fn test<'a>(x: &'a ()) -> impl Sized + 'static {
    capture(x) //~ OK
}

fn main() {
    println!("Hello, world!");
}
