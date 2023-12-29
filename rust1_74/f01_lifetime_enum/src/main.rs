#[derive(Debug)]
enum MyEnum<'a> {
    Nothing,
    RefVector(&'a Vec<i32>),
}
fn main() {
    let my_vec: Vec<i32> = vec![32];
    println!("My Enum _ RefVector{:?}", MyEnum::RefVector(&my_vec));
}
