fn main() {
    let my_array: [i32; 5] = std::array::from_fn(|i| 5);
    println!("{my_array:?}");
}
