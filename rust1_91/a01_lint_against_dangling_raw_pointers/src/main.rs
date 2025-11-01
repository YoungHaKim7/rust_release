fn f() -> *const u8 {
    let x = 0;
    &x
}

fn main() {
    f();
    println!(
        "Add lint against dangling raw pointers from local variables : \n\t{:?}",
        f()
    );
}
