const fn get_some_fn() -> fn(&mut String) {
    String::clear
}
fn main() {
    dbg!(get_some_fn());
}
