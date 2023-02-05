fn main() {
    let my_array: [i32; 10] = std::array::from_fn(|_| 7);
    println!("{my_array:?}");
}
